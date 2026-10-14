"""🪓️ Splits `semio-framework-plugin-interpreter` into three acyclic owners under the global source admission ceiling:
the core interpreter (`semio-framework-plugin-interpreter`), the guest-runtime contract every backend shares
(`semio-framework-plugin-guest-contract`) and the owned runtime (`semio-framework-plugin-interpreter-runtime`). Every cut is
anchored on the CURRENT source and asserts its match count; nothing is written unless `--write` is passed."""
import pathlib, re, sys

REPO = pathlib.Path("/Users/ueli/Documents/semio")
PLUGIN = REPO / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
CORE = PLUGIN / "🧠️interpreter"
RUNTIME = CORE / "🏃️runtime"
CONTRACT = PLUGIN / "🧬️guest-contract"
writes, deletes = {}, []


def text_of(path):
    return writes[path] if path in writes else path.read_text()


def edit(path, pairs):
    text = text_of(path)
    for old, new, count in pairs:
        found = text.count(old)
        assert found == count, f"{path.relative_to(REPO)}: {found} != {count}: {old[:140]!r}"
        text = text.replace(old, new)
    writes[path] = text


def cut(text, start, end):
    assert text.count(start) == 1, start[:90]
    a = text.index(start)
    return text[a:text.index(end, a) + len(end)]


runtime_path = RUNTIME / "🦀️.rs"
runtime = runtime_path.read_text()
faults = cut(runtime, "//#region 🧯️GuestFaults\n", "        return Err(TurnFault::Trapped(\"cold-pair page length or verified frontier is invalid\".to_owned()));\n    }\n    Ok(())\n}\n")
documents = cut(runtime, "/// 📦️ One document's authoritative binary pair as the owned `codec` exports return it.\n", "pub struct GuestDocumentMirror {\n    pub dsl: String,\n    pub ops: String,\n}\n")
identity = cut(runtime, "/// 🛂️ A refused original native admission, as the host fault every runtime reports it as.\n", "TurnFault{TurnFault::Host(PluginHostError::NativeIo(error))}\n")

contract_source = '''//! 🧬️ `semio-framework-plugin-guest-contract`: the vocabulary every plugin guest runtime shares — the owned interpreter's
//! runtime and the Wasmtime component host alike. Guest call faults ([`TurnFault`], [`PluginHostError`]), job budgets and
//! steps, codec document pairs, caller cancellation, the original operation authority wire, the original actor context and
//! the SQLite snapshot wire. It links no engine.
//! @see https://component-model.bytecodealliance.org/design/wit.html
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[path = "../🚪️io/🛂️authority/🦀️.rs"]
pub mod operation_authority;
#[path = "../⚛️reactor/🎟️context/🦀️.rs"]
pub mod original_actor_context;
#[path = "../🧬️schema/🪶️sqlite/🦀️.rs"]
pub mod sqlite_wire;

/// 🧮️ The heap witness the crate's allocation laws read through `semio_framework_trace::observe_heap_allocations_on_this_thread`.
#[cfg(test)]
#[global_allocator]
static GUEST_CONTRACT_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

''' + faults.replace("([`OWNED_STEP_FUEL`] instructions)", "(`OWNED_STEP_FUEL` instructions of the owned runtime)") + "\n" + documents + "\n" + identity

