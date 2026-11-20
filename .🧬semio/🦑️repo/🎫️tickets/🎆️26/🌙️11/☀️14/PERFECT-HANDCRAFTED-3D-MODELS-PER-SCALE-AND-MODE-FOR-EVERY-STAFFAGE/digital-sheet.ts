/** 🗿 Catalog contact sheets from the shipping JSON in its presentation pose. */
import { join } from "node:path";
import { Canvas } from "./volumes-raster.ts";
import { bodyItems, thermalItems, structureItems } from "./volumes-render.ts";
import { catalogEntries, loadAsset, GENERATED } from "./volumes-lib.ts";
import { Panel, cameraAt, listsIn, paperBox, pointsOf, rigAt, savePng } from "./r3-lib.ts";
const [category, denominator, list="body"] = process.argv.slice(2);
const scale=Number(denominator);
const entries=catalogEntries().filter(e=>e.category===category);
const w=230,h=155;
const sheet=new Canvas(w*5,(h+20)*Math.ceil(entries.length/5),[255,255,255]);
for(const [i,entry] of entries.entries()){
  const asset=loadAsset(entry.path), rig=rigAt(asset,scale,{kind:"pose",pose:asset.presentation.defaultPose}), sm=listsIn(asset,scale,rig.frames);
  const cam=cameraAt(42,21), bounds=paperBox(cam,pointsOf(sm.body));
  const items=(list==="thermal"?thermalItems:list==="structure"?structureItems:bodyItems)(asset,sm,asset.presentation.defaultScale);
  const place={cam,scale:Math.min((w-12)/(bounds.maxX-bounds.minX),(h-12)/(bounds.maxY-bounds.minY)),cx:(bounds.maxX+bounds.minX)/2,cy:(bounds.maxY+bounds.minY)/2,ox:w/2,oy:h/2};
  const panel=new Panel(w,h,[244,244,240],2);panel.draw(place,items,Infinity);panel.shade(place);
  const x=(i%5)*w,y=Math.floor(i/5)*(h+20);
  sheet.blit(panel.canvas(),x,y);sheet.text(x+4,y+h+3,entry.slug,[20,20,20],1);
  if(sm.errors.length||sm.unresolved.length)console.error(entry.slug,sm.errors,sm.unresolved);
}
savePng(join(GENERATED,`digital-${category}-${scale}-${list}.png`),sheet);
