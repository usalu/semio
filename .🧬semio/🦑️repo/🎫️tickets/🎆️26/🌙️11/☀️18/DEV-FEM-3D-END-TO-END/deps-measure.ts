import {lstatSync,readdirSync} from "node:fs";
import {join} from "node:path";

const walk=(directory:string,total:{allocated:number;logical:number;files:number})=>{
 let names:string[]=[];
 try{names=readdirSync(directory);}catch{return;}
 for(const name of names){
  const path=join(directory,name);
  try{
   const stat=lstatSync(path,{bigint:true});
   total.allocated+=Number(stat.blocks??0n)*512;total.logical+=Number(stat.size);total.files++;
   if(stat.isDirectory())walk(path,total);
  }catch{}
 }
};
for(const root of process.argv.slice(2)){
 const total={allocated:0,logical:0,files:0};walk(root,total);
 console.log(JSON.stringify({root,allocatedMiB:Math.round(total.allocated/1048576),logicalMiB:Math.round(total.logical/1048576),files:total.files}));
}
