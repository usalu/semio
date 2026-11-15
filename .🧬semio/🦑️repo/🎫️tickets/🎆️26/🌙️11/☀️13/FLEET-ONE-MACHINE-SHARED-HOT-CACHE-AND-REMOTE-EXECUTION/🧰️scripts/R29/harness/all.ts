import {readFileSync,writeFileSync} from "node:fs";
import * as baseFs from "node:fs";
import {createHash} from "node:crypto";
import {dirname,basename,join} from "node:path";
const inference=await import("../../🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import("../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any;
const ws="/Users/Shared/semio",LIB=ws+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",PROV=LIB+"/🏃️process/🔌️providers";
const want=JSON.parse(process.env.R5C_MIX??'{"native-target":150,"generic-target":120,"target-sources":100}') as Record<string,number>;
const sample:any[]=[];
{const seen=new Set<string>();
 for(const file of (process.env.R29_RECIPES??"").split(",").filter(Boolean)){
  for(const r of JSON.parse(readFileSync(file,"utf8")) as any[]){
   if(!r.target||!(r.fn in want))continue;
   const key=r.fn+"|"+r.projectRoot+"|"+JSON.stringify(r.target);if(seen.has(key))continue;seen.add(key);sample.push(r);
  }
 }
 const stride=Number(process.env.R29_STRIDE??"1"),offset=Number(process.env.R29_OFFSET??"0"),limit=Number(process.env.R29_LIMIT??"0");
 const kept=sample.filter((_,i)=>i%stride===offset);sample.length=0;sample.push(...(limit?kept.slice(0,limit):kept));
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
const specs=JSON.parse(readFileSync(process.env.R29_WORLDS!,"utf8")) as Record<string,{add?:Record<string,string>;append?:Record<string,string>;roots?:string[]}>;
const worlds:Record<string,{add:Record<string,string>;edit:Record<string,string>}>={};
for(const [name,spec] of Object.entries(specs)){worlds[name]={add:spec.add??{},edit:Object.fromEntries(Object.entries(spec.append??{}).map(([path,text])=>[path,live(path)+text]))};}
const sha=(b:any)=>createHash("sha256").update(b).digest("hex");
const files=(list:any[])=>list.filter(x=>typeof x==="string"&&x.startsWith("{workspaceRoot}/")).map(x=>x.slice(16));
const dict=new Map<string,number>(),id=(f:string)=>{let i=dict.get(f);if(i===undefined){i=dict.size;dict.set(f,i);}return i;};
const out:any={sample:sample.map(r=>[r.fn,r.projectRoot]),worlds:{}};
for(const name of Object.keys(worlds)){
 const only=specs[name].roots?new Set(specs[name].roots):undefined,todo=only?sample.filter(r=>only.has(r.projectRoot)):sample,port=makeWorld(worlds[name]),result:any={nodes:[],keys:[],own:[],refs:[],groups:{},errors:0,ms:0,rootNode:{},rootKey:{},sets:{},recipes:todo.length,sample:todo.map(r=>[r.fn,r.projectRoot])};const rootNodes=new Map<string,string[]>(),rootKeys=new Map<string,string[]>(),rootSets=new Map<string,Set<number>>(),rootPk=new Map<string,Set<string>>();
 const started=performance.now();
 await observation.withObservedWorldV1(port,()=>observation.observeInferenceSnapshotV1(async()=>{
  const cache=scriptInputs.createScriptInputCache({workspaceRoot:ws,lateClosures:false}),api=scriptInputs.closureApiV1(cache,ws);
  const groups=new Map<string,any[]>();
  for(const [group,list] of await api.sharedClosureGroups(PROV.slice(ws.length+1),"🔣️.json"))groups.set(group,list);
  for(const [group,list] of groups){result.groups[group]=sha(JSON.stringify(list));if(process.env.R29_GROUPFILES)(result.groupFiles??={})[group]=files(list).map(id);}
  const expand=(list:any[])=>{const set=new Set<string>(),packages=new Set<string>();for(const item of list){if(typeof item==="string"){if(item.startsWith("{workspaceRoot}/"))set.add(item.slice(16));}else if(item?.input){for(const x of groups.get(item.input)??[]){if(typeof x==="string"){if(x.startsWith("{workspaceRoot}/"))set.add(x.slice(16));}else if(x?.externalDependencies)for(const p of x.externalDependencies)packages.add(p);}}else if(item?.externalDependencies)for(const p of item.externalDependencies)packages.add(p);}return {set,packages};};
  const keyOf=(list:any[])=>{const {set,packages}=expand(list),hash=createHash("sha256");for(const f of [...set].sort()){hash.update(f).update("\0");try{hash.update(sha(port.readFileSync(ws+"/"+f)));}catch{hash.update("missing");}hash.update("\0");}hash.update(JSON.stringify([...packages].sort()));return hash.digest("hex");};
  let done=0;
  for(const r of todo){
   const {inputsDigest,...target}=r.target;
   try{
    let list:any[];
    if(r.fn==="native-target")list=await scriptInputs.nativeTargetCommandInputs(target,ws,undefined,cache,r.projectRoot);
    else if(r.fn==="generic-target")list=await scriptInputs.genericTargetCommandInputs(target,ws,scriptInputs.genericCommandFallbackInputs(ws),cache,r.projectRoot);
    else list=(await scriptInputs.targetScriptSources(target,ws,cache,r.projectRoot,r.nativeManifest,"queries")).closure??[];
    const nh=sha(JSON.stringify(list));result.nodes.push(nh);
    (rootNodes.get(r.projectRoot??"")??rootNodes.set(r.projectRoot??"",[]).get(r.projectRoot??"")!).push(nh);
    if(process.env.R29_SETS==="1"||process.env.R29_KEYS==="1"){const {set,packages}=expand(list),acc=rootSets.get(r.projectRoot??"")??rootSets.set(r.projectRoot??"",new Set()).get(r.projectRoot??"")!;for(const f of set)acc.add(id(f));const pk=rootPk.get(r.projectRoot??"")??rootPk.set(r.projectRoot??"",new Set()).get(r.projectRoot??"")!;for(const x of packages)pk.add(x);}
    if(name===(process.env.R29_OWNWORLD??"base")){result.own.push(files(list).map(id));result.refs.push(list.filter((x:any)=>x?.input).map((x:any)=>x.input));}
   }catch(error){result.errors++;result.nodes.push("error");result.keys.push("error");if(name===(process.env.R29_OWNWORLD??"base")){result.own.push([]);result.refs.push([]);}}
   if(++done%1000===0)console.log(name,"done",done,"of",todo.length,Math.round((performance.now()-started)/1000)+"s");
  }
 }));
 for(const [root,list] of rootNodes)result.rootNode[root]=sha([...list].sort().join("|"));const fileSha=new Map<string,string>(),rev=[...dict.keys()];if(process.env.R29_KEYS==="1")for(const [root,set] of rootSets){const hash=createHash("sha256");for(const f of [...set].map(i=>rev[i]).sort()){let h=fileSha.get(f);if(h===undefined){try{h=sha(port.readFileSync(ws+"/"+f));}catch{h="missing";}fileSha.set(f,h);}hash.update(f).update("\0").update(h).update("\0");}hash.update(JSON.stringify([...(rootPk.get(root)??[])].sort()));result.rootKey[root]=hash.digest("hex");}for(const [root,set] of rootSets)if(process.env.R29_SETS==="1")result.sets[root]=[[...set].sort((a,b)=>a-b),[...(rootPk.get(root)??[])].sort()];
 result.ms=Math.round(performance.now()-started);
 out.worlds[name]=result;
 console.log(name,"owners",sample.length,"errors",result.errors,"groups",Object.keys(result.groups).length,"ms",result.ms);
}
out.dict=[...dict.keys()];writeFileSync(process.env.R29_OUT!,JSON.stringify(out));
