import sys,io
def sub(path,old,new,count=1):
    s=open(path,encoding="utf-8").read()
    n=s.count(old)
    if n!=count: raise SystemExit(f"{path}: expected {count} of {old[:80]!r}, found {n}")
    open(path,"w",encoding="utf-8").write(s.replace(old,new))
def cut(path,start,end,include_end=True,count=1):
    s=open(path,encoding="utf-8").read()
    i=s.index(start); j=s.index(end,i)+(len(end) if include_end else 0)
    open(path,"w",encoding="utf-8").write(s[:i]+s[j:])
