/** 🖼️ Render the current body's surface, thermal regions and structure in the same pose and camera, one solid at a time. */
import {createHash} from 'node:crypto'
import {mkdirSync,readFileSync,writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {posedMesh} from './living-world-mesh.ts'
import {catalogEntries} from './volumes-lib.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {solveThermal} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts'
import {solidLook} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🎨️looks/🟦️.ts'
import {thermalLook} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🌡️thermal/🟦️.ts'
import {thermalPaint} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🌡️thermal-view/🟦️.ts'
import {paperInput} from './living-paper-input.ts'
import {Canvas,hsl} from './volumes-raster.ts'
import {drawAxon,PART_COLOR} from './volumes-render.ts'
import {eachSolid,sceneInput} from './living-volume-input.ts'
const [name,which,poseRequested]=process.argv.slice(2),entry=catalogEntries().find(e=>e.slug===name)!
const raw=readFileSync(process.argv[5]??entry.path,'utf8'),sha=createHash('sha256').update(raw).digest('hex')
const asset=sceneInput(raw),pose=poseRequested||asset.presentation.defaultPose
const temperatures=process.argv.includes('--temperature'),settings={quantity:'surface-temperature',range:{mode:'shared'},theme:'light'} as const
const thermal=temperatures?solveThermal(evaluateDrawing(paperInput(raw),{kind:'pose',pose})):undefined
const scale=thermal?thermalPaint(asset,thermal,settings.quantity,settings.range,settings.theme).scale:undefined
const cache=new Map<string,number[]>()
function surfaceTint(s){
 const surface={...s,regions:s.regions??(s.region?[s.region]:[])}
 const key=JSON.stringify([surface.regions,s.layer,s.tone,s.role])
 if(!cache.has(key)){
  const look=thermalLook(asset,settings,[surface]),hex=look.paint(thermal!).get(look.keyOf(surface))!.hex
  cache.set(key,[1,3,5].map(i=>parseInt(hex.slice(i,i+2),16)))
 }
 return cache.get(key)!
}
const smooth=process.argv.includes('--shaded'),asMesh=posedMesh(asset,pose,smooth)
const presentation=process.argv.includes('--presentation'),opposite=process.argv.includes('--opposite'),az=(opposite?214:34)*Math.PI/180,under=process.argv.includes('--underside'),el=(under?-38:18)*Math.PI/180
const cam={e:[Math.sin(az)*Math.cos(el),Math.sin(el),Math.cos(az)*Math.cos(el)],r:[Math.cos(az),0,-Math.sin(az)],u:[-Math.sin(az)*Math.sin(el),Math.cos(el),-Math.cos(az)*Math.sin(el)]}
const w=320,h=235,scales=which&&which!=='all'?[which]:['50','100','200','500'],sheet=new Canvas(w*3,h*scales.length)
function colour(s,mode){
 if(mode==='structure')return PART_COLOR[s.part]??[95,95,95]
 if(mode==='thermal'&&thermal)return surfaceTint(s)
 if(mode==='thermal')return hsl(asset.thermal.regions.findIndex(x=>x.id===s.region)*137.5,.63,.53)
 const m=s.material,gray=Math.min(.82,.27+.52*(s.tone??.55))
 const hue=/(leaf|needle|foliage|blade|culm|grass)/.test(m)?102:/(fruit|flower)/.test(m)?13:/(wood|bark|stem|branch|horn|hoof)/.test(m)?32:/(plumage|feather)/.test(m)?44:22
 return hsl(hue,/(eye|iron|steel|bone)/.test(m)?.06:.36,gray)
}
for(const [row,k] of scales.entries()){
 const visit=eachSolid(raw,k),q=qualityOf(asset,Number(k))
 let rendered=0
 function mesh(s){
  if(++rendered%128===0&&raw.length>5_000_000)Bun.gc(true)
  return asMesh(s,q)
 }
 const extent=[Infinity,-Infinity,Infinity,-Infinity]
 for(const s of visit('body')){const v=mesh(s).world;for(let i=0;i<v.length;i+=3){const dx=v[i]*cam.r[0]+v[i+2]*cam.r[2],dy=v[i]*cam.u[0]+v[i+1]*cam.u[1]+v[i+2]*cam.u[2];extent[0]=Math.min(extent[0],dx);extent[1]=Math.max(extent[1],dx);extent[2]=Math.min(extent[2],dy);extent[3]=Math.max(extent[3],dy)}}
 const ax={cam,scale:Math.min((w-20)/(extent[1]-extent[0]),(h-32)/(extent[3]-extent[2])),cx:(extent[0]+extent[1])/2,cy:(extent[2]+extent[3])/2,cz:0,ox:w/2,oy:h/2+8,zc:0}
 for(const [col,mode] of ['body','thermal','structure'].entries()){
  const view=new Canvas(w,h,[247,247,244])
  if(mode==='body'&&presentation){
   const panes=[]
   for(const solid of visit(mode)){
    const look=solidLook('light','light-solid',[solid]),paint=look.paint().get(look.keyOf(solid)),color=[1,3,5].map(i=>parseInt(paint.hex.slice(i,i+2),16))
    const item={mesh:mesh(solid),color}
    if(paint.alpha>=1)drawAxon(view,ax,[item],{clearDepth:false})
    else{
     const v=item.mesh.world,e=cam.e;let depth=0
     for(let i=0;i<v.length;i+=3)depth+=(v[i]*e[0]+v[i+1]*e[1]+v[i+2]*e[2])/(v.length/3)
     panes.push({item,alpha:paint.alpha,depth})
    }
   }
   for(const {item,alpha} of panes.sort((p,q)=>p.depth-q.depth)){
    const layer=new Canvas(w,h);layer.depth.set(view.depth);drawAxon(layer,ax,[item],{clearDepth:false})
    for(let i=0;i<w*h;i++)if(layer.depth[i]>view.depth[i]){
     for(let c=0;c<3;c++){const j=i*3+c;view.data[j]=alpha*layer.data[j]+(1-alpha)*view.data[j]}
    }
   }
  }else for(const s of visit(mode))drawAxon(view,ax,[{mesh:mesh(s),color:colour(s,mode)}],{clearDepth:false,unlit:mode==='thermal'&&temperatures})
  view.text(7,5,'1:'+k+'  '+(mode==='thermal'?(scale?'SURFACE TEMP '+scale.min.toFixed(0)+' TO '+scale.max.toFixed(0)+' C':'THERMAL REGIONS'):mode.toUpperCase()),[40,45,42],1);sheet.blit(view,col*w,row*h);Bun.gc(true)
 }
}
const out=join(import.meta.dir,'🗑️generated','living-current-views');mkdirSync(out,{recursive:true})
const flags=[...(opposite?['opposite']:[]),...(under?['underside']:[]),...(presentation?['presentation']:[]),...(smooth?['shaded']:[]),...(temperatures?['temperature']:[])]
const label=process.argv.includes('--short-name') ? flags.map(f=>f.slice(0,1)).join('') : flags.join('-')
const path=join(out,[name,pose,which||'all',...(label?[label]:[]),sha.slice(0,10)].join('-')+'.png')
writeFileSync(path,sheet.png());console.log('[DEBUG]',path)
