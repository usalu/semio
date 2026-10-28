"""🧳 Anchored edits of batch `carry` (lane printhot): `python3 -I edit.py <root> check|apply`.

The graph memo carries a sealed slice to a newer generation when the slice is the same there (memo core: optional `carry` port in `lookup`; schema: policy `carry.shared` + EN/DE `carried` text; command: wires the port to the generation store and prints the carried line). New files come from `new/`. `check` exits 0 when every edit is applicable or already applied, 2 otherwise; `apply` is idempotent."""
import os, sys

M = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo"
HERE = os.path.dirname(os.path.abspath(__file__))

EDITS = {
    M + "/🟦️.ts": [
        ('texts:Readonly<Record<string,Readonly<Record<"served"|"refused",string>>>>;\n}>);',
         'texts:Readonly<Record<string,Readonly<Record<"served"|"refused"|"carried",string>>>>;carry:Readonly<{shared:readonly string[]}>;\n}>);'),
        ("rows:number;rechecked:number}>|Readonly<{hit:false;reason:MemoReasonV1;detail?:string}>;",
         "rows:number;rechecked:number;carried?:Readonly<{from:number;to:number}>}>|Readonly<{hit:false;reason:MemoReasonV1;detail?:string}>;"),
        ('export function renderGraphMemoTextV1(key:"served"|"refused",',
         'export function renderGraphMemoTextV1(key:"served"|"refused"|"carried",'),
        ("/** 🧱️ The ports of one memo: the journal client, the record directory, the workspace root, the identity of the request and the engine check of the rows out of the journal's sight. */",
         "/** 🧱️ The ports of one memo: the journal client, the record directory, the workspace root, the identity of the request, the engine check of the rows out of the journal's sight and the carry of a sealed slice to the generation the pointer names now (without it a moved pointer refuses). */"),
        (" holds?:(rows:readonly MemoRowV1[],signal?:AbortSignal)=>Promise<boolean>;\n}>;",
         " holds?:(rows:readonly MemoRowV1[],signal?:AbortSignal)=>Promise<boolean>;\n carry?:(input:Readonly<{from:Readonly<{generation:number;manifest:string}>;to:Readonly<{generation:number;manifest:string}>;pin:string}>,signal?:AbortSignal)=>Promise<Readonly<{carried:true;pin:string}>|Readonly<{carried:false;reason:string}>>;\n}>;"),
        ('const pointer=pointerOf(readBounded(record.pointer.file,65536));if(!pointer||pointer.generation!==record.pointer.generation||pointer.manifest!==record.pointer.manifest)return miss("pointer");',
         'const pointer=pointerOf(readBounded(record.pointer.file,65536)),moved=!pointer||pointer.generation!==record.pointer.generation||pointer.manifest!==record.pointer.manifest;if(!pointer||moved&&!options.carry)return miss("pointer");'),
        ("    const planBytes=readBounded(named(directory,record.plan.sha256,policy.planSuffix),policy.maximumPlanBytes);",
         "    let served=record;\n    if(moved){const carried=await options.carry!({from:{generation:record.pointer.generation,manifest:record.pointer.manifest},to:pointer,pin:record.pin},signal);if(!carried.carried)return miss(\"pointer\",carried.reason);served={...record,manifest:pointer.manifest,pointer:{...record.pointer,generation:pointer.generation,manifest:pointer.manifest},pin:carried.pin};}\n    const planBytes=readBounded(named(directory,record.plan.sha256,policy.planSuffix),policy.maximumPlanBytes);"),
        ("const at=now(),pin=reown(record.pin,environment,at);", "const at=now(),pin=reown(served.pin,environment,at);"),
        ("try{atomic(recordFile,JSON.stringify({...record,cursor:formatChangeCursorV1(answer.cursor)}));}catch{}",
         "try{atomic(recordFile,JSON.stringify({...served,cursor:formatChangeCursorV1(answer.cursor)}));}catch{}"),
        ("ageMilliseconds:Math.max(0,at-record.sealedAtMs),rows:record.relevance.rows,rechecked};",
         "ageMilliseconds:Math.max(0,at-record.sealedAtMs),rows:record.relevance.rows,rechecked,...(moved?{carried:{from:record.pointer.generation,to:pointer.generation}}:{})};"),
    ],
    M + "/🧬️schema/🔣️.json": [
        ("(except the modification of a directory).\"",
         "(except the modification of a directory). `carry`: a sealed slice whose generation pointer moved is served under the newer generation when every generation-wide digest named in `shared` and the row of every owner the slice covers are identical in both manifests (the planner would verify the same facts and assemble the same projects from the same rows); otherwise the reason `pointer` names the first difference.\""),
        ('        "texts": {\n          "en": {',
         '        "carry": {\n          "shared": [\n            "env",\n            "global",\n            "identity",\n            "implementation"\n          ]\n        },\n        "texts": {\n          "en": {'),
        ('            "refused": "graph: the memo cannot serve this request ({reason}); the graph is planned"\n',
         '            "refused": "graph: the memo cannot serve this request ({reason}); the graph is planned",\n            "carried": "graph: the memo of generation {from} still describes generation {to}: the {projects} projects it serves are unchanged there"\n'),
        ('            "refused": "graph: das Memo kann diese Anfrage nicht bedienen ({reason}); der Graph wird geplant"\n',
         '            "refused": "graph: das Memo kann diese Anfrage nicht bedienen ({reason}); der Graph wird geplant",\n            "carried": "graph: das Memo der Generation {from} beschreibt auch Generation {to}: die {projects} gelieferten Projekte sind dort unverändert"\n'),
    ],
    M + "/🚪️command/🟦️.ts": [
        ("/** 🧠️ Opens the graph memo of one command in this process: off when the switch says so or when this process does not run a captured implementation (the identity of the implementation is part of the key). The journal cursor is taken here, before any planner child starts.",
         "/** 🧠️ Opens the graph memo of one command in this process: off when the switch says so or when this process does not run a captured implementation (the identity of the implementation is part of the key). The journal cursor is taken here, before any planner child starts. A sealed slice whose generation was superseded by another request's refresh is carried to the current generation when it is the same there.\n * @see ../🧳️carry/🟦️.ts"),
        ("  holds:async(rows,abort)=>{const {observationsHold}=await import(\"../../../../👁️observation/🟨️.mjs\");return observationsHold(rows,64,new Set(),true,abort);}});",
         "  holds:async(rows,abort)=>{const {observationsHold}=await import(\"../../../../👁️observation/🟨️.mjs\");return observationsHold(rows,64,new Set(),true,abort);},\n  carry:async(input,abort)=>{const [{carryGraphMemoPinV1},{repositoryCachePartitionRootV1}]=await Promise.all([import(\"../🧳️carry/🟦️.ts\"),import(\"../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts\")]);return carryGraphMemoPinV1({generations:join(repositoryCachePartitionRootV1(root,\"graph\"),\"generations\"),...input,...(abort?{signal:abort}:{})});}});"),
        ("export async function renderGraphMemoServedV1(answer:Extract<MemoAnswerV1,{hit:true}>,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<string>{\n return renderGraphMemoTextV1(\"served\",{projects:answer.projects,seconds:Math.round(answer.ageMilliseconds/1000),rows:answer.rows,rechecked:answer.rechecked},await memoLanguageV1(environment));\n}",
         "export async function renderGraphMemoServedV1(answer:Extract<MemoAnswerV1,{hit:true}>,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<string>{\n const language=await memoLanguageV1(environment),line=renderGraphMemoTextV1(\"served\",{projects:answer.projects,seconds:Math.round(answer.ageMilliseconds/1000),rows:answer.rows,rechecked:answer.rechecked},language);\n return answer.carried?line+\"\\n\"+renderGraphMemoTextV1(\"carried\",{from:answer.carried.from,to:answer.carried.to,projects:answer.projects},language):line;\n}"),
        ('/** 🗣️ The one line a served graph prints, in the language of the command. */',
         '/** 🗣️ The line a served graph prints, in the language of the command, and a second one when its slice was carried to a newer generation. */'),
        ('input.mark("graph.memo",served===undefined?"off":served.hit?"hit":"miss:"+served.reason);',
         'input.mark("graph.memo",served===undefined?"off":served.hit?served.carried?"hit:carried":"hit":"miss:"+served.reason);'),
    ],
}
NEW = {
    M + "/🧳️carry/🟦️.ts": "new/carry.ts",
    M + "/🧪️tests/🧳️carry/🟦️.ts": "new/law.ts",
    M + "/🧪️tests/🧳️carry/🔣️.json": "new/cases.json",
    M + "/🧪️tests/🧳️carry/🐍️.py": "new/oracle.py",
}


def run(root, verb):
    bad = 0
    for rel, edits in EDITS.items():
        path = os.path.join(root, rel)
        text = open(path, encoding="utf8").read()
        for old, new in edits:
            if text.count(new) == 1 and new not in old:
                continue
            if text.count(old) != 1:
                print("ANCHOR", text.count(old), rel[-60:], old[:80].replace("\n", "\\n")); bad += 1; continue
            text = text.replace(old, new)
        if verb == "apply" and not bad:
            open(path, "w", encoding="utf8").write(text)
    for rel, source in NEW.items():
        data = open(os.path.join(HERE, source), "rb").read()
        if verb == "apply" and not bad:
            os.makedirs(os.path.dirname(os.path.join(root, rel)), exist_ok=True)
            open(os.path.join(root, rel), "wb").write(data)
    print(("applied" if verb == "apply" else "applicable") if not bad else "DRIFT", bad)
    sys.exit(2 if bad else 0)


run(os.path.abspath(sys.argv[1]), sys.argv[2])
