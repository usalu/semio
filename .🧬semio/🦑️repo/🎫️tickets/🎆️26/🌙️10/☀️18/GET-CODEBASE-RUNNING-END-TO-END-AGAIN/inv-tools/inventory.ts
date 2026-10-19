import {readdirSync,readFileSync,writeFileSync} from "node:fs";
import {join,resolve,dirname,relative} from "node:path";
import ts from "typescript";
const root=resolve(process.argv[2]??"🧰️framework"),out=process.argv[3]!;
const skip=new Set(["node_modules","target","dist","🗑️generated","🤖️generated",".git","🛍️products","🛒️products","pkg"]);
const files:string[]=[];
const walk=(d:string)=>{for(const e of readdirSync(d,{withFileTypes:true})){if(e.isSymbolicLink())continue;const p=join(d,e.name);if(e.isDirectory()){if(!skip.has(e.name))walk(p);}else if(/\.[cm]?[jt]sx?$/u.test(e.name))files.push(p);}};
walk(root);
const specific=(p:string)=>/(?:^|[\\/])(?:🛍️products|✏️s|🌎️hub)(?:[\\/]|$)/u.test(p);
const rows:{file:string;spec:string;target:string}[]=[];
for(const f of files){
 const text=readFileSync(f,"utf8");
 const imps=ts.preProcessFile(text,true,true).importedFiles;
 for(const i of imps){
  let target:string|undefined;
  if(i.fileName.startsWith(".")){target=resolve(dirname(f),i.fileName);}
  else if(i.fileName.startsWith("@semio-tech/")){try{target=Bun.resolveSync(i.fileName,dirname(f));}catch{}}
  if(target&&specific(target))rows.push({file:relative(resolve(root,".."),f),spec:i.fileName,target:relative(resolve(root,".."),target)});
 }
}
writeFileSync(out,JSON.stringify(rows,null,1));
const byFile=new Map<string,number>();for(const r of rows)byFile.set(r.file,(byFile.get(r.file)??0)+1);
console.log("files scanned",files.length,"rows",rows.length,"importing files",byFile.size);
