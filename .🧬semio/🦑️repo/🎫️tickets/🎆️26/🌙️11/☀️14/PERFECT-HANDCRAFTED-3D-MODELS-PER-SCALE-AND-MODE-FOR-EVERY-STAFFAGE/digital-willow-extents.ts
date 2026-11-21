/** 🌳 Compare the outer extremities of willow heat fields with their native surface meshes. */
import {readFileSync} from 'node:fs';
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts';
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,pose=evaluateDrawing(a,{kind:'pose',pose:a.presentation.defaultPose});
const body=worldSolids(a,pose.bones,50),heat=worldSolids(a,pose.bones,50,'thermal',body);
for(const [g,solids] of Object.entries({body,heat})){
 const b=[Infinity,Infinity,Infinity,-Infinity,-Infinity,-Infinity],ids=Array(6).fill('');
 for(const s of solids)for(let j=0;j<s.positions.length;j++){
  const i=j%3,p=s.positions[j];
  if(p<b[i]){b[i]=p;ids[i]=s.id;}
  if(p>b[i+3]){b[i+3]=p;ids[i+3]=s.id;}
 }
 console.log(g,b,ids);
}
