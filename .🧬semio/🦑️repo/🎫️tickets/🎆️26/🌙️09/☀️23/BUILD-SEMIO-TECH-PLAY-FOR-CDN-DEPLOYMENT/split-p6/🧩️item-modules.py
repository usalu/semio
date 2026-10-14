#!/usr/bin/env python3
"""🧩️ Splits one oversized Rust module file into child module files by an item-to-child spec.

Parses the file's top-level items (doc comments, attributes and `//#region` markers kept), assigns each to a child by
the first matching name regex of the spec (an `impl` follows its target type, a top-level macro invocation follows the
item before it, a `macro_rules!` goes to the spec's `macros` child), checks that the child graph is acyclic, and with
`apply` writes every child as `<dir>/🦀️.rs` (the items in source order, the parent's `use` lines each child needs,
`use crate::<parent>::<sibling>::{..}` lines for sibling items, cross-child private items made `pub`) and rewrites the
parent to its remaining items plus one `#[path] pub mod` per child and one `pub use <child>::*;` re-export per child.
Those re-exports are temporary: `🧭️canonical-paths.py` retargets every path through them, then they are removed.

usage: 🧩️item-modules.py plan|apply <spec.json>
spec: {"file": repo-relative parent file, "module": "crate::..::parent", "macros": child-or-null,
       "children": [{"name": ident, "dir": taxonomy dir, "doc": "//! ...", "match": [regex on item name...]}],
       "keep": [regex on item name staying in the parent], "budget": {"lines": n, "bytes": n}}
"""
import collections, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
ITEM_SPLIT = os.path.join(HERE, "..", "e3-format-split", "🔀️item-split.py")
ns = {"__file__": ITEM_SPLIT}
exec(compile(open(ITEM_SPLIT, encoding="utf-8").read().split("\nif __name__")[0], ITEM_SPLIT, "exec"), ns)
parse = ns["parse"]
DEFINITION = ("fn", "struct", "enum", "trait", "type", "const", "static", "mod", "union", "macro_rules!")


def macro_names(item):
    head = re.sub(r"^(\s*(#!?\[[^\]]*\]|///[^\n]*|//[^\n]*)\s*)*", "", item.stripped)
    m = re.match(r"\s*([A-Za-z_][A-Za-z0-9_]*)\s*!\s*[\(\{\[]([^;]*?)(?:=>|;)", head, re.S)
    if not m:
        return set(), None
    owner = re.search(r"=>\s*([A-Za-z_][A-Za-z0-9_]*)", head.split(";", 1)[0])
    return set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", m.group(2))), owner.group(1) if owner else None


def analyse(spec):
    path = os.path.join(REPO, spec["file"])
    src, items = parse(path)
    names = collections.defaultdict(list)
    macro_owner = {}
    for it in items:
        if it.kind != "use":
            it.idents = set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", it.stripped))
    for it in items:
        if it.kind in DEFINITION:
            names[it.name].append(it)
        elif it.kind == "macro":
            defined, owner = macro_names(it)
            it.defines = defined
            macro_owner[it.i] = owner
            for name in defined:
                names[name].append(it)
    if any(it.kind == "macro_rules!" for it in items):
        for it in items:
            if it.kind == "macro_rules!":
                names[it.name].append(it)
    child_rules = [(child["name"], [re.compile(r) for r in child["match"]]) for child in spec["children"]]
    keep = [re.compile(r) for r in spec.get("keep", [])]
    assign = {}
    for it in items:
        if it.kind in DEFINITION and it.kind != "macro_rules!":
            if any(r.search(it.name) for r in keep):
                assign[it.i] = None
                continue
            for name, rules in child_rules:
                if any(r.search(it.name) for r in rules):
                    assign[it.i] = name
                    break
        elif it.kind == "macro_rules!":
            assign[it.i] = spec.get("macros")
    for it in items:
        if it.kind == "impl":
            owners = names.get(it.target, []) or [t for ident in re.findall(r"[A-Za-z_][A-Za-z0-9_]*", it.trait or "") for t in names.get(ident, []) if t.kind in ("struct", "enum", "union", "type", "trait")]
            if owners and owners[0].i in assign:
                assign[it.i] = assign[owners[0].i]
        elif it.kind == "macro" and macro_owner.get(it.i):
            owners = names.get(macro_owner[it.i], [])
            if owners and owners[0].i in assign:
                assign[it.i] = assign[owners[0].i]
    previous = None
    for it in items:
        if it.i in assign:
            previous = assign[it.i]
        elif it.kind in ("macro", "?"):
            assign[it.i] = previous
    unassigned = [it for it in items if it.i not in assign and it.kind not in ("use",)]
    edges = collections.defaultdict(set)
    for it in items:
        if it.kind == "use":
            continue
        for ident in it.idents:
            for target in names.get(ident, []):
                if target.i != it.i:
                    edges[it.i].add(target.i)
        if it.kind == "macro" and macro_owner.get(it.i):
            for target in names.get(macro_owner[it.i], []):
                edges[it.i].add(target.i)
    return src, items, names, assign, edges, unassigned


