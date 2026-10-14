#!/usr/bin/env python3
"""🧭️ Retargets every path through a module split by `🧩️item-modules.py` to the child module that owns the item, then
drops the parent's temporary `pub use <child>::*;` re-exports.

For every Rust file under the roots, each spelling of the split module (`crate::<module>` inside the owning crate,
`<crate ident>::<module>` outside it, `super` in files mounted directly below the module, and every alias module that
glob re-exports it) is rewritten: `<M>::Item` paths become `<M>::<child>::Item`, `<M>::{..}` groups are split into one
group per owner, and `use <M>::*;` globs become explicit per-child imports of the names the file uses (a `super::*`
glob is kept for the parent's own items).

usage: 🧭️retarget-split.py <spec.json> <crate ident> <crate root.rs> [--alias <module>]... [--write] <root>...
"""
import collections, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
ITEM_SPLIT = os.path.join(HERE, "..", "e3-format-split", "🔀️item-split.py")
ns = {"__file__": ITEM_SPLIT}
exec(compile(open(ITEM_SPLIT, encoding="utf-8").read().split("\nif __name__")[0], ITEM_SPLIT, "exec"), ns)
parse, strip = ns["parse"], ns["strip"]
CUT = os.path.join(HERE, "✂️crate-cut.py")
cut = {"__file__": CUT}
exec(compile(open(CUT, encoding="utf-8").read().rsplit("\nmain()", 1)[0].replace("\naggregator_layer = None", ""), CUT, "exec"), cut)
Crate, offset_strip = cut["Crate"], cut["strip"]
IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
SEP = r"\s*::\s*"
DEFINITION = ("fn", "struct", "enum", "trait", "type", "const", "static", "mod", "union")


def owners(spec):
    parent = os.path.join(REPO, spec["file"])
    table = {}
    for child in [None] + spec["children"]:
        path = parent if child is None else os.path.join(os.path.dirname(parent), child["dir"], "🦀️.rs")
        src, items = parse(path)
        for it in items:
            if it.kind in DEFINITION and not (child is None and it.kind == "mod" and it.name in {c["name"] for c in spec["children"]}):
                table.setdefault(it.name, None if child is None else child["name"])
            elif it.kind == "macro":
                head = re.match(r"\s*" + IDENT + r"\s*!\s*[\(\{\[]([^;]*?)(?:=>|;)", re.sub(r"^(\s*(#!?\[[^\]]*\]|///[^\n]*|//[^\n]*)\s*)*", "", it.stripped), re.S)
                if head:
                    for name in re.findall(r"\b" + IDENT + r"\b", re.sub(r"<[^<>]*>", "", head.group(1))):
                        table.setdefault(name, None if child is None else child["name"])
    return table


def split_top(text):
    parts, depth, cur = [], 0, ""
    for ch in text:
        if ch == "{":
            depth += 1
        if ch == "}":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur)
    return [p.strip() for p in parts if p.strip()]


