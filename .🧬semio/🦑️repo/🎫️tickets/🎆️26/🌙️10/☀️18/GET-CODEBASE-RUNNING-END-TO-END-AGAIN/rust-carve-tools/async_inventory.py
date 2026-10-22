import re,sys,subprocess,os,json
R="/Users/ueli/Documents/semio"
MOVED={
 "timer":["TimerWheel","TimerSleep"],
 "testkit":["testkit","ManualRuntime"],
 "sync":["oneshot","Semaphore","Notify","Notified","OwnedPermit","AcquireOwned"],
 "slots":["ShardedMap","boxed_slots","boxed_fixed_slots","FixedSlotTableBudget","assert_fixed_slot_tables","on_bounded_stack","OriginalFixedSlotPages"],
}
sym2grp={s:g for g,l in MOVED.items() for s in l}
files=subprocess.run(["rg","-l","-g","*.rs","-g","!.🧬semio","-g","!node_modules","-g","!target","semio_framework_async",R],capture_output=True,text=True).stdout.split("\n")
files=[f for f in files if f]
out={}
brace=re.compile(r'semio_framework_async::\{')
def balanced(s,i):
    d=0
    for j in range(i,len(s)):
        if s[j]=="{":d+=1
        elif s[j]=="}":
            d-=1
            if d==0:return j
    return -1
def split_top(s):
    parts=[];d=0;cur=""
    for ch in s:
        if ch=="{":d+=1
        if ch=="}":d-=1
        if ch=="," and d==0:parts.append(cur);cur=""
        else:cur+=ch
    if cur.strip():parts.append(cur)
    return [p.strip() for p in parts]
for f in files:
    s=open(f,encoding="utf8",errors="replace").read()
    hits=[]
    for m in re.finditer(r'semio_framework_async::([A-Za-z_]+)',s):
        n=m.group(1)
        if n in sym2grp:hits.append(("path",n,s.count("\n",0,m.start())+1))
    for m in brace.finditer(s):
        i=m.end()-1;j=balanced(s,i)
        for p in split_top(s[i+1:j]):
            head=re.match(r'[A-Za-z_]+',p)
            if head and head.group(0) in sym2grp:hits.append(("brace",head.group(0),s.count("\n",0,m.start())+1))
    if "semio_framework_async::*" in s:hits.append(("glob","*",0))
    if hits:out[f.replace(R+"/","")]=hits
json.dump(out,open(sys.argv[1],"w"),ensure_ascii=False,indent=1)
print(len(files),"files mention;",len(out),"files use moved symbols")
from collections import Counter
print(Counter(h[0] for v in out.values() for h in v))
print(Counter(sym2grp.get(h[1],"glob") for v in out.values() for h in v))
