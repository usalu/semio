"""🧬️ Extracts the owned guest runtime and the shared guest-runtime vocabulary out of the plugin host root into
`🔌️plugin/🧠️interpreter/🏃️runtime/🦀️.rs`, rewriting the owned API onto owned-only types. Every slice is cut at exact
anchors of the CURRENT host source and every rewrite asserts its match count, so a concurrent edit fails loudly."""
import pathlib, sys

PLUGIN = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
HOST = PLUGIN / "🖥️host/🦀️.rs"
COLD = PLUGIN / "🖥️host/📥️cold-pair/🦀️.rs"
OUT = PLUGIN / "🧠️interpreter/🏃️runtime/🦀️.rs"


def cut(text, start, end, include_end=True):
    a = text.index(start)
    assert text.count(start) == 1, start
    b = text.index(end, a)
    return text[a:b + (len(end) if include_end else 0)]


def rewrite(text, pairs):
    for old, new, count in pairs:
        found = text.count(old)
        assert found == count, f"{found} != {count}: {old[:120]!r}"
        text = text.replace(old, new)
    return text


host = HOST.read_text()
cold = COLD.read_text()

errors = cut(host, "/// 🧯️ Errors from the host's engine/component/call-boundary plumbing", "\n//#endregion ⚠️ Errors", include_end=False)
vocabulary = cut(host, "/// ⛽️ `jobs.wit`'s `job-budget` record", "\nfn decode_guest_plugin_error", include_end=False)
from_host = cut(host, "impl From<PluginHostError> for TurnFault {", "\n}\n")
owned = cut(host, "//#region 🧠️OwnedRuntime\n", "//#endregion 🧠️OwnedRuntime\n")
diagnostics = cut(host, "/// 🩺️ How much guest stderr one instance retains while diagnostics are armed.", "const GUEST_DIAGNOSTICS_CAPACITY_BYTES: usize = 4 * 1024 * 1024;\n")
cold_page = cut(cold, "pub(super) fn validate_cold_page(", "\n}\n")

errors = rewrite(errors, [
    ("/// 🧯️ Errors from the host's engine/component/call-boundary plumbing (`SharedWasmtimeEngine`,\n/// `WasmtimeRuntime`, `PluginInstanceHandle`'s post-turn job dispatch).\n/// 🗝️ Conflicting ownership of one composer route.\n", "/// 🗝️ Conflicting ownership of one composer route.\n", 1),
    ("#[derive(Debug)]\npub enum PluginHostError {", "/// 🧯️ Errors from a guest runtime's host plumbing: the engine, the compiled component, the call boundary and the\n/// host routers that admit what a guest declares.\n#[derive(Debug)]\npub enum PluginHostError {", 1),
])

vocabulary = rewrite(vocabulary, [
    ("    ActorCall(crate::reactor::original_actor_context::OriginalActorFixedCall),", "    ActorCall(original_actor_context::OriginalActorFixedCall),", 1),
    ("const GUEST_FAULT_MAXIMUM_BYTES: usize = 65_536;\n\nfn decode_guest_fault_bytes(bytes: &[u8]) -> TurnFault {",
     "/// 📏️ The largest encoded guest fault a host decodes; a larger one is refused as a host fault, never trusted.\npub const GUEST_FAULT_MAXIMUM_BYTES: usize = 65_536;\n\n/// 🧯️ Decodes the bounded fault bytes a guest export returned into its typed [`TurnFault::Guest`].\npub fn decode_guest_fault_bytes(bytes: &[u8]) -> TurnFault {", 1),
    ("(`OWNED_STEP_FUEL` instructions)", "([`OWNED_STEP_FUEL`] instructions)", 1),
])

diagnostics = rewrite(diagnostics, [
    ("const GUEST_DIAGNOSTICS_CAPACITY_BYTES", "pub const GUEST_DIAGNOSTICS_CAPACITY_BYTES", 1),
])

cold_page = rewrite(cold_page, [
    ("pub(super) fn validate_cold_page(page: &ColdDocumentPairPage)", "/// 🧊️ Admits one native cold-pair page: its length is the one its header assigns to its index and its baseline frontier\n/// carries a verified chain. Both runtimes run it before a page reaches a guest.\npub fn validate_cold_page(page: &semio_framework::kernel::ColdDocumentPairPage)", 1),
    (".map_err(invalid)?;", ".map_err(|reason| TurnFault::Trapped(reason.to_owned()))?;", 1),
    ("return Err(invalid(\"cold-pair page length or verified frontier is invalid\"));", "return Err(TurnFault::Trapped(\"cold-pair page length or verified frontier is invalid\".to_owned()));", 1),
])

