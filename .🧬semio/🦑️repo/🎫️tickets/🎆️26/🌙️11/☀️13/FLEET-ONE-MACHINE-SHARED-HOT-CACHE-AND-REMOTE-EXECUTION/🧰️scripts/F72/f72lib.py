#!/usr/bin/env python3
"""F72 helpers: caching-cargo lock (lane F72) and anchored atomic edits of the Rust units test."""
import os,sys,time
sys.path.insert(0,os.path.join(os.path.dirname(__file__),"..","F66"))
from lib import TICKET,LIB,write,replace_once,locks,release
TEST=LIB+"/⚡️caching/🦀️cargo/📦️units/🧪️tests/🟦️.ts"
def take(seconds=100):
    end=time.time()+seconds
    while True:
        got,held=locks({"caching-cargo"},"F72")
        if not held:return got
        release(got)
        if time.time()>end:return None
        time.sleep(10)
def have():
    o=f"{TICKET}/landing.lock.caching-cargo/owner"
    return os.path.exists(o) and open(o).read().startswith("F72")
