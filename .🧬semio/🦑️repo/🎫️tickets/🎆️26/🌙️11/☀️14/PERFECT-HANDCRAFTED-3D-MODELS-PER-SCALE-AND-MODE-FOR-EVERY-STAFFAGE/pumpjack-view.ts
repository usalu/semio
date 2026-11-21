import {join} from 'node:path';
import {Canvas} from './volumes-raster.ts';
import {GENERATED,entryOf} from './volumes-lib.ts';
import {rigAt,meshesIn,cameraAt,savePng} from './r3-lib.ts';
import {fit,drawAxon} from './volumes-render.ts';
const {asset}=entryOf('machinery/oil-pumpjack');
const solids=asset.volumes['50'].body.filter((s:any)=>/^(horsehead|bridle-rope|carrier|rod-clamp|polished-rod|wellhead|stuffing-box|well-casing)/.test(s.id));
const camera=cameraAt(32,8),sheet=new Canvas(720,480,[248,248,245]);
const rows=['stopped','mid-stroke','bottom-of-stroke'].map(pose=>meshesIn(asset,rigAt(asset,50,{kind:'pose',pose}).frames,solids,50));
const projection=fit(rows.flat(),camera,240,480,.07);
for(let i=0;i<rows.length;i++){
 const frame=new Canvas(240,480,[248,248,245]);
 const items=rows[i]!.map((mesh,j)=>({mesh,color:(solids[j].id.startsWith('bridle')?[172,105,48]:solids[j].id==='polished-rod'?[125,161,177]:solids[j].id.startsWith('horsehead')?[70,82,92]:[134,138,136]) as [number,number,number]}));
 drawAxon(frame,projection,items);sheet.blit(frame,240*i,0);
}
savePng(join(GENERATED,'equipment-check','pumpjack-stroke.png'),sheet);
