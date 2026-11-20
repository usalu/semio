import {readFileSync,writeFileSync} from "node:fs";
const root="C:/semio/",ticket=root+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️18/DEV-FEM-3D-END-TO-END/🗑️generated/";
const files=readFileSync(ticket+"users.txt","utf8").split(/\r?\n/).filter(f=>f&&!f.endsWith(".lock")&&!f.includes("🗑️generated")&&!f.endsWith(".toml"));
const p="semio_s_artifact_fem_3d_model::standards::v1::subsets::any::";
const rules:[RegExp,string][]=[
 [new RegExp(p.replaceAll("::","::")+"schema::guards","g"),"semio_s_artifact_fem_3d_guards::standards::v1::subsets::any::schema::guards"],
 [new RegExp(p+"schema::diff","g"),"semio_s_artifact_fem_3d_diff::standards::v1::subsets::any::schema::diff"],
 [new RegExp(p+"io::text::diff","g"),"semio_s_artifact_fem_3d_diff::standards::v1::subsets::any::io::text::diff"],
 [new RegExp(p+"io::binary::diff","g"),"semio_s_artifact_fem_3d_diff::standards::v1::subsets::any::io::binary::diff"]];
const changed:string[]=[];
for(const rel of files){const path=root+rel.replaceAll("\\","/");let text:string;try{text=readFileSync(path,"utf8");}catch{continue;}let next=text;for(const[re,to]of rules)next=next.replace(re,to);if(next!==text){writeFileSync(path,next);changed.push(rel);}}
writeFileSync(ticket+"changed.txt",changed.join("\n"));console.log(changed.length);
