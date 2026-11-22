/** 🦮 Check corneal relief, orbital containment and open nostrils on posed production meshes. */
import {readFileSync} from 'node:fs'
import {Ray,Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {worldSolids} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
import {exposedArea} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./lab-face-case.json',import.meta.url),'utf8')),fails=new Set()
const verify=(b,s)=>{if(!b)fails.add(s)}
const vec=p=>new Vector3(p.x,p.y,p.z),v3=(m,i)=>new Vector3().fromArray(m.positions,3*m.indices[i])
for(const k of f.scaleDenominators)for(const {id:pose} of a.poses){
 const bones=evaluateDrawing(a,{kind:'pose',pose}).bones,fr=bones.head.frame,vol=a.volumes[k]
 const parts=worldSolids(a,bones,k).filter(s=>['head','jaw','neck','ear-n','ear-f'].includes(s.bone)),byId=new Map(parts.map(s=>[s.id,s]))
 const world=p=>vec(fr.o).addScaledVector(vec(fr.u),p[0]).addScaledVector(vec(fr.v),p[1]).addScaledVector(vec(fr.w),p[2])
 const rayHit=(mesh,p,d)=>{if(!mesh)return Infinity;const ray=new Ray(world(p),d),hit=new Vector3();let near=Infinity
  for(let i=0;i<mesh.indices.length;i+=3)if(ray.intersectTriangle(v3(mesh,i),v3(mesh,i+1),v3(mesh,i+2),false,hit))near=Math.min(near,hit.distanceTo(ray.origin))
  return near}
 const exposed=id=>{const m=byId.get(id);return m?exposedArea([m],parts,[],()=>false,.00015,0):0}
 const noseArea=exposed('nose-leather'),eyes=[],depths=[]
 const supports=pose!=='sitting-waiting'?[]:worldSolids(a,bones,k,'structure').filter(m=>/^(skull|mandible|zygomatic-[nf])$/.test(m.id))
 const worst={}
 for(const support of supports){
  const local=bones[support.bone].frame,frameO=vec(local.o),frameU=vec(local.u),frameV=vec(local.v),frameW=vec(local.w)
  const cover=support.bone==='jaw'?[byId.get('jaw')]:[byId.get('head'),byId.get('nose-leather'),byId.get('flew-n'),byId.get('flew-f')]
  let excess=0
  for(let i=0;i<support.positions.length;i+=3){
   const boneVertex=new Vector3().fromArray(support.positions,i),disp=boneVertex.clone().sub(frameO),z=disp.dot(frameW),s=z<0?-1:1
   const outer=frameO.clone().addScaledVector(frameU,disp.dot(frameU)).addScaledVector(frameV,disp.dot(frameV)).addScaledVector(frameW,s*.19)
   const ray=new Ray(outer,frameW.clone().multiplyScalar(-s)),hit=new Vector3();let dist=Infinity
   for(const skin of cover.filter(Boolean))for(let j=0;j<skin.indices.length;j+=3)if(ray.intersectTriangle(v3(skin,j),v3(skin,j+1),v3(skin,j+2),false,hit))dist=Math.min(dist,hit.distanceTo(outer))
   excess=Math.max(excess,dist-outer.distanceTo(boneVertex))
  }
  worst[support.id]=excess;verify(excess<.002,k+' '+pose+' exposed bone '+support.id+' '+excess)
 }
 verify(noseArea>=f.minExposedPlanumM2,k+' exposed nose '+noseArea)
 for(const [name,sign] of [['n',-1],['f',1]]){
  const tag=k+' '+name,eye=exposed('eye-'+name),pupil=exposed('pupil-'+name)
  eyes.push(eye);verify(eye>=f.minExposedCorneaM2,tag+' exposed eye '+eye);verify(pupil>.000014,tag+' pupil '+pupil)
  verify(byId.has('lid-upper-'+name)&&byId.has('lid-lower-'+name)&&byId.has('flew-'+name),tag+' lids/flew')
  const axis=vec(fr.u).negate(),front=[.27,.006,sign*.011]
  const channel=rayHit(byId.get('nose-leather'),front,axis),seat=rayHit(byId.get('naris-'+name),front,axis),lip=rayHit(byId.get('nose-leather'),[.27,.006,0],axis)
  verify(channel>seat&&seat-lip>.003&&seat-lip<.014,tag+' open airway '+[channel,seat,lip])
  depths.push(seat-lip)
  verify(vol.thermal.some(q=>q.of==='eye-'+name&&q.region==='eyes')&&vol.thermal.some(q=>q.of==='nose-leather'&&q.region==='nose-skin'),tag+' thermal')
  verify(vol.structure.some(q=>q.id==='zygomatic-'+name&&q.member==='head'),tag+' bony orbit')
 }
 if(pose==='trot-phase-0'){
  const wet=exposed('tongue')
  verify(wet>.005,k+' panting tongue '+wet)
  verify(vol.thermal.some(q=>q.of==='tongue'&&q.region==='tongue'),k+' panting thermal reference')
  console.log(JSON.stringify({k,pose,tongueAreaMm2:Math.round(wet*1e6)}))
 }
 if(pose==='sitting-waiting')console.log(JSON.stringify({k,pose,eyeAreaMm2:eyes.map(v=>Math.round(v*1e6)),noseAreaMm2:Math.round(noseArea*1e6),narisMm:depths.map(v=>+(v*1000).toFixed(2)),boneExcessM:worst}))
}
console.log(JSON.stringify({failures:[...fails]}));if(fails.size)process.exitCode=1
