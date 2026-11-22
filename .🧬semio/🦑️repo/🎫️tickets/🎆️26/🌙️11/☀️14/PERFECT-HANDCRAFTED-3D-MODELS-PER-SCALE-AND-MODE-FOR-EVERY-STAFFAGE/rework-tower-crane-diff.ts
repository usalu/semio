/** 🩻 Writes the silhouette comparison of one view as a picture (grey = drawing and body agree, red = drawn only, blue = body only) in the grid of the product rule.
 *
 * Usage: bun rework-tower-crane-diff.ts <slug> <scale> <view> <asset json> <result json> --ppm <out.ppm> [--window x0,y0,x1,y1]
 */
import {join} from 'node:path';
import {readFileSync} from 'node:fs';
const at=(flag:string)=>{const i=process.argv.indexOf(flag);return i<0?undefined:process.argv[i+1];};
Bun.plugin({name:'silhouette-picture',setup(b){b.onLoad({filter:/🌓️silhouette\/🟦️\.ts$/},args=>{
 const s=readFileSync(args.path,'utf8'),hook='  const spread = boundarySpread(drawnMask, reference3d';
 if(!s.includes(hook))throw Error('Raster hook moved');
 return {loader:'ts',contents:s.replace(hook,`  if(window===undefined){
  const at=(flag)=>{const i=process.argv.indexOf(flag);return i<0?undefined:process.argv[i+1];};
  const w=volume.width,h=drawnMask.length/w;
  const win=(at('--window')??[box.minX,box.minY,box.minX+w*cell,box.minY+h*cell].join(',')).split(',').map(Number);
  const c0=Math.max(0,Math.floor((win[0]-box.minX)/cell)),c1=Math.min(w,Math.ceil((win[2]-box.minX)/cell));
  const r0=Math.max(0,Math.floor((win[1]-box.minY)/cell)),r1=Math.min(h,Math.ceil((win[3]-box.minY)/cell));
  const up=Math.max(1,Math.floor(2400/Math.max(c1-c0,(r1-r0)*1.5)));
  const W=(c1-c0)*up,H=(r1-r0)*up,data=new Uint8Array(W*H*3);
  let both=0,excess=0,missing=0;
  for(let r=r0;r<r1;r++)for(let c=c0;c<c1;c++){
   const ink=drawnMask[r*w+c],mesh=reference3d[r*w+c];
   if(ink&&mesh)both++;else if(ink)excess++;else if(mesh)missing++;
   const colour=ink&&mesh?[150,150,150]:ink?[230,30,30]:mesh?[30,60,230]:[250,250,246];
   for(let dy=0;dy<up;dy++)for(let dx=0;dx<up;dx++){const k=((H-1-((r-r0)*up+dy))*W+(c-c0)*up+dx)*3;data[k]=colour[0];data[k+1]=colour[1];data[k+2]=colour[2];}
  }
  const zones=new Map();
  for(let k=0;k<drawnMask.length;k++)if(drawnMask[k]!==reference3d[k]){
   const x=box.minX+(k%w+.5)*cell,y=box.minY+(Math.floor(k/w)+.5)*cell;
   const key=Math.floor(x)+'/'+Math.floor(y)+'/'+(drawnMask[k]?'excess':'missing');
   zones.set(key,(zones.get(key)??0)+1);
  }
  console.log('[DEBUG] zones (x m/y m/kind, cells)',JSON.stringify([...zones].sort((a,b)=>b[1]-a[1]).slice(0,40)));
  const header=new TextEncoder().encode('P6 '+W+' '+H+' 255 ');
  const file=new Uint8Array(header.length+data.length);file.set(header);file.set(data,header.length);
  require('node:fs').writeFileSync(at('--ppm'),file);
  console.log('[DEBUG]',view,'cell',cell.toFixed(4),'box',box.minX.toFixed(2),box.minY.toFixed(2),'size',w,h,'window cells',c1-c0,r1-r0,'x'+up,'both',both,'excess',excess,'missing',missing);
 }
`+hook)};
});}});
await import(join(import.meta.dir,'living-silhouette-view.ts'));
