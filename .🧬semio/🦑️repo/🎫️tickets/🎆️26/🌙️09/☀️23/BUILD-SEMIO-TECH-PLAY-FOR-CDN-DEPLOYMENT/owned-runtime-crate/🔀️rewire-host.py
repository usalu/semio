"""🔀️ One compile-atomic pass that moves the owned runtime out of the plugin host: writes the runtime file
(`🧬️extract-runtime.py`), moves the owned operation framing, and rewires the host root, its package root, the snapshot
call, the component codec, the cold-pair module and the host's owned tests onto `semio-framework-plugin-interpreter`.
Every edit asserts its match count against the CURRENT source, so a concurrent change fails before anything is written."""
import pathlib, re, shutil, subprocess, sys

PLUGIN = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
HOSTDIR = PLUGIN / "🖥️host"
RUNTIME = PLUGIN / "🧠️interpreter/🏃️runtime"
TICKET = pathlib.Path(__file__).resolve().parent
writes = {}


def edit(path, pairs, text=None):
    text = path.read_text() if text is None else text
    for old, new, count in pairs:
        found = text.count(old)
        assert found == count, f"{path.name}: {found} != {count}: {old[:140]!r}"
        text = text.replace(old, new)
    writes[path] = text
    return text


def cut(text, start, end):
    assert text.count(start) == 1, start[:80]
    a = text.index(start)
    return text[a:text.index(end, a) + len(end)]


host_path = HOSTDIR / "🦀️.rs"
host = host_path.read_text()
errors = cut(host, "/// 🧯️ Errors from the host's engine/component/call-boundary plumbing", "//#endregion ⚠️ Errors\n")
vocabulary = cut(host, "/// ⛽️ `jobs.wit`'s `job-budget` record", "    TurnFault::Guest(semio_framework_diagnostic::decode_fault_bytes(bytes))\n}\n")
from_host = cut(host, "impl From<PluginHostError> for TurnFault {", "\n}\n")
owned = cut(host, "//#region 🧠️OwnedRuntime\n", "//#endregion 🧠️OwnedRuntime\n")
transport = cut(owned, "fn snapshot_component_transport<", "\n}\n")
diagnostics = cut(host, "/// 🩺️ How much guest stderr one instance retains while diagnostics are armed.", "const GUEST_DIAGNOSTICS_CAPACITY_BYTES: usize = 4 * 1024 * 1024;\n")

