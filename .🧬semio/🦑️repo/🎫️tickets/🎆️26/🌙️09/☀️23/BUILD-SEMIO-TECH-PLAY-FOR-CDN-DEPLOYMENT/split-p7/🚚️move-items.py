#!/usr/bin/env python3
"""🚚️ Moves named top-level items (with their impl blocks) from one Rust module file into another module file of the
same monolith, before a family cut.

The moved items keep their docs, attributes and `//#region` markers; the target file receives exactly the `use` leaves
of the source file that the moved code names (relative paths absolutised against the source module), plus imports of
the source items it still names (reported as BACK edges, because they point up once the files land in different
crates). The source file imports the moved names it still uses and keeps one temporary `//#region 🚚️Moved` block of
`pub use` re-exports so the monolith still compiles; `✂️family-cut.py` resolves paths through that block (its `moves`
config) and deletes it. Private moved items named across the two files become `pub`.

usage: 🚚️move-items.py plan|apply <spec.json>
spec: {"moves": [{"from": repo file, "from_module": "crate::a::b", "to": repo file, "to_module": "crate::a::c",
                  "items": [regex on item name], "doc": "//! … (new target file only)",
                  "mount": "#[path = …]\\npub mod …; (inserted into the source file for a new target)"}]}
"""
import collections, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
ITEM_SPLIT = os.path.join(HERE, "..", "e3-format-split", "🔀️item-split.py")
ns = {"__file__": ITEM_SPLIT}
exec(compile(open(ITEM_SPLIT, encoding="utf-8").read().split("\ndef analyse")[0], ITEM_SPLIT, "exec"), ns)
parse = ns["parse"]
IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
SEP = r"\s*::\s*"
DEFINITION = ("fn", "struct", "enum", "trait", "type", "const", "static", "union", "mod", "macro_rules!")
PUB = re.compile(r"^((?:\s*(?:#!?\[[^\]]*\]|///[^\n]*|//[^\n]*)\s*)*?\s*)(?:pub\s*\((?:crate|super|self|in [^)]*)\)\s+)?((?:(?:const|async|unsafe)\s+)*(?:fn|struct|enum|trait|type|const|static|union|mod)\b)")


def leaves(prefix, body):
    m = re.match(r"^((?:" + IDENT + SEP + r")*)\{(.*)\}$", body.strip(), re.S)
    if m:
        pre = prefix + [x for x in re.split(SEP, m.group(1)) if x]
        out, depth, cur = [], 0, ""
        for ch in m.group(2):
            depth += ch == "{"
            depth -= ch == "}"
            if ch == "," and depth == 0:
                out += leaves(pre, cur)
                cur = ""
            else:
                cur += ch
        if cur.strip():
            out += leaves(pre, cur)
        return out
    alias = re.search(r"\s+as\s+(" + IDENT + r")$", body.strip())
    core = re.sub(r"\s+as\s+" + IDENT + r"$", "", body.strip())
    return [(prefix + [x for x in re.split(SEP, core) if x], alias.group(1) if alias else None)]


def absolute(segs, module):
    base = module.split("::")
    if segs[0] in ("super", "self"):
        b, k = list(base), 0
        while k < len(segs) and segs[k] in ("super", "self"):
            if segs[k] == "super":
                b.pop()
            k += 1
        return b + segs[k:]
    return segs


def use_leaves(item, module):
    """🧾️ (bound name, absolute path segments) of every leaf of a `use` item."""
    body = re.sub(r"(?s)^.*?\buse\s+", "", re.sub(r"(?m)^\s*(?:#\[[^\]]*\]|//[^\n]*)\s*$", "", item.stripped)).rstrip().rstrip(";").strip()
    out = []
    for segs, alias in leaves([], body):
        segs = absolute(segs, module) if segs and segs[0] in ("super", "self") else segs
        if segs[-1] == "self":
            segs = segs[:-1]
        name = alias or segs[-1]
        out.append((name, segs, alias))
    return out


def drop_bound(text, bound):
    """✂️ Removes from every top-level `use` of `text` the leaves that bind one of `bound`."""
    def fix(m):
        rows = [(segs, alias) for segs, alias in leaves([], m.group("body")) if (alias or segs[-1]) not in bound]
        if len(rows) == len(leaves([], m.group("body"))):
            return m.group(0)
        return "\n".join(render_use(rows, m.group("vis") or "")) + ("\n" if rows else "")
    return re.sub(r"(?m)^(?P<vis>pub(?:\s*\([^)]*\))?\s+)?use\s+(?P<body>[^;]*);[ \t]*\n", fix, text)


