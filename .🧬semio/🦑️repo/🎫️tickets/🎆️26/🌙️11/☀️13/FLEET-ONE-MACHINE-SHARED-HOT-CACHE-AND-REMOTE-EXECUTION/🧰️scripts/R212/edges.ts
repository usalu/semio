import {readdirSync,lstatSync} from "node:fs";
import {join} from "node:path";
import {withArtifactRelativeDirectoryV1,takeArtifactRelativeNativeProfileV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
const root=process.argv[2]!;
const checkpoint=()=>{};
let entries=0,dirs=0,links=0;
const t0=performance.now();
const walk=async(directory:any)=>{
 const children:string[]=[];
 for(const name of await directory.names()){entries++;const entry=await directory.entry(name);if(!entry)throw Error("gone");if(entry.kind==="directory"){children.push(name);continue;}if(entry.kind==="link")links++;}
 dirs++;
 for(const name of children)await directory.withDirectory(name,(child:any)=>walk(child));
};
await withArtifactRelativeDirectoryV1(root,checkpoint,walk);
const t1=performance.now();
console.log("descriptor walk ms",(t1-t0).toFixed(0),"entries",entries,"dirs",dirs,"links",links,JSON.stringify(takeArtifactRelativeNativeProfileV1()));
let n=0,d=0;
const plain=(dir:string)=>{for(const e of readdirSync(dir,{withFileTypes:true})){n++;if(e.isDirectory()){d++;plain(join(dir,e.name));}}};
const t2=performance.now();plain(root);console.log("plain readdir withFileTypes ms",(performance.now()-t2).toFixed(0),"entries",n,"dirs",d);
