import os, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
LIB = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
DEP = LIB + "/⚡️caching/🚀️bootstrap/📦️dependencies"
GEN = DEP + "/🧾️generation"
TICKET = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
TOOLS = TICKET + "/dep-receipt-tools"
STAGE = TICKET + "/🗑️generated/dep-receipt/stage"

FILES = {
    "schema.json": GEN + "/🧬️schema/🔣️.json",
    "generation.ts": GEN + "/🟦️.ts",
    "storage.ts": GEN + "/💾️storage/🟦️.ts",
    "incremental.ts": DEP + "/🧾️incremental/💾️storage/🟦️.ts",
    "depstorage.ts": DEP + "/💾️storage/🟦️.ts",
    "bootstrap.ts": LIB + "/🏗️build/🚀️bootstrap/🟦️.ts",
}


def read(name):
    return open(FILES[name], encoding="utf-8").read()


def once(text, old, new, label):
    count = text.count(old)
    if count != 1:
        raise SystemExit("anchor %s matched %d times" % (label, count))
    return text.replace(old, new)


def stage():
    os.makedirs(STAGE, exist_ok=True)
    subprocess.check_call(["python3", TOOLS + "/schema_edit.py", FILES["schema.json"], STAGE + "/schema.json"])

    g = read("generation.ts")
    g = once(g, '"implementation-changed"|"runtime-changed"', '"protocol-changed"|"runtime-changed"', "g-class")
    g = once(g, '''/** 🗃️ Seals the runtime import closure of the walk and fence entries: the sorted entry list and the content-addressed per-file digest rows. */
export interface GenerationImplementationV1{readonly entries:readonly string[];readonly evidence:GenerationEvidenceV1}
''', "", "g-impl-iface")
    g = once(g, "readonly scope:string;readonly implementation:GenerationImplementationV1;readonly runtime", "readonly scope:string;readonly walkStamp:string;readonly runtime", "g-receipt")
    g = once(g, "readonly startedAtMs:number;readonly implementation:Readonly<{entries:readonly string[];loadedAtMs:number;files:readonly Readonly<{path:string;sha256:string;bytes:number;mtimeNs:string;ctimeNs:string}>[]}>;readonly runtime", "readonly startedAtMs:number;readonly runtime", "g-capture-input")
    g = once(g, " readonly implementationEntries:readonly string[];readonly implementationFile:(path:string)=>Readonly<{sha256:string;bytes:number}>|undefined;readonly producer", " readonly producer", "g-verify-input")
    g = once(g, ''' const loaded=BigInt(Math.trunc(input.implementation.loadedAtMs)-protocol.settleMilliseconds)*1000000n,unsettled=input.implementation.files.find(file=>BigInt(file.mtimeNs)>=loaded||BigInt(file.ctimeNs)>=loaded);
 if(unsettled)return deferred("implementation-unsettled",unsettled.path);
 const closure=[...input.implementation.files].sort((left,right)=>compareBytes(left.path,right.path)).map(file=>["f",file.path,file.sha256,file.bytes]as const);
 if(closure.length>protocol.implementation.maximumFiles||new Set(closure.map(row=>row[1])).size!==closure.length)throw Error("Bounded unique implementation closure required");
 const frontierFile=evidenceOf(frontier,protocol.frontierSuffix),executionFile=evidenceOf(observations,protocol.executionSuffix),closureFile=evidenceOf(closure,protocol.implementation.closureSuffix),sha=''', ''' const frontierFile=evidenceOf(frontier,protocol.frontierSuffix),executionFile=evidenceOf(observations,protocol.executionSuffix),sha=''', "g-capture-closure")
    g = once(g, "scope:input.scope,implementation:{entries:[...input.implementation.entries].sort(compareBytes),evidence:closureFile.evidence},runtime:input.runtime", "scope:input.scope,walkStamp:protocol.walkStamp,runtime:input.runtime", "g-capture-receipt")
    g = once(g, "files:Object.freeze([frontierFile.file,executionFile.file,closureFile.file])", "files:Object.freeze([frontierFile.file,executionFile.file])", "g-capture-files")
    g = once(g, '''const compareBytes=(left:string,right:string):number=>Buffer.compare(Buffer.from(left),Buffer.from(right));
''', "", "g-compare")
    g = once(g, '''const sameRuntime=(left:GenerationRuntimeV1,right:GenerationRuntimeV1)=>left.bun===right.bun&&left.revision===right.revision&&left.node===right.node&&left.platform===right.platform&&left.arch===right.arch&&left.sha256===right.sha256;
''', '''const runtimeFields=["bun","revision","node","platform","arch","sha256"]as const,movedRuntime=(left:GenerationRuntimeV1,right:GenerationRuntimeV1):string|undefined=>runtimeFields.find(field=>left[field]!==right[field]);
/** 🗣️ Renders the exact input a mismatch names in one declared language; there is no hidden default language. */
export function describeGenerationMismatchV1(language:string,result:Readonly<{class:string;path?:string;detail?:string}>,values:Readonly<Record<string,string|number>>={}):string{
 const table=(protocol.texts as Readonly<Record<string,Readonly<Record<string,string>>>>)[language],template=table?.[result.class+"."+(result.detail??"")]??table?.[result.class];
 if(!table||typeof template!=="string")throw Error("Declared language and message required");
 return Object.entries({path:result.path??"",detail:result.detail??"",expected:protocol.walkStamp,limit:protocol.maximumRelist,...values}).reduce((text,[key,value])=>text.replaceAll("{"+key+"}",String(value)),template);
}
''', "g-same-runtime")
    g = once(g, '''  if(generationCanonicalJsonV1([...input.implementationEntries].sort(compareBytes))!==generationCanonicalJsonV1(receipt.implementation.entries))return mismatch("implementation-changed",undefined,"entries");
  for(const row of decodeRows(input.readEvidence(receipt.implementation.evidence.name,protocol.maximumEvidenceBytes),receipt.implementation.evidence)){
   await advance();const live=input.implementationFile(row[1]as string);
   if(row[0]!=="f"||!live||live.sha256!==row[2]||live.bytes!==row[3])return mismatch("implementation-changed",row[1]as string,live?"bytes":"missing");
  }
  if(!sameRuntime(input.runtime(),receipt.runtime))return mismatch("runtime-changed");
''', '''  if(receipt.walkStamp!==protocol.walkStamp)return mismatch("protocol-changed",undefined,receipt.walkStamp);
  const movedField=movedRuntime(input.runtime(),receipt.runtime);if(movedField!==undefined)return mismatch("runtime-changed",undefined,movedField);
''', "g-verify-tier")
    open(STAGE + "/generation.ts", "w", encoding="utf-8").write(g)

    s = read("storage.ts")
    s = once(s, '''import {homedir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
''', '''import {homedir} from "node:os";
import {join,resolve} from "node:path";
''', "s-imports")
    s = once(s, '''import type {ScriptInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
''', "", "s-script-invocation")
    s = once(s, '''import {captureGenerationReceiptV1,confirmGenerationWalkV1,''', '''import {concurrentLanguageV1} from "../../🚦️concurrent/🟦️.ts";
import {captureGenerationReceiptV1,confirmGenerationWalkV1,describeGenerationMismatchV1,''', "s-generation-import")
    s = once(s, '''const repositoryRoot=resolve(dirname(fileURLToPath(import.meta.url)),"../../../../../../../../../.."),compareBytes=''', '''const compareBytes=''', "s-root")
    s = once(s, '''const evidenceName=/^[a-f0-9]{64}\\.(?:frontier|execution|closure)\\.ndjson$/u;''', '''const evidenceName=/^[a-f0-9]{64}\\.(?:frontier|execution)\\.ndjson$/u,evidenceSweep=/^[a-f0-9]{64}\\.[a-z]+\\.ndjson$/u;''', "s-evidence-name")
    s = once(s, '''/** 🧭️ Names what a receipt is sealed against: where the implementation closure lives, which entries span it and which ambient locations the installer environment could read. */
export interface JavascriptDependencyGenerationSubjectV1{readonly root:string;readonly entries:readonly string[];readonly ambient:JavascriptDependencyGenerationAmbientV1;readonly loadedAtMs:number}
''', '''/** 🧭️ Names what a receipt is sealed against besides the tree: the ambient locations the installer environment could read. */
export interface JavascriptDependencyGenerationSubjectV1{readonly ambient:JavascriptDependencyGenerationAmbientV1}
''', "s-subject")
    start = s.index("/** 🧱️ Reads one runtime closure file as the capture hashed it;")
    end = s.index("/** ⚙️ Reads the live tool identity from the same source the provider walk fingerprints. */")
    s = s[:start] + s[end:]
    s = once(s, '''/** 🎯️ Names the live subject: this checkout's closure entries and the process ambient. */
export const javascriptDependencyGenerationSubjectV1=():JavascriptDependencyGenerationSubjectV1=>Object.freeze({root:repositoryRoot,entries:protocol.implementation.entries,ambient:javascriptDependencyGenerationAmbientV1(),loadedAtMs:Date.now()-process.uptime()*1000});''', '''/** 🎯️ Names the live subject: the process ambient. */
export const javascriptDependencyGenerationSubjectV1=():JavascriptDependencyGenerationSubjectV1=>Object.freeze({ambient:javascriptDependencyGenerationAmbientV1()});''', "s-live-subject")
    s = once(s, "readEvidence:read,implementationEntries:subject.entries,implementationFile:javascriptDependencyGenerationImplementationFileV1(subject.root),runtime:", "readEvidence:read,runtime:", "s-ports")
    s = once(s, "export async function captureJavascriptDependencyGenerationReceiptV1(workspace:string,store:Readonly<{root:string}>,control:GenerationControlV1,walk:JavascriptDependencyGenerationWalkV1,observed:BunWorkspaceObservedOwnershipV1,startedAtMs:number,invocation:ScriptInvocation,subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1(),implementation?:Awaited<ReturnType<typeof captureJavascriptDependencyGenerationImplementationV1>>){", "export async function captureJavascriptDependencyGenerationReceiptV1(workspace:string,store:Readonly<{root:string}>,control:GenerationControlV1,walk:JavascriptDependencyGenerationWalkV1,observed:BunWorkspaceObservedOwnershipV1,startedAtMs:number,subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1()){", "s-capture-sig")
    s = once(s, "scope:evidence.scope,startedAtMs,implementation:implementation??await captureJavascriptDependencyGenerationImplementationV1(invocation,subject),runtime:", "scope:evidence.scope,startedAtMs,runtime:", "s-capture-call")
    s = once(s, '''const relativeWriteCeilingBytes=65536,replayEntries:readonly string[]=Object.freeze(["in-process-replay"]),replayImplementation:Awaited<ReturnType<typeof captureJavascriptDependencyGenerationImplementationV1>>=Object.freeze({entries:replayEntries,loadedAtMs:0,files:Object.freeze([])}),sharedRetireGraceMilliseconds=600000;''', '''const relativeWriteCeilingBytes=65536,sharedRetireGraceMilliseconds=600000;''', "s-replay-consts")
    s = once(s, "nothing is persisted, so the implementation closure and its settle rule do not apply. */", "nothing is persisted, so only the provider settle rule applies. */", "s-replay-doc")
    s = once(s, '''const generation=controlOf(context,control,"shared",checkpoint),replaySubject:JavascriptDependencyGenerationSubjectV1=Object.freeze({...subject,entries:replayEntries}),captured=await captureJavascriptDependencyGenerationReceiptV1(workspace,{root:context.root},generation,walk,observed,startedAtMs,context.invocation,replaySubject,replayImplementation);''', '''const generation=controlOf(context,control,"shared",checkpoint),captured=await captureJavascriptDependencyGenerationReceiptV1(workspace,{root:context.root},generation,walk,observed,startedAtMs,subject);''', "s-replay-capture")
    s = once(s, "captured.receipt,replaySubject));\n if(proof.status!==\"match\")return Object.freeze({status:\"mismatch\",class:proof.class", "captured.receipt,subject));\n if(proof.status!==\"match\")return Object.freeze({status:\"mismatch\",class:proof.class", "s-replay-verify")
    s = once(s, "const captured=await captureJavascriptDependencyGenerationReceiptV1(workspace,store,generation,walk,observed,startedAtMs,context.invocation,subject);", "const captured=await captureJavascriptDependencyGenerationReceiptV1(workspace,store,generation,walk,observed,startedAtMs,subject);", "s-write-capture")
    s = once(s, '''console.error("[DATA] Store dependency generation receipt stale "+JSON.stringify({version:1,purpose:"current-dependency-generation-receipt-stale",scope,...(proof.status==="mismatch"?{class:proof.class,...(proof.path===undefined?{}:{path:proof.path}),...(proof.detail===undefined?{}:{detail:proof.detail})}:{})}));return "stale";}''', '''reportJavascriptDependencyGenerationMismatchV1(scope,proof.status==="mismatch"?proof:{class:"seal"});return "stale";}''', "s-stale")
    s = once(s, '''/** 📣️ Reports the publication outcomes the receipt writer itself stays silent about. */''', '''/** 📣️ Reports exactly which input made a receipt stale: language-neutral DATA on stderr and one line in the declared language. */
export function reportJavascriptDependencyGenerationMismatchV1(scope:string,mismatch:Readonly<{class:string;path?:string;detail?:string}>,language:string=concurrentLanguageV1(process.env)):void{
 console.error("[DATA] Store dependency generation receipt stale "+JSON.stringify({version:1,purpose:"current-dependency-generation-receipt-stale",scope,class:mismatch.class,...(mismatch.path===undefined?{}:{path:mismatch.path}),...(mismatch.detail===undefined?{}:{detail:mismatch.detail})}));
 console.error(describeGenerationMismatchV1(language,{class:"stale"},{scope,reason:describeGenerationMismatchV1(language,mismatch)}));
}
/** 📣️ Reports the publication outcomes the receipt writer itself stays silent about. */''', "s-report")
    s = once(s, '''keep.add(receipt.membership.frontier.name);keep.add(receipt.execution.evidence.name);keep.add(receipt.implementation.evidence.name);}catch{}''', '''keep.add(receipt.membership.frontier.name);keep.add(receipt.execution.evidence.name);}catch{}''', "s-retire-keep")
    s = once(s, '''for(const name of names)if(evidenceName.test(name)&&!keep.has(name)&&aged(name))unlinkSync(join(directory,name));''', '''for(const name of names)if(evidenceSweep.test(name)&&!keep.has(name)&&aged(name))unlinkSync(join(directory,name));''', "s-retire-sweep")
    open(STAGE + "/storage.ts", "w", encoding="utf-8").write(s)

    g = open(STAGE + "/generation.ts", encoding="utf-8").read()
    g = once(g, "/** 🗣️ Renders the exact input a mismatch names", """/** 🔖️ Reduces any verification answer to the language-neutral cause a report names. */
export function generationCauseV1(result:GenerationVerifyResultV1|Readonly<{status:"absent"}>,scope:string):Readonly<{class:string;path?:string;detail?:string}>{
 if(result.status==="mismatch")return Object.freeze({class:result.class,...(result.path===undefined?{}:{path:result.path}),...(result.detail===undefined?{}:{detail:result.detail})});
 return Object.freeze(result.status==="absent"?{class:"absent",path:scope}:{class:"sha-changed"});
}
/** 🗣️ Renders the exact input a mismatch names""", "g-cause")
    open(STAGE + "/generation.ts", "w", encoding="utf-8").write(g)

    i = read("incremental.ts")
    i = once(i, 'import {generationSha256V1} from "../../🧾️generation/🟦️.ts";', 'import {generationCauseV1,generationSha256V1} from "../../🧾️generation/🟦️.ts";', "i-import")
    i = once(i, "Promise<Readonly<{status:\"current\";scopes:number}>|Readonly<{status:\"stale\";reason:string}>>{\n const stale=(reason:string)=>Object.freeze({status:\"stale\"as const,reason}),root=context.root;", "Promise<Readonly<{status:\"current\";scopes:number}>|Readonly<{status:\"stale\";reason:string;cause?:Readonly<{class:string;path?:string;detail?:string}>}>>{\n const stale=(reason:string,cause?:Readonly<{class:string;path?:string;detail?:string}>)=>Object.freeze({status:\"stale\"as const,reason,...(cause===undefined?{}:{cause})}),root=context.root;", "i-stale")
    i = once(i, 'if(read.status!=="match"||read.sha256!==row.read||read.receipts.find(receipt=>receipt.scope===row.scope)?.generation!==row.generation)return stale("generation:"+row.scope);', 'if(read.status!=="match")return stale("generation:"+row.scope,generationCauseV1(read,row.scope));\n  if(read.sha256!==row.read||read.receipts.find(receipt=>receipt.scope===row.scope)?.generation!==row.generation)return stale("generation:"+row.scope,{class:"sha-changed"});', "i-gen")
    open(STAGE + "/incremental.ts", "w", encoding="utf-8").write(i)

    d = read("depstorage.ts")
    d = once(d, 'await writeOwnedCommandOutputV1(process.stderr,"[DATA] Store dependency sync stale "+JSON.stringify({version:1,purpose:"current-store-dependency-sync-stale",reason:gate.reason})+"\\n",control);', 'await writeOwnedCommandOutputV1(process.stderr,"[DATA] Store dependency sync stale "+JSON.stringify({version:1,purpose:"current-store-dependency-sync-stale",reason:gate.reason,...(gate.cause===undefined?{}:{cause:gate.cause})})+"\\n",control);if(gate.cause){const{describeGenerationMismatchV1}=await import("../🧾️generation/🟦️.ts"),language=concurrentLanguageV1(process.env);await writeOwnedCommandOutputV1(process.stderr,describeGenerationMismatchV1(language,{class:"stale"},{scope:gate.reason,reason:describeGenerationMismatchV1(language,gate.cause)})+"\\n",control);}', "d-stale")
    open(STAGE + "/depstorage.ts", "w", encoding="utf-8").write(d)

    b = read("bootstrap.ts")
    b = once(b, 'fast=await generation.verifyJavascriptDependencyGenerationReceiptV1(workspace,context,actualControl,bootstrap.scope);if(cheap&&fast.status!=="match")return "material-required";', 'fast=await generation.verifyJavascriptDependencyGenerationReceiptV1(workspace,context,actualControl,bootstrap.scope);if(fast.status==="mismatch"&&!cheap&&!publishReceipt)generation.reportJavascriptDependencyGenerationMismatchV1(bootstrap.scope,fast);if(cheap&&fast.status!=="match")return "material-required";', "b-report")
    open(STAGE + "/bootstrap.ts", "w", encoding="utf-8").write(b)


if __name__ == "__main__":
    stage()
    print("staged")
