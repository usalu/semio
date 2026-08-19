# 📓️ terra-cold-kinds report

Packet: implement the missing cold job kinds (`semio.infer`, `semio.mutation-plan`, `semio.migrate`)
and the host-side wiring for them plus `semio.compose`'s routing. Owned paths:
`🔌️plugin/⚛️reactor/💼️jobs/**`, `🔌️plugin/🖥️host/🦀️component.rs`. `semio.compose`'s guest body is
explicitly NOT mine (live `compose-await` packet owns `ComposeStepper`/`ComposeState`) — not defined,
not stubbed. Acceptance builds belong to the coordinator (rule 4/23) — my own runs below are pasted
verbatim with exit codes but are **executor-observed, not acceptance**.

## delivered — guest side (`🔌️plugin/⚛️reactor/💼️jobs/**`)

Three new submodules, one per kind, declared via `#[path]` from `⚛️reactor/💼️jobs/🦀️component.rs`
(mirrors this ticket's own one-`component.rs`-per-directory convention — `⚛️reactor` itself does the
same for `executor`/`requests`/`patches`/`jobs`/`checkpoint`):

- **`💼️jobs/💡️infer/🦀️component.rs`** — `semio.infer`. `input`/result are the SAME JSON
  `WireArtifactInferenceRequest`/`WireArtifactInferenceResult` bytes `PluginInstanceHandle::infer`
  already passed through unmodified (that host method predates this packet — `terra-jobs-runtime`
  or an earlier packet already wrote it). Dispatch goes through the bare `crate::app::
  wire_artifact_infer` (the process-global `ArtifactInferenceServiceRegistry` lookup), the SAME
  kind of process-global registry `job_io_run`/`job_io_sniff` (delivered by `terra-jobs-runtime`)
  already read from — not the per-`PLUGIN`-instance `plugin_wire_artifact_infer`, which is scoped to
  a completely different installation mechanism (`install_plugin_bundle`) a native inference-service
  registration never requires.
- **`💼️jobs/🧬️mutation-plan/🦀️component.rs`** — `semio.mutation-plan`. `input`/result are the
  DSL wire-pack bytes `🖥️host/🦀️component.rs`'s own `HostArtifactMutationPlanRequest`/`Result`
  already mirror field-for-field (`store::pack_rt::encode_wire_value(&dsl::to_dsl_value(...))`, NOT
  plain JSON). Dispatch goes through the bare `crate::plugin_runtime::wire_artifact_mutation_plan`
  (process-global `contributed_mutation_plan` registry), whose own `Result<Vec<u8>,
  semio_framework::Fault>` already matches `JobFn`'s exactly — no fault-code translation needed,
  unlike infer's `ArtifactInferenceExecutionError` boundary.
- **`💼️jobs/🔀️migrate/🦀️component.rs`** — `semio.migrate`, a versioned re-encode. `input` bundles
  the three former `migrate-artifact` export params into one JSON `{from, to, pack}` triple — the
  SAME "tuple/struct positional decode" idiom `job_io_run`'s own `IoRunInput` established. Dispatch
  goes through `store::migrate_document`, the process-global `DialectMigration` registry every
  plugin's own `PluginBuilder::migrations(...)` declarations already populate at `try_build()` (an
  earlier packet's wiring, outside my owned paths, already live).

### Slice/checkpoint shape (identical across all three, factored into one shared helper)

