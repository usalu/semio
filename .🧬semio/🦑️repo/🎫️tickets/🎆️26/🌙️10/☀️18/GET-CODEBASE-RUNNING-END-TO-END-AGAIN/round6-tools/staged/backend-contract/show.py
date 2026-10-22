#!/usr/bin/env python3
"""Prints one file as the staged edits leave it. Usage: show.py <substring of repo-relative path> [grep]"""
import sys,os,re
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import wire,names
from collections import OrderedDict
g=OrderedDict()
for rel,fn in wire.EDITS:g.setdefault(rel,[]).append(fn)
for rel,fns in g.items():
    if sys.argv[1] in rel:
        t=open(names.R+'/'+rel,encoding='utf8').read()
        for fn in fns:t=fn(t)
        for i,l in enumerate(t.split('\n'),1):
            if len(sys.argv)<3 or re.search(sys.argv[2],l):print(i,l[:int(os.environ.get('W','400'))])
