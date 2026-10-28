import {expect,test} from "bun:test";
import {mkdtempSync,readdirSync,readFileSync,rmSync,statSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {tmpdir} from "node:os";
import {testPythonV1} from "../../../../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {freeSpaceOfRowV1,freeSpaceSyncV1} from "../../../../../../../../🆓️free-space/🟦️.ts";
import {leaseStoreFreeRatioV1} from "../../../../../../../../🔒️leases/🟦️.ts";
import {nativeFreeSpaceSyncV1} from "../../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";

const python=():string=>{const found=testPythonV1()??Bun.which("python3")??Bun.which("python");if(!found)throw Error("Independent Python oracle required");return found;};
const ask=(program:string,...args:string[]):unknown=>{const run=Bun.spawnSync([python(),"-I","-c",program,...args],{stdout:"pipe",stderr:"pipe"});if(run.exitCode!==0)throw Error(run.stderr.toString());return JSON.parse(run.stdout.toString());};
const scratch=():string=>mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR?resolve(process.env.SEMIO_TEST_ARTIFACT_DIR):tmpdir(),"free-"));
const gibibyte=1073741824;

test("a statfs row is judged exactly as the independent Python reading judges it, and an incoherent row is unknown",()=>{
 const program="import json,sys,math\nout=[]\nfor r in json.loads(sys.argv[1]):\n b,k,a=r['bsize'],r['blocks'],r['bavail']\n if not(b>=1) or not(a>=0) or not math.isfinite(a*b):\n  out.append(None);continue\n out.append([a*b,b*k if (k>0 and k>=a and math.isfinite(b*k)) else None])\nprint(json.dumps(out))";
 const independent=ask(program,JSON.stringify(fixture.rows.map(row=>row.row))) as ([number,number|null]|null)[];
 fixture.rows.forEach((row,index)=>{
  const actual=freeSpaceOfRowV1(row.row),shape=actual===undefined?null:[actual.availableBytes,actual.totalBytes??null];
  expect<unknown>(shape,row.name).toEqual(row.expected);expect<unknown>(independent[index],row.name).toEqual(row.expected);
 });
 expect(freeSpaceOfRowV1({bsize:4096,blocks:10,bavail:Number.NaN})).toBeUndefined();
 expect(freeSpaceOfRowV1({bsize:4096n,blocks:1000n,bavail:250n})).toEqual({availableBytes:1024000,totalBytes:4096000});
});

