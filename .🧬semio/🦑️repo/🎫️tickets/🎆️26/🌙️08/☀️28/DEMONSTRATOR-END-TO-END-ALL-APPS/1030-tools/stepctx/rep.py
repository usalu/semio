import sys,json
def rep(path, pairs):
    s=open(path,encoding='utf8').read()
    for old,new in pairs:
        if s.count(old)!=1: sys.exit(f"MISMATCH({s.count(old)}) in {path}: {old[:90]}")
        s=s.replace(old,new)
    open(path,'w',encoding='utf8').write(s)
