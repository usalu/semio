import {mkdtempSync,realpathSync,statSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {withArtifactRelativeDirectoryV1} from "../../../../../../../../🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";

/** 🔬️ Proves the relative-create write ceiling the generation publisher exceeds: 262144-byte chunks fail, 65536-byte chunks succeed. */
const root=realpathSync(mkdtempSync(join(tmpdir(),"x-ingress-")));
const bytes=Buffer.alloc(300000,97);
const publish=(name:string,chunk:number)=>withArtifactRelativeDirectoryV1(root,()=>{},async base=>{
  await base.withNewFile(name,async handle=>{let offset=0;while(offset<bytes.length){offset+=handle.write(bytes,offset,Math.min(chunk,bytes.length-offset),offset);}handle.sync();});
});
const attempt=async(name:string,chunk:number)=>{
  try{await publish(name,chunk);return {name,chunk,error:null,size:statSync(join(root,name)).size};}
  catch(error){return {name,chunk,error:String((error as Error).message),size:statSync(join(root,name)).size};}
};
console.log(JSON.stringify([await attempt("big.bin",262144),await attempt("ok.bin",65536)],null,1));
console.log(readdirSync(root));
rmSync(root,{recursive:true,force:true});
