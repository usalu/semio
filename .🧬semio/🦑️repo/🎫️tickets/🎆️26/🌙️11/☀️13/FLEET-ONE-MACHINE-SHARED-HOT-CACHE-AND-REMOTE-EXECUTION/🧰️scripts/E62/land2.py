import hashlib
B = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"

def edit(path, pairs):
    text = open(B + path, encoding="utf8").read()
    before = hashlib.sha256(text.encode()).hexdigest()[:8]
    for old, new in pairs:
        if text.count(old) != 1:
            raise SystemExit(f"{path}: anchor found {text.count(old)} times: {old[:90]}")
        text = text.replace(old, new)
    open(B + path, "w", encoding="utf8").write(text)
    print(path.split("/")[-2], before, "->", hashlib.sha256(text.encode()).hexdigest()[:8])

WASM = '''/** 🕸️ The digest of a Wasm compile output index: every key with its producer, and the namespaces. */
const wasmIndexDigests=new WeakMap();
const wasmIndexDigestV1=index=>{let known=wasmIndexDigests.get(index);if(known===undefined){known=createHash("sha256").update(JSON.stringify([[...index.outputs].map(([key,row])=>[key,row.project,row.target,row.manifest,row.output,row.declarations??null,row.sources]).sort((a,b)=>a[0]<b[0]?-1:a[0]>b[0]?1:0),[...index.namespaces].sort()])).digest("hex");wasmIndexDigests.set(index,known);}return known;};
const wasmFileTextV1=file=>{if(!nodeFs.existsSync(file))return undefined;if(file.endsWith("📋️project.json")){try{return JSON.stringify(wasmProducerDeclarationV1(nodeFs.readFileSync(file,"utf8")));}catch{return undefined;}}if(file.endsWith("/package.json")){try{return JSON.stringify(wasmPackageContractV1(nodeFs.readFileSync(file,"utf8")));}catch{return undefined;}}return file.endsWith("/Cargo.toml")?"":nodeFs.readFileSync(file,"utf8");};
registerOwnerRegistryQuestionV1("wasm-index",(registry,_arguments,workspaceRoot)=>wasmIndexDigestV1(wasmCompileOutputIndexV1(workspaceRoot,registry.owners,wasmFileTextV1)));
'''

edit("🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs", [
 ('import { registryOwnerIndexesV1, buildInputSourceRoleV1,', 'import { askOwnerRegistryV1, observeOwnerRegistryAnswerV1, observedOwnerRegistryViewV1, registerOwnerRegistryQuestionV1 } from "../../🧾️registry-reads/🟨️.mjs";\nimport * as nodeFs from "node:fs";\nimport { buildInputSourceRoleV1,'),
 (' await stage("declaration");\n', ' await stage("declaration");\n askOwnerRegistryV1(workspaceRoot,registry,["row",owner.name]);\n'),
 (' const contractsPath=join(workspaceRoot,', ' askOwnerRegistryV1(workspaceRoot,registry,["providers",owner.root,targets.length>0,[...inputNames]]);\n const contractsPath=join(workspaceRoot,'),
 ('=registryOwnerIndexesV1(registry);', '=observedOwnerRegistryViewV1(workspaceRoot,registry);'),
 ('export async function catalogNativeManifestCandidatesV1(workspaceRoot,registry,accept,context){\n', 'export async function catalogNativeManifestCandidatesV1(workspaceRoot,registry,accept,context){\n askOwnerRegistryV1(workspaceRoot,registry,["candidates"]);\n'),
 ('registerFieldExtractorV1("wasm-producer-declaration",text=>{const declaration=JSON.parse(text),targets={};', 'const wasmProducerDeclarationV1=text=>{const declaration=JSON.parse(text),targets={};'),
 ('return{name:declaration.name,targets};});\n', 'return{name:declaration.name,targets};};\nregisterFieldExtractorV1("wasm-producer-declaration",wasmProducerDeclarationV1);\n'),
 ('registerFieldExtractorV1("wasm-package-contract",text=>{const manifest=JSON.parse(text);return{name:manifest.name,exports:manifest.exports};});\n', 'const wasmPackageContractV1=text=>{const manifest=JSON.parse(text);return{name:manifest.name,exports:manifest.exports};};\nregisterFieldExtractorV1("wasm-package-contract",wasmPackageContractV1);\n' + WASM),
 (')),wasm=wasmCompileOutputV1(index,path);if(wasm)return wasm;}', '));observeOwnerRegistryAnswerV1(workspaceRoot,["wasm-index"],wasmIndexDigestV1(index));const wasm=wasmCompileOutputV1(index,path);if(wasm)return wasm;}'),
])
