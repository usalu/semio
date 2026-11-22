/** 🧤 Measure the human contact surfaces against the grips, pushrims and pedals of each occupied device. */
import {readFileSync} from 'node:fs'
import {strict as assert} from 'node:assert'
import {geometry,surface} from './digital-surface-distance.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {posedMesh} from './living-world-mesh.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,ride=a.id==='cyclist',chair=a.id==='wheelchair-user'
const fixture=JSON.parse(readFileSync(new URL('./digital-occupied-wheel-case.json',import.meta.url),'utf8')),limits=fixture.subjects[a.id],issues=[]
const cycle=a.clips[0],samples=ride?[0,.0625,.125,.25,.375,.5,.75,.875]:chair?[0,.08,.17,.4,.65,.89]:[0,.16,.33,.49,.65,.81,1.14]
const states=a.poses.map(p=>({id:p.id,at:p.id,grasp:limits.graspingPoses.includes(p.id),pedal:ride&&limits.pedallingPoses.includes(p.id)}))
for(const t of samples)states.push({id:cycle.id+'@'+t,at:{kind:'clip',clip:cycle.id,timeS:t},grasp:chair&&t>.1875?undefined:true,pedal:ride})
for(const pose of states){
 const m=posedMesh(a,pose.at),rows=[]
 for(const d of typeof pose.at==='string'?fixture.scales:[50,500]){const q=qualityOf(a,d),s=a.volumes[d].body,sol=part=>part.map(x=>geometry(q,m,x))
  for(const side of ['l','r']){
   const parts=s.filter(p=>p.bone==='hand-'+side),named=s.filter(p=>p.id===(chair?'hand-rim-':'grip-')+side),pedal=s.filter(p=>p.id==='pedal-'+side)
   const H=sol(parts.length?parts:s.filter(p=>p.bone==='forearm-'+side)),G=sol(named.length?named:s.filter(p=>p.id==='handlebar')),F=sol(s.filter(p=>p.bone==='foot-'+side||p.bone==='toe-'+side)),P=sol(pedal.length?pedal:s.filter(p=>p.bone==='crank-'+side))
   if(chair&&pose.id==='hands-on-rims'){
    const skin=id=>sol(s.filter(p=>p.id===id+'-'+side)),internal=id=>sol(a.volumes[d].structure.filter(p=>p.id===id+'-'+side))
    const wrist=surface(skin('glove-cuff'),skin('glove'));if(wrist===null||wrist>3)issues.push([d,side,'wrist',wrist])
    for(const digit of ['finger-1','finger-2','finger-3','finger-4','thumb']){
     const finger=skin(digit),leather=skin('glove-'+digit),bone=internal(digit==='thumb'?'thumb-bones':'phalanges-'+digit.slice(-1))
     const seam=surface(finger,leather),depth=surface(finger,bone)
     if(seam===null||seam>2||depth===null||depth>10)issues.push([d,side,digit,seam,depth])
    }
   }
   const hand=surface(H,G),foot=ride?surface(F,P):undefined,grasp=pose.grasp
   if(hand===null||grasp!==undefined&&(grasp?hand>limits.maximumGraspGapMm:hand<limits.minimumReleasedGapMm))issues.push([pose.id,d,side,'grip',hand])
   if(ride&&(foot===null||(pose.pedal?foot>limits.maximumPedalGapMm:foot<limits.minimumReleasedGapMm)))issues.push([pose.id,d,side,'pedal',foot])
   rows.push([d,side,hand,foot])
  }
 }
 if(chair)for(const [d,side,hand] of rows){
  const reference=rows.find(r=>r[0]===50&&r[1]===side)[2]
  if(Math.abs(hand-reference)>limits.maximumRecoveryDriftMm)issues.push([pose.id,d,side,'pushrim trajectory mismatch',hand,reference])
 }
 console.log(a.id,pose.id,JSON.stringify(rows))
}
assert.deepEqual(issues,[])
