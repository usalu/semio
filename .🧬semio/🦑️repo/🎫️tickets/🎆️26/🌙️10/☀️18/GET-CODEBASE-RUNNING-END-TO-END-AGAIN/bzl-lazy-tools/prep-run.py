#!/usr/bin/env python3
# Makes runnable copies of the staged adapter and selection next to the harness: relative specifiers re-pointed, staged replacements substituted for live files.
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
B = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
BZ = B + "/🔧️tools/🏗️bazel"
S = HERE + "/staged"
REDIRECT = {
    BZ + "/🧬️schema/🔣️.json": S + "/schema.json",
    BZ + "/🧭️commands/🟦️.ts": S + "/commands.ts",
    BZ + "/🎯️affected/🟦️.ts": S + "/affected.ts",
    B + "/🧭️selection/🟦️.ts": HERE + "/run/selection.ts",
    BZ + "/📜️script.ts": HERE + "/run/adapter.ts",
}
FILES = [(S + "/neutral.ts", B + "/📜️script.ts", HERE + "/run/neutral.ts"), (S + "/script.ts", BZ + "/📜️script.ts", HERE + "/run/adapter.ts"), (S + "/selection.ts", B + "/🧭️selection/🟦️.ts", HERE + "/run/selection.ts")]
SPEC = re.compile(r"""(from\s*|import\s*\(\s*)(["'])(\.{1,2}/[^"']+)\2""")

def main():
    os.makedirs(HERE + "/run", exist_ok=True)
    for staged, live, out in FILES:
        text = open(staged, encoding="utf-8").read()
        def fix(m):
            target = os.path.normpath(os.path.join(os.path.dirname(live), m.group(3)))
            target = REDIRECT.get(target, target)
            rel = os.path.relpath(target, os.path.dirname(out))
            if not rel.startswith("."): rel = "./" + rel
            return m.group(1) + m.group(2) + rel + m.group(2)
        open(out, "w", encoding="utf-8").write(SPEC.sub(fix, text))
        print(out, len(text))

if __name__ == "__main__": main()
