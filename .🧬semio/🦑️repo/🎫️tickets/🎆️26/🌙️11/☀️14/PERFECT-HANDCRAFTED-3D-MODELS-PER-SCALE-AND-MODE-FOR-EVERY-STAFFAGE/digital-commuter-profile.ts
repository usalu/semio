/** 🧑 Compare the live and sculpted head straight on and in three-quarter view at the same magnification. */
import {readFileSync,writeFileSync} from 'node:fs'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {tessellateSolid} from '../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🧱️solid/🟦️.ts'
import {Canvas} from './volumes-raster.ts'
import {drawAxon} from './volumes-render.ts'
const args=process.argv.slice(2),sheet=new Canvas(660,580)
for(const [row,path] of args.slice(0,2).entries()){
 const a=JSON.parse(readFileSync(path,'utf8')).asset
 for(const [col,az] of [0,.55].entries()){
  const p=new Canvas(330,290,[246,243,237]),cam={e:[Math.sin(az),0,Math.cos(az)],r:[Math.cos(az),0,-Math.sin(az)],u:[0,1,0]}
  const ax={cam,scale:1050,ox:165,oy:153,cx:0,cy:.027,cz:0,zc:0}
  for(const s of a.volumes['50'].body.filter(s=>s.bone==='head')){
   const m=tessellateSolid({...s,node:'head'},qualityOf(a,50)),w=new Float64Array(m.positions.length)
   for(let i=0;i<w.length;i+=3){w[i]=m.positions[i+2];w[i+1]=m.positions[i];w[i+2]=-m.positions[i+1]}
   const color=s.id==='hair'?[63,49,42]:s.id.includes('pupil')?[33,28,23]:s.id.includes('iris')?[105,80,55]:s.id.includes('eye-')?s.kind==='ellipsoid'?[71,50,41]:[221,216,201]:s.id.includes('lips')?[177,114,100]:[205,161,136]
   drawAxon(p,ax,[{mesh:{world:w,indices:m.indices},color}],{clearDepth:false})
  }
  p.text(8,6,row?'SCULPTED  1:50':'ORIGINAL  1:50',[50,50,50],1)
  sheet.blit(p,col*330,row*290)
 }
}
writeFileSync(args[2],sheet.png())
