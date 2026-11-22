/** 🔍 Inspect the actual lumbar shells or skateboard running gear from three directions. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {posedMesh} from './living-world-mesh.ts'
import {Canvas} from './volumes-raster.ts'
import {drawAxon,fit} from './volumes-render.ts'
const [path,output]=process.argv.slice(2),a=JSON.parse(readFileSync(path,'utf8')).asset
const dog=a.id==='labrador-dog',view=new Canvas(870,240,[248,248,245])
const v=a.volumes[dog?'50':'500'],make=posedMesh(a,a.presentation.defaultPose)
const solids=dog?v.structure.filter(q=>/^lumbar-l[1-7]$/.test(q.id)):v.body.filter(q=>['board','wheel-f','wheel-r'].includes(q.bone))
for(const [i,[az,el]] of (dog?[[23,22],[4,83],[90,0]]:[[24,-18],[-35,-38],[5,6]]).entries()){
 const A=az*Math.PI/180,E=el*Math.PI/180,e=new Vector3(Math.sin(A)*Math.cos(E),Math.sin(E),Math.cos(A)*Math.cos(E)),r=new Vector3(e.z,0,-e.x).normalize(),u=e.clone().cross(r)
 const items=(dog&&i===2?solids.filter(s=>s.id==='lumbar-l4'):solids).map(s=>({mesh:dog?{world:s.vertices.flat(),indices:s.triangles.flat()}:make(s,qualityOf(a,500)),color:dog?[231,215,177]:s.material==='maple'?[187,139,84]:s.material==='polyurethane'?[219,201,166]:[117,128,140]}))
 const pane=new Canvas(290,240,[248,248,245]),cam={e:e.toArray(),r:r.toArray(),u:u.toArray()}
 drawAxon(pane,fit(items.map(q=>q.mesh),cam,290,240,.07),items)
 pane.text(6,5,(dog?['L1-L7 OBLIQUE','L1-L7 DORSAL','L4 AXIAL']:['1:500 UNDERSIDE','1:500 TRUCKS','1:500 PROFILE'])[i],[68,62,55],1)
 view.blit(pane,290*i,0)
}
writeFileSync(output,view.png())
