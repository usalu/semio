#!/usr/bin/env python3
"""Merged copy of the polyglot-units set: the help snapshot is not part of the apply (it would be a whole-file write over a fixture five other sets change); the chain regenerates it once from the catalogue (regen-help step)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/polyglot-units-tools", "polyglot-units")
A = T + "/apply.py"
text = open(A, encoding="utf8").read()
a = text.index("        path = os.path.join(LIB, HELP)\n")
b = text.index("    return ops, refused")
assert a < b
open(A, "w", encoding="utf8").write(text[:a] + text[b:])
print("polyglot-units merged copy:", T)
