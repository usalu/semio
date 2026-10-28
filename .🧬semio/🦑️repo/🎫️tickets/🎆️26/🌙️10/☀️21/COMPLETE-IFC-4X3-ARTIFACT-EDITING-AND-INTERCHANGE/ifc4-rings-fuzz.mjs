/** 🧪 Deterministic IFC ring differential fuzz; no production writes. */
import fs from 'node:fs/promises';
import path from 'node:path';
import {stripTypeScriptTypes,createRequire} from 'node:module';
import {pathToFileURL} from 'node:url';
export async function run(root,ticket){
 const owner=path.join(root,'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🕳️rings'), output=path.join(ticket,'🗑️generated/rings-fuzz');
 await fs.mkdir(output,{recursive:true});
 const src=await fs.readFile(path.join(owner,'🟦️.ts'),'utf8'), compiled=path.join(output,'rings.mjs');
 await fs.writeFile(compiled,stripTypeScriptTypes(src));
 const {triangulateRings}=await import(pathToFileURL(compiled).href+'?v='+Date.now()),require=createRequire(path.join(root,'package.json')),{ShapeUtils,Vector2}=require('three');
 let seed=0x5e4d10;const random=()=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed/4294967296;};
 const failures=[];let cases=0,maxWork=0;
 const area=(p,t)=>t.reduce((s,[a,b,c])=>s+Math.abs((p[b][0]-p[a][0])*(p[c][1]-p[a][1])-(p[b][1]-p[a][1])*(p[c][0]-p[a][0]))/2,0);
 const signed=r=>r.reduce((s,a,i)=>{const b=r[(i+1)%r.length];return s+a[0]*b[1]-b[0]*a[1];},0)/2;
 for(let ncase=0;ncase<80;ncase++){
  const n=5+Math.floor(random()*12),outer=[];for(let i=0;i<n;i++){const t=2*Math.PI*i/n,rad=7+random()*3;outer.push([rad*Math.cos(t),rad*Math.sin(t)]);}
  const holes=[],count=1+ncase%4;for(let h=0;h<count;h++){const x=-3+h*2,y=-.8+random()*.4,w=.6+random()*.4,ht=.8+random()*.4;holes.push([[x,y],[x+w,y],[x+w,y+ht],[x,y+ht]]);}
  const neutral=[outer,...holes],expect=Math.abs(signed(outer))-holes.reduce((s,r)=>s+Math.abs(signed(r)),0);
  for(const plane of [0,1,2,3])for(const scale of [1,1e-90,1e90])for(const reverse of [false,true]){
   cases++;const rr=neutral.map((r,i)=>(reverse&&i%2===0?[...r].reverse():r).map(p=>[...p])),flat=rr.flat(),indices=[];let off=0;for(const r of rr){indices.push(r.map((_,i)=>off+i));off+=r.length;}
   const vertices=flat.map(([x,y])=>{let p=plane===0?[x,y,0]:plane===1?[0,x,y]:plane===2?[y,0,x]:[x/Math.SQRT2-y/Math.sqrt(6),x/Math.SQRT2+y/Math.sqrt(6),2*y/Math.sqrt(6)];return p.map(v=>v*scale);});
   let work=0;try{
    const tr=triangulateRings(vertices,indices,()=>{if(++work>2000000)throw Error('work-budget');});maxWork=Math.max(maxWork,work);
    const reference=ShapeUtils.triangulateShape(rr[0].map(p=>new Vector2(...p)),rr.slice(1).map(r=>r.map(p=>new Vector2(...p))));
    const ownArea=area(flat,tr),oracleArea=area(flat,reference);
    if(Math.abs(ownArea-expect)>1e-8||Math.abs(oracleArea-expect)>1e-8)throw Error('coverage '+ownArea+' '+oracleArea+' expected '+expect);
    if(tr.length!==flat.length+2*holes.length-2)throw Error('triangle-count '+tr.length);
   }catch(error){failures.push({ncase,plane,scale,reverse,error:String(error),work,rings:neutral});}
  }
 }
 return {schema:'ifc-rings-fuzz-v1',seed:'0x5e4d10',cases,maxWork,failures};
}
