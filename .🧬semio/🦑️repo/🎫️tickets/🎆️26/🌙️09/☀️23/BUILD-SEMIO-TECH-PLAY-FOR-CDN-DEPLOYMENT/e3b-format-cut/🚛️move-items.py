#!/usr/bin/env python3
"""🚛️ Moves named top-level items (with their docs, attributes and `impl` blocks) from one module file to another and
rewrites every path that names them.

The items are cut from the source file and appended to the target file (inside the target's last `//#region` when the
spec names one). Every `crate::<from>::<item>` / `<ident>::<from>::<item>` path and every `use <from>::{.., item, ..}`
group under the roots is retargeted to `<to>`; the source and target files get the imports the moved and the remaining
items need from each other.

usage: 🚛️move-items.py <spec.json> [--write] <root>...
spec: {"ident": crate ident, "source": file, "target": file, "from": "a::b", "to": "a::b::c", "items": [name...]}
"""
import collections, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
ITEM_SPLIT = os.path.join(HERE, "..", "e3-format-split", "🔀️item-split.py")
ns = {"__file__": ITEM_SPLIT}
exec(compile(open(ITEM_SPLIT, encoding="utf-8").read().split("\nif __name__")[0], ITEM_SPLIT, "exec"), ns)
parse = ns["parse"]
SEP = r"\s*::\s*"
IDENT = r"[A-Za-z_][A-Za-z0-9_]*"


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


def main():
    spec = json.load(open(sys.argv[1], encoding="utf-8"))
    write = "--write" in sys.argv
    roots = [a for a in sys.argv[2:] if not a.startswith("--")]
    source = os.path.join(REPO, spec["source"])
    target = os.path.join(REPO, spec["target"])
    names = set(spec["items"])
    src, items = parse(source)
    moved = [it for it in items if (it.kind != "impl" and it.name in names) or (it.kind == "impl" and it.target in names)]
    missing = names - {it.name for it in moved if it.kind != "impl"}
    if missing:
        raise SystemExit(f"items not found: {sorted(missing)}")
    remaining = src
    for it in sorted(moved, key=lambda it: -it.start):
        remaining = remaining[:it.start] + remaining[it.end:]
    block = "\n\n".join(it.text.strip("\n") for it in moved)
    target_text = open(target, encoding="utf-8").read()
    used_by_moved = set(re.findall(r"\b" + IDENT + r"\b", " ".join(it.stripped for it in moved)))
    source_names = {it.name for it in items if it.kind not in ("impl", "use", "?", "macro") and it.name not in names}
    needs_from_source = sorted(used_by_moved & source_names)
    source_uses = [it.text.strip("\n") for it in items if it.kind == "use"]
    carried = [u for u in source_uses if set(re.findall(r"\b" + IDENT + r"\b", re.sub(r"^\s*use\s+", "", u))) & used_by_moved and "#[cfg(test)]" not in u and not re.search(r"use\s+crate::" + re.escape(spec["to"]) + r"::(?:\{|" + IDENT + r"\s*;)", u)]
    header = carried + ([f"use crate::{spec['from']}::{{{', '.join(needs_from_source)}}};"] if needs_from_source else [])
    uses = list(re.finditer(r"(?m)^use [^\n]*;\n", target_text))
    doc = re.match(r"(?://![^\n]*\n)*", target_text)
    at = uses[-1].end() if uses else doc.end()
    existing = set(m.group(0).strip() for m in uses)
    header = [h for h in header if h.strip() not in existing]
    target_text = target_text[:at] + "".join(h + "\n" for h in header) + target_text[at:]
    target_text = target_text.rstrip("\n") + "\n\n" + block + "\n"
    remaining_uses_names = set(re.findall(r"\b" + IDENT + r"\b", remaining))
    back = sorted(n for n in names if re.search(r"\b" + re.escape(n) + r"\b", re.sub(r"(?m)^use [^\n]*\n", "", remaining)))
    if back:
        line = f"use crate::{spec['to']}::{{{', '.join(back)}}};\n"
        uses = list(re.finditer(r"(?m)^use [^\n]*;\n", remaining))
        doc = re.match(r"(?://![^\n]*\n)*", remaining)
        at = uses[-1].end() if uses else doc.end()
        remaining = remaining[:at] + line + remaining[at:]
    changed = {source: remaining, target: target_text}
    head = re.escape(spec["from"]).replace(r"::", SEP)
    files = []
    for base in roots:
        for dirpath, dirnames, filenames in os.walk(os.path.join(REPO, base)):
            dirnames[:] = [d for d in dirnames if d not in ("node_modules", "target", "🗑️generated", ".git")]
            files += [os.path.join(dirpath, f) for f in filenames if f.endswith(".rs")]
    for path in files:
        try:
            raw = changed.get(path) or open(path, encoding="utf-8").read()
        except (FileNotFoundError, UnicodeDecodeError):
            continue
        text = raw
        group = re.compile(r"(?m)^(?P<lead>[ \t]*(?:#\[[^\n]*\]\n[ \t]*)*(?:pub(?:\([^)]*\))?\s+)?use\s+(?P<crate>crate|" + re.escape(spec["ident"]) + r")" + SEP + head + r")" + SEP + r"\{(?P<body>[^;]*)\}\s*;")
        def regroup(m):
            parts = split_top(m.group("body"))
            stay = [p for p in parts if re.split(r"\s*::\s*|\s+as\s+", p)[0] not in names]
            go = [p for p in parts if re.split(r"\s*::\s*|\s+as\s+", p)[0] in names]
            if not go:
                return m.group(0)
            lead = m.group("lead")
            new_lead = re.sub(head + r"$", spec["to"], lead)
            out = []
            if stay:
                out.append(f"{lead}::{{{', '.join(stay)}}};")
            out.append(f"{new_lead}::{{{', '.join(go)}}};")
            return "\n".join(out)
        text = group.sub(regroup, text)
        path_re = re.compile(r"(?<![\w:])(?P<crate>crate|" + re.escape(spec["ident"]) + r")" + SEP + head + SEP + r"(?P<name>" + "|".join(re.escape(n) for n in sorted(names)) + r")(?![\w])")
        text = path_re.sub(lambda m: f"{m.group('crate')}::{spec['to']}::{m.group('name')}", text)
        if text != raw:
            changed[path] = text
    own = re.compile(r"(?m)^[ \t]*use\s+crate" + SEP + re.escape(spec["to"]).replace(r"::", SEP) + SEP + r"(?:\{(?P<body>[^;]*)\}|(?P<one>" + IDENT + r"))\s*;\n")
    def drop_self(m):
        parts = split_top(m.group("body")) if m.group("body") is not None else [m.group("one")]
        keep = [p for p in parts if p not in names]
        return "" if not keep else f"use crate::{spec['to']}::{{{', '.join(keep)}}};\n"
    changed[target] = own.sub(drop_self, changed[target])
    for path in sorted(changed):
        print(f"rewrite {os.path.relpath(path, REPO)}")
    if write:
        for path, text in changed.items():
            open(path, "w", encoding="utf-8").write(text)


main()
