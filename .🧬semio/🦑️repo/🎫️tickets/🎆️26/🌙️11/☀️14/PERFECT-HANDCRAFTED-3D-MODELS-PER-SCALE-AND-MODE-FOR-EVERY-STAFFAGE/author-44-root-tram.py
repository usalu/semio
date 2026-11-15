import sys, os, re, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from author_14_root import patch
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-44-lib.py")).read())


def ws_plan(o):
    o["points"] = [[8.7, -1.0], [9.4, -0.85], [9.4, 0.85], [8.7, 1.0]]


print(patch("tram", [("50", "plan", "windscreen", ws_plan), ("100", "plan", "windscreen", ws_plan)]))
for r, v in (("body-panels", 79.82), ("glazing", 32.83), ("doors", 8.89), ("brake-discs", 1.89), ("hvac-unit", 8.96)):
    set_area("tram", r, v)
