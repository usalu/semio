# 🧹️ Leftover consumer calls: lines that still call a WP5 use case directly on a value after the consumer rewrite (overlay first, then the working copy, then the repo).
import sys, os, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, ext
from blank import blank
tab = ext.method_table(cfg.NEW)
names = "|".join(sorted(map(re.escape, tab), key=len, reverse=True))
rx = re.compile(r"[.](" + names + r")\s*(::\s*<[^>]*>)?\s*\(")
SHEL = cfg.SHELL_REL + "/"
for rel in ext.candidates(tab):
    if rel.startswith(SHEL + cfg.TGT + "/"): continue
    ov = os.path.join(cfg.GEN, "new-ext", rel)
    p = ov if os.path.exists(ov) else (os.path.join(cfg.NEW, rel[len(SHEL):]) if rel.startswith(SHEL) else os.path.join(cfg.ROOT, rel))
    b = blank(open(p, encoding="utf-8").read()); left = []
    for i, l in enumerate(b.split("\n")):
        for m in rx.finditer(l):
            if "Ctx::of(" in l[max(0, m.start() - 80):m.end()]: continue
            left.append((i + 1, l.strip()[:140]))
    if left:
        print("==", rel.replace(cfg.ENG + "/", "E/")[:90], len(left))
        for x in left[:10]: print("   ", x)
