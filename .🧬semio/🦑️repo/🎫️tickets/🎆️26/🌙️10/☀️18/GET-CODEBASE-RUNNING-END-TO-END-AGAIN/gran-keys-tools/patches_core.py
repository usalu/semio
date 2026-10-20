# 🪝 Anchored edits of the live lock reader, the task-environment digest, the cache policy and its schema, the project input inference and the graph assembly.
from patchlib import chain

IMPORT_LOCK = 'import schema from"../../🧮️digest/📥️read/🧬️schema/🔣️.json"with{type:"json"};'
LOCK_IMPORTS = IMPORT_LOCK + '''
import{createCargoResolutionContextV1,cargoResolutionPartsV1,composeCargoResolutionV1,readCargoResolutionPolicyV1}from"../../🦀️resolution/🟦️.ts";
import{createCargoReceiptReaderV1}from"../../🦀️resolution/💾️receipt/🟨️.mjs";
import{repositoryBuildGraphDataDirectoryV1}from"../../💾️storage/🟨️.mjs";
import resolutionDocument from"../../🦀️resolution/🔣️.json"with{type:"json"};
const resolutionPolicy=readCargoResolutionPolicyV1(resolutionDocument);'''
LOCK_TAIL_OLD = 'export function selectedCargoLockDigest(workspaceRoot,manifest,control){requireIssuedResourceControlV1(control);if(typeof manifest!=="string")throw Error("Selected Cargo manifest path required");return withExactSourceReadBatchV1(workspaceRoot,control,batch=>selectedCargoLockDigestInBatchV1(batch,workspaceRoot,manifest));}'
LOCK_TAIL_NEW = LOCK_TAIL_OLD + '''
/** 🦀️ Opens one resolution run over a live Source batch: every manifest is read once through the batch (parsed once, digested with the same read), the receipts the graph build persisted are read from the graph data directory and are used only while the digests they recorded still hold, and the run never writes. */
export function cargoResolutionContextInBatchV1(batch,workspaceRoot){requireExactSourceReadBatchV1(batch,workspaceRoot);let store;try{store=createCargoReceiptReaderV1(repositoryBuildGraphDataDirectoryV1(workspaceRoot,"inference"));}catch{store=undefined;}return createCargoResolutionContextV1({read:async path=>{const file=await readExactSourceTomlV1(batch,path);return file?{value:file.value,sha:file.sha256}:null;},digest:async path=>(await readExactSourceFileV1(batch,path))?.sha256??null},resolutionPolicy,store);}
/** 🦀️ The resolution identity of one crate: its locked dependency slice, what it and every crate it compiles with inherit from their own workspaces, the pinned toolchain and the Cargo configuration without comments; nx and Bazel fold it with this one function. */
export async function cargoResolutionDigestInBatchV1(batch,workspaceRoot,manifest,context=cargoResolutionContextInBatchV1(batch,workspaceRoot)){return composeCargoResolutionV1(await cargoResolutionPartsV1(context,captureCargoLockInventoryV1({version:1,manifest},workspaceRoot).value.manifest));}
/** 🦀️ Resolves many crates under one fresh batch and one shared reading state, for an adapter that keys a whole plan at once; a crate whose lock refuses it (stale or missing local package) keeps the refusal in its key, because the action itself fails under `--locked`, and does not stop the plan. */
export function selectedCargoResolutionDigests(workspaceRoot,manifests,control){requireIssuedResourceControlV1(control);return withExactSourceReadBatchV1(workspaceRoot,control,async batch=>{const context=cargoResolutionContextInBatchV1(batch,workspaceRoot),result=new Map();for(const manifest of manifests)result.set(manifest,composeCargoResolutionV1(await cargoResolutionPartsV1(context,captureCargoLockInventoryV1({version:1,manifest},workspaceRoot).value.manifest,{tolerant:true})));return result;});}'''

READ_IMPORT_OLD = 'import{withExactSourceReadBatchV1,readExactSourceTomlV1,requireExactSourceReadBatchV1}from"../../🧮️digest/📥️read/🟨️.mjs";'
READ_IMPORT_NEW = 'import{withExactSourceReadBatchV1,readExactSourceTomlV1,readExactSourceFileV1,requireExactSourceReadBatchV1}from"../../🧮️digest/📥️read/🟨️.mjs";'
def lock_read(text):
    return chain(text, [("lock-read-import", READ_IMPORT_OLD, READ_IMPORT_NEW), ("lock-imports", IMPORT_LOCK, LOCK_IMPORTS), ("lock-tail", LOCK_TAIL_OLD, LOCK_TAIL_NEW)])

