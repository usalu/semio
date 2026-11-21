/** 🖨️ Measure one authored print band at one scale using the public office-laser profile. */
import {readFileSync} from "node:fs";
import {DEFAULT_PRINT_PROFILE,measureSuitability} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const [file,band,den,pose]=process.argv.slice(2);
const {asset}=JSON.parse(readFileSync(file,"utf8"));
if(pose&&!asset.poses.some((p:any)=>p.id===pose&&!p.key))throw new Error("Unknown drawing pose");
const probe=pose?{...asset,poses:asset.poses.map((p:any)=>p.id===pose||p.key?p:{...p,key:true})}:asset;
const result=measureSuitability(probe,probe.representations[band],Number(den),DEFAULT_PRINT_PROFILE);
console.log(JSON.stringify({id:asset.id,band,den,pose,issues:result.violations}));
if(!result.suitable)process.exitCode=1;
