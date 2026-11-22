/** 📐 Isolate which physical member or drafted stroke changes the chalkboard elevation. */
import {readFileSync} from 'node:fs';
import {catalogEntries} from './volumes-lib.ts';
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
import {measureView,worldSolids,strokesOf,drawnFor} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts';
const e=catalogEntries().find(p=>p.slug==='chalkboard')!,a=JSON.parse(readFileSync(e.path,'utf8')).asset;
for(const k of [50,100]) {
 const state=evaluateDrawing(a,{kind:'pose',pose:a.presentation.defaultPose},{kind:'scale',scale:{denominator:k}});
 const full=worldSolids(a,state.bones,k).filter(p=>p.texture!==true);
 const ds=drawnFor(a,state,'elevation',k),bb=points=>['x','y'].map(key=>{const f=points.map(q=>q[key]);return [Math.min(...f),Math.max(...f)].map(v=>+v.toFixed(3))})
 for(const w of full.filter(w=>w.slender)){
  const bounds=d=>bb(strokesOf('elevation',w,d).flat()),native=JSON.stringify(bounds([])),ink=JSON.stringify(bounds(ds))
  const candidates=ds.map((q,i)=>[q.bone,q.material||'-',i,JSON.stringify(bounds([q]))]).filter(t=>t[3]!==native)
  console.log(JSON.stringify([w.id,w.material,native,ink,candidates]))
 }
}