DTS_OLD = 'export function selectedCargoLockDigest(workspaceRoot:string,manifest:string,control:ResourceLeaseControl):Promise<string>;'
DTS_NEW = DTS_OLD + '''
/** 🦀️ One resolution run over a live Source batch: manifests are read once through it. */
export function cargoResolutionContextInBatchV1(batch:ExactSourceReadBatchV1,workspaceRoot:string):unknown;
/** 🦀️ The resolution identity of one crate under a live batch: locked slice, workspace inheritance of its path closure, pinned toolchain and configuration. */
export function cargoResolutionDigestInBatchV1(batch:ExactSourceReadBatchV1,workspaceRoot:string,manifest:string,context?:unknown):Promise<string>;
/** 🦀️ The resolution identities of many crates under one fresh batch and one shared reading state. */
export function selectedCargoResolutionDigests(workspaceRoot:string,manifests:readonly string[],control:ResourceLeaseControl):Promise<ReadonlyMap<string,string>>;'''
def lock_dts(text): return chain(text, [("lock-dts", DTS_OLD, DTS_NEW)])

def digest(text):
    return chain(text, [
        ("digest-import", 'import{selectedCargoLockDigestInBatchV1}from"../🔒️lock/📥️read/🟨️.mjs";', 'import{cargoResolutionContextInBatchV1,cargoResolutionDigestInBatchV1}from"../🔒️lock/📥️read/🟨️.mjs";'),
        ("digest-state", 'const values=new Map(),result={};', 'const values=new Map(),result={};let resolution;'),
        ("digest-lock", 'values.set(name,await selectedCargoLockDigestInBatchV1(batch,root,contract.manifest))', 'values.set(name,await cargoResolutionDigestInBatchV1(batch,root,contract.manifest,resolution??=cargoResolutionContextInBatchV1(batch,root)))'),
    ])

FILES_OLD = '''      "files": [
        "Cargo.toml",
        ".cargo/config.toml",
        "rust-toolchain.toml",
        "rustfmt.toml",
'''
FILES_NEW = '''      "files": [
        "rustfmt.toml",
'''
CARGO_ENV_ANCHOR = '''        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️execution/📜️script.ts"
      ],
      "environment": [
        "RUSTUP_TOOLCHAIN",'''
CARGO_ENV_NEW = '''        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️execution/📜️script.ts"
      ],
      "identity": [
        "Cargo.toml",
        ".cargo/config.toml",
        "rust-toolchain.toml"
      ],
      "environment": [
        "RUSTUP_TOOLCHAIN",'''
def policy(text):
    return chain(text, [("policy-files", FILES_OLD, FILES_NEW), ("policy-identity", CARGO_ENV_ANCHOR, CARGO_ENV_NEW)])

SCHEMA_OLD = '''        "properties": {
          "files": {
            "type": "array",
            "items": {
              "type": "string"
            }
          },
          "environment": {'''
SCHEMA_NEW = '''        "properties": {
          "files": {
            "type": "array",
            "items": {
              "type": "string"
            }
          },
          "identity": {
            "description": "Files whose meaning, not their bytes, keys a task: the Cargo resolution identity digests them semantically (comments, ordering and artifact locations never count), so a task never hashes them raw; they are still watched.",
            "type": "array",
            "items": {
              "type": "string"
            }
          },
          "environment": {'''
def policy_schema(text): return chain(text, [("policy-schema", SCHEMA_OLD, SCHEMA_NEW)])

INF_OLD = "  const groups = (sourceContracts) => ({ ...declarations, ...sourceContracts, default: [...inputs,"
INF_NEW = '''  const cargoManifest = existsSync(join(workspaceRoot, nativeRoot, "Cargo.toml")) ? `${nativeRoot}/Cargo.toml` : undefined, persisted = context.sourceRepresentation !== "source";
  if (persisted && cargoManifest && tools.includes("cargo")) await warmCargoResolutionV1(workspaceRoot, scripts, cargoManifest, context);
  const cargoToolchain = tools.includes("cargo") ? [...POLICY.toolchains.cargo.files.map((file) => `{workspaceRoot}/${file}`), ...(persisted && cargoManifest ? [cargoLockInput(cargoManifest, workspaceRoot)] : [])] : undefined;
  const groups = (sourceContracts) => ({ ...declarations, ...sourceContracts, ...(cargoToolchain ? { cargoToolchain } : {}), default: [...inputs,'''
