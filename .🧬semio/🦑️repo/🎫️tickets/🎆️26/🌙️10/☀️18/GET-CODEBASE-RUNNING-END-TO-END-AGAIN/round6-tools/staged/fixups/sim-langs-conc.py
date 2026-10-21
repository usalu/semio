#!/usr/bin/env python3
"""Simulation only: the overlay has no tools partition and the sandbox has no network, so the pinned-Go provisioning of langs-conc (which the live apply performs) is skipped in the overlay copy of its apply.py. usage: sim-langs-conc.py <apply.py of the overlay copy>"""
import sys
path = sys.argv[1]
text = open(path, encoding="utf8").read()
old = "if not OVERLAY and not go_provisioned():"
if text.count(old) != 1:
    sys.exit("sim fixup: gate not found once")
open(path, "w", encoding="utf8").write(text.replace(old, "if False and not go_provisioned():"))
print("provisioning gate skipped in", path)
