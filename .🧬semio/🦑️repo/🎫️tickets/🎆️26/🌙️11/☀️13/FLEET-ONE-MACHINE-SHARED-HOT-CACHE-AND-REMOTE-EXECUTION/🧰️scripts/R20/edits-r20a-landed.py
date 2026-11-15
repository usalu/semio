"""R20: the serving claim stays alive, the serving read never waits silently. Anchored edits (rel path from the worktree root, old, new, count)."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
RX=B+"🌐️remote-execution/"
SC=RX+"🛰️sidecar/"
WG=B+"🪪️graph-state/🙋️wanted/"
TS="🟦️.ts"
E=[]
def edit(rel,old,new,count=1):E.append((rel,old,new,count))

SCHEMA=WG+"🧬️schema/🔣️.json"
edit(SCHEMA,'''        "availableFloorMegabytes": 2048
      }''','''        "availableFloorMegabytes": 2048,
        "claimTtlMilliseconds": 180000,
        "claimRenewMilliseconds": 45000,
        "claimExpectedMilliseconds": 450000
      }''')
edit(SCHEMA,'''        "serveTotalMegabytes": 15000,
        "costsMilliseconds": 60000''','''        "serveTotalMegabytes": 24000,
        "costsMilliseconds": 60000,
        "stallDumpMilliseconds": 120000,
        "stallDumpRepeatMilliseconds": 300000,
        "leasePollMilliseconds": 5000,
        "leaseNoticeMilliseconds": 60000,
        "leaseWaitMilliseconds": 1200000,
        "leaseResourcePrefixes": [
          "native-fixed-image:"
        ]''')
edit(SCHEMA,'''"lost": "graph-server: the claim for {requester} is held by another machine; this machine leaves the request to it",''','''"lost": "graph-server: the claim for {requester} is held by {by}; this machine leaves the request to it",''')
edit(SCHEMA,'''"lost": "Graph-Server: den Anspruch für {requester} hält eine andere Maschine; diese Maschine überlässt ihr die Anfrage",''','''"lost": "Graph-Server: den Anspruch für {requester} hält {by}; diese Maschine überlässt ihm die Anfrage",''')
edit(SCHEMA,'''          "paused": "graph: paused after generation {generation}: {waiters} command(s) wait for memory on this machine ({seconds} s)"''','''          "paused": "graph: paused after generation {generation}: {waiters} command(s) wait for memory on this machine ({seconds} s)",
          "capped": "graph-server: the claim for {requester} reached the lifetime of the cache server; it is taken again in a moment",
          "reclaimed": "graph-server: the claim for {requester} had lapsed; this machine holds it again",
          "leaseWait": "graph-server: the read waits to start: {resource} is held exclusively by pid {holder} ({command}, {phase}) for {held} s, last progress {progress} s ago; waited {seconds} s so far",
          "leaseWaitFree": "graph-server: {resource} is free; the read starts after waiting {seconds} s",
          "leaseWaitLimit": "graph-server: waited {seconds} s for {resource}; the read starts now and queues in the highest class",
          "stallHead": "graph-server: the read (pid {pid}) made no progress for {seconds} s; what it waits for:",
          "stallLease": "graph-server:   the lease {resource} ({mode}) for {seconds} s, behind pid {holder} ({command}, {phase}), held for {held} s, last progress {progress} s ago",
          "stallLeaseFree": "graph-server:   the lease {resource} ({mode}) for {seconds} s; no live holder is registered",
          "stallGrant": "graph-server:   capacity gate: the read holds the grant {grant} of {reserved} MB; {grants} grants, {waiters} command(s) wait, pressure {pressure}, {free} of {total} MB free",
          "stallQueued": "graph-server:   capacity gate: the read waits in the queue as {id} (class {class}) behind {ahead} command(s); pressure {pressure}, {free} of {total} MB free",
          "stallNothing": "graph-server:   no lease and no capacity wait is registered for the read",
          "stallOutput": "graph-server:   last output of the read: {last}"''')
edit(SCHEMA,'''          "paused": "Graph: pausiert nach Generation {generation}: {waiters} Befehl(e) warten auf Speicher auf diesem Rechner ({seconds} s)"''','''          "paused": "Graph: pausiert nach Generation {generation}: {waiters} Befehl(e) warten auf Speicher auf diesem Rechner ({seconds} s)",
          "capped": "Graph-Server: der Anspruch für {requester} erreichte die Lebensdauer des Cache-Servers; er wird gleich neu genommen",
          "reclaimed": "Graph-Server: der Anspruch für {requester} war abgelaufen; diese Maschine hält ihn wieder",
          "leaseWait": "Graph-Server: das Lesen wartet auf den Start: {resource} hält pid {holder} ({command}, {phase}) seit {held} s exklusiv, letzter Fortschritt vor {progress} s; bisher {seconds} s gewartet",
          "leaseWaitFree": "Graph-Server: {resource} ist frei; das Lesen beginnt nach {seconds} s Warten",
          "leaseWaitLimit": "Graph-Server: {seconds} s auf {resource} gewartet; das Lesen beginnt jetzt und reiht sich in der höchsten Klasse ein",
          "stallHead": "Graph-Server: das Lesen (pid {pid}) kam {seconds} s lang nicht voran; worauf es wartet:",
          "stallLease": "Graph-Server:   die Sperre {resource} ({mode}) seit {seconds} s, hinter pid {holder} ({command}, {phase}), gehalten seit {held} s, letzter Fortschritt vor {progress} s",
          "stallLeaseFree": "Graph-Server:   die Sperre {resource} ({mode}) seit {seconds} s; kein lebender Halter ist eingetragen",
          "stallGrant": "Graph-Server:   Kapazitätstor: das Lesen hält die Zuteilung {grant} mit {reserved} MB; {grants} Zuteilungen, {waiters} Befehl(e) warten, Druck {pressure}, {free} von {total} MB frei",
          "stallQueued": "Graph-Server:   Kapazitätstor: das Lesen wartet in der Schlange als {id} (Klasse {class}) hinter {ahead} Befehl(en); Druck {pressure}, {free} von {total} MB frei",
          "stallNothing": "Graph-Server:   für das Lesen ist weder eine Sperre noch ein Kapazitätswarten eingetragen",
          "stallOutput": "Graph-Server:   letzte Ausgabe des Lesens: {last}"''')

edit(WG+TS,'''if((await input.client.claim(key,input.owner,policy.ttlMilliseconds)).state==="granted")taken.push(request);''','''if((await input.client.claim(key,input.owner,policy.claimTtlMilliseconds,policy.claimExpectedMilliseconds)).state==="granted")taken.push(request);''')

SERVER=WG+"🛠️server/"+TS
edit(SERVER,'''    if(now()-entry.claimedAt<policy.renewMilliseconds)continue;
    entry.claimedAt=now();
    if(await input.client.renew(wantedClaimKeyV1(entry.request),input.owner,policy.ttlMilliseconds).catch(()=>"fault")==="lost"){held.delete(requester);input.say(wantedTextV1("lost",language,{requester}));}''','''    if(now()-entry.claimedAt<policy.claimRenewMilliseconds)continue;
    entry.claimedAt=now();
    const claimKey=wantedClaimKeyV1(entry.request);
    if(await input.client.renew(claimKey,input.owner,policy.claimTtlMilliseconds).catch(()=>"fault")==="lost"){
     const again=await input.client.claim(claimKey,input.owner,policy.claimTtlMilliseconds,policy.claimExpectedMilliseconds).catch(()=>undefined);
     if(again?.state==="granted"){entry.claimedAt=now();input.say(wantedTextV1("reclaimed",language,{requester}));}
     else if(again?.state==="held"&&again.by==="lifetime-cap")input.say(wantedTextV1("capped",language,{requester}));
     else if(again?.state==="held"){held.delete(requester);input.say(wantedTextV1("lost",language,{requester,by:again.by}));}
    }''')

G=SC+"🕸️graph/"+TS
edit(G,'''import {sleep,type RemoteExecutionSidecarContextV1} from "../🧰️shared/🟦️.ts";''','''import {sleep,type RemoteExecutionSidecarContextV1} from "../🧰️shared/🟦️.ts";
import {awaitGraphReadLeasesV1,describeGraphReadStallV1,graphReadBlockersV1,readGraphLeaseEntriesV1} from "./🔬️stall/🟦️.ts";''')
edit(G,'''say?:(line:string)=>void;onWave?:(row:GraphWaveRowV1)=>void}>):Promise<WantedInferV1>{''','''say?:(line:string)=>void;onWave?:(row:GraphWaveRowV1)=>void;diagnose?:(input:Readonly<{pid:number;silentMilliseconds:number;lastOutput:string}>)=>Promise<readonly string[]>}>):Promise<WantedInferV1>{''')
edit(G,'''SEMIO_GRAPH_FLEET_READ:"off",SEMIO_GRAPH_BACKGROUND:"1",''','''SEMIO_GRAPH_FLEET_READ:"off",SEMIO_GRAPH_BACKGROUND:"1",SEMIO_LEASE_PRIORITY:input.reason==="request"?"interactive":"background",''')
edit(G,'''const timer=setInterval(()=>{const verdict=watch.verdict();if(verdict!=="go"&&ended===undefined){ended=verdict;stop();}},policy.watchMilliseconds);''','''let dumpedAt=-Infinity;
 const timer=setInterval(()=>{
  const verdict=watch.verdict();if(verdict!=="go"&&ended===undefined){ended=verdict;stop();}
  if(input.diagnose!==undefined&&ended===undefined&&watch.silentMilliseconds()>=policy.stallDumpMilliseconds&&now()-dumpedAt>=policy.stallDumpRepeatMilliseconds){
   dumpedAt=now();void input.diagnose({pid:child.pid,silentMilliseconds:watch.silentMilliseconds(),lastOutput:tail}).then(lines=>{for(const line of lines)input.say?.(line);}).catch(()=>undefined);
  }
 },policy.watchMilliseconds);''')
edit(G,'''  return inferGraphReadV1({command:graphServerCommandV1(environment),cwd:context.root,environment,reason,signal,language,steerDirectory,costPath,say:context.write,onWave:row=>uploader?.wave(row)});''','''  await awaitGraphReadLeasesV1({blockers:()=>graphReadBlockersV1({entries:readGraphLeaseEntriesV1(environment),prefixes:policy.leaseResourcePrefixes,ownPid:process.pid,nowMs:Date.now()}),language,say:context.write,signal});
  if(signal?.aborted)return{ok:false,status:1,seconds:0,generation:null,detail:"stopped"};
  return inferGraphReadV1({command:graphServerCommandV1(environment),cwd:context.root,environment,reason,signal,language,steerDirectory,costPath,say:context.write,onWave:row=>uploader?.wave(row),diagnose:async input=>describeGraphReadStallV1({rootPid:input.pid,entries:readGraphLeaseEntriesV1(environment),view:await readRemoteExecutionHostViewV1(environment).catch(()=>undefined),nowMs:Date.now(),silentMilliseconds:input.silentMilliseconds,lastOutput:input.lastOutput,language})});''')

GT=SC+"🧪️tests/🕸️graph/"+TS
edit(GT,'''test("the fleet's 16 GB machines serve the graph: the serving memory lies below their physical memory and a requested read needs less than a refresh",()=>{
 expect(wantedServerPolicyV1.serveTotalMegabytes).toBeLessThan(16384);
 expect(wantedServerPolicyV1.requestHeadroomMegabytes).toBeLessThan(wantedServerPolicyV1.refreshHeadroomMegabytes);
 expect(graphServerHeadroomV1({totalMegabytes:16384,availableMegabytes:7000,pressure:"normal"},wantedServerPolicyV1.serveTotalMegabytes)).toEqual({freeMegabytes:7000,pressure:"normal"});
 expect(graphServerHeadroomV1({totalMegabytes:8192,availableMegabytes:7000,pressure:"normal"},wantedServerPolicyV1.serveTotalMegabytes)).toEqual({freeMegabytes:0,pressure:"normal"});
});''','''test("the fleet's 16 GB workers never serve the graph: the serving memory lies above their physical memory and a requested read needs less than a refresh",()=>{
 expect(wantedServerPolicyV1.serveTotalMegabytes).toBeGreaterThan(16384);
 expect(wantedServerPolicyV1.requestHeadroomMegabytes).toBeLessThan(wantedServerPolicyV1.refreshHeadroomMegabytes);
 expect(graphServerHeadroomV1({totalMegabytes:16384,availableMegabytes:7000,pressure:"normal"},wantedServerPolicyV1.serveTotalMegabytes)).toEqual({freeMegabytes:0,pressure:"normal"});
 expect(graphServerHeadroomV1({totalMegabytes:32768,availableMegabytes:7000,pressure:"normal"},wantedServerPolicyV1.serveTotalMegabytes)).toEqual({freeMegabytes:7000,pressure:"normal"});
});''')

edit(WG+"🧪️tests/🔁️resume/"+TS,'''expect(lines[1],"a claim that nobody holds any more is given up, not kept").toBe(wantedTextV1("lost","en",{requester:"g1-m06"}));''','''expect(lines[1],"a claim that nobody holds any more is taken again at once, the request is still wanted").toBe(wantedTextV1("reclaimed","en",{requester:"g1-m06"}));''')
edit(WG+"🧪️tests/🔁️resume/"+TS,'''wantedTextV1("lost","en",{requester:"g1-m06"})),"the claim was renewed, not lost"''','''wantedTextV1("lost","en",{requester:"g1-m06",by:"g1-m09"})),"the claim was renewed, not lost"''')

edit(B+"🕸️graph/🕰️generations/🌐️remote-exchange/🧬️schema/🔣️.json",'''"servingTotalMegabytes": 15000,''','''"servingTotalMegabytes": 24000,''')

STALL=SC+"🕸️graph/🔬️stall/"+TS
edit(STALL,'''  if(blocker===undefined)return waited>0?(input.say(wantedTextV1("leaseWaitFree",input.language,{resource:last?.resource??"",seconds:Math.round(waited/1000)})),waited):0;''','''  if(blocker===undefined){
   if(last===undefined)return 0;
   input.say(wantedTextV1("leaseWaitFree",input.language,{resource:last.resource,seconds:Math.round(waited/1000)}));return waited;
  }''')
edit(SC+"🧪️tests/🕸️graph/🔬️stall/"+TS,'''const fakeRepo=(script:string)=>''','''test("a start that finds no blocker says nothing, however long looking took",async()=>{
 const lines:string[]=[];let time=0;
 expect(await awaitGraphReadLeasesV1({blockers:()=>{time+=2000;return[];},language:"en",say:line=>lines.push(line),signal:undefined,now:()=>time,sleep:async()=>{throw new Error("must not sleep");}})).toBe(0);
 expect(lines).toEqual([]);
});

const fakeRepo=(script:string)=>''')