owned = rewrite(owned, [
    ("//#region 🧠️OwnedRuntime\n\nconst OWNED_RESULT_LIMIT", "//#region 🧠️OwnedRuntime\n/// 📦️ The largest result bytes one owned export may return.\nconst OWNED_RESULT_LIMIT", 1),
    ("const OWNED_CHECKPOINT_MAGIC: &[u8; 8] = b\"SMOWNH01\";\nconst OWNED_STEP_FUEL: u64 = 4_096;\nconst OWNED_SLICE_DEADLINE_MS: u32 = 8;",
     "/// 🪪️ The header of an owned host checkpoint frame.\nconst OWNED_CHECKPOINT_MAGIC: &[u8; 8] = b\"SMOWNH01\";\n/// ⛽️ The instructions one interpreter step may run before the host looks at its deadline, cancellation and progress.\npub const OWNED_STEP_FUEL: u64 = 4_096;\n/// ⏱️ The wall bound of one owned job cancellation slice.\nconst OWNED_SLICE_DEADLINE_MS: u32 = 8;", 1),
    ("/// once one of them assembled it.\nstruct OwnedCompiledGuest {\n    artifact: Arc<OwnedSemioArtifact>,\n    codec_origin: OwnedCodecOriginCell,\n}\n",
     "/// once one of them assembled it. A clone shares both, so every holder of one compiled guest starts its codec calls from the\n/// same origin.\n#[derive(Clone)]\npub struct OwnedCompiledGuest {\n    artifact: Arc<OwnedSemioArtifact>,\n    codec_origin: Arc<OwnedCodecOriginCell>,\n}\n\nimpl OwnedCompiledGuest {\n    /// 📏️ The bytes this compiled guest holds beyond its parsed component: its codec origin once a codec call assembled it\n    /// ([`OwnedCodecOriginCell`]), else none. A residency budget charges a compiled guest this on top of its component.\n    pub fn codec_origin_bytes(&self) -> u64 {\n        self.codec_origin.resident_bytes()\n    }\n}\n", 1),
    ("struct OwnedInstanceState {\n", "/// 🏃️ One instance of an owned compiled guest: its interpreter state, the operation it has in flight, the original receipt\n/// that operation keeps, and what the guest printed.\npub struct OwnedGuestInstance {\n", 1),
    ("OwnedInstanceState", "OwnedGuestInstance", 14),
    ("#[derive(serde::Serialize)]\nstruct OwnedSnapshotInput<'a> {\n    dialect: &'a str,\n    encoding: &'a str,\n    payload: &'a [u8],\n    limits: sqlite_wire::SnapshotLimits,\n    native: sqlite_wire::SnapshotGrant,\n}\n",
     "/// 🪶️ The input envelope of the owned `codec.sqlite-export`/`sqlite-import` exports — the mirror of the guest's own snapshot\n/// input.\n#[derive(serde::Serialize)]\npub struct OwnedSnapshotInput<'a> {\n    pub dialect: &'a str,\n    pub encoding: &'a str,\n    pub payload: &'a [u8],\n    pub limits: sqlite_wire::SnapshotLimits,\n    pub native: sqlite_wire::SnapshotGrant,\n}\n", 1),
    ("struct OwnedInvocation {\n    retained_input: Option<Vec<u8>>,\n    output: Vec<u8>,\n    fuel_used: u64,\n}\n",
     "/// 🧾️ One completed owned export: the input bytes the call retained for its caller, the result bytes and the fuel it ran.\npub struct OwnedInvocation {\n    pub retained_input: Option<Vec<u8>>,\n    pub output: Vec<u8>,\n    pub fuel_used: u64,\n}\n", 1),
    ("        let interpreter = include_str!(\"../🧠️interpreter/🦀️.rs\").as_bytes();", "        let interpreter = include_str!(\"../🦀️.rs\").as_bytes();", 1),
    ("        format!(\"owned-engine-v1:{}\", hex_encode(semio_framework_hash::hash(&engine).as_bytes()))", "        format!(\"owned-engine-v1:{}\", semio_framework_hash::hex_lower(semio_framework_hash::hash(&engine).as_bytes()))", 1),
    ("pub struct OwnedRuntime {\n", "/// 🏃️ The owned interpreter as a guest runtime: compiles components, instantiates actors and runs their turns, jobs,\n/// checkpoints and pure `codec` exports under instruction fuel. Holds no engine; instance ids are its only state.\npub struct OwnedRuntime {\n", 1),
    ("    pub fn new() -> Self {\n        Self { next_instance_id", "    /// 🌱️ A runtime whose first instance is id 1.\n    pub fn new() -> Self {\n        Self { next_instance_id", 1),
    ("    pub fn compile_component(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {\n        let artifact = OwnedSemioArtifact::parse(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;\n        Ok(CompiledHandle { package_hash: package.hash.0, component: None, owned: Some(Arc::new(OwnedCompiledGuest { artifact: Arc::new(artifact), codec_origin: OwnedCodecOriginCell::new() })) })\n    }\n",
     "    /// 🧩️ Parses `bytes` as an owned-ABI component, ready to instantiate; no codec origin is assembled yet.\n    pub fn compile_component(&self, bytes: &[u8]) -> Result<OwnedCompiledGuest, PluginHostError> {\n        let artifact = OwnedSemioArtifact::parse(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;\n        Ok(OwnedCompiledGuest { artifact: Arc::new(artifact), codec_origin: Arc::new(OwnedCodecOriginCell::new()) })\n    }\n", 1),
    ("    pub fn instantiate_actor(&self, compiled: &CompiledHandle, actor: RuntimeActorId) -> Result<GuestInstance, PluginHostError> {\n        let artifact = &owned_compiled_guest(compiled)?.artifact;\n",
     "    /// 🏃️ A fresh instance of `compiled` whose app channel is admitted ([`admit_owned_channel`]).\n    pub fn instantiate_actor(&self, compiled: &OwnedCompiledGuest) -> Result<OwnedGuestInstance, PluginHostError> {\n        let artifact = &compiled.artifact;\n", 1),
    ("        admit_owned_channel(&mut state).map_err(turn_fault_host)?;\n        Ok(GuestInstance { actor, state: GuestInstanceState::Owned(state) })\n    }\n",
     "        admit_owned_channel(&mut state).map_err(turn_fault_host)?;\n        Ok(state)\n    }\n", 1),
    ("        let cell = &owned.codec_origin;\n", "        let cell: &OwnedCodecOriginCell = &owned.codec_origin;\n", 1),
    ("    /// 🧬️ A throwaway instance of `owned` in its codec origin's state.\n    fn codec_instance(&self, owned: &OwnedCompiledGuest, origin: &OwnedCodecOrigin) -> GuestInstance {\n        let instance_id = self.next_instance_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);\n        let state = OwnedGuestInstance { artifact: Arc::clone(&owned.artifact), actor: origin.actor.clone(), pending: None, original_receipt: None, poisoned: false, diagnostics: origin.diagnostics.clone(), context: origin.context, next_resource: origin.next_resource, instance_id };\n        GuestInstance { actor: RuntimeActorId(0), state: GuestInstanceState::Owned(state) }\n    }\n",
     "    /// 🧬️ A throwaway instance of `owned` in its codec origin's state.\n    fn origin_instance(&self, owned: &OwnedCompiledGuest, origin: &OwnedCodecOrigin) -> OwnedGuestInstance {\n        let instance_id = self.next_instance_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);\n        OwnedGuestInstance { artifact: Arc::clone(&owned.artifact), actor: origin.actor.clone(), pending: None, original_receipt: None, poisoned: false, diagnostics: origin.diagnostics.clone(), context: origin.context, next_resource: origin.next_resource, instance_id }\n    }\n\n    /// 🧬️ A throwaway instance of `compiled` in its codec origin's state, for a caller that drives its own codec operation on\n    /// it (a retained SQLite snapshot call), with the fuel this call spent assembling the origin ([`Self::codec_origin`]).\n    pub fn codec_instance(&self, compiled: &OwnedCompiledGuest, budget: Budget, progress: &mut impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>) -> Result<(OwnedGuestInstance, u64), TurnFault> {\n        let used = self.codec_origin(compiled, budget, progress, cancellation)?;\n        Ok((self.origin_instance(compiled, &used.origin), used.spent_fuel))\n    }\n", 1),
    ("    fn actor_context_call(&self,inst:&mut GuestInstance,operation:OwnedOperation,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<crate::reactor::original_actor_context::OriginalActorFixedCall,TurnFault>{\n        let state=owned_state_mut(inst)?;\n",
     "    fn actor_context_call(&self,state:&mut OwnedGuestInstance,operation:OwnedOperation,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<original_actor_context::OriginalActorFixedCall,TurnFault>{\n", 1),
    ("crate::reactor::original_actor_context::", "original_actor_context::", 8),
    ("    pub fn execute_actor_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault> {\n        let state = owned_state_mut(inst)?;\n",
     "    /// 🎬️ Runs one actor turn over `events`, or resumes the turn `state` has in flight when `events` is empty.\n    pub fn execute_actor_turn(&self, state: &mut OwnedGuestInstance, events: &[Event], budget: Budget, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault> {\n", 1),
    ("    pub fn drop_actor(&self, _inst: GuestInstance) {}\n\n", "", 1),
    ("    pub async fn describe(&self, compiled: &CompiledHandle, budget: Budget) -> Result<Vec<u8>, TurnFault> {", "    /// 🛂️ Executes owned `describe` on a fresh instance of `compiled`.\n    pub async fn describe(&self, compiled: &OwnedCompiledGuest, budget: Budget) -> Result<Vec<u8>, TurnFault> {", 1),
    ("    fn codec_call<T: serde::de::DeserializeOwned, I: serde::Serialize>(&self, compiled: &CompiledHandle, operation: OwnedOperation, input: &I, budget: Budget, mut progress: impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>) -> Result<T, TurnFault> {\n        let owned = owned_compiled_guest(compiled)?;\n        let used = self.codec_origin(owned, budget, &mut progress, cancellation)?;\n        let mut instance = self.codec_instance(owned, &used.origin);\n        let state = owned_state_mut(&mut instance)?;\n",
     "    fn codec_call<T: serde::de::DeserializeOwned, I: serde::Serialize>(&self, compiled: &OwnedCompiledGuest, operation: OwnedOperation, input: &I, budget: Budget, mut progress: impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>) -> Result<T, TurnFault> {\n        let used = self.codec_origin(compiled, budget, &mut progress, cancellation)?;\n        let state = &mut self.origin_instance(compiled, &used.origin);\n", 1),
    ("(&self,compiled:&CompiledHandle,operation:OwnedOperation,input:&I,budget:Budget,mut progress:impl FnMut(u64,std::time::Duration),cancellation:Option<&GuestCallCancellation>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>)->Result<T,TurnFault>{\n        let owned=owned_compiled_guest(compiled)?;\n        let used=self.codec_origin(owned,budget,&mut progress,cancellation)?;\n        let mut instance=self.codec_instance(owned,&used.origin);\n        let state=owned_state_mut(&mut instance)?;\n",
     "(&self,compiled:&OwnedCompiledGuest,operation:OwnedOperation,input:&I,budget:Budget,mut progress:impl FnMut(u64,std::time::Duration),cancellation:Option<&GuestCallCancellation>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>)->Result<T,TurnFault>{\n        let used=self.codec_origin(compiled,budget,&mut progress,cancellation)?;\n        let state=&mut self.origin_instance(compiled,&used.origin);\n", 1),
    ("compiled: &CompiledHandle", "compiled: &OwnedCompiledGuest", 15),
    ("    pub async fn describe_observed(&self, compiled: &OwnedCompiledGuest, budget: Budget, progress: impl FnMut(u64, std::time::Duration)) -> Result<Vec<u8>, TurnFault> {\n        let mut instance = self.instantiate_actor(compiled, RuntimeActorId(0)).map_err(TurnFault::Host)?;\n        let state = owned_state_mut(&mut instance)?;\n",
     "    pub async fn describe_observed(&self, compiled: &OwnedCompiledGuest, budget: Budget, progress: impl FnMut(u64, std::time::Duration)) -> Result<Vec<u8>, TurnFault> {\n        let state = &mut self.instantiate_actor(compiled).map_err(TurnFault::Host)?;\n", 1),
    ("    async fn admit_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<original_actor_context::OriginalActorAdmissionReply,TurnFault>{let call=self.actor_context_call(inst,",
     "    /// 🎟️ Admits the original actor context of `state` for one retained turn.\n    pub fn admit_context(&self,state:&mut OwnedGuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<original_actor_context::OriginalActorAdmissionReply,TurnFault>{let call=self.actor_context_call(state,", 1),
    ("    async fn close_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<original_actor_context::OriginalActorCloseReply,TurnFault>{let call=self.actor_context_call(inst,",
     "    /// ♻️ Runs one closing step of the original actor context of `state`.\n    pub fn close_context(&self,state:&mut OwnedGuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<original_actor_context::OriginalActorCloseReply,TurnFault>{let call=self.actor_context_call(state,", 1),
    ("\n    async fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {\n        self.compile_component(package, bytes)\n    }\n\n    async fn instantiate(&self, compiled: &OwnedCompiledGuest, actor: RuntimeActorId, _caps: &[BrokerCapabilityGrant], _budget: &Budget) -> Result<GuestInstance, PluginHostError> {\n        self.instantiate_actor(compiled, actor)\n    }\n\n    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault> {\n        self.execute_actor_turn(inst, events, budget, identity)\n    }\n", "", 1),
    ("    async fn start_job(&self, inst: &mut GuestInstance, job: u64, kind: &str, input: Vec<u8>) -> Result<(), TurnFault> {\n        let state = owned_state_mut(inst)?;\n",
     "    /// 👷️ Starts guest job `job` of `kind` on `state`.\n    pub fn start_actor_job(&self, state: &mut OwnedGuestInstance, job: u64, kind: &str, input: Vec<u8>) -> Result<(), TurnFault> {\n", 1),
    ("    async fn step_job(&self, inst: &mut GuestInstance, job: u64, budget: JobBudget) -> Result<JobStep, TurnFault> {\n        let state = owned_state_mut(inst)?;\n",
     "    /// 🪜️ Steps guest job `job` of `state` once under `budget`.\n    pub fn step_actor_job(&self, state: &mut OwnedGuestInstance, job: u64, budget: JobBudget) -> Result<JobStep, TurnFault> {\n", 1),
    ("    async fn cancel_job(&self, inst: &mut GuestInstance, job: u64) -> Result<(), TurnFault> {\n        let state = owned_state_mut(inst)?;\n",
     "    /// 🛑️ Cancels guest job `job` of `state`, abandoning whatever owned operation is in flight first.\n    pub fn cancel_actor_job(&self, state: &mut OwnedGuestInstance, job: u64) -> Result<(), TurnFault> {\n", 1),
    ("    async fn checkpoint(&self, inst: &mut GuestInstance, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<u8>, PluginHostError> {\n        let state = match &mut inst.state {\n            GuestInstanceState::Owned(state) => state,\n            _ => return Err(PluginHostError::Plugin(\"checkpoint called on a non-owned GuestInstance\".to_string())),\n        };\n",
     "    /// 📸️ Frames `state` — its in-flight operation, original receipt, guest checkpoint and interpreter memory — as one owned\n    /// host checkpoint.\n    pub fn checkpoint_actor(&self, state: &mut OwnedGuestInstance, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<u8>, PluginHostError> {\n", 1),
    ("    async fn restore(&self, inst: &mut GuestInstance, checkpoint: &[u8], _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), PluginHostError> {\n        let state = match &mut inst.state {\n            GuestInstanceState::Owned(state) => state,\n            _ => return Err(PluginHostError::Plugin(\"restore called on a non-owned GuestInstance\".to_string())),\n        };\n",
     "    /// ⏮️ Restores `state` from an owned host checkpoint [`Self::checkpoint_actor`] framed.\n    pub fn restore_actor(&self, state: &mut OwnedGuestInstance, checkpoint: &[u8]) -> Result<(), PluginHostError> {\n", 1),
    ("        Ok(())\n    }\n\n    async fn drop_instance(&self, inst: GuestInstance) {\n        self.drop_actor(inst);\n    }\n}\n\nfn owned_compiled_guest(compiled: &OwnedCompiledGuest) -> Result<&OwnedCompiledGuest, PluginHostError> {\n    compiled.owned.as_deref().ok_or_else(|| PluginHostError::Plugin(\"CompiledHandle has no repository-owned actor artifact\".to_string()))\n}\n\nfn owned_state_mut(inst: &mut GuestInstance) -> Result<&mut OwnedGuestInstance, TurnFault> {\n    match &mut inst.state {\n        GuestInstanceState::Owned(state) => Ok(state),\n        _ => Err(TurnFault::Trapped(\"owned runtime called on a non-owned GuestInstance\".to_string())),\n    }\n}\n",
     "        Ok(())\n    }\n}\n", 1),
    ("\n}\n\nimpl GuestRuntime for OwnedRuntime {\n", "\n\n", 1),
    ("fn identity_turn_fault(error:semio_framework_value::ValueError)->TurnFault{TurnFault::Host(PluginHostError::NativeIo(error))}\nfn receiving_turn_fault(error:operation::wasmtime::ReceiveError)->TurnFault{match error{operation::wasmtime::ReceiveError::Refused(error)=>identity_turn_fault(error),operation::wasmtime::ReceiveError::Runtime(error)=>TurnFault::Trapped(error.to_string())}}\n",
     "/// 🛂️ A refused original native admission, as the host fault every runtime reports it as.\npub fn identity_turn_fault(error:semio_framework_value::ValueError)->TurnFault{TurnFault::Host(PluginHostError::NativeIo(error))}\n", 1),
    ("/// ([`protocol::admit_guest_channel_version`]) before", "/// ([`semio_framework_os_kernel::admit_guest_channel_version`]) before", 1),
    ("    protocol::admit_guest_channel_version(guest, protocol::CHANNEL_VERSION)", "    semio_framework_os_kernel::admit_guest_channel_version(guest, semio_framework_os_kernel::CHANNEL_VERSION)", 1),
    ("fn decode_owned_result<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, TurnFault> {", "/// 📥️ Decodes an owned export's JSON `Result` envelope; its error arm is the guest's typed fault bytes.\npub fn decode_owned_result<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, TurnFault> {", 1),
])

