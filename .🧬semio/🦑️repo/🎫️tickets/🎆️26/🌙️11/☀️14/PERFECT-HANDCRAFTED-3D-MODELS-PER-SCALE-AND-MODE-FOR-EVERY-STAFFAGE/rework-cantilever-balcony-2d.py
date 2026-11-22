"""✏️ Redraws the 1:500 front view of components/cantilever-balcony by hand to the rebuilt 75 mm railing (post 75 mm from the foot to the handrail, upper flat merged into the handrail, balusters up to the handrail): rework-cantilever-balcony-2d.py <catalog asset> <output>."""
import sys

source, target = sys.argv[1], sys.argv[2]
text = open(source).read()
start = text.index('"denominator": 500')
front = text.index('"front"', start)
plan = text.index('"plan"', front)
segment = text[front:plan]


def swap(old, new, count=1):
    global segment
    assert segment.count(old) == count, (old, segment.count(old))
    segment = segment.replace(old, new)


swap('"x": -0.025,\n              "y": 0.2045,\n              "w": 0.05,\n              "h": 1.0165,', '"x": -0.0375,\n              "y": 0.1793,\n              "w": 0.075,\n              "h": 1.025,')
chord = segment.index('            {\n              "id": "railing-top-chord"')
segment = segment[:chord] + segment[segment.index('            {\n              "id": "railing-bottom-chord"'):]
swap('[0.33967, 1.1343]', '[0.33967, 1.2418]')
swap('[-0.33967, 1.1343]', '[-0.33967, 1.2418]')
text = text[:front] + segment + text[plan:]
for old, new in (
    ("with the post and the rail as two pen lines", "with the 75 mm post, the handrail line, the bottom rail and two balusters up to the handrail"),
    ("mit Pfosten und Holm als zwei Federlinien", "mit dem Pfosten von 75 mm, der Handlauflinie, dem unteren Holm und zwei Füllstäben bis zum Handlauf"),
):
    print("[DEBUG] decisions phrase", repr(old), text.count(old))
    text = text.replace(old, new)
open(target, "w").write(text)
