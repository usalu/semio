import {readFileSync} from 'node:fs';
import {meshes} from './volumes-base.ts';
import {Vector3, Triangle} from 'three';
import {analyticRules} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts';
const {asset}=JSON.parse(readFileSync(process.argv[2], 'utf8'));
const steel=asset.materials.find(m=>m.id==='steel-slides');
const weights=new Map<string,{kg:number,comY:number,area:number}>();
const p=new Vector3(),q=new Vector3(),r=new Vector3(),cross=new Vector3();
for(const b of asset.volumes['50'].body.filter(b=>b.id.startsWith('lift-'))){
  const copies=b.repeat?.count??1;
  const part={...b};delete part.repeat;
  const [mesh]=meshes(asset,[part],1.6);
  let vol=0,firstY=0,area=0;
  for(let j=0;j<mesh.indices.length;j+=3){
    p.fromArray(mesh.world,3*mesh.indices[j]);q.fromArray(mesh.world,3*mesh.indices[j+1]);r.fromArray(mesh.world,3*mesh.indices[j+2]);
    const dv=p.dot(cross.crossVectors(q,r))/6;vol+=dv;firstY+=dv*(p.y+q.y+r.y)/4;area+=new Triangle(p,q,r).getArea();
  }
  const kg=Math.abs(vol)*copies*steel.densityKgM3;
  const f=weights.get(b.bone)??{kg:0,comY:0,area:0};
  f.kg+=kg;f.comY+=kg*firstY/vol;f.area+=copies*area;weights.set(b.bone,f);
}
for(const [k,v] of weights) console.log(k, {kg:v.kg,comY:v.comY/v.kg,area:v.area});
console.log('analytic',JSON.stringify(analyticRules(asset).map(f=>[f.rule,f.path,f.message.en])));
