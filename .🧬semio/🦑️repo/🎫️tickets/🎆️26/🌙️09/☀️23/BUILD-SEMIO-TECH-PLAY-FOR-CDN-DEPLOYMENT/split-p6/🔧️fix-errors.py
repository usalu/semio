#!/usr/bin/env python3
"""🔧️ Applies the mechanical repairs rustc proposes after a crate cut, read from `cargo --message-format=json` output.

Only files under the given roots are touched. Repairs, each applied at most once per location:
- private item / method / associated item (E0603, E0624): the defining item becomes `pub` at the span rustc names;
- private field (E0616) and private-field struct literals (E0451): the field becomes `pub` in its struct definition;
- unresolved names (E0412, E0425, E0433, E0422, E0423, E0432, E0531, E0574, E0599 import hints) with exactly one
  "consider importing" suggestion: that `use` line is inserted where rustc proposes;
- anything else is listed for hand repair.

usage: 🔧️fix-errors.py <cargo-json-log> <root>... [--write]
"""
import collections, json, os, re, sys

log, roots = sys.argv[1], [os.path.abspath(r) for r in sys.argv[2:] if not r.startswith("--")]
write = "--write" in sys.argv
edits = collections.defaultdict(set)
inserts = collections.defaultdict(set)
manual = collections.Counter()
seen_spans = set()


def inside(path):
    return any(os.path.abspath(path).startswith(r) for r in roots)


def make_pub(path, line, col):
    """➕️ Records `pub` for the item/field starting at (line, col) (1-based)."""
    edits[path].add(("pub", line, col))


for raw in open(log, encoding="utf-8"):
    raw = raw.strip()
    if not raw.startswith("{"):
        continue
    try:
        msg = json.loads(raw)
    except ValueError:
        continue
    if msg.get("reason") != "compiler-message":
        continue
    m = msg["message"]
    if m.get("level") != "error":
        continue
    code = (m.get("code") or {}).get("code")
    primary = next((s for s in m.get("spans", []) if s.get("is_primary")), None)
    handled = False
    if code in ("E0603", "E0624"):
        for child in m.get("children", []):
            for span in child.get("spans", []):
                if inside(span["file_name"]) and ("defined here" in (child.get("message") or "") or span.get("label")):
                    make_pub(span["file_name"], span["line_start"], span["column_start"])
                    handled = True
        for span in m.get("spans", []):
            if not span.get("is_primary") and inside(span["file_name"]) and "defined here" in (span.get("label") or ""):
                make_pub(span["file_name"], span["line_start"], span["column_start"])
                handled = True
    if code in ("E0616", "E0451"):
        text = m.get("message", "")
        f = re.search(r"field `(\w+)` of (?:struct|enum variant|union) `([\w:]+)", text)
        if f:
            edits["@field"].add((f.group(2).split("::")[-1], f.group(1)))
            handled = True
    if not handled:
        candidates = []
        for child in m.get("children", []):
            for span in child.get("spans", []):
                if span.get("suggested_replacement") and re.match(r"\s*use\s", span["suggested_replacement"]) and inside(span["file_name"]):
                    candidates.append((span["file_name"], span["line_start"], span["suggested_replacement"]))
        if candidates and len({c[2].strip() for c in candidates}) == 1:
            path, line, text = candidates[0]
            inserts[path].add((line, text if text.endswith("\n") else text + "\n"))
            handled = True
    if not handled:
        where = f"{os.path.relpath(primary['file_name'])}:{primary['line_start']}" if primary else "?"
        manual[(code, m.get("message", "").split("\n")[0][:160], where)] += 1

fields = edits.pop("@field", set())
if fields:
    for root in roots:
        for dirpath, _dirs, files in os.walk(root):
            if "target" in dirpath or "🗑️generated" in dirpath:
                continue
            for name in files:
                if not name.endswith(".rs"):
                    continue
                path = os.path.join(dirpath, name)
                text = open(path, encoding="utf-8").read()
                for struct, field in fields:
                    for s in re.finditer(r"\bstruct\s+" + struct + r"\b[^{;(]*\{", text):
                        depth, j = 1, s.end()
                        while j < len(text) and depth:
                            depth += {"{": 1, "}": -1}.get(text[j], 0)
                            j += 1
                        body = text[s.end():j]
                        fm = re.search(r"(?m)^(\s*)(?:pub\s*\([^)]*\)\s+)?(" + field + r")\s*:", body)
                        if fm and not re.match(r"\s*pub\s", body[fm.start():].lstrip("\n")):
                            off = s.end() + fm.start(2)
                            line = text.count("\n", 0, off) + 1
                            col = off - text.rfind("\n", 0, off)
                            edits[path].add(("pub", line, col))

changed = 0
for path in set(edits) | set(inserts):
    lines = open(path, encoding="utf-8").read().split("\n")
    for kind, line, col in sorted(edits.get(path, ()), key=lambda e: (-e[1], -e[2])):
        text = lines[line - 1]
        head, tail = text[:col - 1], text[col - 1:]
        restricted = re.match(r"pub\s*\([^)]*\)\s*", tail)
        if restricted:
            tail = "pub " + tail[restricted.end():]
        elif re.match(r"pub\b", tail):
            pass
        else:
            vis = re.search(r"pub\s*\([^)]*\)\s*$", head)
            if vis:
                head = head[:vis.start()] + "pub "
            elif not re.search(r"\bpub\s+$", head):
                item = re.match(r"((?:const\s+|async\s+|unsafe\s+|extern\s+\"[^\"]*\"\s+)*(?:fn|struct|enum|trait|type|const|static|mod|union|use)\b)", tail)
                tail = "pub " + tail
        lines[line - 1] = head + tail
    for line, text in sorted(inserts.get(path, ()), key=lambda e: -e[0]):
        if text.strip() not in "\n".join(lines):
            lines.insert(line - 1, text.rstrip("\n"))
    changed += 1
    if write:
        open(path, "w", encoding="utf-8").write("\n".join(lines))
print(f"files repaired: {changed} (pub: {sum(len(v) for v in edits.values())}, imports: {sum(len(v) for v in inserts.values())}, fields: {len(fields)})")
for (code, text, where), count in sorted(manual.items(), key=lambda kv: (kv[0][0] or "", kv[0][2])):
    print(f"MANUAL {code} x{count} {where}: {text}")
