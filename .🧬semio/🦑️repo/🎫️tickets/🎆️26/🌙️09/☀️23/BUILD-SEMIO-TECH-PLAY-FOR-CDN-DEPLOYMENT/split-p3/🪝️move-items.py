#!/usr/bin/env python3
"""🪝️ Moves named top-level items (with their doc comments) from one Rust file into a region of another, made `pub`.

usage: 🪝️move-items.py <from.rs> <to.rs> <region name> <item> [<item>...]
The region `//#region <name>` is created after the target's leading `extern crate`/`use` header when absent. Each item
is a `fn`, `const` or `static` at column 0 whose body ends at the first column-0 `}` (or `;` for consts).
"""
import re, sys

src_path, dst_path, region, names = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4:]
src, dst = open(src_path, encoding="utf-8").read(), open(dst_path, encoding="utf-8").read()
moved = []
for name in names:
    m = re.search(r"(?m)^(?:pub(?:\([^)]*\))? )?(?:fn|const|static) " + re.escape(name) + r"\b", src)
    if not m:
        sys.exit(f"missing {name}")
    start = m.start()
    while True:
        prev = src.rfind("\n", 0, start - 1) + 1
        if start > 0 and src[prev:start].lstrip().startswith("///"):
            start = prev
        else:
            break
    if re.match(r"(?:pub(?:\([^)]*\))? )?(?:const|static) ", src[m.start():]):
        end = re.compile(r";\n").search(src, m.start()).end() - 1
    else:
        end = src.index("\n}\n", m.start()) + 2
    item = src[start:end].rstrip("\n") + "\n"
    item = re.sub(r"(?m)^pub\(crate\) ", "pub ", item, count=0)
    if not re.search(r"(?m)^pub ", item[item.index(name) - 20:]):
        item = re.sub(r"(?m)^(fn|const|static) ", r"pub \1 ", item)
    moved.append(item)
    src = src[:start] + src[end:].lstrip("\n")
block = "\n".join(moved)
open_tag, close_tag = f"//#region {region}\n", f"//#endregion {region}\n"
if open_tag in dst:
    i = dst.index(close_tag)
    dst = dst[:i] + block + dst[i:]
else:
    lines = dst.split("\n")
    k = 0
    for j, line in enumerate(lines):
        if line.startswith(("//!", "extern crate", "use ", "#![")) or not line.strip():
            k = j + 1
        else:
            break
    dst = "\n".join(lines[:k]).rstrip("\n") + "\n\n" + open_tag + block + close_tag + "\n" + "\n".join(lines[k:]).lstrip("\n")
open(src_path, "w", encoding="utf-8").write(re.sub(r"\n{3,}", "\n\n", src))
open(dst_path, "w", encoding="utf-8").write(dst)
print(f"moved {len(moved)} items")
