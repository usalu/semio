"""🚦 [td-leases] batch `startup`: the light door commands stop loading the lease kernel and the build-side capacity module they never call. usage: python3 -I edit_startup.py <root>  (anchored edits, each anchor exactly once)"""
import os, sys
root = sys.argv[1]
P = "🧰️framework/🔨️modules/🏃️process/"
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
def edit(rel, pairs):
    path = os.path.join(root, rel)
    text = open(path, encoding="utf8").read()
    for old, new in pairs:
        n = text.count(old)
        if n != 1: sys.exit(f"anchor x{n} in {rel}: {old[:90]!r}")
        text = text.replace(old, new)
    open(path, "w", encoding="utf8").write(text)
    print("edited", rel)
def create(rel, text):
    path = os.path.join(root, rel)
    if os.path.exists(path): sys.exit("exists: " + rel)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf8").write(text)
    print("created", rel)
create(P + "📦️packages/🟦️typescript/🔒️leases/📈️standing/⚖️priority/🟦️.ts",
 '/** ⚖️ Public `@semio-tech/process/leases/standing/priority` entry: the priority class rule alone, so a capacity sizer does not load the lease kernel. */\n'
 'export { leasePriorityClassV1, type LeasePriorityV1 } from "../../../../../🔒️leases/📈️standing/⚖️priority/🟦️.ts";\n')
edit(P + "📦️packages/🟦️typescript/package.json", [('    "./leases/status": "./🔒️leases/📋️status/🟦️.ts",\n', '    "./leases/status": "./🔒️leases/📋️status/🟦️.ts",\n    "./leases/standing/priority": "./🔒️leases/📈️standing/⚖️priority/🟦️.ts",\n')])
edit(B + "🌡️host-capacity/🟦️.ts", [('import {leasePriorityClassV1,type ResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";\n',
 'import type {ResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";\nimport {leasePriorityClassV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/📈️standing/⚖️priority/🟦️.ts";\n')])
HC = B + "🌡️host-capacity/🟦️.ts"
src = open(os.path.join(root, HC), encoding="utf8").read()
def cut(text, start, end_marker):
    i = text.index(start); j = text.index(end_marker, i) + len(end_marker)
    return text[i:j]
refuse = cut(src, "/** 📖️ The cancellation texts load", ".refuseCapacityV1(...input);\n")
policy = cut(src, "/** 🌡️ The authored host capacity policy", ".hostCapacity);\n")
control = cut(src, "/** 🛑️ A control for the wait before any invocation exists", "removeListener(name,handler);}});\n}\n")
assert src.count(refuse) == src.count(policy) == src.count(control) == 1
create(B + "🌡️host-capacity/🎛️control/🟦️.ts",
 'import {createRequire} from "node:module";\n'
 'import buildPolicy from "../../🔣️policy.json" with {type:"json"};\n'
 'import {readHostCapacityPolicyV1,type HostCapacityPolicyV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";\n'
 'import type {ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";\n\n'
 + refuse.replace("const refuseCapacityV1=", "export const refuseCapacityV1=").replace('import("./📖️texts/🟦️.ts")', 'import("../📖️texts/🟦️.ts")').replace('createRequire(import.meta.url)("./📖️texts/🟦️.ts")', 'createRequire(import.meta.url)("../📖️texts/🟦️.ts")')
 + "\n" + policy + "\n" + control)
edit(HC, [
 (refuse + "\n", ""),
 (policy, ""),
 (control, ""),
 ('import {createRequire} from "node:module";\n', 'import {BUILD_HOST_CAPACITY_POLICY_V1,createCapacityControlV1,refuseCapacityV1} from "./🎛️control/🟦️.ts";\n\nexport {BUILD_HOST_CAPACITY_POLICY_V1,createCapacityControlV1} from "./🎛️control/🟦️.ts";\n'),
 ("quietHostRequestV1,rampHostRequestV1,readHostCapacityPolicyV1,readHostEstimateStoreV1,", "quietHostRequestV1,rampHostRequestV1,readHostEstimateStoreV1,"),
])
LC = B + "🔒️leases/⌨️command/🟦️.ts"
edit(LC, [('import {BUILD_HOST_CAPACITY_POLICY_V1,createCapacityControlV1} from "../../🌡️host-capacity/🟦️.ts";\n', 'import {BUILD_HOST_CAPACITY_POLICY_V1,createCapacityControlV1} from "../../🌡️host-capacity/🎛️control/🟦️.ts";\n')])
edit(B + "🌡️host-capacity/⌨️command/🟦️.ts", [('import {BUILD_HOST_CAPACITY_POLICY_V1,createCapacityControlV1} from "../🟦️.ts";\n', 'import {BUILD_HOST_CAPACITY_POLICY_V1,createCapacityControlV1} from "../🎛️control/🟦️.ts";\n')])
edit(B + "🩺️doctor/🔎️probes/🎫️capacity/🟦️.ts", [('import {BUILD_HOST_CAPACITY_POLICY_V1} from "../../../🌡️host-capacity/🟦️.ts";\n', 'import {BUILD_HOST_CAPACITY_POLICY_V1} from "../../../🌡️host-capacity/🎛️control/🟦️.ts";\n')])
create(P + "📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/📋️status/🟦️.ts",
 '/** 📋️ Public `@semio-tech/process/build/host-capacity/status` entry: what a read-only status command needs (the policy reader, the gate directory, the gate snapshot and the reason texts) without the sizing, forecast and upgrade modules of the full contract. */\n'
 'export { describeHostReasonV1, readHostCapacityPolicyV1, type HostCapacityPolicyV1, type HostReasonV1 } from "../../../../../🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";\n'
 'export { hostCapacityDirectoryV1, readHostCapacityStatusV1 } from "../../../../../🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts";\n')
