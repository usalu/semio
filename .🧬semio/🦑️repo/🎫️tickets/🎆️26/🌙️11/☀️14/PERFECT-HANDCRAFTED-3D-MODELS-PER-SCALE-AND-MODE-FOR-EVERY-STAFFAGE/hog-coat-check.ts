/** 🦔 Measure exposed spine tips above the mantle on the posed production triangles. */
import {readFileSync} from 'node:fs'
import {Ray,Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./hog-coat-case.json',import.meta.url),'utf8')),failure=[]
const vec=p=>new Vector3(p.x,p.y,p.z)
for(const [i,k] of f.scales.entries())for(const pose of f.poses){
 const vol=a.volumes[k],bones=evaluateDrawing(a,{kind:'pose',pose}).bones
 const groups=vol.body.filter(m=>m.id.startsWith('fascicle-'))
 const count=Object.fromEntries(f.mounts.map(m=>[m,0])),rings=vol.structure.map(m=>m.id)
 const problem=s=>failure.push(k+'/'+pose+'/'+s)
 if(groups.length<f.minFascicles[i])problem('spine count '+groups.length)
 if(f.structuralRings.some(s=>!rings.includes(s)))problem('orbicularis')
 const skins=worldSolids({...a,volumes:{...a.volumes,[k]:{...vol,body:vol.body.filter(s=>!s.id.startsWith('fascicle-')&&!s.id.startsWith('sp-'))}}},bones,k)
 const faces=skins.map(s=>{const v=[];for(let j=0;j<s.positions.length;j+=3)v.push(new Vector3().fromArray(s.positions,j));return {v,indices:s.indices}})
 let visible=0,overlong=0,misses=0
 for(const g of groups){
  const fr=bones[g.bone].frame,w=p=>vec(fr.o).addScaledVector(vec(fr.u),p[0]).addScaledVector(vec(fr.v),p[1]).addScaledVector(vec(fr.w),p[2])
  const origin=w(g.vertices[4]),tip=w(g.vertices[13]),d=tip.clone().sub(origin).normalize()
  const ray=new Ray(tip.clone().addScaledVector(d,.15),d.clone().negate()),p=new Vector3();let nearest=Infinity
  for(const {v,indices} of faces)for(let j=0;j<indices.length;j+=3){
   if(ray.intersectTriangle(v[indices[j]],v[indices[j+1]],v[indices[j+2]],false,p))nearest=Math.min(nearest,p.distanceTo(ray.origin))
  }
  const clearance=nearest-.15
  if(!Number.isFinite(nearest)){misses++;console.log('miss',g.id);continue}
  if(clearance>f.maximumTipClearanceM){overlong++;continue}
  if(clearance>f.minimumTipClearanceM){visible++;count[g.bone]++}
  const region=/^coat-/.test(g.bone)?'spines-skirt':'spines-dome',layer=region==='spines-skirt'?'skirt-spines':'dome-spines'
  if(!vol.thermal.some(h=>h.of===g.id&&h.region===region&&h.layer===layer))problem('thermal '+g.id)
 }
 if(visible<f.minimumExposedTips[pose]||!count.trunk||!count.chest)problem('external spine coverage')
 if(overlong||misses)problem('unsupported '+overlong+'/'+misses)
 console.log(JSON.stringify({scale:k,pose,groups:groups.length,visible,count,overlong,misses}))
}
if(failure.length){console.error(failure);process.exitCode=1}
