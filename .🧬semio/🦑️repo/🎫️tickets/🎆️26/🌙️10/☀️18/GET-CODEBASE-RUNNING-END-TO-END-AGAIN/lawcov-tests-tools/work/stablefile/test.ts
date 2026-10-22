/** 🧊 Contract: capturing one build input reads a regular file whole through one retained descriptor, equal to the bytes a second reader digests, spends the shared budget exactly by the file size, refuses links, directories, oversized files and files that outgrow the budget, checks for cancellation before the first access and before every 64 KiB chunk, and reports a file that changes while it is read. */
import {afterAll,beforeAll,expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {appendFileSync,mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,truncateSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "@FW/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {readStableBuildFile} from "@UNIT/🟦️.ts";
import corpus from "@CORPUS";
import corpusSchema from "@SCHEMA";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"@ORACLE");
const classify=(paths:string[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({paths}),encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const content=(size:number):Buffer=>{const bytes=Buffer.alloc(size);for(let index=0;index<size;index++)bytes[index]=(index*31+7)&0xff;return bytes;};
let root="";
beforeAll(()=>{root=realpathSync(mkdtempSync(join(tmpdir(),"stable-file-")));});
afterAll(()=>{rmSync(root,{recursive:true,force:true});});

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,chunk:4096})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[0],kind:"socket"}]})).toBe(false);
});

test("every case captures, refuses and spends the budget as authored, with the authored number of cancellation checkpoints",()=>{
 const paths:string[]=[];
 for(const[index,row]of corpus.cases.entries()){
  const directory=join(root,String(index));mkdirSync(directory);const path=join(directory,"input.bin");paths.push(path);
  if(row.kind==="file")writeFileSync(path,content(row.size));
  else if(row.kind==="symlink"){writeFileSync(join(directory,"target.bin"),content(row.size));if(process.platform!=="win32")symlinkSync("target.bin",path);else paths[paths.length-1]=join(directory,"target.bin");}
  else if(row.kind==="directory")mkdirSync(path);
  if(row.kind==="symlink"&&process.platform==="win32")continue;
  const admission={remaining:row.remaining};let checks=0;
  const check=()=>{
   checks++;
   if(row.cancelAt===checks)throw Error("cancelled at check "+checks);
   if(row.mutateAt===checks){
    if(row.mutation==="append")appendFileSync(path,"x");
    else if(row.mutation==="touch"){const when=new Date(Date.now()+60_000);utimesSync(path,when,when);}
    else truncateSync(path,row.size-10);
   }
  };
  let bytes:Uint8Array|undefined,failure:(Error&{code?:string})|undefined;
  try{bytes=readStableBuildFile(path,row.maximum,admission,check);}catch(error){failure=error as Error;}
  expect(admission.remaining,row.name).toBe(row.remainingAfter);
  expect(checks,row.name).toBe(row.checks);
  if((row as any).ok){
   expect(failure,row.name+" "+failure?.message).toBeUndefined();
   expect(bytes!.byteLength).toBe(row.size);
   expect(createHash("sha256").update(bytes!).digest("hex"),row.name).toBe(createHash("sha256").update(content(row.size)).digest("hex"));
   const [seen]=classify([path]);expect(seen,row.name).toEqual({kind:"file",size:row.size,sha256:createHash("sha256").update(bytes!).digest("hex")});
  }else{
   expect(failure,row.name).toBeDefined();
   if((row as any).error)expect(failure!.message,row.name).toContain((row as any).error);
   if((row as any).errorAny)expect((row as any).errorAny.some((text:string)=>failure!.message.includes(text)),row.name+" "+failure!.message).toBe(true);
   if((row as any).errorCode)expect(failure!.code,row.name).toBe((row as any).errorCode);
   expect(bytes,row.name).toBeUndefined();
  }
 }
 const kinds=classify(paths.filter((_,index)=>corpus.cases[index]!.kind!=="file"&&corpus.cases[index]!.kind!=="symlink"));
 expect(kinds.map(row=>row.kind)).toEqual(corpus.cases.filter(row=>row.kind==="directory"||row.kind==="missing").map(row=>row.kind));
});

test("the refusal message of a non regular file and of an oversized file names the path and the bound",()=>{
 const directory=join(root,"messages");mkdirSync(directory);
 const target=join(directory,"d");mkdirSync(target);
 expect(()=>readStableBuildFile(target,10,{remaining:10},()=>undefined)).toThrow(`build input: not a regular file (${target})`);
 const big=join(directory,"big");writeFileSync(big,Buffer.alloc(11));
 expect(()=>readStableBuildFile(big,10,{remaining:100},()=>undefined)).toThrow(`build input: ${big} is 11 bytes, over the 10-byte bound`);
});
