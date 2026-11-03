#!/usr/bin/env python3
import json,os,subprocess,collections,time,re
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
H=C+"/1102-tools/winplan"
subprocess.run(["python3",H+"/scan.py"],check=True,capture_output=True)
rows=json.load(open(C+"/🗑️generated/1102/winplan-scan.json"))
NOTE=json.load(open(H+"/notes.json"))
import glob
_L=[l for l in open(H+"/order.txt").read().splitlines() if l.strip() and "1102-tools/cadi18n/patches/" not in l]
_cad=sorted(glob.glob(C+"/1102-tools/cadi18n/patches/*.patch"))
_b8=["B8 "+os.path.relpath(p,C) for p in _cad if not os.path.basename(p).startswith("web-")]
_b10=["B10 "+os.path.relpath(p,C) for p in _cad if os.path.basename(p).startswith("web-")]
_sp=C+"/1102-tools/phase/patches/supersedes.txt"
SUPER={}
if os.path.exists(_sp):
  for _l in open(_sp).read().splitlines():
    _x=_l.split()
    if len(_x)==2:SUPER["1102-tools/policy/patches/"+_x[1]]="1102-tools/phase/patches/"+_x[1]
_L=[l for l in _L if l.split(None,1)[1] not in SUPER and "1102-tools/phase/patches/" not in l]
_po=C+"/1102-tools/phase/patches/order.txt"
_ph=[n.strip() for n in open(_po).read().splitlines() if n.strip()] if os.path.exists(_po) else []
_ph=[n for n in _ph if os.path.exists(C+"/1102-tools/phase/patches/"+n+".patch")]
_pf=["B1P 1102-tools/phase/patches/"+n+".patch" for n in _ph if n.startswith("semio-framework-")]
_pp=["B5P 1102-tools/phase/patches/"+n+".patch" for n in _ph if not n.startswith("semio-framework-")]
_i2=max(i for i,l in enumerate(_L) if l.startswith("B2 "))+1
_L=_L[:_i2]+_pf+_L[_i2:]
_i5=max(i for i,l in enumerate(_L) if l.startswith("B5 "))+1
_L=_L[:_i5]+_pp+_L[_i5:]
_i7=max(i for i,l in enumerate(_L) if l.startswith("B7 "))+1
_L=_L[:_i7]+_b8+_L[_i7:]+_b10
_pri=["emitter-request-bytes","retention-intermediates","worker-spare-instances"]
_b0=[l for l in _L if l.startswith("B0 ")];_b0.sort(key=lambda l:next((i for i,n in enumerate(_pri) if n in l),9))
_L=_b0+[l for l in _L if not l.startswith("B0 ")]
open(H+"/order.txt","w").write("\n".join(_L)+"\n")
auto=[]
_have=set(l.split(None,1)[1] for l in open(H+"/order.txt").read().splitlines() if l.strip())
for r in rows:
  if r['sup'] or r['kind']!='diff' or r['rel'] in _have or r['rel'] in NOTE or r['check']=='ALREADY-APPLIED' or r['rel'] in SUPER or r['lane'] not in ('policy','actinfra'):continue
  n=os.path.basename(r['rel'])
  b=("B1P" if n.startswith("semio-framework-") else "B5P") if r['lane']=="phase" else "B0" if r['lane']=="actinfra" else "B9" if n.startswith("ts-") else "B1" if n.startswith("semio-framework-") else "B6" if "stdio" in n else "B5"
  auto.append(f"{b} {r['rel']}")
if auto:
  open(H+"/order.txt","a").write("\n".join(auto)+"\n")
  open(H+"/auto.txt","a").write("\n".join(auto)+"\n")
seq=subprocess.run([H+"/seq.sh"],capture_output=True,text=True).stdout
open(C+"/🗑️generated/1102/winplan-seq.txt","w").write(seq)
order=[l.split(None,1) for l in open(H+"/order.txt").read().splitlines() if l.strip()]
bmap={p:b for b,p in order}
seqres={}
for l in seq.splitlines():
  m=re.match(r"(B\d+P?) (OK|FAIL) (\S+?)(?::|$)",l)
  if m:seqres[m.group(3)]=m.group(2)
