import fs from "node:fs";
const rows:{spec:string;ms:number}[]=[];(globalThis as any).__bfi=async(spec:string,f:()=>Promise<unknown>)=>{const s=Bun.nanoseconds();const r=await f();rows.push({spec:spec.split("/").slice(-3).join("/"),ms:(Bun.nanoseconds()-s)/1e6});return r;};
Bun.plugin({name:"bfit",setup(b){b.onLoad({filter:/\.(ts|mjs)$/},a=>{let t=fs.readFileSync(a.path,"utf8");
 if(a.path.includes("/📥️caller/📤️child/")){t=t.replace(/await import\(("[^"]+")\)/g,(_m,s)=>`await globalThis.__bfi(${s},()=>import(${s}))`);t=t.replace(/await Promise\.all\(\[(import\([^\]]+)\]\)/,(m)=>m);}
 return {contents:t,loader:a.path.endsWith(".ts")?"ts":"js"};});}});
process.on("exit",()=>{let tot=0;for(const r of rows){tot+=r.ms;console.error(`[import-timer] ${r.ms.toFixed(1).padStart(7)} ms ${r.spec}`);}console.error("[import-timer] sum",tot.toFixed(1));});