adapter = '''//#region 🧠️OwnedRuntime
/// 🧠️ The owned compiled guest of `compiled`; a handle another runtime compiled has none.
fn owned_compiled(compiled: &CompiledHandle) -> Result<&OwnedCompiledGuest, PluginHostError> {
    compiled.owned.as_ref().ok_or_else(|| PluginHostError::Plugin("CompiledHandle has no repository-owned actor artifact".to_string()))
}

/// 🧠️ The owned instance of `inst`; an instance another runtime built has none.
fn owned_instance_mut(inst: &mut GuestInstance) -> Result<&mut OwnedGuestInstance, TurnFault> {
    match &mut inst.state {
        GuestInstanceState::Owned(state) => Ok(state),
        _ => Err(TurnFault::Trapped("owned runtime called on a non-owned GuestInstance".to_string())),
    }
}

/// 🧳️ Copies a component snapshot's dialect coordinate and payload into storage the caller's original authority admitted,
/// for the Wasmtime codec exports that take both by value.
''' + transport + '''
impl GuestRuntime for OwnedRuntime {
    async fn admit_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<crate::original_actor_context::OriginalActorAdmissionReply,TurnFault>{self.admit_context(owned_instance_mut(inst)?,input,budget,identity)}
    async fn close_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<crate::original_actor_context::OriginalActorCloseReply,TurnFault>{self.close_context(owned_instance_mut(inst)?,input,budget,identity)}

    async fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {
        Ok(CompiledHandle { package_hash: package.hash.0, component: None, owned: Some(self.compile_component(bytes)?) })
    }

    async fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, _caps: &[BrokerCapabilityGrant], _budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        Ok(GuestInstance { actor, state: GuestInstanceState::Owned(self.instantiate_actor(owned_compiled(compiled)?)?) })
    }

    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault> {
        self.execute_actor_turn(owned_instance_mut(inst)?, events, budget, identity)
    }

    async fn start_job(&self, inst: &mut GuestInstance, job: u64, kind: &str, input: Vec<u8>) -> Result<(), TurnFault> {
        self.start_actor_job(owned_instance_mut(inst)?, job, kind, input)
    }

    async fn step_job(&self, inst: &mut GuestInstance, job: u64, budget: JobBudget) -> Result<JobStep, TurnFault> {
        self.step_actor_job(owned_instance_mut(inst)?, job, budget)
    }

    async fn cancel_job(&self, inst: &mut GuestInstance, job: u64) -> Result<(), TurnFault> {
        self.cancel_actor_job(owned_instance_mut(inst)?, job)
    }

    async fn checkpoint(&self, inst: &mut GuestInstance, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<u8>, PluginHostError> {
        let GuestInstanceState::Owned(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("checkpoint called on a non-owned GuestInstance".to_string()));
        };
        self.checkpoint_actor(state, identity)
    }

    async fn restore(&self, inst: &mut GuestInstance, checkpoint: &[u8], _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), PluginHostError> {
        let GuestInstanceState::Owned(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("restore called on a non-owned GuestInstance".to_string()));
        };
        self.restore_actor(state, checkpoint)
    }

    async fn drop_instance(&self, _inst: GuestInstance) {}
}

#[cfg(test)]
#[path = "🧪️tests/🔬️owned-instance-open/🦀️.rs"]
mod owned_instance_open_tests;
//#endregion 🧠️OwnedRuntime
'''

host = edit(host_path, [
    ('#[path = "../🧬️schema/🪶️sqlite/🦀️.rs"]\npub mod sqlite_wire;\n', '', 1),
    (errors, '', 1),
    (vocabulary, '', 1),
    (from_host + "\n", '', 1),
    (owned, adapter, 1),
    (diagnostics, '', 1),
    ('use crate::interpreter::{CoreStepOutcome, HostCall, OwnedSemioArtifact, OwnedSemioExport, OwnedSemioInstance, StepControl, Value};\n',
     'use semio_framework_plugin_interpreter::{decode_guest_fault_bytes, identity_turn_fault, sqlite_wire, GuestCallCancellation, GuestDocumentMirror, GuestDocumentPair, IoEntryRouteConflict, IoRouteConflict, JobBudget, JobStep, OwnedCompiledGuest, OwnedGuestInstance, OwnedRuntime, PluginHostError, TurnFault, GUEST_DIAGNOSTICS_CAPACITY_BYTES};\n', 1),
    ('    owned: Option<Arc<OwnedCompiledGuest>>,\n', '    owned: Option<OwnedCompiledGuest>,\n', 1),
    ('    /// 📏️ The bytes this compiled guest holds beyond its parsed component: the owned interpreter\'s codec origin once a\n    /// codec call assembled it ([`OwnedCodecOriginCell`]), else none. A residency budget charges a compiled guest this\n    /// on top of its component.\n    pub fn codec_origin_bytes(&self) -> u64 {\n        self.owned.as_ref().map_or(0, |owned| owned.codec_origin.resident_bytes())\n    }',
     '    /// 📏️ The bytes this compiled guest holds beyond its parsed component: the owned interpreter\'s codec origin once a\n    /// codec call assembled it ([`OwnedCompiledGuest::codec_origin_bytes`]), else none. A residency budget charges a compiled\n    /// guest this on top of its component.\n    pub fn codec_origin_bytes(&self) -> u64 {\n        self.owned.as_ref().map_or(0, OwnedCompiledGuest::codec_origin_bytes)\n    }', 1),
    ('matches!(&self.state, GuestInstanceState::Owned(state) if state.pending.is_some())', 'matches!(&self.state, GuestInstanceState::Owned(state) if state.turn_in_flight())', 1),
    ('GuestInstanceState::Owned(state) => Some(String::from_utf8_lossy(&state.diagnostics).into_owned()),', 'GuestInstanceState::Owned(state) => Some(String::from_utf8_lossy(state.diagnostics()).into_owned()),', 1),
    ('    Owned(OwnedInstanceState),\n', '    Owned(OwnedGuestInstance),\n', 1),
    ('use cold_pair::{kernel_cold_page_to_wit, validate_cold_page, wit_cold_ingress_to_kernel};', 'use cold_pair::{kernel_cold_page_to_wit, wit_cold_ingress_to_kernel};', 1),
    ('crate::reactor::original_actor_context', 'crate::original_actor_context', host.count('crate::reactor::original_actor_context') - owned.count('crate::reactor::original_actor_context') - vocabulary.count('crate::reactor::original_actor_context')),
])
dispatch = cut(host, "//#region 🗂️GuestCodecDispatch\n", "//#endregion 🗂️GuestCodecDispatch\n")
rewired = re.sub(r"Self::Owned\(runtime\) => runtime\.(codec_[a-z_]+)\(compiled,", r"Self::Owned(runtime) => runtime.\1(owned_compiled(compiled)?,", dispatch)
assert rewired.count("owned_compiled(compiled)?") == 8
host = edit(host_path, [(dispatch, rewired, 1)], host)

