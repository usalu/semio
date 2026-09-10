# Artifact envelope load — decode poll and decode-worker pump (2026-09-10)

Lane: envelope-load. Takes the cross-lane failure `📓️unit-suite-2d-2026-09-09.md` §4 handed over
(`vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`, identical
in generation2d and generation3d, the 3d report's class D) and fixes it at the root in the
framework.

Private target `$S/target-env` (APFS clone of the shared warm `debug/`), `RUSTC_WRAPPER=""`,
`RUST_MIN_STACK=536870912`, `--test-threads=2`, `--no-fail-fast` (`--keep-going` is build-only).
Raw logs: `🗑️generated/env-*.txt`.

---

## 0. Reproduction (before any edit)

| artifact | command | result |
|---|---|---|
| generation3d | `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib` | `left: Fault, right: Ready`, **finished in 0.02 s** (`env-1`) |
| generation2d | `cargo test -p semio-s-artifact-procedural-generation2d --features component-app-assembly --lib` | `left: Fault, right: Ready`, **finished in 0.02 s** (`env-2`) |

0.02 s is the whole story: `drive_production_envelope` loops up to 300 000 maintenance turns and
returns the moment the poll is terminal, so both artifacts fail-closed on the **first** turn. The
2d lane's "Pending for all 300 000 turns" was measured with the early return removed.

## 1. Root cause A — an unfinished decode was reported as a terminal `Fault`

`VcsArtifactApp::advance_artifact_envelope_load`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19262-19276` before the fix) read:

```rust
if let Some(active) = self.envelope_decode_jobs.get(handle.operation.0) {
    …
    if active.poll() == ArtifactEnvelopeDecodeOperationPoll::Ready && !self.try_begin_artifact_store_replacement(handle)? {
        return Ok(ArtifactEnvelopeDecodeOperationPoll::Progress);
    }
    return Ok(self.poll_artifact_store_replacement(handle));
}
```

When the decode is **not** `Ready` the `&&` short-circuits and control falls straight through to
`poll_artifact_store_replacement(handle)` (`:19441`), whose `map_or` maps a **missing** replacement
job to `ArtifactEnvelopeDecodeOperationPoll::Fault`. The replacement job cannot exist yet — it is
created by `try_begin_artifact_store_replacement`, which only runs once the decode is `Ready` — so
every in-progress decode was reported to every caller as a terminal fault. Eight plugins share this
loop shape (`🔌️jack`, `🖨️raster`, `🧊️process3d`, `✒️writer`, `🧊️generation3d`, `🌀️generation2d`,
`🗺️gismap`, `🖍️drawing`, plus the two `🧩️puzzle` wasm bridges). At runtime this is a fail-close on
opening or replacing a document (example switch, undo across a store swap).

**Fix.** The decode's own poll is returned whenever it is not `Ready`:

```rust
let decode = active.poll();
if decode != ArtifactEnvelopeDecodeOperationPoll::Ready {
    return Ok(decode);
}
if !self.try_begin_artifact_store_replacement(handle)? {
    return Ok(ArtifactEnvelopeDecodeOperationPoll::Progress);
}
return Ok(self.poll_artifact_store_replacement(handle));
```

`ActiveArtifactEnvelopeDecode::poll` already maps `Active → Pending`, `ClosingCancelled → Cancelled`,
`ClosingFault → Fault`, so cancellation and real faults keep their exact codes.

## 2. Root cause B — the decode worker had no reactor-turn pump

`ActiveArtifactEnvelopeDecode::drive` (`🔌️plugin/🦀️.rs:17201`) only **submits** a step to the
process worker pool (`session.pump_one(pool, Lane::Interactive)`); on wasm that pool has no threads
and runs a submitted step solely when it is pumped. Its only driver was the cooperative-maintenance
rotation — `drive_envelope_decode_jobs` is stage **11** of `MAINTENANCE_STAGES`, one visit per full
rotation — exactly the pathology `drive_typed_operation_worker` (`:22150`) was created for on
2026-09-09 (measured then: 3 interactive job steps in 4 096 reactor turns). A live envelope decode
also never made the reactor select its instance: `has_runnable_typed_operations` (`:24727`) only
looked at typed tool operations, so `plugin_continue_typed_operations` (`:31393`) skipped the app.

**Fix**, mirroring the typed-operation pump exactly:

- `VcsArtifactApp::drive_artifact_envelope_decode_worker` — bounded by `INTERACTIVE_TURN_WORKER_PUMPS`
  (256) and `INTERACTIVE_TURN_WORKER_WALL_US` (4 000 µs, half the interactive lane ceiling), called
  from `advance_typed_operation_publication`, which is the reactor-turn entry point
  (`⚛️reactor/🔄️turn/🦀️.rs:749` → `plugin_continue_typed_operations` → `advance_typed_operation_output`
  → `PluginApp::advance_typed_operation_publication`).
- The pump drives the whole decode ladder, not just the worker: `drive_envelope_decode_jobs`,
  `drive_envelope_field_decoder_returns` and `drive_envelope_completed_record_returns`. A decode
  parks in `release_step` until its field-decoder lease **and** its completed record are reclaimed,
  so pumping only the worker stalls every load one step short of terminal (measured: 260 decoder
  steps then `Pending` forever, `env-20`).
- `ActiveArtifactEnvelopeDecode::has_runnable_work` (`state != Ready`) feeds a new
  `has_runnable_artifact_envelope_decode`, which `has_runnable_typed_operations` now ORs in. `Ready`
  is excluded on purpose: it awaits its exact consumer publication, not a worker step, so it must
  never hold the reactor in `more-work`.
- `drive_envelope_decode_jobs` pumps the cooperative pool after each drive under
  `#[cfg(target_arch = "wasm32")]`, exactly as `drive_typed_operation_worker` does.
- `TYPED_OPERATION_WORKER_PUMPS_PER_TURN` / `TYPED_OPERATION_WORKER_WALL_US` are renamed
  `INTERACTIVE_TURN_WORKER_PUMPS` / `INTERACTIVE_TURN_WORKER_WALL_US` — one bound now shared by both
  mounted-worker pumps rather than a typed-operation-specific name copied twice.

## 3. Three further defects on the same path, found once the decode actually ran

Root cause A hid everything downstream: no caller ever drove a decode past its first turn. With A
fixed the pool worker reached the real envelope decoder and **aborted the process**.

### 3.1 The taken owner was abandoned on every `?` path — process abort

`ArtifactEnvelopeFreshVcsAuthority::accept_token` and
`ArtifactEnvelopeFreshFieldDecoder::accept_field_token` (`🏪️store/🦀️.rs`) take their live child out
of `self.active` so the borrow checker admits the sibling-field writes, and restore it only on the
`Pending`/`TokenComplete` arms. Every `?`, every `ok_or_else(…)?` and every `FieldComplete` whose
`take_values()` returned `None` dropped the local instead — and
`OwnedSchemaBoundedArrayAuthority::drop` (`🏪️store/🦀️.rs:8052`) is a fail-closed assert:

```
thread 'semio-pool-worker-2' panicked at 🏪️store/🦀️.rs:8052:9:
bounded history array reached Drop before every exact entry owner was taken or cursor-retired
  2: <OwnedSchemaBoundedArrayAuthority<os_vcs::Alternative> as Drop>::drop
  4: drop_glue::<ArtifactEnvelopeFreshVcsActive<Generation3dMutation>>
  5: <ArtifactEnvelopeFreshVcsAuthority<…> as ArtifactEnvelopeVcsFieldAuthority<…>>::accept_token
```

so a decode diagnostic — the fail-closed path — killed the process instead of faulting the job.
**Fix.** Both authorities restore `self.active` on every exit that does not consume the field, and a
`take_values()` that refuses now returns `artifact-envelope.vcs-history-not-taken` with the owner
retained for the close ladder.

### 3.2 `close_step` dropped a live history array

`ArtifactEnvelopeFreshVcsAuthority::close_step`'s snapshot branch ran
`if let Some(ArtifactEnvelopeFreshVcsActive::Snapshot { reservation, .. }) = self.active.take()`.
`self.active` is `Some(Edits|Changes|Checkpoints|Alternatives)` for the whole history phase (the
initial-snapshot field authority is only released by `publish_reserved`, which runs last), so the
unconditional `take()` dropped a live array authority whenever a decode closed mid-history — the
same abort, now during cleanup. **Fix.** The arm is matched through `as_ref()` and only a `Snapshot`
arm is taken; every other arm is left for `close_pending_history`.

### 3.3 Nested record authorities handed their children the PARENT's terminal flag

This is why the decode could never have succeeded. `ArtifactEnvelopeFreshVcsAuthority` and
`ArtifactOwnedSprEditAuthority` own an `OwnedSchemaNestedRecordCursor` that tracks their record's
fields and nesting depth, but both **short-circuited that cursor** for every token that arrived
while a child owner was live, and routed the token with the `terminal` flag their *caller* passed.
That flag comes from the parent record cursor, which marks `terminal: true` only for the token that
closes the parent's own field (`OwnedSchemaRecordCursor::step`, `🏪️store/🦀️.rs:6386-6393` —
`depth == 1`). So `vcs.alternatives`' closing `]` reached
`OwnedSchemaBoundedArrayAuthority::accept` with `terminal == false` and fell through to

