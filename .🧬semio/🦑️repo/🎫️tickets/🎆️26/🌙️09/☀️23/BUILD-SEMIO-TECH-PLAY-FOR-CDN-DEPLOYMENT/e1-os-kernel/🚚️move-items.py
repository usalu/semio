#!/usr/bin/env python3
"""🚚️ Moves named top-level items of the os store root (`🏪️store/🦀️.rs`) into a new child module file (wave E1, step 1).

Spec (JSON, one per move, in `🔀️moves/`):
  {"target": "<path under 🏪️store/>", "module": "<mod name>", "anchor": "<exact source text the mount follows>",
   "doc": ["//! ...", ...], "items": ["Name", ...], "regions": ["<//#region name>", ...], "exceptItems": [..],
   "movableTraits": ["LocalTraitName", ...], "keepInherentImplsOf": ["TypeName", ...], "implsOf": ["TypeMovedEarlier", ...],
   "source": "<child file under 🏪️store/, default the root>", "sourceModule": "<its module name in the root>"}
With `source`, items leave that child file; the new module is still mounted in the store root and the child file
imports the moved names back from it.
`regions` adds every named top-level item whose innermost enclosing region is one of those (nested regions are listed separately).

What it does, in one read-modify-write of the store root:
- every top-level item named in `items` moves with its leading docs, attributes and plain comments (region markers
  stay where they are), and so does every `impl` whose self type is moved, unless it implements a crate-local trait
  that is neither moved nor listed in `movableTraits` (those stay with their trait);
- paths into the store root's own child modules (`member_open::…`) are rebased onto `super::`;
- a moved private item becomes `pub(crate)`; the store root re-exports the moved `pub` items (`pub use`) and the
  moved crate-private ones at `pub(crate)` (cfg-gated like their items), so every existing path keeps resolving inside and outside the crate;
- the new file starts with the doc header and the store root's own `use` preamble (made private; sibling-module paths
  rebased onto `super::`), plus `use super::{..}` for every store-root item the moved items name.
Compile-driven visibility and import fixes follow with `🔁️fix-loop.py`; unused preamble imports are pruned by
`🧹️prune-imports.py`. Usage: move-items.py SPEC_JSON [--owners OWNERS_JSON] [--write]
"""
import importlib.util, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("ri", os.path.join(HERE, "🧱️rust_items.py"))
ri = importlib.util.module_from_spec(spec); spec.loader.exec_module(ri)
STORE = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
ROOT = f"{STORE}/🦀️.rs"
NAMED = {"fn", "struct", "enum", "trait", "union", "type", "static", "const", "macro_rules"}


def lead_split(text):
    keep, move = [], []
    for line in text.split("\n"):
        (keep if re.match(r"\s*//#(end)?region", line) else move).append(line)
    return "\n".join(keep), "\n".join(move)


def widen(text, head_off):
    m = re.compile(r"(pub(?:\s*\([^)]*\))?\s+)?").match(text, head_off)
    if m.group(1): return text, m.group(1).strip() == "pub"
    return text[:head_off] + "pub(crate) " + text[head_off:], False


def region_stacks(raw):
    stack, out = [], [()]
    for line in raw.split("\n"):
        t = line.strip()
        if t.startswith("//#region"): stack.append(t[9:].strip())
        elif t.startswith("//#endregion"):
            name = t[12:].strip()
            if name in stack:
                while stack and stack.pop() != name: pass
        out.append(tuple(stack))
    return out


