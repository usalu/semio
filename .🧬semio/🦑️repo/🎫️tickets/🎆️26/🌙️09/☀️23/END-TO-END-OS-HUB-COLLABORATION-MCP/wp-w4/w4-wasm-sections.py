#!/usr/bin/env python3
"""📏️ W4 (14c): section sizes of a wasm component (and its nested core modules) — where a component's bytes sit."""
import sys

def leb(b, i):
    r = s = 0
    while True:
        x = b[i]; i += 1
        r |= (x & 0x7F) << s; s += 7
        if x < 0x80: return r, i

def walk(b, depth, out):
    i = 8
    while i < len(b):
        sid = b[i]; size, j = leb(b, i + 1); body = b[j:j + size]
        if sid == 0:
            n, k = leb(body, 0); key = f"{'  ' * depth}custom:{body[k:k + n].decode(errors='replace')}"
        elif depth == 0 and sid == 1:
            key = f"core-module"; walk(body, depth + 1, out)
        else:
            key = f"{'  ' * depth}{'component' if depth == 0 else 'core'}:{sid}"
        out[key] = out.get(key, 0) + size
        i = j + size

data = open(sys.argv[1], 'rb').read()
out = {}
walk(data, 0, out)
print(f"total {len(data)}")
for k, v in sorted(out.items(), key=lambda kv: -kv[1])[:14]: print(f"{v:>12} {k}")
