/** 🌳 Locate the hard-surface outline errors in world coordinates on every measured tile. */
import {readFileSync} from 'node:fs';
import {join} from 'node:path';
const buckets=new Map<string,number>();
globalThis.locateBirch=(x,y,side)=>{const k=[Math.floor(x/.15),Math.floor(y/.25),side].join('/');buckets.set(k,(buckets.get(k)??0)+1);};
Bun.plugin({name:'birch-outline-location',setup(b){b.onLoad({filter:/🌓️silhouette\/🟦️\.ts$/},args=>{
 const s=readFileSync(args.path,'utf8');return {loader:'ts',contents:s.replace('  const spread = boundarySpread(drawnMask','  globalThis.tileOrigin=[box.minX,box.minY];\n  const spread = boundarySpread(drawnMask').replace('else hard[bin]!++;','else {hard[bin]!++; if(d*cell>.03 && globalThis.tileOrigin)globalThis.locateBirch(globalThis.tileOrigin[0]+(k%width+.5)*cell,globalThis.tileOrigin[1]+(Math.floor(k/width)+.5)*cell,from===ba?"drawing":"wood");}')}
});}});
await import(join(import.meta.dir,'living-silhouette-view.ts'));
console.log([...buckets].sort((a,b)=>b[1]-a[1]).slice(0,30));
