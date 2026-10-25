import {readFileSync,existsSync} from "node:fs";
import {dirname,resolve} from "node:path";
const t=new Bun.Transpiler({loader:"ts"});
const seen=new Set<string>(),parent=new Map<string,string>();
const walk=(file:string)=>{
 if(seen.has(file))return;seen.add(file);
 let imports:{path:string}[]=[];try{imports=t.scanImports(readFileSync(file,"utf8").replace(/^#!.*\n/u,""));}catch(e){console.error(file.slice(-50),String(e).slice(0,100));return;}
 for(const row of imports){if(!row.path.startsWith("."))continue;const target=resolve(dirname(file),row.path);if(existsSync(target)&&/\.(ts|mjs|tsx|js)$/u.test(target)){if(!seen.has(target))parent.set(target,file);walk(target);}}
};
walk(resolve(process.argv[2]!));
const needle=process.argv[3];
console.log(seen.size,"files");
if(needle){for(const f of seen)if(f.includes(needle)){let chain=[f];while(parent.has(chain[0]!))chain.unshift(parent.get(chain[0]!)!);console.log(chain.map(x=>x.slice(-70)).join("\n -> "));break;}}
