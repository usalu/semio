"""🧮 Anchored edit of the memo edit law (`🧠️memo/🧪️tests/✏️edit/🟦️.ts`) for batch `derived`: the fixture's derived fact becomes a real one (a model question asked like the planner's deriver asks, held by the project and supported by its pseudo owner in a real fact index), the adversarial corpus gains seven derived changes, the soundness law a negative control that accepts every derivation and the landed-memo assertion, the REAL watcher law two derived changes, and a new law covers the derivation child (refusals, child entry against a real generation store, wiring).
usage: python3 -I edit_law_derived.py <root>   (refuses when an anchor is missing; run once on the base law)"""
import os, sys
root = sys.argv[1]
path = os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/✏️edit/🟦️.ts")
t = open(path, encoding="utf8").read()
if "deriveModel" in t: sys.exit("[DEBUG] already edited")
reps = [
("""import {join} from "node:path";""", """import {dirname,join} from "node:path";"""),
("""import {canonicalJsonV1,compareCodePointsV1,createGenerationStoreV1,createNodeFilePortV1,type GenerationSealedV1} from "../../../../../🕸️graph/🕰️generations/🟨️.mjs";""",
"""import {GENERATION_POLICY_V1,canonicalJsonV1,compareCodePointsV1,createFactIndexV1,createGenerationStoreV1,createNodeFilePortV1,packFactIndexV1,type FactIndexV1,type GenerationSealedV1} from "../../../../../🕸️graph/🕰️generations/🟨️.mjs";
import {derivedOwnerNameV1} from "../../../../../🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs";"""),
("""import {memoFactsHoldV1} from "../../🚪️command/🟦️.ts";""", """import {memoFactsHoldV1} from "../../🚪️command/🟦️.ts";
import {answerGraphMemoDerivationV1,deriveGraphMemoSuspectsV1,graphMemoDerivationHeldV1,graphMemoDeriveAnswerFileV1,graphMemoSuspectsV1,readGraphMemoDeriveFileV1,settleGraphMemoDerivationV1,writeGraphMemoDeriveFileV1,type MemoDeriverV1} from "../../🧮️derive/🟦️.ts";"""),
(""""lib/d.ts":"d\\n",""", """"lib/d.ts":"import './x.ts';\\nd\\n","lib/x.ts":"x\\n","lib/y.ts":"y\\n","""),
("""/** 📸️ The facts of one reading session, taken with the real observation module, plus one derived fact naming `lib/d.ts` (the engine never checks a derived fact; the planner asks its question again). */
async function harvest(root:string):Promise<{rows:readonly MemoRowV1[];raw:readonly MemoRowV1[]}>{""", """const modelImports=(text:string):string[]=>[...text.matchAll(/^import ["']([^"']+)["'];?$/gmu)].map(match=>match[1]!);
/** 🧮️ The law's derived question, asked like the planner's deriver asks one (its own observation capture): how many imports of `lib/d.ts` name an existing file; it reads the file as TypeScript text and probes every import target, and fails on a `FAIL` marker. */
const deriveModel:MemoDeriverV1=async row=>{
 const [,file]=JSON.parse(row.path) as [string,string];
 const captured=await observation.captureObservedSourceInputsV1(()=>{const text=(observation.observedTypeScriptHost({readFile:(path:string)=>readFileSync(path,"utf8")} as never) as {readFile(path:string):string|undefined}).readFile(file)??"";if(text.includes("FAIL"))throw Error("[DEBUG] derivation failed");return modelImports(text).filter(spec=>observation.observedFs.existsSync(join(dirname(file),spec))).length;},undefined,false);
 return{fact:captured.value,rows:captured.observations as unknown as MemoRowV1[]};
};
/** 🔮️ The independent answer of the derived question on the tree as it is now (plain file reads, no observation layer); `undefined` when the question fails. */
const modelAnswer=(root:string):number|undefined=>{try{const text=readFileSync(join(root,"lib/d.ts"),"utf8");return text.includes("FAIL")?undefined:modelImports(text).filter(spec=>{try{lstatSync(join(root,"lib",spec));return true;}catch{return false;}}).length;}catch{return undefined;}};
/** 📸️ The facts of one reading session, taken with the real observation module, and one derived fact (how many imports of `lib/d.ts` name an existing file) held by the project and supported by its pseudo owner with the rows its derivation read, all in a real fact index (`rows` are its rows, so a position is an id). */
async function harvest(root:string):Promise<{rows:readonly MemoRowV1[];raw:readonly MemoRowV1[];index:FactIndexV1}>{"""),
(""" const raw=captured.observations as unknown as MemoRowV1[];
 return{raw,rows:[...raw,{kind:"derived",path:JSON.stringify(["memo-edit-law:question",at("lib/d.ts")]),fact:"answer"}]};""", """ const raw=captured.observations as unknown as MemoRowV1[],question={kind:"derived",path:JSON.stringify(["memo-edit-law:imports",at("lib/d.ts")]),fact:null as unknown},answer=await deriveModel(question),derived={...question,fact:answer.fact};
 const index=createFactIndexV1(new Map([["proj",[...raw,derived]],[derivedOwnerNameV1(derived),[derived,...answer.rows]]]) as never) as FactIndexV1;
 return{raw,rows:index.rows as readonly MemoRowV1[],index};"""),
(""" {name:"edit of a file a derived fact names",serve:false,apply:root=>appendFileSync(join(root,"lib/d.ts"),"more\\n")},""", """ {name:"content-only edit of a file a derived fact reads (the answer is kept)",serve:true,apply:root=>appendFileSync(join(root,"lib/d.ts"),"more\\n")},
 {name:"atomic save of a derived fact's input keeping the answer",serve:true,transient:["lib/d.ts.tmp"],apply:root=>atomic(root,"lib/d.ts","import './x.ts';\\nd2\\n")},
 {name:"edit that changes a derived answer (an import added to its input)",serve:false,apply:root=>appendFileSync(join(root,"lib/d.ts"),"import './y.ts';\\n")},
 {name:"a derived answer kept, but its question now reads another file",serve:false,apply:root=>writeFileSync(join(root,"lib/d.ts"),"import './y.ts';\\nd\\n")},
 {name:"a derived question that now fails",serve:false,apply:root=>appendFileSync(join(root,"lib/d.ts"),"// FAIL\\n")},
 {name:"content edit of a file a derived question probes and the project does not read",serve:true,apply:root=>appendFileSync(join(root,"lib/x.ts"),"more\\n")},
 {name:"delete a file a derived question probes",serve:false,apply:root=>unlinkSync(join(root,"lib/x.ts"))},"""),
(""" const {rows,raw}=await harvest(root),frontier=await walk(root);
 const built=await buildGraphMemoRelevanceV1""", """ const {rows,raw,index}=await harvest(root),frontier=await walk(root);
 const built=await buildGraphMemoRelevanceV1"""),
(""" const edit:GraphMemoEditPortsV1={rows:async(index,ids)=>index===INDEX?ids.map(id=>rows[id]!):undefined,frontier:async digest=>digest===FRONTIER?{frontier,descend:(name:string)=>descend(name),member}:undefined};""",
""" const edit:GraphMemoEditPortsV1={rows:async(digest,ids)=>digest===INDEX?ids.map(id=>rows[id]!):undefined,frontier:async digest=>digest===FRONTIER?{frontier,descend:(name:string)=>descend(name),member}:undefined,derive:async({stale,derived})=>settleGraphMemoDerivationV1({index,stale,derived,ask:suspects=>deriveGraphMemoSuspectsV1({index,suspects,derive:deriveModel})})};"""),
(""" return{rows,raw,frontier,built,journal,directory,open,edit};""", """ return{rows,raw,index,frontier,built,journal,directory,open,edit};"""),
("""/** ⚖️ The planner's verdict on the tree: every raw fact holds by the engine, the membership advance keeps the frontier, and the derived fact's input is the one it read. */
async function planner(root:string,raw:readonly MemoRowV1[],frontier:Awaited<ReturnType<typeof walk>>,d:string):Promise<boolean>{
 let same=false;try{same=sha(readFileSync(join(root,"lib/d.ts")))===d;}catch{}
 return""", """/** ⚖️ The planner's verdict on the tree: every fact the project reads holds by the engine, the membership advance keeps the frontier, and the derived question answers what it recorded (the planner's cutoff: a stale support whose question answers the same keeps the project). */
async function planner(root:string,raw:readonly MemoRowV1[],frontier:Awaited<ReturnType<typeof walk>>,d:number|undefined):Promise<boolean>{
 const same=d!==undefined&&modelAnswer(root)===d;
 return"""),
("""memo=await sealed(root),d=sha(readFileSync(join(root,"lib/d.ts")));""", """memo=await sealed(root),d=modelAnswer(root);"""),
(""" expect(edits.named("lib/d.ts").length,"the derived fact names its input").toBe(1);""", """ expect(edits.named("lib/d.ts").length,"the derived fact and the TypeScript text its derivation read name its input").toBe(2);"""),
(""" let served=0,refused=0,trustingWrong=0,blindWrong=0;""", """ let served=0,refused=0,trustingWrong=0,blindWrong=0,acceptingWrong=0,baseDerived=0;"""),
("""base=await (await memo.open({edit:undefined})).lookup(),answer=await (await memo.open()).lookup();""",
"""base=await (await memo.open({edit:undefined})).lookup(),{derive:_derive,...without}=memo.edit,withoutDerive=mutation.name.includes("derived")&&mutation.serve?await (await memo.open({edit:without})).lookup():undefined,answer=await (await memo.open()).lookup();"""),
("""  if((await settle({ports:blind})).held&&!verdict)blindWrong++;
 }""", """  if((await settle({ports:blind})).held&&!verdict)blindWrong++;
  if((await settle({ports:{...memo.edit,derive:async()=>({held:true,questions:1})}})).held&&!verdict)acceptingWrong++;
  if(mutation.name.includes("derived")&&mutation.serve){expect(withoutDerive?.hit,"the landed memo (no derivation) plans: "+mutation.name).toBe(false);baseDerived++;if(answer.hit)expect(answer.edited?.questions,mutation.name).toBe(1);}
 }"""),
(""" expect(served).toBeGreaterThanOrEqual(7);expect(refused).toBeGreaterThanOrEqual(15);""", """ expect(served).toBeGreaterThanOrEqual(10);expect(refused).toBeGreaterThanOrEqual(19);expect(baseDerived).toBe(3);
 expect(acceptingWrong,"a settlement that accepts every derivation must be wrong somewhere").toBeGreaterThan(0);"""),
("""a trusting settlement or one that reads the frontier of the present tree is caught""", """a trusting settlement, one that reads the frontier of the present tree and one that accepts every derivation are caught"""),
("""   const state:{rows:readonly MemoRowV1[];frontier?:Awaited<ReturnType<typeof walk>>}={rows:[]},stem""", """   const state:{rows:readonly MemoRowV1[];index?:FactIndexV1;frontier?:Awaited<ReturnType<typeof walk>>}={rows:[]},stem"""),
("""frontier:async()=>state.frontier&&{frontier:state.frontier,descend:(name:string)=>descend(name),member}};""", """frontier:async()=>state.frontier&&{frontier:state.frontier,descend:(name:string)=>descend(name),member},derive:async({stale,derived})=>settleGraphMemoDerivationV1({index:state.index!,stale,derived,ask:suspects=>deriveGraphMemoSuspectsV1({index:state.index!,suspects,derive:deriveModel})})};"""),
("""   const sealing=await open(),{rows,raw}=await harvest(root),frontier=await walk(root),d=sha(readFileSync(join(root,"lib/d.ts")));state.rows=rows;""", """   const sealing=await open(),{rows,raw,index}=await harvest(root),frontier=await walk(root),d=modelAnswer(root);state.rows=rows;state.index=index;"""),
('''"touch a walked directory","change the mode of the analysed file","rename a walked directory"]),must=new Set([''', '''"touch a walked directory","change the mode of the analysed file","rename a walked directory","content-only edit of a file a derived fact reads (the answer is kept)","edit that changes a derived answer (an import added to its input)"]),must=new Set(["content-only edit of a file a derived fact reads (the answer is kept)",'''),
("""/** ✏️️ Contract: a sealed memo whose journal names a relevant change""", """/** ✏️️ Contract (derived facts included: a stale support or a named derived fact is settled by asking the derived question again, served only when the answer is the recorded one and the new support reads nothing new): a sealed memo whose journal names a relevant change"""),
]
for old, new in reps:
    if t.count(old) != 1: sys.exit("[DEBUG] anchor %d: %s" % (t.count(old), old[:90]))
    t = t.replace(old, new)
