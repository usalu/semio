"""⚖️ [td-observe] brings the block and presence laws of `👁️observation/🧪️tests` up to the current design (run AFTER edit_children.py):
1. blocks: a damaged or absent persisted block is a MISS (graph-b 10-25, `observationBlockDamagedV1`: absent / packed / hash / closed-rows faults
   recompute once and republish the leaf), every unsafe physical shape still refuses; schema enum + corpus + both laws + child `republished` probe.
2. both child laws become one law per corpus row: 14 (blocks) and 26 (presence) fixture births inside ONE law take 8-12 s each on a loaded
   host and exhaust the 110 s `sdk-runtime` body budget (run 4 at a time they serialize inside and take 20-50 s each, measured), so each row
   gets its own budget and its own verdict.
3. presence terminal law: SIGINT aborts with the process module's own text "Original Script process interrupted"
   (`🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`), which the cancellation vocabulary lacked.
usage: python3 -I edit_laws.py <root> (idempotent, anchored, refuses unknown shapes)."""
import json, sys

ROOT = sys.argv[1]
OBS = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/"
RECOMPUTE = ["missing", "packed", "hash", "contract"]

def edit(path, pairs):
    with open(path, encoding="utf8") as handle:
        text = handle.read()
    before = text
    for old, new in pairs:
        if new in text:
            continue
        if text.count(old) != 1:
            raise SystemExit(f"refused: anchor ({old[:60]}...) in {path}")
        text = text.replace(old, new)
    if text != before:
        with open(path, "w", encoding="utf8") as handle:
            handle.write(text)
    print(("edited " if text != before else "already ") + path.split("👁️observation/")[1])


def edit_json(path, change):
    with open(path, encoding="utf8") as handle:
        raw = handle.read()
    data = json.loads(raw)
    change(data)
    text = json.dumps(data, ensure_ascii=False, indent=2) + "\n"
    if text != raw:
        with open(path, "w", encoding="utf8") as handle:
            handle.write(text)
    print(("edited " if text != raw else "already ") + path.split("👁️observation/")[1])


def schema(data):
    defs = data["$defs"]
    defs["blockRevalidationCase"]["properties"]["outcome"] = {"enum": ["refuse", "recompute"]}
    defs["blockRecoveryLifecycle"]["properties"]["states"]["items"]["enum"] = ["warm", "missing-block-recomputed", "group-absent", "recomputed"]


def corpus(data):
    for row in data["faults"]:
        row["outcome"] = "recompute" if row["fault"] in RECOMPUTE else "refuse"
    data["recovery"]["states"] = ["warm", "missing-block-recomputed", "group-absent", "recomputed"]


edit_json(OBS + "🧬️schema/🔣️.json", schema)
edit_json(OBS + "🧪️tests/🧱️blocks/🧫️cases.json", corpus)

BLOCK_LAW_OLD = 'registerBuildTestLawV1("observation block readers refuse stale physical memos without recomputing or deleting evidence","sdk-runtime",async original=>{\n const control=createIssuedResourceControlV1(original,"observation:block-refusals",120000);for(const row of corpus.faults){expect(new Ajv({strict:true}).validate(schema.$defs.blockRevalidationCase,row)).toBe(true);const entry=fileURLToPath(new URL("./👷️child/📜️script.ts",import.meta.url)),result=await captureOwnedCommandResultV1(process.execPath,[entry,row.fault],workspace,{control,env:scriptInvocationEnvironment(original,process.env),output:{maximumBytes:32768,maximumLines:256}}),value=JSON.parse(result.stdout.trim().split("\\n").at(-1)??"");expect(result.signal,result.stderr).toBeNull();expect(result.status,result.stderr).toBe(1);expect(value.fault).toBe(row.fault);expect(value.warm).toBe(true);expect(value.calls).toBe(1);expect(value.readerFailed).toBe(true);expect(value.samePrimary).toBe(true);expect(value.evidenceRetained).toBe(row.fault!=="missing");if(row.fault==="missing")expect(value.code).toBe("BUILD_INFERENCE_BLOCK_ABSENT");'
BLOCK_LAW_NEW = 'for(const row of corpus.faults)registerBuildTestLawV1("observation block reader "+(row.outcome==="recompute"?"recomputes a":"refuses a")+" "+row.id+" block "+(row.outcome==="recompute"?"and republishes its leaf":"without recomputing or deleting evidence"),"sdk-runtime",async original=>{\n expect(new Ajv({strict:true}).validate(schema.$defs.blockRevalidationCase,row)).toBe(true);const control=createIssuedResourceControlV1(original,"observation:block-refusals",120000),entry=fileURLToPath(new URL("./👷️child/📜️script.ts",import.meta.url)),result=await captureOwnedCommandResultV1(process.execPath,[entry,row.fault],workspace,{control,env:scriptInvocationEnvironment(original,process.env),output:{maximumBytes:32768,maximumLines:256}}),value=JSON.parse(result.stdout.trim().split("\\n").at(-1)??"");expect(result.signal,result.stderr).toBeNull();expect(value.fault).toBe(row.fault);expect(value.warm).toBe(true);expect(value.samePrimary).toBe(true);if(row.outcome==="recompute"){expect(result.status,result.stderr).toBe(0);expect(value.calls).toBe(2);expect(value.readerFailed).toBe(false);expect(value.republished).toBe(true);return;}expect(result.status,result.stderr).toBe(1);expect(value.calls).toBe(1);expect(value.readerFailed).toBe(true);expect(value.evidenceRetained).toBe(true);'
BLOCK_TAIL_OLD = 'toBe(process.platform!=="win32");}\n});'
BLOCK_TAIL_NEW = 'toBe(process.platform!=="win32");\n});'
RECOVERY_OLD = 'let refused=false;try{await observeInference({cachePath:path,key:"first"},compute);}catch(error){expect((error as {code?:string}).code).toBe("BUILD_INFERENCE_BLOCK_ABSENT");refused=true;}expect(refused).toBe(true);expect(calls).toBe(1);expect(existsSync(path)).toBe(true);expect(existsSync(missing)).toBe(false);states.push("typed-missing-refusal");'
RECOVERY_NEW = 'expect((await observeInference({cachePath:path,key:"first"},compute)).generation).toBe(2);expect(calls).toBe(2);expect(existsSync(path)).toBe(true);expect(hash(brotliDecompressSync(readFileSync(missing)))).toBe(digests[0]!);states.push("missing-block-recomputed");'
FRESH_OLD = 'expect(value.generation).toBe(2);expect(calls).toBe(2);states.push("recomputed");'
FRESH_NEW = 'expect(value.generation).toBe(3);expect(calls).toBe(3);states.push("recomputed");'
edit(OBS + "🧪️tests/🧱️blocks/🟦️.ts", [
    (BLOCK_LAW_OLD, BLOCK_LAW_NEW),
    (RECOVERY_OLD, RECOVERY_NEW),
    (BLOCK_TAIL_OLD, BLOCK_TAIL_NEW),
    (FRESH_OLD, FRESH_NEW),
])

