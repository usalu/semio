import json,os,sys
F=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
G=F+"/🗑️generated"
rows=json.load(open(G+"/hdr-near.json"))
status=json.load(open(F+"/hdr-tools/status.json")) if os.path.exists(F+"/hdr-tools/status.json") else {}
def short(n): return n.replace('semio-framework-','F:').replace('semio-s-artifact-','A:')
def row(r,withstat):
    s=status.get(r['name'],{})
    base=f"| `{short(r['name'])}` | {r['lf']}/{r['lb']}/{r['ll']} | {r['tf']}/{r['tb']}/{r['tl']} | {r['m']:.3f} ({r['tbi'] if r['tr']>=r['lr'] else r['lbi']}) |"
    if withstat:
        return base+f" {s.get('state','TODO')} | {s.get('after','')} | {s.get('note','')} |"
    return base+f" {r['own']} |"
out=[]
head=open(F+"/hdr-tools/parts/head.md").read()
out.append(head)
mine=[r for r in rows if not r['own']]
exc=[r for r in rows if r['own']]
out.append("## 1. Crates at >= 0.90 of any limit, not owned by a running slice (%d)\n\nBefore columns: lib and test closure as files/bytes/active lines (cgc7 reader, `hdr-census.ts`, 2026-10-17 15:1x, 2,873 crates). Ratio = worst of the three limits over lib and test, binding limit in brackets. `after` is the re-measured worst ratio.\n"%len(mine))
out.append("| crate | lib | test | before | state | after | note |\n| --- | --- | --- | --- | --- | --- | --- |")
for r in mine: out.append(row(r,True))
out.append("\n## 2. Excluded (owned by a running slice) (%d)\n"%len(exc))
out.append("| crate | lib | test | ratio | owner |\n| --- | --- | --- | --- | --- |")
for r in exc: out.append(row(r,False))
import csv
csv.field_size_limit(10**9)
cur={}
for r in csv.DictReader(open(G+"/hdr-crate-census.csv")):
    f=lambda x: float(x) if x not in ('','refused') else 0.0
    cur[r['name']]=(max(f(r['libRatio']),f(r['testRatio'])),r['libFiles'],r['libBytes'],r['libLines'],r['testFiles'],r['testBytes'],r['testLines'])
old=json.load(open(G+"/hdr-near-1.json"))
fixed=[(r,cur[r['name']]) for r in old if r['name'] in cur and cur[r['name']][0]<0.9]
fixed.sort(key=lambda x:-x[0]['m'])
out.append("\n## 1b. Crates brought under 0.90 (before -> after, final census; includes crates that were owned by a slice at the start and were freed or fixed later) (%d)\n"%len(fixed))
out.append("| crate | before (worst) | after lib f/b/l | after test f/b/l | after (worst) |\n| --- | --- | --- | --- | --- |")
for r,c in fixed:
    out.append(f"| `{short(r['name'])}` | {r['m']:.3f} | {c[1]}/{c[2]}/{c[3]} | {c[4]}/{c[5]}/{c[6]} | {c[0]:.3f} |")
out.append("\n"+open(F+"/hdr-tools/parts/tail.md").read())
open(F+"/RESULT-CRATE-HEADROOM.md","w").write("\n".join(out)+"\n")
print("ok",len(mine),len(exc))
