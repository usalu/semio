# 🪝 Anchored edits of the cache-correctness law: the rule `whole-workspace-rust-key`, its corpus cases and the corpus schema.
import json
from patchlib import chain

IMPORT_OLD = 'import { globSelectsRepositoryV1, inputPrecisionDecisionV1, readInputPrecisionV1, selectRuleV1, type InputPrecisionDocumentV1 } from "../../🏗️build/📥️inputs/🎯️precision/🟦️.ts";'
IMPORT_NEW = IMPORT_OLD + '''
import { cargoResolutionLanguageV1, cargoResolutionTextV1, readCargoResolutionPolicyV1, workspaceKeyViolationsV1 } from "../../🏗️build/📥️inputs/🦀️resolution/🟦️.ts";
import resolutionDocument from "../../🏗️build/📥️inputs/🦀️resolution/🔣️.json" with { type: "json" };

const RESOLUTION = readCargoResolutionPolicyV1(resolutionDocument);'''
RULES_OLD = '"uncached-without-reason", "whole-repository-input"] as const;'
RULES_NEW = '"uncached-without-reason", "whole-repository-input", "whole-workspace-rust-key"] as const;'
CHECK_OLD = '''  if (Array.isArray(target.inputs) && target.inputs.length === 0 && target.executor !== "repo:noop")'''
CHECK_NEW = '''  const wholeKeys = workspaceKeyViolationsV1(RESOLUTION, leaves, commandOf(target), id);
  if (wholeKeys.length && !BROAD_EVIDENCE.has(semio.inputPrecision?.evidence)) found("whole-workspace-rust-key", cargoResolutionTextV1(RESOLUTION, "workspaceKey", cargoResolutionLanguageV1(RESOLUTION, process.env), { subject: id, keys: wholeKeys.join(", ") }));
''' + CHECK_OLD
STALE_OLD = "/** 🧮️ Every finding of every rule over a set of resolved targets. */"
STALE_NEW = '''/** 🪜️ The exempt targets that no longer declare a whole-workspace Rust key (or no longer exist): the exemption list is a ratchet, so an owner who derives a narrower key must delete the entry. */
export function staleRustKeyExemptionsV1(records: readonly CacheRecordV1[]): string[] {
  const open = { ...RESOLUTION, workspaceKeys: { ...RESOLUTION.workspaceKeys, exemptTargets: [] } }, byId = new Map(records.map((record) => [record.id, record]));
  return RESOLUTION.workspaceKeys.exemptTargets.flatMap((row) => row.targets).filter((id) => {
    const record = byId.get(id);
    return !record || record.target.cache !== true || workspaceKeyViolationsV1(open, expandInputs(record.target.inputs ?? ["default", "^default"], record.namedInputs), commandOf(record.target)).length === 0;
  }).sort();
}

''' + STALE_OLD
def law(text):
    return chain(text, [("law-import", IMPORT_OLD, IMPORT_NEW), ("law-rules", RULES_OLD, RULES_NEW), ("law-check", CHECK_OLD, CHECK_NEW), ("law-stale", STALE_OLD, STALE_NEW)])

TEST_IMPORT_OLD = "import { cacheCorrectnessRepositoryFindingsV1, globMatchesV1, policyCacheCorrectnessBreaches, repositoryCacheRecordsV1, type CacheFindingV1 } from \"../../🟦️.ts\";"
TEST_IMPORT_NEW = "import { cacheCorrectnessRepositoryFindingsV1, globMatchesV1, policyCacheCorrectnessBreaches, repositoryCacheRecordsV1, staleRustKeyExemptionsV1, type CacheFindingV1 } from \"../../🟦️.ts\";"
TEST_TAIL = '''

test("every exempt whole-workspace Rust key still exists: the exemption list only shrinks", async () => {
  const plugin = await import("../../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs");
  await plugin.libraryBootstrap;
  expect(staleRustKeyExemptionsV1(repositoryCacheRecordsV1(workspace, plugin.cacheInternals as never))).toEqual([]);
}, 120000);
'''
def law_test(text):
    text, changed = chain(text, [("law-test-import", TEST_IMPORT_OLD, TEST_IMPORT_NEW)])
    if TEST_TAIL.strip() in text: return text, changed
    return text.rstrip("\n") + "\n" + TEST_TAIL, True

