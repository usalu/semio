#!/usr/bin/env python3
"""🩺 list_delta! marker rule: every `by <Delta>Key` has a `struct` and `KeyOf<Row>` impl in its artifact; no `key: id` in artifacts."""
import os, re
here = os.getcwd()
while not os.path.isdir(os.path.join(here, "✏️s", "🔌️plugins", "📕️norm")): here = os.path.dirname(here)
arts = os.path.join(here, "✏️s", "🔌️plugins", "📕️norm", "🗿️artifacts")
out = set()
for a in sorted(os.listdir(arts)):
    root = os.path.join(arts, a); texts = {}
    for d, ds, fs in os.walk(root):
        ds[:] = [x for x in ds if x not in ("target", "node_modules", "🏭️bridge", "👽️guest")]
        if os.path.basename(d) not in ("🔺️diff",) and "🔺️diff" not in d and "diff" not in d: continue
        for f in fs:
            if f.endswith(".rs"): texts[os.path.join(d, f)] = open(os.path.join(d, f), encoding="utf-8").read()
    allt = "\n".join(texts.values())
    for f, t in texts.items():
        for m in re.finditer(r'list_delta!\s*\{(.*?)\n\}', t, re.S):
            for blk in re.finditer(r'pub\s+(\w+)\s*\{[^}]*\}', m.group(1)):
                b = blk.group(0)
                if re.search(r'key:\s*id\b', b): out.add(f"{os.path.relpath(f, arts)}: {blk.group(1)} uses `key: id` (E0117)")
                by = re.search(r'by\s+(\w+)', b)
                if by:
                    k = by.group(1)
                    if not re.search(r'struct\s+' + k + r'\b', allt): out.add(f"{os.path.relpath(f, arts)}: marker `{k}` struct missing")
                    if not re.search(r'KeyOf<[^>]*>\s*for\s+' + k + r'\b', allt): out.add(f"{os.path.relpath(f, arts)}: marker `{k}` has no KeyOf impl")
print("\n".join(sorted(out))); print(len(out), "issues")
