import sys,re,json,subprocess,os
# 📝️ done.py <state> <note> name... : re-measures the named crates (cen.sh) and records state, worst ratio and note in status.json.
F=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
st=F+"/hdr-tools/status.json"
S=json.load(open(st)) if os.path.exists(st) else {}
state,note=sys.argv[1],sys.argv[2]; names=sys.argv[3:]
out=subprocess.run([F+"/hdr-tools/cen.sh"]+names,capture_output=True,text=True).stdout
cur=None
res={}
for line in out.split("\n"):
    if line.startswith("## "): cur=line[3:]; res[cur]=[]; continue
    m=re.match(r"# (lib|test): (\d+) files (\d+) bytes (\d+) lines",line)
    if m and cur:
        f,b,l=int(m.group(2)),int(m.group(3)),int(m.group(4))
        res[cur].append((m.group(1),f,b,l,max(f/32,b/262144,l/4000)))
for n,rows in res.items():
    if not rows: continue
    worst=max(r[4] for r in rows)
    desc="; ".join(f"{r[0]} {r[1]}/{r[2]}/{r[3]}" for r in rows)
    S[n]={'state':(('DONE' if worst<0.90 else 'PARTIAL') if state=='AUTO' else state),'after':f"{worst:.3f} ({desc})",'note':note}
    print(f"{n} {worst:.3f}")
json.dump(S,open(st,"w"),ensure_ascii=False,indent=1)
