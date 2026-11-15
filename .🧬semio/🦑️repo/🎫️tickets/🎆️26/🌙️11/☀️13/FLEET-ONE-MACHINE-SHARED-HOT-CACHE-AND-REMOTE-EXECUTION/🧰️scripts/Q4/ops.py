import json,os,sys
HERE=os.path.dirname(os.path.abspath(__file__))
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
CL=LIB+"/🏗️build/📥️source/🔏️closure"
CHURN="🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn"
FT=LIB+"/🏗️build/🔧️tools/🧶️nx/🪵️failure-tail"
GUIDE="🧰️framework/🛍️products/🦑️repo/📓️build-guide.md"
MISMATCH=CL+"/🧪️tests/🧾️mismatch/🟦️.ts"
EDITS=[GUIDE,MISMATCH,FT+"/🧪️tests/🟦️.ts",CL+"/🟦️.ts",LIB+"/🏗️build/📖️texts/🔣️.json",CHURN+"/🧬️schema/🧾️refusals.json",CHURN+"/🧪️tests/🧾️refusals/🔣️.json",FT+"/🟦️.ts",FT+"/🧬️schema/🔣️.json",LIB+"/⚡️caching/🚀️bootstrap/📜️script.ts"]
NEW_DIRS=[CL+"/🛂️readmission",FT+"/🧪️tests/🧾️summary"]
def snip(name):
    return open(os.path.join(HERE,"snippets",name),encoding="utf8").read()
def once(text,old,new,what):
    if text.count(old)!=1:raise SystemExit("anchor not unique (%d) for %s"%(text.count(old),what))
    return text.replace(old,new,1)
def jb(obj,indent):
    return json.dumps(obj,ensure_ascii=False,indent=2).replace("\n","\n"+" "*indent)
