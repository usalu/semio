import {readFileSync,writeFileSync,existsSync,appendFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {loadGraph,closure} from "./graph.mjs";
const dir=process.argv[2],idx=Number(process.argv[3]),extra=process.argv.slice(4);
const batches=JSON.parse(readFileSync(dir+"/cc-tools/batches.json","utf8"));
const moved=new Set(["semio-framework-io-sqlite-snapshot-testing-backing","semio-framework-test-transient-preparation-law","semio-framework-test-typed-input-law","semio-framework-test-original-snapshot-owners","semio-framework-plugin-declared-verdict","semio-framework-plugin-child-group-wire-laws","semio-framework-plugin-surface-carrier-laws","semio-framework-plugin-surface-fixture"]);
if(process.env.CC_CMD)batches[idx]={section:"adhoc",cmd:process.env.CC_CMD};
let args=batches[idx].cmd.split(/\s+/).slice(1);
const mi=args.indexOf("--manifest-path");
const manifest=mi>=0?args[mi+1]:"Cargo.toml";
if(manifest==="🧰️framework/Cargo.toml"){const out=[];for(let i=0;i<args.length;i++){if(args[i]==="-p"&&moved.has(args[i+1])){i++;continue;}out.push(args[i]);}args=out;}
const name=String(idx).padStart(3,"0");
const log=`${dir}/🗑️generated/cc-b${name}.log`;
const refusedFile=dir+"/cc-tools/refused.txt";
const readRefused=()=>existsSync(refusedFile)?new Set(readFileSync(refusedFile,"utf8").split("\n").filter(Boolean).map(l=>l.split("\t")[0])):new Set();
const graphFile=manifest==="🧰️framework/Cargo.toml"?dir+"/cc-tools/meta-framework.json":manifest==="Cargo.toml"?dir+"/cc-tools/meta-root.json":null;
const graph=graphFile&&existsSync(graphFile)?loadGraph(graphFile):null;
const withDev=args.includes("--tests")||args.includes("--all-targets");
const env={...process.env,SEMIO_SCRIPT_POLICY:'{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":14400000}'};
const waitForeign=()=>{for(let i=0;i<360;i++){const r=spawnSync("pgrep",["-f","bun repo cargo|script.ts repo cargo"],{encoding:"utf8"});if(!r.stdout.trim())return;spawnSync("sleep",["10"]);}};
const launch=a=>{waitForeign();return spawnSync("bun",["repo","cargo","--store=shared","--configuration=prototype","--parallelism=4","--",...a],{cwd:process.cwd(),env,encoding:"utf8",maxBuffer:1<<30});};
let skipped=[],text="",r;
for(let attempt=0;attempt<14;attempt++){
 const refused=readRefused();
 const cur=[];skipped=[];
 for(let i=0;i<args.length;i++){if(args[i]==="-p"){const n=args[i+1];const bad=graph?[...closure(graph,n,withDev)].filter(x=>refused.has(x)):[];if(bad.length){skipped.push(n+" <- "+bad.join(","));i++;continue;}}cur.push(args[i]);}
 if(!cur.includes("-p")&&!cur.includes("--package")&&["check","build","test","clippy"].includes(cur[0])){text+="[cc] every requested crate is behind a refused crate\n";r={status:"skipped"};break;}
 r=launch([...cur,...extra]);
 const t=(r.stdout??"")+(r.stderr??"");text=t;
 if(/System image plist required|Bounded unique parent kernel roster required|Stale loaded repository|topology changed/u.test(t)&&attempt<10){spawnSync("sleep",["25"]);continue;}
 const ref=t.match(/Source refused before compiler: (\S+) \((Rust crate Source extent or boundary exceeded[^)]*|[^)]*boundary exceeded[^)]*)\)/u);
 if(ref&&!refused.has(ref[1])){appendFileSync(refusedFile,`${ref[1]}\t${ref[2]}\tbatch ${idx}\n`);continue;}
 if(/cannot update the lock file/u.test(t)){launch(["update","--workspace","--manifest-path",manifest]);continue;}
 if(/Original Cargo workspace topology changed/u.test(t)){spawnSync("sleep",["20"]);continue;}
 break;
}
writeFileSync(log,text);
const lines=text.split("\n");
const errs=lines.filter(l=>/(^|\s)error(\[E\d+\])?:/.test(l)&&!/^\[script\]/.test(l));
const warn=lines.filter(l=>/warning:/.test(l)).length;
writeFileSync(`${dir}/🗑️generated/cc-b${name}.errs`,errs.join("\n")+"\n");
writeFileSync(`${dir}/🗑️generated/cc-b${name}.skipped`,skipped.join("\n")+"\n");
writeFileSync(`${dir}/🗑️generated/cc-b${name}.done`,`exit=${r.status} errors=${errs.length} warnings=${warn} skipped=${skipped.length}\n`);
console.log(`batch ${idx} exit=${r.status} errors=${errs.length} warnings=${warn} skipped=${skipped.length}`);
