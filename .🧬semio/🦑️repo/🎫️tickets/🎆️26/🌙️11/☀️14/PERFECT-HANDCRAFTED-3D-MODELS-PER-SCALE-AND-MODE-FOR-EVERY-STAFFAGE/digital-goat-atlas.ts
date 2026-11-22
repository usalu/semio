/** 🐐 Read the standing doe's anatomical section heights and width independently from the closed source meshes. */
import {readFileSync} from 'node:fs'
import {Vector3,Box3} from 'three'
import {posedMesh} from './living-world-mesh.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,q=qualityOf(a,50),mesh=posedMesh(a,'standing-alert')
const ids=['ribcage','abdomen','loin','withers','croup','neck','udder','head','thigh-n','shoulder-n','chest','hindbody','quarter','barrel','fore-chest','haunch','tail-1']
const targets=a.volumes['50'].body.filter(s=>ids.includes(s.id)),m=r=>+r.toFixed(3)
const group=[]
for(const s of targets){const g=mesh(s,q),box=new Box3()
 for(let i=0;i<g.world.length;i+=3)box.expandByPoint(new Vector3(...g.world.subarray(i,i+3)))
 console.log(s.id,box.min.toArray().map(m),box.max.toArray().map(m))
 if(ids.slice(0,5).includes(s.id))group.push(g)
}
for(let x=-.65;x<=.81;x+=.04){
 const box=new Box3()
 for(const g of group)for(let i=0;i<g.indices.length;i+=3){
  const points=[0,1,2].map(k=>new Vector3(...g.world.subarray(g.indices[i+k]*3,g.indices[i+k]*3+3)))
  for(let j=0;j<3;j++){const p=points[j],q=points[(j+1)%3];if((p.x-x)*(q.x-x)<=0&&q.x!==p.x)box.expandByPoint(p.clone().lerp(q,(x-p.x)/(q.x-p.x)))}
 }
 if(!box.isEmpty())console.log('section',m(x),m(box.min.y),m(box.max.y),m(box.max.z))
}
