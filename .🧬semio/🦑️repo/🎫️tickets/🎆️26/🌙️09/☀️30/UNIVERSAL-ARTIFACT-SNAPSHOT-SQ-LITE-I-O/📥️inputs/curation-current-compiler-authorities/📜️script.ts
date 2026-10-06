import {readFileSync,writeFileSync} from "node:fs";
import {resolve,join} from "node:path";
const ticket=resolve(import.meta.dir,"../..");
const log=readFileSync(join(ticket,"🗑️generated/physical-curation-declaration-census-native-before.log"),"utf8").replace(/\u001b\[[0-9;]*m/gu,"");
const errors=[...log.matchAll(/^error\[(\w+)\]: ([^\n]+)\n([\s\S]*?)(?=^error\[|^error:|(?![\s\S]))/gmu)].map(match=>({code:match[1],message:match[2],site:match[3]!.match(/\s--> ([^\n]+)/u)?.[1]??"no primary location",body:match[3]}));
const groups=new Map<string,{count:number,sites:string[]}>();
for(const error of errors){const key=error.code+" "+error.message;const group=groups.get(key)??{count:0,sites:[]};group.count++;group.sites.push(error.site);groups.set(key,group);}
writeFileSync(join(ticket,"🗑️generated/physical-curation-current-compiler-authorities.json"),JSON.stringify(errors,null,2)+"\n");
writeFileSync(join(ticket,"📓️curation-current-native-compiler-authority-census.md"),"# Curation Current Native Compiler Authority Census\n\nActual registered `@semio-tech/sourcing-curation-rs:test-snapshot-sqlite-native` stopped compiler-only with94 diagnostics and zero assertions,1m16Nx. This bounded census records original lib-test prerequisites; no registration/provider runtime failure or success is inferred. Full receipt: [Native BEFORE](🗑️generated/physical-curation-declaration-census-native-before.log).\n\n"+[...groups].map(([message,group])=>"- "+group.count+" × `"+message.replaceAll("`","'")+"` at `"+group.sites[0]+"`").join("\n")+"\n");
console.log("[DEBUG] Curation current Native compiler census captured="+errors.length+" groups="+groups.size);
for(const [message,group]of groups)console.log(group.count+" "+message+" "+group.sites[0]);