edit(runtime_path, [
    (faults + "\n", "", 1),
    (documents + "\n", "", 1),
    (identity + "\n", "", 1),
    ('''//! 🏃️ The owned interpreter as a guest runtime ([`OwnedRuntime`]): compiles a `world actor` component for the
//! repository-owned interpreter ([`crate::interpreter`]), runs its actor turns, jobs, checkpoints and pure `codec` exports
//! under instruction fuel, and answers every host import a guest can observe. It also owns the guest-runtime vocabulary every
//! backend shares — [`TurnFault`], [`PluginHostError`], [`JobBudget`], [`JobStep`], [`GuestCallCancellation`] — so the
//! Wasmtime host builds on it, and a tool that only describes or verifies components links no JIT.
''', '''//! 🏃️ `semio-framework-plugin-interpreter-runtime`: the owned interpreter as a guest runtime ([`OwnedRuntime`]). It compiles a
//! `world actor` component for the repository-owned interpreter (`semio-framework-plugin-interpreter`), runs its actor turns,
//! jobs, checkpoints and pure `codec` exports under instruction fuel, and answers every host import a guest can observe, in
//! the vocabulary of `semio-framework-plugin-guest-contract`. A tool that only describes or verifies components links this and
//! no JIT.
''', 1),
    ("use crate::interpreter::{CoreStepOutcome, HostCall, OwnedSemioArtifact, OwnedSemioExport, OwnedSemioInstance, StepControl, Value};\nuse crate::{operation_authority, original_actor_context, sqlite_wire};\n",
     "use semio_framework_plugin_guest_contract::{decode_guest_fault_bytes, identity_turn_fault, operation_authority, original_actor_context, sqlite_wire, validate_cold_page, GuestCallCancellation, GuestDocumentMirror, GuestDocumentPair, JobBudget, JobStep, PluginHostError, TurnFault};\nuse semio_framework_plugin_interpreter::{CoreError, CoreStepOutcome, HostCall, OwnedSemioArtifact, OwnedSemioExport, OwnedSemioInstance, StepControl, Value};\n", 1),
    ("use std::sync::atomic::AtomicBool;\nuse std::sync::{Arc, Mutex};\n", "use std::sync::{Arc, Mutex};\n\n/// 🛂️ The neutral test caller admission the plugin host's runtime laws share; this crate's laws drive part of it.\n#[cfg(test)]\n#[allow(dead_code)]\n#[path = \"../../🖥️host/🧪️tests/🛂️receiving/🦀️.rs\"]\npub(crate) mod test_native_authority;\n\n/// 🧮️ The heap witness the crate's allocation laws read through `semio_framework_trace::observe_heap_allocations_on_this_thread`.\n#[cfg(test)]\n#[global_allocator]\nstatic OWNED_RUNTIME_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;\n", 1),
    ('        let interpreter = include_str!("../🦀️.rs").as_bytes();', "        let interpreter = semio_framework_plugin_interpreter::INTERPRETER_SOURCE.as_bytes();", 1),
    ("impl From<crate::interpreter::CoreError> for TurnFault {\n    fn from(error: crate::interpreter::CoreError) -> Self {\n        Self::Trapped(error.to_string())\n    }\n}\n",
     "/// 🧯️ A core interpreter error, as the trap a guest call reports.\nfn core_fault(error: CoreError) -> TurnFault {\n    TurnFault::Trapped(error.to_string())\n}\n", 1),
])
for path in [RUNTIME / "🎟️call/🦀️.rs", RUNTIME / "🏗️startup/🦀️.rs"]:
    edit(path, [("use crate::interpreter::{", "use semio_framework_plugin_interpreter::{", 1)])

core_path = CORE / "🦀️.rs"
edit(core_path, [("use std::collections::{BTreeMap, BTreeSet};\n", '''use std::collections::{BTreeMap, BTreeSet};

/// 🪪️ The interpreter's authored source: the half of the owned engine identity that names every instruction a guest executes.
pub const INTERPRETER_SOURCE: &str = include_str!("🦀️.rs");

/// 🧮️ The heap witness the crate's allocation laws read through `semio_framework_trace::observe_heap_allocations_on_this_thread`.
#[cfg(test)]
#[global_allocator]
static INTERPRETER_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
''', 1)])
deletes.append(CORE / "📦️packages/🦀️rust/🦀️.rs")

