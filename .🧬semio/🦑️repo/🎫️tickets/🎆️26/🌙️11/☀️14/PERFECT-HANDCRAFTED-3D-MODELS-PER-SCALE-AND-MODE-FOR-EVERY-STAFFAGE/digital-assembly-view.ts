/** 🔎 Inspect the travelling crane carriage, road wheel and railing section from axial, face and oblique views. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {posedMesh} from './living-world-mesh.ts'
import {Canvas} from './volumes-raster.ts'
import {drawAxon,fit} from './volumes-render.ts'
const [path,output]=process.argv.slice(2),a=JSON.parse(readFileSync(path,'utf8')).asset,crane=a.id==='tower-crane',guard=a.id==='cantilever-balcony'
const v=a.volumes['500'],w=340,h=270,sheet=new Canvas(w*3,h),asMesh=posedMesh(a,a.presentation.defaultPose,true)
const solids=v.body.filter(s=>crane?s.bone==='trolley':guard?s.bone==='railing':s.bone==='wheel-f'&&/(^fl-|-fl$)/.test(s.id))
const colors={rubber:[68,69,67],alloy:[180,187,190],'steel-rim':[157,165,164],'cast-iron':[122,105,91],'nickel-plate':[203,209,209],steel:[152,157,163]}
for(const [i,[az,el]] of (crane?[[30,18],[135,-21],[2,2]]:guard?[[86,6],[48,13],[8,16]]:[[2,3],[37,17],[144,-17]]).entries()){
 const A=az*Math.PI/180,E=el*Math.PI/180,e=new Vector3(Math.sin(A)*Math.cos(E),Math.sin(E),Math.cos(A)*Math.cos(E)),r=new Vector3(e.z,0,-e.x).normalize(),u=e.clone().cross(r),cam={e:e.toArray(),r:r.toArray(),u:u.toArray()}
 const items=solids.map(s=>({mesh:asMesh(s,qualityOf(a,500)),color:colors[s.material]??(s.id.includes('sheave')?[166,150,115]:s.id.includes('wheel')?[118,127,135]:[152,161,169])}))
 const p=new Canvas(w,h,[246,246,242]);drawAxon(p,fit(items.map(q=>q.mesh),cam,w,h,.1),items);sheet.blit(p,i*w,0)
}
writeFileSync(output,sheet.png())