Added a `//#region 🔖️Phased` region to `⚛️reactor/💼️jobs/🦀️component.rs` itself:
`async fn run_two_phase(ctx, restored, decode, execute)` — private (visible to `jobs`'s descendant
modules via `super::`, per Rust's normal privacy rule for private items), so all three new kinds
share it instead of tripling the boilerplate (CLAUDE.md: "if code is repeated, it must be close to
each other").

- **Slice 1** (skipped on restore if `restored == Some(PHASE_DECODED)`): `ctx.tick().await`, then
  `decode()` — a REAL decode/validate of `input` (not a placeholder), reporting an identity-shaped
  progress payload (`(artifact_kind, inference_schema)` for infer, `(artifact_kind, mutation_id)` for
  mutation-plan, `"{from}->{to}"` for migrate), then `ctx.checkpoint(PHASE_DECODED)`.
- **Slice 2**: `ctx.tick().await`, then `execute()` — the real dispatch (`wire_artifact_infer` /
  `wire_artifact_mutation_plan` / `migrate_document`), `ctx.progress(b"phase.executed")`, return
  `Done`/propagate `Err`.

Both closures independently re-parse `input` from scratch rather than threading a decoded value
across the `tick()` boundary — a second cheap parse is harmless and keeps `restore_job` correct
without a richer checkpoint payload (checkpoint carries only the phase marker, never the input,
since `restore_job` always re-supplies the original `input` alongside the checkpoint bytes).

**Honest scope note on "sliceable":** none of the three underlying native calls
(`wire_artifact_infer`, `wire_artifact_mutation_plan`, `migrate_document`) are themselves chunked —
each is one atomic blocking Rust call. Real sub-call preemption (a WFC/SfM-class solve calling
`ctx.tick()` from INSIDE its own loop) is the same "blocked upstream" gap the dormant WFC solver's
own comment names (`✏️s/🔌️plugins/🌀️procedural/🦀️component.rs`, read but not touched — that
migration is explicitly out of scope, a W7 flagship owns it). What IS delivered and real: the JOB
ITSELF takes ≥2 genuine `step_job` calls, reports monotonic progress each slice, and correctly
resumes from a checkpoint — the mission's literal bar, not overclaimed as "the native call can be
interrupted mid-flight" (it can't, yet).

### `JobCtx::host()` — not used, by design, confirmed against the coordinator's live spike

None of the three kinds call `ctx.host()`. All three are `world actor` (poll-world) kinds, and
`JobCtx::host()` is `#[cfg(feature = "component-guest-async")]`-gated specifically because
`run_job_to_completion`'s relay never pumps `poll` between `step_job` calls (see `⚛️reactor/💼️jobs/
🦀️component.rs`'s own module doc, "Host-await restriction", written by `terra-jobs-runtime`). The
coordinator's mid-task message confirmed independently (a wasmtime spike) that a plain sync `func`
export is **uncallable at all** on a `component_model_async`-configured `Store`, so `world actor`'s
sync `jobs` interface could never have called a host import even once — this packet's design already
avoided that trap by construction, not by luck.

### tests (guest side) — UNRUN, blocked by a concurrent crate-wide compile failure (see `## honest gaps`)

10 new tests, 3-4 per kind, every one exercising `start_job`/`step_job`/`checkpoint_jobs`/
`cancel_job`/`restore_job` for real (no mocking of the job engine itself) AND a real registered
native service/contribution/migration (no mocking of the dispatch target either):

- **`💡️infer`**: `a_two_slice_infer_job_decodes_then_dispatches_to_the_registered_service` (2 real
  `step_job` calls, `Running(Some(identity))` then `Done(result)`, against a REAL
  `ArtifactInferenceService` registered via `crate::app::register_artifact_inference_service`),
  `infer_job_checkpoint_restore_matches_an_uninterrupted_run` (interrupt after slice 1, checkpoint,
  cancel, restore, resume — asserts byte-identical final output vs. an uninterrupted run),
  `infer_job_reports_a_named_decode_fault_on_garbage_input`.
- **`🧬️mutation-plan`**: `a_two_slice_mutation_plan_job_decodes_then_dispatches_to_the_registered_kind`
  (against a REAL contributed `CompositeMutationKind` committed via `crate::app::
  commit_contributed_mutation_services`, running a real `Planner`), the same checkpoint/restore
  shape, the same decode-fault shape.
- **`🔀️migrate`**: `a_two_slice_migrate_job_decodes_then_dispatches_to_the_registered_migration`
  (against a REAL `store::DialectMigration` registered via `store::register_dialect_migration`),
  checkpoint/restore, decode-fault, PLUS `migrate_job_reports_a_named_fault_when_no_migration_is_registered`
  (a real "no route" failure, not a placeholder).

## delivered — host side (`🔌️plugin/🖥️host/🦀️component.rs`)

- **`PluginInstanceHandle::mutation_plan`** — new, mirrors `infer` exactly:
  `self.run_job_to_completion("semio.mutation-plan", request.to_vec())`. Wire bytes pass straight
  through (DSL wire-pack, matching the guest's own decode).
- **`PluginInstanceHandle::migrate`** — new: bundles `(from, to, pack)` into one JSON tuple (mirrors
  `io_run`'s own bundling of what used to be separate export params) and drives `semio.migrate`. Its
  CALLER belongs to the pending runtime/db refactor per the brief — left unwired, not stubbed;
  nothing calls it in production yet, which is honest (nothing called `io_run`/`infer` in production
  before this ticket's runtime existed either).
- **`PluginInstanceHandle::compose`** — new: `IoRouter::compose`'s hand-written host error is GONE.
  It now bundles `key_bytes`/`sources_bytes` into a small JSON `ComposeInput` and drives
  `"semio.compose"` through the same `run_job_to_completion` relay every other kind uses. Until
  `compose-await` registers `"semio.compose"` on the guest side, this surfaces the ORDINARY
  `job.unknown-kind` fault `step_job` already produces for any unregistered kind (a real, dynamic
  failure reflecting actual guest state) instead of the old PERMANENT hand-written refusal — once
  `compose-await` lands, this exact host code starts succeeding with zero further host change. The
  `ComposeInput { key, sources }` wire shape is provisional and clearly marked as such in its doc
  comment — coordinate with `compose-await` before changing it.
- **`IoRouter::compose`** — resolution unchanged (still the same pure route lookup + self-route
  guard); the dispatch tail now calls `handle.compose(key_bytes, sources_bytes)` for real instead of
  returning a canned `Err`.
- **`ArtifactMutationRouter`** — gained a `runtimes: Mutex<HashMap<String, Arc<PluginInstanceHandle>>>`
  field and a `plan(&self, request_bytes: &[u8]) -> Result<Vec<u8>, PluginHostError>` method,
  mirroring `ArtifactInferenceRouter`'s exact shape (same field type, `plan` mirrors `infer`'s
  decode → resolve → dispatch → return-raw-wire-bytes shape, reusing `resolve()` rather than
  duplicating the ownership lookup). `register_plugin` (the wire-decoding wrapper — confirmed via a
  repo-wide python census that NOTHING calls it in production; only the pure `register_roster` is
  called, by `🏃️run/🦀️component.rs`) now also takes `handle: Arc<PluginInstanceHandle>` and stores
  it, matching `ArtifactInferenceRouter::register_plugin`'s 4-argument shape exactly.
  `unregister_plugin` now also drops the runtime handle.

## line ranges edited

- `⚛️reactor/💼️jobs/🦀️component.rs` — was 704 lines, now 763; added `mod` declarations (~L55-65), 3
  new `JOB_KIND_*` consts (~L75-83), 3 `builtin_registry()` inserts (~L126-128), the `//#region
  🔖️Phased` block (~L462-491, `PHASE_DECODED` + `run_two_phase`).
- `⚛️reactor/💼️jobs/💡️infer/🦀️component.rs` — new file, 178 lines.
- `⚛️reactor/💼️jobs/🧬️mutation-plan/🦀️component.rs` — new file, 210 lines.
- `⚛️reactor/💼️jobs/🔀️migrate/🦀️component.rs` — new file, 171 lines.
- `🖥️host/🦀️component.rs`:
  - `PluginInstanceHandle::mutation_plan`/`migrate`/`compose` — new methods, right after the
    existing `infer` (region `🔀️PostTurnRelay`).
  - `IoRouter::compose` — dispatch tail replaced (region around the old `:2097-2121` line numbers
    the brief cited — file has grown since, methods are ~60 lines further down after this packet's
    own insertions above them).
  - `ArtifactMutationRouter` struct/`new`/`register_plugin`/`plan`/`unregister_plugin` — region
    `🎯️MutationRouter`.
  - Tests: replaced `io_router_compose_resolves_ownership_but_dispatch_is_not_yet_wired` with
    `io_router_compose_resolves_ownership_and_drives_the_semio_compose_job_to_completion` +
    `io_router_compose_still_refuses_to_route_back_into_the_calling_plugin`; added
    `plugin_instance_handle_migrate_drives_the_semio_migrate_job_to_completion`,
    `plugin_instance_handle_mutation_plan_passes_wire_bytes_through_to_done` (region
    `🔀️IoRouterPostTurnRelay`); added `plan_drives_the_registered_owners_mutation_plan_job_to_completion`,
    `plan_fails_with_a_named_error_when_the_owner_is_not_loaded` (region `🎯️MutationRouter`'s own
    test module).

## commands + exit codes (real execution evidence, host side)

```
CARGO_TARGET_DIR=/private/tmp/claude-501/.../scratchpad/target-cold-kinds \
  cargo check -p semio-framework-plugin-host --lib
```
exit 0, clean (only pre-existing unrelated warnings in `🏪️store`).

```
CARGO_TARGET_DIR=/private/tmp/claude-501/.../scratchpad/target-cold-kinds \
  cargo test -p semio-framework-plugin-host --lib -- --skip schema_parity
```
exit 0 — **`test result: ok. 118 passed; 0 failed; 1 ignored; 0 measured; 4 filtered out`**
(baseline 113 + net +5: +6 new tests, −1 retired test that pinned down the now-deleted hand-written
refusal). Every new/changed test name confirmed **by name** in the output, e.g.:
```
test component::tests::io_router_compose_resolves_ownership_and_drives_the_semio_compose_job_to_completion ... ok
test component::tests::io_router_compose_still_refuses_to_route_back_into_the_calling_plugin ... ok
test component::tests::plugin_instance_handle_migrate_drives_the_semio_migrate_job_to_completion ... ok
test component::tests::plugin_instance_handle_mutation_plan_passes_wire_bytes_through_to_done ... ok
test component::artifact_mutation_router_tests::plan_fails_with_a_named_error_when_the_owner_is_not_loaded ... ok
test component::artifact_mutation_router_tests::plan_drives_the_registered_owners_mutation_plan_job_to_completion ... ok
```

```
CARGO_TARGET_DIR=/private/tmp/claude-501/.../scratchpad/target-cold-kinds \
  cargo test -p semio-framework-plugin-host --lib schema_parity
```
exit 0 — **4 passed / 0 failed** (unchanged baseline, I never touched WIT/schema).

```
CARGO_TARGET_DIR=/private/tmp/claude-501/.../scratchpad/target-cold-kinds \
  cargo check -p semio-framework-plugin-host --all-targets
```
exit 0, clean.

Host side is fully green, both gates (rule 26), no regressions, every new test genuinely exercises
`MockGuestRuntime`'s real `start_job`/`step_job` (some scripted `Running` THEN `Done`, proving a real
loop not a single call — same pattern `terra-jobs-runtime`'s own host tests already established).

## honest gaps

- **`semio-framework-plugin --lib` (the guest crate my 3 new kinds live in) currently FAILS to
  compile — this is a pre-existing, concurrent, cross-packet blocker, NOT my bug.** Confirmed 3
  times (same 8 errors, same locations, across ~5 minutes of retries):
  ```
  error[E0432]: unresolved imports `semio_framework::io_compose_via`, `semio_framework::resolve_ready`,
  `semio_framework::AsyncComposeFn`, `semio_framework::ComposeFuture`
    --> 🔌️plugin/🦀️component.rs:453
  error[E0425]: cannot find function `resolve_ready` in this scope
    --> 🔌️plugin/🦀️component.rs:16330
  error: future cannot be sent between threads safely  (×6, `export_media`/`media_fingerprint`, lines 12548/12565)
  ```
  All 8 errors are inside `crate::app` (unconditionally-compiled, not the wasm32-gated `component`
  module) in `🔌️plugin/🦀️component.rs` — a file explicitly outside my owned paths and explicitly
  named in my brief's exclusion list (`🚪️io/**`, "live atomic io-signature sweep"). Root cause,
  confirmed by reading the actual working-tree files rather than assuming: `git diff HEAD --stat`
  shows `🔌️plugin/🦀️component.rs` carrying 640 uncommitted insertions and `🧰️framework/🔨️modules/
  🚪️io/🦀️component.rs` carrying 50 uncommitted insertions that DO define `ComposeFuture`/
  `AsyncComposeFn`/`resolve_ready`/`io_compose_via` (confirmed present in that file's current working
  tree) — they are just not yet re-exported at `semio_framework`'s own crate root (`🧰️framework/
  📦️packages/🦀️rust/📦️glue.rs`, last touched 2026-08-17, i.e. NOT part of today's in-flight edit).
  `🚪️io/🦀️component.rs` was modified 2 minutes before my last retry (actively being worked on right
  now). **None of the 8 errors reference any file under `💼️jobs/**`** — grepped explicitly, zero hits,
  every retry.
- **Consequence: the 10 new guest-side job-kind tests are UNRUN, not passing-and-unreported.** I
  cannot honestly claim `cargo test` evidence for them — rule 7 forbids it — so I am not claiming it.
  What I CAN and did verify: `cargo check`/`cargo test` on `semio-framework-plugin-host` (a separate
  crate with no Cargo dependency on `semio-framework-plugin` at all — confirmed by reading its
  `Cargo.toml`) is fully green and covers the HOST half of every kind's dispatch path end-to-end
  through `MockGuestRuntime`, including scripted multi-step `Running`→`Done` sequences. The guest-side
  logic itself (decode → `run_two_phase` → dispatch → checkpoint) was hand-traced against the exact
  types/functions it calls (`WireArtifactInferenceRequest`, `wire_artifact_infer`,
  `WireArtifactMutationPlanRequest`, `wire_artifact_mutation_plan`, `store::migrate_document`,
  `store::DialectMigration`) by reading their real definitions in the guest crate, not guessed.
  **Coordinator: please re-run `CARGO_TARGET_DIR=<scratch> cargo test -p semio-framework-plugin --lib
  reactor::jobs::` once `🔌️plugin/🦀️component.rs`/`🚪️io/🦀️component.rs` stabilize — that is the
  first real acceptance evidence for the guest half of this packet.**
- **`ArtifactMutationRouter::plan()` has no live production caller yet.** `🏃️run/🦀️component.rs`
  (not owned by me) calls `mutation_router.register_roster(...)` directly (3 args, no handle) at
  line ~1521 — it never calls the `register_plugin` wrapper I extended, so `runtimes` stays empty in
  production until that call site is updated. **Lease-request**: either (a) switch that call site
  from `register_roster(plugin_id, &manifest.dependencies, mutation_roster)` to
  `register_plugin(plugin_id, &manifest.dependencies, Arc::clone(&handle), &mutation_roster_wire_bytes)`
  (needs the RAW wire bytes available nearby, not just the already-decoded `mutation_roster` — check
  what that call site's own surrounding code holds), or (b) if wire bytes aren't conveniently
  available there, tell me and I'll add a small additive `register_runtime(plugin_id, handle)` method
  next to `register_plugin` so the existing `register_roster` call needs only one extra line beside
  it. I did not add (b) speculatively since it's genuinely two ways to solve the same gap and the
  right one depends on what `🏃️run` already has in scope, which I can't edit to check.
- **`semio.compose`'s wire shape (`ComposeInput { key, sources }`) is provisional.** I own the host
  method that builds it, but the guest-side decode belongs to `compose-await`. Flagging, not leasing
  (no edit needed from me) — if that packet wants a different shape, the one-line change is entirely
  inside `PluginInstanceHandle::compose`, which I own.
- **No `descriptor_is_fresh`/descriptor regeneration touched** — matches `terra-jobs-runtime`'s own
  precedent: jobs aren't part of `PackageDescriptor` (design-abi.md §3), so no plugin needs a
  descriptor re-emit for this packet either.
- **`JobCtx::host()` deliberately unused** — see the coordinator's mid-task schema-spike message
  (sync `jobs` exports are uncallable on an async-configured `Store`; async `jobs-async` is coming
  later, scoped to `world actor-async`). All three kinds here target `world actor` (poll-world) only,
  by design, matching `terra-jobs-runtime`'s own documented restriction — no code change was needed
  in response to that message, only this acknowledgment.
