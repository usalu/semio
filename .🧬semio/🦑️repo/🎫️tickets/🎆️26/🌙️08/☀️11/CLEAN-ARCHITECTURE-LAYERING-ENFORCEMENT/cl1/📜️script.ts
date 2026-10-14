import {readFileSync,mkdirSync,writeFileSync} from "node:fs";
import {resolve,join} from "node:path";
import {isDeepStrictEqual} from "node:util";
import {createHash} from "node:crypto";
import Ajv from "ajv";

const root=process.cwd(),ticket=resolve(import.meta.dir,".."),directory=join(ticket,"🗑️generated/cl1"),base=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),schemaPath=join(base,"🏗️build/🧬️schema/📥️authoring.json"),schema=JSON.parse(readFileSync(schemaPath,"utf8"));
const {validateJsonSchemaSubset}=await import(join(root,"🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts")),{readBuildAuthoringCapabilitiesV1}=await import(join(base,"🏗️build/📥️caller/🟦️.ts"));
const mirror=JSON.parse(readFileSync(join(root,".vscode/launch.json"),"utf8")),seed=JSON.parse(readFileSync(join(root,".vscode/🧩️launch.seed.jsonc"),"utf8")),owners=new Set(["process:exact-cargo-storage:caller","repo:exact-cargo:caller","repo:rust-source:caller"]),selected=mirror.configurations.filter((row:any)=>row&&typeof row==="object"&&owners.has(JSON.parse(row.env?.SEMIO_SCRIPT_POLICY??"null")?.owner)),validate=new Ajv({strict:true}).compile(schema),observations:unknown[]=[];
if(selected.length!==6)throw Error("Six direct Cargo Source launch callers required");
let assertions=0;const check=(condition:boolean,message:string)=>{assertions++;if(!condition)throw Error(message);};
for(const row of selected){
 const matches=seed.configurations.filter((value:any)=>value&&typeof value==="object"&&value.name===row.name);check(matches.length===1&&isDeepStrictEqual(matches[0],row),"Seed/mirror caller parity refused");
 const args:string[]=row.command.split(" ").slice(2),raw=JSON.parse(row.env.SEMIO_SCRIPT_CAPABILITIES),policy=JSON.parse(row.env.SEMIO_SCRIPT_POLICY);check(validate(raw)===true&&validateJsonSchemaSubset(schema,raw).length===0,"Independent authoring schema parity refused");
 const admitted=readBuildAuthoringCapabilitiesV1(root,args,"nx",raw);check(admitted.workspaceRoot===root&&admitted.native.artifactDirectory===resolve(root,raw.native.artifactDirectory),"Actual original authoring reader changed selection");check(policy.maximumElapsedMilliseconds<=600000&&admitted.native.child.maximumElapsedMilliseconds===600000,"Original finite limits refused");
 for(const specimen of [{...raw,storage:{}},{...raw,native:{...raw.native,network:{offline:"false"}}}])check(validate(specimen)===false&&validateJsonSchemaSubset(schema,specimen).length>0,"Malformed authoring admitted by schema oracle");
 let refused=false;try{readBuildAuthoringCapabilitiesV1(root,[...args,"--foreign"],"nx",raw);}catch{refused=true;}check(refused,"Changed direct selected command admitted");
 observations.push({name:row.name,command:row.command,owner:policy.owner,maximumElapsedMilliseconds:policy.maximumElapsedMilliseconds,artifactDirectory:admitted.native.artifactDirectory});
}
mkdirSync(directory,{recursive:false});writeFileSync(join(directory,"result.json"),JSON.stringify({kind:"isolated-source-schema-reader-oracle-not-factory-or-runtime",assertions,observations,schemaSha256:createHash("sha256").update(readFileSync(schemaPath)).digest("hex")},null,2)+"\n",{flag:"wx"});console.log("[DEBUG] Direct Cargo launch schema-reader/oracle assertions "+assertions+"; no factory/frontend/Store/native executed");
