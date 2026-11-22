/** 🪽 Articulate the carpal overlap across the decelerating primary and secondary fans. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Quaternion,Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,f=JSON.parse(readFileSync(new URL('./gull-touch-case.json',import.meta.url),'utf8'))
const V=p=>new Vector3(p.x,p.y,p.z),rad=Math.PI/180,round=n=>+n.toFixed(7),wrap=d=>((d+540)%360)-180
for(const pose of a.poses.filter(p=>f.poses.includes(p.id))){
 const state=()=>evaluateDrawing(a,{kind:'pose',pose:pose.id}).bones,base=state()
 for(const [side,label] of [['n','near'],['f','far']]){
  const S=base['s1-'+side].frame,P=base['p1-'+side].frame
  const aim=V(S.u).lerp(V(P.u),.61).normalize()
  for(const [prefix,weights] of [['s',f.secondaryInfluence],['p',f.primaryInfluence]]){
   const anchor=prefix==='s'?S:P,turn=new Quaternion().setFromUnitVectors(V(anchor.u),aim)
   for(const [i,weight] of weights.entries()){
    if(!weight)continue
    const id=prefix+(i+1)+'-'+side,b=a.rig.bones.find(q=>q.id===id),fr=base[b.parent].frame
    const dir=V(base[id].frame.u).applyQuaternion(new Quaternion().slerp(turn,weight))
    pose.bones[id]={...pose.bones[id],angleDeg:round(wrap(Math.atan2(dir.dot(V(fr.v)),dir.dot(V(fr.u)))/rad-b.angleDeg)),abductionDeg:round(Math.asin(Math.max(-1,Math.min(1,dir.dot(V(fr.w)))))/rad),twistDeg:0}
   }
  }
  const changed=state(),chain=[...Array.from({length:12},(_,i)=>'s'+(12-i)),...Array.from({length:10},(_,i)=>'p'+(i+1))]
  const point=(key)=>{const fr=changed[key+'-'+side].frame;return V(fr.o).addScaledVector(V(fr.u),.102)}
  for(let i=1;i<chain.length-1;i++){
   const key=chain[i],id=key+'-'+side,w=(key.startsWith('s')?f.secondaryInfluence:f.primaryInfluence)[+key.slice(1)-1];if(!w)continue
   const frame=changed[id].frame,dir=point(chain[i+1]).sub(point(chain[i-1])).normalize()
   const s=a.volumes[50].body.find(q=>q.id===label+'-'+(key.startsWith('s')?'secondary':'primary')+'-'+key.slice(1)),mid=s.sections[Math.floor(s.sections.length/2)]
   const width=new Vector3(0,1,0).applyQuaternion(new Quaternion(...mid.rotation))
   const target=Math.atan2(dir.dot(V(frame.w)),dir.dot(V(frame.v)))/rad,old=Math.atan2(width.z,width.y)/rad
   pose.bones[id].twistDeg=round(((old-target+270)%180+180)%180-90)
  }
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc))
