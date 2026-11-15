#!/usr/bin/env python3
"""R2b: writes edits.json and baseline.json."""
import json,hashlib,os,sys
HERE=os.path.dirname(os.path.abspath(__file__));P=os.path.dirname(HERE);sys.path.insert(0,P)
import payload
LIVE=next(c for c in ["/Users/Shared/semio","/Users/ueli/Documents/semio"] if os.path.exists(c+"/AGENTS.md"))
L=payload.LIB
SERVICE=L+"/🏡️resident/🏛️service/🌐️host/🟨️.mjs"
TEST=L+"/🏡️resident/🧪️tests/🌐️host/🟦️.ts"
E=[]
def lit(path,old,new,count=1):E.append({"path":path,"kind":"literal","old":old,"new":new,"count":count})
lit(SERVICE,"void runInFlightV1().then(running=>","void (ports.runInFlight??runInFlightV1)().then(running=>")
lit(TEST,'const h=inProcess("hold",{idleShutdownMilliseconds:200});await h.service.start();','const h=inProcess("hold",{idleShutdownMilliseconds:200},0,{runInFlight:async()=>false});await h.service.start();')
lit(TEST,"function inProcess(name:string,extra:Record<string,unknown>={},loadMs=0){","function inProcess(name:string,extra:Record<string,unknown>={},loadMs=0,morePorts:Record<string,unknown>={}){")
lit(TEST,"ports:{planner},policy:{residentCheckMilliseconds:50,...extra}});\n const spawnService","ports:{planner,...morePorts},policy:{residentCheckMilliseconds:50,...extra}});\n const spawnService")
lit(TEST,'describe("host service held by a long run",()=>{','describe("host service held by a long run",()=>{\n test("a run the machine reports in flight keeps an otherwise idle service alive: the probe is asked again and again and the service never retires",async()=>{\n  let asked=0;const h=inProcess("inflight",{idleShutdownMilliseconds:200},0,{runInFlight:async()=>{asked++;return true;}});await h.service.start();\n  const exits:string[]=[];h.service.closed.then(reason=>exits.push(reason));\n  for(let at=0;at<200&&asked<3;at++)await sleep(25);\n  expect(asked).toBeGreaterThanOrEqual(3);expect(exits).toEqual([]);\n  await h.service.stop("test");\n });')
json.dump(E,open(P+"/edits.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
base={p:hashlib.sha256(open(LIVE+"/"+p,"rb").read()).hexdigest() for p in sorted({e["path"] for e in E})}
json.dump(base,open(P+"/baseline.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
print(len(E),"edits")
