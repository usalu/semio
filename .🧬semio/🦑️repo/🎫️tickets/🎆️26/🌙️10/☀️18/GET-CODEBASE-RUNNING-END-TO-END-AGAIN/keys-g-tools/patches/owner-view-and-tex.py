"""python3 owner-view-and-tex.py [--check] : the keys-g inference batch.
1. policy `ownerViews` (the framework owner keeps `🛍️products` private to its own tasks) + schema, 2. policy toolchain `tex` activated by its provisioning targets + schema `activatedBy`,
3. inference: owner globs split into the shared part and `ownerPrivate`, toolchain activation, 4. assembly: own cached targets add `ownerPrivate`.
Anchored, idempotent, all-or-nothing (every file is re-read right before the write; any anchor miss writes nothing)."""
import json, sys
ROOT = "/Users/ueli/Documents/semio/"
L = ROOT + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
FILES = {
    "policy": L + "⚡️caching/🔣️policy.json",
    "schema": L + "⚡️caching/🧬️schema/🔣️.json",
    "inputs": L + "🏗️build/📥️inputs/🟨️.mjs",
    "assembly": L + "🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs",
    "inputsTest": L + "🏗️build/📥️inputs/🧪️tests/🟦️.ts",
}
text = {k: open(v, encoding="utf-8").read() for k, v in FILES.items()}

def sub(key, anchor, new, marker):
    if marker in text[key]:
        return
    if text[key].count(anchor) != 1:
        sys.exit(f"{key}: anchor not unique ({text[key].count(anchor)}): {anchor[:80]}")
    text[key] = text[key].replace(anchor, new)

# 1+2 policy (JSON edited structurally; the file keeps its two-space layout)
policy = json.loads(text["policy"])
if "ownerViews" not in policy:
    items = list(policy.items())
    at = [k for k, _ in items].index("generatedDirectories")
    items.insert(at + 1, ("ownerViews", [{"owner": "🧰️framework", "private": ["🛍️products"]}]))
    policy = dict(items)
tool = "🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/"
if "tex" not in policy["toolchains"]:
    policy["toolchains"]["tex"] = {"files": [tool + "🔧️toolchain/🔣️.json", tool + "📚️bundle/🔒️dependencies.json"], "environment": [], "commands": [], "activatedBy": ["deps-tectonic", "deps-tex"]}
text["policy"] = json.dumps(policy, ensure_ascii=False, indent=2) + "\n"

OWNER_VIEWS_SCHEMA = """    "ownerViews": {
      "description": "Owner trees whose subtrees stay private to the tasks of the owning project: its dependents key the shared part of the owner glob only, so an edit inside a private subtree (the products under the framework owner) never re-keys a dependent. A framework module cannot depend on a product (dependency-direction law); a task of the owner that reads a private subtree adds the named input `ownerPrivate`.",
      "type": "array",
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": ["owner", "private"],
        "properties": {
          "owner": { "type": "string" },
          "private": { "type": "array", "uniqueItems": true, "items": { "type": "string" } }
        }
      }
    },
"""
sub("schema", '    "toolchains": {\n      "type": "object",\n', OWNER_VIEWS_SCHEMA + '    "toolchains": {\n      "type": "object",\n', '"ownerViews"')
sub("schema", """          "commands": {
            "type": "array",
            "items": {
              "type": "string"
            }
          }
        }
      }
    },
    "targetDefaults": {""", """          "commands": {
            "type": "array",
            "items": {
              "type": "string"
            }
          },
          "activatedBy": {
            "description": "Names of provisioning targets (`deps-tectonic`): a project whose targets depend on one of them hashes this toolchain.",
            "type": "array",
            "uniqueItems": true,
            "items": {
              "type": "string"
            }
          }
        }
      }
    },
    "targetDefaults": {""", '"activatedBy"')

# 3 inference
sub("inputs", 'import { adoptVerifiedBlockV1, observedFs } from "../👁️observation/🟨️.mjs";\n',
    'import { adoptVerifiedBlockV1, observedFs } from "../👁️observation/🟨️.mjs";\nimport { ownerOfProjectV1, ownerTreeGlobsV1, ownerViewV1 } from "./🌳️owner-view/🟨️.mjs";\nimport { toolchainsActivatedByV1 } from "./🪛️toolchain-activation/🟨️.mjs";\n', "toolchainsActivatedByV1 }")
sub("inputs", '  if (!workspaceProject && existsSync(join(workspaceRoot, root, "CMakeLists.txt"))) tools.push("cmake");\n',
    '  if (!workspaceProject && existsSync(join(workspaceRoot, root, "CMakeLists.txt"))) tools.push("cmake");\n  for (const tool of toolchainsActivatedByV1(json.targets, POLICY.toolchains)) if (!tools.includes(tool)) tools.push(tool);\n', "toolchainsActivatedByV1(json.targets")
sub("inputs", '  const deliveryOffsets = TAXONOMY.testDeliveryScopeDirectoryNames.map((name) => root.indexOf(`/${name}/`)).filter((index) => index >= 0);\n  const owner = deliveryOffsets.length > 0 ? root.slice(0, Math.min(...deliveryOffsets)) : root;\n',
    '  const owner = ownerOfProjectV1(root, TAXONOMY.testDeliveryScopeDirectoryNames), ownerView = owner === root ? undefined : ownerViewV1(owner, POLICY.ownerViews);\n  let ownerPrivate = [];\n', "ownerOfProjectV1(root, TAXONOMY")
