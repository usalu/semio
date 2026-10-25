import {readFileSync,existsSync} from "node:fs";
import {dirname,resolve,relative} from "node:path";
const t=new Bun.Transpiler({loader:"ts"});
const cache=new Map<string,string[]>();
const edges=(file:string):string[]=>{
 let known=cache.get(file);if(known)return known;known=[];cache.set(file,known);
 if(!/\.(ts|mts|mjs|tsx|js)$/u.test(file))return known;
 let imports:{path:string}[]=[];try{imports=t.scanImports(readFileSync(file,"utf8").replace(/^#!.*\n/u,""));}catch{return known;}
 for(const row of imports){if(!row.path.startsWith("."))continue;const target=resolve(dirname(file),row.path);if(existsSync(target)&&/\.(ts|mjs|tsx|js|json)$/u.test(target))known.push(target);}
 return known;
};
export const closure=(roots:string[]):Set<string>=>{const seen=new Set<string>(),pending=[...roots];while(pending.length){const f=pending.pop()!;if(seen.has(f))continue;seen.add(f);pending.push(...edges(f));}return seen;};
if(import.meta.main){
 const args=process.argv.slice(2);const roots=args.map(a=>resolve(a));
 const all=closure(roots);
 for(const r of roots){console.log(closure([r]).size,relative(process.cwd(),r).slice(-90));}
 console.log("union",all.size);
}