```
[DEBUG] history-expected-record path $.alternatives kind ArrayEnd terminal false start 125
```

Nothing else was wrong with the wire: `serde_json` has no `preserve_order` here, so the fixture's
vcs object is emitted alphabetically and `alternatives` is the FIRST vcs field — offsets 108/109/123/124/125
are exactly `{`, `"alternatives"`, `:`, `[`, `]`. The identical defect one level down
(`ArtifactOwnedSprEditAuthority` → `ArtifactOwnedSprMutationArrayAuthority`) produced
`artifact-spr.mutation-array-entry at offset 1928 path $.edits` for an edit's `forwards`/`inverse`.

**Fix.** Both authorities now advance their own cursor exactly once per **distinct** token and route
to the child with the cursor's per-field `terminal`. The decode authority re-delivers the same token
while a field returns `Pending`, so a `pending: Option<(OwnedSchemaToken, bool)>` slot holds the
cursor-attributed pair across that redelivery loop and is cleared on any non-`Pending` outcome and
at the top of `close_step`. Field installation and record assembly moved into
`begin_vcs_field`/`finish_vcs_record` and `begin_edit_field`/`accept_active_edit_token`.

## 4. New framework laws

Both live in `🔌️plugin/🧪️tests/🧩️composition/🦀️.rs` (region `📨️EnvelopeDecodeLadder`), driven by
`SlowEnvelopeFieldDecoder`, a field owner that yields a fixed number of decode steps before it
completes — so the decode ladder is provable without a domain field catalog. Admitting it needs no
test-only hook: `submit_artifact_envelope_decode` was factored into
`admit_artifact_envelope_decode_owner(operation, generation, pages, fields, completion)`, and the
domain route just builds `fields` from its declared bundle.

