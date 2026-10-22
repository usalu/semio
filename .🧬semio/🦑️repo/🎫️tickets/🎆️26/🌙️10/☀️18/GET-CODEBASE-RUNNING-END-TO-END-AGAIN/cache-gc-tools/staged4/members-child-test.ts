import{test,expect}from"bun:test";
import{lstatSync,mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{spawnSync}from"node:child_process";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{createHash}from"node:crypto";
import{brotliCompressSync}from"node:zlib";
import{fileURLToPath,pathToFileURL}from"node:url";
import closures from"../../../🧬️schema/🔣️.json";

const sha=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex"),worker=fileURLToPath(new URL("../../../👷️read.mjs",import.meta.url)),P=closures.definitions.Protocol.const;
const SESSION_CREDIT_V1=16764800;
const readSessionV1=(worker:string,cwd:string,packets:readonly string[]):string[]=>{
 const input=packets.map(packet=>JSON.stringify({version:1,maximumReplyBytes:SESSION_CREDIT_V1})+"\n"+packet+"\n").join("")+"null\n",run=spawnSync(process.versions.bun?"node":process.execPath,["--max-old-space-size="+P.heapMiB,"--input-type=module","-e","const module=await import(process.argv[1]);await module.runCapturedBuildSourceV1();",pathToFileURL(worker).href],{input,cwd,encoding:"utf8",maxBuffer:P.maximumResultBytes});
 if(run.status!==0)throw Error(run.stderr);
 return run.stdout.split("\n").filter(Boolean);
};
const stamp=(path:string)=>{const row=lstatSync(path,{bigint:true});return[row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].join(":");};

test("the closure read child answers a one-leaf request with that leaf's own hash, references and links, so an index can record them per leaf",()=>{
 const root=mkdtempSync(join(tmpdir(),"mc-")),group=join(root,"inference","selections","a".repeat(64)),block=sha("block"),closure=sha("closure");
 try{
  mkdirSync(join(group,"np"),{recursive:true});mkdirSync(join(group,"rr"),{recursive:true});
  const body=JSON.stringify({value:"SEMIO_SOURCE_CLOSURE_"+closure}),header=JSON.stringify({authorityObservations:[],blocks:[block],digest:sha("d"),observations:[],resultSha256:sha(body)}),leaf=join(group,"np","1".repeat(24)+".json"),plain=join(group,"np","2".repeat(24)+".json");
  writeFileSync(leaf,brotliCompressSync(Buffer.from(header+"\n"+body)));
  const secondBody=JSON.stringify({value:"nothing"}),secondHeader=JSON.stringify({authorityObservations:[],blocks:[],digest:sha("e"),observations:[],resultSha256:sha(secondBody)});writeFileSync(plain,brotliCompressSync(Buffer.from(secondHeader+"\n"+secondBody)));
  const requests=[leaf,plain].map(path=>({path,stamp:stamp(path),kind:"inference",bytes:lstatSync(path).size})),lines=readSessionV1(worker,root,requests.map(request=>JSON.stringify({files:[request],imageRoot:root,remainingBytes:P.maximumProofBytes,root:process.cwd()})));
  expect(lines).toHaveLength(2);
  const [first,second]=lines.map(line=>JSON.parse(line));
  expect(Object.keys(first).sort()).toEqual(["links","references","rows","version"]);
  expect(first.rows).toHaveLength(1);expect(first.rows[0]).toMatchObject({path:leaf,hash:sha(brotliCompressSync(Buffer.from(header+"\n"+body))),publicationGraph:null});
  expect(first.references).toEqual(["source:"+closure]);expect(first.links).toEqual([{path:join(dirname(leaf),"ob",block+".br"),missing:"block"}]);
  expect(second.rows[0]).toMatchObject({path:plain});expect(second.references).toEqual([]);expect(second.links).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);
