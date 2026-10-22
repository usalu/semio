#!/usr/bin/env python3
"""📐 Independent Python oracle of the performance SLO table and bench: statistics (the standard library's `statistics.quantiles`, inclusive method, and with `stats-numpy` NumPy's linear percentile), host load classes, verdicts, series, baseline comparison, objective selection, source edits, tool marker lines and process-tree memory, from the same schema and fixture, using only the standard library (NumPy only for `stats-numpy`). Reads one JSON request on stdin, writes one JSON answer on stdout."""
import json,re,statistics,sys

def key(text): return text.encode("utf-16-be","surrogatepass")
request=json.load(sys.stdin)
policy=json.load(open(request["schemaPath"],encoding="utf8"))["definitions"]["Policy"]["const"]
METRICS=["wallMs","startMs","greenMs","peakRssBytes","hitRatio","makespanMs","maxUnrelatedWaitMs","speedup"]

def stats(values):
    xs=sorted(values);n=len(xs)
    if n==0: return None
    if n==1: q={50:xs[0],95:xs[0],99:xs[0]}
    else:
        cuts=statistics.quantiles(xs,n=100,method="inclusive");q={50:cuts[49],95:cuts[94],99:cuts[98]}
    total=0.0
    for x in xs: total+=x
    return {"n":n,"min":xs[0],"p50":q[50],"p95":q[95],"p99":q[99],"max":xs[-1],"mean":total/n}

def load_class(per_core):
    if per_core is None: return "unknown"
    if per_core<policy["loadPerCore"]["quietBelow"]: return "quiet"
    if per_core>=policy["loadPerCore"]["saturatedFrom"]: return "saturated"
    return "busy"

def series(case):
    good=[s for s in case["samples"] if s["ok"]]
    st={}
    for m in METRICS:
        vals=[s["values"][m] for s in good if m in s["values"]]
        if vals: st[m]=stats(vals)
    loads=[s["loadPerCore"] for s in case["samples"] if s["loadPerCore"] is not None]
    kinds={"marker":0,"first-output":0,"exit":0}
    for s in good:
        if s["startKind"]: kinds[s["startKind"]]+=1
    return {"probe":case["probe"],"tool":case["tool"],"lane":case["lane"],"status":case["status"],"reason":case["reason"],"samples":case["samples"],"failed":len(case["samples"])-len(good),"stats":st,"loadClass":load_class(max(loads) if loads else None),"startKinds":kinds}

def judge(slo,tool,s):
    unit=policy["units"][slo["metric"]]
    row={"slo":slo["id"],"tool":tool,"probe":slo["probe"],"metric":slo["metric"],"statistic":slo["statistic"],"direction":slo["direction"],"unit":unit,"value":None,"goal":slo["goal"],"gate":slo["gate"],"n":0,"status":"unproven","reason":None,"inflated":False,"enforcement":slo["enforcement"],"loadClass":s["loadClass"] if s else "unknown"}
    if s is None or s["status"]!="measured":
        row["reason"]=(s or {}).get("reason") or "unavailable";return row
    st=s["stats"].get(slo["metric"])
    if st is None:
        row["reason"]="probe-failed" if s["failed"]>0 else "no-samples";return row
    row["n"]=st["n"]
    if st["n"]<policy["minimumSamples"][slo["statistic"]]:
        row["reason"]="too-few-samples";row["value"]=st[slo["statistic"]];return row
    v=st[slo["statistic"]];row["value"]=v
    inflated=unit=="ms" and s["loadClass"]=="saturated"
    lower=slo["direction"]=="lower"
    def within(limit): return v<=limit if lower else v>=limit
    goal,gate=slo["goal"],slo["gate"]
    k=s["startKinds"];seen=sum(k.values())
    bound="exact" if slo["metric"]!="startMs" or k["marker"]==seen else "lower" if k["first-output"]==seen else "upper" if k["exit"]==seen else "mixed"
    if goal is None and gate is None: status="recorded"
    elif goal is not None and within(goal): status="pass"
    elif gate is not None and within(gate): status="pass" if goal is None else "warn"
    else: status="fail"
    if bound!="exact" and status!="recorded" and not ((bound=="lower" and status=="fail") or (bound=="upper" and status!="fail")):
        row["status"]="unproven";row["reason"]="start-unobserved";return row
    if status=="fail" and inflated:
        row["status"]="unproven";row["reason"]="host-saturated";row["inflated"]=True;return row
    row["status"]=status;row["inflated"]=bool(inflated and status not in("pass","recorded"));return row

