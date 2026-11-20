import {withArtifactRelativeDirectoryV1,takeArtifactRelativeNativeProfileV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
const root=process.argv[2]!;
let entries=0,dirs=0,links=0;
const t0=performance.now();
const walk=async(directory:any)=>{
 const children:string[]=[];
 for(const row of await directory.scan()){entries++;if(row.kind==="directory"){children.push(row.name);continue;}if(row.kind==="link")links++;}
 dirs++;
 for(const name of children)await directory.withDirectory(name,(child:any)=>walk(child));
};
await withArtifactRelativeDirectoryV1(root,()=>{},walk);
console.log("scan walk ms",(performance.now()-t0).toFixed(0),"entries",entries,"dirs",dirs,"links",links,JSON.stringify(takeArtifactRelativeNativeProfileV1()));
