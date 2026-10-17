import json,subprocess,re,os,sys
F=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
st=F+"/hdr-tools/status.json"; S=json.load(open(st))
names=sys.argv[1:] or list(S)
out=subprocess.run([F+"/hdr-tools/cen.sh"]+names,capture_output=True,text=True).stdout
cur=None;res={}
for line in out.split("\n"):
    if line.startswith("## "): cur=line[3:]; res[cur]=[]; continue
    m=re.match(r"# (lib|test): (\d+) files (\d+) bytes (\d+) lines",line)
    if m and cur:
        f,b,l=map(int,m.groups()[1:]); res[cur].append((m.group(1),f,b,l,max(f/32,b/262144,l/4000)))
for n,rows in res.items():
    if not rows: continue
    w=max(r[4] for r in rows)
    e=S.setdefault(n,{'note':''})
    e['after']=f"{w:.3f} ("+"; ".join(f"{r[0]} {r[1]}/{r[2]}/{r[3]}" for r in rows)+")"
    e['state']='DONE' if w<0.90 else ('PARTIAL' if e.get('note') else 'OPEN')
    if w>=0.90: print("STILL",n,round(w,3))
json.dump(S,open(st,"w"),ensure_ascii=False,indent=1)
