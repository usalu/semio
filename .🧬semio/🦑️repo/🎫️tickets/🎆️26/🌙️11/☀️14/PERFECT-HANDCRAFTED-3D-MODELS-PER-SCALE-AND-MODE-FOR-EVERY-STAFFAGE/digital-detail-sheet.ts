/** 🪟 Technical assembly views of current shipped staffage in its working poses. */
import {join} from 'node:path';
import {Canvas} from './volumes-raster.ts';
import {bodyItems} from './volumes-render.ts';
import {GENERATED,catalogEntries,loadAsset} from './volumes-lib.ts';
import {Panel,cameraAt,listsIn,paperBox,pointsOf,rigAt,savePng} from './r3-lib.ts';
const entries=catalogEntries();
const views=[
 ['market-parasol','open-still','Open canopy',40,22,''],
 ['market-parasol','furled','Furled canopy',40,22,''],
 ['bookshelf','loaded','Loaded bookshelf',-35,17,''],
 ['storage-crate-stack','','Stacked moulded crates',42,21,''],
 ['kitchen-extractor-hood','extracting','Chimney hood',35,17,''],
 ['kitchen-extractor-hood','extracting','Centrifugal blower wheel',-134,19,'wheel-|blower-motor'],
] as const;
const w=300,h=242,caption=20,sheet=new Canvas(w*3,(h+caption)*2,[255,255,255]);
for(const [i,[name,pose,title,turn,tilt,pattern]] of views.entries()){
 const asset=loadAsset(entries.find(e=>e.slug===name)!.path);
 const rig=rigAt(asset,50,{kind:'pose',pose:pose||asset.presentation.defaultPose}),sm=listsIn(asset,50,rig.frames);
 if(sm.errors.length||sm.unresolved.length)throw Error(name+' failed tessellation');
 const keep=(id:string)=>!pattern||new RegExp(pattern).test(id);
 const cam=cameraAt(turn,tilt),bounds=paperBox(cam,pointsOf(sm.body.filter(m=>keep(m.id))));
 const items=bodyItems(asset,sm,50).filter(item=>keep(item.mesh.id));
 const place={cam,scale:Math.min((w-22)/(bounds.maxX-bounds.minX),(h-15)/(bounds.maxY-bounds.minY)),cx:(bounds.minX+bounds.maxX)/2,cy:(bounds.minY+bounds.maxY)/2,ox:w/2,oy:h/2};
 const panel=new Panel(w,h,[241,242,238],1);panel.draw(place,items,Infinity);panel.shade(place);
 const x=i%3*w,y=Math.floor(i/3)*(h+caption);sheet.blit(panel.canvas(),x,y);sheet.text(x+6,y+h+3,title,[28,34,32],1);
}
savePng(join(GENERATED,'digital-detail-sheet.png'),sheet);
