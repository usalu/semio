#!/usr/bin/env python3
"""unique errors (by normalized location + full message line) with the primary span source line"""
import re,sys
t=open(sys.argv[1]).read()
blocks=re.split(r'\n(?=(?:error|warning)(?:\[E\d+\])?: )',t)
seen={}
for b in blocks:
    if not b.startswith("error"): continue
    lines=b.split("\n")
    head=lines[0]
    m=re.search(r'--> (\S+):(\d+):(\d+)',b)
    if m:
        loc=re.sub(r'(?:\.\./)+|📦️packages/🦀️rust/','',m.group(1)).replace("🧪️tests/","tests/")
        loc=loc.split("subsets/")[-1]
        loc=re.sub(r'^.*?(🧪️tests/)',r'\1',loc)
        key=(loc,m.group(2),head)
    else:
        key=("?","0",head)
    if key not in seen: seen[key]=b
for (loc,ln,head),b in sorted(seen.items()):
    print(f"{loc}:{ln} | {head[:230]}")
print(len(seen),"unique errors")
