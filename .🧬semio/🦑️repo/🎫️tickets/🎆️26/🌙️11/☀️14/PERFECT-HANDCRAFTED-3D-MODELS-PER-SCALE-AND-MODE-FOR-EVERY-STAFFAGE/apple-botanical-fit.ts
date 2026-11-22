/** 🍎 Open the apple crown around its scaffold limbs, retain bearing fruit and put every crown group back on its own joint. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Quaternion,Vector3,Box3} from 'three'
import {frames,toWorld} from './volumes-base.ts'
import {project,toDrawing,hull,outline,pathOf} from './r3-weeping-willow-kit.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,v=a.volumes[500],ref=a.volumes[200],fr=frames(a)
const world=(q,p)=>toWorld(fr[q.bone],new Vector3(...p).applyQuaternion(new Quaternion(...(q.transform?.rotation??[0,0,0,1]))).add(new Vector3(...(q.transform?.translation??[0,0,0]))).toArray())
const bounds=new Box3()
for(const q of ref.body.filter(q=>q.material.startsWith('apple-leaf')))for(const p of q.vertices)bounds.expandByPoint(new Vector3(...world(q,p)))
const sel=[]
for(const original of ref.body){
 if(!original.material.startsWith('apple-leaf'))continue
 const q=structuredClone(original),match=q.id.match(/^(lo|sk|mi|up|tp)-(\d+)-(m|b\d+)$/),tier=match?.[1],i=Number(match?.[2]??1)
 if(match&&match[3]!=='m'&&!(match[3]==='b1'&&(i%2===1||tier==='mi'))&&!(match[3]==='b6'&&i%3===0))continue
 const direction=(i%2===0?1:-1),s=({lo:.76,sk:.72,mi:.81,up:.91,tp:.88}[tier]??(q.material.endsWith('shade')?.73:.95))+(i%3-1)*.025
 q.vertices=q.vertices.map(p=>{
  const [x,y,z]=world(q,p),edge=Math.min(x-bounds.min.x,bounds.max.x-x,z-bounds.min.z,bounds.max.z-z,bounds.max.y-y),w=Math.min(1,Math.max(0,(edge-.045)/.28))
  const h=Math.max(-1,Math.min(1,p[2])),sx=1-w*(1-s),sy=1-w*(1-s*.96),sz=1-w*(1-s*.95)
  return [+(p[0]*sx+.035*direction*w*h*h).toFixed(6),+(p[1]*sy).toFixed(6),+(p[2]*sz).toFixed(6)]
 })
 sel.push(q)
}
const woody=ref.body.filter(q=>q.material==='apple-bark-young'),fruit=ref.body.filter(q=>q.material==='apple-fruit'),bases=v.body.filter(q=>!q.material.startsWith('apple-leaf'))
v.body=[...bases,...structuredClone(woody),...sel,...structuredClone(fruit)]
const regions=new Map(ref.thermal.filter(s=>s.of).map(s=>[s.of,s.region]))
const remaining=v.thermal.filter(t=>t.region==='spur-twig')
v.thermal=v.body.map(q=>({id:'th-'+q.id,of:q.id,region:regions.get(q.id)??(q.material==='apple-bark'?'trunk-bark':undefined)}))
v.thermal.push(...remaining)
for(const e of ref.thermal.filter(t=>t.tags?.length)){
 const candidates=sel.filter(q=>q.bone===e.bone&&regions.get(q.id)===e.region)
 if(!candidates.length)throw Error(e.id)
 const epos=new Vector3(...(e.transform?.translation??[0,0,0]));candidates.sort((l,r)=>new Vector3(...l.transform.translation).distanceToSquared(epos)-new Vector3(...r.transform.translation).distanceToSquared(epos))
 v.thermal.push({id:e.id,region:e.region,tags:e.tags,of:candidates[0].id})
}
for(const [view,shapes] of Object.entries(a.representations[200].geometry)){
 const crown=s=>s.bone.startsWith('fol-')&&s.region!=='fruit'&&s.filled
  const geometry=structuredClone(shapes.filter(s=>!crown(s)))
 for(const group of new Set(sel.map(q=>q.bone))){
  const q=sel.find(q=>q.bone===group),old=shapes.find(s=>s.bone===group&&crown(s))??{id:'canopy-'+group,bone:group,role:'body',z:1.2,tone:q.tone,filled:true,region:regions.get(q.id)},rings=outline(sel.filter(q=>q.bone===group).map(s=>hull(s.vertices.map(p=>project(view,world(s,p))))),.006,.0003)
  if(!rings.length)throw Error(group+' '+view)
  const ink=structuredClone(old);delete ink.points;delete ink.closed;delete ink.solid
  geometry.push({...ink,kind:'path',d:pathOf(toDrawing(view,fr[group],rings))})
 }
 a.representations[500].geometry[view]=geometry.sort((l,r)=>l.z-r.z)
}
v.structure=v.structure.map(s=>s.part==='member'&&s.member!=='root-plate'?(structuredClone(ref.structure.find(q=>q.id===s.id)??s)):s)
v.decisions.unshift({en:'The fork opens into eight visible scaffold sections. Separate fruiting crown groups follow their seven wind joints, leaving irregular windows between the lower branches. Apples remain on the fruiting wood.',de:'Die Gabel öffnet sich in acht Leitäste. Getrennte Fruchtkronen folgen sieben Windgelenken und lassen ungleichmäßige Durchblicke zwischen den unteren Ästen. Die Äpfel bleiben am Fruchtholz.'})
writeFileSync(process.argv[3],JSON.stringify(doc))
