/** 🌱 Restore both interlocking courses of timber lath behind the climbing ivy. */
import {readFileSync,writeFileSync} from 'node:fs'
import {frames,toWorld} from './volumes-base.ts'
import {project} from './r3-weeping-willow-kit.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,reference=a.volumes[100]
for(const k of [200,500]){
 const v=a.volumes[k],rep=a.representations[k]
 for(const id of ['lath-h','lath-v']){
  const timber=structuredClone(reference.body.find(s=>s.id===id))
  delete timber.slender
  v.body.splice(v.body.findIndex(s=>s.id==='rail-bottom')+1,0,timber)
  v.thermal=v.thermal.filter(s=>s.id!=='th-'+id)
  v.thermal.push({id:'th-'+id,of:id,region:'trellis-timber'})
  v.structure.push(structuredClone(reference.structure.find(s=>s.id==='member-'+id)))
 }
 for(const [view,ink] of Object.entries(rep.geometry)){
  const fine=a.representations[100].geometry[view].filter(s=>s.id.startsWith('lath-'))
  const outlines=structuredClone(fine)
  if(k===500&&view==='elevation')for(const s of outlines.filter(s=>s.id.startsWith('lath-v-'))){
   const center=(s.points[0][0]+s.points[1][0])/2
   s.points=s.points.map(([x,y])=>[+(center+(x-center)*1.6).toFixed(5),y])
  }
  if(view!=='elevation'){
   const fr=frames(a).panel
   for(const q of v.body.filter(q=>q.id==='lath-h'||q.id==='lath-v')){
    const unique=new Set()
    for(let i=0;i<q.repeat.count;i++){
     const center=q.transform.translation.map((p,j)=>p+i*q.repeat.translation[j]),corners=[]
     for(const u of [-1,1])for(const v of [-1,1])for(const w of [-1,1])corners.push(project(view,toWorld(fr,center.map((p,j)=>p+q.size[j]*[u,v,w][j]/2))))
     const x0=Math.min(...corners.map(p=>p.x)),x1=Math.max(...corners.map(p=>p.x)),y0=Math.min(...corners.map(p=>p.y)),y1=Math.max(...corners.map(p=>p.y)),points=[[x0,y0],[x1,y0],[x1,y1],[x0,y1]].map(p=>p.map(n=>+n.toFixed(5)))
     const key=JSON.stringify(points);if(unique.has(key))continue;unique.add(key)
     outlines.push({id:q.id+'-edge-'+(i+1),bone:'panel',role:'body',kind:'poly',closed:true,filled:true,z:.95,tone:q.tone,region:'trellis-timber',material:q.material,points})
    }
   }
  }
  rep.geometry[view]=[...ink,...outlines].sort((l,r)=>l.z-r.z)
 }
 v.decisions.unshift({en:'Eight vertical and seven horizontal timber laths remain physically separate. The two courses touch face to face, support the ivy and carry its load to the rails and posts.',de:'Acht senkrechte und sieben waagerechte Holzlatten bleiben einzeln erhalten. Die beiden Lagen berühren sich flächig, stützen den Efeu und leiten seine Last in Riegel und Pfosten.'})
}
writeFileSync(process.argv[3],JSON.stringify(doc))
