import {readFileSync} from "node:fs";
import {transform,FRAMEWORK} from "./seam.mjs";
const JSON_PATCHES:[RegExp,(doc:any)=>void][]=[[/resident\/🧬️schema\/🔣️\.json$/u,doc=>{const m=doc.definitions.Policy.const.messages;m.en["host-plan-roster"]??="Planning all targets.";m.de["host-plan-roster"]??="Plane alle Ziele.";}]];
Bun.plugin({name:"r133-seam",setup(b:any){
 b.onLoad({filter:/(🟨️\.mjs|🟦️\.ts|📜️\.ts|script\.ts|\.mjs)$/u},(args:any)=>{const text=readFileSync(args.path,"utf8");return{contents:args.path.startsWith(FRAMEWORK)?transform(args.path,text):text,loader:/\.(ts|mts)$/u.test(args.path)?"ts":"js"};});
 b.onLoad({filter:/🧬️schema\/🔣️\.json$/u},(args:any)=>{const doc=JSON.parse(readFileSync(args.path,"utf8"));for(const[re,fn]of JSON_PATCHES)if(re.test(args.path))fn(doc);return{exports:{...doc,default:doc},loader:"object"};});
}});