def render_use(rows, vis=""):
    groups = collections.OrderedDict()
    for segs, alias in rows:
        groups.setdefault(segs[0], []).append("::".join(segs[1:]) + (f" as {alias}" if alias else ""))
    lines = []
    for head, parts in groups.items():
        parts = sorted(set(parts))
        tree = parts[0] if len(parts) == 1 else "{" + ", ".join(parts) + "}"
        lines.append(f"{vis}use {head}::{tree};")
    return lines


def relative_paths(text, stripped, module, roots=()):
    """🧭️ Absolutises every `super::`/`self::` path expression (and, out of the crate root, every root-relative path) of
    moved code against its old module."""
    edits = []
    if module == "crate" and roots:
        for m in re.finditer(r"(?<![\w:$.])((?:" + "|".join(roots) + r")\s*::)", stripped):
            edits.append((m.start(), m.start(), "crate::"))
    for m in re.finditer(r"(?<![\w:$])((?:super|self)(?:" + SEP + r"(?:" + IDENT + r"|super))+)", stripped):
        segs = [x for x in re.split(SEP, m.group(1)) if x]
        edits.append((m.start(), m.end(), "::".join(absolute(segs, module))))
    for a, b, rep in sorted(edits, reverse=True):
        text = text[:a] + rep + text[b:]
    return text


