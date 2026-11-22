/** 🖐️ Compare the palm and articulated skeleton in a wrist-fixed camera across the four authored gestures. */
import {readFileSync} from 'node:fs'
import {join} from 'node:path'
import {Canvas} from './volumes-raster.ts'
import {drawAxon} from './volumes-render.ts'
import {posedMesh} from './living-world-mesh.ts'
import {GENERATED} from './volumes-lib.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {Vector3} from 'three'
import {savePng} from './r3-lib.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,f=JSON.parse(readFileSync(new URL('./digital-hand-case.json',import.meta.url),'utf8'))
const W=280,H=158,sheet=new Canvas(W*3,H*4),xyz=p=>new Vector3(p.x,p.y,p.z),e=new Vector3(.35,.54,.77).normalize(),right=new Vector3(1,0,0).addScaledVector(e,-e.x).normalize(),up=e.clone().cross(right).normalize(),cam={e:e.toArray(),r:right.toArray(),u:up.toArray()}
for(const [row,pose] of a.poses.entries()){
 const fr=evaluateDrawing(a,{kind:'pose',pose:pose.id}).bones[f.wrist].frame,o=xyz(fr.o),u=xyz(fr.u),v=xyz(fr.v),w=xyz(fr.w),mesh=posedMesh(a,pose.id)
 for(const [col,[scale,mode]] of [[50,'body'],[500,'body'],[50,'structure']].entries()){
  const q=qualityOf(a,scale),vol=a.volumes[scale],parts=vol[mode].filter(s=>s.bone===f.wrist||/^(index|middle|ring|little|thumb)-.*-r$/.test(s.bone))
  const items=parts.map(s=>{
   const g=mesh(s,q)
   for(let i=0;i<g.world.length;i+=3){const p=new Vector3(...g.world.subarray(i,i+3)).sub(o);g.world.set([p.dot(u),p.dot(v),p.dot(w)],i)}
   return {mesh:g,color:mode==='structure'?[147,158,161]:[205,180,156]}
  })
  const panel=new Canvas(W,H,[245,244,239])
  drawAxon(panel,{cam,scale:1150,cx:.080,cy:0,cz:0,ox:W/2,oy:H/2+11,zc:0},items)
  panel.text(5,5,pose.id.toUpperCase()+'  1:'+scale+' '+mode.toUpperCase(),[37,39,35],1);sheet.blit(panel,col*W,row*H)
 }
}
savePng(join(GENERATED,'standing-adult-wrist-poses.png'),sheet)
