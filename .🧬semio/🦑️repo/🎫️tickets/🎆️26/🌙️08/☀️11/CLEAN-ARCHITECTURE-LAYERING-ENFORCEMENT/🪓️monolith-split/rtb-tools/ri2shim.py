"""Loads the ri2 tools with a tolerant reader (the ri2 pre-flip chunk files are gone)."""
import sys,os
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
H=F+"/ri2-tools"
sys.path.insert(0,H+"/w1"); sys.path.insert(0,H)
import common
_r=common.read
def _read(p):
    try: return _r(p)
    except FileNotFoundError: return ""
common.read=_read
import model
model.read=_read
import bins as B, flipfc as X, flip as FL, paths as P
from flip import *
E_=E
