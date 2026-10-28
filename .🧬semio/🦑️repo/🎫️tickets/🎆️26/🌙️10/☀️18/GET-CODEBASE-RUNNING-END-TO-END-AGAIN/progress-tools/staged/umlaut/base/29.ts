/** 🗂️ Contract: the sealed store keeps a half-finished edit of the live tree away from every command. A snapshot is promoted only after every file parses and the candidate boots in a loader; a failure names the file in English and German and the last good snapshot keeps running; the same broken tree is never captured twice; a touched but unchanged tree is not drift; an older snapshot returns without a second validation; garbage collection keeps the current snapshot and the declared previous ones. The digest of a stored snapshot is the one the loader checks, and the stored text runs in the real loader. */
import{afterAll,expect,test}from"bun:test";
import{spawn}from"node:child_process";
import{existsSync,mkdirSync,readFileSync,readdirSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{join}from"node:path";
import Ajv from"ajv";
import ts from"typescript";
import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{buildSourceClosureIdentityV1,buildSourceClosureStreamLoaderV1}from"../../../📥️source/🔏️closure/🟦️.ts";
import policyData from"../../🔣️.json";
import schema from"../../🧬️schema/🔣️.json";
import{implementationModeV1,type ImplementationModeV1,readSealedBlockedV1,readSealedPointerV1,readSealedReceiptV1,readSealedSnapshotV1,sealedBlockedOfV1,sealedEntryOfV1,sealedLanguagesV1,sealedPointerOfV1,sealedReceiptOfV1,sealedTextV1,sealedTreeDriftV1,sealedTreeOfCaptureV1,sealedChangesV1,type SealedPolicyV1}from"../../🟦️.ts";
import{collectSealedStoreV1,resolveSealedImplementationV1,syntaxRowsOfSnapshotV1}from"../../📤️promotion/🟦️.ts";
import{fixtureCaptureV1,fixtureEntryV1,fixtureLoaderV1,labelOfTextV1,makeFixtureWorkspaceV1,setFixtureTreeV1,testPolicyV1}from"../🧫️fixtures/🟦️.ts";
import syntaxCorpus from"../🔬️syntax/🔣️.json";
import ledger from"../../../../⚡️caching/💾️storage/🧾️ledger/🔣️policy.json";

const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const world=(label="A",policy:SealedPolicyV1=testPolicyV1)=>{
 const root=makeFixtureWorkspaceV1(label);roots.push(root);
 const entry=sealedEntryOfV1(root,fixtureEntryV1,policy),count={captures:0,loads:0},capture=fixtureCaptureV1(root,fixtureEntryV1,()=>{count.captures++;}),notices:string[]=[];
 const run=(extra:Record<string,unknown>={})=>resolveSealedImplementationV1(root,entry,{capture,loader:fixtureLoaderV1,policy,environment:{SEMIO_LANGUAGE:undefined,LC_ALL:undefined,LANG:undefined},write:text=>notices.push(text),...extra});
 return{root,entry,count,notices,run,policy};
};
const ajv=new Ajv({strict:false}),definition=(name:string)=>ajv.compile({$ref:"#/definitions/"+name,definitions:schema.definitions} as object);

test("the policy is closed and bilingual with the same placeholders",()=>{
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
 expect(validate(policyData),JSON.stringify(validate.errors)).toBe(true);
 expect(validateJsonSchemaSubset(schema,policyData)).toEqual([]);
 const mutate=(change:(copy:any)=>void)=>{const copy=structuredClone(policyData);change(copy);return copy;};
 for(const bad of[mutate(copy=>{copy.extra=1;}),mutate(copy=>{delete copy.texts.blocked.de;}),mutate(copy=>{copy.defaultMode="maybe";}),mutate(copy=>{copy.keepPrevious=99;}),mutate(copy=>{copy.cacheDirectory="/absolute";})])expect(validate(bad)).toBe(false);
 const placeholders=(value:string)=>[...value.matchAll(/\{([A-Za-z]+)\}/gu)].map(match=>match[1]).sort().join(",");
 const walk=(table:any,path:string):void=>{if(typeof table.en==="string"){expect(placeholders(table.en),path).toBe(placeholders(table.de));return;}for(const[key,value] of Object.entries(table))walk(value,path+"."+key);};
 walk(policyData.texts,"texts");
  expect(policyData.defaultMode==="live"||policyData.defaultMode==="sealed").toBe(true);
});

test("the mode switch reads the declared variable and refuses an unknown value in both languages",()=>{
 expect(implementationModeV1({})).toBe(policyData.defaultMode as ImplementationModeV1);
 expect(implementationModeV1({[policyData.modeVariable]:"live"})).toBe("live");
 expect(implementationModeV1({[policyData.modeVariable]:"SEALED"})).toBe("sealed");
 expect(()=>implementationModeV1({[policyData.modeVariable]:"fast"})).toThrow(/Expected SEMIO_REPO_SEAL=sealed\|live[\s\S]*Erwartet SEMIO_REPO_SEAL=sealed\|live/u);
 expect(()=>implementationModeV1({[policyData.modeVariable]:"fast",SEMIO_LANGUAGE:"de"})).toThrow(/^Erwartet/u);
 expect(sealedLanguagesV1({LANG:"de_CH.UTF-8"})).toEqual(["de"]);
 expect(sealedLanguagesV1({})).toEqual(["en","de"]);
 expect(()=>sealedTextV1(["en"],"blocked",{})).toThrow(/placeholder/u);
});

test("a cold promotion publishes snapshot, receipt and pointer; the digest is the loader's; the stored text runs in the real loader; the next command captures nothing",async()=>{
 const w=world();
 const first=await w.run();
 expect(first.status).toBe("promoted");expect(w.count.captures).toBe(1);
 const pointer=readSealedPointerV1(w.entry)!,receipt=readSealedReceiptV1(w.entry,pointer.digest)!;
 expect(first.pointer.digest).toBe(pointer.digest);
 expect(pointer.digest).toBe(buildSourceClosureIdentityV1(JSON.parse(first.text)));
 expect(receipt.validation.loadMilliseconds).not.toBeNull();expect(receipt.changes).toEqual({previous:null,added:4,removed:0,changed:0,paths:["package.json","tool/data.json","tool/entry.ts","tool/helper.ts"]});
 expect(definition("Pointer")(pointer)).toBe(true);expect(definition("Receipt")(receipt)).toBe(true);expect(sealedPointerOfV1(pointer)).toBeDefined();expect(sealedReceiptOfV1(receipt)).toBeDefined();
 expect(readSealedSnapshotV1(w.entry,pointer)).toBe(first.text);
 expect(readFileSync(w.entry.pointer,"utf8")).toBe(JSON.stringify(pointer));
 const second=await w.run();
 expect(second.status).toBe("fresh");expect(w.count.captures).toBe(1);expect(second.text).toBe(first.text);
 const out=await new Promise<string>(accept=>{const child=spawn(process.execPath,["--eval",buildSourceClosureStreamLoaderV1,"--",pointer.digest,"x","y"],{stdio:["ignore","pipe","pipe","pipe"],env:{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"}});let output="";child.stdout!.on("data",data=>output+=data);child.stderr!.on("data",data=>output+=data);child.on("close",()=>accept(output));(child.stdio[3] as NodeJS.WritableStream).end(second.text);});
 expect(out).toBe("RAN A 1 x,y\n");
});

test("an edit is promoted, a touch is not drift, and the promotion records what changed",async()=>{
 const w=world();await w.run();
 const touched=new Date(Date.now()+5000);utimesSync(join(w.root,"tool/helper.ts"),touched,touched);
 expect((await w.run()).status).toBe("fresh");expect(w.count.captures).toBe(1);
 setFixtureTreeV1(w.root,"B");
 const next=await w.run();
 expect(next.status).toBe("promoted");expect(w.count.captures).toBe(2);expect(labelOfTextV1(next.text)).toBe("B");
 const receipt=readSealedReceiptV1(w.entry,next.pointer.digest)!;
 expect(receipt.changes).toMatchObject({added:0,removed:0,changed:1,paths:["tool/helper.ts"]});
 expect(next.pointer.previous).toHaveLength(1);
 const log=readFileSync(join(w.entry.directory,"promotions.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));
 expect(log).toHaveLength(2);expect(log[1]).toMatchObject({digest:next.pointer.digest,previous:log[0].digest,changed:1});
 expect(sealedChangesV1({digest:"a".repeat(64),files:[["a","1"],["b","2"]]},[["b","3"],["c","4"]])).toMatchObject({added:1,removed:1,changed:1});
 const tree=receipt.tree!;
 expect(sealedTreeDriftV1(w.root,tree).fresh).toBe(true);
 writeFileSync(join(w.root,"tool/extra.json"),"{}");
 expect(sealedTreeDriftV1(w.root,tree).fresh).toBe(true);
 writeFileSync(join(w.root,"tool/helper.ts"),'export const v="Z";');
 expect(sealedTreeDriftV1(w.root,tree)).toEqual({fresh:false,changed:["tool/helper.ts"]});
});

test("a load failure keeps the last good snapshot, names the file in English and German, and the same tree is not captured again; the fix is promoted and clears the record",async()=>{
 const w=world();const good=await w.run();
 setFixtureTreeV1(w.root,"X");
 const blocked=await w.run();
 expect(blocked.status).toBe("blocked");expect(blocked.text).toBe(good.text);expect(blocked.pointer.digest).toBe(good.pointer.digest);
 expect(blocked.rows[0]).toMatchObject({path:"tool/helper.ts"});
 const printed=w.notices.join("");
 expect(printed).toContain("The live tree is not promoted: tool/helper.ts");expect(printed).toContain("Der Arbeitsbaum wird nicht befoerdert: tool/helper.ts");
 expect(printed).toContain("SEMIO_REPO_SEAL=live");
 const record=readSealedBlockedV1(w.entry)!;expect(definition("Blocked")(record)).toBe(true);expect(sealedBlockedOfV1(record)).toBeDefined();
 const before=w.count.captures;
 expect((await w.run()).status).toBe("blocked");expect(w.count.captures).toBe(before);
 setFixtureTreeV1(w.root,"C");
 const fixed=await w.run();
 expect(fixed.status).toBe("promoted");expect(labelOfTextV1(fixed.text)).toBe("C");expect(readSealedBlockedV1(w.entry)).toBeUndefined();
});

test("a file that does not parse is found by the capture's failure, the sweep names it with line and column, and without any good snapshot the failure is the refusal",async()=>{
 const w=world();const good=await w.run();
 setFixtureTreeV1(w.root,"S");
 const blocked=await w.run();
 expect(blocked.status).toBe("blocked");expect(blocked.text).toBe(good.text);expect(blocked.rows[0]).toMatchObject({path:"tool/helper.ts",line:1,column:16});
 expect(w.notices.join("")).toContain("tool/helper.ts:1:16");
 const cold=world("S");
 await expect(cold.run()).rejects.toMatchObject({code:"SEMIO_IMPLEMENTATION_NONE",message:expect.stringMatching(/No sealed snapshot exists yet[\s\S]*Es gibt noch keinen versiegelten Schnappschuss/u)});
 const german=world("S");
 await expect(german.run({environment:{SEMIO_LANGUAGE:"de"}})).rejects.toThrow(/^\[implementation\] Es gibt noch keinen/u);
});

test("an older digest returns without a second validation, and collection keeps the current and the declared previous snapshots",async()=>{
 const policy:SealedPolicyV1={...testPolicyV1,keepPrevious:1,quietMilliseconds:0};
 const w=world("A",policy);const seen:string[]=[];
 const run=async(label:string)=>{setFixtureTreeV1(w.root,label);const loads=w.count.loads;const result=await w.run({load:async()=>{w.count.loads++;return{ok:true as const,milliseconds:1};}});seen.push(result.pointer.digest);return{result,validated:w.count.loads-loads};};
 expect((await run("A")).validated).toBe(1);
 expect((await run("B")).validated).toBe(1);
 const back=await run("A");
 expect(back.result.status).toBe("promoted");expect(back.validated).toBe(0);expect(back.result.pointer.digest).toBe(seen[0]);
 await run("C");await run("B");
 const files=readdirSync(w.entry.directory).filter(name=>/^[0-9a-f]{64}\.(seal\.)?json$/u.test(name));
 const pointer=readSealedPointerV1(w.entry)!;
 expect(new Set(files.map(name=>name.slice(0,64)))).toEqual(new Set([pointer.digest,...pointer.previous]));
 expect(pointer.previous).toHaveLength(1);
 expect(collectSealedStoreV1(w.root,w.entry,pointer,Date.now(),policy).removed).toBe(0);
 const tight:SealedPolicyV1={...policy,maximumBytes:1};
 expect(tight.maximumBytes).toBe(1);
});

test("a tree that changes while it is captured is captured again and the stable result is promoted; a tree that never holds still keeps the last good snapshot",async()=>{
 const w=world();const good=await w.run();
 let calls=0;const base=fixtureCaptureV1(w.root);
 setFixtureTreeV1(w.root,"B");
 const moving=async(signal:AbortSignal)=>{const candidate=await base(signal);calls++;if(calls===1)setFixtureTreeV1(w.root,"C");return candidate;};
 const settled=await w.run({capture:moving});
 expect(settled.status).toBe("promoted");expect(calls).toBe(2);expect(labelOfTextV1(settled.text)).toBe("C");
 let flips=0;const restless=async(signal:AbortSignal)=>{const candidate=await base(signal);setFixtureTreeV1(w.root,["A","C","B","X"][flips++%4]!);return candidate;};
 setFixtureTreeV1(w.root,"B");
 const unstable=await w.run({capture:restless});
 expect(unstable.status).toBe("unstable");expect(unstable.pointer.digest).toBe(settled.pointer.digest);expect(good.pointer.digest).not.toBe(settled.pointer.digest);
 expect(w.notices.join("")).toContain("kept changing while it was captured");expect(w.notices.join("")).toContain("aenderte sich waehrend der Erfassung weiter");
});

test("a waiting promoter can be cancelled, and the transpiler agrees with the independent TypeScript parser on the syntax corpus",async()=>{
 const w=world();await w.run();setFixtureTreeV1(w.root,"B");
 const abort=new AbortController();
 const holder=w.run({step:async(name:string)=>{if(name==="capture")await new Promise(accept=>setTimeout(accept,300));}});
 await new Promise(accept=>setTimeout(accept,60));
 const waiter=w.run({signal:abort.signal});setTimeout(()=>abort.abort(new Error("stopped")),80);
 await expect(waiter).rejects.toThrow("stopped");
 expect((await holder).status).toBe("promoted");
 for(const item of syntaxCorpus.cases){
  const mine=syntaxRowsOfSnapshotV1([{path:item.path,base64:Buffer.from(item.source).toString("base64")}]);
  const theirs=ts.transpileModule(item.source,{fileName:item.path,reportDiagnostics:true,compilerOptions:{target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext,jsx:ts.JsxEmit.Preserve}}).diagnostics??[];
  expect(mine.length===0,item.id+" "+JSON.stringify(mine)).toBe(item.valid);
  expect(theirs.length===0,item.id+" typescript").toBe(item.valid);
 }
 expect(existsSync(w.entry.directory)).toBe(true);
},60_000);

test("a drift check that runs inside a snapshot reads the live disk, not the snapshot's own image of the file (and the naive read is blind: negative control)",async()=>{
 for(const naive of[false,true]){
 const w=world();
 const original=readFileSync(fileURLToPath(new URL("../../🟦️.ts",import.meta.url)),"utf8").replace('"./🔣️.json"','"./policy.json"'),core=naive?original.replace("sealedHashV1(readLiveFileV1(absolute(root,path)))!==sha256","sealedHashV1(readFileSync(absolute(root,path)))!==sha256"):original;
 expect(core===original,"mutation applies").toBe(!naive);
 writeFileSync(join(w.root,"tool/sealed.ts"),core);writeFileSync(join(w.root,"tool/policy.json"),JSON.stringify(policyData));
 writeFileSync(join(w.root,"tool/drift.ts"),'import{readFileSync}from"node:fs";import{sealedTreeDriftV1}from"./sealed.ts";import"./helper.ts";export async function runCapturedBuildSourceV1(args:string[]){console.log(JSON.stringify(sealedTreeDriftV1(args[0]!,JSON.parse(readFileSync(args[1]!,"utf8")))));}');
 const candidate=await fixtureCaptureV1(w.root,"tool/drift.ts")(new AbortController().signal),text=JSON.stringify(candidate.snapshot),digest=buildSourceClosureIdentityV1(candidate.snapshot);
 const tree=sealedTreeOfCaptureV1(w.root,candidate.snapshot.files,candidate.structure)!,treeFile=join(w.root,"tree.json");writeFileSync(treeFile,JSON.stringify(tree));
 const inside=()=>new Promise<unknown>(accept=>{const child=spawn(process.execPath,["--eval",buildSourceClosureStreamLoaderV1,"--",digest,w.root,treeFile],{stdio:["ignore","pipe","pipe","pipe"],env:{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"}});let out="";child.stdout!.on("data",data=>out+=data);child.stderr!.on("data",data=>out+=data);child.on("close",()=>{try{accept(JSON.parse(out.trim().split("\n").pop()!));}catch{accept(out);}});(child.stdio[3] as NodeJS.WritableStream).end(text);});
 expect(await inside()).toEqual({fresh:true,changed:[]});
 setFixtureTreeV1(w.root,"B");
 expect(await inside()).toEqual(naive?{fresh:true,changed:[]}:{fresh:false,changed:["tool/helper.ts"]});
 expect(sealedTreeDriftV1(w.root,tree)).toEqual({fresh:false,changed:["tool/helper.ts"]});
 }
},120_000);

test("the store is a declared ledger area with the policy's budget, and collection keeps what a pointer needs while it enforces the budget oldest first across entries",()=>{
 const area=ledger.areas.find(row=>row.id==="implementation")!;
 expect(area).toMatchObject({root:"repository",kind:"tree",directory:policyData.cacheDirectory.split("/").at(-1),budget:{bytes:policyData.maximumBytes}});
 expect(ledger.roots.repository.directory+"/"+area.directory).toBe(policyData.cacheDirectory);
 const w=world("A");const store=join(w.root,...policyData.cacheDirectory.split("/"));
 const make=(entryKey:string,digests:string[],current:string)=>{
  const directory=join(store,entryKey);mkdirSync(directory,{recursive:true});
  writeFileSync(join(directory,"current.json"),JSON.stringify({version:1,entry:"e/"+entryKey,digest:current,loader:"b".repeat(64),bytes:10,files:1,promotedAtMs:1,previous:[]}));
  digests.forEach((digest,index)=>{const path=join(directory,digest+".json");writeFileSync(path,"x".repeat(1000));const at=new Date(Date.now()-(100-index)*60_000);utimesSync(path,at,at);});
  return directory;
 };
 const hex=(character:string)=>character.repeat(64),first=make("aaaa",[hex("1"),hex("2"),hex("3")],hex("3")),second=make("bbbb",[hex("4"),hex("5")],hex("5"));
 const policy:SealedPolicyV1={...testPolicyV1,quietMilliseconds:60_000,maximumBytes:3000},entry=sealedEntryOfV1(w.root,"e/aaaa",policy);
 const pointer={version:1 as const,entry:"e/aaaa",digest:hex("3"),loader:"b".repeat(64),bytes:10,files:1,promotedAtMs:1,previous:[]};
 const removed=collectSealedStoreV1(w.root,{...entry,directory:first,pointer:join(first,"current.json")},pointer,Date.now(),policy);
 expect(removed.removed).toBeGreaterThan(0);
 const left=(directory:string)=>readdirSync(directory).filter(name=>/^[0-9a-f]{64}\.json$/u.test(name)).map(name=>name[0]).sort();
 expect(left(first)).toContain("3");expect(left(second)).toContain("5");
 expect(left(first).length+left(second).length).toBeLessThan(5);
});

test("a candidate that loads but cannot do what the declared self-check commands do is not promoted, and the failure names the command and the place",async()=>{
 const w=world();const good=await w.run();
 setFixtureTreeV1(w.root,"R");
 const blocked=await w.run();
 expect(blocked.status).toBe("blocked");expect(blocked.pointer.digest).toBe(good.pointer.digest);
 expect(blocked.rows[0]!.message).toContain("repo leases status ended with status");expect(blocked.rows[0]!.message).toContain("self-check runtime failure");
 expect(w.notices.join("")).toContain("The live tree is not promoted");expect(w.notices.join("")).toContain("Der Arbeitsbaum wird nicht befoerdert");
 setFixtureTreeV1(w.root,"B");
 expect((await w.run()).status).toBe("promoted");
},60_000);

import{Database as LightModelDatabaseV1}from"bun:sqlite";
import{readSealedCheckedV1,sealedCheckedOfV1,sealedExplicitActionV1,sealedLightActionV1,writeSealedCheckedV1}from"../../🟦️.ts";

test("what a light command does with a snapshot equals the verdict of an independent SQL model over the whole grid, and it looks at the tree only when it must",()=>{
 const light=policyData.light,db=new LightModelDatabaseV1(":memory:"),now=10_000_000_000_000;
 const model=db.query(`select case
   when min(?1,coalesce(?2,1e18))<?3 then 'run'
   when not ?4 then 'run'
   when ?5 then 'run'
   when ?1>=?6 then 'promote'
   else 'refresh' end action,
  min(?1,coalesce(?2,1e18))>=?3 probed,
  (min(?1,coalesce(?2,1e18))>=?3 and ?4 and not ?5 and ?1<?6 and not (?7 is not null and ?7<?8)) start`);
 let cases=0;
 try{
  for(const age of[0,light.freshMilliseconds-1,light.freshMilliseconds,light.freshMilliseconds*3,light.promoteAfterMilliseconds-1,light.promoteAfterMilliseconds,light.refuseAfterMilliseconds+1])
   for(const since of[null,0,light.freshMilliseconds-1,light.freshMilliseconds,light.freshMilliseconds*40])
    for(const drifted of[false,true])for(const blocked of[false,true])for(const refreshedAge of[null,0,light.refreshDebounceMilliseconds-1,light.refreshDebounceMilliseconds]){
     if(since===null&&refreshedAge!==null)continue;
     const probes={drifted:0,blocked:0},plan=sealedLightActionV1({now,promotedAtMs:now-age,checked:since===null?undefined:{atMs:now-since,drifted:true,refreshedAtMs:refreshedAge===null?null:now-refreshedAge},drifted:()=>{probes.drifted++;return drifted;},blocked:()=>{probes.blocked++;return blocked;}});
     const expected=model.get(age,since,light.freshMilliseconds,drifted?1:0,blocked?1:0,light.promoteAfterMilliseconds,refreshedAge,light.refreshDebounceMilliseconds)as{action:"run"|"promote"|"refresh";probed:number;start:number};
     expect(plan.action,JSON.stringify({age,since,drifted,blocked,refreshedAge})).toBe(expected.action);
     expect(plan.probed).toBe(Boolean(expected.probed));
     expect(plan.start).toBe(Boolean(expected.start)&&Boolean(expected.probed));
     expect(probes.drifted).toBe(expected.probed?1:0);expect(probes.blocked).toBe(expected.probed&&drifted?1:0);
     cases++;
    }
 }finally{db.close();}
 expect(cases).toBe(7*2*2*(1+4*4));
});

test("what a command does with a snapshot under explicit promotion equals the verdict of an independent SQL model over the whole grid, it looks at the tree only when it must, and only an aged or restarted unblocked drift promotes",()=>{
 const promotion=policyData.promotion,db=new LightModelDatabaseV1(":memory:"),now=10_000_000_000_000;
 expect(promotion.noticeMilliseconds).toBeGreaterThan(0);expect(promotion.noticeMilliseconds).toBeLessThan(promotion.agedMilliseconds);
 const model=db.query(`select case when (?1>=?3 or ?8) and ?4 and not ?5 then 'promote' else 'run' end action,
  (?1>=?3 or ?8 or coalesce(?2,1e18)>=?6) probed,
  case when (?1>=?3 or ?8 or coalesce(?2,1e18)>=?6) then ?4 else ?7 end drifted,
  ((?1>=?3 or ?8 or coalesce(?2,1e18)>=?6) and ?4 and not ((?1>=?3 or ?8) and not ?5)) notice`);
 let cases=0;
 try{
  for(const age of[0,promotion.noticeMilliseconds,promotion.agedMilliseconds-1,promotion.agedMilliseconds,promotion.agedMilliseconds*3])
   for(const since of[null,0,promotion.noticeMilliseconds-1,promotion.noticeMilliseconds,promotion.agedMilliseconds*2])
    for(const recorded of[false,true])for(const restarted of[false,true])for(const drifted of[false,true])for(const blocked of[false,true]){
     if(since===null&&recorded)continue;
     const probes={drifted:0,blocked:0},plan=sealedExplicitActionV1({now,promotedAtMs:now-age,checked:since===null?undefined:{atMs:now-since,drifted:recorded},restarted,drifted:()=>{probes.drifted++;return drifted;},blocked:()=>{probes.blocked++;return blocked;}});
     const expected=model.get(age,since,promotion.agedMilliseconds,drifted?1:0,blocked?1:0,promotion.noticeMilliseconds,recorded?1:0,restarted?1:0)as{action:"run"|"promote";probed:number;drifted:number;notice:number};
     const label=JSON.stringify({age,since,recorded,restarted,drifted,blocked});
     expect(plan.action,label).toBe(expected.action);expect(plan.probed,label).toBe(Boolean(expected.probed));expect(plan.drifted,label).toBe(Boolean(expected.drifted));expect(plan.notice,label).toBe(Boolean(expected.notice));
     expect(probes.drifted).toBe(expected.probed?1:0);expect(probes.blocked).toBe(expected.probed&&drifted&&(age>=promotion.agedMilliseconds||restarted)?1:0);
     cases++;
    }
 }finally{db.close();}
 expect(cases).toBe(5*(1+4*2)*2*2*2);
 const placeholders=(text:string)=>[...text.matchAll(/\{([A-Za-z]+)\}/gu)].map(match=>match[1]!).sort();
 for(const language of["en","de"] as const)expect(placeholders(policyData.texts.unpromoted[language])).toEqual(["count","digest","first","promoted","variable","variable"]);
});

test("the light policy is ordered, its two messages exist in both languages with their placeholders, and the check record is closed, bound to one snapshot and written atomically",()=>{
 const light=policyData.light;
 expect(light.freshMilliseconds).toBeGreaterThan(0);expect(light.freshMilliseconds).toBeLessThan(light.promoteAfterMilliseconds);expect(light.promoteAfterMilliseconds).toBeLessThan(light.refuseAfterMilliseconds);expect(light.refreshDebounceMilliseconds).toBeGreaterThan(0);
 const placeholders=(text:string)=>[...text.matchAll(/\{([A-Za-z]+)\}/gu)].map(match=>match[1]!).sort();
 for(const[key,names]of Object.entries({stale:["digest","minutes","pid"],refused:["digest","hours","rows","variable"]}))for(const language of["en","de"] as const)expect(placeholders((policyData.texts as Record<string,Record<string,string>>)[key]![language]!),key+" "+language).toEqual(names);
 const w=makeFixtureWorkspaceV1("A");
 try{
  const entry=sealedEntryOfV1(w,fixtureEntryV1),digest="a".repeat(64),record={version:1 as const,entry:fixtureEntryV1,digest,atMs:5,drifted:true,refreshedAtMs:null};
  expect(readSealedCheckedV1(entry,digest)).toBeUndefined();
  mkdirSync(entry.directory,{recursive:true});writeSealedCheckedV1(entry,record);
  expect(readSealedCheckedV1(entry,digest)).toEqual(record);expect(readSealedCheckedV1(entry,"b".repeat(64))).toBeUndefined();
  expect(sealedCheckedOfV1({...record,extra:1})).toBeUndefined();expect(sealedCheckedOfV1({...record,drifted:"yes"})).toBeUndefined();expect(sealedCheckedOfV1({...record,atMs:-1})).toBeUndefined();expect(sealedCheckedOfV1({...record,refreshedAtMs:2.5})).toBeUndefined();expect(sealedCheckedOfV1({...record,digest:"zz"})).toBeUndefined();
  expect(readdirSync(entry.directory).filter(name=>name.endsWith(".tmp"))).toEqual([]);
  writeFileSync(entry.checked,"{torn");expect(readSealedCheckedV1(entry,digest)).toBeUndefined();
  const document=new Ajv({strict:false}).addSchema(schema,"seal").compile({$ref:"seal#/definitions/Checked"});expect(document(record)).toBe(true);expect(document({...record,atMs:"x"})).toBe(false);
 }finally{rmSync(w,{recursive:true,force:true});}
});
