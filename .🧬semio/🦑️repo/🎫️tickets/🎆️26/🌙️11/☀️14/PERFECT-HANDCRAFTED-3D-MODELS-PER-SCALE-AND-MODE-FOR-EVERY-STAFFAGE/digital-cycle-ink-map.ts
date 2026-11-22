/** 🍂 Paint the exact measured front silhouette, its two outlines and each long boundary deviation. */
import {readFileSync,writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {PNG} from 'pngjs'
const dest=join(import.meta.dir,'🗑️generated','cargo-frame-overlap.png')
globalThis.maplePlot=(a,b,w,h,cell,box)=>{
 const edge=z=>{const e=new Uint8Array(z.length);for(let y=1;y<h-1;y++)for(let x=1;x<w-1;x++){const i=y*w+x;if(z[i]&&(!z[i-1]||!z[i+1]||!z[i-w]||!z[i+w]))e[i]=1}return e}
 const ea=edge(a),eb=edge(b),eA=[],eB=[];for(let i=0;i<a.length;i++){if(ea[i])eA.push(i);if(eb[i])eB.push(i)}
 const magnify=3,img=new PNG({width:w*magnify,height:h*magnify}),worst=[]
 for(let y=0;y<h;y++)for(let x=0;x<w;x++){
  const i=y*w+x;let c=a[i]&&b[i]?[170,180,166]:a[i]?[215,124,67]:b[i]?[74,135,189]:[245,244,236]
  for(const [is,targets,key] of [[ea[i],eB,'drawing'],[eb[i],eA,'mesh']])if(is){
   const near=Math.sqrt(Math.min(...targets.map(n=>(n%w-x)**2+(Math.floor(n/w)-y)**2)))*cell
   if(near>.20){c=key==='drawing'?[237,44,11]:[38,36,204];worst.push([key,+near.toFixed(3),+(box.minX+(x+.5)*cell).toFixed(2),+(box.minY+(y+.5)*cell).toFixed(2)])}
  }
  for(let j=0;j<magnify;j++)for(let k=0;k<magnify;k++){const d=4*(((h-y-1)*magnify+j)*img.width+x*magnify+k);img.data.set([...c,255],d)}
 }
 writeFileSync(dest,PNG.sync.write(img));console.log(JSON.stringify(worst.sort((a,b)=>b[1]-a[1]).slice(0,30)));console.log(box,cell)
}
Bun.plugin({name:'bicycle-boundaries',setup(b){b.onLoad({filter:/🌓️silhouette\/🟦️\.ts$/},args=>{
 const s=readFileSync(args.path,'utf8'),hook='  const spread = boundarySpread(drawnMask, reference3d'
 if(!s.includes(hook))throw Error(args.path)
 return {loader:'ts',contents:s.replace(hook,'  if(window===undefined)globalThis.maplePlot(fillSmallHoles(drawnMask,volume.width,volume.height,(4*featureM/cell)**2),fillSmallHoles(reference3d,volume.width,volume.height,(4*featureM/cell)**2),volume.width,volume.height,cell,box);\n'+hook)}
})}})
await import(join(import.meta.dir,'living-silhouette-view.ts'))
