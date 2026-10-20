import {existsSync,mkdirSync,readFileSync,rmSync,symlinkSync} from "node:fs";
import {dirname,join} from "node:path";
const root=process.argv[2]!,found:string[]=[];
for await(const file of new Bun.Glob("🧰️framework/**/package.json").scan({cwd:root,dot:false,onlyFiles:true})){
 if(file.includes("node_modules"))continue;
 try{const manifest=JSON.parse(readFileSync(join(root,file),"utf8"));if(typeof manifest.name==="string"&&manifest.name.startsWith("@semio-tech/")){
  const link=join(root,"node_modules",manifest.name);mkdirSync(dirname(link),{recursive:true});rmSync(link,{force:true});symlinkSync(join(root,dirname(file)),link,"dir");found.push(manifest.name);}}catch{}
}
console.log(found.length+" workspace links");
