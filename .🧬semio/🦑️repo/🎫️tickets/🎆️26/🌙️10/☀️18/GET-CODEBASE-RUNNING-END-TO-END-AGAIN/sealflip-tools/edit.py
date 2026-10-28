#!/usr/bin/env python3
"""✂️ [sealflip] The anchored edits of batch `sealflip`, replayable on a newer tree (the rebase script for the hot files): every edit is (file, old, new) and must find `old` exactly once (or `new` already, idempotent). usage: edit.py <root> [--check]"""
import os, sys
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
S = B + "/🏛️sealed"
OLD, NEW = "SEMIO_REPO_IMPLEMENTATION", "SEMIO_REPO_SEAL"
E = []
def e(rel, old, new): E.append((rel, old, new))
e(f"{S}/🔣️.json", f'"modeVariable": "{OLD}",', f'"modeVariable": "{NEW}",')
e(f"{S}/🔣️.json", '    "refreshDebounceMilliseconds": 120000\n  },\n', '    "refreshDebounceMilliseconds": 120000\n  },\n  "promotion": {\n    "rule": "explicit",\n    "agedMilliseconds": 86400000,\n    "noticeMilliseconds": 300000\n  },\n')
e(f"{S}/🔣️.json", '    "refused": {\n', '    "unpromoted": {\n      "en": "[implementation] The live tree has {count} change(s) that are not promoted (first: {first}); running the sealed snapshot {digest} promoted at {promoted}. Promote them with {variable}=promote, or run the live tree with {variable}=live.",\n      "de": "[implementation] Der Arbeitsbaum hat {count} nicht befoerderte Aenderung(en) (zuerst: {first}); es laeuft der versiegelte Schnappschuss {digest}, befoerdert um {promoted}. Mit {variable}=promote befoerdern oder mit {variable}=live den Arbeitsbaum ausfuehren."\n    },\n    "refused": {\n')
e(f"{S}/🧬️schema/🔣️.json", '"coalesceMilliseconds", "light", ', '"coalesceMilliseconds", "light", "promotion", ')
e(f"{S}/🧬️schema/🔣️.json", '        "refreshDebounceMilliseconds": { "type": "integer", "minimum": 0 }\n      }\n    },\n', '        "refreshDebounceMilliseconds": { "type": "integer", "minimum": 0 }\n      }\n    },\n    "promotion": {\n      "description": "🧷️ When a command promotes the live tree. `explicit`: it runs the promoted snapshot; only a `promote` run, a store without a snapshot or a snapshot older than `agedMilliseconds` whose tree drifted promote, and the drift is announced at most once per `noticeMilliseconds`. `drift`: a heavy command promotes every drifted tree before it runs, light commands follow `light`.",\n      "type": "object",\n      "additionalProperties": false,\n      "required": ["rule", "agedMilliseconds", "noticeMilliseconds"],\n      "properties": {\n        "rule": { "enum": ["explicit", "drift"] },\n        "agedMilliseconds": { "type": "integer", "minimum": 0 },\n        "noticeMilliseconds": { "type": "integer", "minimum": 0 }\n      }\n    },\n')
e(f"{S}/🧬️schema/🔣️.json", '"stale", "refused"]', '"stale", "refused", "unpromoted"]')
e(f"{S}/🧬️schema/🔣️.json", '        "refused": { "$ref": "#/definitions/Text" }\n', '        "refused": { "$ref": "#/definitions/Text" },\n        "unpromoted": { "$ref": "#/definitions/Text" }\n')
e(f"{S}/🟦️.ts", "refreshDebounceMilliseconds:number}>;readonly check:", "refreshDebounceMilliseconds:number}>;readonly promotion:Readonly<{rule:\"explicit\"|\"drift\";agedMilliseconds:number;noticeMilliseconds:number}>;readonly check:")
e(f"{S}/🟦️.ts", "/** 📖️ The text of a stored snapshot when the file is there,", """/** 🧷️ What a command does with the snapshot it found when promotion is explicit (`promotion.rule` = `explicit`): it runs it, so a peer's edit of the tool never moves the implementation digest (the key of the graph memo and of the Bazel plan) under a running fleet; the landing gate promotes with a `promote` run after a landing. Only a snapshot older than `promotion.agedMilliseconds`, or a restart after the running implementation yielded to a moved format stamp (it cannot follow the live tree without a new snapshot), promotes a drifted tree that is not known to be blocked on its own (a forgotten promotion cannot run day-old code for ever). The tree is compared at most once per `promotion.noticeMilliseconds` (the shared check record) and a drift that is not promoted is announced once per comparison.
 * @see ./🧪️tests/🗂️store/🟦️.ts the laws, including the independent model, ./🚦️trampoline/🟦️.ts the caller */
export function sealedExplicitActionV1(input:Readonly<{now:number;promotedAtMs:number;checked:Pick<SealedCheckedV1,"atMs"|"drifted">|undefined;restarted:boolean;drifted:()=>boolean;blocked:()=>boolean}>,policy:SealedPolicyV1=sealedPolicyV1):Readonly<{action:"run"|"promote";probed:boolean;drifted:boolean;notice:boolean}>{
 const urgent=input.restarted||input.now-input.promotedAtMs>=policy.promotion.agedMilliseconds,since=input.checked?input.now-input.checked.atMs:Infinity;
 if(!urgent&&since<policy.promotion.noticeMilliseconds)return Object.freeze({action:"run",probed:false,drifted:input.checked?.drifted??false,notice:false});
 if(!input.drifted())return Object.freeze({action:"run",probed:true,drifted:false,notice:false});
 const promote=urgent&&!input.blocked();
 return Object.freeze({action:promote?"promote":"run",probed:true,drifted:true,notice:!promote});
}
/** 📖️ The text of a stored snapshot when the file is there,""")
T = f"{S}/🚦️trampoline/🟦️.ts"
e(T, "readSealedReceiptV1,sealedEntryOfV1,sealedLanguagesV1,sealedLightActionV1,sealedLightCommandV1,sealedPolicyV1,sealedTextV1,sealedTreeDriftV1,writeSealedCheckedV1,type ImplementationModeV1,type SealedEntryV1,type SealedPointerV1}", "readSealedReceiptV1,sealedEntryOfV1,sealedExplicitActionV1,sealedLanguagesV1,sealedLightActionV1,sealedLightCommandV1,sealedPolicyV1,sealedTextV1,sealedTreeDriftV1,writeSealedCheckedV1,type ImplementationModeV1,type SealedEntryV1,type SealedPointerV1,type SealedPolicyV1}")
e(T, 'import door from"../../🚪️door/🔣️.json"with{type:"json"};', 'import door from"../../🚪️door/🔣️.json"with{type:"json"};\nimport stamps from"../../🧷️format-stamps/🧬️schema/🔣️.json"with{type:"json"};')
e(T, "readonly refresh?:(argv:readonly string[],environment:NodeJS.ProcessEnv)=>number|undefined}", "readonly refresh?:(argv:readonly string[],environment:NodeJS.ProcessEnv)=>number|undefined;readonly policy?:SealedPolicyV1}")
e(T, "/** ⏳️ What a start does with the pointer it found. A heavy command", "/** ⏳️ What a start does with the pointer it found. Under explicit promotion (the policy's `promotion.rule`) every command runs the promoted snapshot and a drift is only announced (`changed`), unless the snapshot is aged or this start is the restart of a command whose implementation yielded to a moved format stamp, see `sealedExplicitActionV1`. Under `drift` a heavy command")
e(T, "function startVerdictV1(root:string,store:SealedEntryV1,mode:ImplementationModeV1,pointer:SealedPointerV1|undefined,light:boolean,now=Date.now()):Readonly<{action:\"run\"|\"promote\"|\"refresh\";start:boolean}>{\n if(mode===\"promote\"||!pointer)return{action:\"promote\",start:false};\n if(!light){const receipt=readSealedReceiptV1(store,pointer.digest);return{action:!receipt?.tree||now-pointer.promotedAtMs>=sealedPolicyV1.coalesceMilliseconds&&!sealedTreeDriftV1(root,receipt.tree).fresh?\"promote\":\"run\",start:false};}\n const checked=readSealedCheckedV1(store,pointer.digest),plan=sealedLightActionV1({",
  "function startVerdictV1(root:string,store:SealedEntryV1,mode:ImplementationModeV1,pointer:SealedPointerV1|undefined,light:boolean,policy:SealedPolicyV1,restarted:boolean,now=Date.now()):Readonly<{action:\"run\"|\"promote\"|\"refresh\";start:boolean;changed:readonly string[]}>{\n if(mode===\"promote\"||!pointer)return{action:\"promote\",start:false,changed:[]};\n if(policy.promotion.rule===\"explicit\"){\n  let changed:readonly string[]=[];\n  const checked=readSealedCheckedV1(store,pointer.digest),plan=sealedExplicitActionV1({now,promotedAtMs:pointer.promotedAtMs,checked,restarted,drifted:()=>{const receipt=readSealedReceiptV1(store,pointer.digest);changed=receipt?.tree?sealedTreeDriftV1(root,receipt.tree).changed:[\".\"];return changed.length>0;},blocked:()=>{const known=readSealedBlockedV1(store);return known?.tree!=null&&sealedTreeDriftV1(root,known.tree).fresh;}},policy);\n  if(plan.probed)writeSealedCheckedV1(store,{version:1,entry:store.entry,digest:pointer.digest,atMs:now,drifted:plan.drifted,refreshedAtMs:checked?.refreshedAtMs??null});\n  return{action:plan.action,start:false,changed:plan.notice?changed:[]};\n }\n if(!light){const receipt=readSealedReceiptV1(store,pointer.digest);return{action:!receipt?.tree||now-pointer.promotedAtMs>=policy.coalesceMilliseconds&&!sealedTreeDriftV1(root,receipt.tree).fresh?\"promote\":\"run\",start:false,changed:[]};}\n const checked=readSealedCheckedV1(store,pointer.digest),plan=sealedLightActionV1({")
