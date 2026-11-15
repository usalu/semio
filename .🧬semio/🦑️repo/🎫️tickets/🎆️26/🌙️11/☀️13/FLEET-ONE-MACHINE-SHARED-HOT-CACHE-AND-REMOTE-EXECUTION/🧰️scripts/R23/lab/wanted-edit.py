import sys,json
W=sys.argv[1]
def edit(path,pairs):
    t=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert t.count(old)==1,(t.count(old),old[:90]);t=t.replace(old,new)
    open(path,"w",encoding="utf8").write(t)
edit(W+"/🟦️.ts",[
('import{servesPlatformV1}from"../🌍️platform-class/🟦️.ts";','import{platformClassV1,splitRequestV1}from"../🌍️platform-class/🟦️.ts";'),
('heavy?:readonly string[];createdMs:number;expiresMs:number}>;','heavy?:readonly string[];left?:readonly string[];createdMs:number;expiresMs:number}>;'),
('the live request of a machine of its own platform class (a request of another class goes to `foreign` and is left to a machine of that class) of the same graph state','the live request of a machine of any platform class (a request of another class is served except its bound owners: `cross` is told, the taken request holds the served projects and names the left ones in `left`; a request of another class that holds nothing but bound owners goes to `foreign` and is left to a machine of that class) of the same graph state'),
('foreign?(request:WantedRequestV1):void;live?','foreign?(request:WantedRequestV1):void;cross?(request:WantedRequestV1,split:Readonly<{served:readonly string[];left:readonly string[]}>):void;live?'),
('''  if(!servesPlatformV1(input.platform,request.platform)){input.foreign?.(request);continue;}
  if(input.graphKey!==undefined&&request.graphKey!==await input.graphKey())continue;
  input.live?.(request);
  const key=wantedClaimKeyV1(request);
  if(input.known(key))continue;
  if((await input.client.claim(key,input.owner,policy.claimTtlMilliseconds,policy.claimExpectedMilliseconds)).state==="granted")taken.push(request);''','''  const split=splitRequestV1(request.projects,input.platform,request.platform);
  if(split.served.length===0){input.foreign?.(request);continue;}
  if(input.graphKey!==undefined&&request.graphKey!==await input.graphKey())continue;
  const mine=split.left.length===0?request:{...request,projects:split.served,left:split.left};
  input.live?.(mine);
  const key=wantedClaimKeyV1(request);
  if(input.known(key))continue;
  if((await input.client.claim(key,input.owner,policy.claimTtlMilliseconds,policy.claimExpectedMilliseconds)).state==="granted"){taken.push(mine);if(platformClassV1(input.platform)!==platformClassV1(request.platform))input.cross?.(mine,split);}'''),
('platform:string;server:string;projects:readonly string[];generation:number|null;atMs:number;total?:number;','platform:string;server:string;projects:readonly string[];left?:readonly string[];generation:number|null;atMs:number;total?:number;'),
('projects:[...input.projects].sort(),generation:input.generation,atMs:(input.now??Date.now)(),...input.progress===undefined?{}:input.progress};','projects:[...input.projects].sort(),...input.request.left===undefined||input.request.left.length===0?{}:{left:[...input.request.left].sort()},generation:input.generation,atMs:(input.now??Date.now)(),...input.progress===undefined?{}:input.progress};'),
])
edit(W+"/🛠️server/🟦️.ts",[
('const foreignNow=new Map<string,string>(),toldForeign=new Set<string>(),noteForeign=(request:WantedRequestV1)=>{foreignNow.set(request.requester,request.platform);','const foreignNow=new Map<string,number>(),toldForeign=new Set<string>(),toldCross=new Set<string>(),noteForeign=(request:WantedRequestV1)=>{foreignNow.set(request.requester,request.projects.length);'),
('class:platformClassV1(request.platform),own:platformClassV1(input.platform)}));};','class:platformClassV1(request.platform),own:platformClassV1(input.platform)}));};\n const noteCross=(request:WantedRequestV1,split:Readonly<{served:readonly string[];left:readonly string[]}>)=>{const key=wantedClaimKeyV1(request);if(toldCross.has(key))return;toldCross.add(key);input.say(wantedTextV1("cross",language,{requester:request.requester,platform:request.platform,class:platformClassV1(request.platform),own:platformClassV1(input.platform),served:split.served.length,left:split.left.length,named:split.left.length===0?"":": "+split.left.join(", ")}));};'),
('foreign:noteForeign,live:renewHeld,','foreign:noteForeign,cross:noteCross,live:renewHeld,'),
('{requesters:seen,held:held.size,foreign:foreignNow.size,reading:running===undefined?"none":running.reason}','{requesters:seen,held:held.size,cross:[...held.values()].filter(entry=>platformClassV1(entry.request.platform)!==platformClassV1(input.platform)).reduce((sum,entry)=>sum+entry.request.projects.length,0),left:[...held.values()].reduce((sum,entry)=>sum+(entry.request.left?.length??0),0)+[...foreignNow.values()].reduce((sum,count)=>sum+count,0),reading:running===undefined?"none":running.reason}'),
])
edit(W+"/📖️read/🟦️.ts",[
('let answer=new Set<string>(),fetchedAt','let answer=new Set<string>(),leftHere=new Set<string>(),fetchedAt'),
('if(row!==undefined){answer=new Set(row.projects);','if(row!==undefined){answer=new Set(row.projects);leftHere=new Set(row.left??[]);'),
('take:async name=>{await refresh();return answer.has(name);}','take:async name=>{await refresh();return answer.has(name)||leftHere.has(name);}'),
(' const arrived=rest.length-missing.length;\n if(plan.waveProjects===0)refuse(arrived,missing,endCause);\n const alone=solo(missing);',' const kept=[...missing,...rest.filter(name=>leftHere.has(name)&&!answer.has(name)&&!missing.includes(name))],arrived=rest.length-kept.length;\n if(leftHere.size>0)input.say(wantedTextV1("leftHere",language,{count:leftHere.size,named:[...leftHere].sort().join(", ")}));\n if(plan.waveProjects===0)refuse(arrived,kept,endCause);\n const alone=solo(kept);'),
('return{outcome:"served",plan,arrived:arrived+taken.length,missing,waitedMilliseconds:now()-started,','return{outcome:"served",plan,arrived:arrived+taken.length,missing:kept,waitedMilliseconds:now()-started,'),
])
print("edited")
