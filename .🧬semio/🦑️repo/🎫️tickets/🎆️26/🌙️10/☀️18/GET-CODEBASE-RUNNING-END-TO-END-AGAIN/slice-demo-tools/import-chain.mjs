import {readFileSync,existsSync} from "node:fs";
import {dirname,resolve,relative} from "node:path";
const root="/Users/ueli/Documents/semio",target=process.argv[3],entries=process.argv.slice(4),seen=new Map();
const t=new Bun.Transpiler({loader:"ts"});
const queue=[...entries.map(e=>[resolve(root,e),null])];
while(queue.length){const [p,from]=queue.shift();if(seen.has(p))continue;seen.set(p,from);if(!existsSync(p))continue;
 if(p.includes(target)){let c=[],x=p;while(x){c.push(relative(root,x));x=seen.get(x);}console.log(c.reverse().join("\n  -> "));process.exit(0);}
 let src;try{src=readFileSync(p,"utf8");}catch{continue;}
 const loader=p.endsWith(".mjs")||p.endsWith(".js")?"js":"ts";let imps=[];try{imps=new Bun.Transpiler({loader}).scan(src).imports;}catch{continue;}
 for(const i of imps){if(i.path.startsWith(".")){let q=resolve(dirname(p),i.path);queue.push([q,p]);}}}
console.log("not reached",seen.size);
