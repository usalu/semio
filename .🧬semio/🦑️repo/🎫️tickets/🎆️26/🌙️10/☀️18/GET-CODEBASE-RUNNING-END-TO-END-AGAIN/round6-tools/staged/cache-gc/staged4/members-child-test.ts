import{test,expect}from"bun:test";
import{lstatSync,mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{spawnSync}from"node:child_process";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{createHash}from"node:crypto";
import{brotliCompressSync}from"node:zlib";
import{fileURLToPath}from"node:url";
import closures from"../../../🧬️schema/🔣️.json";

const sha=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex"),worker=fileURLToPath(new URL("../../../👷️read.mjs",import.meta.url)),P=closures.definitions.Protocol.const;
const stamp=(path:string)=>{const row=lstatSync(path,{bigint:true});return[row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].join(":");};

test("the closure read child reports each leaf's own hash, references and links, so an index can record them per leaf",()=>{
 const root=mkdtempSync(join(tmpdir(),"mc-")),group=join(root,"inference","selections","a".repeat(64)),block=sha("block"),closure=sha("closure");
 try{
  mkdirSync(join(group,"np"),{recursive:true});mkdirSync(join(group,"rr"),{recursive:true});
  const body=JSON.stringify({value:"SEMIO_SOURCE_CLOSURE_"+closure}),header=JSON.stringify({authorityObservations:[],blocks:[block],digest:sha("d"),observations:[],resultSha256:sha(body)}),leaf=join(group,"np","1".repeat(24)+".json"),plain=join(group,"np","2".repeat(24)+".json");
  writeFileSync(leaf,brotliCompressSync(Buffer.from(header+"\n"+body)));
  const secondBody=JSON.stringify({value:"nothing"}),secondHeader=JSON.stringify({authorityObservations:[],blocks:[],digest:sha("e"),observations:[],resultSha256:sha(secondBody)});writeFileSync(plain,brotliCompressSync(Buffer.from(secondHeader+"\n"+secondBody)));
  const files=[leaf,plain].map(path=>({path,stamp:stamp(path),kind:"inference",bytes:lstatSync(path).size})),request=JSON.stringify({files,imageRoot:root,remainingBytes:P.maximumProofBytes,root:process.cwd()});
  const run=spawnSync("node",["--max-old-space-size="+P.heapMiB,worker],{input:request,cwd:root,encoding:"utf8",maxBuffer:P.maximumResultBytes});
  expect(run.status,run.stderr).toBe(0);
  const result=JSON.parse(run.stdout);expect(Object.keys(result).sort()).toEqual(["rows","version"]);expect(result.rows).toHaveLength(2);
  expect(result.rows[0]).toMatchObject({path:leaf,hash:sha(brotliCompressSync(Buffer.from(header+"\n"+body))),publicationGraph:null,references:["source:"+closure],links:[{path:join(dirname(leaf),"ob",block+".br"),missing:"block"}]});
  expect(result.rows[1]).toMatchObject({path:plain,references:[],links:[]});
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);
