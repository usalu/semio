"""🧵️ [obsopen] anchored edits of batch `nestedfix`: an outer Graph fixture reopens its receiver epoch inside the async context it was born in (product), the nested-fixture laws are bounded and declare their envelope, the presence law passes the consumer's own control to its nested fixture, keeps the primary first beneath aggregated failures and runs one law per child row. usage: edit_nested.py <root>"""
import os, sys

ROOT = sys.argv[1]
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
OBS = L + "🏗️build/👁️observation/🧪️tests/"
STORE = L + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts"


def edit(rel, pairs):
    path = os.path.join(ROOT, rel)
    text = open(path, encoding="utf8").read()
    for old, new in pairs:
        if text.count(old) != 1:
            raise SystemExit(f"refused: {text.count(old)} anchors in {rel[-60:]}: {old[:80]!r}")
        text = text.replace(old, new)
    open(path, "w", encoding="utf8").write(text)


edit(STORE, [
    ("const openEpoch=async()=>{if(++epochs>receiverPolicy.maximumEpochs)",
     "const home=AsyncLocalStorage.snapshot(),openEpoch=()=>home(async()=>{if(++epochs>receiverPolicy.maximumEpochs)"),
    ("await announced;};\n", "await announced;});\n"),
])

DECLARE = 'import{declareLibraryBuildTestEnvelopeV1}from"../../../../🧪️testing/🪪️native-birth/🟦️.ts";\ndeclareLibraryBuildTestEnvelopeV1({suite:"build-observation",maximumElapsedMilliseconds:900000});\n'
WITHIN = '''
/** ⏱️ Settles one nested fixture operation inside a finite bound and names the stuck operation instead of waiting for the law deadline. */
function withinV1<T>(work:Promise<T>,milliseconds:number,what:string):Promise<T>{let timer:ReturnType<typeof setTimeout>|undefined;return Promise.race([work,new Promise<never>((_,reject)=>{timer=setTimeout(()=>reject(Error(what+" exceeded "+milliseconds+" ms")),milliseconds);})]).finally(()=>clearTimeout(timer));}
'''
NESTED_LAW = '''
registerBuildTestLawV1("observation block nested fixtures settle an outer maintenance that reopens beneath an inner fixture","sdk-runtime",async original=>{
 const control=createIssuedResourceControlV1(original,"observation:nested-maintenance",120000);let entered=false,reopened=false,innerRoot="",outerRoot="";
 await withRepositoryBuildGraphPublicationFixtureV1(workspace,original,control,async outer=>{
  outerRoot=outer.root;const before=outer.image;
  await withinV1(withRepositoryBuildGraphPublicationFixtureV1(workspace,original,control,async inner=>{innerRoot=bounded(inner.root);expect(inner.root).not.toBe(outer.root);await outer.withMaintenance(async()=>{entered=true;});reopened=outer.image!==before;}),60000,"Nested Graph fixture settlement");
  expect(existsSync(innerRoot)).toBe(false);await outer.withConsumer(async store=>{expect(store.image).toBe(outer.image);});
 });
 expect(entered).toBe(true);expect(reopened).toBe(true);expect(existsSync(outerRoot)).toBe(false);
});
'''
BLOCKS = OBS + "🧱️blocks/🟦️.ts"
edit(BLOCKS, [
    ('import corpus from"./🧫️cases.json";\n', 'import corpus from"./🧫️cases.json";\n' + DECLARE),
    ('const workspace=resolve(process.cwd()),hash=', 'const interruptedStatusV1=130,workspace=resolve(process.cwd()),hash='),
    ('expect(result.status,result.stderr).toBe(1);expect(value.calls).toBe(1);', 'expect(result.status,result.stderr).toBe(["cancel","cancel-close"].includes(row.fault)?interruptedStatusV1:1);expect(value.calls).toBe(1);'),
])
text = open(os.path.join(ROOT, BLOCKS), encoding="utf8").read()
anchor = 'expect(path.length).toBeLessThanOrEqual(256);return path;};\n'
if text.count(anchor) != 1:
    raise SystemExit("refused: bounded anchor")
