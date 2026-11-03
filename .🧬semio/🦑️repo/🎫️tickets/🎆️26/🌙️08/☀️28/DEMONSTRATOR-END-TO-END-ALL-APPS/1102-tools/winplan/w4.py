#!/usr/bin/env python3
import json,os,subprocess,time,re,collections
R="/Users/ueli/Documents/semio"
C=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
H=C+"/1102-tools/winplan"
SB="/private/tmp/claude-501/-Users-ueli-Documents-semio/2c8171ff-0be5-4b23-8c30-2258199d4040/scratchpad/w4"
subprocess.run(["python3",H+"/scan.py"],check=True,capture_output=True)
rows=[r for r in json.load(open(C+"/🗑️generated/1102/winplan-scan.json")) if r['gen']=='1102' and not r['sup']]
OV=json.load(open(H+"/w4-notes.json"))
def run_script(rel,args=()):
  d=SB+"/"+re.sub(r"\W","_",rel)
  subprocess.run(["rm","-rf",d])
  p=subprocess.run(["python3","-I",H+"/sandbox.py",d,C+"/"+rel,*args],capture_output=True,text=True)
  out=p.stdout.strip().split("\n")
  written=[];msg=""
  for l in out:
    if l.startswith("WRITTEN:"):written=json.loads(l[8:])
    elif l.strip():msg=l
  return p.returncode,msg,written
res=[]
for r in sorted(rows,key=lambda r:(r['lane'],r['rel'])):
  rel=r['rel'];name=os.path.basename(rel);o=OV.get(rel,{})
  st=o.get('state');note=o.get('note','');wr=[];batch=o.get('batch','-')
  if r['kind']=='diff':
    if not st:
      if r['check']=='ALREADY-APPLIED':st="APPLIED"
      elif r['check']=='CLEAN':
        tg=[t for t in r['targets'] if os.path.exists(R+"/"+t)]
        newfile=open(C+"/"+rel,errors="replace").read().count("--- /dev/null")>0
        st="APPLIED (new files exist)" if newfile and tg else "PENDING"
      else:st="APPLIED, later edited (old lines gone)"
    wr=r['targets']
  elif r['kind']=='script' and not st:
    args=[R] if name=="prototype-wasm-debug0.py" else []
    if name in("decouple-mesh-export-from-pdf.py",):st="APPLIED? (dry-run script)";
    else:
      rc,msg,wr=run_script(rel,args)
      st="PENDING (applies on scratch)" if rc==0 else "APPLIED/DRIFTED (anchor assert fails: "+msg[:60]+")"
  elif not st:st="UNCLASSIFIED (read it)"
  if r['kind']=='script' and st.startswith('PENDING') and not wr:
    rc,msg,wr=run_script(rel,[R] if name=='prototype-wasm-debug0.py' else [])
    if rc!=0:st='PENDING-BROKEN ('+msg[:60]+')'
  res.append(dict(rel=rel,lane=r['lane'],kind=r['kind'],state=st,note=note,batch=batch,written=wr,mtime=r['mtime']))
json.dump(res,open(C+"/🗑️generated/1102/winplan-w4.json","w"),ensure_ascii=False,indent=1)
pend=[x for x in res if x['state'].startswith("PENDING")]
new=[x for x in res if x['state'].startswith("UNCLASSIFIED") or (x['rel'] not in OV and x['state'].startswith("PENDING") and x['rel'] not in OV)]
t=collections.Counter(x['state'].split(" ")[0] for x in res)
now=time.strftime("%H:%M")
ex=os.path.exists(C+"/🗑️generated/1102/act20.status") and "exit" in open(C+"/🗑️generated/1102/act20.status").read()
o=[]
w=o.append
w(f"STATUS: W4 PLAN CURRENT as of {now} Nov 3 ({len(res)} 1102 patch files; {len(pend)} pending, rest applied/superseded/held); act20 {'ENDED' if ex else 'running'}; compile UNVERIFIED (no cargo).\n")
w("# 1102 Patch Window W4 Plan (lane winplan)\n")
w("Method as in `1102-window-plan.md` (W3). Tools: `1102-tools/winplan/{w4.py,sandbox.py,w4-notes.json,drift.py}`; regenerate with `python3 1102-tools/winplan/w4.py`. Diffs: `git apply --check` forward and `-R` (applied) plus a '+'/'-' line presence check for drifted ones. Scripts/plans: each python script is run in a copy-on-read sandbox (`sandbox.py`: reads copy the repo file into a scratch dir, writes go to scratch, the repo is never touched); rc 0 = applies now, AssertionError = already applied or drifted. Raw: `🗑️generated/1102/winplan-w4.json`.\n")
w("## 0. Verdict\n")
for l in OV["_verdict"]:w("- "+l)
w("\n## 1. Pending patches (the W4 work)\n")
w("| lane | patch | kind | batch | writes | note |\n|---|---|---|---|---|---|")
for x in pend:
  w(f"| {x['lane']} | `{os.path.basename(x['rel'])}` | {x['kind']} | {x['batch']} | {len(x['written'])} file(s) | {x['note']} |")
w("\n## 2. Held / absent / not patches\n")
for l in OV["_held"]:w("- "+l)
w("\n## 3. File-overlap matrix (pending x pending, by written file)\n")
f2=collections.defaultdict(list)
for x in pend:
  for f in set(x['written']):f2[f].append(os.path.basename(x['rel']))
ovl=[(f,l) for f,l in f2.items() if len(l)>1]
w("| file (tail) | patches | verdict |\n|---|---|---|")
for f,l in ovl:w(f"| `…{f[-70:]}` | {', '.join(l)} | {OV['_overlap_verdicts'].get(l[0]+'+'+l[1],'CHECK')} |")
if not ovl:w("| none | | |")
w("\nDeclared/observed collisions (sequential sandbox run):\n")
for l in OV["_collisions"]:w("- "+l)
w("\n## 4. Ordered batches (single owner, narrow check)\n")
for b in OV["_batches"]:
  w(f"### {b['id']} {b['title']} (owner: `{b['owner']}`)")
  for i,p in enumerate(b['items'],1):w(f"{i}. {p}")
  w(f"- Check: {b['check']}")
  w(f"- Why here: {b['why']}\n")
w("## 5. Risky patches\n")
for l in OV["_risks"]:w("- "+l)
w("\n## 6. Applied / obsolete inventory (do not apply)\n")
w("| lane | patch | state |\n|---|---|---|")
for x in res:
  if not x['state'].startswith("PENDING"):w(f"| {x['lane']} | `{os.path.basename(x['rel'])}` | {x['state'][:90]} |")
w("\n## 7. New / unclassified since last refresh\n")
un=[x for x in res if x['state'].startswith("UNCLASSIFIED") or (x['state'].startswith("PENDING") and x['rel'] not in OV)]
for x in un:w(f"- `{x['rel']}` ({x['kind']}, {x['state']}): place in a batch")
if not un:w("- none")
w(f"\n## 8. Refresh log\n\n- {now}: regenerated; {len(res)} files, {len(pend)} pending, {len(un)} unplaced.")
_old=open(C+"/1102-window-plan-w4.md").read().split("## 8. Refresh log")[1].strip().split("\n")[1:] if os.path.exists(C+"/1102-window-plan-w4.md") else []
o+= [l for l in _old if l.startswith("- ")]
open(C+"/1102-window-plan-w4.md","w").write("\n".join(o)+"\n")
print(len(res),len(pend),len(un))
