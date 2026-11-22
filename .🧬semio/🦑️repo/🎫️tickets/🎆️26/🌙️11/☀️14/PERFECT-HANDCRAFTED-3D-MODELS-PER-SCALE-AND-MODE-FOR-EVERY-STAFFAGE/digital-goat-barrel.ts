/** 🐐 Give the doe a continuous topline and abdominal barrel from the sacrum through the brisket. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Quaternion,Matrix4,Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,f=JSON.parse(readFileSync(new URL('./digital-goat-barrel.json',import.meta.url),'utf8')),r=v=>+v.toFixed(8),V=p=>new Vector3(p.x,p.y,p.z)
const frames=evaluateDrawing(a,{kind:'pose',pose:'standing-alert'}).bones,old=new Set(['ribcage','abdomen','loin','withers','croup','chest','hindbody'])
for(const [scale,vol] of Object.entries(a.volumes)){
 if(['200','500'].includes(scale)){
  const joints=/^(thigh|shank|metatarsus|scapula|humerus|forearm|metacarpus)-[fh][nf]$/
  const merged=new Set(vol.body.filter(s=>joints.test(s.bone)).map(s=>s.id)),fine=a.volumes['50']
  vol.body=vol.body.filter(s=>!merged.has(s.id));vol.thermal=vol.thermal.filter(s=>!merged.has(s.of))
  for(const part of fine.body.filter(s=>joints.test(s.bone))){
   const solid=structuredClone(part)
   if(solid.sections){
    const ss=solid.sections,n=ss.length
    const keep=new Set([0,Math.round((n-1)*.35),Math.round((n-1)*.65),n-1])
    if(scale==='200')keep.add(Math.round((n-1)*.18))
    solid.sections=ss.filter((s,i)=>keep.has(i))
   }
   vol.body.push(solid)
   for(const heat of fine.thermal.filter(s=>s.of===part.id))vol.thermal.push(structuredClone(heat))
  }
 }
 const surface=vol.thermal.find(s=>s.of&&old.has(s.of)),stock=vol.body.find(s=>old.has(s.id))
 vol.body=vol.body.filter(s=>!old.has(s.id));vol.thermal=vol.thermal.filter(s=>!old.has(s.of))
 for(const [bone,id,low,high] of [['pelvis','pelvic-barrel',0,f.jointsAtStation[0]],['lumbar','abdominal-barrel',...f.jointsAtStation],['thorax','costal-barrel',f.jointsAtStation[1],f.stations.length-1]]){
  const fr=frames[bone].frame,u=V(fr.u),v=V(fr.v),w=V(fr.w),o=V(fr.o),base=new Matrix4().makeBasis(u,v,w)
  const q=new Quaternion().setFromRotationMatrix(base).invert().multiply(new Quaternion().setFromAxisAngle(new Vector3(0,1,0),Math.PI/2)).normalize().toArray().map(r)
  const sections=f.scaleStations[scale].filter(i=>i>=low&&i<=high).map(i=>{
   const [x,bottom,top,radius,exponent]=f.stations[i],p=new Vector3(x,(top+bottom)/2,0).sub(o)
   return {at:[p.dot(u),p.dot(v),p.dot(w)].map(r),rx:radius,ry:r((top-bottom)/2),exponent,rotation:q}
  })
  vol.body.push({id,bone,kind:'loft',role:'body',material:stock.material,tone:stock.tone,sections,capStart:true,capEnd:true})
  vol.thermal.push({...surface,id:'th-'+id,of:id})
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc))