text = text.replace(anchor, anchor + WITHIN, 1)
old_nested = 'await withRepositoryBuildGraphPublicationFixtureV1(workspace,original,control,async()=>fixture.withMaintenance(async(image,custody)=>{expect(()=>requireRepositoryBuildGraphMaintenanceImageV1(image,custody)).toThrow();}));'
new_nested = 'await withinV1(withRepositoryBuildGraphPublicationFixtureV1(workspace,original,control,async()=>fixture.withMaintenance(async(image,custody)=>{expect(()=>requireRepositoryBuildGraphMaintenanceImageV1(image,custody)).toThrow();})),60000,"Nested Graph fixture settlement");'
if text.count(old_nested) != 1:
    raise SystemExit("refused: nested custody anchor")
text = text.replace(old_nested, new_nested, 1)
tail = '\nregisterBuildTestLawV1("observation block path custody classifies actual issued images and ordinary fixtures"'
if text.count(tail) != 1:
    raise SystemExit("refused: custody law anchor")
text = text.replace(tail, NESTED_LAW + tail, 1)
first = text.index('registerBuildTestLawV1("observation block publisher rebuilds retired shared bytes')
last = text.index('\nfor(const row of corpus.faults)')
if not first < last:
    raise SystemExit("refused: publisher law anchors")
text = text[:first] + text[last + 1:].rstrip("\n") + "\n\n" + text[first:last].rstrip("\n") + "\n"
open(os.path.join(ROOT, BLOCKS), "w", encoding="utf8").write(text)

PRESENCE = OBS + "🪪️presence/🟦️.ts"
LOOPS = ('for(const kind of corpus.primaryFailures){const{result,value}=await childOutcome(original,phase,"primary","source",kind);expect(result.status,result.stderr).toBe(1);expect(value.bodyStarted).toBe(true);expect(value.childFinished).toBe(true);expect(value.samePrimary).toBe(true);}'
         'for(const row of receiverCorpus.receiverTransitions.filter(row=>!row.admitted)){const{result,value}=await childOutcome(original,phase,"maintenance","source",row.outcome);expect(result.status,result.stderr).toBe(1);expect(value.bodyStarted).toBe(true);expect(value.maintenanceStarted).toBe(true);expect(value.admissionRefused).toBe(true);expect(value.childFinished).toBe(false);expect(value.samePrimary).toBe(true);}\n});\n')
ROWS = ('});\n\n'
        'for(const kind of corpus.primaryFailures)registerBuildTestLawV1("fixed graph fixtures keep a "+kind+" primary first beneath accepted child failures","sdk-runtime",async original=>{\n'
        ' const phase=createIssuedResourceControlV1(original,"graph:fixture-primary",180000),{result,value}=await childOutcome(original,phase,"primary","source",kind);expect(result.status,result.stderr).toBe(1);expect(value.bodyStarted).toBe(true);expect(value.childFinished).toBe(true);expect(value.samePrimary).toBe(true);\n});\n\n'
        'for(const row of receiverCorpus.receiverTransitions.filter(row=>!row.admitted))registerBuildTestLawV1("fixed graph fixtures refuse admission after a "+row.outcome+" maintenance receiver","sdk-runtime",async original=>{\n'
        ' const phase=createIssuedResourceControlV1(original,"graph:fixture-maintenance",180000),{result,value}=await childOutcome(original,phase,"maintenance","source",row.outcome);expect(result.status,result.stderr).toBe(row.outcome==="cancel"?interruptedStatusV1:1);expect(value.bodyStarted).toBe(true);expect(value.maintenanceStarted).toBe(true);expect(value.admissionRefused).toBe(true);expect(value.childFinished).toBe(false);expect(value.samePrimary).toBe(true);\n});\n')
edit(PRESENCE, [
    ('const hash=(text:string)=>createHash("sha256").update(text).digest("hex"),workspace=resolve(process.cwd());', 'const hash=(text:string)=>createHash("sha256").update(text).digest("hex"),workspace=resolve(process.cwd()),interruptedStatusV1=130;'),
    ('"terminal","lock",kind);expect(result.status,result.stderr).toBe(1);', '"terminal","lock",kind);expect(result.status,result.stderr).toBe(kind==="cancel"?interruptedStatusV1:1);'),
    ('import fixtureSchema from"../../../📥️inputs/💾️storage/🧪️fixture/🧬️schema/🔣️.json";\n', 'import fixtureSchema from"../../../📥️inputs/💾️storage/🧪️fixture/🧬️schema/🔣️.json";\n' + DECLARE),
    ('await outer.withConsumer(async()=>{await withRepositoryBuildGraphFixtureV1(workspace,original,phase,async inner=>{',
     'await outer.withConsumer(async(_store,actual)=>{await withRepositoryBuildGraphFixtureV1(workspace,original,actual,async inner=>{'),
    (LOOPS, ROWS),
])
edit(OBS + "🪪️presence/👷️child/🟦️.ts", [
    ("samePrimary:Object.is(terminal,primary)", "samePrimary:Object.is(flattenFailuresV1(terminal)[0],primary)"),
    ("/** 👷️ Receives one admitted static child",
     "/** 🧵️ Lists every failure of a nested aggregate depth-first in raised order, so the primary is the head. */\nfunction flattenFailuresV1(error:unknown):unknown[]{return error instanceof AggregateError?error.errors.flatMap(flattenFailuresV1):[error];}\n\n/** 👷️ Receives one admitted static child"),
])


