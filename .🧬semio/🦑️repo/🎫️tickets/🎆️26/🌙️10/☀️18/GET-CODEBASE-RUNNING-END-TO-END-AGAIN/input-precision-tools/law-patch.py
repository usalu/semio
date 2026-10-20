#!/usr/bin/env python3
# 🔒 Anchored edits that add the `whole-repository-input` rule to the cache-correctness law, its strict corpus schema and its language-agnostic corpus; every function maps the CURRENT live text to the new text and is idempotent.
import json
MARK = "whole-repository-input"
IMPORT_ANCHOR = 'import type { BreachRecord } from "../../🚨️policy/🔖️breach/🟦️.ts";\n'
RULES_ANCHOR = '"command-network", "uncached-without-reason"] as const;'
LEAVES_ANCHOR = '  const leaves = expandInputs(target.inputs ?? ["default", "^default"], record.namedInputs);\n'
SIGNATURE_ANCHOR = "export function repositoryCacheRecordsV1(repoRoot: string, internals: CachePolicyInternalsV1, tracked?: readonly string[]): CacheRecordV1[] {\n"
PUSH_ANCHOR = "target: internals.targetPolicy(name, structuredClone(base)), namedInputs: names });"
SCHEMA_ANCHOR = '"command-network", "uncached-without-reason"]'

NARROW = '''
const PRECISION_FILE = "🏗️build/📥️inputs/🎯️precision/🔣️.json";
const BROAD_EVIDENCE = new Set(["whole", "assumed"]);

/** 🎯️ Applies the input precision rule of a target the way the inference does: records its decision and, for a narrowing rule, replaces the broad groups it names by the retained inputs (the derived reads themselves are the engine's, not the law's). */
function narrowedTargetV1(document: InputPrecisionDocumentV1 | undefined, project: string, name: string, target: CacheTargetV1): CacheTargetV1 {
  const rule = document && target.cache === true ? selectRuleV1(document, { project, target: name, command: commandOf(target) }) : undefined;
  if (!rule) return target;
  const metadata = { ...target.metadata, semio: { ...target.metadata?.semio, inputPrecision: inputPrecisionDecisionV1(rule) } };
  if (BROAD_EVIDENCE.has(rule.evidence)) return { ...target, metadata };
  return { ...target, metadata, inputs: [...(target.inputs ?? ["default", "^default"]).filter((input) => !(typeof input === "string" && rule.replaces.includes(input))), ...document!.retained] };
}
'''

FINDING = '''  if (leaves.some((leaf) => typeof leaf === "string" && !leaf.startsWith("!") && globSelectsRepositoryV1(projectPath(leaf, root))) && !BROAD_EVIDENCE.has(semio.inputPrecision?.evidence)) found("whole-repository-input", "declares the whole repository as input and no input precision rule bounds it: add a rule to the input precision document that names what the task reads (traced, static or bound), or record why every file is read (whole, assumed)");
'''

def law(text):
    if MARK in text: return text, False
    edits = [
        (IMPORT_ANCHOR, IMPORT_ANCHOR + 'import { globSelectsRepositoryV1, inputPrecisionDecisionV1, readInputPrecisionV1, selectRuleV1, type InputPrecisionDocumentV1 } from "../../🏗️build/📥️inputs/🎯️precision/🟦️.ts";\n'),
        (RULES_ANCHOR, '"command-network", "uncached-without-reason", "whole-repository-input"] as const;'),
        (LEAVES_ANCHOR, LEAVES_ANCHOR + FINDING),
        (SIGNATURE_ANCHOR, NARROW.lstrip("\n") + "\n" + "/** 📚️ The cache records of the tracked manifests; `precision` defaults to the repository's own input precision document. */\nexport function repositoryCacheRecordsV1(repoRoot: string, internals: CachePolicyInternalsV1, tracked?: readonly string[], precision?: InputPrecisionDocumentV1): CacheRecordV1[] {\n"),
        (PUSH_ANCHOR, "target: narrowedTargetV1(document, json.name ?? root, name, internals.targetPolicy(name, structuredClone(base))), namedInputs: names });"),
    ]
    for anchor, replacement in edits:
        if text.count(anchor) != 1: raise SystemExit("law anchor count %d: %s" % (text.count(anchor), anchor[:80]))
        text = text.replace(anchor, replacement)
    doc = '  const document = precision ?? (existsSync(join(repoRoot, library, PRECISION_FILE)) ? readInputPrecisionV1(read(`${library}/${PRECISION_FILE}`)) : undefined);\n'
    marker = "  const injected = new Map<string, string[]>();\n"
    if text.count(marker) != 1: raise SystemExit("law anchor count %d: injected" % text.count(marker))
    return text.replace(marker, doc + marker), True

def schema(text):
    if MARK in text: return text, False
    if text.count(SCHEMA_ANCHOR) != 1: raise SystemExit("schema anchor count %d" % text.count(SCHEMA_ANCHOR))
    return text.replace(SCHEMA_ANCHOR, '"command-network", "uncached-without-reason", "whole-repository-input"]'), True

NAMED = {"default": ["{projectRoot}/**/*", "sharedGlobals"], "sharedGlobals": [{"runtime": "bun --version"}, {"env": "SEMIO_BUILD_MODE"}], "production": ["{projectRoot}/**/*", "!{projectRoot}/**/🧪️tests/**/*", "sharedGlobals"], "testEnv": [{"env": "SEMIO_TEST_LEVEL"}]}
CHECK = {"version": 1, "class": "deterministic", "reason": "check-without-outputs"}
def case(id, root, inputs, expect, precision=None, cache=True):
    semio = {"cache": CHECK, **({"inputPrecision": {"version": 1, **precision}} if precision else {})}
    return {"id": id, "records": [{"id": "w:scan", "root": root, "authoredCache": None, "target": {"cache": cache, "inputs": inputs, "outputs": [], "metadata": {"semio": semio}}, "namedInputs": NAMED}], "expect": [["w:scan", "whole-repository-input"]] if expect else []}

CASES = [
    case("whole-repository-default-at-the-workspace-root", ".", ["default"], True),
    case("whole-repository-explicit-glob-in-a-package", "lib/pkg", ["{workspaceRoot}/**/*"], True),
    case("whole-repository-double-star-in-a-package", "lib/pkg", ["{workspaceRoot}/**"], True),
    case("narrowed-root-target-keeps-only-reads", ".", ["{workspaceRoot}/tools/**/*.ts", "sharedGlobals", "{projectRoot}/📋️project.json"], False, {"rule": "scan", "evidence": "static"}),
    case("narrowing-rule-that-left-the-whole-repository", ".", ["default"], True, {"rule": "scan", "evidence": "static"}),
    case("whole-by-design-is-a-recorded-reason", ".", ["default"], False, {"rule": "census", "evidence": "whole"}),
    case("assumed-reads-are-a-recorded-reason", ".", ["default"], False, {"rule": "pending", "evidence": "assumed"}),
    case("broad-but-not-whole-glob-is-allowed", "lib/pkg", ["{workspaceRoot}/🌎️hub/**/*.ts"], False),
    case("uncached-target-has-no-input-contract", ".", ["default"], False, None, False),
]

def corpus(text):
    if MARK in text: return text, False
    document = json.loads(text)
    for row in document["cases"]:
        if row["id"] == "env-covered-by-the-manifest-at-the-workspace-root":
            row["records"][0]["target"]["metadata"]["semio"]["inputPrecision"] = {"version": 1, "rule": "census", "evidence": "whole"}
    document["cases"].extend(CASES)
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n", True
