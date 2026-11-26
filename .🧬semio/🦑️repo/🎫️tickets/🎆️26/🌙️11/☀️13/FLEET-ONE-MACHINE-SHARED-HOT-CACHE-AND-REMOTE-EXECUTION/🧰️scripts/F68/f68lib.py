#!/usr/bin/env python3
"""F68 helpers: per-area landing locks (lane F68) and anchored atomic edits, on top of the F66 helpers."""
import os,sys,time
sys.path.insert(0,os.path.join(os.path.dirname(__file__),"..","F66"))
from lib import TICKET,LIB,write,replace_once,locks as _locks,release,wait_locks as _wait
def take(areas,minutes=10):return _wait(areas,minutes,"F68")
def edit(path,old,new):
    return replace_once(path,old,new)
