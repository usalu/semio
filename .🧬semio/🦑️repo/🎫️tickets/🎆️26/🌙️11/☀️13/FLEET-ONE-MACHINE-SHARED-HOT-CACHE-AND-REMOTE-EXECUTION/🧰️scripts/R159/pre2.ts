import {plugin} from "bun";
const dump=process.env.R159_DUMP;
if(dump)plugin({name:"r159-key-dump",setup(b){
 b.onLoad({filter:/project-records\/[^/]+\.ts$/},async({path})=>{
  if(path.includes("/tests/")||path.includes("portable")||path.includes("/packed-rows/"))return undefined;
  const text=await Bun.file(path).text(),anchor='return Object.freeze({version:1,frame:Object.freeze({kind:"project-inference" as const';
  if(!text.includes(anchor))return undefined;
  const add=`try{(process.getBuiltinModule("node:fs") as any).appendFileSync(${JSON.stringify(dump)}+"."+(process.env.SEMIO_SLICE_LAB_MACHINE??"x"),JSON.stringify({pid:process.pid,keyText:neutral(input.keyText),fleet:neutral(input.fleet),platform,digest})+"\\n");}catch{}`;
  return {contents:text.replace(anchor,add+anchor),loader:"ts"};
 });
}});
