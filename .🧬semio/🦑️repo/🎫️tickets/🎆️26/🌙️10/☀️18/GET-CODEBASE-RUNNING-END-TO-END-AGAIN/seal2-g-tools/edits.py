"""🧩️ The anchored edits of the seal2-g batch, computed against a tree (normally the live one). Each edit is (file, name, old, new); `old` occurs exactly once in the file, `new` replaces it. Anchors avoid the lines the conv-g batch rewrites, so the batch applies on both trees.
usage (module): edits(root) -> list of dicts; build-stage.py turns them into the staged directory."""
import os
HERE = os.path.dirname(os.path.abspath(__file__))
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
F_LOADER = B + "/📥️source/🔏️closure/🍞️bun/🟦️.ts"
F_TRAMP = B + "/🏛️sealed/🚦️trampoline/🟦️.ts"
F_CORE = B + "/🏛️sealed/🟦️.ts"
F_POLICY = B + "/🏛️sealed/🔣️.json"
F_SCHEMA = B + "/🏛️sealed/🧬️schema/🔣️.json"
F_README = B + "/🏛️sealed/README.md"
F_T_STREAM = B + "/📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts"
F_T_STORE = B + "/🏛️sealed/🧪️tests/🗂️store/🟦️.ts"
F_T_TRAMP = B + "/🏛️sealed/🧪️tests/🚦️trampoline/🟦️.ts"


def read(path):
    return open(path, encoding="utf-8").read()


def span(text, start_marker, end_marker):
    start = text.index(start_marker)
    end = text.index(end_marker, start) + len(end_marker)
    assert text.count(start_marker) == 1
    return text[start:end]


