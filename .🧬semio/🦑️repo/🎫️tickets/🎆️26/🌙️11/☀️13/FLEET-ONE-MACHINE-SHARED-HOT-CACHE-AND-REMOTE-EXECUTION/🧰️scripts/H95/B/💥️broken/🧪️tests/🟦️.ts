import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {appendFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import policy from "../🔣️.json" with {type:"json"};
import texts from "../📖️texts/🔣️.json" with {type:"json"};
import {brokenCrateOfV1,brokenDependentsV1,brokenDirtyCratesV1,brokenFormatV1,brokenIndexCratesV1,brokenMergeV1,brokenPackageNameV1,brokenParseCargoV1,brokenReadBoardsV1,brokenSectionV1,brokenWriteBoardV1,validateBrokenBoardV1,validateBrokenSectionV1,type BrokenBoardV1 } from "../🟦️.ts";
import {journalClockV1,journalFindEditorV1,journalMachineV1,journalReadV1,type JournalEventV1} from "../../📜️change-journal/🟦️.ts";
import {brokenSweepOnceV1,brokenWatchV1,type BrokenSweepPortsV1} from "../🏃️sweep/🟦️.ts";

const scratch=(run:(directory:string)=>void|Promise<void>)=>async()=>{const directory=mkdtempSync(join(tmpdir(),"semio-broken-"));try{await run(directory);}finally{rmSync(directory,{recursive:true,force:true});}};
const write=(directory:string,file:string,text:string)=>{mkdirSync(dirname(join(directory,file)),{recursive:true});writeFileSync(join(directory,file),text);};
const segment=(store:string,origin:string,number:number)=>join(store,"v1","origins",origin,"segments",String(number).padStart(12,"0")+".jsonl");
const line=(origin:string,atMs:number,path:string,operation="put")=>JSON.stringify({version:1,hlc:`${atMs}-000000-${origin}`,operation,path,sha256:"0".repeat(64),bytes:1,executable:false})+"\n";
const append=(store:string,origin:string,number:number,text:string)=>{mkdirSync(dirname(segment(store,origin,number)),{recursive:true});appendFileSync(segment(store,origin,number),text);};
const mini=(directory:string)=>{
 const root=join(directory,"live");
 write(root,"✏️s/Cargo.toml",'[workspace]\nmembers=["base","broken","user","lone"]\n');
 write(root,"✏️s/base/Cargo.toml",'[package]\nname="base"\nversion="0.1.0"\nedition="2021"\n');
 write(root,"✏️s/base/src/lib.rs","pub fn one()->u32{1}\n");
 write(root,"✏️s/broken/Cargo.toml",'[package]\nname="broken"\nversion="0.1.0"\nedition="2021"\n[dependencies]\nbase={path="../base"}\n');
 write(root,"✏️s/broken/src/lib.rs","pub fn two()->u32{ let x:u32=\"no\"; base::one()+x }\n");
 write(root,"✏️s/user/Cargo.toml",'[package]\nname="user"\nversion="0.1.0"\nedition="2021"\n[dependencies]\nbroken={path="../broken"}\n[dev-dependencies]\nbase={path="../base"}\n');
 write(root,"✏️s/user/src/lib.rs","pub fn three()->u32{broken::two()}\n");
 write(root,"✏️s/lone/Cargo.toml",'[package]\nname="lone"\nversion="0.1.0"\nedition="2021"\n');
 write(root,"✏️s/lone/src/lib.rs","pub fn four()->u32{4}\n");
 write(root,"✏️s/target/skip/Cargo.toml",'[package]\nname="skipped"\n');
 return root;
};

test("every text has both languages and the policy has its version",()=>{
 for(const row of Object.values(texts.messages))expect(row.en.length>0&&row.de.length>0).toBe(true);
 expect(policy.version).toBe(1);
});

test("a journal clock names the millisecond and the origin the change was written on",()=>{
 expect(journalClockV1("1795767099824-000000-g1-m01")).toEqual({atMs:1795767099824,origin:"g1-m01"});
 expect(journalClockV1("garbage")).toBeUndefined();
});

test("the journal is read from a cursor: a first read starts every origin at its end, an origin that appears later is read from its newest segment, later lines come once, a half line waits, segments roll over",scratch(directory=>{
 const store=join(directory,"store");
 append(store,"g1-m01",5,line("g1-m01",100,"old/a.rs"));
 const first=journalReadV1(store,{});
 expect(first.events).toEqual([]);expect(first.fresh).toEqual(["g1-m01"]);
 append(store,"g1-m01",5,line("g1-m01",200,"🧰️framework/x/🦀️.rs")+line("g1-m01",210,"b.rs","delete")+'{"hlc":"220-000000-g1-m01","path":"half');
 const second=journalReadV1(store,first.cursor);
 expect(second.events.map(event=>[event.origin,event.atMs,event.path,event.operation])).toEqual([["g1-m01",200,"🧰️framework/x/🦀️.rs","put"],["g1-m01",210,"b.rs","delete"]]);
 expect(journalReadV1(store,second.cursor).events).toEqual([]);
 append(store,"g1-m01",5,'.rs","operation":"put"}\n');
 append(store,"g1-m01",6,line("g1-m01",300,"c.rs"));
 append(store,"g1-m07",2,line("g1-m07",50,"ignored.rs"));
 const third=journalReadV1(store,second.cursor);
 expect(third.events.map(event=>event.path)).toEqual(["half.rs","c.rs","ignored.rs"]);
 expect(third.fresh).toEqual([]);
 const again=journalReadV1(store,third.cursor);
 expect(again.events).toEqual([]);
}));

test("a half line completed later is delivered once, and a rolled segment is read from its start",scratch(directory=>{
 const store=join(directory,"store");
 append(store,"g1-m01",1,line("g1-m01",1,"seed.rs"));
 const start=journalReadV1(store,{});
 append(store,"g1-m01",1,line("g1-m01",10,"a.rs")+'{"hlc":"20-000000-g1-m01","operation":"put","path":"b');
 const part=journalReadV1(store,start.cursor);
 expect(part.events.map(event=>event.path)).toEqual(["a.rs"]);
 append(store,"g1-m01",1,'.rs"}\n');
 append(store,"g1-m01",2,line("g1-m01",30,"c.rs"));
 const rest=journalReadV1(store,part.cursor);
 expect(rest.events.map(event=>event.path)).toEqual(["b.rs","c.rs"]);
}));

test("the last editor of a file is the newest journal entry of that path over all origins",scratch(directory=>{
 const store=join(directory,"store");
 append(store,"g1-m01",1,line("g1-m01",100,"✏️s/broken/src/lib.rs"));
 append(store,"g1-m07",3,line("g1-m07",500,"✏️s/broken/src/lib.rs")+line("g1-m07",600,"other.rs"));
 append(store,"g1-m02",9,line("g1-m02",400,"✏️s/broken/src/lib.rs"));
 expect(journalFindEditorV1(store,"✏️s/broken/src/lib.rs")).toEqual({origin:"g1-m07",atMs:500,path:"✏️s/broken/src/lib.rs"});
 expect(journalFindEditorV1(store,"nothing.rs")).toBeUndefined();
}));

test("the crate index maps directories to crates, reads every dependency table and skips target",scratch(directory=>{
 const root=mini(directory),index=brokenIndexCratesV1(root);
 expect([...index.byDirectory.values()].sort()).toEqual(["base","broken","lone","user"]);
 expect(index.dependencies.get("user")?.slice().sort()).toEqual(["base","broken"]);
 expect(brokenCrateOfV1("✏️s/broken/src/deep/mod.rs",index)).toEqual({crate:"broken",dir:"✏️s/broken"});
 expect(brokenCrateOfV1("README.md",index)).toBeUndefined();
}));

test("dirty crates are the owners of changed Rust files and their dependents; a lock file or an unknown manifest asks for the whole workspace",scratch(directory=>{
 const index=brokenIndexCratesV1(mini(directory)),event=(path:string,operation="put"):JournalEventV1=>({origin:"o",atMs:1,path,operation});
 const one=brokenDirtyCratesV1([event("✏️s/base/src/lib.rs"),event("docs/readme.md")],index);
 expect([...one.direct]).toEqual(["base"]);expect([...one.crates].sort()).toEqual(["base","broken","user"]);expect(one.workspace).toBe(false);
 expect(brokenDirtyCratesV1([event("Cargo.lock")],index).workspace).toBe(true);
 expect(brokenDirtyCratesV1([event("✏️s/new/Cargo.toml")],index).workspace).toBe(true);
 expect([...brokenDependentsV1(["lone"],index)]).toEqual(["lone"]);
}));

test("cargo package ids in both spellings name the crate",()=>{
 expect(brokenPackageNameV1("path+file:///w/✏️s/broken#0.1.0")).toBe("broken");
 expect(brokenPackageNameV1("path+file:///w/✏️s/dir#renamed@0.1.0")).toBe("renamed");
 expect(brokenPackageNameV1("broken 0.1.0 (path+file:///w/✏️s/broken)")).toBe("broken");
 expect(brokenPackageNameV1("registry+https://github.com/rust-lang/crates.io-index#serde@1.0.0")).toBe("serde");
});

const checkout=(()=>{let directory=import.meta.dir;while(!existsSync(join(directory,"MODULE.bazel"))&&dirname(directory)!==directory)directory=dirname(directory);return directory;})();
const toolchain=(():string|undefined=>{const run=spawnSync("rustup",["show","active-toolchain"],{cwd:checkout,encoding:"utf8"});return run.status===0?run.stdout.trim().split(/\s+/u)[0]:undefined;})();
const cargoEnvironment:NodeJS.ProcessEnv={...process.env,...toolchain===undefined?{}:{RUSTUP_TOOLCHAIN:toolchain}};
const hasCargo=spawnSync("cargo",["--version"],{env:cargoEnvironment}).status===0&&spawnSync("cargo",["metadata","--format-version","1","--no-deps","--offline"],{cwd:checkout,env:cargoEnvironment,stdio:"ignore",timeout:60000}).status!==null;
test.skipIf(!hasCargo)("the first error, the built and the blocked crates equal what real cargo reports for a workspace with a broken crate (oracle: cargo itself)",scratch(directory=>{
 const root=mini(directory),target=join(directory,"target"),run=spawnSync("cargo",["check","--workspace","--keep-going","--offline","--message-format=json"],{cwd:join(root,"✏️s"),env:{...cargoEnvironment,CARGO_TARGET_DIR:target},encoding:"utf8",maxBuffer:1<<26});
 expect(run.status).not.toBe(0);
 const parsed=brokenParseCargoV1(run.stdout,[join(root,"✏️s")]),index=brokenIndexCratesV1(root);
 expect([...parsed.errors.keys()]).toEqual(["broken"]);
 const error=parsed.errors.get("broken")!;
 expect(error.code).toBe("E0308");expect(error.file).toBe("broken/src/lib.rs");expect(error.line).toBe(1);
 expect([...parsed.built].sort()).toEqual(["base","lone"]);
 const rows=brokenMergeV1([],new Set(index.byDirectory.values()),parsed,index,{nowMs:1000,origin:"g1-m01",digest:"sha256:ab",editor:()=>({origin:"g1-m07",atMs:900,path:"p"})});
 expect(Object.fromEntries(rows.map(row=>[row.crate,row.status]))).toEqual({broken:"broken",user:"blocked",base:"ok",lone:"ok"});
 expect(rows[0]!.crate).toBe("broken");expect(rows[0]!.lastEditor?.origin).toBe("g1-m07");expect(rows[0]!.brokenSinceMs).toBe(1000);
}));

test("a crate that stays broken keeps its first moment, one that is fixed becomes ok and loses its error",()=>{
 const index={byDirectory:new Map([["a","a"]]),dependencies:new Map<string,readonly string[]>()},context={nowMs:5000,origin:"o",editor:()=>undefined};
 const first=brokenMergeV1([{crate:"a",status:"broken",checkedAtMs:1000,brokenSinceMs:1000,firstError:{message:"old"}}],new Set(["a"]),{errors:new Map([["a",{message:"new"}]]),built:new Set(),finished:false},index,context);
 expect([first[0]!.brokenSinceMs,first[0]!.firstError?.message]).toEqual([1000,"new"]);
 const fixed=brokenMergeV1(first,new Set(["a"]),{errors:new Map(),built:new Set(["a"]),finished:true},index,context);
 expect([fixed[0]!.status,fixed[0]!.firstError,fixed[0]!.brokenSinceMs]).toEqual(["ok",undefined,undefined]);
});

test("boards are published atomically, validated against the schema and merged: the freshest check of a crate wins; the text names error, place, age and editor in both languages",scratch(directory=>{
 const store=join(directory,"store"),row=(crate:string,status:"ok"|"broken"|"blocked"|"unchecked",checkedAtMs:number,extra={})=>({crate,status,checkedAtMs,...extra});
 const a:BrokenBoardV1={version:1,origin:"g1-m01",atMs:10_000,cursor:{},dirty:{},rows:[row("x","broken",1000,{brokenSinceMs:500,firstError:{code:"E0308",message:"mismatched types",file:"✏️s/x/src/lib.rs",line:3},lastEditor:{origin:"g1-m07",atMs:400,path:"✏️s/x/src/lib.rs"}}),row("y","ok",1000),row("z","unchecked",0)]};
 const b:BrokenBoardV1={version:1,origin:"g1-m02",atMs:20_000,cursor:{},dirty:{},rows:[row("x","ok",2000),row("z","blocked",1500),row("w","unchecked",0)]};
 const file=brokenWriteBoardV1(store,a);brokenWriteBoardV1(store,b);
 expect(existsSync(file)).toBe(true);expect(readdirSync(dirname(file)).filter(name=>name.endsWith(".tmp"))).toEqual([]);
 write(store,"v1/origins/g1-m09/broken.json","{not json");write(store,"v1/origins/g1-m10/broken.json",JSON.stringify({version:2}));
 const boards=brokenReadBoardsV1(store);
 expect(boards.map(board=>board.origin)).toEqual(["g1-m01","g1-m02"]);
 const section=brokenSectionV1(boards,30_000);
 expect(validateBrokenSectionV1(section)).toEqual([]);
 expect(Object.fromEntries(section.rows.map(item=>[item.crate,item.status]))).toEqual({z:"blocked",x:"ok",y:"ok",w:"unchecked"});
 expect(section.counts).toEqual({ok:2,broken:0,blocked:1,unchecked:1});
 const only=brokenSectionV1([a],30_000),english=brokenFormatV1(only,"en"),german=brokenFormatV1(only,"de");
 expect(english[0]).toContain("1 broken");expect(english.join("\n")).toContain("x: E0308 mismatched types (✏️s/x/src/lib.rs:3)");expect(english.join("\n")).toContain("g1-m07");
 expect(german.join("\n")).toContain("letzter Bearbeiter g1-m07");
 expect(brokenFormatV1(undefined,"de")[0]).toContain("noch kein Rechner");
 expect(()=>brokenWriteBoardV1(store,{...a,origin:""})).toThrow("broken-board-invalid");
 expect(validateBrokenBoardV1({...a,rows:[{crate:"x"}]}).length>0).toBe(true);
}));

test("the machine record names the origin and the store",scratch(directory=>{
 const root=join(directory,"live"),store=join(directory,"store");mkdirSync(store);
 write(root,".cache/f/machine.json",JSON.stringify({version:1,originId:"g1-m03",storeRoot:store}));
 expect(journalMachineV1(root,{})).toEqual({origin:"g1-m03",store});
 expect(journalMachineV1(join(directory,"none"),{})).toBeUndefined();
}));

const harness=(directory:string)=>{
 const root=mini(directory),store=join(directory,"store"),calls:string[][]=[],notes:string[]=[];
 let clock=1_000_000,answer:{code:number;stdout:string;stderr:string}|undefined={code:0,stdout:"",stderr:""};
 const message=(reason:string,crate:string,extra:Record<string,unknown>)=>JSON.stringify({reason,package_id:`path+file:///w/✏️s/${crate}#0.1.0`,...extra})+"\n";
 const ports:BrokenSweepPortsV1={now:()=>clock,notice:line=>notes.push(line),languages:["en"],snapshot:async path=>({root:path,digest:"sha256:"+"cd".repeat(32)}),check:async request=>{calls.push([...request.argv]);return answer;},sleep:async()=>{}};
 return {root,store,calls,notes,ports,tick:(ms:number)=>{clock+=ms;},answer:(value:typeof answer)=>{answer=value;},clock:()=>clock,message};
};

test("sweep: the first sweep checks the whole workspace and publishes status, first error and last editor; a quiet journal costs no check",scratch(async directory=>{
 const h=harness(directory);
 append(h.store,"g1-m01",1,line("g1-m01",1,"seed.rs"));
 append(h.store,"g1-m07",4,line("g1-m07",900_000,"✏️s/broken/src/lib.rs"));
 h.answer({code:101,stderr:"",stdout:h.message("compiler-artifact","base",{})+h.message("compiler-artifact","lone",{})+h.message("compiler-message","broken",{message:{level:"error",message:"mismatched types",code:{code:"E0308"},spans:[{is_primary:true,file_name:"✏️s/broken/src/lib.rs",line_start:1}]}})+JSON.stringify({reason:"build-finished",success:false})+"\n"});
 const first=await brokenSweepOnceV1({root:h.root,origin:"g1-m01",store:h.store},h.ports);
 expect(first.mode).toBe("workspace");
 expect(h.calls[0]!.slice(-1)).toEqual(["--workspace"]);
 expect(h.calls[0]!.includes("--keep-going")&&h.calls[0]!.includes("--message-format=json")).toBe(true);
 const board=JSON.parse(readFileSync(join(h.store,"v1/origins/g1-m01/broken.json"),"utf8")) as BrokenBoardV1;
 expect(validateBrokenBoardV1(board)).toEqual([]);
 const broken=board.rows.find(row=>row.crate==="broken")!;
 expect(broken.status).toBe("broken");expect(broken.firstError?.code).toBe("E0308");expect(broken.lastEditor?.origin).toBe("g1-m07");expect(broken.digest).toBe("sha256:"+"cd".repeat(32));
 expect(board.rows.find(row=>row.crate==="user")!.status).toBe("blocked");
 h.tick(60_000);
 const quiet=await brokenSweepOnceV1({root:h.root,origin:"g1-m01",store:h.store},h.ports);
 expect(quiet.mode).toBe("idle");expect(h.calls.length).toBe(1);
 expect(h.notes.some(note=>note.includes("nothing changed"))).toBe(true);
}));

test("sweep: only the changed crate and its dependents are checked, and only after the quiet period",scratch(async directory=>{
 const h=harness(directory);
 append(h.store,"g1-m01",1,line("g1-m01",1,"seed.rs"));
 await brokenSweepOnceV1({root:h.root,origin:"g1-m01",store:h.store},h.ports);
 h.calls.length=0;
 append(h.store,"g1-m07",4,line("g1-m07",h.clock()+1_000,"✏️s/base/src/lib.rs"));
 h.tick(5_000);
 const deferred=await brokenSweepOnceV1({root:h.root,origin:"g1-m01",store:h.store},h.ports);
 expect([deferred.mode,deferred.deferred,h.calls.length]).toEqual(["idle",1,0]);
 h.tick(30_000);
 h.answer({code:0,stderr:"",stdout:["base","broken","user"].map(crate=>h.message("compiler-artifact",crate,{})).join("")});
 const checked=await brokenSweepOnceV1({root:h.root,origin:"g1-m01",store:h.store},h.ports);
 expect(checked.mode).toBe("crates");expect(checked.checked).toBe(3);
 expect(h.calls[0]!.slice(-6)).toEqual(["-p","base","-p","broken","-p","user"]);
 expect(JSON.parse(readFileSync(join(h.store,"v1/origins/g1-m01/broken.json"),"utf8")).dirty).toEqual({});
}));

test("sweep: a check that fails without any compiler message becomes a (check) row, and one that could not run keeps the cursor so the changes are replayed",scratch(async directory=>{
 const h=harness(directory);
 append(h.store,"g1-m01",1,line("g1-m01",1,"seed.rs"));
 h.answer({code:101,stdout:"",stderr:"warning: x\nerror: failed to select a version for `nope`\n"});
 const faulty=await brokenSweepOnceV1({root:h.root,origin:"g1-m01",store:h.store},h.ports);
 const fault=faulty.board.rows.find(row=>row.crate==="(check)")!;
 expect(fault.status).toBe("broken");expect(fault.firstError?.message).toContain("failed to select a version");
 h.answer({code:0,stderr:"",stdout:["base","broken","user","lone"].map(crate=>h.message("compiler-artifact",crate,{})).join("")});
 h.tick(60_000);
 const fixed=await brokenSweepOnceV1({root:h.root,origin:"g1-m01",store:h.store,full:true},h.ports);
 expect(fixed.board.rows.some(row=>row.crate==="(check)")).toBe(false);
 append(h.store,"g1-m07",4,line("g1-m07",h.clock()-100_000,"✏️s/lone/src/lib.rs"));
 h.tick(60_000);
 const before=JSON.stringify(fixed.board.cursor);
 h.answer(undefined);
 const lost=await brokenSweepOnceV1({root:h.root,origin:"g1-m01",store:h.store},h.ports);
 expect(lost.checked).toBe(0);expect(JSON.stringify(lost.board.cursor)).toBe(before);expect(lost.board.lastSweep?.code).toBe(-1);expect(lost.board.dirty.lone).toBeDefined();
 h.answer({code:0,stderr:"",stdout:h.message("compiler-artifact","lone",{})});
 h.tick(60_000);
 const replay=await brokenSweepOnceV1({root:h.root,origin:"g1-m01",store:h.store},h.ports);
 expect(replay.mode).toBe("crates");expect(h.calls.at(-1)!.slice(-2)).toEqual(["-p","lone"]);
}));

test("the watch loop sweeps until cancelled and survives a failing sweep",scratch(async directory=>{
 const h=harness(directory),control=new AbortController();let rounds=0;
 const ports:BrokenSweepPortsV1={...h.ports,signal:control.signal,journal:()=>{rounds++;if(rounds===1)throw Error("journal exploded");if(rounds>=3)control.abort();return {events:[],cursor:{},fresh:[]};}};
 await brokenWatchV1({root:h.root,origin:"g1-m01",store:h.store},ports);
 expect(rounds).toBe(3);
 expect(h.notes.some(note=>note.includes("journal exploded"))).toBe(true);
}));

test("the fleet status carries the section: facts to report, schema, `--section broken` selection, detail and summary text in both languages, and the reader over the store",scratch(async directory=>{
 const {summarizeFleetV1,selectFleetSectionsV1,validateFleetReportV1,validateFleetSelectionV1,formatFleetReportV1,summarizeFleetReportV1,validateFleetSummaryV1}=await import("../../🚢️fleet-status/🟦️.ts"),{readFleetBrokenV1}=await import("../../🚢️fleet-status/📥️readers/🟦️.ts"),{fleetArgumentsV1}=await import("../../🚢️fleet-status/⌨️command/🟦️.ts");
 const store=join(directory,"store"),rows=Array.from({length:14},(_,index)=>({crate:`c${index}`,status:"broken" as const,checkedAtMs:1000,brokenSinceMs:900,firstError:{code:"E0425",message:"cannot find value",file:`✏️s/c${index}/src/lib.rs`,line:7},lastEditor:{origin:"g1-m07",atMs:800,path:`✏️s/c${index}/src/lib.rs`}}));
 brokenWriteBoardV1(store,{version:1,origin:"g1-m01",atMs:2000,cursor:{},dirty:{},rows:[...rows,{crate:"fine",status:"ok",checkedAtMs:1000}]});
 const facts=await readFleetBrokenV1({root:directory,environment:{SEMIO_FLEET_STORE_ROOT:store},language:"en",now:()=>3000});
 expect(facts.broken?.counts).toEqual({ok:1,broken:14,blocked:0,unchecked:0});
 expect(await readFleetBrokenV1({root:directory,environment:{SEMIO_FLEET_STORE_ROOT:join(directory,"empty")},language:"en",now:()=>3000})).toEqual({});
 for(const language of ["en","de"] as const){
  const report=summarizeFleetV1({version:1,atMs:3000,workspace:directory,sources:[],broken:facts.broken} as never,language);
  expect(validateFleetReportV1(report)).toEqual([]);
  const selection=selectFleetSectionsV1(report,["broken"]);
  expect(validateFleetSelectionV1(selection)).toEqual([]);
  const text=formatFleetReportV1(language,report,40,new Set(["broken"])).join("\n");
  expect(text).toContain(language==="en"?"14 broken":"14 kaputt");expect(text).toContain("c0: E0425 cannot find value (✏️s/c0/src/lib.rs:7)");expect(text).toContain("g1-m07");
  const summary=summarizeFleetReportV1(report);
  expect(validateFleetSummaryV1(summary)).toEqual([]);
  expect((summary as {broken?:{rows:unknown[]}}).broken?.rows.length).toBe(10);
 }
 expect(fleetArgumentsV1(["status","--section=broken"]).section).toBe("broken");
}));

test("the command: usage and a missing fleet identity are refused in the chosen language, one sweep publishes the board of this machine",scratch(async directory=>{
 const {runBrokenCliV1}=await import("../⌨️command/🟦️.ts"),root=mini(directory),store=join(directory,"store"),out:string[]=[];mkdirSync(store,{recursive:true});
 expect(await runBrokenCliV1(root,["sweep","--bogus","--lang","de"],{},line=>out.push(line))).toBe(2);
 expect(out[0]).toContain("Erwartet: repo fleet broken sweep");
 expect(await runBrokenCliV1(root,["sweep","--lang","en"],{},line=>out.push(line))).toBe(1);
 expect(out.at(-1)).toContain("no fleet identity");
 write(root,".cache/f/machine.json",JSON.stringify({version:1,originId:"g1-m05",storeRoot:store}));
 const calls:string[][]=[];
 const code=await runBrokenCliV1(root,["sweep","--once","--lang","en"],{},line=>out.push(line),{sweep:{snapshot:async path=>({root:path,digest:"sha256:"+"ab".repeat(32)}),check:async request=>{calls.push([...request.argv]);return {code:0,stdout:"",stderr:""};}}});
 expect(code).toBe(0);
 expect(calls[0]!.slice(-1)).toEqual(["--workspace"]);
 expect(brokenReadBoardsV1(store).map(board=>board.origin)).toEqual(["g1-m05"]);
 expect(out.some(line=>line.includes("Broken crates sweep: checking 4 crates (workspace)"))).toBe(true);
}));

test("`repo fleet broken sweep` is dispatched by the fleet command",scratch(async directory=>{
 const {runFleetCliV1}=await import("../../🚢️fleet-status/⌨️command/🟦️.ts"),out:string[]=[];
 expect(await runFleetCliV1(mini(directory),["broken","sweep","--bogus","--lang","en"],{},line=>out.push(line))).toBe(2);
 expect(out[0]).toContain("Expected: repo fleet broken sweep");
}));