INF_DOC_OLD = "/** 📥️ Projects source and compiler input ownership independently of target orchestration. */"
INF_DOC_NEW = "/** 📥️ Projects source and compiler input ownership independently of target orchestration. The named input `cargoToolchain` of a Rust project is generated here (raw tool files plus the crate's resolution marker, which only a persisted graph carries), never authored: an authored definition of it is replaced; a persisted graph also refreshes the crate's resolution receipts. */"
INF_IMPORT_OLD = 'import {repositoryBuildGraphWriterDirectoryV1} from "./💾️storage/🟨️.mjs";'
INF_IMPORT_NEW = '''import {repositoryBuildGraphWriterDirectoryV1, withBuildGraphOptionalPublicationV1} from "./💾️storage/🟨️.mjs";
import { createRequire } from "node:module";
import { createCargoResolutionContextV1, cargoResolutionPartsV1, flushCargoReceiptsV1, readCargoResolutionPolicyV1 } from "./🦀️resolution/🟦️.ts";
import { createCargoReceiptWriterV1 } from "./🦀️resolution/💾️receipt/🟨️.mjs";
import resolutionDocument from "./🦀️resolution/🔣️.json" with { type: "json" };'''
WARM_OLD = "/** 🧮️ Shares exact inventories while backends hash their current byte digest once per unique closure."
WARM_NEW = '''const resolutionPolicy = readCargoResolutionPolicyV1(resolutionDocument), resolutionRuns = new WeakMap();
let tomlParser;

/** 🦀️ Refreshes the resolution receipts of one crate while the graph is built, under the graph writer's custody: every manifest of one graph construction is read and digested once through plain reads (the receipts are validated by digest when they are used, so these reads need no observation), a receipt whose digests still hold costs no derivation, and a failing crate simply has no receipt. */
async function warmCargoResolutionV1(workspaceRoot, scripts, manifest, context) {
  try {
    const key = typeof scripts === "object" && scripts !== null ? scripts : context;
    let run = resolutionRuns.get(key);
    if (!run) {
      tomlParser ??= createRequire(import.meta.url)("@iarna/toml").parse;
      const bytes = (path) => { try { return readFileSync(join(workspaceRoot, path)); } catch (error) { if (error?.code === "ENOENT" || error?.code === "ENOTDIR") return null; throw error; } };
      const digest = (buffer) => createHash("sha256").update(buffer).digest("hex");
      const host = { read: async (path) => { const buffer = bytes(path); return buffer === null ? null : { value: tomlParser(buffer.toString("utf8")), sha: digest(buffer) }; }, digest: async (path) => { const buffer = bytes(path); return buffer === null ? null : digest(buffer); } };
      resolutionRuns.set(key, run = createCargoResolutionContextV1(host, resolutionPolicy, createCargoReceiptWriterV1(repositoryBuildGraphWriterDirectoryV1(workspaceRoot, "inference"), withBuildGraphOptionalPublicationV1)));
    }
    await cargoResolutionPartsV1(run, manifest, { tolerant: true });
    await flushCargoReceiptsV1(run);
  } catch {}
}

''' + WARM_OLD
def inputs_mjs(text): return chain(text, [("inputs-import", INF_IMPORT_OLD, INF_IMPORT_NEW), ("inputs-warm", WARM_OLD, WARM_NEW), ("inputs-doc", INF_DOC_OLD, INF_DOC_NEW), ("inputs-group", INF_OLD, INF_NEW)])

STORAGE_OLD = "(?:(?:np|ip)/[a-f0-9]{24}"
STORAGE_NEW = "(?:(?:np|ip|rr)/[a-f0-9]{24}"
def storage_schema(text): return chain(text, [("storage-rr", STORAGE_OLD, STORAGE_NEW)])
FIXTURE_OLD = '      "ip/aaaaaaaaaaaaaaaaaaaaaaaa.json",\n'
FIXTURE_NEW = FIXTURE_OLD + '      "rr/aaaaaaaaaaaaaaaaaaaaaaaa.json",\n'
def graph_fixture(text): return chain(text, [("graph-fixture-rr", FIXTURE_OLD, FIXTURE_NEW)])

ASM_IMPORT_OLD = 'import { deriveInputsV1, inputPrecisionDecisionV1, inputPrecisionLanguageV1, inputPrecisionTextV1, readInputPrecisionV1, selectRuleV1 } from "../../../📥️inputs/🎯️precision/🟦️.ts";'
ASM_IMPORT_NEW = ASM_IMPORT_OLD + '''
import { cargoResolutionLanguageV1, cargoResolutionTextV1, readCargoResolutionPolicyV1, workspaceKeyViolationsV1 } from "../../../📥️inputs/🦀️resolution/🟦️.ts";
import resolutionDocument from "../../../📥️inputs/🦀️resolution/🔣️.json" with { type: "json" };
const resolutionPolicy = readCargoResolutionPolicyV1(resolutionDocument);'''
def assembly(text):
    return chain(text, [
        ("assembly-import", ASM_IMPORT_OLD, ASM_IMPORT_NEW),
        ("assembly-watch", "    for (const path of POLICY.toolchains.cargo.files) includes.add(path);", "    for (const path of [...POLICY.toolchains.cargo.files, ...POLICY.toolchains.cargo.identity]) includes.add(path);"),
        ("assembly-guard", "      delete policy.cargoTestPolicyInputs;\n    }", '''      delete policy.cargoTestPolicyInputs;
      const wholeKeys = workspaceKeyViolationsV1(resolutionPolicy, commandSources, policy.options?.command);
      if (wholeKeys.length) throw new Error(cargoResolutionTextV1(resolutionPolicy, "workspaceKey", cargoResolutionLanguageV1(resolutionPolicy, process.env), { subject: `${json.name}:${name}`, keys: wholeKeys.join(", ") }));
    }'''),
    ])