def report_child(rel, receive_prefix):
    path = os.path.join(ROOT, rel)
    text = open(path, encoding="utf8").read()
    import re
    printed = re.search(r"console\.log\(JSON\.stringify\((\{.*?\})\)\);process\.exitCode=failed\?1:0;", text, re.S)
    if not printed or text.count(receive_prefix) != 1 or text.count("});}catch(error){failed=true;terminal=error;}finally{restoreFsV1();}") != 1:
        raise SystemExit("refused: child anchors in " + rel[-50:])
    text = text.replace(printed.group(0), "report();process.exitCode=failed?1:0;")
    text = text.replace(receive_prefix, "let reported=false;const report=():void=>{if(reported)return;reported=true;writeSync(1,JSON.stringify(" + printed.group(1) + ")+\"\\n\");};process.once(\"exit\",report);\n" + receive_prefix.replace("async original=>{", "async original=>{try{"))
    text = text.replace("});}catch(error){failed=true;terminal=error;}finally{restoreFsV1();}", "}catch(error){failed=true;terminal=error;throw error;}});}catch(error){failed=true;terminal=error;}finally{restoreFsV1();}")
    text = 'import{writeSync}from"node:fs";\n' + text
    open(path, "w", encoding="utf8").write(text)


report_child(OBS + "🧱️blocks/👷️child/🟦️.ts", " try{await receiveScriptProcessInvocation(process.env,async original=>{")
report_child(OBS + "🪪️presence/👷️child/🟦️.ts", " try{await receiveScriptProcessInvocation(process.env,async original=>{")

edit(OBS + "📥️reuse/🟦️.ts", [
    ('import{createGraphPhaseClockV1}from"../../../🕸️graph/⏱️phases/🟨️.mjs";\n', 'import{createGraphPhaseClockV1}from"../../../🕸️graph/⏱️phases/🟨️.mjs";\n' + DECLARE.replace(',maximumElapsedMilliseconds:900000', '')),
    ("let failed=false,same=false,calls=0,seen=false,message='';const root=",
     "let failed=false,same=false,calls=0,seen=false,message='',reported=false;const{writeSync}=await import('node:fs'),report=()=>{if(reported)return;reported=true;writeSync(1,JSON.stringify({failed,same,calls,seen,message,publications:observer.sourceObservationStatsV1().originalInferencePublications})+'\\\\n');};process.once('exit',report);const root="),
    ("try{await receiveScriptProcessInvocation(process.env,async original=>{const phase=createIssuedResourceControlV1(original,'inference:failed',15000),",
     "try{await receiveScriptProcessInvocation(process.env,async original=>{try{const phase=createIssuedResourceControlV1(original,'inference:failed',15000),"),
    ("return ordinaryInventoryReferencesHoldV1(root,candidate,phase);}});});}catch(error){failed=true;same=Object.is(error,primary);message=error instanceof Error?error.message:String(error);}console.log(JSON.stringify({failed,same,calls,seen,message,publications:observer.sourceObservationStatsV1().originalInferencePublications}));process.exitCode=failed?1:0;",
     "return ordinaryInventoryReferencesHoldV1(root,candidate,phase);}});}catch(error){failed=true;same=Object.is(error,primary);message=error instanceof Error?error.message:String(error);throw error;}});}catch(error){failed=true;same=Object.is(error,primary);message=error instanceof Error?error.message:String(error);}report();process.exitCode=failed?1:0;"),
    ('expect(result.status,kind+":"+cache+":"+result.stderr).toBe(1);', 'expect(result.status,kind+":"+cache+":"+result.stderr).toBe(kind==="cancel"?130:1);'),
])
