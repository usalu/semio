/** 🧹️ Retires only closed failed copy duplicates after complete retained-body and physical custody admission. */
import assert from "node:assert/strict";
import {readFile,writeFile,mkdir,readdir,lstat,unlink,rmdir} from "node:fs/promises";
import {createReadStream} from "node:fs";
import {createInterface} from "node:readline";
import {createHash} from "node:crypto";
import {dirname,join,resolve,isAbsolute,relative} from "node:path";
const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),[command,epoch]=process.argv.slice(2);
assert(command==="inspect"||command==="retire");assert.match(epoch??"",/^\d+$/u);
const {observePhysicalFileV1}=await import(join(root,"🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🟦️.ts")),output=join(ticket,"🗑️generated/physical-observation-copy-retirement",command+"-"+epoch),producer=await readFile(import.meta.path,"utf8");
try{await lstat(output);throw Error("fresh retirement epoch required");}catch(error){if((error as {code?:string}).code!=="ENOENT")throw error;}await mkdir(output,{recursive:true});
let cancelled=false,files=0,bytes=0,deleted=0,failure:string|null=null;const stop=()=>{cancelled=true;};process.once("SIGINT",stop);process.once("SIGTERM",stop);const started=Date.now(),control={maxBytes:8*1024*1024*1024,maxWork:16384,chunkBytes:1048576,cancelled:()=>cancelled,remainingMs:()=>300000-(Date.now()-started),onProgress:()=>{}};
const check=()=>{assert(!cancelled,"retirement cancelled");assert(control.remainingMs()>0,"retirement deadline");},identity=(stat:Awaited<ReturnType<typeof lstat>>)=>({dev:stat.dev,ino:stat.ino,size:stat.size,mtime:stat.mtimeMs,ctime:stat.ctimeMs}),same=(a:unknown,b:unknown)=>JSON.stringify(a)===JSON.stringify(b),rows:any[]=[],roots:any[]=[];
try{
 for(const number of[1,2]){
  const base=join(ticket,"🗑️generated/physical-observation-deletion/proof-"+number),copy=join(base,"workspace"),journal=join(base,"source-bodies.jsonl"),terminalPath=join(base,"terminal.json"),terminal=JSON.parse(await readFile(terminalPath,"utf8"));assert(terminal.error&&!terminal.deletionExecuted&&!terminal.result,"only pre-runtime failed copies retire");
  const owner=identity(await lstat(copy)),journalClaim=await observePhysicalFileV1(journal,control),terminalClaim=await observePhysicalFileV1(terminalPath,control),seen=new Set<string>(),directories:string[]=[],pending=[copy];let actual=0;
  const lines=createInterface({input:createReadStream(journal),crlfDelay:Infinity});
  for await(const line of lines){check();const row=JSON.parse(line);assert(typeof row.path==="string"&&!isAbsolute(row.path)&&!row.path.split(/[\\/]/u).some((part:string)=>part===".."||part===""));assert(!seen.has(row.path));seen.add(row.path);const path=resolve(copy,row.path);assert(relative(copy,path)!==""&&!relative(copy,path).startsWith(".."));const body=Buffer.from(row.body,"base64"),digest=createHash("sha256").update(body).digest("hex");assert.equal(body.toString("base64"),row.body);assert.equal(body.length,row.claim.byteLength);assert.equal(digest,row.claim.sha256);const before=await lstat(path);assert(before.isFile()&&!before.isSymbolicLink());const claim=await observePhysicalFileV1(path,control),after=await lstat(path);assert.equal(claim.sha256,digest);assert.equal(claim.byteLength,body.length);assert(same(identity(before),identity(after)));rows.push({path,claim,identity:identity(after),number});files++;bytes+=body.length;body.fill(0);assert(files<=32768);if(files%256===0)console.log(`[DEBUG] copy duplicate custody files=${files} bytes=${bytes}`);}
  while(pending.length){check();const directory=pending.pop()!,stat=await lstat(directory);assert(stat.isDirectory()&&!stat.isSymbolicLink());directories.push(directory);for(const entry of await readdir(directory,{withFileTypes:true})){const path=join(directory,entry.name),metadata=await lstat(path);assert(!metadata.isSymbolicLink());if(metadata.isDirectory())pending.push(path);else{assert(metadata.isFile());actual++;assert(seen.has(relative(copy,path).replaceAll("\\","/")));}}}
  assert.equal(actual,seen.size);assert(same(identity(await lstat(copy)),owner));roots.push({number,copy,owner,journal,journalClaim,terminalPath,terminalClaim,directories,actual});
 }
 assert.equal(await readFile(import.meta.path,"utf8"),producer);await writeFile(join(output,"admission.json"),JSON.stringify({producer,rows,roots,files,bytes,atomicRetirementClaimed:false,sourceWrites:0}));
 if(command==="retire"){
  for(const owner of roots){for(const[path,claim]of[[owner.journal,owner.journalClaim],[owner.terminalPath,owner.terminalClaim]])assert(same(await observePhysicalFileV1(path,control),claim));assert(same(identity(await lstat(owner.copy)),owner.owner));}
  for(const row of rows){check();assert(same(identity(await lstat(row.path)),row.identity));assert(same(await observePhysicalFileV1(row.path,control),row.claim));await unlink(row.path);deleted++;if(deleted%256===0)console.log(`[DEBUG] copy duplicate retired files=${deleted}/${files}`);}
  for(const owner of roots)for(const directory of owner.directories.sort((a:string,b:string)=>b.length-a.length)){check();await rmdir(directory);}
 }
}catch(error){failure=String(error);process.stderr.write(`[DEBUG] ${failure}\n`);process.exitCode=1;}
finally{await writeFile(join(output,"terminal.json"),JSON.stringify({command,files,bytes,deleted,failure,producerExact:producer===await readFile(import.meta.path,"utf8"),roots:roots.map(owner=>({number:owner.number,copy:owner.copy,journal:owner.journal,terminalPath:owner.terminalPath,actual:owner.actual})),sourceWrites:0,atomicRetirementClaimed:false}));process.off("SIGINT",stop);process.off("SIGTERM",stop);}
