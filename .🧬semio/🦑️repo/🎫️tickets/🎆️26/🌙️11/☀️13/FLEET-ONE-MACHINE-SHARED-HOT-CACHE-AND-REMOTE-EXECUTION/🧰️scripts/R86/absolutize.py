#!/usr/bin/env python3
"""R86: writes a scratch copy of a staged module whose relative imports are absolute (for tests before landing). usage: absolutize.py STAGED LIVE_DIR OUT"""
import re,sys,os
src,live,out=sys.argv[1:4]
t=open(src,encoding="utf8").read()
def fix(m):
    p=os.path.normpath(os.path.join(live,m.group(2)))
    return m.group(1)+'"'+p+'"'
t=re.sub(r'(from)"(\.{1,2}/[^"]*)"',fix,t)
open(out,"w",encoding="utf8").write(t)