core_manifest = CORE / "📦️packages/🦀️rust/Cargo.toml"
manifest = core_manifest.read_text()
tests = cut(manifest, "[[test]]\nname = \"original_native_frame_caller\"", "\n")
writes[core_manifest] = '''[package]
workspace = "../../../../../../../.."
name = "semio-framework-plugin-interpreter"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Repository-owned WebAssembly core interpreter: instruction-fuelled, cancellable, checkpointed core execution with no JIT"

[package.metadata.semio]
role = "framework"
id = "plugin-interpreter"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
semio-framework-actor = { workspace = true }
semio-framework-job = { workspace = true }
semio-framework-value = { workspace = true }

[dev-dependencies]
semio-framework-job = { workspace = true, features = ["testkit"] }
# 🐎️ The independent execution oracle of the interpreter's laws (`🧪️tests/🔬️unit`): the same Wasmtime the plugin host links, never a
# dependency of the crate itself.
wasmtime = { version = "47.0.3", features = ["cranelift", "component-model", "component-model-async"] }
semio-framework-trace = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }

[[test]]
name = "original_host_deadline"
path = "../../../🖥️host/⏱️deadline/🧪️tests/🦀️.rs"

[[test]]
name = "original_host_root"
path = "../../../🖥️host/🔔️wake/🎟️root/🧪️tests/🦀️.rs"
'''
assert "original_host_deadline" in manifest and "original_host_root" in manifest and "original_actor_poll_input" in manifest and "original_native_frame_caller" in manifest

shared = '''
[dependencies]
semio-framework = { workspace = true }
semio-framework-actor = { workspace = true }
semio-framework-artifact-reference = { workspace = true }
semio-framework-diagnostic = { path = "../../../../../../../🔨️modules/⚠️diagnostic/📦️packages/🦀️rust" }
semio-framework-job = { workspace = true }
semio-framework-os-kernel = { workspace = true }
semio-framework-value = { workspace = true }
semio-framework-value-derive = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
'''
writes[CONTRACT / "📦️packages/🦀️rust/Cargo.toml"] = '''[package]
workspace = "../../../../../../../.."
name = "semio-framework-plugin-guest-contract"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Vocabulary every plugin guest runtime shares: guest call faults, job steps, codec pairs, caller cancellation, original operation authority, original actor context and the SQLite snapshot wire"

[package.metadata.semio]
role = "framework"
id = "plugin-guest-contract"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"
''' + shared + '''semio-framework-async = { workspace = true }

[dev-dependencies]
semio-framework-job = { workspace = true, features = ["testkit"] }
semio-framework-trace = { workspace = true }

[[test]]
name = "original_native_frame_caller"
path = "../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎟️native-caller/🧪️tests/🦀️.rs"

[[test]]
name = "original_actor_poll_input"
path = "../../../⚛️reactor/🎟️context/🌉️call/📥️input/🧪️tests/🦀️.rs"
'''
writes[CONTRACT / "🦀️.rs"] = contract_source
writes[RUNTIME / "📦️packages/🦀️rust/Cargo.toml"] = '''[package]
workspace = "../../../../../../../../.."
name = "semio-framework-plugin-interpreter-runtime"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "The owned interpreter as a plugin guest runtime: compiles world-actor components and runs their turns, jobs, checkpoints and pure codec exports under instruction fuel, with no JIT"

[package.metadata.semio]
role = "framework"
id = "plugin-interpreter-runtime"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
semio-framework = { workspace = true }
semio-framework-actor = { workspace = true }
semio-framework-artifact-reference = { workspace = true }
semio-framework-diagnostic = { path = "../../../../../../../../🔨️modules/⚠️diagnostic/📦️packages/🦀️rust" }
semio-framework-hash = { workspace = true }
semio-framework-job = { workspace = true }
semio-framework-os-kernel = { workspace = true }
semio-framework-pack-json = { path = "../../../../../../../../🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust" }
semio-framework-plugin-guest-contract = { workspace = true }
semio-framework-plugin-interpreter = { workspace = true }
semio-framework-trace = { workspace = true }
semio-framework-value = { workspace = true }
semio-framework-value-derive = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }

[dev-dependencies]
semio-framework-job = { workspace = true, features = ["testkit"] }
semio-framework-async-macros = { path = "../../../../../../../../🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust" }
wasmtime = { version = "47.0.3", features = ["cranelift", "component-model", "component-model-async"] }
'''