e(T, "return known?.tree!=null&&sealedTreeDriftV1(root,known.tree).fresh;}});\n if(plan.probed)writeSealedCheckedV1(store,{version:1,entry:store.entry,digest:pointer.digest,atMs:now,drifted:plan.drifted,refreshedAtMs:plan.start?now:checked?.refreshedAtMs??null});\n return{action:plan.action,start:plan.start};",
  "return known?.tree!=null&&sealedTreeDriftV1(root,known.tree).fresh;}},policy);\n if(plan.probed)writeSealedCheckedV1(store,{version:1,entry:store.entry,digest:pointer.digest,atMs:now,drifted:plan.drifted,refreshedAtMs:plan.start?now:checked?.refreshedAtMs??null});\n return{action:plan.action,start:plan.start,changed:[]};")
e(T, "function refreshV1(argv:readonly string[],environment:NodeJS.ProcessEnv):number|undefined{", "function refreshV1(argv:readonly string[],environment:NodeJS.ProcessEnv,policy:SealedPolicyV1=sealedPolicyV1):number|undefined{")
e(T, "env:{...environment,[sealedPolicyV1.modeVariable]:\"promote\"}});child.unref();", "env:{...environment,[policy.modeVariable]:\"promote\"}});child.unref();")
e(T, " let mode;\n try{mode=implementationModeV1(environment);}", " const policy=options.policy??sealedPolicyV1;\n let mode;\n try{mode=implementationModeV1(environment,policy);}")
e(T, "store=sealedEntryOfV1(root,entry),languages=", "store=sealedEntryOfV1(root,entry,policy),languages=")
e(T, "verdict=startVerdictV1(root,store,mode,pointer,light);", "verdict=startVerdictV1(root,store,mode,pointer,light,policy,(Number.parseInt(environment[stamps.$defs.Protocol.const.reexecutionEnvironment]??\"0\",10)||0)>0);")
e(T, "Date.now()-pointer.promotedAtMs>=sealedPolicyV1.light.refuseAfterMilliseconds){\n   const rows=readSealedBlockedV1(store)?.rows??[],shown=rows.slice(0,sealedPolicyV1.shownBlockedRows)", "Date.now()-pointer.promotedAtMs>=policy.light.refuseAfterMilliseconds){\n   const rows=readSealedBlockedV1(store)?.rows??[],shown=rows.slice(0,policy.shownBlockedRows)")
e(T, "rows:shown,variable:sealedPolicyV1.modeVariable})+\"\\n\");process.exitCode=75;return;", "rows:shown,variable:policy.modeVariable},policy)+\"\\n\");process.exitCode=75;return;")
e(T, "  const pid=(options.refresh??refreshV1)(argv,environment);", "  const pid=(options.refresh??refreshV1)(argv,environment,policy);")
e(T, "minutes:Math.floor((Date.now()-pointer.promotedAtMs)/60000),pid})+\"\\n\");\n }\n", "minutes:Math.floor((Date.now()-pointer.promotedAtMs)/60000),pid},policy)+\"\\n\");\n }else if(verdict.changed.length&&pointer)say(sealedTextV1(languages,\"unpromoted\",{count:verdict.changed.length,first:verdict.changed[0]!,digest:pointer.digest.slice(0,12),promoted:new Date(pointer.promotedAtMs).toTimeString().slice(0,8),variable:policy.modeVariable},policy)+\"\\n\");\n")
F = f"{S}/🧪️tests/🧫️fixtures/📜️trampoline.ts"
e(F, 'import{findSealedRootV1,sealedEntryOfV1}from"../../🟦️.ts";', 'import{findSealedRootV1,sealedEntryOfV1,sealedPolicyV1}from"../../🟦️.ts";')
e(F, "/** 🚦️ A front door script of a fixture workspace: the trampoline in front,", "/** 🚦️ A front door script of a fixture workspace (the promotion rule of the real policy, or the one `FIXTURE_PROMOTION` names): the trampoline in front,")
e(F, 'const root=findSealedRootV1(process.env.FIXTURE_ROOT!),argv=process.argv.slice(2);', 'const root=findSealedRootV1(process.env.FIXTURE_ROOT!),argv=process.argv.slice(2),rule=process.env.FIXTURE_PROMOTION;')
e(F, '{entry:"tool/entry.ts",promote});', '{entry:"tool/entry.ts",promote,...(rule==="explicit"||rule==="drift"?{policy:{...sealedPolicyV1,promotion:{...sealedPolicyV1.promotion,rule}}}:{})});')
TT = f"{S}/🧪️tests/🚦️trampoline/🟦️.ts"
e(TT, "LC_MESSAGES:undefined,SEMIO_REPO_IMPLEMENTATION:undefined,...environment}", "LC_MESSAGES:undefined,SEMIO_REPO_IMPLEMENTATION:undefined,SEMIO_REPO_SEAL:undefined,SEMIO_STALE_REEXEC:undefined,FIXTURE_PROMOTION:\"drift\",...environment}")
e(TT, "},300_000);\n", """},300_000);

test("explicit promotion: a valid live edit never moves the snapshot a heavy or a light command runs, the drift is announced once per comparison window in both languages, a promote run moves it, a store without a snapshot promotes, an aged snapshot or the restart after a moved format stamp promotes a drifted tree (and runs the last good one when the tree is blocked), and the dashboard's repo-domain switch (rust or go) never reaches the sealed layer",async()=>{
 expect(policyData.promotion.rule).toBe("explicit");
 const root=await fresh(),explicit={SEMIO_REPO_SEAL:"sealed",FIXTURE_PROMOTION:"explicit"},entry=sealedEntryOfV1(root,fixtureEntryV1);
 const cold=await run(root,["x"],explicit);
 expect(cold).toMatchObject({code:0,out:"DOOR A x\\n"});expect(cold.err).toContain("Promoted the sealed snapshot");
 const first=readSealedPointerV1(entry)!;
 setFixtureTreeV1(root,"B");
 const heavy=await run(root,["x"],explicit);
 expect(heavy).toMatchObject({code:0,out:"DOOR A x\\n"});expect(heavy.err).toContain("The live tree has 1 change(s) that are not promoted (first: tool/helper.ts)");expect(heavy.err).toContain("Der Arbeitsbaum hat 1 nicht befoerderte Aenderung(en) (zuerst: tool/helper.ts)");expect(heavy.err).not.toContain("Promoted the sealed snapshot");expect(heavy.err).not.toContain("Promoting");
 expect(await run(root,["x"],explicit)).toMatchObject({code:0,out:"DOOR A x\\n",err:""});
 expect(await run(root,["leases","status"],explicit)).toMatchObject({code:0,out:"DOOR A leases,status\\n",err:""});
 const checked=JSON.parse(readTrampolineFileV1(entry.checked,"utf8"));expect(checked).toMatchObject({digest:first.digest,drifted:true,refreshedAtMs:null});
 writeTrampolineFileV1(entry.checked,JSON.stringify({...checked,atMs:checked.atMs-10*60_000}));
 const again=await run(root,["leases","status"],explicit);
 expect(again).toMatchObject({code:0,out:"DOOR A leases,status\\n"});expect(again.err).toContain("not promoted");
 expect(readSealedPointerV1(entry)?.digest).toBe(first.digest);
 const promoted=await run(root,["x"],{...explicit,SEMIO_REPO_SEAL:"promote"});
 expect(promoted).toMatchObject({code:0,out:""});expect(promoted.err).toContain("Sealed snapshot");
 expect(await run(root,["x"],explicit)).toMatchObject({code:0,out:"DOOR B x\\n",err:""});
 setFixtureTreeV1(root,"C");backdateV1(root,25*60);
 const aged=await run(root,["x"],explicit);
 expect(aged).toMatchObject({code:0,out:"DOOR C x\\n"});expect(aged.err).toContain("Promoted the sealed snapshot");
 setFixtureTreeV1(root,"S");backdateV1(root,25*60);
 const blocked=await run(root,["x"],explicit);
 expect(blocked).toMatchObject({code:0,out:"DOOR C x\\n"});expect(blocked.err).toContain("The live tree is not promoted: tool/helper.ts:1:16");
 removeTrampolineFileV1(entry.checked,{force:true});
 const known=await run(root,["x"],explicit);
 expect(known).toMatchObject({code:0,out:"DOOR C x\\n"});expect(known.err).toContain("not promoted (first: tool/helper.ts)");expect(known.err).not.toContain("Promoting");
 setFixtureTreeV1(root,"B");
 const restart=await run(root,["x"],{...explicit,SEMIO_STALE_REEXEC:"1"});
 expect(restart).toMatchObject({code:0,out:"DOOR B x\\n"});expect(restart.err).toContain("Promoted the sealed snapshot");
 for(const value of["rust","go"]){
  expect(await run(root,["x"],{...explicit,SEMIO_REPO_IMPLEMENTATION:value})).toMatchObject({code:0,out:"DOOR B x\\n"});
  const plain=await run(root,["x"],{FIXTURE_PROMOTION:"explicit",SEMIO_REPO_IMPLEMENTATION:value});
  expect(plain.code).toBe(0);expect(plain.out).toMatch(/^(LIVE|DOOR B) x\\n$/u);
 }
},300_000);
""")
ST = f"{S}/🧪️tests/🗂️store/🟦️.ts"
e(ST, "light.refreshDebounceMilliseconds)as{action:string;probed:number;start:number};", "light.refreshDebounceMilliseconds)as{action:\"run\"|\"promote\"|\"refresh\";probed:number;start:number};")
e(ST, "import{readSealedCheckedV1,sealedCheckedOfV1,sealedLightActionV1,writeSealedCheckedV1}from\"../../🟦️.ts\";", "import{readSealedCheckedV1,sealedCheckedOfV1,sealedExplicitActionV1,sealedLightActionV1,writeSealedCheckedV1}from\"../../🟦️.ts\";")
e(ST, " expect(cases).toBe(7*2*2*(1+4*4));\n});\n", """ expect(cases).toBe(7*2*2*(1+4*4));
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
 const placeholders=(text:string)=>[...text.matchAll(/\\{([A-Za-z]+)\\}/gu)].map(match=>match[1]!).sort();
 for(const language of["en","de"] as const)expect(placeholders(policyData.texts.unpromoted[language])).toEqual(["count","digest","first","promoted","variable","variable"]);
});
""")
RM = f"{S}/README.md"
e(RM, "Fresh: it starts the sealed door. Drifted: it promotes first.", "Fresh: it starts the sealed door. Drifted: under the promotion rule `drift` it promotes first; under `explicit` (the policy's choice) it starts the sealed door and announces the drift (see Promotion policy).")
e(RM, "Frisch: es startet die versiegelte Tuer. Abgewichen: es befoerdert zuerst.", "Frisch: es startet die versiegelte Tuer. Abgewichen: unter der Befoerderungsregel `drift` befoerdert es zuerst; unter `explicit` (der Wahl der Richtlinie) startet es die versiegelte Tuer und meldet die Abweichung (siehe Befoerderungsregel).")
e(RM, "- **Rebuild cause.**", "- **Promotion policy (`promotion`).** `explicit`: every command runs the promoted snapshot, so a peer's edit of the tool never moves the implementation digest that keys the graph memo and the Bazel plan; the live tree is compared at most once per `noticeMilliseconds` (the shared `checked.json`) and unpromoted changes are announced in English and German (count and first path). Who promotes: the landing gate, with `SEMIO_REPO_SEAL=promote bun repo` after every landing that touches the tool (exit 0 required before its verdict runs); an agent that edits the tool runs its own commands with `SEMIO_REPO_SEAL=live` and never promotes. A store without a snapshot promotes, and a snapshot older than `agedMilliseconds` promotes a drifted tree that is not known to be blocked, so a forgotten promotion cannot run day-old code. Promotions are single-flight under the store lock; a running command keeps the snapshot it started with (the store keeps the previous snapshots until they were quiet for `quietMilliseconds`). `drift`: a heavy command promotes every drifted tree before it runs and light commands follow `light`. `SEMIO_REPO_IMPLEMENTATION` is the dashboard's repo-domain switch (`rust`|`go`) and never reaches this layer.\n- **Rebuild cause.**")
e(RM, "- **Ursache eines Neuaufbaus.**", "- **Befoerderungsregel (`promotion`).** `explicit`: jeder Befehl fuehrt den befoerderten Schnappschuss aus, so dass die Aenderung eines anderen Agenten am Werkzeug nie den Implementierungs-Digest bewegt, der das Graph-Memo und den Bazel-Plan schluesselt; der Arbeitsbaum wird hoechstens einmal je `noticeMilliseconds` verglichen (das geteilte `checked.json`), und nicht befoerderte Aenderungen werden auf Englisch und Deutsch gemeldet (Anzahl und erster Pfad). Wer befoerdert: das Landetor, mit `SEMIO_REPO_SEAL=promote bun repo` nach jeder Landung, die das Werkzeug beruehrt (Exit 0 vor seinen Urteilslaeufen); ein Agent, der das Werkzeug bearbeitet, fuehrt seine eigenen Befehle mit `SEMIO_REPO_SEAL=live` aus und befoerdert nie. Ein Speicher ohne Schnappschuss befoerdert, und ein Schnappschuss, der aelter als `agedMilliseconds` ist, befoerdert einen abgewichenen Baum, der nicht als blockiert bekannt ist, damit eine vergessene Befoerderung keinen tagealten Code ausfuehrt. Befoerderungen laufen einzeln unter der Sperre des Speichers; ein laufender Befehl behaelt den Schnappschuss, mit dem er gestartet ist (der Speicher behaelt die vorherigen, bis sie `quietMilliseconds` ruhig waren). `drift`: ein schwerer Befehl befoerdert jeden abgewichenen Baum, bevor er laeuft, leichte Befehle folgen `light`. `SEMIO_REPO_IMPLEMENTATION` ist der Repo-Domaenen-Schalter des Dashboards (`rust`|`go`) und erreicht diese Schicht nie.\n- **Ursache eines Neuaufbaus.**")
GD = "🧰️framework/🛍️products/🦑️repo/📓️build-guide.md"
e(GD, "- **Sealed snapshot**: live, opt-in. `SEMIO_REPO_IMPLEMENTATION=sealed` runs the tool from a content-addressed snapshot of its own sources, so a half-finished peer edit cannot stop your command; `live` captures the tree as before (the default until gate batch 8 flips it); `promote` promotes now and runs nothing (exit 1 when blocked). Details: `🏗️build/🏛️sealed/README.md`.",
  "- **Sealed snapshot**: `SEMIO_REPO_SEAL=sealed` runs the tool from a content-addressed snapshot of its own sources, so a half-finished peer edit can neither stop your command nor move the implementation digest that keys the graph memo and the Bazel plan; `live` captures the tree as before; `promote` promotes now and runs nothing (exit 1 when blocked); the default is `defaultMode` in `🏗️build/🏛️sealed/🔣️.json`. Promotion is explicit: a command runs the promoted snapshot and says at most every 5 minutes that the live tree has unpromoted changes; the landing gate runs `SEMIO_REPO_SEAL=promote bun repo` after every landing that touches the tool, an agent that edits the tool uses `SEMIO_REPO_SEAL=live` for its own runs, and a snapshot older than a day promotes a drifted tree by itself. `SEMIO_REPO_IMPLEMENTATION` is the dashboard's repo-domain switch (`rust`|`go`), not this one. Details: `🏗️build/🏛️sealed/README.md`.")
