"""🧭️ Resolve every `#[path = "…"]` and `include_str!("…")` under the raster plugin against disk.

Run from the repo root: `python3 <this file>`. Exits non-zero when any mount dangles.
"""

import os
import re
import sys

ROOT = "✏️s/🔌️plugins/🖨️raster"
PATH_ATTR = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
INCLUDE = re.compile(r'include_(?:str|bytes)!\(\s*"([^"]+)"\s*\)')


def rust_files(root):
    for base, _dirs, names in os.walk(root):
        for name in names:
            if name.endswith(".rs"):
                yield os.path.join(base, name)


def main():
    paths = selfs = includes = 0
    dangling = []
    for file in sorted(rust_files(ROOT)):
        here = os.path.dirname(file)
        text = open(file, encoding="utf-8").read()
        for target in PATH_ATTR.findall(text):
            paths += 1
            if target == ".":
                selfs += 1
                continue
            if not os.path.exists(os.path.normpath(os.path.join(here, target))):
                dangling.append(("path", file, target))
        for target in INCLUDE.findall(text):
            includes += 1
            if not os.path.exists(os.path.normpath(os.path.join(here, target))):
                dangling.append(("include", file, target))
    print(f"#[path] attributes: {paths} (self-markers \".\": {selfs}, real targets: {paths - selfs})")
    print(f"include_str!/include_bytes!: {includes}")
    print(f"dangling: {len(dangling)}")
    for kind, file, target in dangling:
        print(f"  {kind} {file} -> {target}")
    return 1 if dangling else 0


if __name__ == "__main__":
    sys.exit(main())
