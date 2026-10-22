#!/usr/bin/env python3
"""Prints errors of a cargo --message-format=short log grouped per failing unit. usage: errs.py <log> [crate-substring] [--lib]"""
import re,sys
log=sys.argv[1]; want=sys.argv[2] if len(sys.argv)>2 else ''
lib='--lib' in sys.argv
buf=[]
for line in open(log,encoding='utf-8',errors='replace'):
    line=line.rstrip('\n')
    if re.search(r'\.rs:\d+:\d+: error',line) or re.match(r'error(\[E\d+\])?: ',line) and 'could not compile' not in line:
        buf.append(line)
    m=re.match(r'error: could not compile `([^`]+)` \(([^)]*)\)',line)
    if m:
        name,kind=m.groups()
        if want in name and (not lib or kind=='lib'):
            print('###',name,kind)
            seen=set()
            for b in buf:
                b2=re.sub(r'^.*?((?:[^/]*/)?[^/]*/[^/]*\.rs:\d+:\d+)',r'\1',b)
                if b not in seen:
                    seen.add(b); print(' ',b[-420:] if len(b)>420 else b)
        buf=[]