CHILD_DECL_OLD = "secondaryClosed=false;const actualOpen=nativeFs.openSync"
CHILD_DECL_NEW = "secondaryClosed=false,republished=false;const actualOpen=nativeFs.openSync"
CHILD_PROBE_OLD = "else await attempt();\n"
CHILD_PROBE_NEW = "else await attempt();\n  if(!readerFailed)republished=readInferenceHeaderV1(cachePath).blocks.every((digest:string)=>hash(brotliDecompressSync(readFileSync(inferenceObservationBlockPathV1(directory,digest))))===digest);\n"
CHILD_OUT_OLD = "fifoSupported,secondaryClosed,message:"
CHILD_OUT_NEW = "fifoSupported,secondaryClosed,republished,message:"
edit(OBS + "🧪️tests/🧱️blocks/👷️child/🟦️.ts", [
    (CHILD_DECL_OLD, CHILD_DECL_NEW),
    (CHILD_PROBE_OLD, CHILD_PROBE_NEW),
    (CHILD_OUT_OLD, CHILD_OUT_NEW),
])

PRESENCE_OLD = 'registerBuildTestLawV1("inference reuse refuses every nonleaf absence and unsafe inventory under managed custody","sdk-runtime",async original=>{\n const phase=createIssuedResourceControlV1(original,"inference:refusals",360000),db=new Database(":memory:");try{for(const row of corpus.errorCases){expect(new Ajv({strict:true}).validate(schema.definitions.ErrorClassificationCase,row)).toBe(true);expect(inferenceInventoryLeafMissingV1(row.code)).toBe(row.missing);expect(db.query<{missing:number},[string]>("select ?1=\'ENOENT\' missing").get(row.code)?.missing===1).toBe(row.missing);}for(const kind of["source","lock"]as const)for(const row of corpus.cases.filter(row=>row.outcome==="refuse")){const{result,value}=await childOutcome(original,phase,"refusal",kind,row.fault),'
PRESENCE_NEW = 'registerBuildTestLawV1("inference reuse classifies only an exact leaf ENOENT as a missing inventory","sdk-runtime",async()=>{\n const db=new Database(":memory:");try{for(const row of corpus.errorCases){expect(new Ajv({strict:true}).validate(schema.definitions.ErrorClassificationCase,row)).toBe(true);expect(inferenceInventoryLeafMissingV1(row.code)).toBe(row.missing);expect(db.query<{missing:number},[string]>("select ?1=\'ENOENT\' missing").get(row.code)?.missing===1).toBe(row.missing);}}finally{db.close();}\n});\n\nfor(const kind of["source","lock"]as const)for(const row of corpus.cases.filter(row=>row.outcome==="refuse"))registerBuildTestLawV1("inference reuse refuses a "+row.fault+" "+kind+" inventory under managed custody","sdk-runtime",async original=>{\n const phase=createIssuedResourceControlV1(original,"inference:refusals",180000),db=new Database(":memory:");try{const{result,value}=await childOutcome(original,phase,"refusal",kind,row.fault),'
PRESENCE_TAIL_OLD = '"cold-fifo","cold-close"].includes(row.fault))expect(value.mutated).toBe(true);}}finally{db.close();}\n});'
PRESENCE_TAIL_NEW = '"cold-fifo","cold-close"].includes(row.fault))expect(value.mutated).toBe(true);}finally{db.close();}\n});'
edit(OBS + "🧪️tests/🪪️presence/🟦️.ts", [
    (PRESENCE_OLD, PRESENCE_NEW),
    (PRESENCE_TAIL_OLD, PRESENCE_TAIL_NEW),
    ("toMatch(/deadline|abort|cancel|SIGINT|terminat|expir|budget|timeout/iu)", "toMatch(/deadline|abort|cancel|SIGINT|interrupt|terminat|expir|budget|timeout/iu)"),
])
