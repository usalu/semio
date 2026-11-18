#!/usr/bin/env python3
"""R84: run a list of test files (paths relative to the build root B, one per line on stdin) through run1.py one after the other; usage: many.py <birth> < list"""
import sys,subprocess,os
H=os.path.dirname(os.path.abspath(__file__))
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
birth=sys.argv[1] if len(sys.argv)>1 else "setup"
for l in sys.stdin.read().split("\n"):
    l=l.strip()
    if not l:continue
    r=subprocess.run(["python3",H+"/run1.py",B+l,birth],capture_output=True,text=True)
    print(l[:70],"|",r.stdout.strip().replace("\n"," || ")[:600],flush=True)