package_path = HOSTDIR / "📦️packages/🦀️rust/🦀️.rs"
edit(package_path, [
    ('/// 🧠️ Repository-owned, instruction-fuelled core execution boundary. Native component lifting is\n/// layered above this module; browser builds retain the platform WebAssembly boundary.\n#[path = "../../../🧠️interpreter/🦀️.rs"]\npub mod interpreter;\n', '', 1),
    ('#[path="../../../🚪️io/🛂️authority/🦀️.rs"]\npub mod operation_authority;\n', 'use semio_framework_plugin_interpreter::{operation_authority, original_actor_context};\n', 1),
    ('\n/// 🎭️ Mounts the plugin reactor\'s original actor-context vocabulary, the only reactor module the host names.\n#[path="../../../⚛️reactor/🎟️context/🦀️.rs"]\npub mod original_actor_context;\npub mod reactor {\n    pub use super::original_actor_context;\n}\n', '', 1),
])

snapshot_path = HOSTDIR / "🚪️io/🪶️snapshot/🦀️.rs"
snapshot = snapshot_path.read_text()
snapshot = edit(snapshot_path, [
    ("use std::mem::ManuallyDrop;\n", "use std::mem::ManuallyDrop;\nuse semio_framework_plugin_interpreter::{operation::input::encode_json_input_into,OwnedSnapshotInput,OwnedSnapshotOperation};\n", 1),
    ("if let GuestInstanceState::Owned(state)=&self.instance.state{if state.pending.is_some(){return Err(", "if let GuestInstanceState::Owned(state)=&self.instance.state{if state.turn_in_flight(){return Err(", 1),
    ("owned_runtime_pending:matches!(&self.instance.state,GuestInstanceState::Owned(state)if state.pending.is_some())", "owned_runtime_pending:matches!(&self.instance.state,GuestInstanceState::Owned(state)if state.turn_in_flight())", 1),
    ("if state.poisoned{return Err(crate::operation_authority::refusal(4))}let Some(pending)=state.pending.as_ref()else{return Ok(())};let operation=pending.operation;let remaining=remaining_fuel(self.fuel_ceiling,self.completed_fuel,pending.fuel_used,budget.fuel)?;",
     "if state.poisoned(){return Err(crate::operation_authority::refusal(4))}let Some((operation,fuel_used))=state.pending_snapshot().map_err(|_|crate::operation_authority::refusal(4))?else{return Ok(())};let remaining=remaining_fuel(self.fuel_ceiling,self.completed_fuel,fuel_used,budget.fuel)?;", 1),
    ("let result=resume_owned_operation_bound(state,operation,remaining,budget.deadline_ms,OwnedDeadline::NoFuelProgress,progress,Some(cancellation),Some(&mut receiver));",
     "let result=state.resume_snapshot(operation,remaining,budget.deadline_ms,progress,Some(cancellation),&mut receiver);", 1),
    ("if matches!(operation,OwnedOperation::SqliteRetirement|OwnedOperation::SqliteClose){", "if matches!(operation,OwnedSnapshotOperation::Retirement|OwnedSnapshotOperation::Close){", 1),
    ("self.metadata_kind=Some(if operation==OwnedOperation::SqliteClose{", "self.metadata_kind=Some(if operation==OwnedSnapshotOperation::Close{", 1),
    ("operation::input::encode_json_input_into(", "encode_json_input_into(", 5),
    ("begin_owned_snapshot_operation(state,OwnedOperation::SqliteRetirement,&mut input)?;resume_owned_operation_bound(state,OwnedOperation::SqliteRetirement,fuel,budget.deadline_ms,OwnedDeadline::NoFuelProgress,|_,_|{},None,Some(&mut receiver))",
     "state.begin_snapshot(OwnedSnapshotOperation::Retirement,&mut input)?;state.resume_snapshot(OwnedSnapshotOperation::Retirement,fuel,budget.deadline_ms,|_,_|{},None,&mut receiver)", 1),
    ("let operation=match direction{Direction::Export=>OwnedOperation::SqliteTakeFile,Direction::Import=>OwnedOperation::SqliteTakePayload};", "let operation=match direction{Direction::Export=>OwnedSnapshotOperation::TakeFile,Direction::Import=>OwnedSnapshotOperation::TakePayload};", 1),
    ("begin_owned_snapshot_operation(state,operation,&mut input)?;resume_owned_operation_bound(state,operation,fuel,budget.deadline_ms,OwnedDeadline::NoFuelProgress,|_,_|{},None,Some(&mut receiver))",
     "state.begin_snapshot(operation,&mut input)?;state.resume_snapshot(operation,fuel,budget.deadline_ms,|_,_|{},None,&mut receiver)", 1),
    ("begin_owned_snapshot_operation(state,OwnedOperation::SqliteClose,&mut input)?;resume_owned_operation_bound(state,OwnedOperation::SqliteClose,fuel,budget.deadline_ms,OwnedDeadline::NoFuelProgress,|_,_|{},None,Some(&mut receiver))",
     "state.begin_snapshot(OwnedSnapshotOperation::Close,&mut input)?;state.resume_snapshot(OwnedSnapshotOperation::Close,fuel,budget.deadline_ms,|_,_|{},None,&mut receiver)", 1),
    ("Self::Owned(runtime)=>{let owned=owned_compiled_guest(compiled)?;let used=runtime.codec_origin(owned,budget.clone(),&mut progress,Some(cancellation))?;let instance=runtime.codec_instance(owned,&used.origin);let mut call=SnapshotCall::new(Arc::clone(self),instance,direction,budget.fuel,used.spent_fuel);",
     "Self::Owned(runtime)=>{let (state,spent_fuel)=runtime.codec_instance(owned_compiled(compiled)?,budget.clone(),&mut progress,Some(cancellation))?;let mut call=SnapshotCall::new(Arc::clone(self),GuestInstance{actor:RuntimeActorId(0),state:GuestInstanceState::Owned(state)},direction,budget.fuel,spent_fuel);", 2),
    ("let state=owned_state_mut(&mut call.instance).unwrap();let result=begin_owned_snapshot_operation(state,OwnedOperation::SqliteExport,&mut input).and_then(|_|resume_owned_operation_bound(state,OwnedOperation::SqliteExport,budget.fuel.saturating_sub(used.spent_fuel),budget.deadline_ms,OwnedDeadline::NoFuelProgress,progress,Some(cancellation),Some(&mut receiver)));",
     "let GuestInstanceState::Owned(state)=&mut call.instance.state else{unreachable!()};let result=state.begin_snapshot(OwnedSnapshotOperation::Export,&mut input).and_then(|_|state.resume_snapshot(OwnedSnapshotOperation::Export,budget.fuel.saturating_sub(spent_fuel),budget.deadline_ms,progress,Some(cancellation),&mut receiver));", 1),
    ("let state=owned_state_mut(&mut call.instance).unwrap();let result=begin_owned_snapshot_operation(state,OwnedOperation::SqliteImport,&mut input).and_then(|_|resume_owned_operation_bound(state,OwnedOperation::SqliteImport,budget.fuel.saturating_sub(used.spent_fuel),budget.deadline_ms,OwnedDeadline::NoFuelProgress,progress,Some(cancellation),Some(&mut receiver)));",
     "let GuestInstanceState::Owned(state)=&mut call.instance.state else{unreachable!()};let result=state.begin_snapshot(OwnedSnapshotOperation::Import,&mut input).and_then(|_|state.resume_snapshot(OwnedSnapshotOperation::Import,budget.fuel.saturating_sub(spent_fuel),budget.deadline_ms,progress,Some(cancellation),&mut receiver));", 1),
])
edit(snapshot_path, [(cut(snapshot, "fn begin_owned_snapshot_operation(", "\n}\n"), "", 1)], snapshot)

