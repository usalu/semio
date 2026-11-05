import fs from "node:fs";
import path from "node:path";
import {fileURLToPath} from "node:url";

const ticket=path.dirname(fileURLToPath(import.meta.url)),generated=path.join(ticket,"🗑️generated"),fixture=path.join(generated,"cold"),source=path.resolve(ticket,"../../../../../../..");
const bounded=value=>{if(value.length>256)throw Error("Fixture path exceeds 256 UTF-16 units");return value;};
const facts=file=>{const row=fs.lstatSync(file,{bigint:true});return{kind:row.isSymbolicLink()?"link":row.isDirectory()?"directory":"file",stamp:[row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].map(String),target:row.isSymbolicLink()?fs.readlinkSync(file):null};};
const snapshot=()=>{const root=path.join(source,"node_modules"),names=fs.readdirSync(root).sort(),entries={};for(const name of names){const file=path.join(root,name);entries[name]=facts(file);if(name.startsWith("@")&&fs.statSync(file).isDirectory()){const children=fs.readdirSync(file).sort();if(children.length>100000)throw Error("Scoped namespace snapshot exceeds its entry budget");entries[name].children=Object.fromEntries(children.map(child=>[child,facts(path.join(file,child))]));}}return{root:facts(root),names,entries};};

try{
 fs.mkdirSync(generated,{recursive:true});
 const mode=process.argv[2];
 if(mode==="prepare"){
  if(fs.existsSync(fixture)){const row=fs.lstatSync(fixture);if(!row.isDirectory()||row.isSymbolicLink()||path.relative(fs.realpathSync(generated),fs.realpathSync(fixture))!=="cold")throw Error("Disposable fixture must stay inside ticket generated storage");fs.rmSync(fixture,{recursive:true,force:true});}
  const input=path.join(ticket,"cold");fs.cpSync(input,bounded(fixture),{recursive:true,errorOnExist:true,force:false,filter:file=>{const relative=path.relative(input,file),row=fs.lstatSync(file);if(row.isSymbolicLink()||relative.split(path.sep).includes("node_modules"))throw Error("Only authored cold fixture inputs may be copied");bounded(path.join(fixture,relative));return true;}});
  console.log("[DEBUG] Prepared disposable cold fixture: "+fixture);
 }else if(mode==="before"){
  fs.writeFileSync(path.join(generated,"cold-source-before.json"),JSON.stringify(snapshot()));console.log("[DEBUG] Captured original Source namespace stamp and roster");
 }else if(mode==="after"){
  const current=JSON.stringify(snapshot()),before=fs.readFileSync(path.join(generated,"cold-source-before.json"),"utf8");fs.writeFileSync(path.join(generated,"cold-source-after.json"),current);if(current!==before)throw Error("Original Source namespace changed during the isolated fixture run");console.log("[DEBUG] Original Source namespace stamp and roster remained unchanged");
 }else throw Error("Expected prepare, before or after");
}catch(error){console.error("[DEBUG] "+error.message);process.exitCode=1;}
