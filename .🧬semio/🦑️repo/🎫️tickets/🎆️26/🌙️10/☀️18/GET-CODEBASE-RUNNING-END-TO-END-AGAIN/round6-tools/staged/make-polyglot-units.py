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
W = T + "/wiring.py"
patch(W, "def catalog_edit(text):\n", '''import re


def add_operation(text, flag, operation):
    start = text.index('"name": "%s",' % flag)
    match = re.compile(r'"operations": \\[([^\\]]*)\\]').search(text, start)
    if match is None or match.start() - start > 400:
        raise SystemExit("DRIFT: operations of %s not found" % flag)
    if '"%s"' % operation in match.group(1):
        return text
    return text[:match.end(1)] + ', "%s"' % operation + text[match.end(1):]


def catalog_edit(text):
''')
patch(W, '''    for old, new in CATALOG_EXTENDS:
        if text.count(old) != 1:
            raise SystemExit("DRIFT: catalogue flag anchor is not unique: " + old[:60])
        text = text.replace(old, new)
''', '''    for flag in ("--file", "--json", "--root"):
        text = add_operation(text, flag, "units")
''')
patch(W, '''    if text.count(SCHEMA_OLD) != 1:
        raise SystemExit("DRIFT: schema OperationName anchor is not unique")
    return text.replace(SCHEMA_OLD, SCHEMA_NEW)''', '''    match = re.search(r'("OperationName": \\{\\s*"enum": \\[[^\\]]*?)("dependencies")', text)
    if match is None:
        raise SystemExit("DRIFT: schema OperationName anchor not found")
    return text[:match.start(2)] + '"units", ' + text[match.start(2):]''')
print("polyglot-units merged copy:", T)
