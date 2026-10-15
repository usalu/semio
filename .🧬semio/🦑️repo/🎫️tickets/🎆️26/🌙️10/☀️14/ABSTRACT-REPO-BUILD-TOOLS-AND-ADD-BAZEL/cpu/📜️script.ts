import { fileURLToPath } from "node:url";
import { Database } from "bun:sqlite";
import { lstat, readFile, open } from "node:fs/promises";
import { constants } from "node:fs";
import { resolve, relative, sep, isAbsolute, dirname, join } from "node:path";

async function wholeChild() {
  const [profilePath, outputPath] = process.argv.slice(3), root=process.env.SEMIO_WORKSPACE_ROOT??process.cwd(),ticket=resolve(import.meta.dir,".."),generated=join(ticket,"🗑️generated"),limits={maximumMilliseconds:60000,maximumBytes:process.argv.includes("--sdk")?134217728:268435456,maximumNodes:500000,maximumSamples:2000000,maximumOracleRows:10000000,maximumOutputBytes:67108864};
  if(!profilePath||!outputPath||resolve(profilePath)===resolve(outputPath))throw Error("Distinct whole child profile and generated output paths required");
  for(const path of[profilePath,outputPath]){const distance=relative(generated,resolve(path));if(isAbsolute(distance)||distance===".."||distance.startsWith(".."+sep))throw Error("Ticket generated profile paths required");}
  const routing=await import(join(root,"🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts")),control=await import(join(root,"🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts"));
  const body=async(original:any)=>{
    if(original.policy.maximumElapsedMilliseconds===0||original.policy.maximumElapsedMilliseconds>limits.maximumMilliseconds)throw Error("Original finite 60-second aggregation authority required");
    const checkpoint=async(index:number,stage:string)=>{control.checkScriptInvocation(original);if(index%4096===0)await control.advanceScriptInvocation(original,"cpu:"+stage,"running");};
    const before=await lstat(profilePath,{bigint:true});if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size>BigInt(limits.maximumBytes))throw Error("Bounded regular complete profile required");
    const file=await open(profilePath,constants.O_RDONLY|(constants.O_NOFOLLOW??0));let bytes:Buffer;
    try{const admitted=await file.stat({bigint:true});if(["dev","ino","size","mtimeNs","ctimeNs"].some(key=>before[key]!==admitted[key]))throw Error("Original flushed profile descriptor required");bytes=Buffer.alloc(Number(before.size));let position=0;while(position<bytes.length){await checkpoint(position,"read");const read=await file.read(bytes,position,Math.min(1048576,bytes.length-position),position);if(!read.bytesRead)throw Error("Complete bounded profile required");position+=read.bytesRead;}const settled=await file.stat({bigint:true});if(["dev","ino","size","mtimeNs","ctimeNs"].some(key=>before[key]!==settled[key]))throw Error("Flushed profile changed during read");}finally{await file.close();}
    const profile=JSON.parse(bytes.toString());control.checkScriptInvocation(original);
    if(!Array.isArray(profile.nodes)||!Array.isArray(profile.samples)||!Array.isArray(profile.timeDeltas)||profile.nodes.length>limits.maximumNodes||profile.samples.length>limits.maximumSamples||profile.samples.length!==profile.timeDeltas.length||!Number.isFinite(profile.startTime)||!Number.isFinite(profile.endTime)||profile.endTime<profile.startTime)throw Error("Closed bounded CPU profile required");
    const nodes=new Map<number,any>(),parents=new Map<number,number>(),groups:any[]=[],groupIds=new Map<string,number>();
    for(let index=0;index<profile.nodes.length;index++){
      await checkpoint(index,"nodes");const value=profile.nodes[index],frame=value.callFrame;if(!Number.isSafeInteger(value.id)||nodes.has(value.id)||!frame||typeof frame.functionName!=="string"||typeof frame.url!=="string"||!Number.isSafeInteger(frame.lineNumber)||!Number.isSafeInteger(frame.columnNumber)||value.children!==undefined&&!Array.isArray(value.children))throw Error("Closed unique profile node required");
      const location={function:frame.functionName,path:frame.url.startsWith("file:")?fileURLToPath(frame.url):frame.url,line:frame.lineNumber+1,column:frame.columnNumber+1},key=JSON.stringify(location);let group=groupIds.get(key);if(group===undefined){group=groups.length;groupIds.set(key,group);groups.push({...location,selfSamples:0,inclusiveSamples:0,selfIntervalMicroseconds:0,inclusiveIntervalMicroseconds:0});}
      const node={id:value.id,children:value.children??[],group,self:0,weight:0,total:0,totalWeight:0,topmost:false,depth:0};nodes.set(value.id,node);
      for(const child of node.children){if(!Number.isSafeInteger(child)||parents.has(child))throw Error("Unique profile parent required");parents.set(child,value.id);}
    }
    const roots=[...nodes.values()].filter(node=>!parents.has(node.id));if(roots.length!==1)throw Error("Single rooted profile required");for(const[id,parent]of parents)if(!nodes.has(id)||!nodes.has(parent))throw Error("Missing profile child required");
    const order:any[]=[],active=new Map<number,number>(),stack=[{id:roots[0].id,exit:false,depth:0}],visited=new Set<number>();
    let treeSteps=0;while(stack.length){await checkpoint(++treeSteps,"tree");const current=stack.pop()!,node=nodes.get(current.id)!;if(current.exit){const count=active.get(node.group)!-1;if(count)active.set(node.group,count);else active.delete(node.group);continue;}if(visited.has(node.id))throw Error("Acyclic profile required");visited.add(node.id);node.depth=current.depth;node.topmost=!active.has(node.group);active.set(node.group,(active.get(node.group)??0)+1);order.push(node);stack.push({...current,exit:true});for(let i=node.children.length-1;i>=0;i--)stack.push({id:node.children[i],exit:false,depth:current.depth+1});}
    if(order.length!==nodes.size)throw Error("Connected profile required");
    let deltaSum=0,minimumDelta=Infinity,maximumDelta=0;
    for(let index=0;index<profile.samples.length;index++){
      await checkpoint(index,"samples");const node=nodes.get(profile.samples[index]),delta=profile.timeDeltas[index];if(!node||!Number.isFinite(delta)||delta<0)throw Error("Known profile sample and nonnegative delta required");node.self++;node.weight+=delta;deltaSum+=delta;minimumDelta=Math.min(minimumDelta,delta);maximumDelta=Math.max(maximumDelta,delta);
    }
    let oracleRows=0;for(let index=order.length-1;index>=0;index--){await checkpoint(index,"inclusive");const node=order[index],frame=groups[node.group];node.total+=node.self;node.totalWeight+=node.weight;frame.selfSamples+=node.self;frame.selfIntervalMicroseconds+=node.weight;if(node.topmost){frame.inclusiveSamples+=node.total;frame.inclusiveIntervalMicroseconds+=node.totalWeight;}const parent=nodes.get(parents.get(node.id)!);if(parent){parent.total+=node.total;parent.totalWeight+=node.totalWeight;}if(node.self)oracleRows+=node.depth+1;}
    if(oracleRows>limits.maximumOracleRows)throw Error("Bounded independent stack traversal exceeded");
    const database=new Database(":memory:");let oracleChecks=0;
    try{
      database.exec("CREATE TABLE nodes(id INTEGER PRIMARY KEY,parent INTEGER,frame INTEGER);CREATE TABLE samples(node INTEGER,delta REAL)");const putNode=database.prepare("INSERT INTO nodes VALUES(?,?,?)"),putSample=database.prepare("INSERT INTO samples VALUES(?,?)");
      database.exec("BEGIN");for(let index=0;index<order.length;index++){await checkpoint(index,"oracle-nodes");const node=order[index];putNode.run(node.id,parents.get(node.id)??null,node.group);}for(let index=0;index<profile.samples.length;index++){await checkpoint(index,"oracle-samples");putSample.run(profile.samples[index],profile.timeDeltas[index]);}database.exec("COMMIT");
      const self:any[]=database.query("SELECT n.frame,COUNT(*) samples FROM samples s JOIN nodes n ON n.id=s.node GROUP BY n.frame").all();control.checkScriptInvocation(original);const expectedSelf=groups.filter(frame=>frame.selfSamples>0);if(self.length!==expectedSelf.length||self.some(row=>groups[row.frame].selfSamples!==row.samples))throw Error("Independent SQLite self samples disagree");oracleChecks++;
      const inclusive:any[]=database.query("WITH RECURSIVE counts AS(SELECT node,COUNT(*) samples FROM samples GROUP BY node),walk(origin,id,samples) AS(SELECT node,node,samples FROM counts UNION ALL SELECT walk.origin,n.parent,walk.samples FROM walk JOIN nodes n ON n.id=walk.id WHERE n.parent IS NOT NULL),dedup AS(SELECT walk.origin,n.frame,MAX(walk.samples) samples FROM walk JOIN nodes n ON n.id=walk.id GROUP BY walk.origin,n.frame) SELECT frame,SUM(samples) samples FROM dedup GROUP BY frame").all();control.checkScriptInvocation(original);const expectedInclusive=groups.filter(frame=>frame.inclusiveSamples>0);if(inclusive.length!==expectedInclusive.length||inclusive.some(row=>groups[row.frame].inclusiveSamples!==row.samples))throw Error("Independent SQLite inclusive samples disagree");oracleChecks++;
    }finally{database.close();}
    const categories=new Map<string,any>();for(const frame of groups){let category="other-source";if(!frame.path)category=/^\(idle\)$/iu.test(frame.function)?"explicit-idle":/^\((?:garbage collector|gc)\)$/iu.test(frame.function)?"explicit-gc":"unattributed-or-native";else if(frame.path.startsWith("node:")||frame.path.startsWith("internal:"))category="runtime-module";else if(frame.path.startsWith(root+sep))category=frame.path.includes("/node_modules/")||frame.path.includes("/.cache/")?"dependency-or-cache":"repository-source";const row=categories.get(category)??{category,samples:0,intervalMicroseconds:0};row.samples+=frame.selfSamples;row.intervalMicroseconds+=frame.selfIntervalMicroseconds;categories.set(category,row);}
    const after=await lstat(profilePath,{bigint:true});if(["dev","ino","size","mtimeNs","ctimeNs"].some(key=>before[key]!==after[key]))throw Error("Flushed profile changed during aggregation");control.checkScriptInvocation(original);
    const hotStacks=[...nodes.values()].filter(node=>node.self).sort((a,b)=>b.self-a.self).slice(0,20).map(node=>{const frames=[];for(let current:any=node;current;current=nodes.get(parents.get(current.id)!))frames.unshift(groups[current.group]);return{samples:node.self,intervalMicroseconds:node.weight,frames:frames.map(({function:name,path,line,column})=>({function:name,path,line,column}))};});
    const frames=groups.filter(frame=>frame.inclusiveSamples>0).map(frame=>({...frame,selfShare:profile.samples.length?frame.selfSamples/profile.samples.length:0,inclusiveShare:profile.samples.length?frame.inclusiveSamples/profile.samples.length:0})),result={scope:"whole actual child profile only; no invented phase or normalizer window; child subprocess CPU excluded",limits,input:resolve(profilePath),inputBytes:bytes.length,profileSha256:new Bun.CryptoHasher("sha256").update(bytes).digest("hex"),nodes:nodes.size,samples:profile.samples.length,profileSpanMicroseconds:profile.endTime-profile.startTime,deltaSumMicroseconds:deltaSum,tailMicroseconds:profile.endTime-profile.startTime-deltaSum,minimumDeltaMicroseconds:profile.samples.length?minimumDelta:null,maximumDeltaMicroseconds:maximumDelta,categories:[...categories.values()],frames,hotStacks,independentSqliteOracle:{checks:oracleChecks,estimatedRows:oracleRows,exactSelfAndInclusiveAgreement:true},limitations:["Self and inclusive sample counts are separate; recursive exact frame occurrences are deduplicated per stack","Inclusive shares overlap and must not be summed","Delta weights describe sampled elapsed intervals, not proven CPU durations","No explicit idle/GC labels proves neither absence nor attribution of unknown native samples","This process profile excludes CPU used by child processes","No phase attribution is made without exact clock alignment"]};const source=JSON.stringify(result,null,2);if(Buffer.byteLength(source)>limits.maximumOutputBytes)throw Error("Bounded aggregation output exceeded");await Bun.write(outputPath,source);control.checkScriptInvocation(original);console.log("[DEBUG] "+JSON.stringify({profile:resolve(profilePath),output:resolve(outputPath),bytes:bytes.length,nodes:nodes.size,samples:profile.samples.length,frames:frames.length,oracleChecks}));
  };
  if(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION)await routing.receiveScriptProcessInvocation(process.env,body);else await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"whole-child-cpu-aggregation",maximumElapsedMilliseconds:limits.maximumMilliseconds},{workspaceRoot:root},Date.now()),body);
}

