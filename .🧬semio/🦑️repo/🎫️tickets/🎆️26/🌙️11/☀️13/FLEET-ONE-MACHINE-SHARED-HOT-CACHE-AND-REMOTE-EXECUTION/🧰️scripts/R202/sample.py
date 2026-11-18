import json,glob,os,re,subprocess,sys,time
R='/Users/ueli/Documents/semio'
L=R+'/.cache/f/graph-server/read.log'
ST='/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R202/hist.json'
os.makedirs(os.path.dirname(ST),exist_ok=True)
def sh(c):
    return subprocess.run(c,shell=True,capture_output=True,text=True).stdout
tail=sh("tail -c 3000000 '%s'"%L)
lines=tail.split('\n')
def last(pat):
    r=None
    for l in lines:
        if re.search(pat,l): r=l
    return r
rec=json.load(open(R+'/.cache/f/graph-server/read-record.json'))
now=time.time()
scr=[l for l in lines if l.startswith('[script]')]
phase=(scr[-1][9:] if scr else '')[:170]
led=last(r'^\[owners\] owners \d+ of')
led=re.sub(r'^\[owners\] owners ','',led or '')[:200]
done=int(re.match(r'(\d+) of',led).group(1)) if led else None
tot=int(re.match(r'\d+ of (\d+)',led).group(1)) if led else None
aside=sum(1 for l in lines if ' is set aside' in l)
cens=last('by reason')
wait=last(r'^\[owners\] no owner arrived')
pres=last(r'owner records present')
hist=[]
try: hist=json.load(open(ST))
except Exception: pass
if done is not None:
    hist.append([now,done]); hist=hist[-400:]; json.dump(hist,open(ST,'w'))
opm=''
if done is not None and hist:
    old=[h for h in hist if h[0]<=now-600]
    b=old[-1] if old else hist[0]
    if now-b[0]>60:
        r=(done-b[1])/((now-b[0])/60); opm='%.2f/min over %dmin'%(r,(now-b[0])/60)
        if r>0 and tot: opm+=', left %d, eta %.1f h'%(tot-done,(tot-done)/r/60)
rd=rec.get('read')
ps=sh("ps -axo pid=,ppid=,etime=,%cpu=,rss=,command=")
kids={}
for l in ps.split('\n'):
    p=l.split(None,5)
    if len(p)>=6: kids[int(p[0])]=p
def children(pp):
    return [k for k in kids.values() if int(k[1])==pp]
tree=[]
for c in children(rd):
    for g in children(int(c[0])):
        tree.append('%s %s %s%% %dMB'%(g[0],g[2],g[3],int(g[4])//1024))
    tree.append('[%s %s %s%%]'%(c[0],c[2],c[3]))
cap=sh('cd %s && repo capacity status 2>&1 | head -6'%R)
cap=' | '.join(x.strip()[:200] for x in cap.split('\n') if x.strip() and 'capacity' in x or 'running' in x)
root='/Users/Shared/semio-store/v1/origins'
wave=(json.load(open(root+'/g1-server/pool.json')).get('wave') or '')[:12]
rows=[];T=[0,0,0]
for f in sorted(glob.glob(root+'/*/pool.json')):
    try: d=json.load(open(f))
    except Exception: continue
    o=f.split('/')[-2]; age=now-os.path.getmtime(f)
    if (d.get('wave') or '').startswith(wave) and age<120:
        T[0]+=d.get('claimed',0);T[1]+=d.get('published',0);T[2]+=d.get('children',0)
        names=0
        try: names=sum(1 for _ in open('%s/%s/pool-published-%s.txt'%(root,o,wave)))
        except Exception: pass
        rows.append('%s:%s/%s/%dc/n%d'%(o[-3:],d.get('claimed'),d.get('published'),d.get('children',0),names))
t=time.strftime('%H:%M:%S',time.gmtime(now))
print(t,'| phase:',phase)
print('  ledger:',led,'| aside',aside,'| rate',opm)
if pres: print('  ',pres[:160])
if cens: print('  ',cens[:260])
if wait: print('  ',wait[:200])
print('  children(pid age cpu mem):',' '.join(tree))
print('  cap:',cap)
print('  pools wave',wave,'claimed/published/children',T,'|',' '.join(rows))
print('  load',os.getloadavg())
