# 2026-10-10 Plugin Migration Exemplar Guide (rn-os, read-from-source, NOT compiler-verified)

Status: `os.status` is RED. No `💻️os` crate can be cargo-checked yet because `semio-framework-io-sqlite-snapshot` (and `semio-framework-tool-run`) are red below the kernel (see the last section). Everything below is read from the committed store/job/value/plugin sources at c44e964 + local value completion; each claim names its file. Treat signatures as authoritative (they are copied from the traits), treat skeleton bodies as unverified sketches. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` itself is only half migrated (see "Not templates").

## 1. Currency model (applies to every close/retire/prepare path)

- `RetainedCloneGrant { maximum_items, maximum_copy_bytes, maximum_capacity_bytes, maximum_release_bytes, maximum_depth }` (value `🧬️retained-clone/🦀️.rs:113`). `RetainedCloneProgress { copied_items, copied_bytes, retained_capacity_bytes, released_bytes }` is the receipt; `progress.fits(grant)` must hold on every turn. `RetirementDemand { copy_bytes, capacity_bytes, release_bytes, depth }` is the quote a state machine publishes before it is granted anything. `RetainedCloneStep::{Progress(p), Complete(p)}`.
- A turn consumes one item and depth; physical backing release must equal `released_bytes`, allocation must equal `retained_capacity_bytes` (the repo tests assert this with `observe_retirement_allocations` / `observe_heap_allocations_on_this_thread`).
- `SnapshotRetirementStep` and `JobPayloadCloseStep` do not exist any more (no alias). `Pending{released_items,released_bytes}` becomes `Progress(RetainedCloneProgress{copied_items:released_items, released_bytes, ..})`, `Complete` becomes `Complete(progress)`, `Blocked` has no equivalent for `RetainedCloneStep` (return `Progress(Default::default())` when the grant is too small; return `Err(ValueError)` for a real refusal). Jobs keep `Blocked` (see 5).

## 2. Close state machine (anything that held `close_step(max_items, max_bytes)`)

Trait (`ErasedSnapshotRetirement`, value `♻️retirement/🧬️contract/🦀️.rs:11`):
```rust
fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
fn terminal_is_empty(&self) -> bool;
fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;
fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
fn next_release_byte_demand(&self) -> Result<usize, ValueError>;
fn next_depth_demand(&self) -> Result<usize, ValueError>;
// next_demand(maximum_body_bytes) -> RetirementDemand is provided
```
Rules visible in every migrated owner (e.g. `BoundedConfigPreparation`, plugin `🦀️.rs:~19000-19120`):
- Every owned field becomes `ManuallyDrop<Option<T>>` (or `Option<T>` when it is a plain handle) and the type gets a `Drop` that asserts `std::thread::panicking() || terminal_is_empty()` before dropping.
- `fn close_demands(&self, body) -> Result<RetirementDemand, ValueError>` returns the demand of the field that the NEXT turn will retire (first non-empty field in a fixed order). Nested owners add 1 to `depth` (`nested(demand)`). The four `next_*_demand` methods read that single quote.
- `close_step`: if `terminal_is_empty()` return `Complete(default)`; if `grant.maximum_items==0` return `Progress(default)`; compute `demand`, refuse with `Err(DepthLimit)` when `grant.maximum_depth < demand.depth`, return `Progress(default)` when another axis is short; retire exactly one field per turn; pass children `RetainedCloneGrant{ maximum_depth: grant.maximum_depth-1, ..grant }`; admit a child's receipt with `semio_framework_value::retained_clone::admit_retained_clone_close(child_grant, step, child.terminal_is_empty(), "label")`.
- Caller loop (sync callers, e.g. `🖥️host/🦀️.rs close_backbone_envelope`): ask `retirement.next_demand(body)`, build `grant = {maximum_items:1, copy/capacity/release = demand.*, maximum_depth: demand.depth.max(1)}`, call `close_step(grant)`, stop at `Complete`.

## 3. Owned-value retirement (`owned_retirement(v)` and factories)

- `semio_framework_value::retirement::owned_retirement_birth_bytes::<T>()` = capacity the birth will allocate. `admit_owned_retirement(value, grant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, T)>` returns the value on refusal.
- `ArtifactOwnedValueRetirementFactory<T>`: `retirement_birth_bytes(&self, &T) -> usize` and `retire_owned(&self, value: T, grant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, T)>`. Canonical impl (os `🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs:165`): `fn retirement_birth_bytes(&self,_:&T)->usize{owned_retirement_birth_bytes::<T>()}` and `fn retire_owned(&self,v:T,g:RetainedCloneGrant)->Result<..>{admit_owned_retirement(v,g)}`. `T` must be `RetireOwned` (`#[derive(semio_framework_value::RetireOwned)]`, value `✨️derive/🦀️.rs:92`).
- `SnapshotRetirementFactory<P>`: `retirement_birth_bytes(&self, &Arc<P>) -> usize`, `retire(&self, Arc<P>, grant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<P>)>`.
- Envelope owners (store): `owners.uninstalled_envelope_retirement_demands(&envelope)` quotes the birth; `owners.retire_envelope_uninstalled(envelope, grant) -> Result<(Box<dyn ErasedSnapshotRetirement>, Progress), (ValueError, Self, ArtifactEnvelope)>`; `owners.retire_envelope(&self, envelope, grant)` for installed catalogs.

