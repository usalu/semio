#!/usr/bin/env python3
"""Measured remote hit with byte identity (wave 1127, lane H40). python3 + `repo` only.

  remote-hit.py --crate semio-framework-pets-clearance --out DIR [--wait-minutes 25]

Phase publish: `repo cargo ... --fresh-lane -- rustc -p <crate> --lib -- --cfg <nonce>` (a key the fleet has never seen): the closure
units hit, the primary unit compiles (or offloads) and is published to the fleet. The probe waits until the uploader reports the
key stored. Then the local unit entries of the closure are forgotten (operator command, no file deleted by hand) and phase hit
runs the identical command in another fresh lane: the primary unit must be a `remote-hit`. Byte identity: the sha-256 of every
output of the unit entry written by the compile (publish) against the sha-256 of the entry the remote hit restored.
"""
import argparse,glob,json,os,subprocess,sys,time
sys.path.insert(0,os.path.join(os.path.dirname(os.path.abspath(__file__)),"..","H16"))
import probe

def entry_outputs(key):
    best=None
    for path in glob.glob(os.path.join(probe.UNIT,"e",key[:2],key,"*.json")):
        if best is None or os.path.basename(path)>os.path.basename(best):best=path
    if best is None:return None
    return {o["name"].replace("@@semio-extra@@",""):o["sha256"] for o in json.load(open(best))["outputs"] if o.get("template") is not True}

def primary_event(rows,primary):
    found=[r for r in rows if r["crate"]==primary and r["kind"] in("miss","remote-hit","hit")]
    return found[-1] if found else None

def uploaded(crate,since):
    path=os.path.join(probe.EVENTS,"uploader.jsonl")
    try:fh=open(path,"rb")
    except OSError:return None
    with fh:
        fh.seek(max(0,os.path.getsize(path)-64*1024*1024))
        for line in fh:
            if crate.encode() not in line:continue
            try:r=json.loads(line)
            except ValueError:continue
            if r.get("crate")==crate and r["atMs"]>=since and r["kind"]=="remote-publish" and r.get("reason")=="uploaded" and r.get("waitedMs",9e9)<600000:return r
    return None

def gate():
    for _ in range(10):
        free=subprocess.run(["memory_pressure"],capture_output=True,text=True).stdout
        swap=subprocess.run(["sysctl","-n","vm.swapusage"],capture_output=True,text=True).stdout
        try:
            pct=int(free.split("free percentage:")[1].split("%")[0]);gib=float(swap.split("free =")[1].split("M")[0])/1024
            if pct>=20 and gib>=1:return
        except Exception:return
        time.sleep(60)

def main():
    ap=argparse.ArgumentParser();ap.add_argument("--crate",required=True);ap.add_argument("--out",required=True);ap.add_argument("--wait-minutes",type=int,default=25)
    args=ap.parse_args();crate=args.crate;primary=crate.replace("-","_");nonce="h40_%d"%int(time.time());crates=probe.closure(crate)
    argv=["nice","-n","10","repo","cargo","--store=shared","--configuration=prototype","--fresh-lane","--","rustc","-p",crate,"--lib","--","--cfg",nonce]
    tag=time.strftime("%H%M%S");result={"crate":crate,"nonce":nonce,"command":" ".join(argv[3:]),"phases":[]}
    try:
        for name in("publish","hit"):
            if name=="hit":
                since=result["publishedAtMs"] if "publishedAtMs" in result else 0
                result["localEntriesForgotten"]=probe.forget(crates)
            gate()
            row,rows=probe.run(name,argv,{},crates,"H40-%s-%s"%(name,tag),primary)
            event=primary_event(rows,primary);row["primaryEvent"]=event
            if event:row["outputs"]=entry_outputs(event["key"])
            result["phases"].append(row)
            print("%-8s exit=%d wall=%6.1fs units=%d local=%d fleet=%d compile=%d offload=%d primary=%s"%(name,row["exit"],row["wallSeconds"],row["unitsTotal"],row["localHit"],row["fleetHit"],row["localCompile"],row["offloaded"],event and event["kind"]),flush=True)
            if name=="publish":
                if not event or event["kind"]!="miss":break
                deadline=time.time()+args.wait_minutes*60;t0=event["atMs"]-60000
                while time.time()<deadline:
                    up=uploaded(primary,t0)
                    if up:result["publishedAtMs"]=up["atMs"];result["uploadReason"]=up.get("reason");break
                    time.sleep(10)
                else:result["uploadTimedOut"]=True;break
    finally:
        for s in probe.OPEN:probe.close_session(s)
    ph=result["phases"]
    if len(ph)==2 and ph[0].get("outputs") and ph[1].get("outputs"):
        a,b=ph[0]["outputs"],ph[1]["outputs"];result["identity"]={"sameNames":sorted(a)==sorted(b),"identical":a==b,"outputs":a}
    os.makedirs(args.out,exist_ok=True)
    json.dump(result,open(os.path.join(args.out,"remote-hit-%s.json"%crate),"w"),indent=1)
    print(json.dumps({k:result.get(k) for k in("identity","uploadReason","uploadTimedOut","localEntriesForgotten")},indent=1))

if __name__=="__main__":main()