def schema(text):
    return chain(text, [("law-schema", '"uncached-without-reason", "whole-repository-input"]', '"uncached-without-reason", "whole-repository-input", "whole-workspace-rust-key"]')])

NAMED = {"default": ["{projectRoot}/**/*", "sharedGlobals"], "sharedGlobals": [{"runtime": "bun --version"}, {"env": "SEMIO_BUILD_MODE"}], "production": ["{projectRoot}/**/*", "!{projectRoot}/**/🧪️tests/**/*", "sharedGlobals"], "testEnv": [{"env": "SEMIO_TEST_LEVEL"}]}
def case(id_, inputs, expect, named=None, decision=None, command=None, root="lib/pkg"):
    semio = {"cache": {"version": 1, "class": "deterministic", "reason": "check-without-outputs"}}
    if decision: semio["inputPrecision"] = {"version": 1, "rule": "r", "evidence": decision}
    target = {"cache": True, "inputs": inputs, "outputs": [], "metadata": {"semio": semio}}
    if command: target["options"] = {"command": command}
    return {"id": id_, "records": [{"id": "r:t", "root": root, "authoredCache": None, "target": target, "namedInputs": {**NAMED, **(named or {})}}], "expect": [["r:t", "whole-workspace-rust-key"]] if expect else []}
CASES = [
    case("rust-key-root-cargo-lock", ["default", "{workspaceRoot}/Cargo.lock"], True),
    case("rust-key-root-manifest-config-toolchain", ["{workspaceRoot}/Cargo.toml", "{workspaceRoot}/.cargo/config.toml", "{workspaceRoot}/rust-toolchain.toml"], True),
    case("rust-key-named-group-expands", ["cargoToolchain"], True, named={"cargoToolchain": ["{workspaceRoot}/Cargo.toml", "{workspaceRoot}/Cargo.lock", "{workspaceRoot}/rustfmt.toml"]}),
    case("rust-key-all-manifests-glob", ["{workspaceRoot}/**/Cargo.toml"], True),
    case("rust-key-all-sources-glob", ["{workspaceRoot}/**/*.rs"], True),
    case("rust-key-all-schema-glob", ["{workspaceRoot}/**/🧬️schema/**/*"], True),
    case("rust-key-braced-tail", ["{workspaceRoot}/**/{Cargo.toml,📋️project.json}"], True),
    case("rust-key-generated-group-is-allowed", ["cargoToolchain", "{workspaceRoot}/rustfmt.toml", {"env": "SEMIO_CARGO_LOCK_" + "0" * 64}], False, named={"cargoToolchain": ["{workspaceRoot}/rustfmt.toml", {"env": "SEMIO_CARGO_LOCK_" + "0" * 64}]}),
    case("rust-key-crate-owned-files-are-allowed", ["{workspaceRoot}/crates/a/Cargo.toml", "{workspaceRoot}/crates/a/**/*.rs", "{workspaceRoot}/🌎️hub/**/*.rs", "{projectRoot}/Cargo.toml"], False),
    case("rust-key-whole-by-design-is-a-recorded-reason", ["{workspaceRoot}/**/Cargo.toml", "{workspaceRoot}/Cargo.lock"], False, decision="whole"),
    case("rust-key-assumed-is-recorded-debt", ["{workspaceRoot}/Cargo.lock"], False, decision="assumed"),
    case("rust-key-narrowing-rule-does-not-excuse-it", ["{workspaceRoot}/Cargo.lock"], True, decision="static"),
    case("rust-key-cargo-metadata-validates-every-manifest", ["{workspaceRoot}/**/Cargo.toml", "{workspaceRoot}/**/Cargo.lock"], False, command="bun x native cargo metadata --manifest a/Cargo.toml"),
    case("rust-key-negation-is-not-a-key", ["{projectRoot}/**/*", "!{workspaceRoot}/**/Cargo.lock"], False),
]
def corpus(text):
    data = json.loads(text)
    have = {c["id"] for c in data["cases"]}
    if all(c["id"] in have for c in CASES): return text, False
    anchor = '    {\n      "id": "broad-but-not-whole-glob-is-allowed"'
    if text.count(anchor) != 1: raise SystemExit("anchor law-corpus: %d" % text.count(anchor))
    add = "".join("    " + json.dumps(c, ensure_ascii=False, indent=2).replace("\n", "\n    ") + ",\n" for c in CASES if c["id"] not in have)
    return text.replace(anchor, add + anchor), True
