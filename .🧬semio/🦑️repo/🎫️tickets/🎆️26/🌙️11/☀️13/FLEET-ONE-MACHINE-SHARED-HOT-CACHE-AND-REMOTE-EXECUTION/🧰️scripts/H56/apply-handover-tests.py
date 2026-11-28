#!/usr/bin/env python3
# H56 (4) tests: idle connection holder, pid+startMs identity, bounded predecessor wait. usage: DRY=1 apply-handover-tests.py <build-root>
import sys,os
R=sys.argv[1]
MEM={}
def rep(path,old,new,count=1):
    p=os.path.join(R,path)
    if p not in MEM: MEM[p]=open(p,encoding="utf-8").read()
    assert MEM[p].count(old)==count,(path,old[:70],MEM[p].count(old))
    MEM[p]=MEM[p].replace(old,new)
def flush():
    if os.environ.get("DRY")=="1": print("dry run: all anchors found"); return
    for p,v in MEM.items():
        open(p+".tmp","w",encoding="utf-8").write(v); os.replace(p+".tmp",p)
T="🏡️resident/🧪️tests/🔀️supersession-drain/"
rep(T+"🔣️.json",'  "stallMilliseconds": 1800000,\n','  "stallMilliseconds": 1800000,\n  "idleConnectionMilliseconds": 300000,\n')
rep(T+"🔣️.json",'{"name": "an open connection without a plan keeps it (a run that uses its cache endpoint)", "supersededSinceMs": 1000000, "nowMs": 9000000, "lastProgressMs": 0, "connections": 1, "inFlight": 0, "due": false},',
'{"name": "an open connection without a plan and with recent traffic keeps it (a run that uses its cache endpoint)", "supersededSinceMs": 1000000, "nowMs": 9000000, "lastProgressMs": 0, "lastActivityMs": 8900000, "connections": 1, "inFlight": 0, "due": false},\n'
'    {"name": "an idle connection holder does not block the hand-over once it was quiet for the idle bound", "supersededSinceMs": 1000000, "nowMs": 9000000, "lastProgressMs": 0, "lastActivityMs": 8700000, "connections": 3, "inFlight": 0, "due": true},\n'
'    {"name": "the quiet time of a connection is measured from the supersession when its last traffic is older", "supersededSinceMs": 8800000, "nowMs": 9000000, "lastProgressMs": 0, "lastActivityMs": 100000, "connections": 3, "inFlight": 0, "due": false},')
rep(T+"🟦️.ts",'const oracle=db.query<{due:number},[number|null,number,number,number,number,number,number]>("SELECT ?1 IS NOT NULL AND ?2-?1>=?6 AND ((?4=0 AND ?5=0) OR (?5>0 AND ?2-MAX(?1,?3)>=?7)) AS due");',
'const oracle=db.query<{due:number},[number|null,number,number,number,number,number,number,number,number]>("SELECT ?1 IS NOT NULL AND ?2-?1>=?6 AND ((?4=0 AND ?5=0) OR (?4>0 AND ?5=0 AND ?2-MAX(?1,?8)>=?9) OR (?5>0 AND ?2-MAX(?1,?3)>=?7)) AS due");')
rep(T+"🟦️.ts",'lastProgressMs:row.lastProgressMs,connections:row.connections,inFlight:row.inFlight,graceMilliseconds:table.graceMilliseconds,stallMilliseconds:table.stallMilliseconds});',
'lastProgressMs:row.lastProgressMs,lastActivityMs:(row as {lastActivityMs?:number}).lastActivityMs??row.nowMs,connections:row.connections,inFlight:row.inFlight,graceMilliseconds:table.graceMilliseconds,stallMilliseconds:table.stallMilliseconds,idleConnectionMilliseconds:table.idleConnectionMilliseconds});')
rep(T+"🟦️.ts",'row.connections,row.inFlight,table.graceMilliseconds,table.stallMilliseconds)!.due));',
'row.connections,row.inFlight,table.graceMilliseconds,table.stallMilliseconds,(row as {lastActivityMs?:number}).lastActivityMs??row.nowMs,table.idleConnectionMilliseconds)!.due));')
G="🏡️resident/🧪️tests/🥇️generation-handover/🟦️.ts"
rep(G,'const harness=(predecessor?:()=>unknown)=>{','const harness=(predecessor?:()=>unknown,bound?:number)=>{')
rep(G,'  ...predecessor===undefined?{}:{predecessor},','  ...predecessor===undefined?{}:{predecessor},...bound===undefined?{}:{predecessorBoundMilliseconds:bound},')
rep(G,'test("a younger generation waits for the older busy one before it starts a worker and says so",async()=>{',
'''test("an older generation whose pid was reused by another process is not alive: the identity is pid and start time",()=>{
 const descriptors=[{name:"host-aaaaaaaaaaaa.json",pid:41,startMs:5000,startedAt:1,busy:2}],seen:[number,number|undefined][]=[];
 const list=()=>descriptors.map(row=>row.name),read=(name:string)=>descriptors.find(row=>row.name===name)!;
 const found=olderBusyServiceV1({directory:"/unused",tag:"bbbbbbbbbbbb",since:10,list,read,alive:(pid,startMs)=>{seen.push([pid,startMs]);return startMs===5000&&pid===41;}});
 expect(found?.pid).toBe(41);expect(seen).toEqual([[41,5000]]);
 expect(olderBusyServiceV1({directory:"/unused",tag:"bbbbbbbbbbbb",since:10,list,read,alive:(_pid,startMs)=>startMs!==5000})).toBeUndefined();
});

test("a younger generation stops waiting for an older busy one after the declared bound and starts its worker (a held old generation never blocks it)",async()=>{
 const older={pid:77,busy:1},{pool,workers,decisions,start,next}=harness(()=>older,150),run=start("bounded");
 await wait(60);expect(workers.length).toBe(0);
 (await next(1))!.emit("message",{type:"done",graph:{ok:3}});
 expect(await run).toEqual({graph:{ok:3}});expect(decisions.some(row=>row.key==="predecessor"&&row.used===77)).toBe(true);await pool.close();
});

test("a younger generation waits for the older busy one before it starts a worker and says so",async()=>{''')
flush()
print("ok",len(MEM),"files")