def rewrite(raw, spellings, table, children, keep_glob):
    text = offset_strip(raw)
    used = set(re.findall(r"\b" + IDENT + r"\b", text))
    edits = []
    for spelling in spellings:
        head = re.escape(spelling).replace(r"::", SEP)
        use = re.compile(r"(?m)^(?P<attrs>(?:[ \t]*#\[[^\n]*\]\n)*)(?P<indent>[ \t]*)(?P<vis>pub(?:\s*\([^)]*\))?\s+)?use\s+(?P<path>" + head + r")" + SEP + r"(?P<body>\*|\{[^;]*\}|" + IDENT + r"(?:\s+as\s+" + IDENT + r")?)\s*;")
        covered = []
        for m in use.finditer(text):
            body = m.group("body")
            lead = f"{m.group('attrs')}{m.group('indent')}{m.group('vis') or ''}use {spelling}"
            groups = collections.OrderedDict()
            if body == "*":
                wanted = sorted(name for name, owner in table.items() if name in used)
                for name in wanted:
                    groups.setdefault(table[name], []).append(name)
                if keep_glob or None in groups:
                    groups.pop(None, None)
                    lines = [f"{lead}::*;"]
                else:
                    lines = []
                lines += [f"{lead}::{child}::{{{', '.join(names)}}};" for child, names in groups.items() if child is not None]
                if not lines:
                    lines = [f"{lead}::*;"]
            else:
                parts = split_top(body[1:-1]) if body.startswith("{") else [body]
                for part in parts:
                    name = re.split(r"\s*::\s*|\s+as\s+|\s*\{", part)[0].strip()
                    groups.setdefault(table.get(name) if name not in children else "@" , []).append(part)
                lines = []
                for child, names in groups.items():
                    prefix = f"{lead}" if child in (None, "@") else f"{lead}::{child}"
                    lines.append(f"{prefix}::{names[0]};" if len(names) == 1 and "{" not in names[0] else f"{prefix}::{{{', '.join(names)}}};")
            replacement = "\n".join(lines)
            if replacement != raw[m.start():m.end()]:
                edits.append((m.start(), m.end(), replacement))
            covered.append((m.start(), m.end()))
        path = re.compile(r"(?<![\w:])(?P<path>" + head + r")" + SEP + r"(?P<name>" + IDENT + r")(?![\w])")
        for m in path.finditer(text):
            if any(a <= m.start() < b for a, b in covered):
                continue
            name = m.group("name")
            owner = table.get(name)
            if owner is None or name in children:
                continue
            edits.append((m.start("name"), m.start("name"), f"{owner}::"))
    edits.sort()
    out, last = [], 0
    for a, b, rep in edits:
        if a < last:
            continue
        out.append(raw[last:a])
        out.append(rep)
        last = b
    out.append(raw[last:])
    return "".join(out)


def main():
    args = sys.argv[1:]
    write = "--write" in args
    aliases = [args[i + 1] for i, a in enumerate(args) if a == "--alias"]
    positional = [a for i, a in enumerate(args) if not a.startswith("--") and (i == 0 or args[i - 1] != "--alias")]
    spec_path, ident, root = positional[:3]
    roots = positional[3:]
    spec = json.load(open(spec_path, encoding="utf-8"))
    table = owners(spec)
    children = {c["name"] for c in spec["children"]}
    module = spec["module"].removeprefix("crate::")
    crate = Crate(os.path.join(REPO, root))
    parent = os.path.normpath(os.path.join(REPO, spec["file"]))
    modpath = tuple(module.split("::"))
    changed = 0
    seen = set()
    files = []
    for base in roots:
        for dirpath, dirnames, filenames in os.walk(os.path.join(REPO, base)):
            dirnames[:] = [d for d in dirnames if d not in ("node_modules", "target", "🗑️generated", ".git")]
            files += [os.path.normpath(os.path.join(dirpath, f)) for f in filenames if f.endswith(".rs")]
    for path in files:
        if path in seen:
            continue
        seen.add(path)
        try:
            raw = open(path, encoding="utf-8").read()
        except (FileNotFoundError, UnicodeDecodeError):
            continue
        inside = path in crate.files
        spellings = []
        if inside:
            spellings += [f"crate::{module}"] + [f"crate::{alias}" for alias in aliases]
            mp = crate.files[path]
            if path != parent and mp[:len(modpath)] == modpath and len([s for s in mp if s != "component"]) == len(modpath) + 1:
                spellings.append("super")
        elif ident in raw:
            spellings += [f"{ident}::{module}"] + [f"{ident}::{alias}" for alias in aliases]
        if not spellings or path == parent or any(path == os.path.join(os.path.dirname(parent), c["dir"], "🦀️.rs") for c in spec["children"]):
            continue
        new = rewrite(raw, spellings, table, children, keep_glob="super" in spellings)
        if new != raw:
            changed += 1
            print(f"rewrite {os.path.relpath(path, REPO)}")
            if write:
                open(path, "w", encoding="utf-8").write(new)
    if write:
        text = open(parent, encoding="utf-8").read()
        text = re.sub(r"(?m)^pub use (?:" + "|".join(re.escape(c) for c in children) + r")::\*;\n", "", text)
        open(parent, "w", encoding="utf-8").write(text)
    print(f"files changed: {changed}")


main()
