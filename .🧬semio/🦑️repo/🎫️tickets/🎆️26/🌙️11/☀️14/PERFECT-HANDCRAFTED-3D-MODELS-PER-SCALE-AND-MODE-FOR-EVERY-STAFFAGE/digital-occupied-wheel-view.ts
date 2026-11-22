/** 🛞 Inspect an occupied device's wheel face and its lateral handrim, flanges or moulded webs on the posed physical mesh. */
import {createHash} from 'node:crypto'
import {readFileSync} from 'node:fs'
import {join} from 'node:path'
import {Vector3} from 'three'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {posedMesh} from './living-world-mesh.ts'
import {Canvas} from './volumes-raster.ts'
import {drawAxon,fit} from './volumes-render.ts'
import {GENERATED} from './volumes-lib.ts'
import {savePng} from './r3-lib.ts'
const raw=readFileSync(process.argv[2]),a=JSON.parse(raw).asset,sha=createHash('sha256').update(raw).digest('hex').slice(0,10)
const cyclist=a.id==='cyclist',chair=a.id==='wheelchair-user',stroller=a.id==='parent-with-stroller',bone=cyclist?'front-wheel':chair?'rear-wheel':stroller?'wheel-rear':'wheel-front'
const W=290,H=230,sheet=new Canvas(W*4,H*2,[246,245,242]),pose=cyclist?'standing-on-pedals':chair?'ramp-ascent':stroller?'kerb-tilt':'walking-midstance',posed=posedMesh(a,pose)
for(const [col,d] of [50,100,200,500].entries()){
 const data=a.volumes[d].body.filter(s=>s.bone===bone&&(cyclist||s.id.endsWith('-l'))),q=qualityOf(a,d)
 const items=data.map(s=>({mesh:posed(s,q),color:s.id.includes('tyre')?[53,54,54]:s.id.startsWith('spoke')||s.id.includes('-spoke')?[148,150,151]:s.id.includes('hand-rim')?[139,140,144]:[108,115,117]}))
 for(const [row,angle] of [4,66].entries()){
  const rad=angle*Math.PI/180,e=new Vector3(Math.sin(rad),.14,Math.cos(rad)).normalize(),r=new Vector3(e.z,0,-e.x).normalize(),u=e.clone().cross(r)
  const factor=4,high=new Canvas(W*factor,H*factor,[246,245,242]),cam={e:e.toArray(),r:r.toArray(),u:u.toArray()}
  drawAxon(high,fit(items.map(x=>x.mesh),cam,W*factor,H*factor,.09),items)
  const pane=high.downsample(factor)
  pane.text(6,5,'1:'+d+(row?' OBLIQUE':' AXLE'),[58,56,54],1);sheet.blit(pane,col*W,row*H)
 }
}
savePng(join(GENERATED,'occupied-wheel-'+a.id+'-'+sha+'.png'),sheet)