def drift(direction,base,cur,tol):
    up=base*(1+tol["relative"])+tol["absolute"];down=base*(1-tol["relative"])-tol["absolute"]
    if direction=="lower": worse,better=cur>up,cur<down
    else: worse,better=cur<down,cur>up
    return "regressed" if worse else "improved" if better else "same"

def compare(slos,cur,base):
    by={s["id"]:s for s in slos};k=lambda r:(r["slo"],r["tool"])
    before={k(r):r for r in base["rows"]};now={k(r) for r in cur["rows"]}
    same=cur["host"]["hostClass"]==base["host"]["hostClass"];rows=[]
    def sat(report,row):
        for s in report["series"]:
            if s["probe"]==row["probe"] and s["tool"]==row["tool"]: return s["loadClass"]=="saturated"
        return False
    def made(row,tol,status,b,c,reason):
        return {"slo":row["slo"],"tool":row["tool"],"metric":row["metric"],"statistic":row["statistic"],"direction":row["direction"],"unit":row["unit"],"base":b,"current":c,"delta":None if b is None or c is None else c-b,"relative":None if b is None or c is None or b==0 else (c-b)/b,"toleranceRelative":tol["relative"],"toleranceAbsolute":tol["absolute"],"status":status,"enforcement":row["enforcement"],"reason":reason}
    for row in cur["rows"]:
        tol=by[row["slo"]]["tolerance"] if row["slo"] in by else policy["defaultTolerance"]
        old=before.get(k(row))
        if old is None: rows.append(made(row,tol,"new",None,row["value"],None))
        elif not same: rows.append(made(row,tol,"incomparable",old["value"],row["value"],"host-class"))
        elif old["status"]=="unproven" or row["status"]=="unproven" or old["value"] is None or row["value"] is None: rows.append(made(row,tol,"incomparable",old["value"],row["value"],row["reason"] or old["reason"] or "unavailable"))
        else:
            verdict=drift(row["direction"],old["value"],row["value"],tol)
            if verdict=="regressed" and row["unit"]=="ms" and (sat(cur,row) or sat(base,old)): rows.append(made(row,tol,"inconclusive",old["value"],row["value"],"host-saturated"))
            else: rows.append(made(row,tol,verdict,old["value"],row["value"],None))
    probes={s["probe"] for s in cur["series"]}
    for row in base["rows"]:
        if k(row) in now or row["probe"] not in probes: continue
        tol=by[row["slo"]]["tolerance"] if row["slo"] in by else policy["defaultTolerance"]
        rows.append(made(row,tol,"missing",row["value"],None,None))
    names=["improved","same","regressed","inconclusive","incomparable","new","missing"]
    tally={n:sum(1 for r in rows if r["status"]==n) for n in names}
    tally["blockingRegressed"]=sum(1 for r in rows if r["status"]=="regressed" and r["enforcement"]=="blocking")
    return {"version":1,"currentHostClass":cur["host"]["hostClass"],"baselineHostClass":base["host"]["hostClass"],"rows":rows,"tally":tally}

