import os, re, sys
ROOT = "/Users/ueli/Documents/semio"
MT = ROOT + "/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts"
p = os.path.join(ROOT, sys.argv[1])
names = sys.argv[2].split(",")
t = open(p, encoding="utf-8").read()
rel = os.path.relpath(MT, os.path.dirname(p))
t = re.sub(r'import \{ ?[A-Za-z, ]+ ?\} from "[./]*🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";\n', "", t)
imp = 'import { %s } from "%s";\n' % (", ".join(names), rel)
lines = t.split("\n")
idx = 0
for i, l in enumerate(lines):
    if l.startswith("import "): idx = i + 1
t = "\n".join(lines[:idx]) + "\n" + imp + "\n".join(lines[idx:]) if idx else imp + t
open(p, "w", encoding="utf-8").write(t.replace("\n\n\n", "\n\n") if False else t)
