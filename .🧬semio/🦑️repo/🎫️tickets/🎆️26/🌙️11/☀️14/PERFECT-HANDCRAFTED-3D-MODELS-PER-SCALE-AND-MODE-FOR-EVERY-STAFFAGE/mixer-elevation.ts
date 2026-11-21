/** 🚚 Render the current mixer drawing over its body outline to locate projection differences. */
import {join} from "node:path";
import {entryOf,GENERATED} from "./volumes-lib.ts";
import {rigAt,meshesIn,savePng} from "./r3-lib.ts";
import {overlayPanel} from "./volumes-render.ts";
const {asset}=entryOf("machinery/concrete-mixer-truck");
const scale=50;
const solids=asset.volumes[scale].body;
const {frames}=rigAt(asset,scale,{kind:"pose",pose:asset.presentation.defaultPose});
const sm={body:meshesIn(asset,frames,solids,scale),solids:{body:solids}};
const png=overlayPanel(1080,460,asset,scale,"elevation",sm as never,{});
console.log(savePng(join(GENERATED,"equipment-check","mixer-elevation.png"),png));
