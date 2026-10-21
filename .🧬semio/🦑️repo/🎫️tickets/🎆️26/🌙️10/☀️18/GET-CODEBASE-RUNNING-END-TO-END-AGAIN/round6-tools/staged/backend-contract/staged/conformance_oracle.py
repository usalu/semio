"""🐍️ Independent oracle for the conformance suite: with only the standard library it recomputes the reference output of every stub task from the workspace files, the canonical action-key serialisation, the output digest, the marker concurrency and the evaluation of a run expectation, so the TypeScript implementation is checked against a second implementation."""
import hashlib,json,re,sys

def sha(text): return hashlib.sha256(text.encode("utf-8")).hexdigest()

def glob_regex(pattern):
    out="";i=0
    while i<len(pattern):
        c=pattern[i]
        if pattern[i:i+3]=="**/": out+="(?:.*/)?";i+=3;continue
        if pattern[i:i+2]=="**": out+=".*";i+=2;continue
        if c=="*": out+="[^/]*"
        elif c=="?": out+="[^/]"
        else: out+=re.escape(c)
        i+=1
    return re.compile("^"+out+"$")

def reference(spec,files):
    producer={}
    for task in spec["tasks"].values():
        for path in task["outputs"]: producer[path]=task["id"]
    done={}
    def compute(identifier):
        if identifier in done: return done[identifier]
        task=spec["tasks"][identifier]
        selected=sorted(path for path in files if any(glob_regex(pattern).match(path) for pattern in task["reads"]))
        def read(path):
            owner=producer.get(path)
            return compute(owner)[path] if owner else files[path]
        def text(path):
            body=read(path)
            if task.get("normalize"): body="\n".join(line for line in body.split("\n") if not line.startswith("#"))
            return body
        parts=[task["id"]]+[path+"\0"+text(path) for path in selected]+[path+"\0"+read(path) for path in sorted(task["dependencyOutputs"])]
        base=sha("\u0001".join(parts))
        done[identifier]={path:sha(base+"\n"+path)+"\n" for path in task["outputs"]}
        return done[identifier]
    result={}
    for identifier in spec["tasks"]: result.update(compute(identifier))
    return result

def canonical(value):
    if isinstance(value,list): return "["+",".join(canonical(v) for v in value)+"]"
    if isinstance(value,dict): return "{"+",".join(json.dumps(k,ensure_ascii=False)+":"+canonical(value[k]) for k in sorted(value) if value[k] is not None)+"}"
    return json.dumps(value,ensure_ascii=False,separators=(",",":"))

def outputs_digest(files):
    rows=[]
    for f in sorted(files,key=lambda f:f["path"]):
        row={"path":f["path"],"kind":f["kind"],"x":(f["mode"]&0o111)!=0}
        if f.get("sha256") is not None: row["sha"]=f["sha256"]
        if f.get("link") is not None: row["link"]=f["link"]
        rows.append(row)
    return sha(canonical(rows))

def action_key(parts):
    body=dict(parts)
    for name in ("files","environment","dependencies"): body[name]=sorted(body[name])
    body["v"]=1
    return sha(canonical(body))

def concurrency(markers):
    ticks=[]
    for m in markers:
        if m["kind"]=="start": ticks.append((m["at"],1))
        elif m["kind"] in ("end","fail"): ticks.append((m["at"],-1))
    ticks.sort(); current=peak=0
    for _,delta in ticks:
        current+=delta; peak=max(peak,current)
    return peak

def evaluate(expect,observation,edges):
    failures=[]
    markers=observation["markers"]
    executed=sorted({m["task"] for m in markers if m["kind"]=="start"})
    code=observation["exitCode"]
    if expect.get("exit")=="zero" and code!=0: failures.append("exit")
    if expect.get("exit")=="nonzero" and code==0: failures.append("exit")
    if "executed" in expect and executed!=sorted(set(expect["executed"])): failures.append("executed")
    for task in expect.get("executedEach",[]):
        if task not in executed: failures.append("executedEach:"+task)
    for task in expect.get("startsOnce",[]):
        if sum(1 for m in markers if m["kind"]=="start" and m["task"]==task)!=1: failures.append("startsOnce:"+task)
    for check in expect.get("outputs",[]):
        now=observation["after"].get(check["path"]);was=observation["before"].get(check["path"])
        state=check["state"]
        if state=="absent" and now is not None: failures.append("absent:"+check["path"])
        if state=="expected" and now!=observation["expected"].get(check["path"]): failures.append("expected:"+check["path"])
        if state=="unchanged" and (now is None or now!=was): failures.append("unchanged:"+check["path"])
        if state=="changed" and (now is None or now==was): failures.append("changed:"+check["path"])
    if expect.get("ordered"):
        end={m["task"]:m["at"] for m in markers if m["kind"]=="end"}
        for m in markers:
            if m["kind"]=="start":
                for dependency in edges.get(m["task"],[]):
                    if dependency in end and end[dependency]>m["at"]: failures.append("ordered:"+m["task"])
    peak=concurrency(markers)
    if "maxConcurrent" in expect and peak>expect["maxConcurrent"]: failures.append("maxConcurrent")
    if "minConcurrent" in expect and peak<expect["minConcurrent"]: failures.append("minConcurrent")
    return sorted(failures)

def select(inventory,includes,excludes):
    chosen=set()
    for pattern in includes:
        if re.search(r"[?*{\[(]",pattern):
            matcher=glob_regex(pattern)
            chosen.update(path for path in inventory if matcher.match(path))
        elif pattern in inventory: chosen.add(pattern)
    drop=[glob_regex(p) for p in excludes]
    return sorted(path for path in chosen if not any(m.match(path) for m in drop))

if __name__=="__main__":
    request=json.load(sys.stdin)
    out={}
    if "reference" in request: out["reference"]=reference(request["reference"]["spec"],request["reference"]["files"])
    if "canonical" in request: out["canonical"]=[canonical(v) for v in request["canonical"]]
    if "digests" in request: out["digests"]=[outputs_digest(v) for v in request["digests"]]
    if "keys" in request: out["keys"]=[action_key(v) for v in request["keys"]]
    if "concurrency" in request: out["concurrency"]=[concurrency(v) for v in request["concurrency"]]
    if "evaluate" in request: out["evaluate"]=[evaluate(c["expect"],c["observation"],c["edges"]) for c in request["evaluate"]]
    if "select" in request: out["select"]=[select(c["inventory"],c["includes"],c["excludes"]) for c in request["select"]]
    json.dump(out,sys.stdout)