def main():
    sp = json.load(open(sys.argv[1], encoding="utf-8"))
    write = "--write" in sys.argv
    src_path = os.path.join(STORE, sp["source"]) if sp.get("source") else ROOT
    src_mod = sp.get("sourceModule")
    raw = open(src_path, encoding="utf-8").read()
    cl = ri.clean(raw)
    its, _ = ri.items(raw, cl, 0, len(cl))
    names = set(sp.get("items", []))
    if sp.get("regions"):
        stacks = region_stacks(raw)
        wanted, skip = set(sp["regions"]), set(sp.get("exceptItems", []))
        for it in its:
            stack = stacks[ri.line_of(raw, it["head"])]
            if it["kind"] in NAMED and it["name"] not in skip and stack and stack[-1] in wanted: names.add(it["name"])
    local_traits = {it["name"] for it in its if it["kind"] == "trait"}
    movable_traits = names | set(sp.get("movableTraits", []))
    found = {it["name"] for it in its if it["kind"] in NAMED and it["name"] in names}
    missing = names - found
    if missing: sys.exit(f"items not found at top level: {sorted(missing)}")
    chosen = []
    for it in its:
        if it["kind"] in NAMED and it["name"] in names: chosen.append(it)
        elif it["kind"] == "impl" and (it.get("self") in names or it.get("self") in sp.get("implsOf", [])):
            tr = it.get("trait")
            if not tr and it["self"] in sp.get("keepInherentImplsOf", []):
                print(f"[keep] inherent impl {it['self']} (stays with the engine)")
                continue
            if tr and tr in local_traits and tr not in movable_traits:
                print(f"[keep] impl {tr} for {it['self']} (local trait stays)")
                continue
            chosen.append(it)
    root_mods = {it["name"] for it in its if it["kind"] == "mod"} if src_path == ROOT else set()
    mod_path = re.compile(r"(?<![\w:])(" + "|".join(sorted(map(re.escape, root_mods), key=len, reverse=True)) + r")::") if root_mods else None

    def rebase(start, end):
        text, clean_text = raw[start:end], cl[start:end]
        if not mod_path: return text
        out, pos = [], 0
        for m in mod_path.finditer(clean_text):
            out.append(text[pos:m.start()]); out.append("super::"); pos = m.start()
        out.append(text[pos:])
        return "".join(out)

    pub_names, crate_names = set(), set()
    moved, keep_marks, cuts = [], {}, []
    for it in chosen:
        lead_keep, lead_move = lead_split(raw[it["start"]:it["head"]])
        body = rebase(it["head"], it["end"])
        if it["kind"] in NAMED:
            body, was_pub = widen(body, 0)
            gate = re.findall(r"#\[cfg\((.*)\)\]\s*$", raw[it["start"]:it["head"]], re.M)
            (pub_names if was_pub else crate_names).add((it["name"], gate[-1] if gate else None))
        moved.append(lead_move.strip("\n") + "\n" + body if lead_move.strip() else body)
        cuts.append((it["start"], it["end"], lead_keep))
    out, pos = [], 0
    for a, b, keep in cuts:
        out.append(raw[pos:a]); pos = b
        if keep.strip(): out.append("\n" + keep.strip("\n"))
    out.append(raw[pos:])
    rest = "".join(out)
    rest = re.sub(r"\n{3,}", "\n\n", rest)
    mod_names = {it["name"] for it in its if it["kind"] == "mod"}
    preamble = []
    for it in its:
        if it["kind"] != "use": continue
        text = raw[it["head"]:it["end"]].strip()
        if re.search(r"cfg\(test\)", raw[it["start"]:it["head"]]): continue
        text = re.sub(r"^pub(?:\s*\([^)]*\))?\s+use\s+", "use ", text)
        m = re.match(r"use\s+(?:self::)?(\w+)", text)
        if src_path != ROOT: text = re.sub(r"^use\s+self::", f"use super::{src_mod}::", text)
        elif m and m.group(1) in mod_names: text = re.sub(r"^use\s+(?:self::)?", "use super::", text)
        gates = re.findall(r"^\s*(#\[cfg\(.*\)\])\s*$", raw[it["start"]:it["head"]], re.M)
        preamble.append((gates[-1] + "\n" if gates else "") + text)
    refs = set(sp.get("superImports", []))
    if "--owners" in sys.argv:
        owners = json.load(open(sys.argv[sys.argv.index("--owners") + 1]))
        P = "os_store::component::" + (src_mod + "::" if src_path != ROOT else "")
        root_defined = {it["name"] for it in its if it["kind"] in NAMED} - names
        def moved_owner(p):
            if not p.startswith(P): return False
            seg = p[len(P):].split("::")
            if seg[0] in names: return True
            if seg[0].startswith("{impl#"):
                o = owners.get(P + seg[0], {})
                return o.get("self", "").startswith(P) and o["self"][len(P):].split("::")[0] in names
            return False
        for p, o in owners.items():
            if not moved_owner(p): continue
            for t in o["refs"]:
                if not t.startswith(P): continue
                head = t[len(P):].split("::")[0]
                if head in root_defined: refs.add(head if src_path == ROOT else f"{src_mod}::{head}")
    mount = f'#[path = "{os.path.relpath(sp["target"], STORE) if os.path.isabs(sp["target"]) else sp["target"]}"]\nmod {sp["module"]};\n'
    for vis, group in (("pub", pub_names), ("pub(crate)", crate_names)):
        for gate in sorted({g for _, g in group}, key=lambda g: g or ""):
            names_in = sorted({n for n, g in group if g == gate} - {n for n, g in group if g is None and gate is not None})
            if names_in: mount += (f"#[cfg({gate})]\n" if gate else "") + f"{vis} use {sp['module']}::{{{', '.join(names_in)}}};\n"
    anchor = sp["anchor"]
    root_text = rest if src_path == ROOT else open(ROOT, encoding="utf-8").read()
    if root_text.count(anchor) != 1: sys.exit(f"anchor occurs {root_text.count(anchor)} times in the store root")
    root_text = root_text.replace(anchor, anchor + "\n" + mount, 1)
    if src_path != ROOT:
        plain = sorted({n for n, g in pub_names | crate_names if g is None})
        lines = rest.split("\n"); i = 0
        while i < len(lines) and lines[i].startswith("//!"): i += 1
        lines.insert(i, f"use super::{sp['module']}::{{{', '.join(plain)}}};")
        rest = "\n".join(lines)
    target = sp["target"] if os.path.isabs(sp["target"]) else os.path.join(STORE, sp["target"])
    header = "\n".join(sp["doc"]) + "\n"
    imports = "\n".join(dict.fromkeys(preamble)) + "\n"
    if refs: imports += f"use super::{{{', '.join(sorted(set(refs)))}}};\n"
    content = header + imports + "\n" + "\n\n".join(m.strip("\n") for m in moved) + "\n"
    print(f"[move] {len(chosen)} items ({len({n for n, _ in pub_names})} pub, {len({n for n, _ in crate_names})} widened to pub(crate)), {content.count(chr(10))} lines -> {os.path.relpath(target, STORE)}")
    if write:
        if os.path.exists(target): sys.exit(f"target exists: {target}")
        os.makedirs(os.path.dirname(target), exist_ok=True)
        open(target, "w", encoding="utf-8").write(content)
        open(ROOT, "w", encoding="utf-8").write(root_text)
        if src_path != ROOT: open(src_path, "w", encoding="utf-8").write(rest)
    else:
        for it in chosen: print(f"   {it['kind']:10} {it.get('name') or (str(it.get('trait')) + ' for ' + str(it.get('self')))}  @{ri.line_of(raw, it['head'])}")


if __name__ == "__main__":
    main()