core_project = CORE / "📦️packages/🦀️rust/📋️project.json"
core_script = CORE / "📦️packages/🦀️rust/📜️script.ts"
script = core_script.read_text()
imports = script[:script.index("/** 🧠️ Type-checks the owned interpreter crate. */")]
for root, project, label in [(CONTRACT, "@semio-tech/framework-plugin-guest-contract", "guest contract"), (RUNTIME, "@semio-tech/framework-plugin-interpreter-runtime", "owned interpreter runtime")]:
    depth = "../" * (len((root / "📦️packages/🦀️rust").relative_to(REPO).parts))
    writes[root / "📦️packages/🦀️rust/📋️project.json"] = '''{
  "name": "%s",
  "root": "%s",
  "sourceRoot": "%s",
  "projectType": "library",
  "tags": [
    "lang:rust",
    "role:framework",
    "family:os-plugin"
  ],
  "targets": {
    "check": {
      "executor": "repo:script",
      "options": {
        "command": "bun 📜️script.ts check",
        "cwd": "{projectRoot}",
        "forwardAllArgs": true
      }
    },
    "test": {
      "executor": "repo:script",
      "options": {
        "command": "bun 📜️script.ts test",
        "cwd": "{projectRoot}",
        "forwardAllArgs": true
      }%s
    }
  }
}
''' % (project, (root / "📦️packages/🦀️rust").relative_to(REPO), root.relative_to(REPO), ''',
      "dependsOn": [
        {
          "projects": [
            "@semio-tech/framework-plugin-host-test-component"
          ],
          "target": "component"
        }
      ]''' if root == RUNTIME else "")
    own_imports = imports
    if root == RUNTIME:
        own_imports = own_imports.replace('"../../../../../../../🔨️modules/', '"../../../../../../../../🔨️modules/').replace('"../../../../../../🦑️repo/', '"../../../../../../../🦑️repo/')
    used = [line for line in own_imports.splitlines() if any(name in line for name in ["receiveScriptProcessInvocation", "runCargo", "BundleScript", "runScriptMain"])]
    used = [line.replace("runCargo, runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1", "runCargo") for line in used]
    writes[root / "📦️packages/🦀️rust/📜️script.ts"] = "#!/usr/bin/env bun\n" + "\n".join(line for line in used if not line.startswith("#!")) + f'''

/** 🧠️ Type-checks the {label} crate. */
class CheckScript extends BundleScript {{
  async run(segments: string[]): Promise<void> {{
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }}
}}

/** 🧪️ Runs the {label} crate's library laws. */
class TestScript extends BundleScript {{
  async run(segments: string[]): Promise<void> {{
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", ...segments], this.root);
  }}
}}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript), {{ invocation: original }}));
'''

edit(core_script, [
    ("/** 🧪️ Runs the interpreter, owned runtime and shared wire laws of the crate's library tests. */", "/** 🧪️ Runs the core interpreter's library laws. */", 1),
    ('const laws = ["interpreter::tests::original_owned_job_export_selectors_match_authored_contract_and_wasmtime", "runtime::owned_runtime_tests::owned_result_preserves_original_fixed_actor_receipt_without_allocation", "original_actor_context::guest_entry::owned_job_receipt::tests::original_owned_job_header_matches_portable_oracle_without_allocation"];',
     'const groups = [{ package: "semio-framework-plugin-interpreter", target: { kind: "lib" as const }, laws: ["tests::original_owned_job_export_selectors_match_authored_contract_and_wasmtime"] }, { package: "semio-framework-plugin-interpreter-runtime", target: { kind: "lib" as const }, laws: ["owned_runtime_tests::owned_result_preserves_original_fixed_actor_receipt_without_allocation"] }, { package: "semio-framework-plugin-guest-contract", target: { kind: "lib" as const }, laws: ["original_actor_context::guest_entry::owned_job_receipt::tests::original_owned_job_header_matches_portable_oracle_without_allocation"] }];', 1),
    ('groups: [{ package: "semio-framework-plugin-interpreter", target: { kind: "lib" }, laws }]', "groups", 1),
    ("exactLaws=${laws.length}", "exactLaws=${groups.reduce((sum, group) => sum + group.laws.length, 0)}", 1),
    ('groups:[{package:"semio-framework-plugin-interpreter",target:{kind:"test",name:"original_actor_poll_input"}', 'groups:[{package:"semio-framework-plugin-guest-contract",target:{kind:"test",name:"original_actor_poll_input"}', 1),
])