owned = rewrite(owned, [(cut(owned, "fn snapshot_component_transport<", "\n}\n"), "", 1)])
owned = owned.replace("\n\n\n/// 📦️ One document's authoritative", "\n\n/// 📦️ One document's authoritative")

instance_api = '''
impl OwnedGuestInstance {
    /// 🔁️ True while an owned operation has not completed — the guest yielded on its fuel or deadline and owns the host's next
    /// call.
    pub fn turn_in_flight(&self) -> bool {
        self.pending.is_some()
    }

    /// ☠️ True once a guest trap left this instance unusable; every later call refuses.
    pub fn poisoned(&self) -> bool {
        self.poisoned
    }

    /// 🩺️ Everything this guest has written to `wasi:cli/stdout`/`stderr`, bounded by [`GUEST_DIAGNOSTICS_CAPACITY_BYTES`].
    pub fn diagnostics(&self) -> &[u8] {
        &self.diagnostics
    }

    /// ⏸️ The snapshot operation this instance has in flight and the fuel it ran so far; refused when another owned operation
    /// is in flight.
    pub fn pending_snapshot(&self) -> Result<Option<(OwnedSnapshotOperation, u64)>, TurnFault> {
        self.pending.as_ref().map(|pending| OwnedSnapshotOperation::from_operation(pending.operation).map(|operation| (operation, pending.fuel_used)).ok_or_else(|| TurnFault::Trapped(format!("owned operation {:?} is not a snapshot operation", pending.operation)))).transpose()
    }

    /// 🌱️ Begins `operation` on the input bytes `input` holds, taking them only once the begin is admitted.
    pub fn begin_snapshot(&mut self, operation: OwnedSnapshotOperation, input: &mut Option<Vec<u8>>) -> Result<(), TurnFault> {
        if self.poisoned {
            return Err(TurnFault::Trapped("owned snapshot instance is poisoned".into()));
        }
        if self.pending.is_some() {
            return Err(TurnFault::Trapped("owned snapshot operation remains pending".into()));
        }
        let bytes = input.as_ref().ok_or_else(|| identity_turn_fault(operation_authority::refusal(4)))?;
        let length = i32::try_from(bytes.len()).map_err(|_| identity_turn_fault(operation_authority::refusal(2)))?;
        self.pending = Some(OwnedPending { retained_input: None, operation: operation.operation(), stage: OwnedStage::Allocate { input: input.take().unwrap_or_default() }, fuel_used: 0 });
        self.actor.begin(OwnedSemioExport::Allocate, vec![Value::I32(length)])?;
        Ok(())
    }

    /// ▶️ Resumes the in-flight `operation` under `fuel`, bounding the gap between fuel observations by `deadline_ms`
    /// (`OwnedDeadline::NoFuelProgress`), with every original operation import answered by `receiver`.
    pub fn resume_snapshot(&mut self, operation: OwnedSnapshotOperation, fuel: u64, deadline_ms: u32, progress: impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>, receiver: &mut dyn OriginalOperationReceiving) -> Result<OwnedInvocation, TurnFault> {
        resume_owned_operation_bound(self, operation.operation(), fuel, deadline_ms, OwnedDeadline::NoFuelProgress, progress, cancellation, Some(receiver))
    }
}

/// 🪶️ The six owned `codec` SQLite snapshot exports a retained snapshot call drives on one instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OwnedSnapshotOperation {
    Export,
    Import,
    Retirement,
    Close,
    TakeFile,
    TakePayload,
}

impl OwnedSnapshotOperation {
    fn operation(self) -> OwnedOperation {
        match self {
            Self::Export => OwnedOperation::SqliteExport,
            Self::Import => OwnedOperation::SqliteImport,
            Self::Retirement => OwnedOperation::SqliteRetirement,
            Self::Close => OwnedOperation::SqliteClose,
            Self::TakeFile => OwnedOperation::SqliteTakeFile,
            Self::TakePayload => OwnedOperation::SqliteTakePayload,
        }
    }

    fn from_operation(operation: OwnedOperation) -> Option<Self> {
        match operation {
            OwnedOperation::SqliteExport => Some(Self::Export),
            OwnedOperation::SqliteImport => Some(Self::Import),
            OwnedOperation::SqliteRetirement => Some(Self::Retirement),
            OwnedOperation::SqliteClose => Some(Self::Close),
            OwnedOperation::SqliteTakeFile => Some(Self::TakeFile),
            OwnedOperation::SqliteTakePayload => Some(Self::TakePayload),
            _ => None,
        }
    }
}
'''

