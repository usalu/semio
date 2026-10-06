import{resolve}from"node:path";
import Ajv from"ajv";
const ticket=resolve(import.meta.dir,"../.."),root=resolve(ticket,"../../../../../../..");
const artifacts=["🌀️procedural/🗿️artifacts/🌀️generation2d","🌀️procedural/🗿️artifacts/🧊️generation3d","🎞️animate/🗿️artifacts/🎬️presentation","🎬️sequence/🗿️artifacts/🎬️sequence","🪵️sourcing/🗿️artifacts/🗂️curation"];
const roots:string[]=[];
for(const artifact of artifacts){
 const base=resolve(root,"✏️s/🔌️plugins",artifact,"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
 const corpus=await Bun.file(resolve(base,"🧫️fixtures/🪶️sqlite/🚦️public/🔣️.json")).json(),schema=await Bun.file(resolve(base,"🧫️fixtures/🪶️sqlite/🚦️public/🧬️schema/🔣️.json")).json();
 const check=new Ajv({strict:true}).compile(schema);if(!check(corpus))throw Error(JSON.stringify(check.errors));
 for(const key of ["schema_version","standard","artifact_kind"]){const bad=structuredClone(corpus);delete bad.metadata[key];if(check(bad))throw Error("metadata omission accepted");}
 for(const changes of [(c:any)=>c.metadata.schema_version=2,(c:any)=>c.metadataColumns.reverse(),(c:any)=>c.retirement="implicit",(c:any)=>c.domainTables.push("snapshot_blob_carrier")]){const bad=structuredClone(corpus);changes(bad);if(check(bad))throw Error("false public corpus admitted");}
 console.log("[DEBUG] Closed language-neutral public witness schema +7 negative admissions verified "+artifact);
 if(artifact.includes("generation"))for(const file of ["🟦️.ts","🪶️sqlite/🟦️.ts","🧪️tests/🪶️sqlite/🟦️.ts"])roots.push(resolve(base,file));
}
const child=Bun.spawn([process.execPath,resolve(root,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--skipLibCheck",...roots],{cwd:root,stdout:"inherit",stderr:"inherit"});const exit=await child.exited;if(exit)throw Error("Actual Generation Source strict public types failed "+exit);
console.log("[DEBUG] Generation2d/3d actual Source public Snapshot facade/provider/test strict types verified");