| law | proves |
|---|---|
| `advance_artifact_envelope_load_reports_a_live_decode_as_pending_not_fault` | §1 — a live decode polls `Pending` through the load API, while `poll_artifact_store_replacement` on the same handle is `Fault` (the reading that must not leak out). |
| `one_reactor_turn_pumps_the_envelope_decode_worker_to_its_terminal_poll` | §2 — `has_runnable_typed_operations` is true for a live decode, the reactor-turn publication pass drives it to a terminal poll (≥ 256 decoder steps), and the retired decode stops being runnable work. |

## 5. Runs

### 5.1 The two artifact tests

| artifact | before | after |
|---|---|---|
| generation3d | `Fault` vs `Ready` in **0.02 s** (turn 1) | `Fault` vs `Ready` in **2.41 s**, now from the plugin's own decoder (`env-24`) |
| generation2d | `Fault` vs `Ready` in **0.02 s** (turn 1) | `generation2d-publication.contended` in **6.95 s** (`env-25`) |

Both are now blocked **downstream of this lane**, in each artifact's own retained owners, and are
handed back with evidence:

- **generation3d** — the framework now decodes the whole envelope record and hands each edit's
  `forwards` entry to `Generation3dMutationDecodeAuthority`, which fails at
  `🌀️procedural/…/🧊️generation3d/…/🧬️mutations/💾️binary/🦀️.rs:2158`:
  `generation3d-envelope.mutation-ingress-malformed at offset 358 path $.edits`
  (`Generation3dMutationSession::grant()` refuses). This is the 3d lane's class C
  (`generation3d-mutation.body-malformed`) on the ingress route; generation2d already fixed its twin
  (`📓️unit-suite-2d-2026-09-09.md` §3.2/§3.3 — wire port roles and ONE nesting bound). Owner:
  generation3d unit-suite lane.
