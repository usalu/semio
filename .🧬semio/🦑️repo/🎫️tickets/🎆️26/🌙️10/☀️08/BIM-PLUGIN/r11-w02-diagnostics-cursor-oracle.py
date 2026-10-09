import json
import sys
p = sys.argv[1]
s = open(p, encoding='utf-8', newline='').read()
def rep(old, new):
    global s
    assert s.count(old) == 1, (old, s.count(old))
    s = s.replace(old, new)
rep('''def expectations(cases):''', '''CURSOR_STEPS = {"left": (-1.0, 0.0), "right": (1.0, 0.0), "up": (0.0, 1.0), "down": (0.0, -1.0)}
"""⌨️ The arrow keys of the keyboard cursor as unit directions: a fine step is 0.1 m, a `_far` step 1 m."""


def cursor_point(start, steps):
    """⌨️ Where the keyboard cursor stands after the arrow keys `steps` from `start`: numpy sums the steps (0.1 m, or 1 m for `<direction>_far`) onto the start."""
    point = numpy.array(start, dtype=float)
    for step in steps:
        name, _, far = step.partition("_")
        point = point + (1.0 if far == "far" else 0.1) * numpy.array(CURSOR_STEPS[name])
    return [float(round(value, 9)) + 0.0 for value in point]


def expectations(cases):''')
rep('''    out["typed"] = [{**c, "point": typed_point(c["entry"], c.get("anchor"), c.get("toward"))} for c in cases["typed"]]
''', '''    out["typed"] = [{**c, "point": typed_point(c["entry"], c.get("anchor"), c.get("toward"))} for c in cases["typed"]]
    out["cursor"] = [{**c, "point": cursor_point(c["start"], c["steps"])} for c in cases["cursor"]]
''')
rep("and the point a typed line names (keyboard entry: absolute, relative, polar, bare length).", "the point a typed line names (keyboard entry: absolute, relative, polar, bare length) and the point the arrow-key cursor stands on after a run of fine and coarse steps.")
open(p + ".new", 'w', encoding='utf-8', newline='').write(s)