e(GD, "- **Versiegelter Schnappschuss**: live, als Opt-in. `SEMIO_REPO_IMPLEMENTATION=sealed` führt das Werkzeug aus einem inhaltsadressierten Schnappschuss seiner eigenen Quellen aus, sodass eine halbfertige Peer-Änderung deinen Befehl nicht stoppt; `live` erfasst den Baum wie bisher (Standard, bis Tor-Paket 8 ihn umlegt); `promote` befördert jetzt und führt nichts aus (Exit 1 bei Blockade). Einzelheiten: `🏗️build/🏛️sealed/README.md`.",
  "- **Versiegelter Schnappschuss**: `SEMIO_REPO_SEAL=sealed` führt das Werkzeug aus einem inhaltsadressierten Schnappschuss seiner eigenen Quellen aus, sodass eine halbfertige Peer-Änderung weder deinen Befehl stoppt noch den Implementierungs-Digest bewegt, der das Graph-Memo und den Bazel-Plan schlüsselt; `live` erfasst den Baum wie bisher; `promote` befördert jetzt und führt nichts aus (Exit 1 bei Blockade); der Standard ist `defaultMode` in `🏗️build/🏛️sealed/🔣️.json`. Die Beförderung ist explizit: ein Befehl führt den beförderten Schnappschuss aus und meldet höchstens alle 5 Minuten, dass der Arbeitsbaum nicht beförderte Änderungen hat; das Landetor führt nach jeder Landung, die das Werkzeug berührt, `SEMIO_REPO_SEAL=promote bun repo` aus, ein Agent, der das Werkzeug bearbeitet, nutzt `SEMIO_REPO_SEAL=live` für seine eigenen Läufe, und ein Schnappschuss, der älter als ein Tag ist, befördert einen abgewichenen Baum von selbst. `SEMIO_REPO_IMPLEMENTATION` ist der Repo-Domänen-Schalter des Dashboards (`rust`|`go`), nicht dieser. Einzelheiten: `🏗️build/🏛️sealed/README.md`.")