- **generation2d** — its retained decoder now passes, and the failure moved to
  `generation2d-publication.contended` raised by `generation2d_refresh_publication_authority` on the
  test thread. `GENERATION2D_PUBLICATION_SLOTS` is a process-global lease table
  (`📓️unit-suite-2d-2026-09-09.md` §3.8) and the decode step now genuinely runs **concurrently** on
  a pool worker, so the test thread's refresh and the worker's hold contend for the same lease. The
  lease needs to admit a concurrent holder (or the refresh needs to retry) — that is generation2d's
  publication authority, not the envelope load. Owner: generation2d lane.

### 5.2 Framework tests

```
cargo test -p semio-framework-plugin --lib --no-fail-fast -- --test-threads=2 <filter>
```

| filter | result |
|---|---|
| `envelope_decode` (new law §2) | **ok. 1 passed; 0 failed** |
| `advance_artifact_envelope_load_reports` (new law §1) | **ok. 1 passed; 0 failed** |
| `retained_composed_replacement` (the store-replacement laws) | **FAILED. 0 passed; 3 failed** |
| `app_maintenance_*_envelope_*`, `canonical_surface_app_id` | **FAILED. 0 passed; 5 failed** |

The eight red ones are **not this lane's** and fail identically when filtered alone:

- `retained_composed_replacement_{publishes_…, cancellation_…, rejects_…}` — `did not reach
  ValidatingClosure` / `CandidateReady` / `left: Fault`. They drive `maintenance_stage = 14`
  (`drive_store_replacement_jobs`) against `ReadyComposedParentInitialization` and never touch an
  envelope decode, an envelope field authority, `advance_artifact_envelope_load`, or the reactor-turn
  pump — every line this lane changed is unreachable from them.
- `app_maintenance_reclaims_late_envelope_field_returns_before_close_terminal` and
  `app_maintenance_and_close_retain_completed_envelope_results_until_terminal_empty` use their own
  `ReturnedDecoder` against the return registries, not the fresh VCS/SPR authorities.
- `editor|viewer_app_envelopes_carry_the_real_canonical_surface_app_id` and
  `handle_action_invocation_accepts_the_real_canonical_surface_app_id` — `artifact store reached Drop
  without its exact terminal-empty shallow-shell witness`, unrelated to envelopes entirely.

`🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` carries uncommitted peer edits
(`git status` ` M`) and the workspace broke twice mid-lane from peers
(`UiText::try_from_str` `Result`→`Option`, `FlowEvalSessionState::published_eval_hash`), so the
framework-plugin suite is red from the concurrent fixture-separation wave. Flagged, not adopted.

### 5.3 Checks

| check | result |
|---|---|
| `cargo check -p semio-framework-plugin --keep-going` (native) | `Finished dev in 38.19s`, **0 errors**, 4 warnings (`env-23`) |
| `cargo check -p semio-s-plugin-procedural --keep-going` (native) | `Finished dev in 3m 16s`, **0 errors**, 8 warnings (`env-26`) |
| `cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev` (`CARGO_PROFILE_WASM_DEV_DEBUG=false`, private `$S/target-env-wasm`) | `Finished wasm-dev in 3m 34s`, **0 errors**, 11 warnings (`env-27`) |

Warnings are present in every check, so each is a real type-check and not an aborted expansion.

## 6. Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — §1 poll fix; §2
  `drive_artifact_envelope_decode_worker`, `has_runnable_artifact_envelope_decode`,
  `ActiveArtifactEnvelopeDecode::has_runnable_work`, the wasm pool pump in
  `drive_envelope_decode_jobs`, the `INTERACTIVE_TURN_WORKER_*` constants, and the
  `admit_artifact_envelope_decode_owner` seam.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` — §3.1 owner restoration in
  `ArtifactEnvelopeFreshVcsAuthority::accept_active_token` and
  `ArtifactEnvelopeFreshFieldDecoder::accept_field_token`; §3.2 the `close_step` snapshot branch;
  §3.3 the `pending` cursor slot plus `begin_vcs_field`/`finish_vcs_record` and
  `begin_edit_field`/`accept_active_edit_token`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs` — §4, the two new
  laws and their `SlowEnvelopeFieldDecoder` fixture.

No `[DEBUG]` and no `#[ignore]` remain from this lane; nothing outside the framework was edited.