def child_graph(items, assign, edges):
    graph = collections.defaultdict(collections.Counter)
    for it in items:
        a = assign.get(it.i, "?")
        for j in edges[it.i]:
            b = assign.get(j, "?")
            if a != b:
                graph[a][(b, items[j].name)] += 1
    return graph


def cycles(graph, nodes):
    order, state, found = [], {}, []

    def visit(node, stack):
        state[node] = 1
        for (target, _), _count in graph.get(node, {}).items():
            if state.get(target) == 1:
                found.append(stack[stack.index(target):] + [target] if target in stack else [node, target])
            elif not state.get(target):
                visit(target, stack + [target])
        state[node] = 2
        order.append(node)

    for node in nodes:
        if not state.get(node):
            visit(node, [node])
    return found, order


def plan(spec, quiet=False):
    src, items, names, assign, edges, unassigned = analyse(spec)
    sizes = collections.Counter()
    nbytes = collections.Counter()
    for it in items:
        key = assign.get(it.i, "?")
        sizes[key] += it.text.count("\n") + 1
        nbytes[key] += len(it.text.encode("utf-8"))
    graph = child_graph(items, assign, edges)
    nodes = [child["name"] for child in spec["children"]] + [None]
    found, order = cycles(graph, nodes)
    if not quiet:
        budget = spec.get("budget", {})
        for node in nodes:
            over = (sizes[node] > budget.get("lines", 10 ** 9)) or (nbytes[node] > budget.get("bytes", 10 ** 12))
            print(f"{'OVER ' if over else ''}{node or '(parent)'}: {sizes[node]} lines, {nbytes[node]} bytes, deps {sorted({str(t) for (t, _) in graph.get(node, {})})}")
        for it in unassigned:
            print(f"UNASSIGNED {it.kind} {it.name} L{src[:it.start].count(chr(10)) + 1}")
        for cycle in [list(c) for c in dict.fromkeys(tuple(c) for c in found)]:
            print("CYCLE " + " -> ".join(str(c) for c in cycle))
            for a, b in zip(cycle, cycle[1:]):
                print(f"   {a} -> {b}: {sorted(n for (t, n) in graph[a] if t == b)[:12]}")
    return src, items, names, assign, edges, unassigned, found


PUB = re.compile(r"^((?:\s*(?:#!?\[[^\]]*\]|///[^\n]*|//[^\n]*)\s*)*?\s*)(?:pub\s*\((?:crate|super|self)\)\s+)?((?:const\s+fn|unsafe\s+fn|fn|struct|enum|trait|type|const|static|union|mod)\b)")


def render(entries):
    body, current = [], ()
    for region, text in entries:
        common = 0
        while common < min(len(current), len(region)) and current[common] == region[common]:
            common += 1
        for r in reversed(current[common:]):
            body.append(f"//#endregion {r}")
        for r in region[common:]:
            body.append(f"//#region {r}")
        current = region
        body.append(text.strip("\n"))
    for r in reversed(current):
        body.append(f"//#endregion {r}")
    return "\n".join(body)