t += '''
test("the derivation child: a stale row a project holds itself, an unsupported derived fact, no or too many questions refuse at once; a request asked through the child entry against a real generation store answers what the in-process derivation answers; a retired index answers missing; the runtime and the graph loader wire the child",async()=>{
 const {root}=fixture(),{index}=await harvest(root),project=(kind:string,path:string)=>index.rows.findIndex(row=>row.kind===kind&&row.path===join(root,path)),support=project("typescriptText","lib/d.ts"),own=project("file","src/sub/c.ts"),derived=index.rows.find(row=>row.kind==="derived") as MemoRowV1;
 expect(support).toBeGreaterThanOrEqual(0);expect(own).toBeGreaterThanOrEqual(0);
 expect(graphMemoSuspectsV1(index,[support],[]).suspects).toEqual(new Set([derivedOwnerNameV1(derived)]));
 const never=async()=>{throw Error("[DEBUG] asked");};
 expect(await settleGraphMemoDerivationV1({index,stale:[support,own],derived:[],ask:never})).toEqual({held:false,reason:"direct:proj"});
 expect(await settleGraphMemoDerivationV1({index,stale:[],derived:[{...derived,fact:7}],ask:never})).toEqual({held:false,reason:"unsupported"});
 expect(await settleGraphMemoDerivationV1({index,stale:[],derived:[],ask:never})).toEqual({held:false,reason:"questions"});
 const suspects=new Set([derivedOwnerNameV1(derived)]),inProcess=await deriveGraphMemoSuspectsV1({index,suspects,derive:deriveModel});
 expect(inProcess).toEqual({version:1,held:[...suspects],changed:[],failed:0,uncovered:[],missing:false});
 expect(graphMemoDerivationHeldV1(suspects,inProcess)).toEqual({held:true,questions:1});
 for(const [answer,reason] of [[undefined,"missing"],[{...inProcess,missing:true},"missing"],[{...inProcess,changed:["x"]},"changed"],[{...inProcess,failed:1},"changed"],[{...inProcess,uncovered:["x"]},"uncovered"],[{...inProcess,held:[]},"unanswered"]] as const)expect(graphMemoDerivationHeldV1(suspects,answer as never),reason).toEqual({held:false,reason});
 const generations=join(sandbox,"gen"+counter++),{bytes}=packFactIndexV1(index as never,GENERATION_POLICY_V1.maximumIndexBytes),digest=sha(Buffer.from(bytes));
 mkdirSync(join(generations,GENERATION_POLICY_V1.indexes),{recursive:true});writeFileSync(join(generations,GENERATION_POLICY_V1.indexes,digest+".facts.br"),bytes);
 const request=join(sandbox,"req"+counter++)+".json",environment={[GRAPH_MEMO_POLICY_V1.derive]:request};
 writeGraphMemoDeriveFileV1(request,{version:1,index:digest,suspects:[...suspects]});
 expect(readGraphMemoDeriveFileV1(request,"request")).toEqual({version:1,index:digest,suspects:[...suspects]});
 await answerGraphMemoDerivationV1(root,environment,undefined,async()=>deriveModel,generations);
 expect(readGraphMemoDeriveFileV1(graphMemoDeriveAnswerFileV1(request),"answer"),"the child answers what the in-process derivation answers").toEqual(inProcess);
 appendFileSync(join(root,"lib/d.ts"),"import './y.ts';\\n");await answerGraphMemoDerivationV1(root,environment,undefined,async()=>deriveModel,generations);
 expect(readGraphMemoDeriveFileV1(graphMemoDeriveAnswerFileV1(request),"answer")).toMatchObject({held:[],changed:[...suspects]});
 writeGraphMemoDeriveFileV1(request,{version:1,index:"9".repeat(64),suspects:[...suspects]});await answerGraphMemoDerivationV1(root,environment,undefined,async()=>deriveModel,generations);
 expect(readGraphMemoDeriveFileV1(graphMemoDeriveAnswerFileV1(request),"answer")).toMatchObject({missing:true});
 writeFileSync(request,JSON.stringify({version:1,index:digest,suspects:[...suspects],extra:1}));await expect(answerGraphMemoDerivationV1(root,environment,undefined,async()=>deriveModel,generations)).rejects.toThrow();
 const here=import.meta.dir,runtime=readFileSync(join(here,"../../../../../⌨️command/⚙️runtime/🟦️.ts"),"utf8"),loader=readFileSync(join(here,"../../../🟦️.ts"),"utf8");
 expect(runtime.includes(`process.env.${GRAPH_MEMO_POLICY_V1.derive}!==undefined`)&&runtime.indexOf(`process.env.${GRAPH_MEMO_POLICY_V1.derive}`)<runtime.indexOf("prepareBuildToolStorageV1"),"the runtime answers a derivation before any tool or graph work").toBe(true);
 expect(loader.includes(`child("build:graph-derive",environment)`),"the graph loader hands the memo its derivation child").toBe(true);
},120000);
'''
open(path, "w", encoding="utf8").write(t)
print("[DEBUG] law edited")
