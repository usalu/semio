
import fs from "node:fs";
import path from "node:path";
const ROOT="/Users/ueli/Documents/semio";
const FW=fs.readdirSync(ROOT).find(n=>n.includes("framework")&&fs.statSync(path.join(ROOT,n)).isDirectory());
const find=(parent,pred)=>fs.readdirSync(parent).map(n=>path.join(parent,n)).find(p=>pred(path.basename(p)));
const modules=path.join(ROOT,FW,"🛍️products/💻️os/🔨️modules");
const DEV=find(modules,b=>b.includes("dev"));
const plugin=find(modules,b=>b.includes("plugin"));
const inf=find(modules,b=>b.includes("infinite"));
const canvas=find(inf,b=>b.includes("canvas"));
const world=find(inf,b=>b.includes("world"));
const r3f=find(world,b=>b.includes("r3f"));
const ui=find(path.join(ROOT,FW,"🔨️modules"),b=>b.includes("ui"));
const pickEntry=(dir,names)=>{for(const n of names){const p=path.join(dir,n);if(fs.existsSync(p))return p;}return dir;};
const reg=find(path.join(plugin,"📦️packages/🟦️typescript"),b=>b.includes("registry"));
const gen=find(reg,b=>b.includes("generated"));
const playgrounds=path.join(gen,fs.readdirSync(gen).find(n=>n.includes("playgrounds")&&n.endsWith(".ts")));
const osDevScript=path.join(DEV,"📦️packages/🟦️typescript/📜️script.ts");
const pluginModules=path.join(DEV,"🔌️plugin-modules");
const canvasRr=find(canvas,b=>b.includes("react-renderer"));
const canvasEntry=pickEntry(path.join(canvasRr,"📦️packages/🟦️typescript"),["📦️index.tsx","🟦️glue.tsx","🟦️glue.ts","📦️index.ts"]);
const worldEntry=pickEntry(path.join(r3f,"📦️packages/🟦️typescript"),["📦️index.tsx","🟦️glue.tsx","🟦️glue.ts","📦️index.ts"]);
const fwCore=pickEntry(path.join(ROOT,FW,"📦️packages/🟦️typescript"),["🟦️glue.ts","📦️index.ts"]);
const osCore=pickEntry(path.join(ROOT,FW,"🛍️products/💻️os/📦️packages/🟦️typescript"),["🟦️glue.ts","📦️index.ts"]);
const uiA=path.join(ui,"⚛️react/📦️packages/🟦️typescript/📦️index.tsx");
const uiB=path.join(ui,"📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx");
const uiReact=fs.existsSync(uiA)?uiA:uiB;
const rel=p=>"./"+path.relative(ROOT,p).split(path.sep).join("/");
const out={
  playgrounds:rel(playgrounds),
  osDevScript:rel(osDevScript),
  pluginModules:rel(pluginModules),
  canvasEntry:rel(canvasEntry),
  worldEntry:rel(worldEntry),
  fwCore:rel(fwCore),
  osCore:rel(osCore),
  uiReact:rel(uiReact),
  exists:Object.fromEntries(Object.entries({playgrounds,osDevScript,pluginModules,canvasEntry,worldEntry,fwCore,osCore,uiReact}).map(([k,v])=>[k,fs.existsSync(v)]))
};
fs.writeFileSync(path.join(process.argv[2],"🧪demonstrator-path-targets.json"), JSON.stringify(out,null,2));
console.log(JSON.stringify(out,null,2));
