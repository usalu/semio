"""Replaces engine-dependent Math functions with the portable ones in the model and drawing pipelines; prints every touched file."""
import os, re, sys
R = "/Users/ueli/Documents/semio"
PORTABLE = f"{R}/🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts"
FN = r"(sin|cos|tan|asin|acos|atan|atan2|exp|pow|hypot|log|log2|log10|cbrt)"
CALL = re.compile(r"\bMath\." + FN + r"\(")
roots = [f"{R}/🧰️framework/🔨️modules/🧊️3d/🗿️model"]
stag = f"{R}/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules"
inc = [""]
skip_dirs = {"🧪️tests", "🧫️fixtures", "node_modules", "🎯️portable"}
files = []
for root in roots:
    for d, ds, fs in os.walk(root):
        ds[:] = [x for x in ds if x not in skip_dirs]
        files += [os.path.join(d, f) for f in fs if f.endswith(".ts")]
for sub in inc:
    for d, ds, fs in os.walk(f"{stag}/{sub}"):
        ds[:] = [x for x in ds if x not in skip_dirs]
        files += [os.path.join(d, f) for f in fs if f.endswith(".ts")]
IMPORT_END = re.compile(r'^(?:import\b.*|\}.*)\bfrom\s+"[^"]+";?\s*$')
touched = 0
for path in sorted(set(files)):
    s = open(path, encoding="utf8").read()
    if not CALL.search(s): continue
    has = "import * as portable from" in s
    lines = s.split("\n")
    last = -1
    for i, l in enumerate(lines):
        if IMPORT_END.match(l): last = i
    rel = os.path.relpath(PORTABLE, os.path.dirname(path))
    if not rel.startswith("."): rel = "./" + rel
    imp = f'import * as portable from "{rel}";'
    s2 = CALL.sub(lambda m: f"portable.{m.group(1)}(", s)
    lines = s2.split("\n")
    if has: pass
    elif last < 0:
        i = 0
        if lines[0].startswith("/**") or lines[0].startswith("//") or lines[0].startswith("#!"):
            while i < len(lines) and not (lines[i].rstrip().endswith("*/") or lines[i].startswith("//") or lines[i].startswith("#!")): i += 1
            i += 1
        lines.insert(i, imp)
    else:
        lines.insert(last + 1, imp)
    open(path, "w", encoding="utf8").write("\n".join(lines))
    touched += 1
    print(path.replace(R + "/", ""))
print("touched", touched, file=sys.stderr)
