#!/usr/bin/env python3
# Copies the neutral selection module (live or staged) with its test folder under /private/tmp so the existing selection tests run unchanged against it.
import os, re, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
SEL = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️selection"
MODE = sys.argv[1]
OUT = "/private/tmp/bzlazy/sel-" + MODE + "/selection"
SPEC = re.compile(r"""(from\s*|import\s*\(\s*|new URL\(\s*)(["'])(\.{1,2}/[^"']+)\2""")
def rewrite(text, live, out):
    def fix(m):
        target = os.path.normpath(os.path.join(os.path.dirname(live), m.group(3)))
        if target == SEL + "/🟦️.ts": target = OUT + "/🟦️.ts"
        elif target.startswith(SEL + "/🧪️tests/"): target = OUT + target[len(SEL):]
        rel = os.path.relpath(target, os.path.dirname(out))
        if not rel.startswith("."): rel = "./" + rel
        return m.group(1) + m.group(2) + rel + m.group(2)
    return SPEC.sub(fix, text)
if os.path.exists(OUT): shutil.rmtree(OUT)
os.makedirs(OUT + "/🧪️tests")
src = HERE + "/staged/selection.ts" if MODE == "staged" else SEL + "/🟦️.ts"
open(OUT + "/🟦️.ts", "w", encoding="utf-8").write(rewrite(open(src, encoding="utf-8").read(), SEL + "/🟦️.ts", OUT + "/🟦️.ts"))
for f in os.listdir(SEL + "/🧪️tests"):
    s = SEL + "/🧪️tests/" + f
    if os.path.isfile(s):
        d = OUT + "/🧪️tests/" + f
        if f == "🟦️.ts" and MODE == "staged": open(d, "w", encoding="utf-8").write(rewrite(open(HERE + "/staged/selection-test.ts", encoding="utf-8").read(), s, d))
        elif f.endswith(".ts"): open(d, "w", encoding="utf-8").write(rewrite(open(s, encoding="utf-8").read(), s, d))
        else: shutil.copy(s, d)
print(OUT)
