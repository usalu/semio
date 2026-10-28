"""🧹 Lists English string literals that a lang batch removes from code (present in base, absent in new) for reference hunting: python3 -I removed_texts.py <batchDir>..."""
import json, pathlib, re, sys
C = pathlib.Path(__file__).resolve().parent.parent
lit = re.compile(r'"((?:[^"\\]|\\.){12,})"|`((?:[^`\\]|\\.){12,})`|\'((?:[^\'\\]|\\.){12,})\'')
out = set()
for d in sys.argv[1:]:
    s = C / d; m = json.loads((s / "manifest.json").read_text())
    for rel, e in m.items():
        if e["base"] is None or e["new"] is None or rel.endswith((".json", ".md")) or "🧪️tests" in rel: continue
        a = (s / "base" / e["file"]).read_text(); b = (s / "files" / e["file"]).read_text()
        for mt in lit.finditer(a):
            t = next(g for g in mt.groups() if g)
            if t in b or not re.search(r"[A-Z][a-z]+ [a-z]+", t): continue
            t = re.split(r"\$\{|\{[A-Za-z]+\}|\\n|:", t)[0].strip()
            if len(t) >= 14: out.add(t)
for t in sorted(out): print(t)