test("the free space of a real directory equals the independent Python statvfs reading, a missing path is unknown and the nearest existing ancestor answers on request",()=>{
 const base=scratch();
 try{
  const program="import json,os,shutil,sys\ns=os.statvfs(sys.argv[1]);u=shutil.disk_usage(sys.argv[1])\nprint(json.dumps([s.f_bavail*s.f_frsize,s.f_blocks*s.f_frsize,u.total,u.free]))";
  const[free,total,usageTotal,usageFree]=ask(program,base) as number[],actual=freeSpaceSyncV1(base)!;
  expect(actual).toBeDefined();expect(Math.abs(actual.totalBytes!-total)/total).toBeLessThan(0.01);expect(Math.abs(actual.availableBytes-free)).toBeLessThan(2*gibibyte);
  expect(Math.abs(actual.totalBytes!-usageTotal)/usageTotal).toBeLessThan(0.01);expect(Math.abs(actual.availableBytes-usageFree)).toBeLessThan(2*gibibyte);
  const missing=join(base,"a","b","c");
  expect(freeSpaceSyncV1(missing)).toBeUndefined();
  expect(Math.abs(freeSpaceSyncV1(missing,{nearest:true})!.totalBytes!-total)/total).toBeLessThan(0.01);
  expect(nativeFreeSpaceSyncV1(missing)).toBeUndefined();expect(Math.abs(nativeFreeSpaceSyncV1(missing,{nearest:true})!.totalBytes!-total)/total).toBeLessThan(0.01);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a reader that throws, answers a negative or non-finite number or an incoherent row makes the free space unknown, never a number",()=>{
 const base=scratch();
 try{
  const boom=():never=>{throw Object.assign(new Error("denied"),{code:"EACCES"});};
  expect(freeSpaceSyncV1(base,{read:boom})).toBeUndefined();
  expect(freeSpaceSyncV1(base,{read:boom,nearest:true})).toBeUndefined();
  expect(freeSpaceSyncV1(base,{read:()=>({availableBytes:-1,totalBytes:undefined})})).toBeUndefined();
  expect(freeSpaceSyncV1(base,{read:()=>({availableBytes:Number.NaN,totalBytes:undefined})})).toBeUndefined();
  expect(freeSpaceSyncV1(base,{read:()=>({bsize:0,blocks:10,bavail:5})})).toBeUndefined();
  expect(freeSpaceSyncV1(base,{read:()=>({bsize:4096,blocks:10,bavail:5})})).toEqual({availableBytes:20480,totalBytes:40960});
  expect(freeSpaceSyncV1(base,{read:()=>({availableBytes:7,totalBytes:undefined})})).toEqual({availableBytes:7,totalBytes:undefined});
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a Windows host path asks the volume for its free bytes and reports no volume size; a failing or nonsensical answer is unknown",()=>{
 const base=scratch();
 try{
  expect(nativeFreeSpaceSyncV1(base,{platform:"win32",availableBytes:()=>123456})).toEqual({availableBytes:123456,totalBytes:undefined});
  expect(nativeFreeSpaceSyncV1(base,{platform:"win32",availableBytes:()=>{throw Error("Host free space could not be read (Win32 error 3)");}})).toBeUndefined();
  expect(nativeFreeSpaceSyncV1(base,{platform:"win32",availableBytes:()=>Number.NaN})).toBeUndefined();
  expect(nativeFreeSpaceSyncV1(base,{platform:"win32",availableBytes:()=>-5})).toBeUndefined();
  expect(nativeFreeSpaceSyncV1(join(base,"missing"),{platform:"win32",nearest:true,availableBytes:path=>path===base?99:Number.NaN})).toEqual({availableBytes:99,totalBytes:undefined});
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the lease store free fraction is the available share of the volume, equal to the independent reading, and unknown for a missing store",()=>{
 const base=scratch();
 try{
  const program="import os,sys\ns=os.statvfs(sys.argv[1])\nprint(s.f_bavail/s.f_blocks)";
  expect(Math.abs(leaseStoreFreeRatioV1(base)!-(ask(program,base) as number))).toBeLessThan(0.05);
  expect(leaseStoreFreeRatioV1(join(base,"missing"))).toBeUndefined();
 }finally{rmSync(base,{recursive:true,force:true});}
});

const repository=(():string=>{for(let current=import.meta.dir;;current=dirname(current)){try{if(statSync(join(current,"🧰️framework")).isDirectory())return current;}catch{}if(dirname(current)===current)throw Error("Repository root required");}})();
const scanned=(root:string):string[]=>{
 const found:string[]=[],walk=(directory:string):void=>{
  for(const entry of readdirSync(directory,{withFileTypes:true})){
   const path=join(directory,entry.name);
   if(entry.isDirectory()||entry.isSymbolicLink()&&statSync(path).isDirectory()){if(!["node_modules","target",".cache","🧪️tests"].includes(entry.name))walk(path);continue;}
   if(/\.(?:ts|mjs|js)$/u.test(entry.name))found.push(path);
  }
 };
 walk(join(repository,root));return found;
};

test("every production call of the host statfs is either routed through the free-space question or kept for a named reason, agreeing with an independent Python walk",()=>{
 const call=/(?<![A-Za-z0-9_.])statfsSync\(/u,question=/(?<![A-Za-z0-9_.])(?:nativeFreeSpaceSyncV1|freeSpaceSyncV1)\(/u,raw:string[]=[],asking:string[]=[];
 for(const root of fixture.sites.roots)for(const path of scanned(root)){const text=readFileSync(path,"utf8"),name=path.slice(repository.length+1);if(call.test(text))raw.push(name);if(question.test(text))asking.push(name);}
 const kept=fixture.sites.kept.map(row=>row.path).sort();
 expect(raw.sort()).toEqual(kept);
 for(const path of fixture.sites.routed)expect(asking,path).toContain(path);
 const program="import json,os,re,sys\ncall=re.compile(r'(?<![A-Za-z0-9_.])statfsSync\\(')\nout=[]\nfor root in json.loads(sys.argv[2]):\n for base,dirs,names in os.walk(os.path.join(sys.argv[1],root),followlinks=True):\n  dirs[:]=[d for d in dirs if d not in ('node_modules','target','.cache','🧪️tests')]\n  for n in names:\n   if n.endswith(('.ts','.mjs','.js')):\n    p=os.path.join(base,n)\n    if call.search(open(p,encoding='utf8').read()): out.append(os.path.relpath(p,sys.argv[1]))\nprint(json.dumps(sorted(out)))";
 expect(ask(program,repository,JSON.stringify(fixture.sites.roots))).toEqual(kept);
 expect(new Set(fixture.sites.kept.map(row=>row.reason)).size).toBeGreaterThan(1);
 for(const path of fixture.sites.routed.filter(path=>!kept.includes(path)))expect(readFileSync(join(repository,path),"utf8"),path).not.toMatch(call);
},120000);