## 4. One-item preparation (the contract every plugin editor implements)

Store source: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18600-18740`.

Mutation type bounds: the store seals every edit through `ArtifactStoreOneItemSealer`, so `Mutation: Clone + RetireOwned + Sync + ArtifactCanonicalJsonTree` (+ the old `OpBinary/OpText/Mutation<P>/ToValue/FromValue`). Add the canonical tree to every mutation type:
```rust
#[derive(Clone, semio_framework_value::ToValue, semio_framework_value::FromValue,
         semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum MyMutation { /* ... */ }
```
`CanonicalJsonTree` (value `✨️derive/🧵️canonical/🦀️.rs`) projects fields by the same `#[value(..)]` wire roles; it refuses custom serializers/`flatten`/`with`, requires `owner` explicitly, supports records, newtypes, tagged enums (`tag`, `content`), `skip_serializing_if` of `Option::is_none`/`Vec::is_empty`/`String::is_empty`/`PagedList::is_empty`/`PagedUtf8::is_empty`. Hand-written impl template (trait in `🎒️pack/🔤️json/🛫️encode/🧭️tree/🦀️.rs:11`): `canonical_tree_node` returns `Node::{Null,Bool,I64,U64,F64,String(&str),Text,Array(len),Object(len)}`, `canonical_tree_child(ordinal)` and `canonical_tree_key(ordinal)` address array elements / object fields by ordinal. Blanket impls exist for `String, &'static str, bool, u8, i32, i64, u64, usize, f64, Vec<T>, [T;N], Option<T>, Box<T>, PagedList, PagedMap, PagedUtf8`. Float/other leaf types without an impl need one.

Request (three generics now; `Input` = `Mutation` for typed lanes):
```rust
pub struct ArtifactStoreOneItemPreparationRequest<P, Input, Mutation> {
    pub operation, pub generation, pub base_revision: [u8;32], pub lane: HistoryLane,
    pub authority: Arc<ArtifactStoreOneItemLiveAuthority>, pub base: SnapshotRead<P>,
    pub mutation: Input,
    pub mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,   // NEW
    pub snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>,                     // NEW
}
```
Factory trait (all required unless defaulted):
```rust
pub trait ArtifactStoreOneItemPreparationFactory<P, Mutation>: FactoryRetirement + Send + Sync {
    fn begin_batch_digest(&self, edit: &mut Option<Box<Edit<Mutation>>>, grant: RetainedCloneGrant)
        -> Result<Option<(Box<dyn ArtifactStoreBatchDigest<Mutation>>, RetainedCloneProgress)>, ValueError>;  // NEW
    fn preflight(&self, mutation: &Mutation, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String>;
    fn begin_demand(&self, mutation: &Mutation, lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError>; // NEW
    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, Mutation, Mutation>, grant: ArtifactStoreOneItemGrant)
        -> Result<(Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>, RetainedCloneProgress),
                  (ValueError, ArtifactStoreOneItemPreparationRequest<P, Mutation, Mutation>)>;            // CHANGED
    // optional: operation_wire_source, operation_schema_parts, stamped_clock, stamped_mutation_id
}
```
`ArtifactStoreOneItemGrant` is now the five-currency grant (`maximum_items, maximum_copy_bytes, maximum_capacity_bytes, maximum_release_bytes, maximum_depth`), with `permits_one()` and `retained_grant()`; `maximum_bytes` is gone.

