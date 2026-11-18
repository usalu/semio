/** 🌊️ Contract: the patch of the fact index after the owners is flat in the rows: the resident set it adds per row stays below a fixed number of bytes and does not grow with the rows per owner, and its time grows with the rows (a straight line, not the 1.4th power the patch step showed at 4774 owners); each size is measured in its own child process, so the peak of one run never hides in the peak of another.  */
import {expect,test} from "bun:test";
import {dirname,join} from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";

const here=dirname(fileURLToPath(import.meta.url)),generations=join(here,"../.."),url=(path:string)=>pathToFileURL(join(generations,path)).href;
const run=(owners:number,per:number)=>{
 const script=`
  import{mkdtempSync,rmSync}from"node:fs";import{tmpdir}from"node:os";import{join}from"node:path";
  const{createRowLogV1}=await import(${JSON.stringify(url("🗃️row-log/🟨️.mjs"))}),{emptyFactIndexViewV1,patchFactIndexBytesV1}=await import(${JSON.stringify(url("🌊️stream-patch/🟨️.mjs"))});
  const directory=mkdtempSync(join(tmpdir(),"flat-memory-")),log=createRowLogV1({path:join(directory,"rl","a.rows")}),upserts=new Map();
  for(let owner=0;owner<${owners};owner++){const builder=log.builder();for(let from=0;from<${per};from+=250)builder.add(Array.from({length:Math.min(250,${per}-from)},(_unused,at)=>({kind:"file",path:"/w/pkg/p"+owner+"/src/f"+(from+at)+".ts",fact:{size:from+at,mtimeMs:1.7e12+from+at,ino:owner*100000+from+at,sha256:String(owner*100000+from+at).padStart(64,"0")}})));upserts.set("o"+String(owner).padStart(4,"0"),builder.finish());}
  Bun.gc(true);await new Promise(resolve=>setTimeout(resolve,50));
  const before=process.memoryUsage().rss;let peak=before;const timer=setInterval(()=>{peak=Math.max(peak,process.memoryUsage().rss);},5);
  const started=performance.now(),out=await patchFactIndexBytesV1(emptyFactIndexViewV1(),{upserts},{maximumBytes:2**30}),seconds=(performance.now()-started)/1000;clearInterval(timer);peak=Math.max(peak,process.memoryUsage().rss);
  console.log(JSON.stringify({growth:(peak-before)/1048576,seconds,rows:${owners}*${per},bytes:out.length}));log.close();rmSync(directory,{recursive:true,force:true});
 `;
 const result=Bun.spawnSync([process.execPath,"-e",script]);
 return JSON.parse(result.stdout.toString().trim().split("\n").at(-1)!) as {growth:number;seconds:number;rows:number;bytes:number};
};

test("the patch adds a bounded number of bytes of resident set for each further row at two sizes, and takes time in proportion to the rows",()=>{
 const small=run(200,400),large=run(200,1600);
 expect(large.rows).toBe(small.rows*4);expect(large.bytes).toBeGreaterThan(small.bytes*3);
 expect((large.growth-small.growth)*1048576/(large.rows-small.rows),"bytes per further row").toBeLessThan(800);
 expect(large.growth).toBeLessThan(small.growth*4);
 expect(large.seconds/small.seconds,"time ratio for four times the rows").toBeLessThan(6);
},240000);
