#!/usr/bin/env python3
"""🪞️ Completes a scratch mirror: every entry of the source tree that the mirror lacks at a level it already holds as a real directory becomes a symlink to the source. usage: fillmirror.py MIRROR SOURCE"""
import os, sys
mirror, source = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2])
added = 0
def fill(relative):
    global added
    here, there = os.path.join(mirror, relative), os.path.join(source, relative)
    for name in sorted(os.listdir(there)):
        target = os.path.join(here, name)
        if not os.path.lexists(target):
            os.symlink(os.path.join(there, name), target)
            added += 1
        elif os.path.isdir(target) and not os.path.islink(target) and os.path.isdir(os.path.join(there, name)):
            fill(os.path.join(relative, name))
fill("")
print("linked", added)
