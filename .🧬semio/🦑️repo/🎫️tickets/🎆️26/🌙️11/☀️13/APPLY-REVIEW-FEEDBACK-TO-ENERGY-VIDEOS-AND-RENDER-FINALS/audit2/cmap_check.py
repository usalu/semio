import pathlib, glob
from fontTools.ttLib import TTFont
from fontTools.pens.recordingPen import RecordingPen
base = pathlib.Path(r"C:/git/semio/🧰️framework/🔨️modules/🖼️assets/🔤️fonts")
paths = sorted(base.rglob("*.ttf")) + sorted(base.rglob("*.otf"))
mathp = pathlib.Path(r"C:/git/semio/🧰️framework/🔨️modules/📚️compiler/🌍️world/🔤️fonts/🧮️LibertinusMath-Regular.otf")
paths.append(mathp)
for p in paths:
    f = TTFont(p)
    cm = f.getBestCmap()
    gs = f.getGlyphSet()
    row = {}
    for ch in "()[]":
        if ord(ch) in cm:
            name = cm[ord(ch)]
            rp = RecordingPen(); gs[name].draw(rp)
            row[ch] = (name, len(rp.value))
        else:
            row[ch] = None
    print(p.relative_to(base) if str(p).startswith(str(base)) else p.name, row, flush=True)
