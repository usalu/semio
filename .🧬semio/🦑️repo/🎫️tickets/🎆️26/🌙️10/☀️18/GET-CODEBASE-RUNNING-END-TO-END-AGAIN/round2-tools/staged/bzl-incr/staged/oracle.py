"""Independent Python oracle of the incremental conformance model (standard library only): it re-implements the evaluation, the verdict, the mutations, the Bazel labels, the environment scrub and the compact execution log decoder, and prints the results for the fixture rows given as JSON on stdin."""
import base64,hashlib,json,re,subprocess,sys

def effect(sens,task,kind):
    if not task["outputs"]:
        return "identical"
    for row in sens:
        if row["family"]==task["family"] and row["kind"]==kind:
            return row["effect"]
    return "unknown"

def expected(slice_,tool,edit,sens):
    tasks=slice_["tasks"];files=set(edit["files"]);kind=edit["kind"]
    edges={t:(tasks[t]["dependsOn"] if tool=="bazel" else tasks[t]["consumes"]) for t in tasks}
    direct={t for t in tasks if files&set(tasks[t]["inputs"])}
    always={t for t in tasks if not tasks[t]["cache"]}
    def settle(follow):
        changed=set(direct)
        while True:
            fresh={d for d in tasks if d not in changed and any(u in changed and follow(u) for u in edges[d])}
            if not fresh:
                break
            changed|=fresh
        return changed|always
    must=settle(lambda t:effect(sens,tasks[t],kind)=="changed")
    may=settle(lambda t:effect(sens,tasks[t],kind)!="identical")
    cutoff={d for d in tasks if d not in may and any(u in may for u in edges[d])}
    return {"direct":sorted(direct),"must":sorted(must),"may":sorted(may),"cutoff":sorted(cutoff)}

def verdict(exp,executed):
    ran=set(executed);stale=sorted(set(exp["must"])-ran);excess=sorted(ran-set(exp["may"]))
    status="stale-excess" if stale and excess else "stale" if stale else "excess" if excess else "pass"
    return {"status":status,"stale":stale,"excess":excess,"exact":not stale and not excess and len(exp["must"])==len(exp["may"])}

def number(nonce):
    return int(nonce[:6],36)%1000003

def mutate(text,op,language,nonce):
    if op=="whitespace":
        return text+"\n"
    n=number(nonce);up=nonce.upper()
    body={
     ("ts","comment"):"// [incremental] "+nonce+"\n",("ts","private-body"):"void "+str(n)+";\n",("ts","public-api"):"export const __incremental"+nonce+"="+str(n)+" as const;\n",
     ("rs","comment"):"// [incremental] "+nonce+"\n",("rs","private-body"):"const _: u32 = "+str(n)+";\n",("rs","public-api"):"/// [incremental] public probe "+nonce+"\npub const __INCREMENTAL_"+up+": u32 = "+str(n)+";\n",
     ("py","comment"):"# [incremental] "+nonce+"\n",("py","private-body"):"_incremental_"+nonce+" = "+str(n)+"\n",("py","public-api"):"INCREMENTAL_"+up+" = "+str(n)+"\n",
     ("text","comment"):"# [incremental] "+nonce+"\n",
    }[(language,op)]
    return (text if text.endswith("\n") or text=="" else text+"\n")+body

def label(kind,value):
    return kind+"_"+hashlib.sha256(value.encode()).hexdigest()[:16]

def child(parent,extra):
    kept={k:v for k,v in parent.items() if k=="SEMIO_LANGUAGE" or not re.match(r"(SEMIO_|NX_)",k)}
    kept.update(extra);return kept

def varint(b,i):
    r=0;s=0
    while True:
        c=b[i];i+=1;r|=(c&0x7f)<<s;s+=7
        if not c&0x80:
            return r,i

def fields(b):
    i=0;out=[]
    while i<len(b):
        k,i=varint(b,i);f,t=k>>3,k&7
        if t==0:
            v,i=varint(b,i)
        elif t==2:
            l,i=varint(b,i);v=b[i:i+l];i+=l
        elif t==1:
            v=b[i:i+8];i+=8
        elif t==5:
            v=b[i:i+4];i+=4
        else:
            raise ValueError("wire type")
        out.append((f,t,v))
    return out

def spawns(compact_b64):
    raw=subprocess.run(["zstd","-dc"],input=base64.b64decode(compact_b64),capture_output=True,check=True).stdout
    i=0;rows=[]
    while i<len(raw):
        l,i=varint(raw,i);entry=raw[i:i+l];i+=l
        for f,t,v in fields(entry):
            if f==7 and t==2:
                row=fields(v);get=lambda n:next((x.decode() for g,u,x in row if g==n and u==2),"")
                rows.append({"label":get(7),"mnemonic":get(8),"runner":get(11),"cacheHit":any(g==12 and u==0 and x==1 for g,u,x in row)})
    return rows

def observation(rows,tasks):
    names={"//:"+label("t",t):t for t in tasks};executed=set();hit=set();unmapped=set()
    for r in rows:
        t=names.get(re.sub(r"^@@?//","//",r["label"]))
        if t is None:
            if r["label"]:
                unmapped.add(r["label"])
            continue
        (hit if r["cacheHit"] else executed).add(t)
    return {"executed":sorted(executed),"restored":sorted(hit-executed),"cached":sorted(set(tasks)-executed-hit),"unmapped":sorted(unmapped)}

def nx(record,tasks):
    known=set(tasks);rows=[r for r in record["tasks"] if r["taskId"] in known]
    return {"executed":sorted(r["taskId"] for r in rows if r["cacheStatus"]=="cache-miss"),"restored":sorted(r["taskId"] for r in rows if r["cacheStatus"]!="cache-miss"),"cached":[],"unmapped":sorted(r["taskId"] for r in record["tasks"] if r["taskId"] not in known)}

def comparisons(rows):
    ok=lambda r:r["verdict"]["status"] not in ("failed","skipped");out=[]
    for n in rows:
        if n["tool"]!="nx" or not ok(n):
            continue
        b=next((r for r in rows if r["tool"]=="bazel" and r["scenario"]==n["scenario"] and r["phase"]==n["phase"] and ok(r)),None)
        if b is None:
            continue
        only_nx=sorted(set(n["observation"]["executed"])-set(b["observation"]["executed"]));only_bazel=sorted(set(b["observation"]["executed"])-set(n["observation"]["executed"]))
        out.append({"scenario":n["scenario"],"phase":n["phase"],"same":not only_nx and not only_bazel,"onlyNx":only_nx,"onlyBazel":only_bazel})
    return out

if __name__=="__main__":
    fx=json.load(sys.stdin)
    out={
     "evaluation":[expected(c["slice"],c["tool"],c["edit"],fx["sensitivity"]) for c in fx["evaluation"]],
     "verdicts":[verdict(c["expected"],c["executed"]) for c in fx["verdicts"]],
     "mutations":[mutate(c["before"],c["op"],c["language"],c["nonce"]) for c in fx["mutations"]],
     "labels":[label("t",c["task"]) for c in fx["labels"]],
     "environments":[child(c["parent"],c["extra"]) for c in fx["environments"]],
     "bazelLogs":[{"spawns":spawns(c["compact"]),"observation":observation(spawns(c["compact"]),c["tasks"])} for c in fx["bazelLogs"]],
     "nxRecords":[nx(c["record"],c["tasks"]) for c in fx["nxRecords"]],
     "comparisons":comparisons(fx.get("comparisonRows",[])),
    }
    json.dump(out,sys.stdout)