CATALOG=[
 ("closure.readmitted","The sources of the build implementation moved while this task was running ({count} files, first: {files}; last writer: {origin}). The task admits the current sources for itself and goes on.","Die Quellen der Build-Implementierung haben sich geändert, während diese Aufgabe lief ({count} Dateien, zuerst: {files}; letzter Schreiber: {origin}). Die Aufgabe übernimmt die aktuellen Quellen für sich und läuft weiter."),
 ("closure.readmit-retry","The sources of the build implementation are still changing ({count} files, first: {files}; last writer: {origin}): admission {attempt} of {maximum} did not hold still, trying again in {seconds} s.","Die Quellen der Build-Implementierung ändern sich noch ({count} Dateien, zuerst: {files}; letzter Schreiber: {origin}): Übernahme {attempt} von {maximum} blieb nicht stabil, neuer Versuch in {seconds} s."),
 ("closure.moving","The sources of the build implementation kept changing during {attempts} admissions ({count} files, first: {files}; last writer: {origin}). Nothing was run with a mixed version; try again later.","Die Quellen der Build-Implementierung änderten sich während {attempts} Übernahmen fortlaufend ({count} Dateien, zuerst: {files}; letzter Schreiber: {origin}). Es lief nichts mit einer gemischten Version; bitte später erneut versuchen."),
]
SITE_CLOSURE=CL+"/🟦️.ts"
SITE_READMIT=CL+"/🛂️readmission/🟦️.ts"
CLASSES=[
 {"id":"moved-source-closure","disposition":"reexec","markers":[],"messages":[{"text":"closure.source-differs","sites":[SITE_CLOSURE]},{"text":"closure.source-not-admitted","sites":[SITE_CLOSURE]}],"describe":{"en":"A task found that a file of the build implementation differs from the list its process admitted at its start and could not admit the current sources for itself: the outermost command starts again with the current implementation, bounded.","de":"Eine Aufgabe fand, dass sich eine Datei der Build-Implementierung von der Liste unterscheidet, die ihr Prozess beim Start zugelassen hat, und konnte die aktuellen Quellen nicht für sich zulassen: der äußerste Befehl startet mit der aktuellen Implementierung neu, begrenzt."}},
 {"id":"closure-moving","disposition":"reobserve","layer":"build","markers":[],"messages":[{"text":"closure.moving","sites":[SITE_READMIT]}],"describe":{"en":"The sources of the build implementation kept changing during every admission of one task: the lane that runs the actions runs the step again after a pause, bounded; only a closure that never holds still fails.","de":"Die Quellen der Build-Vorgabe änderten sich bei jeder Übernahme einer Aufgabe weiter: die Bahn, die die Aktionen ausführt, führt den Schritt nach einer Pause erneut aus, begrenzt; nur ein Abschluss, der nie stillhält, schlägt fehl."}},
]
VECTORS=[
 {"id":"closure-moved-nx","text":"[script] build:implementation-source · Reading the sources of the build implementation\nRepoError: stale build.closure.source-differs {\"path\":\"a/b.ts\",\"expectedSha256\":\"ff\",\"observedSha256\":\"88\"}","class":"moved-source-closure","disposition":"reexec"},
 {"id":"closure-not-admitted-nx","text":"RepoError: refused build.closure.source-not-admitted {\"path\":\"a/new.ts\"}","class":"moved-source-closure","disposition":"reexec"},
 {"id":"closure-moving-nx","text":"RepoError: unserved build.closure.moving {\"count\":2,\"files\":\"a.ts, b.ts\",\"origin\":\"-\",\"attempts\":5}","class":"closure-moving","disposition":"reobserve"},
]
GUIDE_EN="- **Build-implementation edits while a task runs**: a long run (the `workspace:setup` orchestration takes 20 to 60 minutes) admits the sources of the build implementation once, and a task that captures a child image compares the files with that list. When a peer edited a file of the closure meanwhile, the task no longer fails with `stale build.closure.source-differs`: it admits the current closure for itself (a new consistent capture, a new snapshot digest, handed to a fresh child) and says so in English and German. Only a closure that keeps changing under one admission is retried (5 retries after the first attempt, pauses of 0.25, 0.5, 1, 2 and 4 s) and then ends with the refusal `closure.moving`, class `unserved` (exit status 75, try again later), never with an internal error. Edits outside the closure change nothing. When a run ends in failure the parent lists per failed task why it failed and whether another rerun is likely to help; a task output that still names `closure.source-differs` restarts the outermost command with the current implementation, one that names `closure.moving` runs the Nx invocation again (finished tasks are cache hits, 4 attempts of the `build` layer). The pinned alternative (a task reads its own implementation from an immutable copy) exists today for descendants of a captured process (`runAdoptedCapturedBuildSourceV1`) and for the sealed store (`SEMIO_REPO_SEAL`, opt-in); the Nx-resident hooks hold only the hashes of their admission, not the bytes.\n"
GUIDE_DE="- **Änderungen der Build-Implementierung während eine Aufgabe läuft**: ein langer Lauf (die Orchestrierung `workspace:setup` dauert 20 bis 60 Minuten) lässt die Quellen der Build-Implementierung einmal zu, und eine Aufgabe, die ein Kind-Abbild erfasst, vergleicht die Dateien mit dieser Liste. Hat ein anderer Prozess inzwischen eine Datei des Abschlusses bearbeitet, scheitert die Aufgabe nicht mehr mit `stale build.closure.source-differs`: sie lässt den aktuellen Abschluss für sich zu (neue konsistente Erfassung, neuer Schnappschuss-Digest, übergeben an ein frisches Kind) und sagt das auf Englisch und Deutsch. Nur ein Abschluss, der sich während einer Zulassung weiter ändert, wird wiederholt (5 Wiederholungen nach dem ersten Versuch, Pausen von 0,25, 0,5, 1, 2 und 4 s) und endet dann mit der Ablehnung `closure.moving`, Klasse `unserved` (Exit-Status 75, später erneut versuchen), nie mit einem internen Fehler. Änderungen außerhalb des Abschlusses ändern nichts. Endet ein Lauf mit einem Fehler, listet der Elternprozess je fehlgeschlagener Aufgabe, warum sie scheiterte und ob eine weitere Wiederholung wahrscheinlich hilft; eine Aufgabenausgabe, die weiter `closure.source-differs` nennt, startet den äußersten Befehl mit der aktuellen Implementierung neu, eine, die `closure.moving` nennt, führt den Nx-Aufruf erneut aus (fertige Aufgaben sind Cache-Treffer, 4 Versuche der Schicht `build`). Die angeheftete Alternative (eine Aufgabe liest ihre Implementierung aus einer unveränderlichen Kopie) gibt es heute für Nachkommen eines erfassten Prozesses (`runAdoptedCapturedBuildSourceV1`) und für den versiegelten Speicher (`SEMIO_REPO_SEAL`, Opt-in); die Nx-residenten Haken halten von ihrer Zulassung nur die Hashes, nicht die Bytes.\n"
def after_line(text,start,add,what):
    i=text.count(start)
    if i!=1:raise SystemExit("anchor not unique (%d) for %s"%(i,what))
    at=text.index(start);end=text.index("\n",at)+1
    return text[:end]+add+text[end:]
