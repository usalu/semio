#!/usr/bin/env python3
"""Fleet-as-one-machine probe (wave 1127, lane H16).

Every phase runs in a fresh agent session (= fresh cargo lane = cold cargo target), so cargo dispatches every unit of the
closure through the unit-cache wrapper and the per-lane event log tells exactly what happened to each unit.

  probe.py rust --crate <package> [--out DIR]     phases: local, fleet, warm, fresh-cold, fresh-warm, fresh-fleet, down
  probe.py ts   --project <nx project:target> [--out DIR]
Needs only python3 (3.9+) and the `repo` binary on PATH; run it from any directory.
"""
import argparse,glob,hashlib,json,os,re,socket,subprocess,sys,time

ROOT=os.environ.get("SEMIO_PROBE_ROOT") or subprocess.run(["git","rev-parse","--show-toplevel"],capture_output=True,text=True,cwd=os.path.dirname(os.path.abspath(__file__))).stdout.strip()
UNIT=os.path.join(ROOT,".cache/b/r/m/u")
EVENTS=os.path.join(UNIT,"x")
HERE=os.path.dirname(os.path.abspath(__file__))

def closed_port():
    s=socket.socket();s.bind(("127.0.0.1",0));p=s.getsockname()[1];s.close();return p

def open_session(name):
    out=subprocess.run(["repo","session","open","--agent="+name,"--owner-pid="+str(os.getpid()),"--export"],capture_output=True,text=True,cwd=ROOT).stdout
    env=re.search(r"^SEMIO_AGENT_SESSION=(\S+)",out,re.M);lane=re.search(r"cargo lane (\S+?),",out)
    sid=re.search(r"Agent session (\S+) ",out)
    if not(env and lane and sid):raise SystemExit("session open failed: "+out)
    return {"env":env.group(1),"lane":lane.group(1),"id":sid.group(1)}

def lane_of(agent):
    for path in glob.glob(os.path.join(UNIT,"l","*","lane.json")):
        try:
            if json.load(open(path)).get("agent")==agent:return os.path.basename(os.path.dirname(path))
        except Exception:pass
    return None

def close_session(s):
    subprocess.run(["repo","session","close",s["id"],"--force"],capture_output=True,text=True,cwd=ROOT)

def events(lane,t0,t1):
    rows=[]
    for path in (os.path.join(EVENTS,lane+".jsonl"),os.path.join(EVENTS,"offload.jsonl")):
        try:fh=open(path,"rb")
        except OSError:continue
        with fh:
            fh.seek(max(0,os.path.getsize(path)-64*1024*1024))
            for line in fh:
                if lane.encode() not in line:continue
                try:r=json.loads(line)
                except ValueError:continue
                if r.get("lane")==lane and t0<=r["atMs"]<=t1+2000:rows.append(r)
    return rows

def tally(rows):
    units={};scripts={"hit":0,"miss":0,"publish":0}
    offloaded={r["key"]:r for r in rows if r["kind"]=="offload-run"}
    for r in rows:
        k=r["kind"]
        if k in("hit","remote-hit","miss"):units[(r["crate"],r["key"])]=k
        if k.startswith("script-"):scripts[k[7:]]=scripts.get(k[7:],0)+1
    count=lambda kind:sum(1 for v in units.values() if v==kind)
    off_units=[r for r in rows if r["kind"]=="offload-run" and not r["crate"].endswith("build script")]
    origins={}
    for r in off_units:origins[r["origin"]]=origins.get(r["origin"],0)+1
    miss_keys={k for (c,k),v in units.items() if v=="miss"}
    return {"unitsTotal":len(units),"localHit":count("hit"),"fleetHit":count("remote-hit"),"localCompile":len(miss_keys-set(offloaded)),
            "offloaded":len(off_units),"offloadOrigins":origins,"offloadedAndMissed":len(miss_keys&set(offloaded)),"published":sum(1 for r in rows if r["kind"]=="publish"),
            "buildScripts":scripts,"remoteSkips":sorted({(r.get("reason") or "").split(":")[0][:60] for r in rows if r["kind"]=="remote-skip"}),
            "crates":sorted({c for (c,k) in units})}