def run(move, mode):
    src_path, dst_path = os.path.join(REPO, move["from"]), os.path.join(REPO, move["to"])
    src, items = parse(src_path)
    rules = [re.compile(r) for r in move["items"]]
    chosen = {it.i for it in items if it.kind in DEFINITION and any(r.fullmatch(it.name) for r in rules)}
    names = {items[i].name for i in chosen}
    missing = [r.pattern for r in rules if not any(r.fullmatch(n) for n in names)]
    for it in items:
        if it.kind == "impl" and it.target in names:
            chosen.add(it.i)
    kept_names = {it.name for it in items if it.kind in DEFINITION and it.kind != "mod" and it.i not in chosen}
    for it in items:
        previous = items[it.i - 1] if it.i else None
        if it.kind == "macro" and it.i not in chosen and previous is not None and previous.i in chosen and previous.region == it.region:
            idents = set(re.findall(IDENT, it.stripped))
            if idents & names and not idents & kept_names:
                chosen.add(it.i)
    if missing:
        print(f"WARN unmatched {missing}")
    defined = collections.defaultdict(list)
    for it in items:
        if it.kind in DEFINITION:
            defined[it.name].append(it)
    moved = [it for it in items if it.i in chosen]
    kept = [it for it in items if it.i not in chosen]
    moved_idents = set().union(*[it.idents for it in moved]) if moved else set()
    kept_idents = set().union(*[it.idents for it in kept if it.kind != "use"]) if kept else set()
    uses = [it for it in kept if it.kind == "use"]
    def named(name, pool):
        pattern = re.compile(r"(?<![\w.])" + re.escape(name) + r"(?!\s*:(?!:))(?!\w)")
        return any(pattern.search(it.stripped) for it in pool)
    back = sorted(n for n in moved_idents if n in defined and n not in names and any(d.i not in chosen and d.kind != "mod" for d in defined[n]) and named(n, moved))
    forward = sorted(n for n in names if n in kept_idents and named(n, [it for it in kept if it.kind != "use"]))
    src_mod, dst_mod = move["from_module"], move["to_module"]
    target_uses = []
    for it in uses:
        for name, segs, alias in use_leaves(it, src_mod.replace("crate::", "crate::", 1)):
            if name in moved_idents and (alias or segs[-1]) not in names:
                if src_mod == "crate" and segs[0] in move.get("root_relative", ()):
                    segs = ["crate"] + segs
                target_uses.append((segs, alias))
    target_uses += [(src_mod.split("::") + [n], None) for n in back]
    print(f"move {len(moved)} items {sorted(names)} -> {move['to']}")
    print(f"  BACK (target names source items) {back}")
    print(f"  FORWARD (source still names moved items) {forward}")
    if mode != "apply":
        return []
    publicize = set(back) | set(forward)
    def bump(it):
        if it.kind in DEFINITION and it.name in publicize and not re.match(r"^(?:\s*(?:#!?\[[^\]]*\]|///[^\n]*|//[^\n]*)\s*)*\s*pub\s+(?!\()", it.text):
            return PUB.sub(lambda m: m.group(1) + "pub " + m.group(2), it.text, count=1)
        return it.text
    body, region = [], ()
    for it in moved:
        text = relative_paths(bump(it), it.stripped if bump(it) == it.text else ns["strip"](bump(it)), src_mod, move.get("root_relative", ()))
        text = re.sub(r'(include(?:_str|_bytes)?!\s*\(\s*")([^"]+)(")', lambda m: m.group(1) + os.path.relpath(os.path.normpath(os.path.join(os.path.dirname(src_path), m.group(2))), os.path.dirname(dst_path)) + m.group(3), text)
        if it.region != region:
            for r in reversed(region):
                body.append(f"//#endregion {r}")
            for r in it.region:
                body.append(f"//#region {r}")
            region = it.region
        body.append(text.strip("\n"))
    for r in reversed(region):
        body.append(f"//#endregion {r}")
    out, last = [], 0
    for it in items:
        if it.i in chosen:
            out.append(src[last:it.start])
            last = it.end
        elif it.kind in DEFINITION and it.name in publicize:
            out.append(src[last:it.start])
            out.append(bump(it))
            last = it.end
    out.append(src[last:])
    remaining = "".join(out)
    remaining = re.sub(r"(?m)^[ \t]*//#region ([^\n]*)\n(?:[ \t]*\n)*[ \t]*//#endregion \1[ \t]*\n", "", remaining)
    shim = ["//#region 🚚️Moved"] + render_use([(dst_mod.split("::") + [n], None) for n in sorted(names) if items[[i for i in chosen if items[i].name == n][0]].text and n in names], "pub ") + ["//#endregion 🚚️Moved"]
    doc = re.match(r"((?:[ \t]*//![^\n]*\n)+)", remaining)
    at = doc.end() if doc else 0
    mount = [move["mount"]] if move.get("mount") else []
    remaining = remaining[:at] + "\n".join(shim + mount) + "\n" + remaining[at:]
    if os.path.exists(dst_path):
        target = open(dst_path, encoding="utf-8").read()
    else:
        target = move.get("doc", "//! 🧩️ Moved items.") + "\n"
    tdoc = re.match(r"((?:[ \t]*//![^\n]*\n)+)", target)
    tat = tdoc.end() if tdoc else 0
    have = set(re.findall(r"(?m)^\s*(?:pub\s+)?use\s+[^;]*;", target))
    defined_there = set(re.findall(r"(?m)^\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:(?:const|async|unsafe)\s+)*(?:fn|struct|enum|trait|type|const|static|union|mod)\s+(" + IDENT + r")", target)) | names
    bound_there = {alias or segs[-1] for m in re.finditer(r"(?m)^\s*(?:pub(?:\s*\([^)]*\))?\s+)?use\s+([^;]*);", target) for segs, alias in leaves([], m.group(1))}
    target_uses = [(segs, alias) for segs, alias in target_uses if (alias or segs[-1]) not in defined_there | bound_there]
    target = drop_bound(target, names)
    new_uses = [line for line in render_use(target_uses) if line not in have]
    target = target[:tat] + ("\n".join(new_uses) + "\n" if new_uses else "") + target[tat:]
    target = target.rstrip("\n") + "\n\n" + "\n".join(body) + "\n"
    os.makedirs(os.path.dirname(dst_path), exist_ok=True)
    open(src_path, "w", encoding="utf-8").write(remaining)
    open(dst_path, "w", encoding="utf-8").write(target)
    return [{"from": src_mod, "to": dst_mod, "items": sorted(names)}]


def main():
    mode, spec_path = sys.argv[1], sys.argv[2]
    spec = json.load(open(spec_path, encoding="utf-8"))
    record = []
    for move in spec["moves"]:
        record += run(move, mode)
    if mode == "apply":
        out = os.path.splitext(spec_path)[0] + ".moved.json"
        previous = json.load(open(out, encoding="utf-8")) if os.path.exists(out) else []
        json.dump(previous + record, open(out, "w", encoding="utf-8"), indent=1, ensure_ascii=False)


main()