codec_path = HOSTDIR / "🧬️component-codec/🦀️.rs"
edit(codec_path, [
    ("use super::{CompiledHandle, GuestRuntimes};", "use super::{CompiledHandle, GuestRuntimes, OwnedCompiledGuest};", 1),
    ("    runtime: Arc<GuestRuntimes>,\n    compiled: CompiledHandle,\n", "    runtime: Arc<GuestRuntimes>,\n    compiled: OwnedCompiledGuest,\n", 1),
    ("        Ok(Self { runtime, compiled, schema, pack_schema_hash: OnceLock::new() })", "        let compiled = super::owned_compiled(&compiled)?.clone();\n        Ok(Self { runtime, compiled, schema, pack_schema_hash: OnceLock::new() })", 1),
])

cold_path = HOSTDIR / "📥️cold-pair/🦀️.rs"
cold = cold_path.read_text()
edit(cold_path, [
    (cut(cold, "pub(super) fn validate_cold_page(", "\n}\n") + "\n", "", 1),
    ("use super::*;\n", "use super::*;\nuse semio_framework_plugin_interpreter::validate_cold_page;\n", 1),
])

lifecycle_path = HOSTDIR / "🔁️lifecycle/🧪️tests/🔁️lifecycle/🦀️.rs"
edit(lifecycle_path, [("use super::*;\n", "use super::*;\nuse semio_framework_plugin_interpreter::{decode_owned_result, GUEST_FAULT_MAXIMUM_BYTES};\n", 1)])

