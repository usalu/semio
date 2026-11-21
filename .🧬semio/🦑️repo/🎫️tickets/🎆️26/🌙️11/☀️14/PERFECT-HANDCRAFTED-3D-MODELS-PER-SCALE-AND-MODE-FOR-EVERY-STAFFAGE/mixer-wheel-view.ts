/** 🛞 Render the front steering tyre, pressed disc and flanged nuts at their actual mount positions. */
import {join} from "node:path";
import {Canvas} from "./volumes-raster.ts";
import {drawAxon,fit} from "./volumes-render.ts";
import {cameraAt,meshesIn,rigAt,savePng} from "./r3-lib.ts";
import {entryOf,GENERATED} from "./volumes-lib.ts";
const {asset}=entryOf("machinery/concrete-mixer-truck");
const parts=asset.volumes["50"].body.filter((p:any)=>/^(tyre|rim|wheel-disc|hub|wheel-nuts)-1-r$/.test(p.id));
const state=rigAt(asset,50,{kind:"pose",pose:"parked-off"});
const meshes=meshesIn(asset,state.frames,parts,50);
const view=new Canvas(560,500,[243,243,238]);
const ax=fit(meshes,cameraAt(32,17),560,500,.065);
drawAxon(view,ax,meshes.map((mesh,i)=>({mesh,color:(parts[i].id.startsWith("tyre")?[74,76,75]:parts[i].id.startsWith("wheel-nuts")?[180,165,132]:[165,172,171]) as [number,number,number]})));
console.log(savePng(join(GENERATED,"equipment-check","mixer-wheel.png"),view));
