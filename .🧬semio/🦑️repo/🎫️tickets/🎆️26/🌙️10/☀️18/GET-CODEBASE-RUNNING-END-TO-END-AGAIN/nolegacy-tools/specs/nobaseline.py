"""🗑️ Batch `nl-nobaseline`: the original-namespace baseline (`🔒️leases/📦️namespace/📒️baseline`) was a cutover module without a production caller ("canonical wiring is separate"); it, its tests and every reference go (descriptor law cases, drain-harness branches, statfs kept-site row, publication type-check list, schema-document index). The allowlist row follows in nl-lang."""
import json

TITLE = "🗑️ Staged batch `nl-nobaseline` (lane nolegacy): the unwired cutover module `🔒️leases/📦️namespace/📒️baseline` and its tests are deleted with every reference (descriptor law cases, drain branches, statfs kept site, publication type-check list, schema-document index)."
P = "🧰️framework/🔨️modules/🏃️process/"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B = P + "🔒️leases/📦️namespace/📒️baseline/"
DELETE = [B + rel for rel in ("🔣️policy.json", "🟦️.ts", "🧪️tests/🔣️.json", "🧪️tests/🚫️limits.json", "🧪️tests/🟦️.ts", "🧬️schema/📥️receipt.json", "🧬️schema/🔣️.json", "🧬️schema/🚫️limits.json", "🧬️schema/🧪️fixture.json")]
D = P + "🔒️leases/📁️physical/"


def corpus(text):
    doc = json.loads(text)
    doc["cases"] = [row for row in doc["cases"] if row["owner"] == "physical"]
    assert len(doc["cases"]) == 6
    return json.dumps(doc, indent=2, ensure_ascii=False) + "\n"


def schema(text):
    doc = json.loads(text)
    cases = doc["properties"]["cases"]
    cases["minItems"] = cases["maxItems"] = 6
    item = cases["items"]["properties"]
    item["owner"] = {"const": "physical"}
    item["failure"]["enum"] = [value for value in item["failure"]["enum"] if value in ("open", "before", "named", "identity", "retain", "none")]
    return json.dumps(doc, indent=2, ensure_ascii=False) + "\n"


def statfs(text):
    doc = json.loads(text)
    kept = doc["sites"]["kept"]
    doc["sites"]["kept"] = [row for row in kept if row["path"] != B + "🟦️.ts"]
    assert len(doc["sites"]["kept"]) == len(kept) - 1
    return json.dumps(doc, indent=2, ensure_ascii=False) + "\n"


def index(text):
    doc = json.loads(text)
    hits = 0
    for table in doc["roots"].values():
        for key in [key for key, value in table.items() if value.startswith(B)]: del table[key]; hits += 1
    assert hits == 2, hits
    return json.dumps(doc, indent=2, ensure_ascii=False) + "\n"


