const R="/Users/ueli/Documents/semio/";
const L=Bun.JSONC.parse(await Bun.file(R+".vscode/launch.json").text());
const canon=(v:any):string=>v===undefined?"U":Array.isArray(v)?"["+v.map(canon).join(",")+"]":v&&typeof v==="object"?"{"+Object.keys(v).sort().map(k=>JSON.stringify(k)+":"+canon(v[k])).join(",")+"}":JSON.stringify(v);
const env={"BUN_RUNTIME_TRANSPILER_CACHE_PATH":"0","NODE_DISABLE_COMPILE_CACHE":"1"};
const proto=JSON.parse(await Bun.file(R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🚀️launch-routes/🧬️schema/🔣️.json").text()).definitions.Protocol.const;
const fam=L.configurations.filter((c:any)=>typeof c.name==='string'&&c.name.startsWith("🦀️ ")&&/^bun repo run \S+:(build|check|test) --configuration=(prototype|release) --parallel=1 -- /.test(c.command));
const rx=/^🦀️ (\S+) (🏗️build|🔎️check|🧪️test) (🛠️prototype|🚀️release)$/u;
const by=new Map<string,any[]>(); const odd:string[]=[];
for(const c of fam){const m=c.name.match(rx); if(!m){odd.push(c.name);continue} const a=by.get(m[1])??[];a.push(c);by.set(m[1],a)}
console.log("fam",fam.length,"crates",by.size,"oddnames",odd.length,odd.slice(0,10));
let std=0;const bad:Record<string,number>={};const examples:Record<string,string>={};
for(const [crate,rows] of by){
 const projs=new Set(rows.map(r=>r.command.match(/^bun repo run (\S+):/)![1]));
 const shapes=new Set<string>();
 let ok=rows.length===6&&projs.size===1;
 const reasons:string[]=[];
 if(rows.length!==6)reasons.push("count"+rows.length); if(projs.size!==1)reasons.push("multi-project");
 for(const r of rows){
  const m=r.name.match(rx)!; const t=proto.targets.find((x:any)=>m[2].endsWith(x.id)); const cf=proto.configurations.find((x:any)=>m[3].endsWith(x.id));
  const args=r.command.match(/ -- (.*)$/)![1]; shapes.add(t.id+":"+args);
  if(canon(r.env)!==canon(env))reasons.push("env"); if(r.presentation.group!==t.group)reasons.push("group"); if(r.cwd!=="${workspaceFolder}")reasons.push("cwd");
  const exp=`bun repo run ${[...projs][0]}:${t.id} --configuration=${cf.id} --parallel=1 -- ${args}`; if(r.command!==exp)reasons.push("cmd");
  if(Object.keys(r).join()!=="name,type,request,command,cwd,env,presentation")reasons.push("keys:"+Object.keys(r).join());
 }
 const lib=["build:--lib --offline","check:--lib --offline","test:--lib --offline"].every(s=>shapes.has(s)), tests=["build:--tests --offline","check:--tests --offline","test:--offline"].every(s=>shapes.has(s));
 if(!lib&&!tests)reasons.push("shape:"+[...shapes].join("|"));
 const k=[...new Set(reasons)].join(",")||"OK"; bad[k]=(bad[k]??0)+1; examples[k]??=crate;
 if(k==="OK")std++;
}
console.log("standard",std,JSON.stringify(bad),JSON.stringify(examples));
// project vs crate name
let same=0,diff:string[]=[];for(const [crate,rows] of by){const p=rows[0].command.match(/^bun repo run (\S+):/)![1];if(p===crate)same++;else diff.push(crate+"->"+p)}
console.log("project==crate",same,"differs",diff.length,diff.slice(0,8));
