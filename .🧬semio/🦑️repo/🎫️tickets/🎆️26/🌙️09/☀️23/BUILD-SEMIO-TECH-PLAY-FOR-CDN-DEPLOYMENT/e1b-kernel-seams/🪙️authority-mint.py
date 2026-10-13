#!/usr/bin/env python3
"""🪙️ Rewrites every `ArtifactStoreOneItemLiveAuthority { .. }` struct literal outside its owning module into the
module's `ArtifactStoreOneItemLiveAuthority::mint(..)` constructor, arguments in declaration order (wave E1b).
Usage: authority-mint.py FILE [FILE ...]
"""
import re, sys

FIELDS = ["operation", "generation", "base_revision", "base_applied_edit_count", "next_sequence_number", "next_clock", "actor", "line", "group_id", "stamped_edit_id"]


def split_top(body):
    parts, depth, start = [], 0, 0
    for i, c in enumerate(body):
        if c in "([{": depth += 1
        elif c in ")]}": depth -= 1
        elif c == "," and depth == 0: parts.append(body[start:i]); start = i + 1
    if body[start:].strip(): parts.append(body[start:])
    return parts


total = 0
for f in sys.argv[1:]:
    src = open(f, encoding="utf-8").read()
    out, pos = [], 0
    for m in re.finditer(r"(?<!for )(?<!impl )ArtifactStoreOneItemLiveAuthority\s*\{", src):
        if m.start() < pos: continue
        j, depth = m.end() - 1, 0
        for k in range(j, len(src)):
            if src[k] == "{": depth += 1
            elif src[k] == "}":
                depth -= 1
                if depth == 0: break
        body = src[j + 1:k]
        values = {}
        for part in split_top(body):
            name, _, expr = part.strip().partition(":")
            values[name.strip()] = expr.strip() if expr else name.strip()
        if set(values) != set(FIELDS): sys.exit(f"{f}: literal fields {sorted(values)}")
        if "\n" in body:
            indent = re.search(r"\n([ \t]*)\S", body).group(1)
            close = re.search(r"\n([ \t]*)$", body)
            args = "".join(f"\n{indent}{values[n]}," for n in FIELDS) + "\n" + (close.group(1) if close else "")
        else:
            args = ", ".join(values[n] for n in FIELDS)
        out.append(src[pos:m.start()] + f"ArtifactStoreOneItemLiveAuthority::mint({args})")
        pos = k + 1
        total += 1
    out.append(src[pos:])
    open(f, "w", encoding="utf-8").write("".join(out))
print(f"[mint] {total} literals rewritten")
