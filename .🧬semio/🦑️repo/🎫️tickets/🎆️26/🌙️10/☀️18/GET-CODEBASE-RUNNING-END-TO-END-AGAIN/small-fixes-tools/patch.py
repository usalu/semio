import sys, shutil, pathlib
L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
R = "/Users/ueli/Documents/semio/🧰️framework"
C = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
BK = pathlib.Path(C) / "🗑️generated/small-fixes/backup"
INF = L + "/🏗️build/🕸️graph/📥️inference"

def edit(path, pairs, tag):
    p = pathlib.Path(path)
    text = p.read_text(encoding="utf-8")
    shutil.copy2(p, BK / (tag + "-" + p.name))
    for old, new in pairs:
        n = text.count(old)
        if n != 1:
            sys.exit(f"ANCHOR COUNT {n} in {path}: {old[:90]!r}")
        text = text.replace(old, new)
    p.write_text(text, encoding="utf-8")
    print("patched", path, len(pairs))

# E1 inference declarations
d = INF + "/🟨️.d.mts"
decl = """/** 🧱️ Classifies the loaded implementation rows by the declared stamp of every inferrer: data no inferrer covers stays a content fact, the code of an inferrer is named by its stamp. */
export function classifyImplementationRowsV1(rows:readonly ImplementationRowV1[],document?:InferenceProtocolV1):ImplementationRowV1[];
/** 🚧️ Names the executable files of the loaded implementation that no inferrer and no infrastructure fragment of the protocol claims. */
export function unclassifiedImplementationCodeV1(rows:readonly ImplementationRowV1[],document?:InferenceProtocolV1):string[];
/** 📜️ The declared inference protocol of the loaded revision. */
export function inferenceProtocolV1():InferenceProtocolV1;
"""
txt = pathlib.Path(d).read_text(encoding="utf-8")
edit(d, [
    ('import type{ScriptInvocation}from', 'import type{ImplementationRowV1,InferenceProtocolV1}from"./🔖️stamps/🟨️.mjs";\nimport type{ScriptInvocation}from'),
    ('export const loadedImplementationRowsV1:', decl + 'export const loadedImplementationRowsV1:'),
], "inference-dmts")

# E2 stamps wire signature
edit(INF + "/🔖️stamps/🟨️.d.mts", [("export function wire(registry?: unknown): void;", "export function wire(): void;")], "stamps-dmts")

# E3 format-stamps optional options
edit(L + "/🏗️build/🧷️format-stamps/🟨️.d.mts", [("export function createFormatStampsV1(options:{", "export function createFormatStampsV1(options?:{")], "format-stamps-dmts")

# E4 output-stamps test typing
edit(INF + "/🧪️tests/🔖️output-stamps/🟦️.ts", [('const moved={...digests,"target-policy/policy-build"', 'const moved:Record<string,string>={...digests,"target-policy/policy-build"')], "output-stamps-test")

# R1 build aggregator
edit(L + "/🏗️build/🧪️tests/🟦️.ts", [(
 'import "../🕸️graph/📥️inference/🧪️tests/🔌️providers/🟦️.ts";\n',
 'import "../🕸️graph/📥️inference/🧪️tests/🔌️providers/🟦️.ts";\n'
 'import "../🕸️graph/📥️inference/🔖️stamps/🧪️tests/🟦️.ts";\n'
 'import "../🕸️graph/📥️inference/🧪️tests/🔖️output-stamps/🟦️.ts";\n'
 'import "../🧷️format-stamps/🧪️tests/🟦️.ts";\n'
 'import "../📥️source/🔏️closure/🧪️tests/🧭️sites/🟦️.ts";\n'
 'import "../../⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🟦️.ts";\n'
 'import "../../⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🚪️ingress/🟦️.ts";\n'
 'import "../../⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🛡️followup/🟦️.ts";\n')], "build-aggregator")

# R2 authorizers aggregator
edit(L + "/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🟦️.ts", [('import "./🧭️closure-law/🟦️.ts";\n', 'import "./🧭️closure-law/🟦️.ts";\nimport "../🧭️sites/🧪️tests/🟦️.ts";\n')], "authorizers-aggregator")

# R3 cache lane map (generation tests are lanes, like their javascript siblings)
edit(L + "/⚡️caching/🧪️tests/🔬️script-commands/🟦️.ts", [(
 '"javascript-concurrent-generations":"🚀️bootstrap/📦️dependencies/🚦️concurrent/🧪️tests/🟦️.ts"',
 '"javascript-concurrent-generations":"🚀️bootstrap/📦️dependencies/🚦️concurrent/🧪️tests/🟦️.ts","javascript-generation":"🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🟦️.ts","javascript-generation-ingress":"🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🚪️ingress/🟦️.ts","javascript-generation-followup":"🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🛡️followup/🟦️.ts"')], "cache-script-commands")

# R4 process suite map
edit(R + "/🔨️modules/🏃️process/📜️script.ts", [(
 '"resource-lease-admission": {source:"🔒️leases/🚪️admission/🧪️tests/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick},',
 '"resource-lease-admission": {source:"🔒️leases/🚪️admission/🧪️tests/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick},\n      "resource-lease-writer": {source:"🔒️leases/🧪️tests/🎟️writer/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick},')], "process-script")

# project.json targets: copy sibling blocks with renamed lanes
def copy_block(path, start_marker, names, tag, single_line):
    p = pathlib.Path(path)
    lines = p.read_text(encoding="utf-8").split("\n")
    shutil.copy2(p, BK / (tag + "-" + p.name))
    idx = [i for i, l in enumerate(lines) if l.startswith(start_marker)]
    if len(idx) != 1:
        sys.exit(f"START COUNT {len(idx)} in {path}: {start_marker}")
    i = idx[0]
    if single_line:
        block = [lines[i]]
        end = i
    else:
        end = next(j for j in range(i + 1, len(lines)) if lines[j] in ("    },", "    }"))
        block = lines[i:end + 1]
    new = []
    for name in names:
        new += [b.replace(start_marker.split('"')[1], name) .replace(start_marker.split('"')[1].replace("test-", "test ", 1), name.replace("test-", "test ", 1)) for b in block]
    lines[end + 1:end + 1] = new
    p.write_text("\n".join(lines), encoding="utf-8")
    print("project", path, names)

copy_block(L + "/⚡️caching/📋️project.json", '    "test-javascript-concurrent-generations":', ["test-javascript-generation", "test-javascript-generation-ingress", "test-javascript-generation-followup"], "caching-project", True)
copy_block(R + "/🔨️modules/🏃️process/📋️project.json", '    "test-resource-lease-admission": {', ["test-resource-lease-writer"], "process-project", False)
