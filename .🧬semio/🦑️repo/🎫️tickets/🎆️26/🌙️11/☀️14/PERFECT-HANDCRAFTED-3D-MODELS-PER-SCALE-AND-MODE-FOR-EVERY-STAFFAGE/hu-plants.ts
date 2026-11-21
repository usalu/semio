/** 👣 Reconciles named weight-bearing poses with the actual rig and its declared support plane. */
import {writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {entryOf,GENERATED} from './volumes-lib.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const out=[]
for(const [slug,poses] of Object.entries({'jogger':['midstance-l','midstance-r','stretch-hamstring'],'parent-carrying-toddler':['walk-1','walk-5'],'parent-with-stroller':['checking-child'],'pregnant-person':['waddle-contact']})){
 const {asset}=entryOf('humans/'+slug)
 for(const id of poses){
  const p=asset.poses.find(x=>x.id===id),q={kind:'pose' as const,pose:id}
  const y=(k:string)=>evaluateDrawing(asset,q).anchors[k].at.y
  const changes:Record<string,number>={}
  function solve(bones:string[],first:string,last:string,gains=bones.map(()=>1)){
   let n=0
   while(Math.abs(y(first)-y(last))>.00001&&n++<10){
    const f=y(first)-y(last)
    for(let i=0;i<bones.length;i++)p.bones[bones[i]].angleDeg+=.01*gains[i]
    const m=(y(first)-y(last)-f)/.01
    const d=Math.max(-3,Math.min(3,-f/m))
    for(let i=0;i<bones.length;i++)p.bones[bones[i]].angleDeg+=(d-.01)*gains[i]
   }
   for(const b of bones)changes[b]=+p.bones[b].angleDeg.toFixed(5)
  }
  if(slug==='parent-with-stroller')changes['thigh-l']=p.bones['thigh-l'].angleDeg=10.5
  if(slug==='pregnant-person')solve(['shank-r','foot-r'],'heel-l','ball-r',[1,-1])
  else{
   for(const side of ['l','r'])if(p.contacts.includes('heel-'+side))solve([slug==='jogger'&&id.startsWith('midstance')?'shank-'+side:'foot-'+side],'heel-'+side,'contact-'+side)
   if(slug==='parent-with-stroller'){
    for(const side of ['l','r'])solve(['shank-'+side,'foot-'+side],'heel-'+side,'wheel-rear',[1,-1])
   }
  }
  const heights=p.contacts.map(k=>[k,y(k)])
  console.log(slug,id,changes,heights)
  if(heights.some(([_,h])=>Math.abs(h)>.001))throw Error(slug+' '+id)
  out.push({slug,id,changes})
 }
}
writeFileSync(join(GENERATED,'hu-review','plants.json'),JSON.stringify(out))