def apply(spec):
    src, items, names, assign, edges, unassigned, found = plan(spec, quiet=True)
    if unassigned or found:
        plan(spec)
        raise SystemExit("refused: unassigned items or child cycles")
    path = os.path.join(REPO, spec["file"])
    parent_dir = os.path.dirname(path)
    module = spec["module"]
    macros = spec.get("macros")
    uses = [it for it in items if it.kind == "use"]
    publicize = set()
    needs = collections.defaultdict(lambda: collections.defaultdict(set))
    for it in items:
        if it.kind == "use":
            continue
        a = assign.get(it.i)
        for j in edges[it.i]:
            target = items[j]
            b = assign.get(j)
            if a == b or target.kind in ("impl", "use", "?"):
                continue
            if target.kind == "macro_rules!":
                continue
            if target.kind == "macro":
                for name in target.defines & it.idents:
                    needs[a][b].add(name)
            else:
                needs[a][b].add(target.name)
            publicize.add(j)
    out = collections.defaultdict(list)
    for it in items:
        if it.kind == "use":
            continue
        text = it.text
        if it.i in publicize and it.kind in DEFINITION:
            text = PUB.sub(lambda m: m.group(1) + "pub " + m.group(2), text, count=1) if not re.match(r"^(?:\s*(?:#!?\[[^\]]*\]|///[^\n]*|//[^\n]*)\s*)*\s*pub\s+(?!\()", text) else text
        if it.kind == "macro" and it.i in publicize:
            text = re.sub(r"\bfn\s+", "pub fn ", text)
        out[assign.get(it.i)].append((it.region, text))
    header_uses = {}
    for it in uses:
        text = re.sub(r"^(?:[ \t]*//![^\n]*\n|[ \t]*\n)*", "", it.text).strip("\n")
        header_uses[it.i] = text
        body = re.sub(r"^\s*(?:pub(?:\s*\([^)]*\))?\s+)?use\s+", "", re.sub(r"(?m)^\s*(?:#\[[^\]]*\]|//[^\n]*)\s*$", "", text)).rstrip(";").strip()
        leaves = set()
        for leaf in re.split(r"[{},]", body):
            leaf = leaf.strip()
            if not leaf:
                continue
            alias = re.search(r"\bas\s+([A-Za-z_][A-Za-z0-9_]*)$", leaf)
            last = re.findall(r"[A-Za-z_][A-Za-z0-9_]*", leaf)
            if alias:
                leaves.add(alias.group(1))
            elif last and not leaf.endswith("::"):
                leaves.add(last[-1])
        it.idents = leaves - {"self", "crate", "super"} or set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", body))
    writes = {}
    for child in spec["children"]:
        name = child["name"]
        entries = out[name]
        idents = set()
        for it in items:
            if assign.get(it.i) == name and it.kind not in ("use", "macro_rules!"):
                idents |= it.idents
                for j in edges[it.i]:
                    if items[j].kind == "macro_rules!":
                        idents |= items[j].idents
        lines = [child["doc"], ""]
        for it in uses:
            if it.idents & idents:
                lines.append(header_uses[it.i])
        for other, wanted in sorted(needs[name].items(), key=lambda kv: str(kv[0])):
            prefix = module if other is None else f"{module}::{other}"
            lines.append(f"use {prefix}::{{{', '.join(sorted(wanted))}}};")
        writes[os.path.join(parent_dir, child["dir"], "🦀️.rs")] = "\n".join(lines).rstrip() + "\n\n" + render(entries) + "\n"
    parent_entries = out[None]
    idents = set()
    for it in items:
        if assign.get(it.i) is None and it.kind != "use":
            idents |= it.idents
    doc = re.match(r"((?://![^\n]*\n)+)", src)
    lines = [doc.group(1).rstrip("\n")] if doc else []
    lines.append("")
    for it in uses:
        if it.idents & idents:
            lines.append(header_uses[it.i])
    for other, wanted in sorted(needs[None].items(), key=lambda kv: str(kv[0])):
        lines.append(f"use {module}::{other}::{{{', '.join(sorted(wanted))}}};")
    lines.append("")
    for child in spec["children"]:
        attr = "#[macro_use]\n" if child["name"] == macros else ""
        lines.append(f'{attr}#[path = "{child["dir"]}/🦀️.rs"]\npub mod {child["name"]};')
    for child in spec["children"]:
        lines.append(f"pub use {child['name']}::*;")
    body = render([(region, text) for region, text in parent_entries if text.strip()])
    writes[path] = "\n".join(lines).rstrip() + "\n\n" + body + "\n"
    for target, text in writes.items():
        os.makedirs(os.path.dirname(target), exist_ok=True)
        open(target, "w", encoding="utf-8").write(text)
        print(f"wrote {os.path.relpath(target, REPO)} ({text.count(chr(10))} lines, {len(text.encode())} bytes)")


if __name__ == "__main__":
    mode, spec_path = sys.argv[1], sys.argv[2]
    spec = json.load(open(spec_path, encoding="utf-8"))
    (plan if mode == "plan" else apply)(spec)
