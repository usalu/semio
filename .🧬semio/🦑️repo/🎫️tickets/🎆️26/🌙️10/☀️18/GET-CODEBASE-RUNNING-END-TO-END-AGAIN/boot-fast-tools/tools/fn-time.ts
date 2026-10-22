import fs from "node:fs";
type T={file:string;fn:string};const targets:T[]=JSON.parse(process.env.BOOTFAST_WRAP??"[]"),rows=new Map<string,{n:number;ms:number}>();
Bun.plugin({name:"bf-fn",setup(b){b.onLoad({filter:/\.(ts|mjs)$/},a=>{let t=fs.readFileSync(a.path,"utf8");
 for(const x of targets)if(a.path.includes(x.file)){
  const re=new RegExp(`export (async )?function ${x.fn}\\(`);if(!re.test(t))continue;
  t=t.replace(re,(_m,as)=>`${as??""}function __bf_${x.fn}(`);
  t+=`\nexport const ${x.fn}=((f)=>function(...a){const s=Bun.nanoseconds();const r=f.apply(this,a);if(r&&typeof r.then==="function")return r.finally(()=>globalThis.__bfT("${x.fn}",(Bun.nanoseconds()-s)/1e6));globalThis.__bfT("${x.fn}",(Bun.nanoseconds()-s)/1e6);return r;})(__bf_${x.fn});\n`;
 }
 return {contents:t,loader:a.path.endsWith(".ts")?"ts":"js"};});}});
(globalThis as any).__bfT=(k:string,ms:number)=>{const r=rows.get(k)??{n:0,ms:0};r.n++;r.ms+=ms;rows.set(k,r);};
process.on("exit",()=>{for(const [k,v] of rows)console.error(`[fn-time] ${k} n=${v.n} total=${v.ms.toFixed(1)}ms mean=${(v.ms/v.n).toFixed(3)}ms`);});
