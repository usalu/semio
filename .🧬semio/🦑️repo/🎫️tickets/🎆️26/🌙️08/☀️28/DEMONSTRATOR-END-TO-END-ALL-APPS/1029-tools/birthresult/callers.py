#!/usr/bin/env python3
"""Appends the Result unwrap to every test caller of the hook: `reference_birth().expect("explicit reference snapshot producer")()`.
usage: callers.py <files-list> <dry|apply>"""
import sys
FILES,MODE=sys.argv[1],sys.argv[2]
files=[l.rstrip('\n') for l in open(FILES,encoding='utf-8') if l.strip()]
if not files: sys.exit("empty root: refusing to run")
OLD='reference_birth().expect("explicit reference snapshot producer")()'
NEW='reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births")'
total=0
for f in files:
    t=open(f,encoding='utf-8').read()
    n=t.count(OLD)
    if not n:continue
    total+=n
    if MODE=='apply':
        t=t.replace(NEW,OLD)
        t=t.replace(OLD,NEW)
        open(f,'w',encoding='utf-8').write(t)
print("replacements",total)
