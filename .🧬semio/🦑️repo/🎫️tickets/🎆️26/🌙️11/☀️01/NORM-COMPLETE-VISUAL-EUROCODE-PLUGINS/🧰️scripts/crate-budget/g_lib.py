"""🪚 Crate-budget G helpers: `lib` with a fast lock-file search (find with a depth bound, then grep) and `new_crate` from d_lib."""
import os, subprocess, sys
sys.path.insert(0, os.path.dirname(__file__))
import lib
from d_lib import *


def _fast(old):
    found = subprocess.run(["find", SEMIO, "-maxdepth", "9", "-name", "Cargo.lock", "-not", "-path", "*/target/*", "-not", "-path", "*/node_modules/*", "-not", "-path", "*/⚡️cache/*", "-not", "-path", "*/🎫️tickets/*", "-not", "-path", "*/.git/*"], capture_output=True, text=True).stdout.split("\n")
    return [p for p in found if p and 'name = "%s"' % old in open(p, encoding="utf-8").read()]


lib.lock_files = _fast
lock_files = _fast
