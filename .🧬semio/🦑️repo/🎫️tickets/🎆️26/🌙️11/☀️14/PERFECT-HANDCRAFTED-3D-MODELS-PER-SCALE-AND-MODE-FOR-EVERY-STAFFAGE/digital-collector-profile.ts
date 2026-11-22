/** 📐 Isolate which physical member or drafted stroke changes the chalkboard elevation. */
import {readFileSync} from 'node:fs';
import {catalogEntries} from './volumes-lib.ts';
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
import {measureView,worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts';
const file=process.argv[2]!, a=JSON.parse(readFileSync(file,'utf8')).asset;
for(const dk of [50,100,200,500]){
 const b=structuredClone(a),k=500;
 b.representations[k].geometry.elevation=structuredClone(a.representations[dk].geometry.elevation);
 const state=evaluateDrawing(b,{kind:'pose',pose:b.presentation.defaultPose},{kind:'scale',scale:{denominator:k}});
 const full=worldSolids(b,state.bones,k).filter(p=>p.texture!==true);
 for(const name of ['all','none','struct','panel']) {
 const parts=full.map(p=>((name==='none'||name==='panel'&&p.bone==='stand'||name==='struct'&&p.bone!=='stand')?{...p,slender:false}:p));
 const r=measureView(b,state,parts,'elevation',k,96,.024);
 console.log('[DEBUG]',dk,name,r.iou,r.p95M);Bun.gc(true);
 }
}
