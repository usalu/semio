import {join} from "node:path";
import {Canvas,hashHue,hsl} from "./volumes-raster.ts";
import {GENERATED,entryOf} from "./volumes-lib.ts";
import {listsIn,rigAt,cameraAt,savePng} from "./r3-lib.ts";
import {fit,drawAxon,PART_COLOR} from "./volumes-render.ts";

const assets = [
  "components/glass-balustrade","components/tilt-window","components/panel-radiator",
  "machinery/evaporative-humidifier","machinery/finned-heat-exchanger","machinery/plate-heat-exchanger",
  "machinery/expansion-valve","machinery/scroll-compressor","machinery/sorption-wheel","machinery/scissor-lift",
  "vehicles/city-bicycle","vehicles/electric-kick-scooter",
];
const cam = cameraAt(35,20);
for (let k=0;k<2;k++) {
  const sheet = new Canvas(750,1068);
  for (let row=0;row<6;row++) {
    const spec=assets[row+6*k]!;
    const {asset} = entryOf(spec);
    const rig=rigAt(asset,500,{kind:"pose",pose:asset.presentation.defaultPose});
    const mesh=listsIn(asset,500,rig.frames);
    if(mesh.errors.length) throw new Error(mesh.errors.join("\n"));
    const ax=fit(mesh.body,cam,244,154,.08);
    for (const [column,list] of (["body","thermal","structure"] as const).entries()) {
      const frame=new Canvas(244,154,[245,245,242]);
      const items=mesh[list].map((shape,i)=>{
        const part=mesh.solids[list][i]!;
        const color=list==="body"?hsl(hashHue(part.material),.11,.43+part.tone*.37):list==="structure"?PART_COLOR[part.part]:hsl(hashHue(part.region),.63,.59);
        return {mesh:shape,color};
      });
      drawAxon(frame,ax,items);
      sheet.blit(frame,column*250,row*178+18);
      sheet.text(column*250+3,row*178+4,column===0?spec.split("/")[1]!:list,[40,40,40],1);
    }
  }
  console.log(savePng(join(GENERATED,"equipment-check","digital-"+k+".png"),sheet));
}

