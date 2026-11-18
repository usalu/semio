import {readFileSync,writeFileSync,mkdirSync,readdirSync,cpSync,existsSync} from "node:fs";
import {pathToFileURL} from "node:url";
import {resolve,join,basename,dirname} from "node:path";
import {PATCHES,NEWS,NEWS_ROOT,applyEdits} from "./patches.mjs";
const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/";
const out=process.argv[2],base=process.env.R130_BASE==="1";mkdirSync(out,{recursive:true});
const absolute=(text,dir)=>text.replace(/(from\s*|import\s*\(\s*|import\s*)(["'])(\.{1,2}\/[^"']+)\2/gu,(all,head,quote,spec)=>head+quote+pathToFileURL(resolve(dir,spec)).href+quote);
const modules=new Map();
for(const p of PATCHES)if(p.file.startsWith(G)&&p.file.endsWith(".mjs"))modules.set(p.file,{file:p.file,edits:p.edits});
for(const file of[G+"🟨️.mjs",G+"🔌️load/🟨️.mjs",G+"🌊️stream-patch/🟨️.mjs",G+"🗃️row-log/🟨️.mjs"])if(!modules.has(file))modules.set(file,{file,edits:[]});
if(!base)for(const item of NEWS)if(item.to.endsWith(".mjs"))modules.set(item.to,{file:item.to,edits:[],source:join(NEWS_ROOT,item.from)});
const DEBUG=process.env.R130_DEBUG==="1",trace=(label)=>`;{const u=process.memoryUsage();console.error("[DEBUG] ${label} t="+Math.round(performance.now())+" rss="+Math.round(u.rss/1048576)+" heap="+Math.round(u.heapUsed/1048576)+" ab="+Math.round(u.arrayBuffers/1048576));}`;
const GCEVERY=process.env.R130_GCEVERY;
const debugEdits=[[" runtimeCollectGarbageV1(true);\n let paths;","trace0"],[" for(const name of[...append.keys()]","trace1"],[" const rowTotal=oldRows+fresh.length","trace2"],[" const marked=new Uint8Array(D.stringCount)","trace3"],[" const sortedFresh=","trace4"],[" const remap=new Uint32Array(D.stringCount)","trace5"],[" const factSink=createFactSinkV1","trace6"],[" const out=createFactSinkV1(maximumBytes);","trace7"]];
const assets=new Map();if(!base)NEWS.filter(item=>item.to.endsWith(".json")).forEach((item,i)=>assets.set(item.to,{name:"a"+i+".json",from:join(NEWS_ROOT,item.from)}));
const names=new Map();[...modules.keys()].forEach((file,i)=>names.set(file,"m"+i+".mjs"));
for(const{name,from}of assets.values())writeFileSync(join(out,name),readFileSync(from));
const WORLD=G+"🧪️tests/🏃️world/🟦️.ts";
const fix=text=>{for(const[file,name]of names)text=text.split(pathToFileURL(file).href).join(pathToFileURL(join(out,name)).href);for(const[file,{name}]of assets)text=text.split(pathToFileURL(file).href).join(pathToFileURL(join(out,name)).href);return text.split(pathToFileURL(WORLD).href).join(pathToFileURL(join(out,"world.ts")).href);};
for(const m of modules.values()){let text=m.source?readFileSync(m.source,"utf8"):base?readFileSync(m.file,"utf8"):applyEdits(readFileSync(m.file,"utf8"),m.edits,m.file);if(GCEVERY&&m.file.endsWith('stream-patch/🟨️.mjs')){text=text.replace('%32===31)runtimeCollectGarbageV1','%'+GCEVERY+'==='+(Number(GCEVERY)-1)+')runtimeCollectGarbageV1');}
if(process.env.R130_TWEAK&&m.file.endsWith('stream-patch/🟨️.mjs')){const tw=[['const remap=new Uint32Array(D.stringCount),stringSink=createFactSinkV1(maximumBytes);','const remap=new Uint32Array(D.stringCount);let stringSink=createFactSinkV1(maximumBytes);'],['const factSink=createFactSinkV1(maximumBytes),rowSink=createFactSinkV1(maximumBytes),kindIds','let factSink=createFactSinkV1(maximumBytes),rowSink=createFactSinkV1(maximumBytes);const kindIds'],[' const out=createFactSinkV1(maximumBytes);',' added.clear();byBytes.clear();fresh.length=0;refs=new Uint32Array(0);const out=createFactSinkV1(maximumBytes);'],['out.bytes(stringSink.result());','out.bytes(stringSink.result());stringSink=undefined;'],['out.bytes(factSink.result());out.bytes(rowSink.result());','out.bytes(factSink.result());factSink=undefined;out.bytes(rowSink.result());rowSink=undefined;']];for(const[a,b]of tw){if(!text.includes(a))throw Error('tweak anchor '+a.slice(0,50));text=text.replace(a,()=>b);}}
if(DEBUG&&m.file.endsWith('stream-patch/🟨️.mjs')){for(const[a,l]of debugEdits){if(!text.includes(a))throw Error('debug anchor '+a);text=text.replace(a,()=>trace(l)+'\n'+a);}}
writeFileSync(join(out,names.get(m.file)),fix(absolute(text,dirname(m.file))));}
const worldFile=G+"🧪️tests/🏃️world/🟦️.ts";{let source=readFileSync(worldFile,"utf8");const row=PATCHES.find(p=>p.file===worldFile);if(row&&!base)source=applyEdits(source,row.edits,worldFile);writeFileSync(join(out,"world.ts"),fix(absolute(source,dirname(worldFile))));}
const walkTs=dir=>readdirSync(dir,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walkTs(join(dir,e.name)):e.name.endsWith(".ts")?[join(dir,e.name)]:[]);
const tests=[];mkdirSync(join(out,"tests"),{recursive:true});writeFileSync(join(out,"tests","🐍️.py"),readFileSync(G+"🧪️tests/🐍️.py"));
for(const dir of process.argv.slice(3)){const target=join(out,"tests","t"+tests.length);tests.push({dir,target});cpSync(dir,target,{recursive:true});for(const file of walkTs(target)){const original=join(dir,file.slice(target.length+1));{let source=readFileSync(original,"utf8");const row=PATCHES.find(p=>p.file===original);if(row&&!base)source=applyEdits(source,row.edits,original);writeFileSync(file,fix(absolute(source,dirname(original))).split('join(here,"../🐍️.py")').join(JSON.stringify(G+"🧪️tests/🐍️.py")).split('join(here,"../../🐍️.py")').join(JSON.stringify(G+"🧪️tests/🐍️.py")));}}}
if(!base){const byDir=new Map();for(const item of NEWS)if(item.to.endsWith(".ts")||item.to.endsWith(".py")){const dir=dirname(item.to);if(!byDir.has(dir))byDir.set(dir,[]);byDir.get(dir).push(item);}
 for(const[dir,items]of byDir){const target=join(out,"tests","n"+tests.length);mkdirSync(target,{recursive:true});tests.push({dir,target});for(const item of items){const text=readFileSync(join(NEWS_ROOT,item.from),"utf8");writeFileSync(join(target,basename(item.to)),item.to.endsWith(".ts")?fix(absolute(text,dir)).split('join(here,"../🐍️.py")').join(JSON.stringify(G+"🧪️tests/🐍️.py")):text);}}}
writeFileSync(join(out,"tests.json"),JSON.stringify(tests));
writeFileSync(join(out,"names.json"),JSON.stringify({load:names.get(G+"🔌️load/🟨️.mjs"),store:names.get(G+"🟨️.mjs"),rowlog:names.get(G+"🗃️row-log/🟨️.mjs")??null,keytable:names.get(G+"🔑️key-table/🟨️.mjs")??null,patch:names.get(G+"🌊️stream-patch/🟨️.mjs")}));
console.log("staged",out,names.size,"modules",tests.length,"test dirs");
