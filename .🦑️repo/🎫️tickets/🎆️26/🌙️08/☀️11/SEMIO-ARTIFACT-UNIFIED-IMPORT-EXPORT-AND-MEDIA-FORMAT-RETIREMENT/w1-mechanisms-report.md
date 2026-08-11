# W1 Mechanisms Report

Agent: W1 (Mechanisms), serial, sole writer this wave for: `📜️script.ts`, plugin SDK `component.rs`,
`🚪️io/🦀️component.rs`, os-kernel/framework/run glue+component files.

All raw command outputs referenced below are saved alongside this report as `.txt` files in this
ticket folder (never `.log`).

---

## Task 1 — script.ts: schema-owning vs delegating subset generalization

**Files changed**: `📜️script.ts`
- `policySchemaRepresentationBreaches` (was ~7628) — rewritten to branch on schema ownership.
- New `policySchemaIsDelegatingPair(repoRoot, schemaRoot)` — structural check: exactly 2 files
  (`🦀️component.rs` + `🟦️component.ts`), no subdirs; rs matches `pub use …::any::schema::*;`; ts is
  either a literal `export * from ".../✳️any/🧬️schema/🟦️component"` re-export (ifc's shape) or a
  `meta` stamp const with no own `interface`/`type`/`enum` (pdf/step/zip's shape) — both real shapes
  found on disk were verified and handled.
- New `policySchemaRootIsOwning(repoRoot, schemaRoot, migrated)` — schema-owning iff unmigrated OR
  `${schemaRoot}/📸️snapshot` exists.
- `policyStandardsCoverageBreaches` reason string (line ~7258) widened: "industry conformance
  profile/class/view" → "…or semantic type", with `✳️brep`/`✳️mesh` added to the example list. No
  logic change (subsetSlugPattern already accepts `brep` etc).
- `policyFieldSweepPresenceBreaches` — was keyed/scoped per **(artifact, standard)** only
  (`policyStdioStandardKey`), meaning ONE `field_sweep` test anywhere under a whole standard's tree
  would silently satisfy ALL of that standard's schema-owning subsets. Widened: new
  `policyStdioSubsetKey(artifactId, standardSlug, subsetId)` key, and the file-walk scope narrowed
  from `standardRel` to `entry.subsetRel` — each schema-owning subset now needs its OWN
  `field_sweep` test. Zero effect today (every current standard has exactly 1 schema-owning
  subset), but this now correctly gates semio v1's future 13-subsets-per-standard shape.
- `script.ts:7041`'s `?? 29` fallback — left untouched per instructions (W1b's job).
- No allowlist entries were needed (the rule fix cleared the breaches directly — allowlisting was
  never necessary, so the "programmatically computed keys" constraint was never exercised).

**Round-trip verification** (`w1-policy-before.txt` → `w1-policy-after-task1a-v2.txt`,
`w1-final-policy.txt`):
```
before:  21564 total, 24 rules, stdio-artifacts/schema-representation = 181
after:   21384 total, 24 rules, stdio-artifacts/schema-representation = 1
```
Diffing every rule's count line before/after: **schema-representation is the only rule whose count
changed** (181→1; every other rule is byte-identical). The 1 remaining breach is
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧬️schema` — a real, unrelated, pre-existing unmigrated artifact
missing its `🧬️schema/` facet entirely (not a delegating-subset false positive). Confirmed by
inspection: step's `✳️cc1..cc6`, pdf's `✳️a/e/h/ua/vt/x`, zip's `✳️iso21320`, ifc's
`✳️cv20/sav/cobie`, xlsx/docx/pptx's `✳️strict/transitional`, svg's `✳️basic/tiny`, jpg's
`✳️baseline`, tiff's `✳️baseline`, json's `✳️i-json`, xml's `✳️valid` — all 31 formerly-breaching
delegating subsets — no longer appear in the breach list at all.

field-sweep-presence breach count: 0 both before and after (33 files already contain `field_sweep`
tests, unaffected by the scope narrowing since today's `✳️any`-only schema-owning entries stay
1:1 with their standard).

---

## Task 2 — SDK helpers: deserializer_entry_of / serializer_entry_of

**File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, `//#region 🔖️Dialect`
(~line 400-580 after edit), immediately after `composer_entry_of`.

Added `pub fn deserializer_entry_of<D: ArtifactDeserializer>() -> ComposerEntry` and
`pub fn serializer_entry_of<S: ArtifactSerializer>() -> ComposerEntry`, both:
- single-read (`match sources { [one] => ..., other => Err(ComposeError{ message: "...needs
  exactly 1 source, got N"... }) }`), unlike `composer_entry_of`'s multi-source union;
- decode the one source's `IoPayload::Binary` via `<D::From as store::ArtifactPack>::decode_pack`,
  call `D::deserialize`/`S::serialize`, re-pack the result via `store::ArtifactPack::encode_pack` —
  same erasure-via-`ComposeSource`/`ComposedArtifact` round-trip `composer_entry_of` already uses;
- `reads: &[D::FROM]` / `&[S::FROM]`, `writes: D::INTO` / `S::INTO`.
- Added both to the SDK's public export barrel (`pub use app::{...}` list, ~line 9398) next to
  `composer_entry_of`.

**Test**: extended the existing `plugin_builder_contract_tests` mod (~line 7193, the only test
region in this file with a ready-made `store::ArtifactPack`-implementing fixture, `TestSnapshot`) —
added `DummySerializer`/`DummyDeserializer` (trivial `TestSnapshot -> TestSnapshot` round-trip) and
`serializer_entry_of_and_deserializer_entry_of_erase_correctly`, asserting correct `writes`/`reads`,
a real erased-compose round trip (encode→erase→compose→decode, byte-for-byte snapshot equality),
and the 0-source/2-source error paths.

**Verified**: `cargo check -p semio-framework-plugin --lib` → exit 0, 0 errors (`w1-task2-cargo-check.txt`).
`cargo test -p semio-framework-plugin --lib serializer_entry_of_and_deserializer_entry_of_erase_correctly`
→ **initially blocked** by 2 pre-existing FOREIGN errors elsewhere in the same crate's test binary
(`TutorialBase.document_dsl` / `ExampleDefinition.document_json` — confirmed via `git diff` hunk
ranges to be entirely outside anything this task touched; these types are actively being reshaped
by the concurrent `26/08/10/ARTIFACT-SYSTEM-OVERHAUL-...` ticket). These 2 errors block the WHOLE
test binary from linking, so the new test could not be run to a green `test result: ok` line without
touching out-of-scope files — recorded here as foreign breakage, not silently fixed, per the
hazard-management convention. My own code compiles error-free at `--lib` scope; the 2 `.unwrap_err()`
calls I originally wrote (which needed `ComposedArtifact: Debug`, not implemented) were fixed to
explicit `match` before this became relevant.

**Open item for W7/orchestrator**: `cargo test -p semio-framework-plugin --lib` cannot currently
produce a green run for ANY test in this crate (not just mine) until the `TutorialBase`/
`ExampleDefinition` foreign breakage is resolved by whichever ticket owns it.

---

## Task 3 — framework io_compose_via helper

**File**: `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`, end of `//#region 🔖️Dispatch`, right after
`io_dispatch`.

```rust
pub fn io_compose_via(hub: &IoKey, target: &IoKey, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    let hub_composed = io_dispatch(hub, sources)?;
    let hop_source = ErasedComposeSource { dialect: hub_composed.dialect, payload: hub_composed.payload };
    io_dispatch(target, std::slice::from_ref(&hop_source))
}
```
Reuses `io_dispatch` (itself reusing `resolve`) for BOTH hops — no duplicated resolve/compose
logic, and both hops get the fallback dispatcher + subset validation `io_dispatch` already
provides. Doc comment states the max-2-hops invariant and why (cycle/blow-up prevention, single
auditable stack frame per hop).

This file had **no prior `#[cfg(test)]` region at all** (confirmed by grep before editing) — and,
being framework-level, cannot depend on stdio's real composer entries (framework has zero
dependency on stdio, confirmed ground truth). Added a new `//#region 🔖️Tests` /
`#[cfg(test)] mod tests` at the file's end registering a minimal synthetic 2-hop chain through the
SAME `register_composer_entries`/`io_dispatch`/real `IO_REGISTRY` machinery any real chain (e.g.
stdio's png↔deflate↔binary) goes through — proving the mechanism against the real registry, not a
hand-simulated call graph. Two tests: the happy 2-hop path, and hub-resolve-failure surfacing.

**Verified**: `cargo test -p semio-framework --lib io_compose_via` (`w1-task3-test-v2.txt`):
```
test io::tests::io_compose_via_chains_two_registered_hops ... ok
test io::tests::io_compose_via_surfaces_hub_resolve_failure ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 98 filtered out
```
`cargo check -p semio-framework` → exit 0, 0 errors (`w1-cargo-check-framework.txt`).

---

## Task 4 — register_document_codec multi-schema-per-standard: BLOCKING FINDING

**Read**: `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs` (the `ArtifactSchemaRegistry`/
`ArtifactSchemaDescriptor` registry — a DIFFERENT, separate registry from `ArtifactCodec`).
`register_document_codec` itself is actually defined in
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:629`.

**Finding — the plan's assumption is WRONG.** `register_document_codec`'s registry is a plain
`HashMap<String, ArtifactCodec>` keyed by `codec.schema`:
```rust
pub fn register_document_codec(codec: ArtifactCodec) {
    let mut registry = document_codec_registry().write()...;
    registry.insert(codec.schema.clone(), codec);   // plain insert — silently overwrites, never panics
}
```
Its own doc comment says **"idempotent, safe to call repeatedly"**. Registering two DIFFERENT
`ArtifactCodec` values under the SAME id does **NOT** panic — it silently overwrites
(last-registered-wins), with zero warning, zero diagnostic, zero side channel.

**Empirically confirmed** (not just static reading, per CLAUDE.md's "must validate assumptions"):
extended the existing `#[cfg(test)]` region in `store/🦀️component.rs` (right after the pre-existing
`document_codec_of_round_trips_dsl_and_pack_and_edit_text` test, which already has the needed
`DemoSnapshot`/`DemoMutation` fixtures) with
`register_document_codec_same_id_twice_overwrites_silently_not_panics`: registers two
distinguishable `ArtifactCodec` values under the same id, asserts no panic, asserts the SECOND one
silently wins. Result (`w1-task4-test.txt`):
```
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 740 filtered out
```

