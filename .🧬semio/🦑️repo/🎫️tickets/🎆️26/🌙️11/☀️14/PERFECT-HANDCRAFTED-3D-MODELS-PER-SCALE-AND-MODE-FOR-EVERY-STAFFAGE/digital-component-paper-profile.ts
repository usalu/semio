/** 📐 Isolate which physical member or drafted stroke changes the chalkboard elevation. */
import {readFileSync} from 'node:fs';
import {catalogEntries} from './volumes-lib.ts';
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
import {measureView,worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts';
const read=(f:string)=>JSON.parse(readFileSync(f,'utf8')).asset;
const a=read(process.argv[2]!), slug=a.id;
const variants=(function*(){
 if(slug==='solar-thermal-collector'){
 const drawings=a.representations;
 const coarse=drawings[500].geometry.elevation;const fine=drawings[100].geometry.elevation;
 const stand=fine.filter((p:any)=>p.bone==='stand');const panel=fine.filter((p:any)=>p.bone!=='stand');
 for(const [label,ds] of [['stock-shell',[...stand,...coarse.filter((p:any)=>p.bone==='housing')]],['stock-sides',[...stand,...panel.filter((p:any)=>['tray-side-wall','glass'].includes(p.id))]],['stock-cover',[...stand,...panel.filter((p:any)=>p.id==='tray-side-wall')]],['paper-stock',[...coarse.filter((p:any)=>p.bone==='stand'),...panel.filter((p:any)=>['tray-side-wall','glass'].includes(p.id))]],['stock-all',[...stand,...panel]]])yield [500,'elevation',label,ds];
 }else{
 for(const k of (process.argv[3]?[Number(process.argv[3])]:slug==='glass-balustrade'?[200]:[500]))for(const view of (process.argv[4]?[process.argv[4]]:slug==='glass-balustrade'?['elevation']:['plan','elevation'])){
  const fine=a.representations[100].geometry[view];
  for(const base of [50,100,200,k])if(a.representations[base].geometry)yield[k,view,'base'+base,a.representations[base].geometry[view]];
  if(slug==='sliding-door'){
   for(const idx of [0,1])yield[k,view,'working'+idx,fine.filter((p:any)=>!p.id.includes('reflection')&&(!p.id.includes('hanger')||idx===1))];
  }
 }
 }
})();
for(const [k,view,label,shapes] of variants) {
 const b=structuredClone(a);b.representations[k].geometry[view]=shapes;
 const state=evaluateDrawing(b,{kind:'pose',pose:b.presentation.defaultPose},{kind:'scale',scale:{denominator:k}});
 const full=worldSolids(b,state.bones,k).filter(p=>p.texture!==true);
 const r=measureView(b,state,full,view,k,160,.012);
 console.log('[DEBUG]',k,view,label,r.iou,r.p95M);Bun.gc(true);
}