if(process.argv[2]==="--whole-child"){await wholeChild();process.exit(0);}

const [profilePath, resultPath, outputPath] = process.argv.slice(2);
if (!profilePath || !resultPath || !outputPath) throw new Error("Profile, result and generated output paths required");
const profile = await Bun.file(profilePath).json();
const result = await Bun.file(resultPath).json();
const nodes = new Map<number, any>();
const parents = new Map<number, number>();
for (const node of profile.nodes) {
  if (nodes.has(node.id)) throw new Error("Duplicate node");
  nodes.set(node.id, node);
  for (const child of node.children ?? []) {
    if (parents.has(child)) throw new Error("Multiple parents");
    parents.set(child, node.id);
  }
}
if (profile.samples.length !== profile.timeDeltas.length) throw new Error("Sample/delta disagreement");
const location = (node: any) => {
  const frame = node.callFrame;
  return { function: frame.functionName, path: frame.url.startsWith("file:") ? fileURLToPath(frame.url) : frame.url, line: frame.lineNumber + 1, column: frame.columnNumber + 1 };
};
const category = (node: any) => {
  const frame = location(node);
  if (/garbage|\(gc\)/i.test(frame.function)) return "explicit-gc";
  if (/idle/i.test(frame.function)) return "explicit-idle";
  if (frame.path.startsWith("/Users/ueli/Documents/semio/")) return frame.path.includes("/node_modules/") || frame.path.includes("/.cache/") ? "dependency-or-cache" : "repository-source";
  if (frame.path.startsWith("node:") || frame.path.startsWith("internal:")) return "runtime-module";
  return frame.path ? "other-source" : "unattributed-or-native";
};
const windows = [
  { name: "whole", start: profile.startTime, end: profile.endTime },
  { name: "normalizerEpoch", start: result.epochStarted * 1000, end: result.epochEnded * 1000 },
];
const analyses: Record<string, any> = {};
for (const window of windows) {
  const self = new Map<string, any>(), inclusive = new Map<string, any>(), stacks = new Map<string, any>(), categories = new Map<string, any>(), nearestRepository = new Map<string, any>();
  let timestamp = profile.startTime, sampleCount = 0, intervalMicroseconds = 0;
  const add = (map: Map<string, any>, key: string, frame: any, count: number, micros: number) => {
    const entry = map.get(key) ?? { ...frame, samples: 0, intervalMicroseconds: 0 };
    entry.samples += count; entry.intervalMicroseconds += micros; map.set(key, entry);
  };
  for (let index = 0; index < profile.samples.length; index++) {
    const delta = profile.timeDeltas[index];
    if (!Number.isFinite(delta) || delta < 0) throw new Error("Invalid delta");
    const previous = timestamp; timestamp += delta;
    const count = timestamp >= window.start && timestamp <= window.end ? 1 : 0;
    const micros = Math.max(0, Math.min(timestamp, window.end) - Math.max(previous, window.start));
    if (!count && !micros) continue;
    const node = nodes.get(profile.samples[index]);
    if (!node) throw new Error("Missing sampled node");
    const frame = location(node), key = JSON.stringify(frame);
    add(self, key, frame, count, micros);
    add(categories, category(node), { category: category(node) }, count, micros);
    const chain: any[] = [], seen = new Set<number>(), frameKeys = new Set<string>();
    for (let cursor: any = node; cursor; cursor = nodes.get(parents.get(cursor.id)!)) {
      if (seen.has(cursor.id)) throw new Error("Cycle");
      seen.add(cursor.id);
      const current = location(cursor), currentKey = JSON.stringify(current);
      chain.unshift(current);
      if (!frameKeys.has(currentKey)) add(inclusive, currentKey, current, count, micros);
      frameKeys.add(currentKey);
    }
    add(stacks, JSON.stringify(chain), { frames: chain }, count, micros);
    const nearest = [...chain].reverse().find(frame => frame.path.startsWith("/Users/ueli/Documents/semio/") && !frame.path.includes("/node_modules/") && !frame.path.includes("/.cache/"));
    if (nearest) add(nearestRepository, JSON.stringify(nearest), nearest, count, micros);
    sampleCount += count; intervalMicroseconds += micros;
  }
  const sorted = (map: Map<string, any>) => [...map.values()].sort((a, b) => b.samples - a.samples || b.intervalMicroseconds - a.intervalMicroseconds);
  analyses[window.name] = { window, sampleCount, intervalMicroseconds, self: sorted(self), inclusive: sorted(inclusive), categories: sorted(categories), nearestRepository: sorted(nearestRepository), stacks: sorted(stacks) };
}
const ticks = new Map<string, any>();
for (const node of nodes.values()) for (const tick of node.positionTicks ?? []) {
  const frame = location(node), key = JSON.stringify([frame.path, tick.line]);
  const entry = ticks.get(key) ?? { path: frame.path, recordedLine: tick.line, ticks: 0 };
  entry.ticks += tick.ticks; ticks.set(key, entry);
}
const database = new Database(":memory:");
database.exec("CREATE TABLE nodes(id INTEGER PRIMARY KEY,parent INTEGER,name TEXT,url TEXT,line INTEGER,col INTEGER); CREATE TABLE samples(sequence INTEGER PRIMARY KEY,node INTEGER,delta REAL)");
const insertNode = database.prepare("INSERT INTO nodes VALUES(?,?,?,?,?,?)"), insertSample = database.prepare("INSERT INTO samples VALUES(?,?,?)");
database.transaction(() => {
  for (const node of nodes.values()) insertNode.run(node.id, parents.get(node.id) ?? null, node.callFrame.functionName, node.callFrame.url, node.callFrame.lineNumber, node.callFrame.columnNumber);
  profile.samples.forEach((id: number, index: number) => insertSample.run(index, id, profile.timeDeltas[index]));
})();
let oracleChecks = 0;
for (const window of windows) for (const mode of ["self", "inclusive"]) {
  const recursive = mode === "inclusive" ? " UNION ALL SELECT walk.sequence,n.parent FROM walk JOIN nodes n ON n.id=walk.id WHERE n.parent IS NOT NULL" : "";
  const rows: any[] = database.prepare(`WITH RECURSIVE timing AS (SELECT sequence,node,?+SUM(delta) OVER(ORDER BY sequence) timestamp FROM samples), walk(sequence,id) AS (SELECT sequence,node FROM timing WHERE timestamp>=? AND timestamp<=?${recursive}) SELECT n.name,n.url,n.line,n.col,COUNT(DISTINCT walk.sequence) samples FROM walk JOIN nodes n ON n.id=walk.id GROUP BY n.name,n.url,n.line,n.col`).all(profile.startTime, window.start, window.end);
  const expected = new Map(analyses[window.name][mode].filter((row: any) => row.samples).map((row: any) => [JSON.stringify([row.function,row.path,row.line,row.column]),row.samples]));
  if (rows.length !== expected.size) throw new Error("SQLite group count disagrees");
  for (const row of rows) {
    const key = JSON.stringify([row.name,row.url.startsWith("file:") ? fileURLToPath(row.url) : row.url,row.line+1,row.col+1]);
    if (expected.get(key) !== row.samples) throw new Error("SQLite sample aggregation disagrees");
  }
  oracleChecks++;
}
database.close();
const schemaCallers = new Map<string, any>();
for (const stack of analyses.normalizerEpoch.stacks) {
  const index = stack.frames.findIndex((frame: any) => frame.function === "validateJsonSchemaSubset" && frame.path.includes("validator"));
  if (index < 1) continue;
  const caller = stack.frames[index - 1], key = JSON.stringify(caller), entry = schemaCallers.get(key) ?? { caller, samples: 0, intervalMicroseconds: 0, example: stack.frames.slice(Math.max(0,index-6),index) };
  entry.samples += stack.samples; entry.intervalMicroseconds += stack.intervalMicroseconds; schemaCallers.set(key, entry);
}
const output = {
  profilePath, resultPath, result,
  profileSha256: new Bun.CryptoHasher("sha256").update(await Bun.file(profilePath).arrayBuffer()).digest("hex"),
  profileSpanMicroseconds: profile.endTime - profile.startTime,
  deltaSumMicroseconds: profile.timeDeltas.reduce((a: number, b: number) => a + b, 0),
  minimumDeltaMicroseconds: Math.min(...profile.timeDeltas), maximumDeltaMicroseconds: Math.max(...profile.timeDeltas),
  nodes: nodes.size, samples: profile.samples.length,
  independentSqliteOracle: { checks: oracleChecks, exactSelfAndInclusiveAgreement: true, windows: windows.map(window => window.name) },
  analyses,
  schemaCallers: [...schemaCallers.values()].sort((a, b) => b.samples - a.samples),
  wholeProfilePositionTicks: [...ticks.values()].sort((a, b) => b.ticks - a.ticks),
  limitations: ["Time-delta weights describe preceding sample intervals, not measured CPU duration", "Epoch interval uses Date.now millisecond precision; process elapsed is separately retained", "Inclusive frame counts deduplicate recursion within each stack and overlap between different frames", "positionTicks have no timestamps and apply only to the whole profile", "No explicit idle/GC frame does not establish absence of idle/GC/native time"],
};
await Bun.write(outputPath, JSON.stringify(output, null, 2));
for (const [name, analysis] of Object.entries(analyses)) console.log(JSON.stringify({ name, samples: analysis.sampleCount, intervalMilliseconds: analysis.intervalMicroseconds / 1000, categories: analysis.categories, self: analysis.self.slice(0, 15), inclusive: analysis.inclusive.slice(0, 18) }));
