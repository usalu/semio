import {readFileSync,writeFileSync} from "node:fs";
import * as baseFs from "node:fs";
import {createHash} from "node:crypto";
import {dirname,basename,join} from "node:path";
const inference=await import("../../🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import("../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any;
const ws="/Users/Shared/semio",LIB=ws+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",PROV=LIB+"/🏃️process/🔌️providers";
const want=JSON.parse(process.env.R5C_MIX??'{"native-target":150,"generic-target":120,"target-sources":100}') as Record<string,number>;
const sample:any[]=[];
for(const file of (process.env.R5C_RECIPES_LIST??"").split(",").filter(Boolean)){
 const recipes=JSON.parse(readFileSync(file,"utf8")) as any[],seen=new Set<string>(),pick:any[]=[];
 const onlyRoots=process.env.R5C_ONLY_ROOTS?new Set(JSON.parse(process.env.R5C_ONLY_ROOTS) as string[]):undefined;
 for(const r of recipes){if(!r.target||!(r.fn in want))continue;if(onlyRoots&&!onlyRoots.has(r.projectRoot))continue;const key=r.fn+"|"+r.projectRoot;if(seen.has(key))continue;seen.add(key);pick.push(r);}
 for(const fn of Object.keys(want)){const of=pick.filter(r=>r.fn===fn);if(!of.length)continue;const step=onlyRoots?1:Math.max(1,Math.floor(of.length/want[fn]));sample.push(...of.filter((_,i)=>i%step===0).slice(0,onlyRoots?100:want[fn]));}
}
const frozen={read:new Map<string,any>(),stat:new Map<string,any>(),dir:new Map<string,any>(),exists:new Map<string,boolean>()};
const once=<T>(table:Map<string,T>,key:string,make:()=>T):T=>{if(table.has(key)){const v=table.get(key);if(v instanceof Error)throw v;return v as T;}try{const v=make();table.set(key,v);return v;}catch(error){table.set(key,error as any);throw error;}};
const memoFs:any={
 readFileSync:(path:any,opt?:any)=>{const key=String(path),b=once(frozen.read,key,()=>baseFs.readFileSync(path));return opt&&(typeof opt==="string"||opt.encoding)?Buffer.from(b).toString(typeof opt==="string"?opt:opt.encoding):Buffer.from(b);},
 existsSync:(path:any)=>{const key=String(path);if(!frozen.exists.has(key))frozen.exists.set(key,baseFs.existsSync(path));return frozen.exists.get(key)!;},
 statSync:(path:any,opt?:any)=>opt?.bigint?baseFs.statSync(path,opt):once(frozen.stat,String(path),()=>baseFs.statSync(path)),
 readdirSync:(path:any,opt?:any)=>once(frozen.dir,String(path)+"|"+(opt?.withFileTypes?"t":"n"),()=>baseFs.readdirSync(path,opt) as any),
};
const makeWorld=(spec:{add:Record<string,string>;edit:Record<string,string>})=>{
 const over=new Map(Object.entries({...spec.add,...spec.edit})),added=new Set(Object.keys(spec.add)),children=new Map<string,Map<string,boolean>>(),vdirs=new Set<string>();
 let counter=0;
 for(const abs of added){let at=abs;while(at.length>1){const parent=dirname(at);if(!children.has(parent))children.set(parent,new Map());children.get(parent)!.set(basename(at),at===abs?false:true);if(at!==abs)vdirs.add(at);if(baseFs.existsSync(parent))break;at=parent;}}
 const anchor=(path:string)=>{let at=dirname(path);while(!baseFs.existsSync(at))at=dirname(at);return at;};
 const fake=(path:string,dir:boolean)=>{const real=baseFs.statSync(dir?anchor(path+"/x"):join(anchor(path),baseFs.readdirSync(anchor(path)).find(n=>baseFs.statSync(join(anchor(path),n)).isFile())??"."),{});const text=over.get(path);return Object.create(real,{size:{value:text===undefined?real.size:Buffer.byteLength(text)},ino:{value:1e12+(++counter)},mtimeMs:{value:real.mtimeMs},ctimeMs:{value:real.ctimeMs},isFile:{value:()=>!dir},isDirectory:{value:()=>dir}});};
 const overrides:any={
  readFileSync:(path:any,opt?:any)=>{const key=String(path);if(over.has(key)){const b=Buffer.from(over.get(key)!);return opt&&(typeof opt==="string"||opt.encoding)?b.toString((typeof opt==="string"?opt:opt.encoding)):b;}return memoFs.readFileSync(path,opt);},
  existsSync:(path:any)=>over.has(String(path))||vdirs.has(String(path))||memoFs.existsSync(path),
  statSync:(path:any,opt?:any)=>{const key=String(path);if(added.has(key))return fake(key,false);if(vdirs.has(key))return fake(key,true);const real=memoFs.statSync(path,opt);if(over.has(key)&&!opt?.bigint)return Object.create(real,{size:{value:Buffer.byteLength(over.get(key)!)}});return real;},
  lstatSync:(path:any,opt?:any)=>overrides.statSync(path,opt),
  readdirSync:(path:any,opt?:any)=>{const key=String(path),extra=children.get(key);const real=memoFs.existsSync(path)?memoFs.readdirSync(path,opt):[];if(!extra)return real;if(opt?.withFileTypes){const rows=[...real as any[]];for(const [name,dir] of extra)if(!rows.some((r:any)=>r.name===name))rows.push({name,parentPath:key,path:key,isFile:()=>!dir,isDirectory:()=>dir,isSymbolicLink:()=>false,isBlockDevice:()=>false,isCharacterDevice:()=>false,isFIFO:()=>false,isSocket:()=>false});return rows;}return [...new Set([...(real as string[]),...extra.keys()])].sort();},
  realpathSync:Object.assign((path:any,opt?:any)=>over.has(String(path))||vdirs.has(String(path))?String(path):baseFs.realpathSync(path,opt),{native:(path:any,opt?:any)=>over.has(String(path))||vdirs.has(String(path))?String(path):baseFs.realpathSync.native(path,opt)}),
 };
 return new Proxy(overrides,{get:(target,key)=>key in target?target[key]:(baseFs as any)[key]});
};
const live=(path:string)=>memoFs.readFileSync(path,"utf8");
const worlds:Record<string,{add:Record<string,string>;edit:Record<string,string>}>={
 base:{add:{},edit:{}},
 unreferenced:{add:{[LIB+"/🏗️build/🧪️tests/r5c-unreferenced/🟦️.ts"]:"export const unreferenced = 1;\n"},edit:{}},
 toolImport:{add:{},edit:{[LIB+"/⚡️caching/🦀️cargo/📜️script.ts"]:live(LIB+"/⚡️caching/🦀️cargo/📜️script.ts")+"\nimport \"./🗃️unit-cache/🧪️tests/🪟️windows/🧫️fixtures/🟦️echo.ts\";\n"}},
};
const walkFiles=(dir:string,out:string[]=[])=>{for(const e of baseFs.readdirSync(dir,{withFileTypes:true})){if(e.name==="node_modules"||e.name.startsWith("."))continue;const p=dir+"/"+e.name;if(e.isDirectory())walkFiles(p,out);else if(/\.(ts|mjs)$/u.test(e.name)&&!p.includes("/🧪️tests/"))out.push(p);}return out;};
const sources=[...walkFiles(LIB+"/🏗️build"),...walkFiles(LIB+"/⚡️caching")];
const neutral=(name:string,markers:string[])=>{const edit:Record<string,string>={};for(const file of sources){const text=live(file);let next=text;for(const marker of markers)next=next.replace(new RegExp("([\"'])(\\.{1,2}/[^\"']*"+marker.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")+"[^\"']*)\\1","gu"),'"node:fs"');if(next!==text)edit[file]=next;}worlds[name]={add:{},edit};};
neutral("beforeP14",["/🧳️portable/","/🌍️platform-class/"]);
neutral("beforeR2",["/🧽️settle/","/🎯️owner-scope/"]);
worlds.base2={add:{},edit:{}};
const only=(process.env.R5C_WORLDS??Object.keys(worlds).join(",")).split(",");
const sha=(b:any)=>createHash("sha256").update(b).digest("hex");
const files=(list:any[])=>list.filter(x=>typeof x==="string"&&x.startsWith("{workspaceRoot}/")).map(x=>x.slice(16));
const out:any={sample:sample.map(r=>[r.fn,r.projectRoot]),worlds:{}};
for(const name of only){
 const spec=worlds[name],port=makeWorld(spec);
 const result:any={nodes:[],keys:[],groups:{},errors:0};
 await observation.withObservedWorldV1(port,()=>observation.observeInferenceSnapshotV1(async()=>{
  const cache=scriptInputs.createScriptInputCache({workspaceRoot:ws,lateClosures:false}),api=scriptInputs.closureApiV1(cache,ws);
  const groups=new Map<string,any[]>();
  for(const [group,list] of await api.sharedClosureGroups(PROV.slice(ws.length+1),"🔣️.json"))groups.set(group,list);
  for(const [group,list] of groups){result.groups[group]=sha(JSON.stringify(list));if(process.env.R5C_DUMP&&group.startsWith("toolClosure:"))(result.dump??={})[group]=list.filter((x:any)=>typeof x==="string").map((x:string)=>x.slice(16));}
  const expand=(list:any[])=>{const set=new Set<string>(),packages=new Set<string>();for(const item of list){if(typeof item==="string"){if(item.startsWith("{workspaceRoot}/"))set.add(item.slice(16));}else if(item?.input){for(const x of groups.get(item.input)??[]){if(typeof x==="string"){if(x.startsWith("{workspaceRoot}/"))set.add(x.slice(16));}else if(x?.externalDependencies)for(const p of x.externalDependencies)packages.add(p);}}else if(item?.externalDependencies)for(const p of item.externalDependencies)packages.add(p);}return {set,packages};};
  const keyOf=(list:any[])=>{const {set,packages}=expand(list),hash=createHash("sha256");for(const f of [...set].sort()){hash.update(f).update("\0");try{hash.update(sha(port.readFileSync(ws+"/"+f)));}catch{hash.update("missing");}hash.update("\0");}hash.update(JSON.stringify([...packages].sort()));return hash.digest("hex");};
  for(const r of sample){
   const {inputsDigest,...target}=r.target;
   try{
    let list:any[];
    if(r.fn==="native-target")list=await scriptInputs.nativeTargetCommandInputs(target,ws,undefined,cache,r.projectRoot);
    else if(r.fn==="generic-target")list=await scriptInputs.genericTargetCommandInputs(target,ws,scriptInputs.genericCommandFallbackInputs(ws),cache,r.projectRoot);
    else list=(await scriptInputs.targetScriptSources(target,ws,cache,r.projectRoot,r.nativeManifest,"queries")).closure??[];
    result.nodes.push(sha(JSON.stringify(list)));result.keys.push(keyOf(list));if(process.env.R5C_SETS){const {set,packages}=expand(list),dict=(out.dict??=new Map<string,number>());(result.sets??=[]).push({f:[...set].map(f=>{let i=dict.get(f);if(i===undefined){i=dict.size;dict.set(f,i);}return i;}),p:[...packages].sort()});}
    if(process.env.R5C_LISTS)(result.lists??=[]).push({files:files(list).length,refs:list.filter((x:any)=>x?.input).map((x:any)=>x.input)});
   }catch(error){result.errors++;result.nodes.push("error");result.keys.push("error:"+String((error as any)?.message??error).slice(0,120));}
  }
 }));
 out.worlds[name]=result;
 console.log(name,"owners",sample.length,"errors",result.errors,"groups",Object.keys(result.groups).length);
}
if(out.dict)out.dict=[...out.dict.keys()];writeFileSync(process.env.R5C_OUT!,JSON.stringify(out));
