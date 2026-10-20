import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { chmodSync, linkSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, unlinkSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { Database } from "bun:sqlite";

type Row = { path: string; kind: "directory" | "file" | "link"; bytes?: number; sha256?: string; executable?: boolean; target?: string };
const ticket=dirname(import.meta.dir),root=resolve(process.cwd()),fixture=JSON.parse(readFileSync(join(import.meta.dir,"🔣️.json"),"utf8")),generated=join(ticket,"🗑️generated");
const base="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/";
const sourceRows=[
 ["📥️reader/🟦️.ts","2bd3d090eefacaf699ec62cbfaf3ae56c1c892fc8afdc6dbb6792633a50b3714"],
 ["🔏️index/📂️tree/🟦️.ts","b8943f495e59926567e3606cace5e7a357e03456ffa234181be213a7fa116e74"],
 ["📚️layers/🟦️.ts","9daedd61f36dfc84eebb927da75bf67eac219d9b85c3ae95d6ffcd2a8ff94ff3"]
];
for(const [path,sha256] of sourceRows)assert.equal(createHash("sha256").update(readFileSync(join(root,base,path!))).digest("hex"),sha256);
const subject=await import(join(root,base,"📥️reader/🟦️.ts"));
const indexSubject=await import(join(root,base,"🔏️index/🟦️.ts"));
assert.equal(typeof subject.verifyNxSdkDerivativeNamespaceReadV1,"function");
const controller=new AbortController(),cancel=()=>controller.abort(Error("Neutral namespace cancelled")),timer=setTimeout(()=>controller.abort(Error("Neutral namespace timeout")),fixture.maximumMilliseconds);
process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
const workspace=mkdtempSync(join(generated,"ns-")),generation=join(workspace,"nx","g"),seed=join(workspace,"seed"),rows:Row[]=[],levels=new Map<number,string[]>(),seedBytes=Buffer.from("x"),seedSha=createHash("sha256").update(seedBytes).digest("hex"),db=new Database(":memory:");
let checks=0,published=0,yielded=0;
const control={signal:controller.signal,remainingMilliseconds:()=>fixture.maximumMilliseconds,publish:async()=>{published++;if(published%128===0)console.log("[DEBUG] Neutral namespace progress "+JSON.stringify({published}));},yieldContinuation:async()=>{yielded++;await new Promise<void>(done=>setImmediate(done));}};
const check=(actual:unknown)=>{assert.equal(actual,control);controller.signal.throwIfAborted();checks++;};
const context={root:workspace,control,check,recheck:async()=>check(control)};
const guard=()=>controller.signal.throwIfAborted();
const results:Record<string,unknown>={scope:"Actual unchanged namespace callback on synthetic host filesystem with neutral control ports; no Native admission, private process or managed test pass",sources:sourceRows,fixtureEntries:fixture.entries};
try{
 mkdirSync(generation,{recursive:true});writeFileSync(seed,seedBytes);chmodSync(seed,0o444);
 for(const [depthText,total] of Object.entries(fixture.directoryDepthHistogram) as [string,number][]){
  const depth=Number(depthText),parents=levels.get(depth-1)??[],paths:string[]=[];
  for(let i=0;i<total;i++){guard();const path=depth===1?"node_modules":depth===2&&i===0?"node_modules/nx":parents[i%parents.length]+"/d"+i.toString(36);assert.ok(path);mkdirSync(join(generation,path));rows.push({path,kind:"directory"});paths.push(path);}
  levels.set(depth,paths);
 }
 let links=0,files=0;
 for(const [depthText,total] of Object.entries(fixture.depthHistogram) as [string,number][]){
  const depth=Number(depthText),count=total-(fixture.directoryDepthHistogram[depthText]??0),parents=levels.get(depth-1)??[];
  for(let i=0;i<count;i++){
   guard();const path=parents[i%parents.length]+"/f"+i.toString(36),absolute=join(generation,path);assert.ok(absolute.length<=256);
   if(links<fixture.links){symlinkSync(".",absolute);rows.push({path,kind:"link",target:"."});links++;}
   else{if(path.startsWith("node_modules/nx/")){writeFileSync(absolute,seedBytes);chmodSync(absolute,0o444);}else linkSync(seed,absolute);rows.push({path,kind:"file",bytes:1,sha256:seedSha,executable:false});files++;}
   if((files+links)%256===0)await new Promise<void>(done=>setImmediate(done));
  }
 }
 rows.sort((a,b)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path)));
 assert.equal(rows.length,fixture.entries);assert.equal(files,fixture.files);assert.equal(links,fixture.links);
 db.exec("CREATE TABLE entries(path TEXT PRIMARY KEY, kind TEXT, depth INTEGER)");
 const insert=db.prepare("INSERT INTO entries VALUES(?,?,?)");
 db.transaction(()=>{for(const row of rows)insert.run(row.path,row.kind,row.path.split("/").length);})();
 const counts=db.query("SELECT kind,COUNT(*) AS total FROM entries GROUP BY kind ORDER BY kind").all() as {kind:string;total:number}[];
 assert.deepEqual(counts,[{kind:"directory",total:fixture.directories},{kind:"file",total:fixture.files},{kind:"link",total:fixture.links}]);
 const depths=db.query("SELECT depth,COUNT(*) AS total FROM entries GROUP BY depth ORDER BY depth").all() as {depth:number;total:number}[];
 assert.deepEqual(Object.fromEntries(depths.map(row=>[row.depth,row.total])),fixture.depthHistogram);
 const independent=(db.query("SELECT COUNT(*) total FROM entries WHERE kind='file' AND path LIKE 'node_modules/nx/%'").get() as {total:number}).total;assert.ok(independent>0);results.fileRoles={independentNx:independent,sharedReadOnly:files-independent};
 assert.equal((db.query("SELECT CAST((4*COUNT(*)-1)/128 AS INTEGER) AS checkpoints FROM entries").get() as {checkpoints:number}).checkpoints,fixture.expectedCheckpoints);
 const index=indexSubject.readNxSdkSourceIndexV1({version:1,snapshot:"0".repeat(64),entries:rows}),stamp=lstatSync(seed),known=new Map(rows.filter(row=>row.kind==="file").map(row=>[row.path,Object.freeze({device:stamp.dev,inode:stamp.ino})]));
 let start=performance.now();await subject.verifyNxSdkDerivativeNamespaceReadV1(generation,index,known,context,control);
 results.normal={milliseconds:performance.now()-start,published,yielded,checks};assert.equal(published,fixture.expectedCheckpoints);assert.equal(yielded,published);
 const extra=join(generation,"node_modules","extra");writeFileSync(extra,"x");start=performance.now();
 await assert.rejects(()=>subject.verifyNxSdkDerivativeNamespaceReadV1(generation,index,known,context,control),/Exact derivative namespace required/);
 results.unexpectedChild={refused:true,milliseconds:performance.now()-start};unlinkSync(extra);
 const link=rows.find(row=>row.kind==="link")!;unlinkSync(join(generation,link.path));symlinkSync("..",join(generation,link.path));start=performance.now();
 await assert.rejects(()=>subject.verifyNxSdkDerivativeNamespaceReadV1(generation,index,known,context,control),/Derivative module link changed/);
 results.symlinkTarget={refused:true,milliseconds:performance.now()-start};
 unlinkSync(join(generation,link.path));symlinkSync(".",join(generation,link.path));const nx=rows.find(row=>row.kind==="file"&&row.path.startsWith("node_modules/nx/"))!;unlinkSync(join(generation,nx.path));linkSync(seed,join(generation,nx.path));start=performance.now();
 await assert.rejects(()=>subject.verifyNxSdkDerivativeNamespaceReadV1(generation,index,known,context,control),/Independent patched Nx source required/);
 results.nxHardlink={refused:true,milliseconds:performance.now()-start};
 controller.abort(Error("Neutral namespace cancellation sentinel"));start=performance.now();
 await assert.rejects(()=>subject.verifyNxSdkDerivativeNamespaceReadV1(generation,index,known,context,control),/Neutral namespace cancellation sentinel/);
 results.cancelled={refused:true,milliseconds:performance.now()-start};
 for(const [path,sha256] of sourceRows)assert.equal(createHash("sha256").update(readFileSync(join(root,base,path!))).digest("hex"),sha256);
 results.oracle="SQLite relational kind/depth/checkpoint aggregates and unchanged Source pre/post hashes";
 console.log("[DEBUG] Neutral namespace GREEN "+JSON.stringify(results));writeFileSync(join(generated,"2026-10-20-namespace-neutral.json"),JSON.stringify(results,null,2)+"\n");
}finally{
 clearTimeout(timer);process.removeListener("SIGINT",cancel);process.removeListener("SIGTERM",cancel);db.close();rmSync(workspace,{recursive:true,force:true});
}

