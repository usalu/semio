import sys
sys.path.insert(0, ".")
from author_69_lib import *
def neck(b, v):
    for x in by_id(b, "guitar-neck")["sections"]: x["rx"] = 0.05; x["ry"] = 0.05
edit("seated-guitarist", 500, "body", neck)
def hand(b, v):
    s = by_id(b, "hand-l")
    for x in s["sections"]: x["rx"] = round(x["rx"] * 1.6, 4); x["at"][2] += -0.04
if "--all" in sys.argv: edit("seated-guitarist", 500, "body", hand)
