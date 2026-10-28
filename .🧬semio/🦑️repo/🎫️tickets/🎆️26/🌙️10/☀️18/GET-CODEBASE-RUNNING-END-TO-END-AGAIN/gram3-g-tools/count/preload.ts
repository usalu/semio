// [gram3-g] bun --preload: staged batches (BATCH_DIRS as walk-g's preload_generic) + counting hooks: node:crypto shim in every library module, global Buffer.from/alloc, dynamic RegExp, JSON.parse/stringify counted by call site. COUNT_OUT=<file> gets the table at exit. Not part of any batch.
import{plugin}from"bun";
import{copyFileSync,existsSync,mkdirSync,readFileSync,rmSync,writeFileSync}from"node:fs";import{createHash}from"node:crypto";import{tmpdir}from"node:os";
import{basename,dirname,join,resolve}from"node:path";import{fileURLToPath}from"node:url";
import{counters,siteOf}from"./shim.mjs";
import ts from"typescript";
const COUNT_RX=process.env.COUNT_RX==="1",COUNT_FN=process.env.COUNT_FN==="1";
(globalThis as any).__g3rx=(site:string,re:RegExp)=>{counters.add("literal",site);return re;};
(globalThis as any).__g3fn=(site:string)=>{counters.add("call",site);};
const instrument=(path:string,text:string):string=>{
 const source=ts.createSourceFile(path,text,ts.ScriptTarget.Latest,true,path.endsWith(".ts")?ts.ScriptKind.TS:ts.ScriptKind.JS),edits:{at:number;text:string}[]=[],short=path.replace(/^.*🏗️build\//,"");
 const line=(node:ts.Node)=>source.getLineAndCharacterOfPosition(node.getStart()).line+1;
 const visit=(node:ts.Node)=>{
  if(COUNT_RX&&node.kind===ts.SyntaxKind.RegularExpressionLiteral){let up:ts.Node|undefined=node.parent,inside=false;while(up){if(ts.isFunctionLike(up)){inside=true;break;}up=up.parent;}if(inside){edits.push({at:node.getStart(),text:'__g3rx('+JSON.stringify(short+":"+line(node))+','});edits.push({at:node.getEnd(),text:')'});}}
  if(COUNT_FN&&(ts.isFunctionDeclaration(node)||ts.isFunctionExpression(node)||ts.isArrowFunction(node)||ts.isMethodDeclaration(node))&&node.body&&ts.isBlock(node.body)){const name=(node as any).name?.getText()??"(anon)";edits.push({at:node.body.getStart()+1,text:'__g3fn('+JSON.stringify(short+":"+name+":"+line(node))+');'});}
  ts.forEachChild(node,visit);
 };
 visit(source);edits.sort((a,b)=>b.at-a.at);let out=text;for(const e of edits)out=out.slice(0,e.at)+e.text+out.slice(e.at);return out;
};
/** 🧊 A flat staged tree (manifest.json + f<NNN>) expanded once to a short scratch directory laid out like the repository (.../staged/<batch>/tree); flatten-g-tools/flat.py `scratch` computes the same path. */
const flatTree=(dir:string)=>{const flat=join(dir,"tree"),raw=readFileSync(join(flat,"manifest.json")),root=join(tmpdir(),"flat-trees",basename(dirname(dirname(dir))),createHash("sha1").update(raw).digest("hex").slice(0,8),"staged",basename(dir)),out=join(root,"tree");if(!existsSync(join(root,".expanded"))){rmSync(root,{recursive:true,force:true});for(const[rel,row]of Object.entries(JSON.parse(raw.toString("utf8"))as Record<string,{file:string}>)){mkdirSync(dirname(join(out,rel)),{recursive:true});copyFileSync(join(flat,row.file),join(out,rel));}writeFileSync(join(root,".expanded"),"");}return out;};
const dirs=(process.env.BATCH_DIRS??"").split(",").filter(Boolean),files=new Map<string,{name:string;old:string;new:string}[]>(),trees:string[]=dirs.map(flatTree);
for(const dir of dirs){const spec=JSON.parse(readFileSync(join(dir,"edits.json"),"utf8"));for(const edit of spec.edits){const file=edit.file??spec.file;if(!/\.(mjs|ts|json)$/.test(file))continue;(files.get(file)??files.set(file,[]).get(file)!).push(edit);}}
const workspace=resolve(dirs[0]??".","..","..","..","..","..","..","..","..","..","..");
const fromTrees=(importer:string,specifier:string)=>{if(!specifier.startsWith("."))return undefined;const local=resolve(dirname(importer),specifier);if(existsSync(local))return undefined;for(const tree of trees){const candidate=join(tree,local.slice(workspace.length+1));if(existsSync(candidate))return "file://"+candidate;}return undefined;};
const SHIM=fileURLToPath(new URL("./shim.mjs",import.meta.url));
const escapeRe=(text:string)=>text.replace(/[.*+?^${}()|[\]\\]/g,"\\$&"),jsonFiles=[...files.keys()].filter(file=>file.endsWith(".json"));
if(jsonFiles.length)plugin({name:"gram3-json",setup(build){
 build.onLoad({filter:new RegExp("("+jsonFiles.map(file=>escapeRe(file.split("/").slice(-3).join("/"))).join("|")+")$")},async args=>{
  const text=await Bun.file(args.path).text();
  for(const[file,edits]of files){if(!args.path.endsWith(file))continue;let out=text;for(const edit of edits){if(out.split(edit.old).length!==2)throw Error("staged anchor missing: "+edit.name);out=out.replace(edit.old,()=>edit.new);}return{exports:{default:JSON.parse(out),...JSON.parse(out)},loader:"object"};}
  return{contents:text,loader:"json"};
 });
}});
plugin({name:"gram3-count",setup(build){
 build.onLoad({filter:/(🏗️build|row-table).*\.(mjs|ts)$/},async args=>{
  if(/\/(🧪️tests|🧬️schema)\//.test(args.path))return undefined;
  let text=await Bun.file(args.path).text();const loader=args.path.endsWith(".ts")?"ts":"js";
  for(const[file,edits]of files){if(!args.path.endsWith(file))continue;for(const edit of edits){if(text.split(edit.old).length!==2)throw Error("staged anchor missing: "+edit.name);text=text.replace(edit.old,()=>edit.new);}}
  if(dirs.length)text=text.replace(/(from\s*)"(\.[^"]+)"/g,(whole,head,specifier)=>{const found=fromTrees(args.path,specifier);return found?head+JSON.stringify(found):whole;});
  if((COUNT_RX||COUNT_FN)&&!/\/(count|row-table)\//.test(args.path))text=instrument(args.path,text);
  text=text.replace(/from\s*"node:crypto"/g,()=>'from '+JSON.stringify(SHIM));
  return{contents:text,loader};
 });
}});
const origFrom=Buffer.from.bind(Buffer),origAlloc=Buffer.alloc.bind(Buffer),origConcat=Buffer.concat.bind(Buffer);
(Buffer as any).from=(...a:any[])=>{const r=origFrom(...(a as [any]));counters.add("Buffer.from",siteOf(),1,r.length);return r;};
(Buffer as any).alloc=(...a:any[])=>{const r=origAlloc(...(a as [number]));counters.add("Buffer.alloc",siteOf(),1,r.length);return r;};
(Buffer as any).concat=(...a:any[])=>{const r=origConcat(...(a as [any]));counters.add("Buffer.concat",siteOf(),1,r.length);return r;};
const RealRegExp=RegExp;
(globalThis as any).RegExp=new Proxy(RealRegExp,{construct(target,args,newTarget){counters.add("new RegExp",siteOf(),1);return Reflect.construct(target,args,newTarget===(globalThis as any).RegExp?target:newTarget);},apply(target,thisArg,args){counters.add("RegExp()",siteOf(),1);return Reflect.apply(target,thisArg,args);}});
const parse=JSON.parse,stringify=JSON.stringify;
JSON.parse=((text:any,reviver?:any)=>{counters.add("JSON.parse",siteOf(),1,typeof text==="string"?text.length:0);return parse(text,reviver);}) as typeof JSON.parse;
JSON.stringify=((value:any,replacer?:any,space?:any)=>{const out=stringify(value,replacer,space);counters.add("JSON.stringify",siteOf(),1,typeof out==="string"?out.length:0);return out;}) as typeof JSON.stringify;
process.on("exit",()=>{const out=process.env.COUNT_OUT;if(!out)return;const rows=[...counters.sites.entries()].sort((a,b)=>b[1].n-a[1].n);writeFileSync(out,rows.map(([k,v])=>v.n+"\t"+v.bytes+"\t"+k).join("\n")+"\n");});
