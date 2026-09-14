# 🚚 Moves the sequence-level statistics kernel out of the charts-distribution Pending region
# into its owning package semio-viz-transform, and renames the module prefix to the package's.
import io, os, re

LATEX = u"C:/git/semio/\U0001f9f0\ufe0fframework/\U0001f6cd\ufe0fproducts/\U0001f4d3\ufe0fprint/\U0001f58b\ufe0flatex"
DIST = os.path.join(LATEX, "semio-viz-charts-distribution.sty")
TR = os.path.join(LATEX, "semio-viz-transform.sty")
BEGIN = u"%region \U0001f516\ufe0fPending-semio-viz-transform"
END = u"%endregion \U0001f516\ufe0fPending-semio-viz-transform"
ANCHOR = u"%region \U0001f516\ufe0fPipeline"
HEAD = u"""%region \U0001f516\ufe0fSeqStatistics
% \U0001f9ee The sequence-level half of the transform vocabulary: the table transforms above take a
% table and produce a table, while a family that has already pulled one column into a sequence --
% a box plot, a histogram, a density curve -- needs the same d3-array algorithms on that sequence
% directly. Every macro here is a faithful port of the named d3 routine, so the oracle comparison
% in the tests is exact rather than approximate.
% https://github.com/d3/d3-array/blob/main/src/bin.js
% https://github.com/d3/d3-array/blob/main/src/quantile.js
"""
FOOT = u"%endregion \U0001f516\ufe0fSeqStatistics\n"


def read(path):
    return io.open(path, encoding="utf-8").read()


def write(path, text):
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(text)
    os.replace(tmp, path)


dist = read(DIST)
start = dist.index(BEGIN)
stop = dist.index(END) + len(END)
region = dist[start:stop]
body = region[len(BEGIN):-len(END)]
write(DIST, dist[:start] + dist[stop:].lstrip("\n"))

tr = read(TR)
tr = tr.replace(ANCHOR, HEAD + body.strip("\n") + "\n" + FOOT + "\n" + ANCHOR, 1)
write(TR, tr)

renamed = 0
for name in sorted(os.listdir(LATEX)):
    if not name.endswith(".sty"):
        continue
    path = os.path.join(LATEX, name)
    src = read(path)
    if "semio_viz_stat_" not in src:
        continue
    renamed += src.count("semio_viz_stat_")
    write(path, src.replace("semio_viz_stat_", "semio_viz_tr_stat_"))

print("moved", len(body.splitlines()), "lines; renamed", renamed, "occurrences")