def status(r):
  n=NOTE.get(r['rel'])
  if r['rel'] in SUPER:return 'SUPERSEDED by phase twin','-','skip; phase `'+os.path.basename(r['rel'])+'` replaces it (phase supersedes.txt)'
  if n:return n['state'],n.get('batch',''),n.get('note','')
  if r['sup']:return "SUPERSEDED","-","policy `_superseded/` copy of a renamed patch"
  if r['kind']=='diff':
    b=bmap.get(r['rel'],"?")
    if r['check'].startswith("FAIL") and b!="?" and seqres.get(r['rel'])=="OK":return "PENDING (alone: FAIL, needs earlier batch)",b,"seq OK; see section 4"
    st={"CLEAN":"PENDING (check clean)","ALREADY-APPLIED":"ALREADY APPLIED"}.get(r['check'],r['check'])
    return st,b,"seq "+seqres.get(r['rel'],"-")
  return "UNCLASSIFIED","?","new non-diff patch: read it"
out=[]
w=out.append
now=time.strftime("%H:%M",time.localtime())
nok=sum(v=="OK" for v in seqres.values());nfail=sum(v=="FAIL" for v in seqres.values())
w(f"STATUS: PLAN CURRENT as of {now} Nov 2 ({len(rows)} patch files scanned, regenerate: `python3 1102-tools/winplan/gen.py`); {nok} of {nok+nfail} pending diffs apply in sequence on a scratch copy ({nfail} fail); compile UNVERIFIED (freeze).\n")
w("# 1102 Patch Window Plan (lane winplan)\n")
w("Tools: `1102-tools/winplan/{scan.py,seq.sh,gen.py,order.txt,notes.json}`. Raw: `🗑️generated/1102/winplan-scan.json`, `winplan-seq.txt`. Paths below are relative to C; patch paths are `a/ b/` from the repo root unless noted (apply with `cd /Users/ueli/Documents/semio && git apply <patch>`; the 1101 `admission` diff needs `-p0` from `🧰️framework`).\n")
w("## 0. Verdict\n")
w(f"- Pending diffs on the current tree: policy {sum(1 for r in rows if r['lane']=='policy' and not r['sup'] and r['kind']=='diff')}, cadi18n 22, uii18n 6, testkit 1, gen-brep 1 (1101). The set of {nok} diffs applies cleanly **in the order of section 3** on a scratch copy of the touched files (`winplan-seq.txt`, {nok} OK / {nfail} FAIL).")
w(f"- TWINS: lane phase (37 patches, own order in `phase/patches/order.txt`) supersedes {len(SUPER)} policy patches (`supersedes.txt`); those policy twins are SKIPPED. Phase batches B1P/B5P follow policy B2/B5.")
w("- ONE ordering trap: `cadi18n/cad-mode-edit.patch` FAILS alone on the current tree (`git apply --check`, hunk @539); it only applies after `uii18n/cad-step-labels-1` and `-2`. Apply uii18n before cadi18n, never the reverse (`cadi18n/verify.sh` already assumes this).")
w("- ONE duplicate: `rtpanes/laws-publish-framework-schema-documents.md` is superseded by `testkit/publish-framework-schemas.patch` (same file, same fix). Apply only the testkit patch; the rtpanes md is DO NOT APPLY.")
w("- Obsolete/already applied: see the inventory state column. `rtpanes/by-example-law-redo.md` is already in the tree (`redo must reapply` present in `🪧️pane-smoke-law/🦀️.rs`).")
w("- Script patches are fail-closed (assert exactly one anchor) and run on a scratch copy of their target OK: `testport/cursor-dev-vcs-testkit.py`, `flowext/directed-dag-facade-exports.py`; `stdio/decouple-mesh-export-from-pdf.py` dry-run plans 17 edits (default is dry-run, `--apply` writes).\n")
w("## 1. Inventory\n")
w("State: PENDING = not in tree, applicable now; APPLIED/OBSOLETE/SUPERSEDED = do not apply. Batch refers to section 3.\n")
w("| gen | lane | patch (under `<gen>-tools/<lane>/patches/`) | kind | targets | state | batch | note |")
w("|---|---|---|---|---|---|---|---|")
for r in sorted(rows,key=lambda r:(r['sup'],r['gen']!='1102',r['lane'],r['rel'])):
  if r['sup']:continue
  st,b,n=status(r)
  tg=len(r['targets']) if r['targets'] else NOTE.get(r['rel'],{}).get('targets','?')
  w(f"| {r['gen']} | {r['lane']} | `{os.path.basename(r['rel'])}` | {r['kind']} | {tg} | {st} | {b} | {n} |")
