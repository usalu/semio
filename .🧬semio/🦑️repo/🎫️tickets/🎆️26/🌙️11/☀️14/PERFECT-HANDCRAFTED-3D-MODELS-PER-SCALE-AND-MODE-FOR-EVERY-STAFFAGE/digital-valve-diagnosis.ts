/** 📐 Isolate which physical member or drafted stroke changes the chalkboard elevation. */
import {readFileSync} from 'node:fs';
import {catalogEntries} from './volumes-lib.ts';
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
import {measureView,worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts';
const a=JSON.parse(readFileSync(process.argv[2]!,'utf8')).asset,k=50;
const shapes=a.representations[k].geometry.front;
const src=a.volumes[k].body;
const physical=(id:string)=>src.find((p:any)=>p.id===id).sections.map((q:any)=>[q.at[2],Number((.086+q.at[0]).toFixed(6))]);
const coil=physical('capillary-coil'),rise=physical('capillary-riser'),lead=physical('capillary');
const poly=(base:any,points:number[][],closed:boolean,filled:boolean,pen?:number)=>({...Object.fromEntries(Object.entries(base).filter(([k])=>!['points','d','closed','filled','kind'].includes(k))),kind:'poly',points,closed,filled,...(pen?{penMm:pen}:{})});
for(const c of ['old','wire','envelope','outer','none'])for(const part of ['old','actual','actual-stem']){
 const b=structuredClone(a);const row=b.representations[k].geometry.front;
 const wire=row.find((s:any)=>s.id==='capillary-coil'),seg=row.find((s:any)=>s.id==='capillary');
 const pieces=row.filter((s:any)=>s.id!=='capillary-coil'&&s.id!=='capillary');
 if(c==='old')pieces.push(wire);
 if(c==='wire')pieces.push(poly(wire,coil,false,false));
 if(c==='envelope')pieces.push(poly(wire,[...coil.slice(0,13),...coil.slice(-13)],true,true));
 if(c==='outer')pieces.push(poly(wire,[...coil.slice(0,13),...coil.slice(-13)],true,false));
 if(part==='old')pieces.push(seg);else {
   pieces.push(poly({...seg,id:'capillary-riser'},rise,false,false),poly(seg,lead,false,false));
   if(part==='actual-stem'){
      const stem=src.find((p:any)=>p.id==='stem-boss');
      console.log('[DEBUG] stem',stem.origin,stem.axis);
   }
 }
 b.representations[k].geometry.front=pieces;
 const state=evaluateDrawing(b,{kind:'pose',pose:b.presentation.defaultPose},{kind:'scale',scale:{denominator:k}});
 const solids=worldSolids(b,state.bones,k).filter(p=>p.texture!==true);
 const r=measureView(b,state,solids,'front',k,undefined,.0075);
 console.log('[DEBUG]',c,part,r.iou,r.p95M);Bun.gc(true);
}
