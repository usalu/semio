import pathlib, collections
from fontTools.ttLib import TTFont
from fontTools.pens.recordingPen import RecordingPen
base = pathlib.Path(r"C:/git/semio/🧰️framework/🔨️modules/🖼️assets/🔤️fonts")
paths = [p for p in sorted(base.rglob("*.ttf")) if "noto-emoji" not in str(p)]
paths.append(pathlib.Path(r"C:/git/semio/🧰️framework/🔨️modules/📚️compiler/🌍️world/🔤️fonts/🧮️LibertinusMath-Regular.otf"))
for p in paths:
    f = TTFont(p); cm = f.getBestCmap(); gs = f.getGlyphSet()
    out = []
    for ch in "()[]":
        if ord(ch) not in cm:
            out.append(f"{ch}:MISSING"); continue
        rp = RecordingPen(); gs[cm[ord(ch)]].draw(rp)
        ops = collections.Counter(op for op, _ in rp.value)
        xs = [pt[0] for _, args in rp.value for pt in args]
        ys = [pt[1] for _, args in rp.value for pt in args]
        bb = (round(min(xs)),round(min(ys)),round(max(xs)),round(max(ys))) if xs else None
        out.append(f"{ch}:{cm[ord(ch)]} ops={dict(ops)} bbox={bb}")
    print(p.name, p.parent.parent.name if False else str(p.relative_to(base)) if str(p).startswith(str(base)) else "math", " | ".join(out), flush=True)
