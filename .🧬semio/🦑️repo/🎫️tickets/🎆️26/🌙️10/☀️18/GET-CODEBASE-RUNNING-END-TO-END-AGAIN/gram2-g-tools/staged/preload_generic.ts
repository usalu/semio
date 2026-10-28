// [gram2-g] bun --preload for staged batches: BATCH_DIRS=<dir>[,<dir>...] serves every .mjs/.ts file named by the edits.json of those batches with its anchored edits applied in memory; relative imports of files that exist only in a batch tree (new modules) are rewritten to their absolute location. Not part of any batch.
import{plugin}from"bun";
import{copyFileSync,existsSync,mkdirSync,readFileSync,rmSync,writeFileSync}from"node:fs";import{createHash}from"node:crypto";import{tmpdir}from"node:os";
import{basename,dirname,join,resolve}from"node:path";
/** 🧊 A flat staged tree (manifest.json + f<NNN>) expanded once to a short scratch directory laid out like the repository (.../staged/<batch>/tree); flatten-g-tools/flat.py `scratch` computes the same path. */
const flatTree=(dir:string)=>{const flat=join(dir,"tree"),raw=readFileSync(join(flat,"manifest.json")),root=join(tmpdir(),"flat-trees",basename(dirname(dirname(dir))),createHash("sha1").update(raw).digest("hex").slice(0,8),"staged",basename(dir)),out=join(root,"tree");if(!existsSync(join(root,".expanded"))){rmSync(root,{recursive:true,force:true});for(const[rel,row]of Object.entries(JSON.parse(raw.toString("utf8"))as Record<string,{file:string}>)){mkdirSync(dirname(join(out,rel)),{recursive:true});copyFileSync(join(flat,row.file),join(out,rel));}writeFileSync(join(root,".expanded"),"");}return out;};
const dirs=(process.env.BATCH_DIRS??process.env.BATCH_DIR??"").split(",").filter(Boolean),files=new Map<string,{name:string;old:string;new:string}[]>(),trees:string[]=dirs.map(flatTree);
for(const dir of dirs){const spec=JSON.parse(readFileSync(join(dir,"edits.json"),"utf8"));for(const edit of spec.edits){const file=edit.file??spec.file;if(!/\.(mjs|ts)$/.test(file))continue;(files.get(file)??files.set(file,[]).get(file)!).push(edit);}}
const escape=(text:string)=>text.replace(/[.*+?^${}()|[\]\\]/g,"\\$&"),workspace=resolve(dirs[0]!,"..","..","..","..","..","..","..","..","..","..");
const fromTrees=(importer:string,specifier:string)=>{
 if(!specifier.startsWith("."))return undefined;
 const local=resolve(dirname(importer),specifier);if(existsSync(local))return undefined;
 for(const tree of trees){const relative=local.slice(workspace.length+1),candidate=join(tree,relative);if(existsSync(candidate))return "file://"+candidate;}
 return undefined;
};
plugin({name:"staged-batches",setup(build){
 build.onLoad({filter:new RegExp("("+[...files.keys()].map(file=>escape(file.split("/").slice(-3).join("/"))).join("|")+")$")},async args=>{
  const text=await Bun.file(args.path).text(),loader=args.path.endsWith(".ts")?"ts":"js";
  for(const[file,edits]of files){
   if(!args.path.endsWith(file))continue;
   let out=text;for(const edit of edits){if(out.split(edit.old).length!==2)throw Error("staged anchor missing: "+edit.name);out=out.replace(edit.old,edit.new);}
   out=out.replace(/(from\s*)"(\.[^"]+)"/g,(whole,head,specifier)=>{const found=fromTrees(args.path,specifier);return found?head+JSON.stringify(found):whole;});
   return{contents:out,loader};
  }
  return{contents:text,loader};
 });
}});
