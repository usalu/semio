import re,sys
rows=[];t=None;last=None
for l in open("samples.txt",encoding="utf8"):
    m=re.match(r"(\d\d:\d\d:\d\d)Z \| phase: (.*)",l)
    if m: t=m.group(1);continue
    m=re.search(r"ledger: (\d+) of 4785 done: (\d+) present, (\d+) fetched from the fleet, (\d+) inferred here \((\d+) running\), (\d+) wanted from the fleet, (\d+) stale in the fleet \| stale-row lines (\d+)",l)
    if m and t:
        rows.append([t]+[int(x) for x in m.groups()])
    m=re.search(r"pools wave (\w+) claimed/published/children \[(\d+), (\d+), (\d+)\]",l)
    if m and rows and len(rows[-1])==9: rows[-1]+= [int(m.group(3))]
lo=sys.argv[1] if len(sys.argv)>1 else "00:00:00"; hi=sys.argv[2] if len(sys.argv)>2 else "99:99:99"
out=[];seen=set();prev=None
for r in rows:
    if r[0]<lo or r[0]>hi or r[0] in seen: continue
    seen.add(r[0])
    if prev is None or r[0][:4]!=prev[0][:4]:   # about every 10 minutes
        out.append(r);prev=r
print("| time UTC | done | present | fetched | inferred here | stale | stale per 100 taken (fetched+stale) | stale-row lines | pools published (sum of wave) |")
print("|---|---|---|---|---|---|---|---|---|")
for r in out:
    taken=r[3]+r[7]
    print("| %s | %d | %d | %d | %d | %d | %s | %d | %s |"%(r[0],r[1],r[2],r[3],r[4],r[7],("%.0f"%(100*r[7]/taken)) if taken else "-",r[8],r[9] if len(r)>9 else "-"))
