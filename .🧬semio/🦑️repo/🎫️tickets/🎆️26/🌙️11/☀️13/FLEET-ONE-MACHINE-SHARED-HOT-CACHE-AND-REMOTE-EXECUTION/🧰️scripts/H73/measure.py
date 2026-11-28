#!/usr/bin/env python3
"""H73 measurement: one fresh-lane cargo command per phase, wall time and the per-lane unit tally (python3 + `repo` only).

  measure.py --crate PKG --phases warm,cold --out DIR [--cas EMPTY_DIR] [--command check]

warm: nothing forgotten (records the crate names of the lane in DIR/crates.json). cold: every recorded crate forgotten first (the
operator command `forget`, no file deleted by hand). Each phase prints its row to stdout and DIR/<phase>.json. Needs the H16 probe."""
import argparse,json,os,subprocess,sys,time
H16=os.path.join(os.path.dirname(os.path.abspath(__file__)),"..","H16");sys.path.insert(0,H16)
import probe

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
    ap=argparse.ArgumentParser();ap.add_argument("--crate",required=True);ap.add_argument("--phases",default="warm,cold");ap.add_argument("--out",required=True);ap.add_argument("--cas");ap.add_argument("--command",default="check");ap.add_argument("--tag",default="")
    a=ap.parse_args();os.makedirs(a.out,exist_ok=True);primary=a.crate.replace("-","_");names=os.path.join(a.out,"crates.json")
    extra={"SEMIO_CAS_ROOT":a.cas} if a.cas else {}
    for phase in a.phases.split(","):
        gate()
        if phase=="cold":
            crates=set(json.load(open(names)));print("forgot",probe.forget(crates),"entries of",len(crates),"crates",flush=True)
        argv=["nice","-n","10","repo","cargo","--store=shared","--configuration=prototype","--fresh-lane","--",a.command,"-p",a.crate]
        t0=int(time.time()*1000);w=time.time()
        env=dict(os.environ,SEMIO_LANGUAGE="en",**extra)
        p=subprocess.run(argv,capture_output=True,text=True,cwd=probe.ROOT,env=env)
        wall=time.time()-w;t1=int(time.time()*1000)
        lane=probe.lane_for(primary,t0,t1,"none")
        rows=probe.events(lane,t0,t1) if lane else []
        row={"phase":phase+a.tag,"exit":p.returncode,"wallSeconds":round(wall,1),"lane":lane,**probe.tally(rows)}
        row["lines"]=[l for l in (p.stdout+p.stderr).splitlines() if "[repo cargo]" in l and ("seed" in l.lower() or "units:" in l or "vorbef" in l or "Units:" in l)]
        row["tail"]=(p.stdout+p.stderr).strip().splitlines()[-4:]
        if phase=="warm":json.dump(sorted({r["crate"] for r in rows}),open(names,"w"))
        json.dump(row,open(os.path.join(a.out,phase+a.tag+".json"),"w"),indent=1,ensure_ascii=False);print(json.dumps({k:v for k,v in row.items() if k not in("crates",)},ensure_ascii=False),flush=True)
main()
