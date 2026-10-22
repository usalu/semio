/** ⏱️ Contract: recency summaries, the diagnostic funding decision and the retirement mismatch report give the answers of the unit's own corpus in `./🔣️.json`, of the extra corpus `./📋️extra.json` and of an independent Python implementation with arbitrary-precision integers; both corpora are closed by schemas. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "@FW/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {describeCacheRecencyChangeV1,fitsRecencySourceBytesV1,summarizeRecencyFilesV1,type RecencyFileV1} from "@UNIT/🟦️.ts";
import unitSchema from "@UNIT/🧬️schema/🔣️.json";
import own from "@UNIT/🧪️tests/🔣️.json";
import corpus from "@CORPUS";
import corpusSchema from "@SCHEMA";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"@ORACLE"),policy=unitSchema.definitions.ProvenancePolicy.const;
const ask=(request:object):any=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify(request),encoding:"utf8",maxBuffer:256*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const refusal=(action:()=>unknown):string=>{try{action();return "";}catch(error){return (error as Error).message;}};
const generated=(count:number):RecencyFileV1[]=>Array.from({length:count},(_,index)=>({path:"p"+String(index).padStart(4,"0"),device:"1",inode:"1",bytes:"1",atimeNs:String(index),mtimeNs:"0",ctimeNs:"0"}));
const rowsOf=(row:{rows:readonly RecencyFileV1[];generate?:number}):RecencyFileV1[]=>[...row.rows,...generated(row.generate??0)];

test("both corpora are closed by their schemas",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(unitSchema,own),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,summaries:[{...corpus.summaries[0],extra:1}]})).toBe(false);
 expect(ajv.validate(unitSchema,{...own,cases:own.cases.slice(1)})).toBe(false);
});

test("the unit's own cases hold and the pinned policy is the one the schema states",()=>{
 for(const row of own.cases)expect(summarizeRecencyFilesV1(row.rows),row.name).toEqual(row.expected);
 for(const row of own.funding)expect(fitsRecencySourceBytesV1(row.used,row.bytes),JSON.stringify(row)).toBe(row.expected);
 expect(policy).toEqual({version:1,maximumSourcePathBytes:1024,maximumSourceBytes:2048,maximumScanSourceBytes:1048576,maximumSourcesPerUnit:1,maximumMismatchBytes:8192});
});

test("summaries equal the Python summary and the corpus verdict for every extra case, with every refusal worded as authored",()=>{
 const answers=ask({op:"summarize",rows:corpus.summaries.map(row=>({rows:row.rows,generate:(row as any).generate}))});
 for(const[index,row]of corpus.summaries.entries()){
  let value:unknown=null;const message=refusal(()=>{value=summarizeRecencyFilesV1(rowsOf(row as any));});
  expect(message,row.name).toBe(row.error);expect(answers[index].error??"",row.name).toBe(row.error);
  if(row.error==="")expect(value,row.name).toEqual(answers[index].value);
 }
 const unordered=summarizeRecencyFilesV1(rowsOf(corpus.summaries[1] as any)),sorted=summarizeRecencyFilesV1([...rowsOf(corpus.summaries[1] as any)].reverse());
 expect(sorted).toEqual(unordered);expect(unordered.sources.map(source=>source.path)).toEqual(["a","a","b","b"]);
 expect(summarizeRecencyFilesV1(rowsOf(corpus.summaries[3] as any)).sources.length).toBe(8);
});

test("the diagnostic funding decision equals the Python formula at every boundary",()=>{
 const rows=[...own.funding,...corpus.funding],answers=ask({op:"fits",policy,rows});
 for(const[index,row]of rows.entries()){expect(fitsRecencySourceBytesV1(row.used,row.bytes),JSON.stringify(row)).toBe(row.expected);expect(answers[index],JSON.stringify(row)).toBe(row.expected);}
});

test("the mismatch report names what changed, keeps captured sources while small and drops them when oversized",()=>{
 const answers=ask({op:"describe",limit:policy.maximumMismatchBytes,rows:corpus.describe});
 for(const[index,row]of corpus.describe.entries()){
  const value=describeCacheRecencyChangeV1(row.root,row.expected as never,(row.current??undefined) as never);
  expect(value,row.name).toEqual(answers[index]);
  expect(Buffer.byteLength(JSON.stringify(value)),row.name).toBeLessThanOrEqual(policy.maximumMismatchBytes);
  expect(value.root.length,row.name).toBeLessThanOrEqual(256);expect(value.unit.length,row.name).toBeLessThanOrEqual(256);
 }
 const byName=(name:string)=>describeCacheRecencyChangeV1(...(()=>{const row=corpus.describe.find(item=>item.name===name)!;return[row.root,row.expected,row.current??undefined] as never;})());
 expect(byName("a vanished unit is reported as missing").changed).toEqual({missing:true,bytes:false,recency:false,lockHeld:false});
 expect(byName("nothing changed except a lock").changed).toEqual({missing:false,bytes:false,recency:false,lockHeld:true});
 expect(byName("bytes and recency changed").expected.sourceStatus).toBe("captured");
 expect(byName("the captured source is kept while the report is small").current!.sourceStatus).toBe("not-retained");
 expect(byName("an oversized report drops the sources").expected.sourceStatus).toBe("not-retained");
});