def select(case):
    sset,ids,tools=case["set"],case["ids"],case["tools"]
    probes={p["id"]:p for p in sset["probes"]}
    chosen=set()
    if ids=="all": chosen={s["id"] for s in sset["slos"]}
    else:
        for i in ids:
            if i.startswith("kind:"): m=[s for s in sset["slos"] if probes[s["probe"]]["kind"]==i[5:]]
            elif i.startswith("probe:"): m=[s for s in sset["slos"] if s["probe"]==i[6:]]
            else: m=[s for s in sset["slos"] if s["id"]==i]
            if not m: return {"error":"unknown","pairs":[]}
            chosen|={s["id"] for s in m}
    pairs={}
    for s in sset["slos"]:
        if s["id"] not in chosen: continue
        for t in s["tools"]:
            if t!="neutral" and t not in tools: continue
            pairs.setdefault((s["probe"],t),[]).append(s["id"])
    if not pairs: return {"error":"empty","pairs":[]}
    position={p["id"]:n for n,p in enumerate(sset["probes"])};rank={k:n for n,k in enumerate(policy["order"])}
    ordered=sorted(pairs,key=lambda pt:(0 if probes[pt[0]].get("cold") else 1,rank[probes[pt[0]]["kind"]],position[pt[0]],key(pt[1])))
    return {"error":None,"pairs":[f"{p}|{t}|{','.join(pairs[(p,t)])}" for p,t in ordered]}

def number(nonce): return int(nonce[:6],36)%1000003
def mutate(text,op,language,nonce):
    n=number(nonce);up=nonce.upper()
    tpl={
     "ts":{"comment":f"// [incremental] {nonce}\n","private-body":f"void {n};\n","public-api":f"export const __incremental{nonce}={n} as const;\n"},
     "rs":{"comment":f"// [incremental] {nonce}\n","private-body":f"const _: u32 = {n};\n","public-api":f"/// [incremental] public probe {nonce}\npub const __INCREMENTAL_{up}: u32 = {n};\n"},
     "py":{"comment":f"# [incremental] {nonce}\n","private-body":f"_incremental_{nonce} = {n}\n","public-api":f"INCREMENTAL_{up} = {n}\n"},
     "go":{"comment":f"// [concurrent] {nonce}\n","private-body":f"var _ = {n}\n","public-api":f"const Concurrent{up} = {n}\n"}}
    return (text if text.endswith("\n") or text=="" else text+"\n")+tpl[language][op]

def first_marker(tool,lines):
    pats=[re.compile(p) for p in policy["markers"][tool]]
    for i,line in enumerate(lines):
        if any(p.search(line) for p in pats): return i
    return None

def tree(table,pid,system):
    rows={}
    for line in table.splitlines():
        cells=line.replace('"',"").split(",") if system=="win32" else line.split()
        if len(cells)<3: continue
        try: a,b,c=[float(x) for x in cells[:3]]
        except ValueError: continue
        rows[int(a)]=(int(b),int(c) if system=="win32" else int(c)*1024)
    if pid not in rows: return None
    seen={pid};frontier=[pid]
    while frontier:
        nxt=[]
        for p,(parent,_) in rows.items():
            if parent in frontier and p not in seen: seen.add(p);nxt.append(p)
        frontier=nxt
    return sum(rows[p][1] for p in seen)

op,cases=request["op"],request["cases"]
if op=="stats": out=[stats(c["values"]) for c in cases]
elif op=="stats-numpy":
    try: import numpy as np
    except ImportError: out=None
    else:
        out=[]
        for c in cases:
            a=np.array(sorted(c["values"]),dtype=float)
            out.append({"n":int(a.size),"min":float(a[0]),"p50":float(np.percentile(a,50,method="linear")),"p95":float(np.percentile(a,95,method="linear")),"p99":float(np.percentile(a,99,method="linear")),"max":float(a[-1]),"mean":float(a.mean())})
elif op=="load": out=[load_class(c["perCore"]) for c in cases]
elif op=="series": out=[series(c) for c in cases]
elif op=="judge": out=[judge(c["slo"],c["tool"],c["series"]) for c in cases]
elif op=="compare": out=[compare(c["slos"],c["current"],c["baseline"]) for c in cases]
elif op=="select": out=[select(c) for c in cases]
elif op=="mutate": out=[mutate(c["text"],c["op"],c["language"],c["nonce"]) for c in cases]
elif op=="markers": out=[first_marker(c["tool"],c["lines"]) for c in cases]
elif op=="tree": out=[tree(c["table"],c["pid"],c["system"]) for c in cases]
else: sys.exit("unknown op "+op)
json.dump(out,sys.stdout,ensure_ascii=False)
