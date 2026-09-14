# 🚚 Moves the remaining Pending-semio-viz-transform regions of the finished chart namespaces
# into their owning package semio-viz-transform, renaming each to the package's module prefix.
import io, os

LATEX = u"C:/git/semio/\U0001f9f0\ufe0fframework/\U0001f6cd\ufe0fproducts/\U0001f4d3\ufe0fprint/\U0001f58b\ufe0flatex"
BEGIN = u"%region \U0001f516\ufe0fPending-semio-viz-transform"
END = u"%endregion \U0001f516\ufe0fPending-semio-viz-transform"
ANCHOR = u"%region \U0001f516\ufe0fPipeline"

# file -> (region title, {old prefix: new prefix}, tail text kept in the source file)
JOBS = [
    ("semio-viz-charts-statistical.sty", u"ModelEvaluation",
     {u"semio_viz_eval_": u"semio_viz_tr_eval_"}, None),
    ("semio-viz-matrix-heatmap.sty", u"MatrixReshape",
     {u"semio_viz_mx_": u"semio_viz_tr_matrix_"}, None),
]
HEAD = {
    u"ModelEvaluation": u"""%region \U0001f516\ufe0fModelEvaluation
% \U0001f4c8 Classifier sweeps: a scored sample plus its binary labels reduced to the curve a model
% evaluation chart draws. Points come out in the order a descending-score threshold sweep visits
% them, which is what the TypeScript oracle reproduces on top of d3-array.
""",
    u"MatrixReshape": u"""%region \U0001f516\ufe0fMatrixReshape
% \U0001f9ee Matrix reshaping: a long row/column/value table becomes an ordered level list per axis
% plus a dense value list, the sequence-level twin of the `pivot` transform above. Heat matrices,
% correlation matrices and adjacency matrices all read their cells through it.
""",
}


def read(path):
    return io.open(path, encoding="utf-8").read()


def write(path, text):
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(text)
    os.replace(tmp, path)


moved = []
for name, title, renames, _ in JOBS:
    path = os.path.join(LATEX, name)
    src = read(path)
    start = src.index(BEGIN)
    stop = src.index(END) + len(END)
    body = src[start + len(BEGIN):stop - len(END)]
    write(path, src[:start] + src[stop:].lstrip("\n"))
    moved.append((title, body))

tr = read(os.path.join(LATEX, "semio-viz-transform.sty"))
block = u""
for title, body in moved:
    block += HEAD[title] + body.strip("\n") + u"\n%endregion \U0001f516\ufe0f" + title + u"\n\n"
tr = tr.replace(ANCHOR, block + ANCHOR, 1)
write(os.path.join(LATEX, "semio-viz-transform.sty"), tr)

count = 0
for _, _, renames, _ in JOBS:
    for old, new in renames.items():
        for name in sorted(os.listdir(LATEX)):
            if not name.endswith(".sty"):
                continue
            path = os.path.join(LATEX, name)
            src = read(path)
            if old not in src:
                continue
            count += src.count(old)
            write(path, src.replace(old, new))

print("moved", len(moved), "regions; renamed", count, "occurrences")
