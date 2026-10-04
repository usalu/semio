#!/usr/bin/env bun
/** 📇️ Publishes precise caller-owned Graph catalog contract fields with full byte inverses. */
import {existsSync,readFileSync,writeFileSync} from "node:fs";
import {resolve} from "node:path";
import {createHash} from "node:crypto";
import Ajv from "ajv";
import JSON5 from "json5";
const ticket=resolve(import.meta.dir,".."),root=resolve(ticket,"../../../../../../.."),input=(name:string)=>resolve(import.meta.dir,name);
const sha=(source:string)=>createHash("sha256").update(source).digest("hex"),read=(path:string)=>readFileSync(path,"utf8"),save=(name:string,value:unknown)=>writeFileSync(input(name),JSON.stringify(value,null,2)+"\n");
const parser=await import(resolve(root,"🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️catalog/🟦️.ts"));
const schema=JSON.parse(read(resolve(root,"🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧬️schema/🔣️.json")));
const validate=new Ajv({strict:true}).compile({...schema.$defs.Outputs,$defs:schema.$defs});
function observe(source:string):void{
 const value=JSON.parse(source);if(JSON.stringify(JSON5.parse(source))!==JSON.stringify(value))throw Error("Independent JSON parser disagrees");
 if(!validate(value))throw Error(JSON.stringify(validate.errors));parser.parseGraphOutputCatalog(value,value.manifests.map((row:{id:string})=>row.id));
 if(validate({...value,foreignField:true}))throw Error("Independent schema admits an unknown field");
 let refused=false;try{parser.parseGraphOutputCatalog({...value,foreignField:true},value.manifests.map((row:{id:string})=>row.id));}catch{refused=true;}if(!refused)throw Error("Runtime parser admits an unknown field");
}
const mode=process.argv[2];if(mode==="prepare"){
 const proposal=JSON.parse(read(input("proposed-caller-contract-oracle-proof.json"))).results as {path:string;before:Record<string,unknown>;authored:Record<string,unknown>;addedFields:string[]}[],frames=JSON.parse(read(input("actual-generate-consumers-full-current.json"))).rows as {path:string;before:string}[],rows=[];
 for(const candidate of proposal.filter(row=>row.addedFields.length)){
  const path=resolve(root,candidate.path),before=read(path);if(before!==frames.find(row=>row.path===candidate.path)?.before)throw Error("Catalog advanced before scoped admission: "+candidate.path);
  const old=JSON.parse(before),additions=Object.fromEntries(candidate.addedFields.map(key=>[key,candidate.authored[key]]));
  const schemaMatch=/"\$schema"\s*:\s*("(?:[^"\\]|\\.)*")/u.exec(before);if(!schemaMatch)throw Error("Catalog has no schema authority");
  const originalSchema=JSON.parse(schemaMatch[1]!),exactSchema=originalSchema.split("#")[0]+"#/$defs/Outputs";
  const start=schemaMatch.index+schemaMatch[0].indexOf(schemaMatch[1]!),schemaBody=before.slice(0,start)+JSON.stringify(exactSchema)+before.slice(start+schemaMatch[1]!.length);
  const boundary=schemaBody.indexOf("\n",start);if(boundary<0)throw Error("Catalog lacks a field insertion boundary");
  const authoredFields=Object.entries(additions).map(([key,value])=>"  "+JSON.stringify(key)+": "+JSON.stringify(value)+",\n").join("");
  const after=schemaBody.slice(0,boundary+1)+authoredFields+schemaBody.slice(boundary+1),value=JSON.parse(after);
  for(const [key,body] of Object.entries(old))if(key!=="$schema"&&JSON.stringify(value[key])!==JSON.stringify(body))throw Error("Existing field changed: "+key);
  let inverse=after.replace(authoredFields,"");const newMatch=/"\$schema"\s*:\s*("(?:[^"\\]|\\.)*")/u.exec(inverse)!;const inverseStart=newMatch.index+newMatch[0].indexOf(newMatch[1]!);inverse=inverse.slice(0,inverseStart)+JSON.stringify(originalSchema)+inverse.slice(inverseStart+newMatch[1]!.length);
  if(inverse!==before)throw Error("Full scoped inverse differs");observe(after);rows.push({path:candidate.path,before,after,inverse,addedFields:candidate.addedFields,shaBefore:sha(before),shaAfter:sha(after),schemaBefore:originalSchema,schemaAfter:exactSchema});
 }
 if(rows.length!==6)throw Error("Expected exactly six independently diagnosed callers");save("root-six-catalog-ready-1.json",{at:new Date().toISOString(),rows,sourceWrites:0,independentOracle:"Ajv and JSON5",nativeExecuted:false});console.log(JSON.stringify({ready:rows.length,sourceWrites:0}));
}else if(mode==="mount"){
 const ready=JSON.parse(read(input("root-six-catalog-ready-1.json"))),rows=[];if(existsSync(input("root-six-catalog-mounted-1.json")))throw Error("Catalog publication already has a receipt");
 for(const row of ready.rows){const path=resolve(root,row.path);if(read(path)!==row.before)throw Error("Catalog changed at immediate publication guard: "+row.path);writeFileSync(path,row.after);const current=read(path);if(current!==row.after)throw Error("Catalog changed at post-publication guard: "+row.path);observe(current);rows.push({...row,current,shaCurrent:sha(current)});save("root-six-catalog-publication-journal-1.json",{at:new Date().toISOString(),rows,sourceWrites:rows.length,nativeExecuted:false});}
 save("root-six-catalog-mounted-1.json",{at:new Date().toISOString(),rows,sourceWrites:rows.length,ownedGaps:0,nativeExecuted:false});console.log(JSON.stringify({mounted:rows.length,ownedGaps:0,oracles:rows.length,nativeExecuted:false}));
}else if(mode==="verify"){
 const mounted=JSON.parse(read(input("root-six-catalog-mounted-1.json")));for(const row of mounted.rows){const current=read(resolve(root,row.path));if(current!==row.after)throw Error("Mounted owned body advanced: "+row.path);observe(current);}console.log(JSON.stringify({current:mounted.rows.length,ownedGaps:0,nativeExecuted:false}));
}else throw Error("Expected prepare, mount or verify");