def apply(rel,text):
    if rel==GUIDE:
        text=after_line(text,"- **Projection edits while a command plans**",GUIDE_EN,"guide en")
        text=after_line(text,"- **Änderungen der Projektion während ein Befehl plant**",GUIDE_DE,"guide de")
    elif rel==MISMATCH:
        text=once(text,'import{BUILD_TEXTS_V1}from"../../../../📖️texts/🟦️.ts";','import{BUILD_TEXTS_V1,refuseBuildV1}from"../../../../📖️texts/🟦️.ts";',"mismatch import")
        text=once(text,'test("real admitted compiler Source mismatch retains exact path and both hashes in stale bilingual error"','test("real admitted compiler Source mismatch is admitted again by the task and the stale refusal still retains exact path and both hashes in both languages"',"mismatch title")
        text=once(text,'const result=await run();expect(result.status,result.stderr).toBe(1);expect(result.signal).toBeNull();const observed=JSON.parse(result.stdout.trim().split("\\n").at(-1)!);','const result=await run();expect(result.status,result.stderr).toBe(0);expect(result.signal).toBeNull();expect(JSON.parse(result.stdout.trim().split("\\n").at(-1)!)).toEqual({accepted:true});expect(result.stdout+result.stderr).toContain(selected);const stale=refuseBuildV1("stale","closure.source-differs",{path:selected,expectedSha256:row.sha256,observedSha256:new Bun.CryptoHasher("sha256").update(corpus.after).digest("hex")}),observed={code:stale.code,key:stale.key,values:stale.values};',"mismatch body")
    elif rel==CL+"/🟦️.ts":
        text=once(text,'import{validateJsonSchemaSubset}from"../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";','import{validateJsonSchemaSubset}from"../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";\nimport{withMovedClosureReadmittedV1}from"./🛂️readmission/🟦️.ts";',"closure import")
        text=once(text,'async function sourceBytes(root:string,path:string,original:ScriptInvocation):Promise<Buffer>{\n const admitted=admittedSourcesV1 as {root:string;','async function sourceBytes(root:string,path:string,original:ScriptInvocation,selection:unknown=admittedSourcesV1):Promise<Buffer>{\n const admitted=selection as {root:string;',"closure sourceBytes")
        text=once(text,'const admittedSourcesV1:unknown=import.meta.semioCapturedPhysicalSources;','const admittedSourcesV1:unknown=import.meta.semioCapturedPhysicalSources,readmittedSnapshotsV1=new WeakSet<BuildSourceClosureV1>();',"closure admitted")
        text=once(text,'requireIssuedScriptInvocationV1(original);requireSourceSelectionDataV1(snapshot,admittedSourcesV1);requireIssuedScriptInvocationV1(original);}','requireIssuedScriptInvocationV1(original);if(!readmittedSnapshotsV1.has(snapshot))requireSourceSelectionDataV1(snapshot,admittedSourcesV1);requireIssuedScriptInvocationV1(original);}',"closure require")
        text=once(text,'export async function captureBuildSourceClosureV1(workspace:string,entry:string,original:ScriptInvocation,scope:BuildSourceScopeV1={kind:"closure"}):Promise<BuildSourceClosureV1>{','export function captureBuildSourceClosureV1(workspace:string,entry:string,original:ScriptInvocation,scope:BuildSourceScopeV1={kind:"closure"}):Promise<BuildSourceClosureV1>{return withMovedClosureReadmittedV1({workspace,entry,scope,signal:original.control.signal,baked:{sources:admittedSourcesV1,resolutions:admittedResolutionsV1},run:(selectedSources,selectedResolutions,fresh)=>captureAdmittedBuildSourceClosureV1(workspace,entry,original,scope,selectedSources,selectedResolutions,fresh)});}\n/** 🔒️ Captures through one admitted selection: the one the parent process admitted at its start, or, after a peer moved the closure, the one this task took from the current disk for itself. */\nasync function captureAdmittedBuildSourceClosureV1(workspace:string,entry:string,original:ScriptInvocation,scope:BuildSourceScopeV1,selectedSources:unknown,selectedResolutions:unknown,fresh:boolean):Promise<BuildSourceClosureV1>{',"closure capture")
        text=once(text,'sourceResolutionMapV1(root,admittedResolutionsV1,admittedSourcesV1)','sourceResolutionMapV1(root,selectedResolutions,selectedSources)',"closure map")
        if text.count("await sourceBytes(root,path,original)")!=2:raise SystemExit("sourceBytes calls")
        text=text.replace("await sourceBytes(root,path,original)","await sourceBytes(root,path,original,selectedSources)")
        text=once(text,'requireAdmittedBuildSourceClosureV1(value,original);checkScriptInvocation(original);return value;','if(fresh){requireIssuedScriptInvocationV1(original);requireSourceSelectionDataV1(value,selectedSources);requireIssuedScriptInvocationV1(original);readmittedSnapshotsV1.add(value);}else requireAdmittedBuildSourceClosureV1(value,original);checkScriptInvocation(original);return value;',"closure end")
    elif rel==LIB+"/🏗️build/📖️texts/🔣️.json":
        entries="".join('    %s: {\n      "en": %s,\n      "de": %s\n    },\n'%(json.dumps(k),json.dumps(e,ensure_ascii=False),json.dumps(d,ensure_ascii=False)) for k,e,d in CATALOG)
        text=once(text,'    "closure.capacity-exceeded": {\n',entries+'    "closure.capacity-exceeded": {\n',"catalog")
    elif rel==CHURN+"/🧬️schema/🧾️refusals.json":
        block="".join("          "+jb(c,10)+",\n" for c in CLASSES)
        text=once(text,'          {\n            "id": "filesystem-race",\n',block+'          {\n            "id": "filesystem-race",\n',"classes")
    elif rel==CHURN+"/🧪️tests/🧾️refusals/🔣️.json":
        d=json.loads(text)
        ids={r["id"] for r in d["texts"]}
        if any(v["id"] in ids for v in VECTORS):raise SystemExit("vector ids exist")
        raw=json.dumps(d,ensure_ascii=False)
        if text.startswith('{"version": 1, "texts": ['):
            ins="".join(json.dumps(v,ensure_ascii=False)+", " for v in VECTORS)
            text=once(text,'{"version": 1, "texts": [','{"version": 1, "texts": ['+ins,"vectors")
        else:
            first=text.index('"texts": [')+len('"texts": [')
            ins="\n    "+",\n    ".join(json.dumps(v,ensure_ascii=False) for v in VECTORS)+","
            text=text[:first]+ins+text[first:]
    elif rel==FT+"/🧪️tests/🟦️.ts":
        text=once(text,'source.indexOf("code = await settleNxRunV1(")','source.indexOf("code = await settleNxRunReportedV1(")',"ft test")
    elif rel==FT+"/🟦️.ts":
        text=once(text,'import {retryOnConcurrentEditV1,type ChurnOptionsV1} from','import {churnLanguagesV1,retryOnConcurrentEditV1,type ChurnOptionsV1} from',"ft import 1")
        text=once(text,'import {declaredRefusalClassOfTextV1,failureChurnOfTailsV1} from','import {declaredRefusalClassOfTextV1,failureChurnOfTailsV1,refusalClassOfTextV1} from',"ft import 2")
        text=text.rstrip("\n")+"\n"+snip("failure-tail.append.ts")
    elif rel==FT+"/🧬️schema/🔣️.json":
        summary=open(os.path.join(HERE,"snippets","summary.schema.json"),encoding="utf8").read().replace("\n","\n    ")
        old='        "maximumPendingLineCharacters": 16384\n      }\n    }\n'
        text=once(text,old,old[:-1]+',\n    "Summary": '+summary+'\n',"ft schema")
    elif rel==LIB+"/⚡️caching/🚀️bootstrap/📜️script.ts":
        text=once(text,'import {createNxFailureTailV1,settleNxRunV1,type NxFailureTailV1} from','import {createNxFailureTailV1,settleNxRunReportedV1,type NxFailureTailV1} from',"script import")
        text=once(text,'code = await settleNxRunV1({ code, tail:','code = await settleNxRunReportedV1({ code, tail:',"script call")
    else:
        raise SystemExit("no op for "+rel)
    return text
def build(root,out):
    res={}
    for rel in EDITS:
        t=open(os.path.join(root,rel),encoding="utf8").read()
        res[rel]=apply(rel,t)
    for rel,t in res.items():
        p=os.path.join(out,rel);os.makedirs(os.path.dirname(p),exist_ok=True);open(p,"w",encoding="utf8").write(t)
    return res
if __name__=="__main__":
    root=sys.argv[1];out=sys.argv[2]
    r=build(root,out);print("built",len(r))