def edits(root):
    out = []

    def add(file, name, old, new):
        out.append({"file": file, "name": name, "old": old, "new": new})

    loader = read(os.path.join(root, F_LOADER))
    functions = read(os.path.join(HERE, "src", "loader-functions.ts")).rstrip("\n") + "\n\n"
    anchor = "export /** 🏃️ Runs only checksum-bound captured modules at their original Source URLs. */"
    add(F_LOADER, "loader-functions", anchor, functions + anchor)
    old_read = "const chunks=[],chunk=Buffer.alloc(${policy.readChunkBytes});let size=0,used=0;for(;;){check();const count=fs.readSync(3,chunk,used,Math.min(chunk.length-used,${policy.maximumSnapshotBytes}-size+1),null);check();if(!count){if(used)chunks.push(Buffer.from(chunk.subarray(0,used)));break;}if(count>${policy.maximumSnapshotBytes}-size)throw Error('Bounded immutable implementation snapshot required');size+=count;used+=count;if(used===chunk.length){chunks.push(Buffer.from(chunk));used=0;}}const encoded=Buffer.concat(chunks,size);check();"
    new_read = "const facts=fs.fstatSync(3),chunks=[],chunk=Buffer.alloc(${policy.readChunkBytes});let size=0,used=0,encoded;if(facts.isFile()){if(facts.size>${policy.maximumSnapshotBytes})throw Error('Bounded immutable implementation snapshot required');encoded=Buffer.allocUnsafe(facts.size);while(used<encoded.length){check();const count=fs.readSync(3,encoded,used,encoded.length-used,null);if(!count)throw Error('Immutable implementation snapshot changed');used+=count;}if(fs.readSync(3,Buffer.alloc(1),0,1,null))throw Error('Bounded immutable implementation snapshot required');}else{for(;;){check();const count=fs.readSync(3,chunk,used,Math.min(chunk.length-used,${policy.maximumSnapshotBytes}-size+1),null);check();if(!count){if(used)chunks.push(Buffer.from(chunk.subarray(0,used)));break;}if(count>${policy.maximumSnapshotBytes}-size)throw Error('Bounded immutable implementation snapshot required');size+=count;used+=count;if(used===chunk.length){chunks.push(Buffer.from(chunk));used=0;}}encoded=Buffer.concat(chunks,size);}check();"
    add(F_LOADER, "loader-read", old_read, new_read)
    add(F_LOADER, "loader-identity", "loaderIdentity=()=>hash(evaluatedLoader),", "loaderIdentity=(()=>{let known;return()=>known??=hash(evaluatedLoader);})(),")
    add(F_LOADER, "loader-split",
        "const value=JSON.parse(encoded.toString('utf8'));${overlay?overlaySource(overlay):''}",
        "const redirectCapturedBuiltinReadsV1=(${redirectCapturedBuiltinReadsV1.toString()}),splitCapturedSnapshotV1=(${splitCapturedSnapshotV1.toString()}),admitCapturedImagesV1=(${admitCapturedImagesV1.toString()});\nconst split=${auxiliary?'splitCapturedSnapshotV1(encoded)':'undefined'},value=JSON.parse(split?split.text:encoded.toString('utf8')),spans=split&&Array.isArray(value?.files)&&split.spans.length===value.files.length?split.spans:undefined;${overlay?overlaySource(overlay):''}")
    old_images = span(loader, "const images=new Map(),resolutions=new Map();let total=0;for(const row of value.files){", "images.set(name,bytes);resolutions.set(name,row.imports);row.base64='';}")
    add(F_LOADER, "loader-images", old_images,
        "const{images,resolutions}=admitCapturedImagesV1(value,encoded,spans,{maximumFileBytes:${policy.maximumFileBytes},maximumSourceBytes:${policy.maximumSourceBytes}},hash,path,check);")
    old_redirect = span(loader, "const redirect=(source,commonJs=false)=>{const parser=new Bun.Transpiler({loader:'js',target:'bun'}),targets=new Map([", "return result;};")
    targets = old_redirect[old_redirect.index("new Map(["):old_redirect.index("]),expected=") + 2]
    add(F_LOADER, "loader-redirect", old_redirect,
        "const redirectParser=new Bun.Transpiler({loader:'js',target:'bun'}),redirectTargets=" + targets + ",redirect=(source,commonJs=false)=>redirectCapturedBuiltinReadsV1(source,commonJs,redirectParser,redirectTargets);")

    tramp = read(os.path.join(root, F_TRAMP))
    add(F_TRAMP, "tramp-imports", 'import{createReadStream,statSync}from"node:fs";', 'import{closeSync,createReadStream,fstatSync,openSync}from"node:fs";')
    old_names = "import{implementationModeV1,readSealedLoaderV1,readSealedPointerV1,readSealedReceiptV1,sealedEntryOfV1,sealedLanguagesV1,sealedLightCommandV1,sealedPolicyV1,sealedTextV1,sealedTreeDriftV1}from\"../🟦️.ts\";"
    assert old_names in tramp
    add(F_TRAMP, "tramp-core-imports", old_names,
        "import{implementationModeV1,readSealedBlockedV1,readSealedCheckedV1,readSealedLoaderV1,readSealedPointerV1,readSealedReceiptV1,sealedEntryOfV1,sealedLanguagesV1,sealedLightActionV1,sealedLightCommandV1,sealedPolicyV1,sealedTextV1,sealedTreeDriftV1,writeSealedCheckedV1,type ImplementationModeV1,type SealedEntryV1,type SealedPointerV1}from\"../🟦️.ts\";")
    add(F_TRAMP, "tramp-options",
        "export interface TrampolineOptionsV1{readonly entry?:string;readonly promote?:(root:string,entry:string,environment:NodeJS.ProcessEnv,write:(text:string)=>void)=>Promise<TrampolinePromotionV1>}",
        "export interface TrampolineOptionsV1{readonly entry?:string;readonly promote?:(root:string,entry:string,environment:NodeJS.ProcessEnv,write:(text:string)=>void)=>Promise<TrampolinePromotionV1>;readonly refresh?:(argv:readonly string[],environment:NodeJS.ProcessEnv)=>number|undefined}")
    helpers = read(os.path.join(HERE, "src", "trampoline-helpers.txt"))
    anchor = "const promoteLiveV1=async("
    assert anchor in tramp
    add(F_TRAMP, "tramp-helpers", anchor, helpers + anchor)
    old_block = span(tramp, " let pointer=readSealedPointerV1(store);\n const receipt=", "  pointer=readSealedPointerV1(store);\n }\n")
    add(F_TRAMP, "tramp-decision", old_block, read(os.path.join(HERE, "src", "trampoline-new.txt")))
    add(F_TRAMP, "tramp-open",
        ' try{if(statSync(store.snapshot(pointer.digest)).size!==pointer.bytes)return live();}catch{return live();}\n',
        ' let snapshot:number|undefined;\n if(process.platform!=="win32")try{snapshot=openSync(store.snapshot(pointer.digest),"r");if(fstatSync(snapshot).size!==pointer.bytes){closeSync(snapshot);return live();}}catch{if(snapshot!==undefined)try{closeSync(snapshot);}catch{}return live();}\n')
    add(F_TRAMP, "tramp-stdio", 'stdio:["inherit","inherit","inherit","pipe","pipe"]', 'stdio:["inherit","inherit","inherit",snapshot??"pipe","pipe"]')
    add(F_TRAMP, "tramp-close", '  const feed=child.stdio[3] as NodeJS.WritableStream|null;\n', '  if(snapshot!==undefined)try{closeSync(snapshot);}catch{}\n  const feed=child.stdio[3] as NodeJS.WritableStream|null;\n')
    add(F_TRAMP, "tramp-doc", "Signals reach the sealed door, its exit code is ours.",
        "Signals reach the sealed door, its exit code is ours. A light command checks the live tree only once its snapshot is older than `light.freshMilliseconds`: a drifted tree older than `light.promoteAfterMilliseconds` is promoted before the command runs, a younger one is promoted by a detached process while the command runs the snapshot (one notice, debounced), and a snapshot still older than `light.refuseAfterMilliseconds` after the promotion attempt is refused with exit 75. On POSIX the snapshot reaches the sealed door as an inherited read-only descriptor of its stored file (one read instead of a pipe through this process).")

    core = read(os.path.join(root, F_CORE))
    add(F_CORE, "core-policy-type", ";readonly check:Readonly<{argument:string",
        ";readonly light:Readonly<{freshMilliseconds:number;promoteAfterMilliseconds:number;refuseAfterMilliseconds:number;refreshDebounceMilliseconds:number}>;readonly check:Readonly<{argument:string")
    add(F_CORE, "core-entry-type", "readonly blocked:string;readonly lock:string;", "readonly blocked:string;readonly checked:string;readonly lock:string;")
    add(F_CORE, "core-entry-value", 'blocked:join(directory,"blocked.json"),', 'blocked:join(directory,"blocked.json"),checked:join(directory,"checked.json"),')
    old_reader = [line for line in core.split("\n") if line.startswith("export const readSealedBlockedV1=")]
    assert len(old_reader) == 1
    add(F_CORE, "core-checked", old_reader[0], old_reader[0] + "\n" + read(os.path.join(HERE, "src", "core-checked.txt")).rstrip("\n"))

    policy = read(os.path.join(root, F_POLICY))
    add(F_POLICY, "policy-light", '  "coalesceMilliseconds": 0,\n',
        '  "coalesceMilliseconds": 0,\n  "light": {\n    "freshMilliseconds": 300000,\n    "promoteAfterMilliseconds": 1800000,\n    "refuseAfterMilliseconds": 86400000,\n    "refreshDebounceMilliseconds": 120000\n  },\n')
    add(F_POLICY, "policy-texts", '    "promoting": {\n', read(os.path.join(HERE, "src", "policy-texts.txt")) + '    "promoting": {\n')

    add(F_SCHEMA, "schema-required", '"coalesceMilliseconds", ', '"coalesceMilliseconds", "light", ')
    add(F_SCHEMA, "schema-light", '    "coalesceMilliseconds": { "type": "integer", "minimum": 0 },\n',
        '    "coalesceMilliseconds": { "type": "integer", "minimum": 0 },\n    "light": {\n      "type": "object",\n      "additionalProperties": false,\n      "required": ["freshMilliseconds", "promoteAfterMilliseconds", "refuseAfterMilliseconds", "refreshDebounceMilliseconds"],\n      "properties": {\n        "freshMilliseconds": { "type": "integer", "minimum": 0 },\n        "promoteAfterMilliseconds": { "type": "integer", "minimum": 0 },\n        "refuseAfterMilliseconds": { "type": "integer", "minimum": 0 },\n        "refreshDebounceMilliseconds": { "type": "integer", "minimum": 0 }\n      }\n    },\n')
    add(F_SCHEMA, "schema-texts-required", '"selfCheckEnded"],', '"selfCheckEnded", "stale", "refused"],')
    add(F_SCHEMA, "schema-texts-properties", '        "startup": { "$ref": "#/definitions/Text" }\n',
        '        "startup": { "$ref": "#/definitions/Text" },\n        "stale": { "$ref": "#/definitions/Text" },\n        "refused": { "$ref": "#/definitions/Text" }\n')
    add(F_SCHEMA, "schema-checked", '    "Digest": { "type": "string", "pattern": "^[a-f0-9]{64}$" },\n',
        '    "Checked": {\n      "type": "object",\n      "additionalProperties": false,\n      "required": ["version", "entry", "digest", "atMs", "drifted", "refreshedAtMs"],\n      "properties": {\n        "version": { "const": 1 },\n        "entry": { "$ref": "#/definitions/Path" },\n        "digest": { "$ref": "#/definitions/Digest" },\n        "atMs": { "type": "integer", "minimum": 0 },\n        "drifted": { "type": "boolean" },\n        "refreshedAtMs": { "type": ["integer", "null"], "minimum": 0 }\n      }\n    },\n    "Digest": { "type": "string", "pattern": "^[a-f0-9]{64}$" },\n')

    def append(file, name, source):
        text = read(os.path.join(root, file))
        tail = text.rstrip("\n")[-200:]
        assert text.count(tail) == 1, name
        add(file, name, tail, tail + "\n" + read(os.path.join(HERE, "src", "tests", source)).rstrip("\n"))

    add(F_T_STREAM, "test-stream-anchor", 'end=loader.indexOf("\\nconst value=JSON.parse")', 'end=loader.indexOf("\\nconst redirectCapturedBuiltinReadsV1=")')
    add(F_T_STREAM, "test-stream-fstat", "fsValue={readSync:", "fsValue={fstatSync:()=>({isFile:()=>false}),readSync:")
    append(F_T_STREAM, "test-stream-laws", "stream.txt")
    append(F_T_STORE, "test-store-laws", "store.txt")
    append(F_T_TRAMP, "test-trampoline-laws", "trampoline.txt")
    return out
