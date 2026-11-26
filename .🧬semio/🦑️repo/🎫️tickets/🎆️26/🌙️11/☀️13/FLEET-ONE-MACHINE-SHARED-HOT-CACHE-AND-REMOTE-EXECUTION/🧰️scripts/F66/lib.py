#!/usr/bin/env python3
"""F66 helpers: parse tsc output, take per-area landing locks, apply anchored atomic edits to test files."""
import os,re,sys,time
TICKET="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
TSCDIR="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript"
LIB="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
ERR=re.compile(r'^(.*?)\((\d+),(\d+)\): error (TS\d+): (.*)$')
def errors(path):
    out=[]
    for l in open(path,encoding="utf8"):
        m=ERR.match(l.rstrip("\n"))
        if m:
            f=os.path.normpath(os.path.join(TSCDIR,m.group(1)))
            out.append((f,int(m.group(2)),int(m.group(3)),m.group(4),m.group(5)))
    return out
def area(f):
    parts=os.path.relpath(f,LIB).split(os.sep)
    pick=parts[1] if len(parts)>2 and parts[0] in("🏗️build","⚡️caching") else parts[0]
    name="".join(ch for ch in pick if ch.isascii()).strip("-") or "misc"
    return "caching-"+name if parts[0]=="⚡️caching" and len(parts)>2 else name
def locks(areas,lane="F66"):
    got=[];held=set()
    for a in sorted(areas):
        d=f"{TICKET}/landing.lock.{a}"
        try:
            os.mkdir(d)
        except FileExistsError:
            try:age=time.time()-os.path.getmtime(d)
            except OSError:age=0
            if age>2400:
                pass
            else:
                held.add(a);continue
        open(d+"/owner","w").write(f"{lane} {time.strftime('%H:%M')}\n");got.append(d)
    return got,held
def release(got):
    for d in got:
        try:
            os.remove(d+"/owner");os.rmdir(d)
        except OSError:pass
def write(f,s):
    t=f+".f66tmp";open(t,"w",encoding="utf8").write(s);os.replace(t,f)
def arg_end(line,i):
    d=0;q=None
    while i<len(line):
        c=line[i]
        if q:
            if c=="\\":i+=1
            elif c==q:q=None
        elif c in "\"'`":q=c
        elif c in "([{":d+=1
        elif c in ")]}":
            if d==0:return i
            d-=1
        elif c=="," and d==0:return i
        i+=1
    return i
def foreign(f):
    names={"".join(ch for ch in p if ch.isascii()).strip("-") for p in os.path.relpath(f,LIB).split(os.sep)}|{area(f)}
    for n in names:
        o=f"{TICKET}/landing.lock.{n}/owner"
        if n and os.path.exists(o) and not open(o).read().startswith("F66"):return True
    return False
def wait_locks(areas,minutes=10,lane="F66"):
    end=time.time()+minutes*60
    while True:
        got,held=locks(areas,lane)
        if not held or time.time()>end:return got,held
        release(got);time.sleep(30)
def replace_once(f,a,b):
    s=open(f,encoding="utf8").read()
    if a not in s:print("missing",f.split("/")[-2],a[:50]);return False
    if s.count(a)!=1:print("ambiguous",f.split("/")[-2],a[:50],s.count(a));return False
    write(f,s.replace(a,b));return True
