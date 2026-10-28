/** 📌 Read-only: the slice closures an outer task and a nested task need inside one generation, exactly as the outer build pins (`sliceClosureV1` over the generation's fragments) and the nested command checks (`readPinnedGenerationV1`). `bun pinprobe.mjs <root> <outer project:target> <nested project:target> [generation manifest digest]` (`PINPROBE_STORE` reads another root's generation store with this root's code) prints both sizes, the names the nested closure needs beyond the outer one and, for each, the admission path (which owner's edge or input reference admitted it). */
import{readFileSync}from"node:fs";
import{join}from"node:path";
const [root,outer,nested,wanted]=process.argv.slice(2);
const L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations");
const generation=await import(join(L,"🟨️.mjs")),load=await import(join(L,"🔌️load/🟨️.mjs"));
const storeRoot=process.env.PINPROBE_STORE??join(root,".cache/b/g/m/generations"),cur=JSON.parse(readFileSync(join(storeRoot,"current.json"),"utf8")),digest=wanted??cur.manifest;
const engine=generation.createGenerationStoreV1({root:storeRoot,holds:async()=>true,resources:{facts:"gatefast-pinprobe"}});
const manifest=await engine.readManifestAt(digest);
const owners=Object.fromEntries(Object.entries(manifest.owners).map(([name,row])=>[name,row.root])),global=await engine.readGlobal(manifest);
const cache=new Map(),parent=new Map();
const fragment=async name=>{let view=cache.get(name);if(!view){view=load.sliceViewV1(await engine.readFragment(manifest.owners[name]));cache.set(name,view);}return view;};
const closure=async spec=>{const[project,target]=spec.split(/:(?=[^:]+$)/u);return load.sliceClosureV1({owners,external:Object.keys(global.externalNodes??{}),fragment,selection:{kind:"tasks",tasks:[{project,target}]},roots:[project]});};
const a=await closure(outer),b=await closure(nested),extra=[...b].filter(name=>!a.has(name)).sort();
const why=new Map();
for(const name of b){const view=await fragment(name);for(const edge of view.edges??[])if(!why.has(edge.target))why.set(edge.target,name+" (edge)");for(const[target,operation]of Object.entries(view.node?.targets??{}))for(const input of operation?.inputs??[])if(input&&typeof input==="object"&&input.projects!==undefined)for(const owner of[input.projects].flat())if(!why.has(owner))why.set(owner,name+":"+target+" (input "+String(input.input)+")");}
const trail=name=>{const out=[];let at=name;for(let step=0;step<12&&why.has(at);step++){out.push(why.get(at));at=why.get(at).split(/[ :]/u)[0];if(at===name)break;}return out.join(" <- ");};
console.log(JSON.stringify({generation:manifest.generation,outer,outerSize:a.size,nested,nestedSize:b.size,extra:extra.length}));
for(const name of extra.slice(0,40))console.log(" ",name,"<-",trail(name));
console.log("rss MB",Math.round(process.memoryUsage().rss/2**20));