anchor = "\n/// 🔀️ `serde`-only, not `ToValue`"
assert owned.count(anchor) == 1
owned = owned.replace(anchor, instance_api + anchor)

header = '''//! 🏃️ The owned interpreter as a guest runtime ([`OwnedRuntime`]): compiles a `world actor` component for the
//! repository-owned interpreter ([`crate::interpreter`]), runs its actor turns, jobs, checkpoints and pure `codec` exports
//! under instruction fuel, and answers every host import a guest can observe. It also owns the guest-runtime vocabulary every
//! backend shares — [`TurnFault`], [`PluginHostError`], [`JobBudget`], [`JobStep`], [`GuestCallCancellation`] — so the
//! Wasmtime host builds on it, and a tool that only describes or verifies components links no JIT.
//! @see https://webassembly.github.io/spec/core/exec/index.html
use crate::interpreter::{CoreStepOutcome, HostCall, OwnedSemioArtifact, OwnedSemioExport, OwnedSemioInstance, StepControl, Value};
use crate::{operation_authority, original_actor_context, sqlite_wire};
use semio_framework::kernel::{Budget, Event, TurnResult};
use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceiving;
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

#[path = "🎛️operation/🦀️.rs"]
pub mod operation;

#[cfg(test)]
#[path = "../../🖥️host/🧪️tests/🛂️receiving/🦀️.rs"]
pub(crate) mod test_native_authority;

//#region 🧯️GuestFaults
'''

text = header + errors.rstrip() + "\n\n" + vocabulary.rstrip() + "\n\n" + from_host.rstrip() + "\n//#endregion 🧯️GuestFaults\n\n" + diagnostics.rstrip() + "\n\n" + cold_page.rstrip() + "\n\n" + owned
OUT.parent.mkdir(parents=True, exist_ok=True)
if "--write" in sys.argv:
    OUT.write_text(text)
else:
    sys.stdout.write(text)
print(f"[extract] errors={len(errors)} vocabulary={len(vocabulary)} owned={len(owned)} total_lines={text.count(chr(10))}", file=sys.stderr)
