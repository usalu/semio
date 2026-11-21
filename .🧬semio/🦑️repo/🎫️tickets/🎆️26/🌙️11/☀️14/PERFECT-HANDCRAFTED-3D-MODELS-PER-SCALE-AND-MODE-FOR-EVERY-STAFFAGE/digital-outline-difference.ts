/** 🔎 Locate missing and excess silhouette ink in the same physical grid as the catalog reviewer. */
import {join} from 'node:path';
import {readFileSync} from 'node:fs';
Bun.plugin({name:'inspect-regions',setup(b){b.onLoad({filter:/🌓️silhouette\/🟦️\.ts$/},args=>{
 const s=readFileSync(args.path,'utf8'),hook='  const spread = boundarySpread(drawnMask, reference3d';
 if(!s.includes(hook))throw Error('Raster hook moved');
 return {loader:'ts',contents:s.replace(hook,`  if(window===undefined){
  const zones=new Map();
  for(let k=0;k<drawnMask.length;k++)if(drawnMask[k]!==reference3d[k]){
   const x=box.minX+(k%volume.width+.5)*cell,y=box.minY+(Math.floor(k/volume.width)+.5)*cell;
   const key=[Math.floor(x/.05),Math.floor(y/.10),drawnMask[k]?'excess':'missing'].join('/');
   zones.set(key,(zones.get(key)??0)+1);
  }
  console.log(view,cell,JSON.stringify([...zones].sort((a,b)=>b[1]-a[1]).slice(0,22)));
 }
`+hook)};
});}});
await import(join(import.meta.dir,'living-silhouette-view.ts'));