open_path = HOSTDIR / "🧪️tests/🔬️owned-instance-open/🦀️.rs"
opened = open_path.read_text()
trapped = cut(opened, "#[semio_framework_async_macros::async_test]\nasync fn a_trapped_owned_instance_refuses_every_later_turn() {", "\n}\n")
origin = cut(opened, "//#region 🧊️CodecOrigin\n", "//#endregion 🧊️CodecOrigin\n")
compiles = 'let compiled = runtime.compile(&package_ref("semio:neutral-host-fixture", &bytes), &bytes).await.expect("compile plugin component");'
opened = edit(open_path, [
    ("//! 🧫️ Private host interpreter and origin laws run against the neutral SDK component.", "//! 🧫️ Owned interpreter laws through the host's public runtime surface, and the owned/JIT codec differentials, run against\n//! the neutral SDK component.", 1),
    (trapped + "\n", "", 1),
    (origin, "", 1),
    ('    let owned_compiled = owned.compile(&package_ref("semio:neutral-host-fixture", &bytes), &bytes).await.expect("compile plugin component");', '    let owned_compiled = owned.compile_component(&bytes).expect("compile plugin component");', 1),
])
for name in ["owned_codec_genesis_answers_on_a_real_plugin_component", "owned_codec_genesis_answers_by_artifact_kind_too", "owned_codec_pack_schema_hash_answers_on_a_real_plugin_component", "owned_codec_print_mirror_round_trips_a_genesis_pair", "owned_codec_genesis_completes_under_the_fixture_budget"]:
    body = cut(opened, f"async fn {name}() {{", "\n}\n")
    opened = edit(open_path, [(body, body.replace(compiles, 'let compiled = runtime.compile_component(&bytes).expect("compile plugin component");'), 1)], opened)
