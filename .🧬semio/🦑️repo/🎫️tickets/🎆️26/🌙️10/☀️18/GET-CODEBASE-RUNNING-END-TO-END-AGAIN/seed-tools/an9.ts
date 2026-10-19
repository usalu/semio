import { existsSync } from "node:fs";
const R="/Users/ueli/Documents/semio/";
const L=Bun.JSONC.parse(await Bun.file(R+".vscode/launch.json").text());
const sub=(s:string)=>s.replaceAll("${workspaceFolder}",R.slice(0,-1));
const dead:Record<string,number>={}; const samples:Record<string,string[]>={};
const deadNames:string[]=[];
for(const c of L.configurations){
  const why:string[]=[];
  if(typeof c.cwd==="string"){const p=sub(c.cwd); if(p.startsWith("/")&&!existsSync(p))why.push("cwd");}
  const cmd:string=c.command??"";
  // script paths: bun "<path>" or bun <path>
  for(const m of cmd.matchAll(/\bbun\s+"?(\$\{workspaceFolder\}\/[^"\s]+\.(?:ts|mjs|js|py|sh|tsx))"?/g)){const p=sub(m[1]); if(!existsSync(p))why.push("script");}
  for(const m of cmd.matchAll(/\bpython3?\s+"?(\$\{workspaceFolder\}\/[^"\s]+\.py)"?/g)){const p=sub(m[1]); if(!existsSync(p))why.push("script");}
  if(why.length){const k=[...new Set(why)].join("+");dead[k]=(dead[k]??0)+1;(samples[k]??=[]).length<4&&samples[k].push(c.name+" :: "+cmd.slice(0,200));deadNames.push(c.name)}
}
console.log(JSON.stringify(dead),deadNames.length);
for(const k in samples)console.log(k,samples[k].join("\n   "));
await Bun.write("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/seed-run/dead-paths.json",JSON.stringify(deadNames));
