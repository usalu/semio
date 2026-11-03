import json, re, sys
from pathlib import Path
ROOT = Path("C:/git/semio/🎓️teaching/🏛️architecture")
inv = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
man = json.loads((ROOT / "⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json").read_text(encoding="utf-8"))["casts"]
kw = re.compile(r"\bplan\b|floor plan|top view|from above|aerial|\bsection\b|elevation|facade|façade|front", re.I)
out = []
for clip, casts in inv["clips"].items():
    p = ROOT / clip
    src = p.read_text(encoding="utf-8")
    words = {}
    for m in kw.finditer(src):
        w = m.group(0).lower(); words[w] = words.get(w, 0) + 1
    out.append(f"## {clip}\nkeywords: {words}")
    for c in casts:
        e = man[c]
        out.append(f"  - {c}: vp={e.get('viewpoint','(implicit elevation)')} mirror={e.get('mirror',False)} scale={e.get('scale',50)}")
    out.append("")
Path(sys.argv[2]).write_text("\n".join(out), encoding="utf-8")
