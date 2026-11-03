import glob
import io
import contextlib
import re
import runpy
import sys

sys.argv = ["extract.py", "x"]
G = re.compile(r"[äöüßÄÖÜ]|\b(und|der|die|das|mit|ohne|von|für|nicht|Wärme\w*|Luft\w*|Heiz\w*|Kühl\w*|Strom|Energie|Haus|Raum|Fenster|Wand|Dach|innen|außen|Gerät\w*|Licht|Wasser|Zuluft|Abluft|Sommer|Winter)\b")
src = open("🎓️teaching/🏛️architecture/⚡️energy/../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🧰️lc2/extract.py", encoding="utf-8").read()
src = src[: src.index("import glob\nif sys.argv[1]")]
ns = {"__name__": "x"}
exec(compile(src, "extract", "exec"), ns)
main = ns["main"]
for f in sorted(glob.glob("🎓️teaching/🏛️architecture/⚡️energy/*/🎬️clip/*/🐍️.py")):
    if "summer" in f or "🏗️" in f or "🧪️tests" in f:
        continue
    buf = io.StringIO()
    with contextlib.redirect_stdout(buf):
        main(f)
    rows = []
    for ln in buf.getvalue().splitlines():
        c = ln.split("\t")
        if len(c) < 3:
            continue
        v = eval(c[2])
        if G.search(v) and not re.search(r"mathrm\{(Luft|WRG|Verlust|Gewinn|nutz|ges|Quelle|Senke|Speicher|Auslegung|Tau|Pers|Raum|Decke)", v) and "Occupants do not" not in v and not c[3].lstrip("| ").startswith(('"', "'", "#")) or (G.search(v) and "text{" in v):
            rows.append(f"{c[0]}:{v[:60]!r}")
    if rows:
        print(f.split("clip")[-1], len(rows), "; ".join(rows[:14]))