Minimal skeleton (after), copied in shape from the plugin's own `BoundedConfigPreparationFactory` (plugin `🦀️.rs:~18995`):
```rust
impl<P, M> store::ArtifactStoreOneItemPreparationFactory<P, M> for MyFactory<P, M> where /* P,M: RetireOwned + Send + Sync + 'static, M: ArtifactCanonicalJsonTree + .. */ {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<M>>>, grant: RetainedCloneGrant)
        -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<M>>, RetainedCloneProgress)>, ValueError>
    { store::admit_artifact_batch_digest(edit, grant) }
    fn preflight(&self, m: &M, lane: HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> { /* unchanged */ }
    fn begin_demand(&self, _m: &M, _lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError> {
        Ok(RetainedCloneBirthDemand { capacity_bytes: std::mem::size_of::<MyPreparation<P, M>>(), depth: 1 })
    }
    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M, M>, grant: store::ArtifactStoreOneItemGrant)
        -> Result<(Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, RetainedCloneProgress), (ValueError, store::ArtifactStoreOneItemPreparationRequest<P, M, M>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(d) => d, Err(e) => return Err((e, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(p) => p, Err(e) => return Err((e, request)) };
        Ok((Box::new(MyPreparation { /* base, mutation, authority, snapshot_factory: Some(request.snapshot_retirement), mutation_factory: Some(request.mutation_retirement), .. */ }), progress))
    }
}
```
Preparation trait (`18642`): `advance(grant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError>` where the step is `Progress(checkpoint, RetainedCloneProgress) | Prepared(checkpoint, RetainedCloneProgress) | Blocked`; `close_step(grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError>`; four `next_close_{copy,capacity,release,depth}_byte/..._demand`; `terminal_is_empty()`. The ephemeral/presence twin (`ArtifactEphemeralOneItemPreparation*`, `5100-5160`) has the same close/demand shape and still a two-generic request (`operation, generation, base, mutation`).

Close order the exemplar uses (one field per turn): active child retirement -> returned read witness -> returned factory aliases -> `prepared` (`prepared.admit_retirement(mutation_factory, snapshot_factory, child_grant)`) -> `mutation` (`FactoryOwnedRetirement::admit_original`) -> `base` (`base.try_return_to_registry_witness()` into a `ControlledRetirement`) -> `authority.retire(child_grant)` -> the two factory `Arc`s into `FactoryAuthority::new(..)` slots, each stepped to empty.

## 5. Interactive jobs (`InteractiveJob`, job `🦀️.rs:1289`)

