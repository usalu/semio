/** 📐 Isolate which physical member or drafted stroke changes the chalkboard elevation. */
import {readFileSync} from 'node:fs';
import {catalogEntries} from './volumes-lib.ts';
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
import {measureView,worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts';
const e=catalogEntries().find(p=>p.slug==='chalkboard')!,a=JSON.parse(readFileSync(e.path,'utf8')).asset;
for(const k of [50,100]) {
 const state=evaluateDrawing(a,{kind:'pose',pose:a.presentation.defaultPose},{kind:'scale',scale:{denominator:k}});
 const full=worldSolids(a,state.bones,k).filter(p=>p.texture!==true);
 for(const ids of [[],['wall-cleat'],['board-cleat'],['ruler'],['wall-cleat','board-cleat','ruler']]){
  const solids=full.map(p=>ids.includes(p.id)?{...p,slender:false}:p);
  const r=measureView(a,state,solids,'elevation',k,96,.024);
  console.log('[DEBUG]',k,ids,r.iou,r.p95M);Bun.gc(true);
 }
}