**Does registering multiple DIFFERENT ids work fine?** Yes — unambiguously. Each of the 31 current
`register_document_codec` call sites (one per artifact-standard) already registers a distinct id
with zero collision, and nothing about the mechanism prevents 13 more distinct ids
(`s.stdio.semio.brep`, `s.stdio.semio.mesh`, …) from coexisting under `artifact_kind
"s.stdio.semio"` / standard `"v1"` — the registry key is the raw schema id STRING, not scoped by
artifact_kind+standard at all, so distinct ids never collide regardless of how many share an
artifact_kind/standard.

**BLOCKING finding for the orchestrator**: the specific phrase "duplicate-id panic keeps collisions
loud" in the master plan's ground truth is factually incorrect for the CURRENT implementation. This
is not a blocker for W2's design working AT ALL (13 distinct ids will register and resolve
correctly) — it IS a blocker for the design's safety net: if two of the 13 parallel W2a/W2b agents
ever typo/copy-paste their way into the same schema id (a real risk when 6+7 agents each build one
subset independently), the second one silently and invisibly wins with no build failure, no test
failure, no runtime error — only a subtly wrong document loaded through the wrong codec at runtime.
Recommend W1b or W2's closer either (a) add a policy rule (`script.ts`) statically checking all
registered-codec-id string literals for duplicates across the whole `stdio` crate, or (b) change
`register_document_codec` to panic/return-Result on a real collision (a framework behavior change,
outside a single subset agent's scope, needs its own decision + review of all 31+14 existing call
sites for accidental collisions this would newly surface).

