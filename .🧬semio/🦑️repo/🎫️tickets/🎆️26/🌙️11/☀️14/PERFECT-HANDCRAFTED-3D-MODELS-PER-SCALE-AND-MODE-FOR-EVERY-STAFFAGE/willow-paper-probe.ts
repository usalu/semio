/** 🌿 Visualize projected print and solid masks at one physical resolution. */
import {readFileSync,writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {Canvas} from './volumes-raster.ts'
const [file,k,view,width,save]=process.argv.slice(2)
globalThis.captureWillow=(ink,vol,x,y)=>{
 const c=new Canvas(x,y,[245,245,239]);let excess=0,missing=0,both=0
 for(let j=0;j<x*y;j++){
  const a=ink[j],b=vol[j],color=a&&b?[102,118,98]:a?[220,67,39]:b?[23,111,201]:[245,245,239]
  c.data.set(color,((y-1-Math.floor(j/x))*x+j%x)*3);if(a&&b)both++;else if(a)excess++;else if(b)missing++
 }
 writeFileSync(save,c.png());console.log(JSON.stringify({excess,missing,both}))
}
Bun.plugin({name:'willow-paper-ink',setup(b){b.onLoad({filter:/🌓️silhouette\/🟦️\.ts$/},args=>{
 const s=readFileSync(args.path,'utf8'),mark='  const spread = boundarySpread(drawnMask, reference3d'
 if(!s.includes(mark))throw Error(args.path)
 return{loader:'ts',contents:s.replace(mark,"  if(window===undefined&&view==='"+view+"')globalThis.captureWillow(drawnMask,reference3d,volume.width,volume.height);\n"+mark)}
})}})
const {measureAsset}=await import(join(import.meta.dir,'../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'))
const a=JSON.parse(readFileSync(file,'utf8')).asset
measureAsset(a,Number(k),'calm',Number(width))
