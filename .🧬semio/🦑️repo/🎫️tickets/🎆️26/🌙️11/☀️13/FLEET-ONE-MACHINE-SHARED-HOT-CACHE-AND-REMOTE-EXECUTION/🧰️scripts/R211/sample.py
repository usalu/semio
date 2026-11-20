import json,glob,os,re,subprocess,sys,time
R='/Users/ueli/Documents/semio'
L=R+'/.cache/f/graph-server/read.log'
S='/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R211/'
def sh(c): return subprocess.run(c,shell=True,capture_output=True,text=True).stdout
tail=sh("tail -c 4000000 '%s'"%L); lines=tail.split('\n')
def last(pat):
    r=None
    for l in lines:
        if re.search(pat,l): r=l
    return r
try: rec=json.load(open(R+'/.cache/f/graph-server/read-record.json'))
except Exception:
    print(time.strftime('%H:%M:%SZ',time.gmtime()),'| no read running; log tail:',(lines[-2] if len(lines)>1 else '')[:120]); sys.exit(0)
now=time.time()
scr=[l for l in lines if l.startswith('[script]')]; phase=(scr[-1][9:] if scr else '')[:110]
led=last(r'^\[owners\] owners \d+ of'); led=re.sub(r'^\[owners\] owners ','',led or '')[:230]
nums=re.match(r'(\d+) of (\d+) done: (\d+) present, (\d+) fetched from the fleet, (\d+) inferred here \((\d+) running\), (\d+) wanted from the fleet, (\d+) stale',led)
st=sum(1 for l in lines if 'does not hold here is' in l)
hf=S+'hist.json'
try: hist=json.load(open(hf))
except Exception: hist=[]
rate=''
if nums:
    done=int(nums.group(1));tot=int(nums.group(2));hist.append([now,done]);hist=hist[-600:];json.dump(hist,open(hf,'w'))
    old=[h for h in hist if h[0]<=now-600];b=old[-1] if old else hist[0]
    if now-b[0]>60:
        r=(done-b[1])/((now-b[0])/60);rate='%.2f/min over %dmin'%(r,(now-b[0])/60)
        if r>0: rate+=', left %d, eta %.1f h'%(tot-done,(tot-done)/r/60)
print(time.strftime('%H:%M:%SZ',time.gmtime(now)),'| phase:',phase)
print('  ledger:',led,'| stale-row lines',st,'| rate',rate)
root='/Users/Shared/semio-store/v1/origins';T=[0,0,0];rows=[]
try:
    wave=(json.load(open(root+'/g1-server/pool.json')).get('wave') or '')[:12]
    for f in sorted(glob.glob(root+'/*/pool.json')):
        d=json.load(open(f));o=f.split('/')[-2];age=now-os.path.getmtime(f)
        if (d.get('wave') or '').startswith(wave) and age<120:
            T[0]+=d.get('claimed',0);T[1]+=d.get('published',0);T[2]+=d.get('children',0);rows.append('%s:%s/%s'%(o[-3:],d.get('claimed'),d.get('published')))
    print('  pools wave',wave,'claimed/published/children',T,' '.join(rows))
except Exception as e: print('  pools:',e)
print('  load',os.getloadavg())