---

## Task 5 — os-run fix attempt

**Pre-flight**: `git status --porcelain` on all hot files → clean (confirmed, matches W0 recon).

Targeted the CORRECTED 13-error list from `w0-recon-report.md` §1c (not the stale `topic_contributions`
claim, which was already independently fixed by another session and does not appear anywhere in the
current error output).

### Step 1 — mount `🔁️workflow` (with a real, confirmed correction to the plan)

**First attempt (per the plan's literal instruction) failed and was reverted.** Mounting
`🔁️workflow/🦀️component.rs` into the **os-kernel** glue.rs (`pub mod os_workflow`, exactly matching
`os_vcs`'s pattern) does NOT work: `🔁️workflow/🦀️component.rs` has hard `use semio_framework::{...}`
dependencies (`AppDefinition`, `MediaClass`, `MediaType`, `ConfigSpec`, `Terminology`, `Locale`, …)
on the FULL `semio-framework` crate — which the wasm-safe `semio-framework-os-kernel` crate
architecturally cannot depend on (that would be a real Cargo dependency CYCLE: `semio-framework`
already depends on `semio-framework-os-kernel`, confirmed in that crate's own `Cargo.toml`).
Mounting it there does not just fail to fix anything — it BREAKS the previously-clean
`semio-framework-os-kernel` crate itself (7 new `E0432`/`E0433` errors), which cascades to every
crate downstream of it (i.e. everything). This is exactly why the kernel glue.rs's own header
comment says these modules are "unwired pending dep-DAG cleanup" — it is not an arbitrary gap.
**Reverted** (kernel glue.rs restored to a functional no-op state, left only an explanatory comment;
confirmed `cargo check -p semio-framework-os-kernel` clean again after revert).

**Real fix**: mounted `🔁️workflow/🦀️component.rs` into `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` (the
actual `semio-framework` crate) instead — verified this crate's own glue.rs ALREADY re-exports
every `semio_framework::{...}` symbol workflow needs (`MediaClass`/`MediaType`/`MediaWireFormat`/
`ConfigSpec`/etc from its `mesh` mount, `Locale`/`Terminology` from `ui_wgpu`). Two small additions
needed beyond the mount itself: `extern crate self as semio_framework;` (so workflow's own
`use semio_framework::{...}` lines resolve — this crate had never needed to self-reference before)
and `extern crate semio_framework_os_kernel as store;` (workflow uses `store::` extensively; this
crate had also never needed that alias before, only re-exported item names). Also updated the run
crate's OWN glue.rs — `extern crate semio_framework_os_kernel as workflow;` → `extern crate
semio_framework as workflow;` (run already depends on `semio-framework` directly per its
`Cargo.toml`).

Files touched: `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` (comment only, net no
functional change — mount attempted then reverted), `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`
(+workflow mount, +2 extern-crate aliases, +`pub use workflow::*;`),
`🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/📦️glue.rs` (1-line alias swap).

**Result**: `cargo check -p semio-framework` → exit 0, 0 errors (`w1-framework-check-with-workflow-v3.txt`).

### Step 2 — re-check os-run

`cargo check -p semio-framework-os-run` (`w1-osrun-check-after-fix.txt`): **13 → 9 errors.** The
entire E0432 (11 unresolved `workflow::*` imports) + both `RunArtifact` E0425s + the
`apply_run_operation_checked` E0425 cluster cleared, exactly as the plan predicted (mount location
corrected).

### Step 3 — remaining 9, fixed 9→0 (lib) / 9→4 (bin, see below), in
`🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`:

1. **5× E0433 `os_dsl` not in scope** (lines 297, 302): `os_dsl::Fault`/`FaultOrigin`/`FaultCode`/
   `decode_fault_bytes`/`encode_fault_bytes` — no `os_dsl` alias exists in this crate's glue.rs, but
   `dsl` (already imported, `use dsl::{from_dsl_value, to_dsl_value};` at the top) aliases the SAME
   kernel crate, which re-exports all of `os_dsl::*` at its own crate root. Replaced `os_dsl::` →
   `dsl::` at both call sites (6 occurrences). Root scope, not an ambiguous design choice.
2. **2× E0592 duplicate `artifact_pack_path`/`artifact_spr_path`** (lines 520 vs 531, 524 vs 535):
   the second definition of each was a dead, SELF-RECURSIVE alias (`fn artifact_pack_path(&self,
   artifact_ref: &str) { self.artifact_pack_path(artifact_ref) }` — would infinitely recurse were it
   even reachable) left over from an apparent past rename; confirmed via every call site in the file
   that the FIRST definition is what all real callers already use. Removed the dead second pair
   (and their misleading "thin alias for existing callers" doc comment, which described a shim that
   never had any actual callers).
3. **1× E0609 `RunSink` has no field `operations`**: the struct's real field is `mutations` (its own
   doc comment even describes it as "every operation `record` has successfully applied" — matching
   `mutations` exactly); `record()`'s body had one stray `self.operations.push(operation)`. Fixed to
   `self.mutations.push(operation)`.
4. **1× E0004 non-exhaustive `AppFrame` match**: `frame_in_reply_to`'s match was missing
   `AppFrame::Emit{..}`/`AppFrame::Draft{..}`. Both variants carry a plain `in_reply_to: u64` field
   (checked the enum definition in `📡️spr/🧵️channel/🦀️component.rs`) — identical shape to sibling
   arms `Document`/`Config`/`Done`. Added `AppFrame::Emit { in_reply_to, .. } => Some(*in_reply_to),`
   and the same for `Draft`, following the exact sibling pattern — real, honest behavior (both are
   host replies to a specific request), no `todo!()`/catch-all.

`cargo check -p semio-framework-os-run --lib` → **exit 0, 0 errors** (`w1-final-osrun-check.txt`
inspected: the lib target compiles clean; only pre-existing-shape warnings, e.g. one now-dead
`run_fault_bytes` fn).

### Step 4 — newly-exposed bin.rs errors (previously masked by the lib failing first)

`cargo check -p semio-framework-os-run` (checking BOTH lib and bin targets) initially still failed
because `📦️bin.rs` — a SEPARATE crate root from the lib, which does NOT inherit the lib's own
`extern crate ... as store/workflow;` aliases — had never actually been reached by the compiler
before (cargo does not check a bin target once the lib it depends on fails). Two more real,
mechanical, non-ambiguous fixes in `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs`:
- Added `extern crate semio_framework_os_kernel as store;` and `extern crate semio_framework as
  workflow;` (same aliases the lib's glue.rs already has, mirrored here since a `[[bin]]` target is
  its own crate root).
- Same `operations`→`mutations` typo, present twice more here (`persist_run`'s doc comment + body).

This brought the bin target 28 → 4 errors, all four now rooted in exactly ONE cause:
**`RunArtifact: ArtifactPack` is not implemented.** `RunArtifact` (defined in
`🔁️workflow/🦀️component.rs:1878`) derives `dsl::DslArtifact` (which only generates internal
`__dsl_*` helper methods, confirmed by inspecting the pattern) but — unlike its sibling
`WorkflowSnapshot` in the SAME file, which has explicit hand-written `impl store::ArtifactDsl for
WorkflowSnapshot` / `impl store::ArtifactPack for WorkflowSnapshot` blocks right after its derive —
`RunArtifact` has **no corresponding hand-written impl anywhere in the file.** This was never
caught before because `🔁️workflow` was never mounted/compiled at all until this task.

**STOPPING HERE, flagged as a genuine open item, not guessed at**, because:
- `🔁️workflow/🦀️component.rs` is **not in this wave's hot-file/write-authority scope** (my assigned
  files are `script.ts`, plugin SDK `component.rs`, `io/component.rs`, os-kernel/framework/run
  glue+component files — NOT workflow's own file).
- Rust's orphan rule means this impl can ONLY be written inside the crate that owns `RunArtifact`
  (now `semio-framework`, since workflow is mounted there) — i.e. it MUST live in
  `🔁️workflow/🦀️component.rs` itself, not in run's files.
- It is genuine missing functionality (a hand-rolled `ArtifactDsl`/`ArtifactPack` pair matching the
  `record`/`table`/`block` shape already declared via `#[dsl(...)]` attributes on `RunArtifact`'s
  fields — likely also needs `RunMutation: protocol::Mutation<RunArtifact>`), not a wiring fix.

**Recommended exact fix for W7/orchestrator**: copy `WorkflowSnapshot`'s hand-written
`impl store::ArtifactDsl for WorkflowSnapshot` / `impl store::ArtifactPack for WorkflowSnapshot`
pattern (same file, ~line 1047-1090) verbatim for `RunArtifact`, swapping the type name and
`S_RUN_SCHEMA`/`"run-document"` extension — the pattern is fully mechanical/boilerplate (3 sibling
examples exist in the codebase: `DemoSnapshot`, `TestSnapshot`, `WorkflowSnapshot`) but touches a
file outside this wave's scope, so it is deliberately NOT done here.

**Also discovered, same root cause, out of scope**: `cargo test -p semio-framework --lib` now fails
with 39 errors, ALL inside `🔁️workflow/🦀️component.rs`'s own `#[cfg(test)]` region, referencing
`store::test_support::assert_op_line_round_trip`/`assert_operation_round_trip` — helper functions
that do not exist anywhere in `store`. This is further evidence `🔁️workflow` has literally never
been compiled (with tests) before this wave; its own test suite has never run. Not fixed (out of
scope, same reasoning as above) — flagged for the same follow-up.

**Foreign, unrelated breakage observed (NOT caused by this wave)**:
`cargo test -p semio-framework-os-kernel --lib` shows 5 pre-existing failures — all in
`os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance`/`m5_production_coverage` for
en1992/fem2d/dag fixture grammars (`"grammar must recognize shipped fixture DSL body"` panics).
Confirmed foreign via `git diff` on the kernel glue.rs: my only change there is a comment (net-zero
functional diff, the workflow-mount attempt was reverted). These tests are unrelated to
io/store/plugin-SDK/dsl-fault machinery this task touched, and the policy baseline's
`handcrafted-grammar/spec-distinctness: 19352` breach count already shows this area is in heavy
concurrent flux repo-wide. Recorded, not chased.

---

## Final exit-checklist commands

```
cargo test -p semio-s-plugin-stdio --lib
```
`w1-final-stdio-test.txt`: **`test result: ok. 1075 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`**
(exact baseline, zero regression, zero growth this wave — no new stdio tests were in scope).

```
bun ./📜️script.ts policy
```
`w1-final-policy.txt`: **`21384 high-priority breach(es) across 24 rule(s)`** (down from W0's 21564,
well within the ≤21564 gate). `stdio-artifacts/schema-representation` = **1** (down from 181).

```
cargo check -p semio-framework
```
`w1-final-framework-check.txt`: **exit 0, 0 errors** (clean, includes the `🔁️workflow` mount).

```
cargo check -p semio-framework-os-run
```
`w1-final-osrun-check.txt`: **exit 101** — `--lib` alone is **0 errors** (verified separately); the
full invocation (lib+bin) has **4 remaining errors**, ALL rooted in the single out-of-scope
`RunArtifact: ArtifactPack` gap in `🔁️workflow/🦀️component.rs` documented above under Task 5 Step 4.
Down from the W0 baseline's 13.

---

## Files changed this wave

- `📜️script.ts` — Task 1 (schema-representation rule generalization, vocabulary reason string,
  field-sweep per-subset key/scope).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — Task 2
  (`deserializer_entry_of`/`serializer_entry_of` + export barrel + smoke test).
- `🧰️framework/🔨️modules/🚪️io/🦀️component.rs` — Task 3 (`io_compose_via` + new `#[cfg(test)]` region,
  the file's first).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` — Task 4 (empirical duplicate-id
  regression test; documents current silent-overwrite behavior).
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` — Task 5 (comment-only; workflow mount
  attempted then reverted here).
- `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` — Task 5 (real `🔁️workflow` mount + 2 extern-crate
  aliases + glob re-export).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/📦️glue.rs` — Task 5 (`workflow`
  alias repointed from the kernel crate to the framework crate).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` — Task 5 (`os_dsl`→`dsl`, removed 2
  dead duplicate fns, `operations`→`mutations` field-name fix, 2 new `AppFrame` match arms).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs` — Task 5 (2 new `extern crate` aliases,
  `operations`→`mutations` fix ×2).

**Not touched** (deliberately, out of write-scope): `🔁️workflow/🦀️component.rs` itself — needs the
`RunArtifact` `ArtifactDsl`/`ArtifactPack` impl pair + `store::test_support` helper functions,
flagged above for W7/orchestrator.

## Blocking / open items for the orchestrator

1. **Task 4**: `register_document_codec` silently overwrites on a duplicate id, does not panic —
   the master plan's ground truth is wrong on this point. Not a blocker for correctness (distinct
   ids always work), but a real safety-net gap for 6+7 parallel W2 agents each minting one of 13
   subset ids under the same artifact_kind/standard. Recommend a script.ts uniqueness policy rule
   or a `register_document_codec` behavior change — needs an explicit decision, not guessed here.
2. **Task 5**: os-run's remaining 4 errors (bin target only; lib is 100% clean) all trace to
   `🔁️workflow/🦀️component.rs` missing `impl store::ArtifactDsl`/`impl store::ArtifactPack for
   RunArtifact` (mechanical fix, exact pattern given above, but outside this wave's file scope).
   Additionally its own `#[cfg(test)]` region references nonexistent `store::test_support` helpers
   (39 compile errors under `cargo test -p semio-framework --lib`) — same file, same reason not
   fixed here.
3. **Task 2**: `cargo test -p semio-framework-plugin --lib` cannot produce a green run for ANY test
   in that crate right now — blocked by 2 confirmed-foreign errors (`TutorialBase.document_dsl` /
   `ExampleDefinition.document_json`) unrelated to this wave's diff hunks (verified via `git diff`).
4. **Observed, not caused**: `cargo test -p semio-framework-os-kernel --lib` has 5 pre-existing
   foreign failures in en1992/fem2d/dag grammar-fixture conformance tests, unrelated to anything
   this wave touched (confirmed net-zero functional diff on that crate's glue.rs).
