import { readFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
const R="/Users/ueli/Documents/semio/";
const out=spawnSync("git",["ls-files","-z","--","*Cargo.toml"],{cwd:R,encoding:"utf8",maxBuffer:1<<28}).stdout.split("\0").filter(Boolean);
const rows:any[]=[];
for(const f of out){let t:string;try{t=readFileSync(R+f,"utf8")}catch{continue} if(!t.includes("semio.playground"))continue; let toml:any;try{toml=Bun.TOML.parse(t)}catch{continue} for(const r of toml.package?.metadata?.semio?.playground??[])rows.push({...r,f})}
console.log("registry rows",rows.length);
const L=Bun.JSONC.parse(await Bun.file(R+".vscode/launch.json").text());
const S=Bun.JSONC.parse(await Bun.file(R+".vscode/🧩️launch.seed.jsonc").text());
const cfgs:any[]=L.configurations;
const devRe=/^bun repo run @semio-tech\/framework-os-dev:dev-(.+)-(react|wgpu)-(prototype|release)\b/;
const byV=new Map<string,any[]>();
for(const c of cfgs){const m=typeof c.command==="string"&&c.command.match(devRe); if(!m)continue; const a=byV.get(m[1])??[]; a.push({c,r:m[2],p:m[3]}); byV.set(m[1],a)}
console.log("variants with dev entries",byV.size,"registry variants",rows.length);
const regV=new Set(rows.map(r=>r.variant));
console.log("L variants not in registry",[...byV.keys()].filter(v=>!regV.has(v)));
console.log("registry variants lacking L entries",rows.filter(r=>!byV.has(r.variant)).map(r=>r.variant));
// compare
let ok=0,bad=0;const diffs:string[]=[];
const cur=new Set(Object.keys(S.devLaunchers));
console.log("curated not in registry",[...cur].filter(v=>!regV.has(v)));
for(const r of rows){ for(const e of byV.get(r.variant)??[]){ const c=e.c; const port=e.r==="react"?r.ports.react:r.ports.wgpu;
  const exp={S_OS_PORT:String(port),SEMIO_PLUGIN:r.variant,SEMIO_RENDERER:e.r,...(r.app?{SEMIO_APP:r.app}:{})};
  const okEnv=Object.entries(exp).every(([k,v])=>c.env?.[k]===v);
  const okSra=c.serverReadyAction?.pattern?.includes(":"+port+")");
  if(okEnv&&okSra)ok++;else{bad++;if(diffs.length<15)diffs.push(c.name+" env="+JSON.stringify(c.env)+" sra="+c.serverReadyAction?.pattern+" exp="+JSON.stringify(exp)+" port "+port)}
 }}
console.log("ok",ok,"bad",bad);console.log(diffs.join("\n"));
// names per variant
const names=[...byV].slice(0,6).map(([v,es])=>v+": "+es.map(e=>e.c.name+" @"+e.c.presentation.order).join(" | "));console.log(names.join("\n"));
console.log("=========== part 2");
const regRows=rows.filter(r=>!r.f.startsWith(".🧬semio"));
console.log("registry rows (non-ticket)",regRows.length,new Set(regRows.map(r=>r.variant)).size);
const envNorm={BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",NODE_DISABLE_COMPILE_CACHE:"1"};
const canon=(v:any):string=>v===undefined?"U":Array.isArray(v)?"["+v.map(canon).join(",")+"]":v&&typeof v==="object"?"{"+Object.keys(v).sort().map(k=>JSON.stringify(k)+":"+canon(v[k])).join(",")+"}":JSON.stringify(v);
const sra=(port:number)=>({action:"openExternally",pattern:`(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):${port})`,uriFormat:"%s"});
const expected=(r:any,rend:"react"|"wgpu",name:string,order:number)=>{const port=rend==="react"?r.ports.react:r.ports.wgpu;return {name,type:"node-terminal",request:"launch",command:`bun repo run @semio-tech/framework-os-dev:dev-${r.variant}-${rend}-prototype --skip-cache --configuration=prototype`,cwd:"${workspaceFolder}",env:{S_OS_PORT:String(port),SEMIO_PLUGIN:r.variant,SEMIO_RENDERER:rend,...(r.app?{SEMIO_APP:r.app}:{}),...envNorm},presentation:{group:"3_dev",order},serverReadyAction:sra(port)}};
let exact=0,near=0,none=0;const nones:string[]=[],nears:string[]=[];
for(const r of regRows){for(const rend of ["react","wgpu"] as const){
  const cands=cfgs.filter(c=>typeof c.command==="string"&&c.command.startsWith(`bun repo run @semio-tech/framework-os-dev:dev-${r.variant}-${rend}-prototype `)&&c.env?.SEMIO_PLUGIN===r.variant);
  const hit=cands.find(c=>{const e=expected(r,rend,c.name,c.presentation.order);return canon(e)===canon(c)});
  if(hit){exact++;continue}
  if(cands.length){near++;if(nears.length<10)nears.push(r.variant+" "+rend+" cands="+cands.map(c=>c.name+" "+canon(c.env)).join(" || "))}else{none++;nones.push(r.variant+":"+rend)}
}}
console.log("exact",exact,"near",near,"none",none);console.log(nears.join("\n"));console.log(nones.slice(0,30));
