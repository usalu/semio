"""cstep scanner: TS source-text laws whose needle lived in a Rust file this port rewrote (present at HEAD, absent now).
usage: lawneedles.py changedlist"""
import re,sys,subprocess,os
def norm(t): return re.sub(r'\s+','',t)
changed=[l for l in open(sys.argv[1]).read().split('\n') if l]
head={};cur={}
for p in changed:
    try: h=subprocess.run(['git','show','HEAD:'+p],capture_output=True,text=True,check=True).stdout
    except Exception: continue
    c=open(p,encoding='utf-8').read()
    if h!=c: head[p]=norm(h);cur[p]=norm(c)
print('rewritten files',len(head),file=sys.stderr)
tsfiles=[]
for root in ['✏️s','🧰️framework']:
    for dp,dn,fn in os.walk(root):
        dn[:]=[d for d in dn if d not in('node_modules','🗑️generated','target','.cache')]
        for f in fn:
            if f.endswith('.ts') and ('🟦️' in f or f.endswith('.test.ts')):
                tsfiles.append(os.path.join(dp,f))
lit=re.compile(r'"((?:[^"\\\n]|\\.)*)"|`((?:[^`\\]|\\.)*)`|\'((?:[^\'\\\n]|\\.)*)\'')
hits={}
for ts in tsfiles:
    try:t=open(ts,encoding='utf-8').read()
    except: continue
    if not re.search(r'close_step|fn step|InteractiveJobCloseStep|WorkClosing|job_close_step|step_value',t): continue
    for m in lit.finditer(t):
        s=m.group(1) or m.group(2) or m.group(3) or ''
        if len(s)<24 or not re.search(r'close_step|fn step|InteractiveJobCloseStep|job_close_step|step_value|Refused',s): continue
        n=norm(s.replace('\\"','"').replace("\\'","'").replace('\\n',''))
        if len(n)<20: continue
        for p,h in head.items():
            if n in h and n not in cur[p]:
                hits.setdefault(ts,[]).append((s[:110],p[-60:]))
for ts,items in hits.items():
    print(ts[-110:])
    for s,p in items[:6]: print('   ',s,'<-',p)
