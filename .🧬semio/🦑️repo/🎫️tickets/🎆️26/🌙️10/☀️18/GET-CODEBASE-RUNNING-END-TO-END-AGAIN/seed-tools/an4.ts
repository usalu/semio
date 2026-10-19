const R="/Users/ueli/Documents/semio/";
const L=Bun.JSONC.parse(await Bun.file(R+".vscode/launch.json").text());
const S=Bun.JSONC.parse(await Bun.file(R+".vscode/🧩️launch.seed.jsonc").text());
const canon=(v:any):string=>v===undefined?"U":Array.isArray(v)?"["+v.map(canon).join(",")+"]":v&&typeof v==="object"?"{"+Object.keys(v).sort().map(k=>JSON.stringify(k)+":"+canon(v[k])).join(",")+"}":JSON.stringify(v);
const env=JSON.parse(await Bun.file(R+"🧰️framework/🔨️modules/🏃️process/🏗️build/🎛️environment/🧬️schema/🔣️.json").text()).definitions.Protocol.const.environment;
console.log("environment",JSON.stringify(env));
const proto=JSON.parse(await Bun.file(R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🚀️launch-routes/🧬️schema/🔣️.json").text()).definitions.Protocol.const;
const cats:Record<string,number>={};
function cat(c:any):string{
 if(c.presentation?.group?.startsWith?.("semio.dashboard."))return "dashboard";
 if(typeof c.name==='string'&&c.name.startsWith("🦀️ ")&&/^bun repo run \S+:(build|check|test) --configuration=(prototype|release) --parallel=1 -- /.test(c.command))return "crate-route";
 if(/^🛠️dev/.test(c.name))return "dev";
 return "other";
}
for(const [lab,doc] of [["launch",L],["seed",S]] as const){
 const m:Record<string,number>={};
 for(const c of doc.configurations){if(typeof c==='string'){m.placeholder=(m.placeholder??0)+1;continue}const k=cat(c);m[k]=(m[k]??0)+1}
 console.log(lab,JSON.stringify(m));
}
// crate-route family analysis
const fam=L.configurations.filter((c:any)=>cat(c)==="crate-route");
const bycrate=new Map<string,any[]>();
for(const c of fam){const m=c.command.match(/^bun repo run (\S+):(build|check|test) --configuration=(prototype|release) --parallel=1 -- (.*)$/)!;const key=m[1];const a=bycrate.get(key)??[];a.push({c,target:m[2],cfg:m[3],args:m[4]});bycrate.set(key,a)}
console.log("crate projects",bycrate.size);
let std=0,nonstd:string[]=[];
for(const [proj,rows] of bycrate){
  // expected name: 🦀️ <crate> <icon>target <icon>cfg
  const ok=rows.length===6&&rows.every(r=>{const t=proto.targets.find((x:any)=>x.id===r.target);const cf=proto.configurations.find((x:any)=>x.id===r.cfg);return r.c.name===`🦀️ ${proj} ${t.icon}${t.id} ${cf.icon}${cf.id}`&&canon(r.c.env)===canon(env)&&r.c.cwd==="${workspaceFolder}"&&r.c.type==="node-terminal"&&r.c.request==="launch"&&r.c.presentation.group===t.group&&["--lib --offline","--tests --offline","--offline"].includes(r.args)});
  if(ok)std++;else nonstd.push(proj+":"+rows.length);
}
console.log("standard",std,"nonstd",nonstd.length,nonstd.slice(0,20));
// order ranges per group
const og:Record<string,number[]>={};for(const c of fam){(og[c.presentation.group]??=[]).push(c.presentation.order)}
for(const g in og){const a=og[g].sort((x,y)=>x-y);console.log(g,a.length,a[0],a[a.length-1],"ints",a.every(Number.isInteger))}
// non-family orders per group max
const mg:Record<string,number>={};for(const c of L.configurations){if(cat(c)==="crate-route")continue;const g=c.presentation?.group;if(g&&typeof c.presentation.order==="number")mg[g]=Math.max(mg[g]??-1,c.presentation.order)}
console.log(JSON.stringify(mg));