def artifacts(lane,crates):
    out={}
    for path in glob.glob(os.path.join(UNIT,"l",lane,"t*","*","deps","lib*.r*"))+glob.glob(os.path.join(UNIT,"l",lane,"t*","*","*","deps","lib*.r*")):
        stem=re.sub(r"-[0-9a-f]{16}(\.r(lib|meta))$",r"\1",os.path.basename(path))
        if not stem.startswith("lib") or not stem.endswith((".rlib",".rmeta")):continue
        name=stem[3:].rsplit(".",1)[0].replace("-","_")
        if crates and name not in crates:continue
        out[stem]=hashlib.sha256(open(path,"rb").read()).hexdigest()
    return out

def run(label,argv,extra_env,crates,session_name):
    s=open_session(session_name)
    env=dict(os.environ,SEMIO_AGENT_SESSION=s["env"],SEMIO_LANGUAGE="en",**extra_env)
    t0=int(time.time()*1000);w=time.time()
    p=subprocess.run(argv,capture_output=True,text=True,cwd=ROOT,env=env)
    wall=time.time()-w;t1=int(time.time()*1000)
    lane=lane_of(session_name) or s["lane"]
    rows=events(lane,t0,t1)
    row={"phase":label,"exit":p.returncode,"wallSeconds":round(wall,1),"lane":lane,**tally(rows),"artifacts":artifacts(lane,crates)}
    row["tail"]=(p.stdout+p.stderr).strip().splitlines()[-6:]
    close_session(s)
    return row,rows

def forget(crates):
    wrapper=os.path.join(UNIT,"bin","rustc-wrapper");n=0
    for c in sorted(crates):
        for spelling in {c,c.replace("_","-")}:
            r=subprocess.run([wrapper,"--semio-unit-cache","forget","--crate",spelling,"--root",UNIT],capture_output=True,text=True,cwd=ROOT)
            try:n+=json.loads(r.stdout.strip().splitlines()[-1])["entries"]
            except Exception:pass
    return n

def rust(args):
    crate=args.crate;nonce="h16_%d"%int(time.time())
    base=["repo","cargo","--store=shared","--configuration=prototype","--","build","-p",crate,"--lib"]
    fresh=["repo","cargo","--store=shared","--configuration=prototype","--","rustc","-p",crate,"--lib","--","--cfg",nonce]
    down={"SEMIO_REMOTE_CACHE_ENDPOINT":"http://127.0.0.1:%d"%closed_port(),"SEMIO_REMOTE_EXECUTION_ENDPOINT":"http://127.0.0.1:%d"%closed_port()}
    rows=[];tag=time.strftime("%H%M%S")
    first,_=run("1 local (cold target, every unit local entry present)",base,{},None,"H16-local-"+tag);rows.append(first)
    crates=set(first["crates"])
    forgotten=forget(crates)
    for label,argv,env,name,pre in [
      ("2 fleet (cold target, local entries forgotten)",base,{},"fleet",True),
      ("3 warm (cold target, local entries present)",base,{},"warm",False),
      ("4 fresh-key cold (primary crate key never seen: compile or offload)",fresh,{},"fresh-cold",False),
      ("5 fresh-key warm (cold target, local entry present)",fresh,{},"fresh-warm",False),
      ("6 fresh-key fleet (local entries forgotten, fleet copy of own bytes)",fresh,{},"fresh-fleet",True),
      ("7 remote down (local entries forgotten, cache+execution endpoints closed)",base,down,"down",True)]:
        if pre:forgotten+=forget(crates)
        r,_=run(label,argv,env,crates,"H16-%s-%s"%(name,tag));rows.append(r)
    return {"crate":crate,"nonce":nonce,"closureCrates":sorted(crates),"localEntriesForgotten":forgotten,"phases":rows}

def main():
    ap=argparse.ArgumentParser();sub=ap.add_subparsers(dest="cmd",required=True)
    a=sub.add_parser("rust");a.add_argument("--crate",required=True);a.add_argument("--out",required=True)
    t=sub.add_parser("ts");t.add_argument("--project",required=True);t.add_argument("--out",required=True)
    args=ap.parse_args()
    result=rust(args) if args.cmd=="rust" else None
    if result is None:raise SystemExit("ts: see ts-probe.py")
    os.makedirs(args.out,exist_ok=True)
    json.dump(result,open(os.path.join(args.out,"rust-%s.json"%args.crate),"w"),indent=1)
    for r in result["phases"]:
        print("%-72s exit=%d wall=%6.1fs units=%d local=%d fleet=%d compile=%d offload=%d %s"%(r["phase"][:72],r["exit"],r["wallSeconds"],r["unitsTotal"],r["localHit"],r["fleetHit"],r["localCompile"],r["offloaded"],r["offloadOrigins"]))

if __name__=="__main__":main()
