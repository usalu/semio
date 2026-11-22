/** 🎋 Compare a terminal bamboo fan and an adjacent side shoot at four model scales. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
import {posedMesh} from './living-world-mesh.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {sceneInput,eachSolid} from './living-volume-input.ts'
import {Canvas} from './volumes-raster.ts'
import {drawAxon,fit} from './volumes-render.ts'
const raw=readFileSync(process.argv[2],'utf8'),a=sceneInput(raw),mesh=posedMesh(a,'fresh-breeze',true),w=270,h=200,sheet=new Canvas(w*3,(h+15)*4,[247,248,241])
for(const [row,k] of [50,100,200,500].entries()){
 const solids=Array.from(eachSolid(raw,String(k))('body'))
 for(const [col,[label,az,el]] of [['tip',25,30],['tip reverse',203,-13],['lateral',40,19]].entries()){
  let selection=solids.filter(q=>col<2?q.id.startsWith('apex1-'):k===500?q.id==='fol-1-plume-1':k===200?q.id==='b1-14p-tuft':q.id==='b1-14p'||q.id.startsWith('b1-14p-'))
  const rad=Math.PI/180,E=new Vector3(Math.sin(az*rad)*Math.cos(el*rad),Math.sin(el*rad),Math.cos(az*rad)*Math.cos(el*rad)),R=new Vector3(E.z,0,-E.x).normalize(),U=E.clone().cross(R),cam={r:R.toArray(),u:U.toArray(),e:E.toArray()}
  const items=selection.map(q=>({mesh:mesh(q,qualityOf(a,k)),color:q.material==='bamboo-culm'?[109,126,64]:[87,127,64]})),v=new Canvas(w,h,[240,242,231])
  drawAxon(v,fit(items.map(i=>i.mesh),cam,w,h,.12),items);const y=row*(h+15);sheet.blit(v,col*w,y);sheet.text(col*w+4,y+h+3,'1/'+k+' '+label,[43,45,39],1)
 }
}
writeFileSync(process.argv[3],sheet.png())
