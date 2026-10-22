import fs from "node:fs";
const calls:{fn:string;ms:number;bytes:number;by:string}[]=[];(globalThis as any).__bfV=calls;
Bun.plugin({name:"bfv",setup(b){b.onLoad({filter:/\.(ts|mjs)$/},a=>{let t=fs.readFileSync(a.path,"utf8");
 if(a.path.endsWith("/✅️validator/🟦️.ts")){
  t=t.replace("export function compileJsonSchemaSubset(","function __c0(").replace("export function validateJsonSchemaSubset(","function __v0(");
  t+=`\nconst __w=(fn:string,f:any)=>(...a:any[])=>{const s=Bun.nanoseconds();try{return f(...a);}finally{const o=Error.stackTraceLimit;Error.stackTraceLimit=4;const st=(new Error().stack??"").split("\\n").slice(2,4).join(" | ");Error.stackTraceLimit=o;(globalThis as any).__bfV.push({fn,ms:(Bun.nanoseconds()-s)/1e6,bytes:JSON.stringify(a[0]).length,by:st});}};\nexport const compileJsonSchemaSubset:typeof __c0=__w("compile",__c0) as any;\nexport const validateJsonSchemaSubset:typeof __v0=__w("validate",__v0) as any;\n`;
 }
 return {contents:t,loader:a.path.endsWith(".ts")?"ts":"js"};});}});
process.on("exit",()=>{const by=new Map<string,{n:number;ms:number;bytes:number}>();for(const c of calls){const k=c.fn+" "+c.by.replace(/\/private\/tmp\/boot-fast\/ov\/🧰️framework\//g,"").slice(0,170);const r=by.get(k)??{n:0,ms:0,bytes:0};r.n++;r.ms+=c.ms;r.bytes=Math.max(r.bytes,c.bytes);by.set(k,r);}
 console.error("[validator] calls",calls.length,"total ms",calls.reduce((a,c)=>a+c.ms,0).toFixed(1));for(const [k,v] of [...by].sort((a,b)=>b[1].ms-a[1].ms).slice(0,10))console.error(` ${v.ms.toFixed(1).padStart(7)} ms n=${v.n} schemaBytes=${v.bytes} ${k}`);});
