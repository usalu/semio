import {test} from "bun:test";
import {StrokeOutlineJob} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/◻️2d/🛤️path/🖊️stroke/🟦️.ts";
import {CoverageJob} from "@semio-tech/framework-pixels-ts/coverage";
import {RetainedWorkLedger} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/◻️2d/../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
const L=(q:number)=>new RetainedWorkLedger({maximumItems:q,maximumCopyBytes:2**30,maximumCapacityBytes:2**30,maximumReleaseBytes:2**30,maximumDepth:64});
test("probe",()=>{
 const base={transform:[1,0,0,1,0,0],tolerance:.0001,style:{width:.8,cap:"butt",join:"miter",miterLimit:4,dash:[],dashOffset:.35}} as any;
 const paths=[[{points:[[3,3]],closed:false}],[{points:[[3,3],[3,3]],closed:false}],[{points:[[1,2],[5,2],[1,2]],closed:false}]];
 for(const [i,contours] of paths.entries()){
  const job=new StrokeOutlineJob({...base,contours} as any);let d=false;for(let t=0;t<100000&&!d;t++)d=job.advance(4096,L(4096)).done;
  const out=job.result();console.log("[DEBUG] path",i,"len",out.length);
  const cov=new CoverageJob({width:6,height:6,contours:out,transform:base.transform,rule:"nonzero"});
  let cd=false;for(let t=0;t<100000&&!cd;t++)cd=cov.advance(4096,L(4096)).done;
  const moved=cov.intoRetirement();let n=0;for(;n<2000&&!moved.job.terminalIsEmpty();n++)moved.job.advance(1,L(1));
  console.log("[DEBUG] cov close turns",n,moved.job.terminalIsEmpty(),moved.output===null);
 }
 for(const patch of [{width:-1},{cap:"triangle"},{join:"arcs"},{miterLimit:0},{dash:[-1,2]},{dashOffset:NaN}]){
  try{new StrokeOutlineJob({...base,contours:paths[2],style:{...base.style,...patch}} as any);console.log("[DEBUG] ctor no throw",JSON.stringify(patch));}catch(e){}
 }
});
