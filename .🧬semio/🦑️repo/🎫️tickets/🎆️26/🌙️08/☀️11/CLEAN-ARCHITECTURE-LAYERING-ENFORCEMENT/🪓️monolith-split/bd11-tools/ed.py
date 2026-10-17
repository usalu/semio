import os, sys
PLUGIN = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/"
class E:
    def __init__(self, rel):
        self.p = rel if rel.startswith("/") else PLUGIN + rel
        self.s = open(self.p).read(); self.n = 0
    def rep(self, old, new, count=1):
        c = self.s.count(old)
        if c != count: raise SystemExit(f"anchor count {c} != {count}: {old[:100]!r}")
        self.s = self.s.replace(old, new); self.n += c
    def save(self):
        open(self.p, "w").write(self.s); print("edited", self.n, "in", os.path.basename(os.path.dirname(os.path.dirname(self.p))))

import re as _re
def _use(self, *names):
    d = os.path.dirname(self.p)
    rel = os.path.relpath(PLUGIN + "🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts", d)
    m = _re.search(r'import\s*\{([^}]*)\}\s*from\s*"[^"]*🔌️plugin-family/🟦️\.ts";?', self.s)
    if m:
        have = {x.strip() for x in m.group(1).split(",") if x.strip()} | set(names)
        self.s = self.s[:m.start()] + "import {" + ", ".join(sorted(have)) + '} from "' + rel + '";' + self.s[m.end():]
    else:
        ms = list(_re.finditer(r'^import .*$', self.s, _re.M))
        pos = ms[-1].end() + 1 if ms else 0
        self.s = self.s[:pos] + "import {" + ", ".join(sorted(names)) + '} from "' + rel + '";\n' + self.s[pos:]
E.use = _use
