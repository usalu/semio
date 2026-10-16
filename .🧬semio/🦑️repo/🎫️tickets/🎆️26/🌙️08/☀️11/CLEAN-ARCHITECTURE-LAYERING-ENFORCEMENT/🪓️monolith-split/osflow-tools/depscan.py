"""🔎 Which old-manifest dependencies a set of files uses. usage: depscan.py <file>... (prefix test files with +)"""
import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from manifests import *
def scan(paths):
    return idents_used(paths)
if __name__=="__main__":
    lib=[a for a in sys.argv[1:] if not a.startswith("+")]
    tst=[a[1:] for a in sys.argv[1:] if a.startswith("+")]
    ul=scan(lib) if lib else set();ut=scan(tst) if tst else set()
    for d in DEPS:
        a=d["ident"] in ul;b=d["ident"] in ut
        if a or b:print("LIB " if a else "TEST",d["where"],d["cfg"] or "",d["key"],d["ident"])
