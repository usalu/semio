#!/usr/bin/env python3
"""🔍️ Finds every `ArtifactEnvelope` bound to a local and then dropped without retirement.

`ArtifactEnvelope::drop` aborts the guest unless `into_owners()` detached its nested owners first,
so a binding that is only *borrowed* (typically `print_document_spr(&envelope)`) and then falls out
of scope is a runtime abort with no compile-time signal. See `📓️envelope-drop-abort.md`.
"""
import io, os, re, sys

ROOTS = ["✏️s", "🧰️framework"]
BIND = re.compile(r"^\s*let\s+(?:mut\s+)?(\w+)\s*=\s*(?:[\w:]*\s*)?(?:store::)?create_document_envelope")
CONSUMERS = ("into_owners", "Store::new", "SpaceHost::new", "retire_envelope", "CompletedRecordOwner::new", ".reset(")

def scan(path):
    lines = io.open(path, encoding="utf-8", errors="replace").read().split("\n")
    out = []
    for i, line in enumerate(lines):
        m = BIND.match(line)
        if not m:
            continue
        name = m.group(1)
        indent = len(line) - len(line.lstrip())
        consumed, borrowed = False, False
        for j in range(i + 1, min(i + 60, len(lines))):
            nxt = lines[j]
            if nxt.strip() and (len(nxt) - len(nxt.lstrip())) < indent:
                break
            if name not in nxt:
                continue
            if any(c in nxt for c in CONSUMERS):
                consumed = True
                break
            if re.search(r"\(\s*&" + name + r"\b", nxt) or re.search(r"&" + name + r"\b", nxt):
                borrowed = True
                continue
            consumed = True
            break
        if not consumed and borrowed:
            out.append((i + 1, line.strip()[:110]))
    return out

total = 0
for root in ROOTS:
    for dirpath, dirnames, filenames in os.walk(root):
        if "target" in dirpath.split(os.sep):
            dirnames[:] = []
            continue
        for name in filenames:
            if not name.endswith(".rs"):
                continue
            path = os.path.join(dirpath, name)
            for line_no, text in scan(path):
                print(f"{path}|{line_no}|{text}")
                total += 1
print(f"# dropped-without-retirement bindings: {total}", file=sys.stderr)
