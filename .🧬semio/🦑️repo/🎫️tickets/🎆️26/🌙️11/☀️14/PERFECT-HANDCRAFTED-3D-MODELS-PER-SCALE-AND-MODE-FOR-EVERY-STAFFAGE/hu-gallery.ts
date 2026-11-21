/** 👥 Renders compact contact sheets directly from each current human catalog entry. */
import { join } from 'node:path'
import { Canvas } from './volumes-raster.ts'
import { catalogEntries, loadAsset, GENERATED } from './volumes-lib.ts'
import { bodyItems, thermalItems, structureItems } from './volumes-render.ts'
import { cameraAt, listsIn, Panel, paperBox, pointsOf, rigAt, savePng } from './r3-lib.ts'
import { solveThermal } from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts'
import { thermalPaint } from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🌡️thermal-view/🟦️.ts'
const surface=process.argv.includes('--surface')
const humans = catalogEntries().filter(e => e.category === 'humans').sort((a,b)=>a.slug.localeCompare(b.slug))
const frameScaleOf=(a,scale)=>a.representations[scale].status==='authored'?scale:a.presentation.defaultScale
const alternate=process.argv.includes('--alternate')
const selectedScale=Number(process.argv.find(arg=>arg.startsWith('--scale='))?.slice(8))
const width=205, height=208
const cam=cameraAt(alternate?-37:56,alternate?8:14)
for(const scale of (selectedScale?[selectedScale]:alternate?[50,500]:[50,100,200,500])){
  const sheets = [0,1,2].map(()=>new Canvas(width*7,height*4,[250,250,248]))
  for(const [i,entry] of humans.entries()){
    const a=loadAsset(entry.path)
    if(a.volumes[scale].status!=='authored')continue
    const rig=rigAt(a,scale,{kind:'pose',pose:alternate?(a.poses.find(p=>p.id!==a.presentation.defaultPose)?.id??a.presentation.defaultPose):a.presentation.defaultPose})
    const mesh=listsIn(a,scale,rig.frames)
    const heat=thermalItems(a,mesh,frameScaleOf(a,scale))
    if(surface){
      const field=solveThermal(rig.state,{...a.thermal.environment,airTempC:14,skyTempC:14,groundTempC:14,meanRadiantTempC:14,solarWM2:0,windMS:.3})
      const paint=thermalPaint(a,field,'surface-temperature',{mode:'span',spanK:32},'light')
      for(const [i,item] of heat.entries()){
        const solid=mesh.solids.thermal[i],region=a.thermal.regions.find(r=>r.id===solid.region)
        const inner=solid.layer&&solid.layer!==region?.layers.at(-1)?.id
        const f=field.regions[solid.region],temp=(inner?f.skinTempC:null)??f.surfaceTempC
        const hex=paint.colourOf(temp).replace('#','')
        item.color=[0,2,4].map(k=>parseInt(hex.slice(k,k+2),16))
      }
    }
    if(mesh.errors.length||mesh.unresolved.length)throw Error(entry.slug+' '+[...mesh.errors,...mesh.unresolved])
    const box=paperBox(cam,pointsOf(mesh.body))
    const place={cam,scale:Math.min((width-12)/(box.maxX-box.minX),(height-35)/(box.maxY-box.minY)),cx:(box.maxX+box.minX)/2,cy:(box.maxY+box.minY)/2,ox:width/2,oy:(height-20)/2}
    const frameScale=frameScaleOf(a,scale)
    for(const [j,items] of [bodyItems(a,mesh,frameScale),heat,structureItems(a,mesh,frameScale)].entries()){
      const panel=new Panel(width,height-18,[245,245,241],1)
      panel.draw(place,items,Infinity)
      panel.shade(place)
      const x=(i%7)*width, y=Math.floor(i/7)*height
      sheets[j].blit(panel.canvas(),x,y)
      sheets[j].text(x+3,y+height-14,entry.slug,[12,12,12],1)
    }
    Bun.gc(false)
  }
  for(const [j,mode] of ['body','thermal','structure'].entries())savePng(join(GENERATED,'hu-review',(surface?'heat-':'')+(alternate?'alt-':'')+scale+'-'+mode+'.png'),sheets[j])
}
