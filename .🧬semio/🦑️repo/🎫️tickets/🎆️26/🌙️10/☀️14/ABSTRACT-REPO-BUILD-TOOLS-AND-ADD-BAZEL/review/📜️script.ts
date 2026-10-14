import {readdir,readFile,writeFile,stat,mkdir} from "node:fs/promises";
import {dirname,join,relative,resolve} from "node:path";

const root=process.env.SEMIO_WORKSPACE_ROOT;
if(!root)throw Error("Workspace root required");
const ticket=resolve(dirname(import.meta.filename),".."),prefix=relative(root,ticket).replaceAll("\\","/"),files=new Set<string>();
const exists=async(path:string)=>{try{await stat(path);return true;}catch{return false;}};
const retain=(path:unknown)=>{if(typeof path!=="string"||!path||path.includes("🗑️generated")||path.startsWith("/"))return;files.add(path.replaceAll("\\","/"));};
for(const name of ["📥️files.json","📥️root-files.json","📥️workspace-files.json","📥️bazel-files.json","📥️bootstrap-files.json","📥dashboard-files.json","📥️artifact-marker-files.json"]){
 const value=JSON.parse(await readFile(join(ticket,name),"utf8"));
 if(Array.isArray(value))value.forEach(retain);
 else{
  for(const key of ["created","edited","removed","files"])if(Array.isArray(value[key]))value[key].forEach(retain);
  for(const move of value.moved??[]){retain(move.from);retain(move.to);}
  for(const move of value.assetMoves??[]){retain(move.old);retain(move.new);}
  for(const input of value.ticketInputs??[])retain(input.startsWith(prefix+"/")?input:prefix+"/"+input);
 }
}
const core=await readFile(join(ticket,"build-core.md"),"utf8");
for(const match of core.matchAll(/`([^`\n]+)`/gu))if(match[1]!.startsWith("🧰️framework/")&&await exists(join(root,match[1]!)))retain(match[1]);
for(const line of core.split("\n").filter(line=>line.startsWith("- Created and then removed before final implementation:")))for(const match of line.matchAll(/`([^`\n]+)`/gu))if(match[1]!.startsWith("🧰️framework/"))retain(match[1]);
const visit=async(directory:string):Promise<void>=>{for(const entry of await readdir(directory,{withFileTypes:true})){if(entry.name==="🗑️generated")continue;const path=join(directory,entry.name);if(entry.isDirectory())await visit(path);else if(entry.isFile())retain(relative(root,path));}};
await visit(ticket);
for(const invalid of ["inventory/📜️script.ts","stdio/📜️script.ts"])files.delete(invalid);
const paths=[...files].sort(),missing:string[]=[],overlength:string[]=[],source=paths.filter(path=>!path.startsWith(prefix+"/"));
for(const path of paths){if(!await exists(join(root,path)))missing.push(path);else if(resolve(root,path).length>256)overlength.push(path);}
await writeFile(join(ticket,"📥️files.json"),JSON.stringify(paths,null,2)+"\n");
const generated=join(ticket,"🗑️generated");await mkdir(generated,{recursive:true});
let status=0,stdout="",stderr="";
for(let index=0;index<source.length;index+=128){
 const child=Bun.spawn(["git","-c","core.fsmonitor=false","diff","--check","--",...source.slice(index,index+128)],{cwd:root,stdout:"pipe",stderr:"pipe"});
 const [code,out,error]=await Promise.all([child.exited,new Response(child.stdout).text(),new Response(child.stderr).text()]);
 status=Math.max(status,code);stdout+=out;stderr+=error;
}
await writeFile(join(generated,"source-review.json"),JSON.stringify({paths:paths.length,source:source.length,ticket:paths.length-source.length,missing,overlength,whitespace:{status,stdout,stderr}},null,2)+"\n");
await writeFile(join(ticket,"source-changes.md"),`# Source Inventory\n\nThe owned inventory contains ${paths.length} paths: ${source.length} source/interface/owned-product paths and ${paths.length-source.length} retained ticket inputs/reports. Exact paths are recorded in \`📥️files.json\`.\n\nRead-only scoped Git whitespace validation exited ${status}. ${overlength.length} existing recorded paths exceed 256 UTF-16 units including the absolute workspace root. No modifying Git command was used.\n\n## Removed Paths\n\n${missing.length} recorded paths are absent; these preserve removed ownership locations, the intermediate declaration path and old artifact marker filenames.\n\n${missing.map(path=>"- `"+path+"`").join("\n")}\n`);
console.log("[DEBUG] Owned source inventory "+JSON.stringify({paths:paths.length,source:source.length,ticket:paths.length-source.length,missing:missing.length,overlength:overlength.length,whitespace:status}));
if(status||overlength.length)throw Error("Owned source review failed");
