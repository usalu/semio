# publication: semio-os-store-one-item-publication redesign onto the 733 contract

## Step 1 (edited, unverified)
- W-publication-1 (native) stopped upstream: replication-mutation-messages (rootfix2: RetirementHeaderRelease) and framework-actor (actor: StagedSourceAdmission project_owned). Both claimed by peers; this crate not reached yet.
- Design decisions (no compat layers):
  - canonical source (store/canonical-edit/source): both sources are identity sources. Authority is `RetainedCloneSource<AuthorityLease>` (A==T, `admission_owned`, `project_owned` at projectionStart); the typed views come from `RetainedCloneRef::project` and `semio_framework_pack_json::retained_alias` (admit+take), one alias per granted turn (`sub` 0 authority alias, 1 edit alias, 2 cursor admit). `AuthorityLease` retires through `ArcLeaseRetirement<Live>`; the late `authority_close` is `ControlledRetirement<ArcLease<Live>>`. Quotes: `retained_alias_demand/work_demand`, `owned_projection_demand`. Alias/projection owners close before the sources.
  - post root retirement: new `ArtifactStorePostRootRetirement<P>` (prepared/retirement) elects the unique Arc payload, converts it once with `SharedOriginalStorage::admit_from` and retires it through the captured `SnapshotRetirementFactory`; used by the prepared retirement and the canonical sealer (replaces `factory_retire_snapshot` over `Arc<P>`). Store-read `ArcRootRetirement` cannot be reused (store-read -> operation-wire -> publication would be cyclic). Debt: same logic exists in store-read; unify into a framework crate later.
  - binary receiving (operation-wire/receiving/binary): typed source is `RetainedCloneSource<FactoryOwnedSourceAuthority<M>>` (identity), alias via `retained_alias(project)`, owned handoff via `take_authority_into` with stashed `take_reply`/`take_pending` (pending keeps state, recover next turn) and `transfer_original_into`; `terminal_is_empty` includes `!take_pending`.
  - `ArtifactOriginalOperationReceiving::advance` now returns `NativeReceivingFailure` and the JSON adapter takes the witness from `control.source_witness()` (follows pack-json).
  - `ErasedSnapshotRetirement::close_step(body,effect)`: wire retirement, prepared, digest, diagnostics stage, authority retirement; `BatchRetirement::step(body,effect)` (+ emission/owned caller); publication authority fields cursor uses `native_retirement_reply!` + inherent `close_step(body,effect)`.
- Rerun: chk.sh W-publication-<n> 🌎️hub/Cargo.toml native -p semio-os-store-one-item-publication


## Step 2 (verified lib)
- Upstream reds cleared by peers; W-publication-3 showed 17 own errors (my splice duplicated a ladder tail in the sealer; `edit_ref` bound needs RetireOwned; binary `step_granted(body,..)`). Fixed.
- Native lib W-publication-4 and W-publication-5 rc 0, wasm32-wasip2 W-publication-6 rc 0 (46 warnings = type-checked; unused imports I introduced removed).
- Rerun: chk.sh W-publication-<n> 🌎️hub/Cargo.toml native|wasm -p semio-os-store-one-item-publication
- Debts: (1) `ArtifactStorePostRootRetirement` duplicates store-read `ArcRootRetirement` logic (store-read depends on operation-wire -> publication, so it cannot be reused); move one shared copy into a framework crate. (2) Cancelling while the canonical source holds an alias mid-turn closes aliases first; no cancel-mid-staged-projection path is needed (all projections are identity). (3) Source tests in store/tests/bin-06 still use SharedControlledRetirement::lease (stale, not this crate). (4) Callers of `ArtifactOriginalOperationReceiving::advance` (emission receiving, operation-wire replacement, tool-run tick operations, window config typed-pack-load) must handle `NativeReceivingFailure`.


## Step 3 (tests)
- The lib-test closure exceeded the Rust Source budget (W-publication-9: refused before compiler, limit not raised). The two in-file tests (target authority, borrowed staged inverse digest) moved to `[[test]] operation-wire-target` (path = lifted/operation-wire/target/rs; own `#[global_allocator]`; imports through the public crate API); `#[cfg(test)]` mod and lib allocator removed. Cargo.toml: only the [[test]] section (LOCK NEEDED logged; no dependency change). Ported `work` to u128.
- Tests need the ROOT manifest (the crate declares workspace = repo root; hub and framework manifests refuse "not a member"). Rerun: tst.sh W-publication-<n> Cargo.toml native -p semio-os-store-one-item-publication --test operation-wire-target
- W-publication-11: 2 passed, 0 failed.

STATUS: FIXED-verified W-publication-5 (native lib), W-publication-6 (wasm lib), W-publication-11 (2/2 tests)