old = '    inputs.push(...(native ? [...native, `{workspaceRoot}/${owner}/**/*.{json,semio,wit,wgsl,glsl,h,c,cpp,ts,tsx,js,mjs,cjs}`] : [`{workspaceRoot}/${owner}/**/*.${extensions}`]));\n'
new = '''    const ownerExtensions = native ? "{json,semio,wit,wgsl,glsl,h,c,cpp,ts,tsx,js,mjs,cjs}" : extensions;
    let ownerTree = [`{workspaceRoot}/${owner}/**/*.${ownerExtensions}`];
    if (ownerView) {
      const split = ownerTreeGlobsV1(owner, ownerExtensions, ownerView.private, readdirSync(join(workspaceRoot, owner), { withFileTypes: true }).filter((entry) => !POLICY.generatedDirectories.includes(entry.name)).map((entry) => ({ name: entry.name, directory: entry.isDirectory() })));
      ownerTree = split.shared;
      ownerPrivate = split.private;
    }
    inputs.push(...(native ? [...native, ...ownerTree] : ownerTree));
'''
sub("inputs", old, new, "ownerExtensions")
sub("inputs", "  const groups = (sourceContracts) => ({ ...declarations, ...sourceContracts, ...(cargoToolchain ? { cargoToolchain } : {}), default:",
    "  const groups = (sourceContracts) => ({ ...declarations, ...sourceContracts, ...(cargoToolchain ? { cargoToolchain } : {}), ...(ownerPrivate.length ? { ownerPrivate } : {}), default:", "...(ownerPrivate.length ? { ownerPrivate }")

# 4 assembly
sub("assembly", 'import { deriveInputsV1, inputPrecisionDecisionV1, inputPrecisionLanguageV1, inputPrecisionTextV1, readInputPrecisionV1, selectRuleV1 } from "../../../📥️inputs/🎯️precision/🟦️.ts";\n',
    'import { deriveInputsV1, inputPrecisionDecisionV1, inputPrecisionLanguageV1, inputPrecisionTextV1, readInputPrecisionV1, selectRuleV1 } from "../../../📥️inputs/🎯️precision/🟦️.ts";\nimport { ownerOfProjectV1, ownerViewV1 } from "../../../📥️inputs/🌳️owner-view/🟨️.mjs";\n', "ownerOfProjectV1, ownerViewV1 }")
sub("assembly", "    if (policy.cache && policy.inputs) policy = await preciseInputsV1(json.name, name, inherited.options?.command ?? inherited.command, policy, root, workspaceRoot, facts, scripts, internCommandSources);\n",
    "    if (policy.cache && policy.inputs) policy = await preciseInputsV1(json.name, name, inherited.options?.command ?? inherited.command, policy, root, workspaceRoot, facts, scripts, internCommandSources);\n    if (ownerPrivateView && policy.cache && Array.isArray(policy.inputs) && policy.metadata?.semio?.inputPrecision === undefined && policy.inputs.some((input) => OWNER_PRIVATE_USERS_V1.has(input))) policy = { ...policy, inputs: [...policy.inputs, \"ownerPrivate\"] };\n", "ownerPrivateView &&")
sub("assembly", "  const nativeProject = existsSync(join(workspaceRoot, nativeRoot, \"Cargo.toml\"));\n  const artifactTypeScript = json.tags?.includes(\"role:artifact\")",
    "  const nativeProject = existsSync(join(workspaceRoot, nativeRoot, \"Cargo.toml\"));\n  const ownerPrivateView = typeof ownerViewV1 === \"function\" && root !== \".\" && TAXONOMY?.testDeliveryScopeDirectoryNames !== undefined && ownerOfProjectV1(root, TAXONOMY.testDeliveryScopeDirectoryNames) !== root && ownerViewV1(ownerOfProjectV1(root, TAXONOMY.testDeliveryScopeDirectoryNames), POLICY?.ownerViews) !== undefined;\n  const artifactTypeScript = json.tags?.includes(\"role:artifact\")", "const ownerPrivateView")
sub("assembly", "const PRECISION_FILES = [", "/** 🌳️ The named inputs whose owner glob a private owner view splits: a task of the owning project that uses one of them also keys the private subtrees. */\nconst OWNER_PRIVATE_USERS_V1 = new Set([\"default\", \"production\", \"nativeTestSources\"]);\nconst PRECISION_FILES = [", "OWNER_PRIVATE_USERS_V1 =")

# 5 the inference declaration is evaluated by its text in the inputs test: the new helpers join its context
sub("inputsTest", 'import * as inputModule from "../🟨️.mjs";\n', 'import * as inputModule from "../🟨️.mjs";\nimport {ownerOfProjectV1,ownerTreeGlobsV1,ownerViewV1} from "../🌳️owner-view/🟨️.mjs";\nimport {toolchainsActivatedByV1} from "../🪛️toolchain-activation/🟨️.mjs";\n', "ownerTreeGlobsV1} from")
sub("inputsTest", "cargoManifestInputV1:inputModule.cargoManifestInputV1,", "cargoManifestInputV1:inputModule.cargoManifestInputV1,ownerOfProjectV1,ownerViewV1,ownerTreeGlobsV1,toolchainsActivatedByV1,", "ownerOfProjectV1,ownerViewV1,ownerTreeGlobsV1,toolchainsActivatedByV1,")

if "--check" in sys.argv:
    print("anchors ok")
    sys.exit(0)
for key, path in FILES.items():
    now = open(path, encoding="utf-8").read()
    if now != open(path, encoding="utf-8").read():
        sys.exit("moved")
for key, path in FILES.items():
    open(path, "w", encoding="utf-8").write(text[key])
print("applied", ", ".join(FILES))
