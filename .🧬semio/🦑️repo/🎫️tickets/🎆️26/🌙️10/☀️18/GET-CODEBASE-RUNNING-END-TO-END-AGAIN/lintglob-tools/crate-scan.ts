/** 🪈️ Reads every tracked crate of the live checkout through the cargoSourceSetV1 of the module in INF (a mirror's), printing one `[DEBUG]` row per unresolved crate and a histogram: bun crate-scan.ts <list of Cargo.toml dirs>; env INF=<inference dir>, ROOT=<live root>. */
import {readFileSync} from "node:fs";
const inference=await import(process.env.INF+"/\u{1F7E8}\u{FE0F}.mjs"),{cacheInternals,libraryBootstrap}=inference as any;await libraryBootstrap;
const roots=readFileSync(process.argv[2]!,"utf8").split("\n").filter(Boolean),histogram:Record<string,number>={};
for(const root of roots)for(const named of [true,false]){
 let row:string;try{const set=cacheInternals.cargoSourceSetV1(root,process.env.ROOT,new Map(),named,!named);row=set.resolved?"resolved":set.reason;if(!set.resolved&&set.reason!=="no-entries")console.log("[DEBUG]",named?"named":"dep",root.slice(-70),set.message);}catch(error){row="THROW "+String(error).slice(0,80);console.log("[DEBUG] throw",root.slice(-60),row);}
 histogram[(named?"named ":"dep   ")+row]=(histogram[(named?"named ":"dep   ")+row]??0)+1;
}
console.log("[DEBUG]",JSON.stringify(histogram));
