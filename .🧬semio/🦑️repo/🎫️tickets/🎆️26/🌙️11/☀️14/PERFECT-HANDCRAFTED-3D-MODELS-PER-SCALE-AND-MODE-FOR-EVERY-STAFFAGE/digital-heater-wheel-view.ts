/** 🛞 Inspect the full wheel pair beside its near-side cutaway, using the installed heater’s scale meshes. */
import {join} from 'node:path'
import {readFileSync} from 'node:fs'
import {Canvas} from './volumes-raster.ts'
import {bodyItems} from './volumes-render.ts'
import {GENERATED} from './volumes-lib.ts'
import {Panel,cameraAt,listsIn,paperBox,pointsOf,rigAt,savePng} from './r3-lib.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,cam=cameraAt(233,15),width=290,height=150,sheet=new Canvas(width*2,(height+22)*4)
for(const [row,d] of [50,100,200,500].entries()){
 const rig=rigAt(a,d,{kind:'pose',pose:'high'}),s=listsIn(a,d,rig.frames)
 if(s.errors.length||s.unresolved.length)throw Error('Failed to resolve running gear')
 const gear=bodyItems(a,s,d).filter(p=>/^(axle|tyre-|hub-|bracket-|bush-)/.test(p.mesh.id)),bounds=paperBox(cam,pointsOf(gear.map(p=>p.mesh)))
 const place={cam,scale:Math.min((width-14)/(bounds.maxX-bounds.minX),(height-12)/(bounds.maxY-bounds.minY)),cx:(bounds.minX+bounds.maxX)/2,cy:(bounds.minY+bounds.maxY)/2,ox:width/2,oy:height/2}
 for(const [col,items] of [gear,gear.filter(p=>!['tyre-left','hub-left'].includes(p.mesh.id))].entries()){
  const panel=new Panel(width,height,[244,244,240],1);panel.draw(place,items,Infinity);panel.shade(place)
  const x=col*width,y=row*(height+22);sheet.blit(panel.canvas(),x,y);sheet.text(x+8,y+height+5,'1:'+d+'  '+(col?'NEAR WHEEL REMOVED':'TRANSPORT WHEELS'),[35,35,32],1)
 }
}
savePng(join(GENERATED,'patio-heater-running-gear.png'),sheet)
