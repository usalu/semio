import {join} from "node:path";
import {readFileSync,existsSync} from "node:fs";
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const code=process.env.CODE??"old";
Bun.plugin({name:"oo",setup(b){b.onLoad({filter:/(?:inference\/[^/]*\/[^/]*|inputs\/[^/]*)\.mjs$/},async args=>{
 let t=args.path.includes("owner-discovery")&&code==="new"?readFileSync("/private/tmp/oo/owner-new.mjs","utf8"):await Bun.file(args.path).text();
 if(args.path.endsWith("🏗️build/📥️inputs/🟨️.mjs"))t=t.replace('repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"build-lock-closures")',`(process.getBuiltinModule("fs").mkdirSync("/private/tmp/oo/scratch/lock",{recursive:true}),"/private/tmp/oo/scratch/lock")`).replace('repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"build-input-closures")',`(process.getBuiltinModule("fs").mkdirSync("/private/tmp/oo/scratch/input",{recursive:true}),"/private/tmp/oo/scratch/input")`);
 return {contents:t,loader:"js"};});}});
const inference:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const admission:any=await import(join(L,"🏗️build/🕸️graph/🚪️admission/🟦️.ts"));
const registry=admission.readBuildOwnerRegistryV1(JSON.parse(inference.readBuildOwnerRegistrySourceV1(W)));
const limit=Number(process.argv[2]??"60");
const rows:{owner:any,targets:string[]}[]=[];
for(const owner of registry.owners){if(!owner.manifest.endsWith("project.json"))continue;let d:any;try{d=JSON.parse(readFileSync(join(W,owner.manifest),"utf8"));}catch{continue;}
 const t=Object.entries(d.targets??{}).filter(([n,v]:any)=>(v.outputs??[]).some((o:string)=>o.includes("{workspaceRoot}")));
 if(t.length)rows.push({owner,targets:Object.keys(d.targets)});}
console.log("candidates",rows.length,"of",registry.owners.length);
let ok=0,refused=0;const t0=performance.now();
for(const row of rows.slice(0,limit)){const s0=performance.now();try{await inference.discoverBuildOwnerAdmissionFactsV1(row.owner,row.targets,registry,{workspaceRoot:W},[]);ok++;}catch(e:any){refused++;console.log("REFUSED",row.owner.name,String(e.message).slice(0,160));}console.log("done",row.owner.name,Math.round(performance.now()-s0)+"ms");}
console.log(`code=${code} ok=${ok} refused=${refused} seconds=${Math.round((performance.now()-t0)/1000)}`);
