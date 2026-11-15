import{readFileSync,readdirSync,statSync}from"node:fs";import{join}from"node:path";
const walk=d=>readdirSync(d).flatMap(n=>{const p=join(d,n);if(n==="node_modules")return[];return statSync(p).isDirectory()?walk(p):[p];});
let bad=0;
for(const p of walk(process.argv[2])){
 const loader=p.endsWith(".ts")||p.endsWith(".d.mts")?"ts":p.endsWith(".mjs")||p.endsWith(".mts")?"js":undefined;if(!loader)continue;
 try{new Bun.Transpiler({loader}).transformSync(readFileSync(p,"utf8"));}catch(e){bad++;console.log("SYNTAX",p.slice(-60),String(e).slice(0,300));}
 if(p.endsWith(".json")){}
}
for(const p of walk(process.argv[2]))if(p.endsWith(".json"))try{JSON.parse(readFileSync(p,"utf8"));}catch(e){bad++;console.log("JSON",p.slice(-60),String(e).slice(0,200));}
console.log("checked, bad =",bad);
