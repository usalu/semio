/** 🐕 Compare the forequarter coat and regional heat surface in standing and seated views. */
import {readFileSync} from 'node:fs'
import {rigAt,listsIn,Panel,pointsOf,paperBox,cameraAt,savePng} from './r3-lib.ts'
import {bodyItems,thermalItems} from './volumes-render.ts'
import {Canvas} from './volumes-raster.ts'
const docs=process.argv.slice(2,4).map(path=>JSON.parse(readFileSync(path,'utf8')).asset)
const w=245,h=235,rows=[['standing-alert',110,11,false],['sitting-waiting',90,88,false],['trot-phase-2',110,13,true]]
for(const k of [50,100,200,500]){
 const page=new Canvas(w*2,(h+15)*rows.length,[255,255,255])
 rows.forEach(([pose,yaw,tilt,heat],j)=>{
  const cam=cameraAt(yaw,tilt)
  let place
  docs.forEach((a,i)=>{
   const rig=rigAt(a,k,{kind:'pose',pose}),sm=listsIn(a,k,rig.frames)
   if(sm.errors.length||sm.unresolved.length)throw Error(JSON.stringify([sm.errors,sm.unresolved]))
   const near=sm.body.filter((m,n)=>/^(chest|neck|head|shoulder-n|upper-arm-n|fore-upper-n|forearm-n|fore-leg-n)$/.test(sm.solids.body[n].id))
   const bounds=paperBox(cam,pointsOf(near))
   place??={cam,scale:Math.min((w-16)/(bounds.maxX-bounds.minX),(h-16)/(bounds.maxY-bounds.minY)),cx:(bounds.maxX+bounds.minX)/2,cy:(bounds.maxY+bounds.minY)/2,ox:w/2,oy:h/2}
   const panel=new Panel(w,h,[244,244,240],2)
   panel.draw(place,(heat?thermalItems:bodyItems)(a,sm,k));panel.shade(place)
   const top=j*(h+15)
   page.blit(panel.canvas(),i*w,top)
   page.text(i*w+4,top+h+3,(i?'FIT ':'BASE ')+pose+' 1/'+k,[0,0,0],1)
  })
 })
 savePng(process.argv[4]+'-'+k+'.png',page)
}