root_manifest = REPO / "Cargo.toml"
edit(root_manifest, [
    ('    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/📦️packages/🦀️rust",\n', '    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/📦️packages/🦀️rust",\n    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/📦️packages/🦀️rust",\n    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️guest-contract/📦️packages/🦀️rust",\n', 1),
    ('semio-framework-plugin-interpreter = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/📦️packages/🦀️rust" }\n', 'semio-framework-plugin-interpreter = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/📦️packages/🦀️rust" }\nsemio-framework-plugin-interpreter-runtime = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/📦️packages/🦀️rust" }\nsemio-framework-plugin-guest-contract = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️guest-contract/📦️packages/🦀️rust" }\n', 1),
])
hub_manifest = REPO / "🌎️hub/Cargo.toml"
edit(hub_manifest, [
    ('semio-framework-plugin-interpreter = { path = "../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/📦️packages/🦀️rust" }\n', 'semio-framework-plugin-interpreter-runtime = { path = "../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/📦️packages/🦀️rust" }\nsemio-framework-plugin-guest-contract = { path = "../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️guest-contract/📦️packages/🦀️rust" }\n', 1),
])

CONTRACT_NAMES = {"TurnFault", "PluginHostError", "GuestCallCancellation", "JobBudget", "JobStep", "GuestDocumentPair", "GuestDocumentMirror", "IoRouteConflict", "IoEntryRouteConflict", "decode_guest_fault_bytes", "identity_turn_fault", "validate_cold_page", "GUEST_FAULT_MAXIMUM_BYTES", "GUEST_DIAGNOSTICS_CAPACITY_BYTES", "operation_authority", "original_actor_context", "sqlite_wire"}
RUNTIME_NAMES = {"OwnedRuntime", "OwnedCompiledGuest", "OwnedGuestInstance", "OwnedInvocation", "OwnedSnapshotInput", "OwnedSnapshotOperation", "operation", "owned_engine_identity", "decode_owned_result", "OWNED_STEP_FUEL", "OriginalGuestJobCall", "OwnedActorStartup", "OwnedActorStartupCall", "OwnedActorStartupStatus", "OwnedOriginalCallStatus", "OwnedOriginalCallProgress", "OwnedOriginalCallResult"}
CRATE = {**{name: "semio_framework_plugin_guest_contract" for name in CONTRACT_NAMES}, **{name: "semio_framework_plugin_interpreter_runtime" for name in RUNTIME_NAMES}}
import subprocess
consumers = [REPO / line for line in subprocess.run(["rg", "-l", "semio_framework_plugin_interpreter", "--glob", "*.rs", "--glob", "!**/🎫️tickets/**", "--glob", "!**/target/**", "."], cwd=REPO, capture_output=True, text=True).stdout.split()]
for path in consumers:
    text = text_of(path).replace("semio_framework_plugin_interpreter::interpreter::", "semio_framework_plugin_interpreter::")

    def split_list(match):
        names = [name.strip() for name in match.group(1).split(",") if name.strip()]
        groups = {}
        for name in names:
            groups.setdefault(CRATE.get(name.split("::")[0].split(" as ")[0].strip(), "semio_framework_plugin_interpreter"), []).append(name)
        sep = ", " if ", " in match.group(1) else ","
        return " ".join(f"use {crate}::{{{sep.join(items)}}};" if len(items) > 1 else f"use {crate}::{items[0]};" for crate, items in groups.items())

    text = re.sub(r"use semio_framework_plugin_interpreter::\{([^}]*)\};", split_list, text)
    text = re.sub(r"semio_framework_plugin_interpreter::(" + "|".join(sorted(CRATE, key=len, reverse=True)) + r")\b", lambda m: f"{CRATE[m.group(1)]}::{m.group(1)}", text)
    writes[path] = text

if "--write" not in sys.argv:
    for path in sorted(writes):
        print(f"[dry-run] {path.relative_to(REPO)}")
    for path in deletes:
        print(f"[dry-run] delete {path.relative_to(REPO)}")
    sys.exit(0)
for path, text in writes.items():
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)
for path in deletes:
    path.unlink()
print(f"[split] wrote {len(writes)} files, deleted {len(deletes)}")
