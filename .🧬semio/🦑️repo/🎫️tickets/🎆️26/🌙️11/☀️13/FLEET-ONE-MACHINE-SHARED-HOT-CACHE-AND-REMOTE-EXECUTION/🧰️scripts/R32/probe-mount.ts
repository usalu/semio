import {withArtifactRelativeDirectoryV1} from "/Users/Shared/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
for(let i=0;i<3;i++){
  try{console.log("ok",JSON.stringify(await withArtifactRelativeDirectoryV1(process.argv[2]!,()=>{},async a=>a.withDirectory("g",async c=>[await c.entry(".reset.json"),(await c.entry("m"))?.kind]))));}
  catch(e:any){console.log("ERR",e.message,e.code,e.entry);}
}
