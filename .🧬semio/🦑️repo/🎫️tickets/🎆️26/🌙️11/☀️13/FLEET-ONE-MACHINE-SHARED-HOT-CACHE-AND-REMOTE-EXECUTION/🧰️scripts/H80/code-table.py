"""H80: one row per origin of the fleet store: what each role of the machine runs against what its disk holds (`sidecar-code.json`), the readiness digest and the newest stage launch digest (`ready.json`, `ready-timings.json`). Usage: code-table.py [--json]"""
import json,os,sys,time
S="/Users/Shared/semio-store/v1/origins"
now=time.time()
def load(path):
    try:return json.load(open(path))
    except Exception:return None
def hm(ms):return time.strftime("%H:%M:%S",time.localtime(ms/1000)) if ms else "-"
rows=[]
for origin in sorted(os.listdir(S)):
    code=load(f"{S}/{origin}/sidecar-code.json");ready=load(f"{S}/{origin}/ready.json");timings=load(f"{S}/{origin}/ready-timings.json")
    runs=(timings or {}).get("runs",[]);last=runs[-1] if runs else None
    roles={}
    if code:
        for role,row in code["roles"].items():roles[role]=f'{row["running"][:6]}{"=" if row["current"] else "!="+row["disk"][:6]}@{row["ageSeconds"]}s'
    rows.append({"origin":origin,"codeAt":hm(code["atMs"]) if code else "-","current":code["current"] if code else None,"roles":roles,"readyState":(ready or {}).get("state"),"readyDigest":((ready or {}).get("digest") or "")[:12],"lastStage":(f'{last["stage"]} {hm(last["startedMs"])[:5]}-{hm(last["endedMs"])[:5]} {last["outcome"]} {(last.get("digest") or "")[:12]}' if last else "-"),"sender":(last or {}).get("sender"),"stopBy":(last or {}).get("stopBy")})
if "--json" in sys.argv:print(json.dumps(rows));sys.exit(0)
print(time.strftime("%H:%M:%S"))
for r in rows:print(r["origin"],"code@",r["codeAt"],"current",r["current"],"ready",r["readyState"],r["readyDigest"],"| last",r["lastStage"],"|",r["roles"])
