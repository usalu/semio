#!/usr/bin/env python3
# usage: edits_tests.py   asserts on the overlay that every anchored edit is idempotent (a second plan finds nothing to write) and that the removed files are gone.
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

OV = "/private/tmp/rexec-inhouse/ov"
writes, removes, problems, notes = plan(OV)
assert not problems, problems
assert not writes, [relative for relative, _ in writes]
assert not removes, removes
print("edits idempotent:", len(notes), "notes, nothing to write, nothing to remove")