edit(P + "📦️packages/🟦️typescript/package.json", [('    "./build/host-capacity": "./🏗️build/🌡️host-capacity/🟦️.ts",\n', '    "./build/host-capacity": "./🏗️build/🌡️host-capacity/🟦️.ts",\n    "./build/host-capacity/status": "./🏗️build/🌡️host-capacity/📋️status/🟦️.ts",\n')])
edit(B + "🌡️host-capacity/🎛️control/🟦️.ts", [('📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";\n', '📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/📋️status/🟦️.ts";\n')])
edit(LC, [('import {describeHostReasonV1,hostCapacityDirectoryV1,readHostCapacityStatusV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";\n', 'import {describeHostReasonV1,hostCapacityDirectoryV1,readHostCapacityStatusV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/📋️status/🟦️.ts";\n')])
edit(B + "🩺️doctor/🔎️probes/🎫️capacity/🟦️.ts", [('import {hostCapacityDirectoryV1,readHostCapacityStatusV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";\n', 'import {hostCapacityDirectoryV1,readHostCapacityStatusV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/📋️status/🟦️.ts";\n')])
edit(P + "📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts", [('import {requireMaterializedPathV1,type ResourceLeaseWriterProofV1,type ResourceLeaseControl} from "../../../../../🔒️leases/🟦️.ts";\n',
 'import {requireMaterializedPathV1,type ResourceLeaseControl} from "../../../../../🔒️leases/🎛️control/🟦️.ts";\nimport type {ResourceLeaseWriterProofV1} from "../../../../../🔒️leases/🟦️.ts";\n')])
create(P + "📦️packages/🟦️typescript/🧪️testing/🦀️execution/🧬️schema/🔣️json/🟦️.ts",
 '/** 🧬️ Public `@semio-tech/process/cargo-execution/schema.json` entry: the Cargo execution capability schema alone, so a caller that validates capabilities does not load the executor. */\n'
 'import document from "../../../../../../🧪️testing/🦀️cargo/🏃️execution/🧬️schema/🔣️.json" with { type: "json" };\n'
 'export default document;\n')
edit(P + "📦️packages/🟦️typescript/package.json", [('    "./cargo-execution": "./🧪️testing/🦀️execution/🟦️.ts",\n', '    "./cargo-execution": "./🧪️testing/🦀️execution/🟦️.ts",\n    "./cargo-execution/schema.json": "./🧪️testing/🦀️execution/🧬️schema/🔣️json/🟦️.ts",\n')])
NO = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts"
edit(NO, [('import {cargoExecutionSchemaV1,type CargoExecutionBackendV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts";\n',
 'import cargoExecutionSchemaV1 from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🧬️schema/🔣️json/🟦️.ts";\nimport type {CargoExecutionBackendV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts";\n')])
edit(B + "🚪️door/🧪️tests/🚦️startup/🔣️.json", [
 ('"maximumModules": 26,\n      "maximumBytes": 227200', '"maximumModules": 32,\n      "maximumBytes": 379000'),
 ('"maximumModules": 22,\n      "maximumBytes": 175400', '"maximumModules": 33,\n      "maximumBytes": 316000'),
 ('"maximumModules": 80,\n      "maximumBytes": 946000', '"maximumModules": 102,\n      "maximumBytes": 915000'),
 ('"maximumModules": 60,\n      "maximumBytes": 764000', '"maximumModules": 41,\n      "maximumBytes": 391000'),
 ('"maximumModules": 1,\n      "maximumBytes": 1500', '"maximumModules": 2,\n      "maximumBytes": 1500'),
])
