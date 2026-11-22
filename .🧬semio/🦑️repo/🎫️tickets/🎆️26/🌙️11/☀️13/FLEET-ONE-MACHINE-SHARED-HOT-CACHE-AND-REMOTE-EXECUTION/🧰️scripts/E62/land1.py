import hashlib, sys
B = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"

def edit(path, pairs):
    text = open(B + path, encoding="utf8").read()
    before = hashlib.sha256(text.encode()).hexdigest()[:8]
    for old, new in pairs:
        if text.count(old) != 1:
            raise SystemExit(f"{path}: anchor found {text.count(old)} times: {old[:80]}")
        text = text.replace(old, new)
    open(B + path, "w", encoding="utf8").write(text)
    print(path.split("/")[-2], before, "->", hashlib.sha256(text.encode()).hexdigest()[:8])

edit("🕸️graph/🚪️admission/🟦️.ts", [(
'''/** 🔑️ Names the complete immutable selection and registry declaration independently of whole graph state. */
export function buildGraphScopeKeyV1(selection:BuildGraphSelectionV1,registry:BuildOwnerRegistryV1):string{
 return createHash("sha256").update(JSON.stringify([readBuildGraphSelectionV1(selection),readBuildOwnerRegistryV1(registry)])).digest("hex");
}''',
'''/** 🔑️ Names the complete immutable selection independently of whole graph state and of the owner registry: what a read asked of the registry travels in the records it derived, so an edit of one registry row never gives a scope a new identity.
 * @see ../🧾️registry-reads/🟨️.mjs */
export function buildGraphScopeKeyV1(selection:BuildGraphSelectionV1):string{
 return createHash("sha256").update(JSON.stringify(readBuildGraphSelectionV1(selection))).digest("hex");
}''')])

edit("🕸️graph/📥️load/🟦️.ts", [
('import {readBuildOwnerRegistryV1,readBuildGraphSelectionV1,closeBuildOwnerAdmissionV1,buildGraphScopeKeyV1,',
 'import {ownerRegistryDigestV1,askOwnerRegistryV1} from "../🧾️registry-reads/🟨️.mjs";\nimport {readBuildOwnerRegistryV1,readBuildGraphSelectionV1,closeBuildOwnerAdmissionV1,buildGraphScopeKeyV1,'),
('  if(!registry)throw Error("Original canonical graph owner registry required");\n',
 '  if(!registry)throw Error("Original canonical graph owner registry required");\n  const registryDigest=ownerRegistryDigestV1(registry);\n'),
('if(buildGraphScopeKeyV1(options.selection,readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))))!==scope)return ownerRegistryInvalidationV1;',
 'if(ownerRegistryDigestV1(readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))))!==registryDigest)return ownerRegistryInvalidationV1;'),
('const current=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));if(buildGraphScopeKeyV1(options.selection,current)!==scope)return ownerRegistryInvalidationV1;',
 'const current=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));if(ownerRegistryDigestV1(current)!==registryDigest)return ownerRegistryInvalidationV1;askOwnerRegistryV1(root,registry!,["digest"]);'),
('if(JSON.stringify(admittedRegistry)!==JSON.stringify(registry))return ownerRegistryInvalidationV1;\n',
 'if(JSON.stringify(admittedRegistry)!==JSON.stringify(registry))return ownerRegistryInvalidationV1;\n    askOwnerRegistryV1(root,registry!,["digest"]);\n'),
('scope=options.selection.kind!=="all"?buildGraphScopeKeyV1(options.selection,registry):undefined;',
 'scope=options.selection.kind!=="all"?buildGraphScopeKeyV1(options.selection):undefined;'),
])
