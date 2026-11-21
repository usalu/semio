/** 🏗️ Measures the tower-crane elevation at the same resolution as the production check. */
import {readFileSync} from "node:fs";
import {evaluate} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import {measureView, worldSolids} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const asset=JSON.parse(readFileSync(process.argv[2],"utf8")).asset;
const state=evaluate(asset,{kind:"pose",pose:asset.presentation.defaultPose},{kind:"scale",scale:{denominator:50}});
const solids=worldSolids(asset,state.bones as never,50).filter(s=>!s.texture);
const result=measureView(asset,state,solids,"elevation",50,undefined,.0075);
console.log(JSON.stringify({id:asset.id,p95M:result.p95M,iou:result.iou,hausdorffM:result.hausdorffM}));
