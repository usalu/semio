/** 🎸 View the actual fingertips and neck from the playable side at two scales and in structure. */
import {readFileSync} from 'node:fs'
import {createHash} from 'node:crypto'
import {join} from 'node:path'
import {Vector3} from 'three'
import {Canvas} from './volumes-raster.ts'
import {drawAxon} from './volumes-render.ts'
import {posedMesh} from './living-world-mesh.ts'
import {GENERATED} from './volumes-lib.ts'
import {savePng} from './r3-lib.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const raw=readFileSync(process.argv[2]),a=JSON.parse(raw).asset,sha=createHash('sha256').update(raw).digest('hex').slice(0,10),states=['strumming','fretting-high','resting','tuning'],W=330,H=165,sheet=new Canvas(W*3,H*states.length),v=p=>new Vector3(p.x,p.y,p.z)
const e=new Vector3(.13,-.91,.46).normalize(),right=new Vector3(1,0,0).addScaledVector(e,-e.x).normalize(),up=e.clone().cross(right),cam={e:e.toArray(),r:right.toArray(),u:up.toArray()}
for(const [row,pose] of states.entries()){
 const bones=evaluateDrawing(a,{kind:'pose',pose}).bones,g=bones.guitar.frame,mesh=posedMesh(a,pose),local=p=>{const x=p.clone().sub(v(g.o));return new Vector3(x.dot(v(g.u)),x.dot(v(g.v)),x.dot(v(g.w)))},cx=local(v(bones['hand-l'].frame.o)).x
 for(const [col,[scale,mode]] of [[50,'body'],[500,'body'],[500,'structure']].entries()){
  const vol=a.volumes[scale],q=qualityOf(a,scale),parts=vol[mode].map(s=>s.of?vol.body.find(b=>b.id===s.of):s).filter(s=>s&&(s.bone==='hand-l'||s.bone==='guitar'||/^(index|middle|ring|little|thumb)-.*-l$/.test(s.bone)))
  const items=parts.map(s=>{const m=mesh(s,q);for(let i=0;i<m.world.length;i+=3)m.world.set(local(new Vector3().fromArray(m.world,i)).toArray(),i)
   const color=s.bone!=='guitar'?(mode==='structure'?[197,198,189]:[218,182,147]):(s.material==='steel'?[207,205,187]:s.id==='guitar-fretboard'?[96,64,47]:[147,105,65]);return {mesh:m,color}
  })
  const focus=new Vector3(cx,-.050,pose==='tuning'?-.152:-.117),panel=new Canvas(W,H,[242,241,236]);drawAxon(panel,{cam,scale:1010,cx:focus.dot(right),cy:focus.dot(up),cz:0,ox:W/2,oy:H/2+6,zc:0},items);panel.text(5,5,pose.toUpperCase()+' 1:'+scale+' '+mode.toUpperCase(),[30,35,31],1);sheet.blit(panel,col*W,row*H)
 }
}
savePng(join(GENERATED,'guitar-fret-hand-'+sha+'.png'),sheet)
