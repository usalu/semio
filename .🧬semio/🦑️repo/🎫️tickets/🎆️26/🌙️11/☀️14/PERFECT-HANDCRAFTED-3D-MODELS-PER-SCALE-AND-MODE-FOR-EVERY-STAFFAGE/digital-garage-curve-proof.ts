/** 📐 Isolate which physical member or drafted stroke changes the chalkboard elevation. */
import {readFileSync} from 'node:fs';
import {catalogEntries} from './volumes-lib.ts';
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
import {measureView,worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts';
const a=JSON.parse(readFileSync(process.argv[2]!,'utf8')).asset,k=200,v='plan';
const baseline=a.representations[k].geometry.plan;
const mid=a.representations[100].geometry.plan,fine=a.representations[50].geometry.plan,coarse=a.representations[500].geometry.plan;
const choices=new Map<string,any[]>([['as-authored',baseline],['whole-mid',mid],['whole-fine',fine],['whole-coarse',coarse]]);
const prefix=(p:any,keys:string[])=>keys.some(q=>p.id.startsWith(q));
const pull=(original:any[],source:any[],include:string[])=>[...original.filter(p=>!prefix(p,include)),...source.filter(p=>prefix(p,include))];
for(const keys of [['shaft','spring-'],['track-','brace-','hanger-'],['panel-','bottom-seal'],['hinge-','roller-'],['track-angle','bottom-seal','anchor-bracket','roller-'],['shaft','spring-','hinge-','roller-']]){
 for(const [key,source] of [['mid',mid],['fine',fine],['coarse',coarse]] as const)choices.set(key+' '+keys.join(','),pull(baseline,source,keys));
 choices.set('omit '+keys.join(','),baseline.filter((q:any)=>!prefix(q,keys)));
}
for(const [name,symbols] of choices){
 const b=structuredClone(a);b.representations[k].geometry[v]=symbols;
 const state=evaluateDrawing(b,{kind:'pose',pose:b.presentation.defaultPose},{kind:'scale',scale:{denominator:k}});
 const solids=worldSolids(b,state.bones,k).filter(p=>p.texture!==true);
 const result=measureView(b,state,solids,v,k,undefined,.030);
 console.log('[DEBUG]',name,result.iou,result.p95M);Bun.gc(true);
}
