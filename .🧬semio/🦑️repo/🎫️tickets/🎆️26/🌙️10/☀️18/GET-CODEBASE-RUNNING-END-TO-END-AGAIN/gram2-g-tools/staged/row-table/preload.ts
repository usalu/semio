// [gram2-g] bun --preload: serves the observation module with the staged row-table edits applied in memory (and the leaf module from the staged tree), so the existing suites run against the patch before it lands. Not part of the batch.
import{plugin}from"bun";
import{copyFileSync,existsSync,mkdirSync,readFileSync,rmSync,writeFileSync}from"node:fs";import{createHash}from"node:crypto";import{tmpdir}from"node:os";
import{basename,dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
const flatTree=(dir:string)=>{const flat=join(dir,"tree"),raw=readFileSync(join(flat,"manifest.json")),root=join(tmpdir(),"flat-trees",basename(dirname(dirname(dir))),createHash("sha1").update(raw).digest("hex").slice(0,8),"staged",basename(dir)),out=join(root,"tree");if(!existsSync(join(root,".expanded"))){rmSync(root,{recursive:true,force:true});for(const[rel,row]of Object.entries(JSON.parse(raw.toString("utf8"))as Record<string,{file:string}>)){mkdirSync(dirname(join(out,rel)),{recursive:true});copyFileSync(join(flat,row.file),join(out,rel));}writeFileSync(join(root,".expanded"),"");}return out;};
const here=dirname(fileURLToPath(import.meta.url)),spec=JSON.parse(readFileSync(join(here,"edits.json"),"utf8")),leaf=join(flatTree(here),"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🔢️row-table/🟨️.mjs");
plugin({name:"row-table-staged",setup(build){
 build.onLoad({filter:/observation\/[^/]*\.mjs$/},async args=>{
  const text=await Bun.file(args.path).text();
  if(!args.path.endsWith("👁️observation/🟨️.mjs"))return{contents:text,loader:"js"};
  let out=text;for(const edit of spec.edits){if(out.split(edit.old).length!==2)throw Error("staged anchor missing: "+edit.name);out=out.replace(edit.old,edit.new);}
  return{contents:out.replace('"./🔢️row-table/🟨️.mjs"',JSON.stringify("file://"+leaf)),loader:"js"};
 });
}});
