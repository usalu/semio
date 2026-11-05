import{readFileSync,writeFileSync}from"node:fs";
import{resolve}from"node:path";
import{pathToFileURL}from"node:url";
const root=process.cwd(),entry=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/🦀️.rs");
const{readRustSourceClosureV1}=await import(pathToFileURL(resolve(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts")).href);
const files=await readRustSourceClosureV1([entry],root,["test","debug_assertions","windows",'target_os="windows"','target_arch="x86_64"','target_family="windows"','target_pointer_width="64"'],async(path:string)=>{try{return readFileSync(path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return null;throw error;}});
const pattern=/serde_json::from_str\(include_str!\("[^"]*host-driver-policy\.json"\)\)\.expect\("explicit host test policy"\)/gu;
let matches=0,saved=0;const updates:{path:string;text:string}[]=[];
for(const file of files){if(!file.path.endsWith(".rs"))continue;const path=resolve(root,file.path),text=readFileSync(path,"utf8");if(path===entry)continue;const rows=[...text.matchAll(pattern)];if(!rows.length)continue;matches+=rows.length;saved+=rows.reduce((n,row)=>n+row[0].length-"crate::test_host_driver_policy()".length,0);updates.push({path,text:text.replace(pattern,"crate::test_host_driver_policy()")});}
console.log(JSON.stringify({matches,saved,files:updates.length,bytesBefore:files.reduce((n:number,f:any)=>n+f.bytes,0)}));
if(process.argv.includes("--apply")){if(matches<15)throw Error("Expected repeated explicit fixture initialization");for(const update of updates)writeFileSync(update.path,update.text);const text=readFileSync(entry,"utf8");if(text.includes("fn test_host_driver_policy"))throw Error("Root fixture policy already declared");writeFileSync(entry,text+'\n/** 🏛️ Shares the explicitly authored host policy across native facade laws. */\n#[cfg(test)]\npub(crate) fn test_host_driver_policy()->semio_framework_plugin_host::GuestRelayDriverPolicy{serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/host-driver-policy.json")).expect("explicit host test policy")}\n');}