```rust
fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError>;   // was -> StepOutcome
fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError>; // NEW
fn begin_close(&mut self);
fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep;   // was (max_items, max_bytes) -> JobPayloadCloseStep
fn next_close_{copy_byte,capacity_byte,release_byte,depth}_demand(..) -> Result<usize, ValueError>;  // defaults refuse unless terminal_is_empty()
fn terminal_is_empty(&self) -> bool;
```
- `InteractiveJobCloseStep::{Pending{progress}, Blocked, Complete{progress}, Refused{kind, progress}}`; `Refused` and `Complete` are struct variants now (`WorkerJobCloseStep` is the same shape). Wrap a child step with `step.admit(grant, child.terminal_is_empty())`.
- Outcomes are borrowed from the job: a job owns a `RetainedJobPublication` (`📬️outcome/📤️publication`) and calls `publication.advance_from_source(JobPublicationKind::{Preview,Checkpoint{applied_progress},Fault}, &bytes, cx)` once per turn; it returns `Ok(None)` while it is still paying (bind source, select stream, initialise bindings, append one page, seal) and `Ok(Some(JobOutcomeBorrow))` when the sealed payload can be lent. `borrow_outcome` forwards to `publication.borrow_outcome(descriptor)`; `close_step` forwards to `publication.close_step(grant)`. Yield/Cancelled/Complete use `JobOutcomeBorrow::admit_yield(cx)`, `admit_cancelled(cx)`, `admit_complete(cx, state: Option<&RetainedJobPayload>, output: Option<&RetainedJobPayload>)`. `cx.retained_grant()` / `cx.consume_retained(progress)` are the per-step wallet. Minimal yield-only job (job `🧪️tests/🔬️clock-stride`):
```rust
fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> { cx.set_stage("stage"); JobOutcomeBorrow::admit_yield(cx) }
fn borrow_outcome<'a>(&'a self, d: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> { match d.kind() { JobOutcomeKind::Yield => d.yielded(), JobOutcomeKind::Cancelled => d.cancelled(), _ => Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "unexpected")) } }
```
- `StepOutcome` is no longer returned by `step`; `drive_step(job, cx, site, stage, &mut callback_verdict)` now takes a caller-made `StepContext` and returns `Result<Option<JobOutcomeBorrow>, ValueError>` (job `📬️outcome/🤝️loan/🦀️.rs`). `StepOutcome::close_step(grant)`, `RetainedJobPayload::close_step(grant)` and `RetainedJobPayloadWriter::close_step(grant)` return `Result<RetainedCloneStep, ValueError>`; `retirement_demands()` quotes the next turn (`♻️retirement/📄️payload/🦀️.rs`). There is no `close_step(1, JOB_PAYLOAD_PAGE_BYTES)` any more.
- `StepBudget::retained` (the incoming five-axis grant) and `StepContext::new(.., preview_sequence, retained_progress)` carry the wallet; the separate `retained_work` budget exists in the job crate but its `StepBudget::with_retained_work` and `retained_work_*` accessors are not wired (job tests that use them stay red).

## 6. Not templates / known stale

- The exemplars named in the earlier briefs (`gismap/✏️editor`, stdio `🧿️semio/✏️editor/📬️preparation`) use the old one-argument request shape (see `📓️2026-10-10-rename-rn-stdio.md`). Use the plugin crate's `BoundedConfigPreparation*` for the preparation shape and the plugin's `ArtifactReservedToolJob::close_step` (`🦀️.rs:~19736`) for a migrated job `close_step`.
- Plugin `🦀️.rs` still has old-shape pieces to be migrated by rn-os next: `ArtifactStoreInitializationAuthority::step -> StepOutcome`, the `framework_reserved_job!` macro `step`, `retained_job_payload(cx, stream, bytes)` helpers, `ArtifactReservedToolJob::step`.

## 7. Red crates below the kernel (blocking all native and wasm verification)

1. `semio-framework-io-sqlite-snapshot` (12 errors): sub-modules (`🧩️artifact`, `🔁️transfer`, `🧪️tests/🫙️prefix`) use `control.ledger.*`, `SqliteSnapshotControl::with_retirement_owner`, `install_native_retirement_recipient`, `transfer::SchemaValidationStorage`/`construct_database_into`, none of which exist in its `🦀️.rs` (the pushed lib is behind its modules, same pattern as the job crate had).
2. `semio-framework-tool-run`: conflicting `RetireOwned` impls in `♻️retirement/🦀️.rs:3-8` (derive vs manual) and a `RetainedCloneSource<_>` generic mismatch in `👥️entities/📸️version/🦀️.rs:38`.
3. `semio-framework-2d` failed in the cold release (not rechecked).
4. Fixed by rn-os (see `📓️2026-10-10-rename-rn-os-edits-below-store.md`): job lib, replication seal.