sup=[r for r in rows if r['sup']]
w(f"\n`policy/patches/_superseded/` ({len(sup)} files incl. `.check` markers excluded): old-name copies of the 17 renamed policy patches and `unknown.patch` (163 KB); never apply.\n")
w("## 2. File-overlap matrix\n")
t=collections.defaultdict(list)
for r in rows:
  if r['sup'] or r['rel'] in SUPER or r['kind']!='diff' or r['check']=="ALREADY-APPLIED" or NOTE.get(r['rel'],{}).get('state','').startswith(("APPLIED","OBSOLETE")):continue
  for f in set(r['targets']):t[f].append(r)
ov=[(f,l) for f,l in t.items() if len(l)>1]
w("Diff x diff (same target file in more than one pending patch):\n")
w("| target (tail) | patches | hunks (old line ranges) | overlap |")
w("|---|---|---|---|")
for f,l in ov:
  hs=[(x['lane']+'/'+os.path.basename(x['rel']),x['hunks'][f]) for x in l]
  inter=False
  for i in range(len(hs)):
    for j in range(i+1,len(hs)):
      for a in hs[i][1]:
        for b in hs[j][1]:
          if a[0]<b[1] and b[0]<a[1]:inter=True
  w(f"| `…{f[-70:]}` | {', '.join(h[0] for h in hs)} | {'; '.join(h[0]+' '+str(h[1]) for h in hs)} | {'HUNKS OVERLAP' if inter else 'files shared, hunks disjoint'} |")
if not ov:w("| none | | | |")
w("\nOther collisions (script/plan vs diff, same crate or same file, from reading the targets):\n")
for l in NOTE.get("_overlaps",[]):w("- "+l)
w("\n## 3. Apply order (batches, single owner each)\n")
w("Rule: apply patches of a batch one by one with `git apply`, then run the narrow check once for the batch. Never run two batches at once (one cargo per owner, slot.sh). If a check fails, bisect inside the batch in the listed order.\n")
for b in NOTE["_batches"]:
  w(f"### {b['id']} {b['title']} (owner: `{b['owner']}`)")
  w("Patches, in order:")
  for bb,p in order:
    if bb==b['id']:w(f"1. `{p}` ({seqres.get(p,'?')})")
  for x in b.get('extra',[]):w(f"1. {x}")
  w(f"- Check: {b['check']}")
  w(f"- Why here: {b['why']}\n")
w("## 4. Conflicting / obsolete against the current tree\n")
for l in NOTE["_conflicts"]:w("- "+l)
w("\n`git apply --check` result per diff (current tree, each alone):\n")
cc=collections.Counter(r['check'] for r in rows if r['kind']=='diff' and not r['sup'])
for k,v in cc.items():w(f"- {k[:150]}: {v}")
w("\n## 5. Risky patches (compile risk named by authors or by reading)\n")
for l in NOTE["_risks"]:w("- "+l)
w("\n## 6. New since last refresh / unplaced\n")
un=[r for r in rows if not r['sup'] and r['rel'] not in SUPER and r['kind']=='diff' and r['rel'] not in bmap and r['rel'] not in NOTE and r['check']!='ALREADY-APPLIED']
un+=[r for r in rows if not r['sup'] and r['kind']!='diff' and r['rel'] not in NOTE]
for r in un:w(f"- `{r['rel']}` ({r['kind']}, {r['check'] or 'n/a'}): needs a batch")
if not un:w("- none")
w("\n## 7. Refresh log\n")
w(f"- {now}: regenerated; {len(rows)} files, {len(un)} unplaced.")
open(C+"/1102-window-plan.md","w").write("\n".join(out)+"\n")
print(len(rows),len(un))
