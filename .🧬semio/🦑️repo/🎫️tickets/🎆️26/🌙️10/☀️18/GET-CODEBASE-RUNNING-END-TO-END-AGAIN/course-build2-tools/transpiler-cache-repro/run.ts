import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
const root=resolve(process.argv[2]);
const images=new Map<string,Buffer>();
for(const f of ["main.ts","big.ts","dep.ts"])images.set(join(root,f),readFileSync(join(root,f)));
Bun.plugin({name:"t",setup(build){
 build.onResolve({filter:/.*/,namespace:"semio-source"},item=>{const n=item.path.replace(/^semio-source:/,"");if(!images.has(n))throw Error("Uncaptured require namespace image refused");return {path:n,namespace:"semio-source"};});
 build.onResolve({filter:/.*/},item=>{const importer=item.importer?.replace(/^[/]?semio-source:/,"");const cand=item.path.startsWith("/")?item.path:join(importer,"..",item.path);console.error("resolve",item.path,"from",item.importer,"->",cand);if(!images.has(cand))throw Error("refused "+cand);return {path:cand,namespace:"semio-source"};});
 build.onLoad({filter:/.*/,namespace:"semio-source"},item=>{const n=resolve(item.path);const b=images.get(n);if(!b)throw Error("Uncaptured implementation module refused: "+n);const c=new Bun.Transpiler({loader:"ts",target:"bun"}).transformSync(b.toString("utf8"));console.error("load",n,"len",c.length);return {contents:c,loader:"js"};});
}});
const m=await import(join(root,"main.ts"));console.log("out",m.out);
