# 🔧 Replaces the placeholder Pipeline region of semio-viz-plot.sty with the real engine.
import io, os

TICKET = os.path.dirname(os.path.abspath(__file__))
NEW = os.path.join(TICKET, u"\U0001f5d1\ufe0fgenerated", "INTEGRATION-1", "plot-pipeline.tex")
LATEX = u"C:/git/semio/\U0001f9f0\ufe0fframework/\U0001f6cd\ufe0fproducts/\U0001f4d3\ufe0fprint/\U0001f58b\ufe0flatex"
TARGET = os.path.join(LATEX, "semio-viz-plot.sty")
BEGIN = u"%region \U0001f516\ufe0fPipeline"
END = u"%endregion \U0001f516\ufe0fPipeline"

src = io.open(TARGET, encoding="utf-8").read()
start = src.index(BEGIN)
stop = src.index(END) + len(END)
body = io.open(NEW, encoding="utf-8").read().rstrip("\n")
out = src[:start] + body + src[stop:]
tmp = TARGET + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(out)
os.replace(tmp, TARGET)
print("spliced", len(body.splitlines()), "lines")
