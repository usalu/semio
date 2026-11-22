/** 👶 Measure support and clearance between the posed passenger, seat, bumper and the parent's hands. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {posedMesh} from './living-world-mesh.ts'
import {geometry,surface,intersects} from './digital-surface-distance.ts'
const limits=JSON.parse(readFileSync(new URL('./digital-occupied-wheel-case.json',import.meta.url),'utf8')).subjects['parent-with-stroller']
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,issues=[],states=a.poses.map(p=>[p.id,p.id])
for(const [clip,times] of [['push-walk',[.19,.58]],['kerb-tilt-up',[1.4]],['stop-and-check-child',[5.2]]])for(const timeS of times)states.push([clip+'@'+timeS,{kind:'clip',clip,timeS}])
for(const [id,pose] of states){
 const m=posedMesh(a,pose),rows=[]
 for(const d of [50,100,200,500]){
  const q=qualityOf(a,d),all=a.volumes[d].body,meshes=solids=>solids.map(s=>geometry(q,m,s)),find=(...names)=>meshes(all.filter(s=>names.includes(s.id))),bones=(...bones)=>meshes(all.filter(s=>bones.includes(s.bone)))
  const grab=find('handle-grip'),handle=grab.length?grab:find('push-bar','push-frame'),seat=find('seat-sling','seat'),bar=find('bumper-bar')
  const hands=['l','r'].map(j=>surface(bones('hand-'+j),handle)),childSeat=surface(bones('child','child-thigh-l','child-thigh-r'),seat),childHands=['l','r'].map(j=>surface(bones('child-hand-'+j),bar))
  if(['pushing-stroller','kerb-tilt'].includes(id))for(const end of ['front','rear'])for(const side of ['l','r']){
   const axle=end==='rear'?find('rear-axle'):find('fork-prong-a-'+side,'fork-prong-b-'+side),hub=find('hub-'+end+'-'+side),gap=intersects(axle,hub)?0:surface(axle,hub)
   if(gap===null||gap>limits.maximumAxleGapMm)issues.push([id,d,end,side,'axle to hub',gap])
  }
  rows.push([d,...hands,childSeat,...childHands])
 }
 const expected=limits.expectedDistancesMm[id]??limits.defaultPushDistancesMm
 for(const row of rows)for(let i=1;i<row.length;i++)if(row[i]===null||Math.abs(row[i]-expected[i-1])>limits.maximumPoseErrorMm)issues.push([id,row[0],limits.distanceOrder[i-1],row[i],expected[i-1]])
 console.log(id,JSON.stringify(rows))
 for(const row of rows.slice(1))for(let i=1;i<row.length;i++)if(row[i]===null||Math.abs(row[i]-rows[0][i])>limits.maximumScaleChangeMm)issues.push([id,row[0],i,row[i],rows[0][i]])
}
assert.deepEqual(issues,[])
