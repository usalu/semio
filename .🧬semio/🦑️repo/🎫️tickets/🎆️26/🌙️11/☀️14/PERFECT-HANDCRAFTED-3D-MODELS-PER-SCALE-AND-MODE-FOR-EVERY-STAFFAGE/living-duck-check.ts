/** 🦆 Independently compare the mallard's rest articulation and stretched wing to Three.js frames. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {join} from 'node:path'
import {Matrix4,Vector3} from 'three'
import {catalogEntries} from './volumes-lib.ts'
import {solveChain,restFrames} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts'
const f=JSON.parse(readFileSync(join(import.meta.dir,'living-duck-wings.json'),'utf8'))
const a=JSON.parse(readFileSync(catalogEntries().find(x=>x.slug===f.asset)!.path,'utf8')).asset
const rest=restFrames(a.rig),p=a.poses.find(x=>x.id===f.pose)
const frames=solveChain(a.rig,p.bones,{x:0,y:0,angleDeg:0})
function matrix(k){const {o,u,v,w}=k.frame;return new Matrix4().set(u[0],v[0],w[0],o[0],u[1],v[1],w[1],o[1],u[2],v[2],w[2],o[2],0,0,0,1)}
for(const [side,name,sign] of f.side){
 const shoulder='wing-'+side,fore='forearm-'+side,hand='hand-'+side
 assert(rest[fore]&&rest[hand],side+' needs an articulated elbow and wrist')
 const local=(p)=>new Vector3(p[0],p[1],p[2]*sign)
 const get=(s,x)=>new Vector3().applyMatrix4(matrix(s[x]))
 const distal=local(f.shoulderFrameManusTip).applyMatrix4(matrix(rest[shoulder]))
 for(const [bone,target] of [[fore,f.shoulderFrameElbow],[hand,f.shoulderFrameWrist]])assert(get(rest,bone).distanceTo(local(target).applyMatrix4(matrix(rest[shoulder])))<f.maximumJointErrorM)
 assert(distal.distanceTo(new Vector3(a.rig.bones.find(b=>b.id===hand).lengthM,0,0).applyMatrix4(matrix(rest[hand])))<f.maximumJointErrorM)
 const tip=new Vector3(a.rig.bones.find(b=>b.id===hand).lengthM,0,0).applyMatrix4(matrix(frames[hand]))
 const old=local(f.shoulderFrameManusTip).applyMatrix4(matrix(frames[shoulder]))
 assert(tip.distanceTo(old)>f.minimumJointMovementM)
 for(const d of f.scales){
  const v=a.volumes[d]
  for(const b of [fore,hand])assert(v.structure.some(s=>s.member===b&&s.part==='member'))
  const primary=v.body.find(s=>s.id===name+'-primaries')
  assert(primary?.bone===hand||primary?.weights?.some(w=>w.bone===hand&&w.weight===1))
 }
 console.log('[DEBUG]',name,'relative primary-tip travel',tip.distanceTo(old))
}