DRAIN = P + "🧭️routing/📥️invocation/🏃️process/📥️issuance/🧪️tests/📥️drain/🟦️.ts"
EDITS = {
    D + "🧪️tests/🧫️fixtures/📂️descriptors.json": [corpus],
    D + "🧬️schema/📂️descriptors.json": [schema],
    D + "🧪️tests/📂️descriptors/🟦️.ts": [
        ('import policy from"../../../📦️namespace/📒️baseline/🔣️policy.json";\n', "", 1),
        ('test("physical descriptor transfer and baseline digest close every refused opened handle",()=>{', 'test("physical descriptor transfer closes every refused opened handle",()=>{', 1),
        (',sources={physical:new URL("../../🟦️.ts",import.meta.url),baseline:new URL("../../../📦️namespace/📒️baseline/🟦️.ts",import.meta.url)};', ',source=new URL("../../🟦️.ts",import.meta.url),name="retainPhysicalDescriptorV1";', 1),
        ('  const source=sources[row.owner as keyof typeof sources],tree=ts.createSourceFile(source.pathname,readFileSync(source,"utf8"),ts.ScriptTarget.Latest,true),name=row.owner==="physical"?"retainPhysicalDescriptorV1":"digest",node=', '  const tree=ts.createSourceFile(source.pathname,readFileSync(source,"utf8"),ts.ScriptTarget.Latest,true),node=', 1),
        (",Buffer:{alloc:(bytes:number)=>{fault(\"buffer\");return Buffer.alloc(bytes);}},policy,PhysicalLockErrorV1};", ",Buffer:{alloc:(bytes:number)=>{fault(\"buffer\");return Buffer.alloc(bytes);}},PhysicalLockErrorV1};", 1),
        ('  try{const result=implementation("fixture:descriptor",held);if(row.owner==="baseline")value=result;}catch(cause){error=(cause as Error).message;}', '  try{implementation("fixture:descriptor",held);}catch(cause){error=(cause as Error).message;}', 1),
        ("let value:string|null=null,error:string|null=null;", "let error:string|null=null;", 1),
        ("retained:held.length,error,digest:value};", "retained:held.length,error,digest:null};", 1),
        ('  if(row.owner==="physical"){const owner=tree.statements.find((node:any)=>ts.isFunctionDeclaration(node)&&node.name?.text==="createPosixPhysicalLockBackendV1");expect(owner.getText(tree)).toContain("retainPhysicalDescriptorV1(path,held)");}', '  const owner=tree.statements.find((node:any)=>ts.isFunctionDeclaration(node)&&node.name?.text==="createPosixPhysicalLockBackendV1");expect(owner.getText(tree)).toContain("retainPhysicalDescriptorV1(path,held)");', 1),
    ],
    D + "🧪️tests/📂️descriptors/🥒️.feature": [("  Scenario: Immutable baseline digest fails after opening its source\n    When validation, hashing, buffer allocation, reading, or final identity fails\n    Then the original descriptor closes once\n    And successful digest bytes remain unchanged\n", "", 1)],
    DRAIN: [
        (' }else if(["issuer-baseline-count-drain","issuer-baseline-keybytes-drain"].includes(mode)){\n  const{proveOriginalBaselineLimitV1}=await import("../../../../../../🔒️leases/📦️namespace/📒️baseline/🧪️tests/🟦️.ts");await proveOriginalBaselineLimitV1(mode,options);\n }else if(mode==="issuer-baseline-funding-drain"){\n  const{proveOriginalBaselineFundingV1}=await import("../../../../../../🔒️leases/📦️namespace/📒️baseline/🧪️tests/🟦️.ts");await proveOriginalBaselineFundingV1(options);\n }else if(mode==="issuer-baseline-drain"){\n  const{proveOriginalResourceBaselineV1}=await import("../../../../../../🔒️leases/📦️namespace/📒️baseline/🧪️tests/🟦️.ts");await proveOriginalResourceBaselineV1(options);\n }else if(mode==="issuer-drain-failure"){', ' }else if(mode==="issuer-drain-failure"){', 1),
        ('  if(["issuer-baseline-count-drain","issuer-baseline-keybytes-drain"].includes(mode)){\n  const{proveOriginalBaselineLimitV1}=await import("../../../../../../🔒️leases/📦️namespace/📒️baseline/🧪️tests/🟦️.ts");await proveOriginalBaselineLimitV1(mode,options);\n }else if(mode==="issuer-baseline-drain"){\n  const{proveOriginalResourceBaselineV1}=await import("../../../../../../🔒️leases/📦️namespace/📒️baseline/🧪️tests/🟦️.ts");await proveOriginalResourceBaselineV1(options);\n }else if(mode==="issuer-drain-failure")result.detachedFailureRejectsOriginal=String(failure).includes("Detached accepted failure");\n', "", 1),
    ],
    P + "📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📐️statfs/🧪️tests/🆓️unknown/🧫️fixtures/🔣️.json": [statfs],
    L + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts": [('join(lease,"📦️namespace/📒️baseline/🟦️.ts"),', "", 1)],
    "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json": [index],
}
TESTS = [
    "./" + D + "🧪️tests/📂️descriptors/🟦️.ts",
    "./" + P + "📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📐️statfs/🧪️tests/🆓️unknown/🟦️.ts",
]
