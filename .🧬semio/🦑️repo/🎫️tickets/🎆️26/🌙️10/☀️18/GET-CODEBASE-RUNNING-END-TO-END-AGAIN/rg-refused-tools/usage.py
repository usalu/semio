#!/usr/bin/env python3
# 🔎️ Per-name consumers of a crate across the repo: usage.py <crate_ident> [name ...]
import re,subprocess,sys,collections
root="/Users/ueli/Documents/semio"
crate=sys.argv[1]
files=subprocess.check_output(["git","-C",root,"grep","-l","-w",crate,"--","*.rs",":!.🧬semio"]).decode().split("\n")
files=[f for f in files if f]
byname=collections.defaultdict(set)
for f in files:
    s=open(root+"/"+f,encoding='utf8').read()
    for m in re.finditer(r'\b'+crate+r'::((?:\{[^;]*?\})|(?:[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*(?:::\*)?))',s,re.S):
        body=m.group(1)
        for ident in re.findall(r'[A-Za-z_][A-Za-z0-9_]*',body):
            byname[ident].add(f)
for k,v in sorted(byname.items(),key=lambda kv:-len(kv[1])):
    print(len(v),k)
