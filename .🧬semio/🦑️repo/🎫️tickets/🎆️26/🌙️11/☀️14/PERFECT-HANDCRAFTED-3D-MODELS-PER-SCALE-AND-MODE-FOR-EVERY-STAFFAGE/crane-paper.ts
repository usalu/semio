/** 🖨️ Recheck the changed print band one named pose and one page scale at a time. */
import {readFileSync} from "node:fs";
import {measureSuitability,DEFAULT_PRINT_PROFILE} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const [file,pose,end]=process.argv.slice(2);
const {asset}=JSON.parse(readFileSync(file,"utf8"));
asset.poses=asset.poses.map((p: {id:string;key?:boolean})=>({...p,key:p.id!==pose}));
const result=measureSuitability(asset,asset.representations["50"],Number(end),DEFAULT_PRINT_PROFILE);
console.log(JSON.stringify({pose,end,issues:result.violations}));
if (!result.suitable) process.exitCode=1;
