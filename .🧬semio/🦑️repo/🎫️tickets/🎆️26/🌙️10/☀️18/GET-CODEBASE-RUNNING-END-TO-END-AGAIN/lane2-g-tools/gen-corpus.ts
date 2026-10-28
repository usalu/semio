/** lane2-g: emits the retention-model corpus (language-agnostic JSON) from the authored policy; usage: bun gen-corpus.ts <out.json> */
import {writeFileSync} from "node:fs";
import {UNIT_CACHE_POLICY_V1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";
const r=UNIT_CACHE_POLICY_V1.retention,l=UNIT_CACHE_POLICY_V1.layout;
const d=(c:string)=>c.repeat(64);
const row=(name:string,c:string,bytes=100)=>({name,sha256:d(c),bytes});
const R="cargo-unit-provenance-";
const a=(action:string,name:string,bytes=100)=>({action,name,bytes});
const plan=(id:string,lane:any[],shared:any[],expect:any)=>({id,lane,shared,expect});
const cases=[
 plan("an empty shared ledger receives every receipt in name order",[row(R+"b.json","b"),row(R+"a.json","a")],[],[a("publish",R+"a.json"),a("publish",R+"b.json")]),
 plan("the same bytes under the same name are dropped",[row(R+"a.json","a")],[row(R+"a.json","a")],[a("same",R+"a.json")]),
 plan("another digest under the same name is a conflict and never published",[row(R+"a.json","a")],[row(R+"a.json","b")],[a("conflict",R+"a.json")]),
 plan("a mixed set decides every name on its own",[row(R+"1.json","a"),row(R+"2.json","b"),row(R+"3.json","c")],[row(R+"2.json","b"),row(R+"3.json","d"),row(R+"9.json","e")],[a("publish",R+"1.json"),a("same",R+"2.json"),a("conflict",R+"3.json")]),
 plan("names outside the ledger pattern are ignored",[row(".tmp-x","a"),row("notes.txt","a"),row(R+".json","a"),row(R+"a/b.json","a"),row(R+"a.json\n","a"),row(R+"é.json","a"),row("cargo-unit-provenance-ok.json","a")],[],[a("publish","cargo-unit-provenance-ok.json")]),
 plan("a receipt over the byte bound is refused, one exactly at the bound is published",[row(R+"big.json","a",r.maximumFileBytes+1),row(R+"edge.json","b",r.maximumFileBytes)],[],[a("refuse",R+"big.json",r.maximumFileBytes+1),a("publish",R+"edge.json",r.maximumFileBytes)]),
 plan("an empty lane plans nothing",[],[row(R+"a.json","a")],[]),
];
const root="/s/u",shared={build:"/s/m/b/prototype",target:"/s/m/t/prototype"};
const origin=(id:string,buildDirectory:string,targetDirectory:string,expect:boolean,cacheRoot=root)=>({id,cacheRoot,shared,origin:{buildDirectory,targetDirectory},expect,lane:new RegExp("^"+cacheRoot.replaceAll("/","\\/")+"\\/l\\/[a-z0-9]{1,12}\\/b$").test(buildDirectory.replace("/./","/").replace(/\/$/u,""))});
const origins=[
 origin("the shared build and target directories are owned","/s/m/b/prototype","/s/m/t/prototype",true),
 origin("the shared build directory with another target is not owned","/s/m/b/prototype","/s/m/t/release",false),
 origin("a lane with its own target is owned","/s/u/l/ab12/b","/s/u/l/ab12/t",true),
 origin("a lane with the recipe slot inside its build directory is owned","/s/u/l/ab12/b","/s/u/l/ab12/b/t/0123456789abcdef",true),
 origin("a lane whose target is its own build directory is not owned","/s/u/l/ab12/b","/s/u/l/ab12/b",false),
 origin("a lane with the target of another lane is not owned","/s/u/l/ab12/b","/s/u/l/zz99/t",false),
 origin("dot segments are normalised before the decision","/s/u/l/ab12/./b/","/s/u/l/ab12/b/../t",true),
 origin("a target that escapes the lane through dot dot is not owned","/s/u/l/ab12/b","/s/u/l/ab12/b/../../x",false),
 origin("a lookalike lanes directory is not owned","/s/u/lx/ab12/b","/s/u/lx/ab12/t",false),
 origin("an upper case lane identifier is not owned","/s/u/l/AB12/b","/s/u/l/AB12/t",false),
 origin("a lane identifier of 13 characters is not owned","/s/u/l/abcdefghijklm/b","/s/u/l/abcdefghijklm/t",false),
 origin("a lane identifier of 12 characters is owned","/s/u/l/abcdefghijkl/b","/s/u/l/abcdefghijkl/t",true),
 origin("a build directory with a suffix is not owned","/s/u/l/ab12/bx","/s/u/l/ab12/t",false),
 origin("a build directory one level deeper is not owned","/s/u/l/ab12/c/b","/s/u/l/ab12/c/t",false),
 origin("a lane directory itself is not owned","/s/u/l/ab12","/s/u/l/ab12/t",false),
 origin("a lane of another cache root is not owned","/other/u/l/ab12/b","/other/u/l/ab12/t",false),
 origin("a lane of the same cache root under another store is not owned","/s/u/l/ab12/b","/s/u/l/ab12/t",false,"/t/u"),
 origin("the cache root itself is not owned","/s/u","/s/u/l/ab12/t",false),
];
writeFileSync(process.argv[2]!,JSON.stringify({version:1,policy:{ledgerPattern:r.ledgerPattern,maximumFiles:r.maximumFiles,maximumFileBytes:r.maximumFileBytes,layout:{lanes:l.lanes,build:l.build,target:l.target}},plan:cases,origin:origins},null,1)+"\n");