assert opened.count(compiles) == 1, "only the trait-driven mid-flight law compiles through GuestRuntime"

lease_path = HOSTDIR / "🧪️tests/🔬️owned-instance-open/🪶️lease/🦀️.rs"
lease = lease_path.read_text()
edit(lease_path, [
    ("async fn export_file(runtime:&OwnedRuntime,compiled:&CompiledHandle,", "async fn export_file(runtime:&OwnedRuntime,compiled:&OwnedCompiledGuest,", 1),
    ('let compiled=runtime.compile(&package_ref("semio:neutral-host-fixture",&component),&component).await.expect("actual component compile");', 'let compiled=runtime.compile_component(&component).expect("actual component compile");', 1),
    ('let compiled=runtime.compile(&package_ref("semio:neutral-host-fixture",&component),&component).await.unwrap();', 'let compiled=runtime.compile_component(&component).unwrap();', 2),
])

pending = [p for p in writes]
if "--write" not in sys.argv:
    for path in pending:
        print(f"[dry-run] would write {path.relative_to(PLUGIN)} ({len(writes[path])} bytes)")
    sys.exit(0)

subprocess.run([sys.executable, "-I", str(TICKET / "🧬️extract-runtime.py"), "--write"], check=True)
operation = RUNTIME / "🎛️operation"
for name in ["📥️input", "📤️checkpoint"]:
    shutil.move(str(HOSTDIR / "🚪️io/🎛️operation" / name), str(operation / name))
for path, pairs in [
    (operation / "📥️input/🦀️.rs", [("pub(crate) fn encode_json_input_into<", "pub fn encode_json_input_into<", 1)]),
    (operation / "📤️checkpoint/🦀️.rs", [("use crate::os_vcs::io::binary::entity_identity::control::", "use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::", 1)]),
    (operation / "📤️checkpoint/🧪️tests/🦀️.rs", [("crate::os_vcs::io::binary::entity_identity::control::", "semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::", 2)]),
]:
    edit(path, pairs)
    path.write_text(writes.pop(path))
operation_module = HOSTDIR / "🚪️io/🎛️operation/🦀️.rs"
edit(operation_module, [('#[path="📥️input/🦀️.rs"]\npub(crate) mod input;\n#[path="📤️checkpoint/🦀️.rs"]\npub(crate) mod checkpoint;\n', "", 1)])
operation_module.write_text(writes.pop(operation_module))
for path, text in writes.items():
    path.write_text(text)
(HOSTDIR / "🧪️tests/🔬️owned-runtime/🦀️.rs").unlink()
(HOSTDIR / "🧪️tests/🔬️owned-runtime").rmdir()
print(f"[rewire] wrote {len(writes)} host files, moved operation framing, removed host owned-runtime tests")
