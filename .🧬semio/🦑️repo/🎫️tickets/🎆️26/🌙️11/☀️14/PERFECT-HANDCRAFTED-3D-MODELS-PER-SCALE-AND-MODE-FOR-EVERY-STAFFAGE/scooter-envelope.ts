import {readFileSync} from "node:fs";
import {join} from "node:path";
import {extentOf} from "./volumes-lib.ts";
import {rigAt,listsIn} from "./r3-lib.ts";
const path=join(import.meta.dir,"🗑️generated/r3-motor-scooter/🔣️.json");
const {asset}=JSON.parse(readFileSync(path,"utf8"));
for(const scale of [50,100,200,500]){
 const rig=rigAt(asset,scale,{kind:"pose",pose:asset.presentation.defaultPose});
 const meshes=listsIn(asset,scale,rig.frames);
 const ranges=meshes.body.map(m=>({id:m.id,...extentOf([m])!}));
 console.log(scale,"left",ranges.toSorted((a,b)=>a.min[0]-b.min[0]).slice(0,2),"right",ranges.toSorted((a,b)=>b.max[0]-a.max[0]).slice(0,2),"top",ranges.toSorted((a,b)=>b.max[1]-a.max[1]).slice(0,2));
}

