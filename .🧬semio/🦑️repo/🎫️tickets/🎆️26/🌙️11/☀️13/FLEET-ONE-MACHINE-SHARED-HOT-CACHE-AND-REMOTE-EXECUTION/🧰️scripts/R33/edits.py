"""R33: the stalled graph read is explained by its process tree and a read that overruns its memory grant under pressure is ended typed. Anchored edits (rel path from the worktree root, old, new, count); already applied edits count as same."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
RX=B+"🌐️remote-execution/"
SC=RX+"🛰️sidecar/"
WG=B+"🪪️graph-state/🙋️wanted/"
TS="🟦️.ts"
E=[]
def add(rel,old,new,count=1):E.append((rel,old,new,count))
J=WG+"🧬️schema/🔣️.json"
add(J,'        "stallDumpRepeatMilliseconds": 300000,\n','        "stallDumpRepeatMilliseconds": 300000,\n        "memorySampleMilliseconds": 30000,\n        "memoryOverrunFactor": 2,\n        "memoryOverrunMilliseconds": 300000,\n')
add(J,'          "stallOutput": "graph-server:   last output of the read: {last}",\n','''          "stallOutput": "graph-server:   last output of the read: {last}",
          "stallTree": "graph-server:   process tree: {processes} processes, {running} running, {idle} idle, {paging} paging; resident {rss} MB, physical footprint {footprint} MB; largest pid {pid} with {largest} MB",
          "stallOverrun": "graph-server:   the read holds {footprint} MB, {times} times its grant of {grant} MB: it pages against the swap and waits for no lease and no queue",
          "memoryOverrun": "the read held {footprint} MB against a grant of {grant} MB with the host under memory pressure and published no wave for {minutes} min: it was paging, not working; it is stopped and the next one starts from the waves published so far",
''')
add(J,'          "stallOutput": "Graph-Server:   letzte Ausgabe des Lesens: {last}",\n','''          "stallOutput": "Graph-Server:   letzte Ausgabe des Lesens: {last}",
          "stallTree": "Graph-Server:   Prozessbaum: {processes} Prozesse, {running} laufen, {idle} ruhen, {paging} lagern aus; resident {rss} MB, physischer Fußabdruck {footprint} MB; größte pid {pid} mit {largest} MB",
          "stallOverrun": "Graph-Server:   das Lesen belegt {footprint} MB, das {times}-Fache seiner Zusage von {grant} MB: es lagert in den Swap aus und wartet auf keine Sperre und keine Warteschlange",
          "memoryOverrun": "das Lesen belegte {footprint} MB bei einer Zusage von {grant} MB, der Speicherdruck der Maschine war hoch und seit {minutes} min wurde keine Welle veröffentlicht: es lagerte aus, statt zu arbeiten; es wird beendet und das nächste beginnt bei den bisher veröffentlichten Wellen",
''')
S=SC+"🕸️graph/🔬️stall/"+TS
add(S,'import{wantedTextV1}from"../../../../🪪️graph-state/🙋️wanted/🟦️.ts";\n','import{wantedTextV1}from"../../../../🪪️graph-state/🙋️wanted/🟦️.ts";\nimport type{ReadTreeSummaryV1}from"./🌳️tree/🟦️.ts";\n')
add(S,'lastOutput:string;language:"en"|"de"}>):readonly string[]{','lastOutput:string;language:"en"|"de";tree?:ReadTreeSummaryV1}>):readonly string[]{')
add(S,' if(!explained&&lines.length===1)',''' const tree=input.tree;
 if(tree!==undefined){
  const footprint=tree.footprintMegabytes,grant=view?.grants.find(row=>row.pid===input.rootPid);
  lines.push(wantedTextV1("stallTree",language,{processes:tree.processes,running:tree.running,idle:tree.idle,paging:tree.paging,rss:tree.rssMegabytes,footprint:footprint??"-",pid:tree.largest?.pid??0,largest:tree.largest?.megabytes??0}));
  if(footprint!==undefined&&grant!==undefined&&footprint>grant.reservedMegabytes){explained=true;lines.push(wantedTextV1("stallOverrun",language,{footprint,grant:Math.round(grant.reservedMegabytes),times:(footprint/grant.reservedMegabytes).toFixed(1)}));}
 }
 if(!explained&&lines.length===1)''')
G=SC+"🕸️graph/"+TS
add(G,'import {awaitGraphReadLeasesV1,describeGraphReadStallV1,graphReadBlockersV1,readGraphLeaseEntriesV1} from "./🔬️stall/🟦️.ts";\n','import {awaitGraphReadLeasesV1,describeGraphReadStallV1,graphReadBlockersV1,readGraphLeaseEntriesV1} from "./🔬️stall/🟦️.ts";\nimport {measureReadTreeV1,memoryOverrunV1} from "./🔬️stall/🌳️tree/🟦️.ts";\n')
add(G,'diagnose?:(input:Readonly<{pid:number;silentMilliseconds:number;lastOutput:string}>)=>Promise<readonly string[]>}>):Promise<WantedInferV1>{','diagnose?:(input:Readonly<{pid:number;silentMilliseconds:number;lastOutput:string}>)=>Promise<readonly string[]>;measure?:(pid:number)=>Promise<Readonly<{footprintMegabytes:number|undefined;grantMegabytes:number|undefined;pressure:string}>|undefined>}>):Promise<WantedInferV1>{')
add(G,' let ended:"stalled"|"too-long"|undefined,tail="";\n let dumpedAt=-Infinity;\n',' let ended:"stalled"|"too-long"|"memory-overrun"|undefined,tail="",overrun:{footprintMegabytes:number|undefined;grantMegabytes:number|undefined}|undefined,sampling=false;\n let dumpedAt=-Infinity;\n')
add(G,'  if(input.diagnose!==undefined&&ended===undefined&&watch.silentMilliseconds()>=policy.stallDumpMilliseconds','''  if(input.measure!==undefined&&ended===undefined&&!sampling&&watch.silentMilliseconds()>=policy.memoryOverrunMilliseconds){
   sampling=true;void input.measure(child.pid).then(sample=>{if(sample!==undefined&&ended===undefined&&memoryOverrunV1({...sample,silentMilliseconds:watch.silentMilliseconds(),policy})){overrun=sample;ended="memory-overrun";stop();}}).catch(()=>undefined).finally(()=>{setTimeout(()=>{sampling=false;},policy.memorySampleMilliseconds).unref();});
  }
  if(input.diagnose!==undefined&&ended===undefined&&watch.silentMilliseconds()>=policy.stallDumpMilliseconds''')
add(G,'detail:ended!==undefined?graphReadEndTextV1(ended,input.language,policy):','detail:ended==="memory-overrun"?wantedTextV1("memoryOverrun",input.language,{footprint:overrun?.footprintMegabytes??0,grant:Math.round(overrun?.grantMegabytes??0),minutes:(watch.silentMilliseconds()/60000).toFixed(1)}):ended!==undefined?graphReadEndTextV1(ended,input.language,policy):')
add(G,'lastOutput:input.lastOutput,language})});','lastOutput:input.lastOutput,language,tree:await measureReadTreeV1(input.pid).catch(()=>undefined)}),measure:async pid=>{const tree=await measureReadTreeV1(pid),view=await readRemoteExecutionHostViewV1(environment).catch(()=>undefined);return{footprintMegabytes:tree.footprintMegabytes??tree.rssMegabytes,grantMegabytes:view?.grants.find(row=>row.pid===pid)?.reservedMegabytes,pressure:view?.host.pressure??"normal"};}});')