MS = f"{B}/🧭️invocation/🕸️graph/🧠️memo/🧬️schema/🔣️.json"
e(MS, '          "SEMIO_BUILD_REUSE_GRAPH",\n          "SEMIO_REPO_PROFILE"\n        ],', '          "SEMIO_BUILD_REUSE_GRAPH",\n          "SEMIO_REPO_IMPLEMENTATION",\n          "SEMIO_REPO_PROFILE",\n          "SEMIO_REPO_SEAL"\n        ],')
MT = f"{B}/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/🟦️.ts"
e(MT, 'SEMIO_BUILD_GRAPH_REQUESTED_AT:"1793081919763",SEMIO_BUILD_GRAPH_YIELD_ROUND:"2"}})).toBe(key);', 'SEMIO_BUILD_GRAPH_REQUESTED_AT:"1793081919763",SEMIO_BUILD_GRAPH_YIELD_ROUND:"2",SEMIO_REPO_SEAL:"sealed",SEMIO_REPO_IMPLEMENTATION:"rust"}})).toBe(key);')
WS = f"{B}/🔭️witness/🧬️schema/🔣️.json"
e(WS, '"SEMIO_REPO_PLAN_REUSE", "SEMIO_REPO_PROFILE", "SEMIO_REPO_SOURCE_MEMO",', '"SEMIO_REPO_PLAN_REUSE", "SEMIO_REPO_PROFILE", "SEMIO_REPO_SOURCE_MEMO", "SEMIO_REPO_SEAL", "SEMIO_REPO_IMPLEMENTATION",')
WT = f"{B}/🔭️witness/🧪️tests/🟦️.ts"
e(WT, '{SEMIO_BUILD_PARALLEL:"6",SEMIO_LEASE_PRIORITY:"batch",SEMIO_FIXTURE_VALUE:"a"});', '{SEMIO_BUILD_PARALLEL:"6",SEMIO_LEASE_PRIORITY:"batch",SEMIO_FIXTURE_VALUE:"a",SEMIO_REPO_SEAL:"sealed",SEMIO_REPO_IMPLEMENTATION:"rust"});')
e(WT, 'for(const name of["SEMIO_BUILD_PARALLEL","SEMIO_LEASE_PRIORITY","PWD","SHLVL","_"])expect(names.has(name),name).toBe(false);', 'for(const name of["SEMIO_BUILD_PARALLEL","SEMIO_LEASE_PRIORITY","SEMIO_REPO_SEAL","SEMIO_REPO_IMPLEMENTATION","PWD","SHLVL","_"])expect(names.has(name),name).toBe(false);')
TEXT_RENAME = [f"{ST}", TT, RM, f"{B}/🚪️door/🧪️tests/🟦️.ts", f"{B}/🚪️door/🧪️tests/🚦️startup/🟦️.ts"]
def apply(root, check):
    texts = {}
    for rel, old, new in E:
        p = os.path.join(root, rel); t = texts.get(rel) or open(p, encoding="utf8").read()
        if new in t: pass
        elif t.count(old) == 1: t = t.replace(old, new)
        else: sys.exit("ANCHOR %d× %s :: %s" % (t.count(old), rel[-60:], old[:80]))
        texts[rel] = t
    for rel in TEXT_RENAME:
        p = os.path.join(root, rel); t = texts.get(rel) or open(p, encoding="utf8").read()
        keep = "SEMIO_REPO_IMPLEMENTATION:undefined,SEMIO_REPO_SEAL:undefined"
        guarded = [keep, "SEMIO_REPO_IMPLEMENTATION:value", "`SEMIO_REPO_IMPLEMENTATION` is the dashboard", "`SEMIO_REPO_IMPLEMENTATION` ist der"]
        for i, g in enumerate(guarded): t = t.replace(g, "\0%d\0" % i)
        t = t.replace(OLD, NEW)
        for i, g in enumerate(guarded): t = t.replace("\0%d\0" % i, g)
        texts[rel] = t
    for rel, t in texts.items():
        p = os.path.join(root, rel)
        if open(p, encoding="utf8").read() == t: continue
        if check: print("would change", rel[-80:]); continue
        mode = os.stat(p).st_mode & 0o7777; tmp = p + ".sealflip.tmp"
        with open(tmp, "w", encoding="utf8") as h: h.write(t)
        os.chmod(tmp, mode); os.replace(tmp, p); print("edited", rel[-80:])
apply(os.path.abspath(sys.argv[1]), "--check" in sys.argv)
