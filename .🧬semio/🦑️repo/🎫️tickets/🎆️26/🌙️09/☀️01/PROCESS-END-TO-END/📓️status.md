# 🏭️ Process End to End — status

## Target
`bun run dev:process:3d` → `bun ./📜️script.ts dev process 3d` → nx `@semio-tech/framework-os-dev:dev -- process3d`,
react renderer on **port 6022** (playground metadata: `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/Cargo.toml:18-22`,
app `s.process.process3d@1/*#editor`).

## Confirmed findings (read from source, not assumed)

### F1 — the shipped wasm is 5 days stale
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/process/semio_s_plugin_process_component.core.wasm`
is dated **Aug 27 16:02** (50 MB); `🛂️descriptor.semio` was regenerated Sep 1 14:13. Same failure shape the
sourcing ticket hit: the browser loads a plugin built against a different framework. A `wasm32-wasip2` build is
the truth gate and is running.

### F2 — both example documents load with an EMPTY scene  ← the "empty window" root cause
`Process3dSnapshot` (`🧬️schema/📸️snapshot/🦀️component.rs:22-52`) carries, since ticket
`26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4, two **inline, authoritative** payload fields beside the
composed child handles:

| field | role |
| --- | --- |
| `stock_payload: Stock` | the real workpiece solid — **the only thing the renderer reads** |
| `step_payloads: Vec<ProcessStep>` | the real timeline — **the only thing the stepper reads** |
| `stock_solid` / `steps` / `tool_solids` | `ArtifactChild<…>` handles; composition identity only, never resolved (no `LinkResolver` exists) |

The DSL printer emits ten lines including `stockPayload=` and `stepPayloads=`
(`🧬️schema/📸️snapshot/🦀️component.rs:140-153`). **Both shipped example fixtures emit only eight** — they predate
wave 4 and were never regenerated:

- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (the `timber-beam-joinery` / `default_document()` fixture) —
  fields present: `workshop, stockId, stockLabel, stockPose, stockSolid, steps, toolSolids, resolvedUpTo`.
- `PROCESS_3D_PLATE_EXAMPLE_TEXT` (`🧬️schema/📸️snapshot/📝️text/🦀️component.rs:14-25`) — same eight.

`parse_process3d_snapshot_body` (`…/📸️snapshot/🦀️component.rs:155-190`) starts from
`empty_process3d_snapshot()` and only overwrites the lines it sees, so the missing lines silently fall back to
`ProcessWorkingScene::default()` → **stock = a 1×1×1 unit box, steps = `[]`**.

Consequence in the app (`✏️editor/🦀️component.rs:997` boots `default_document()`):
- `🪚️workpiece` renders `processed_mesh(scene, resolved_up_to)` over that scene
  (`…/🪟️windows/🪚️workpiece/🦀️component.rs:92-107`) → a unit cube labelled "Timber Beam", not a beam.
- the engagement stepper (`…:151-162`) reports **0 steps**, so the timeline is dead.
- the existing tests never catch it: `default_document_parses_timber_example`
  (`🧬️schema/🦀️component.rs:954-958`) asserts only `steps.child_id` is non-empty, and
  `render_world_scene_contains_processed_mesh` (`✏️editor/🦀️component.rs:1760`) only greps the string
  `"processed"` — which the fallback mesh also produces.

### F3 — the seven step-scoped mutations are declared no-ops
`🧬️schema/🧬️mutations/{🌱create-step,🗑️delete-step,🏷️rename-step,🔘change-step-enabled,🧷change-step-origin,📐replace-step-measure,🔀reorder-steps}`
each return `MutationOutcome::empty().warn("mutation.no-op", "…pending a link resolver for the composed steps child.")`,
and `🧪️tests/mutate-process3d-1/🦀️.rs:61-69` lists all seven under `UNOBSERVABLE`. That reasoning is now stale:
`step_payloads` is the authoritative durable record and is inline, so these verbs can be implemented against it
(re-minting `steps`/`tool_solids` exactly as `process_working_scene_to_snapshot`
(`🗿️artifacts/🧊️process3d/🦀️component.rs:786-820`) already does) without any `LinkResolver`.
**Today the app cannot add, remove, rename, reorder, enable or re-measure a single process step.**

### F4 — not ours
`semio-s-plugin-stdio` still has no `🔣️descriptor.json` in `🔌️plugin-modules/stdio/` (last successful build Aug 18;
its component link overruns the 1 000 000-function ceiling). Process depends on stdio only for **types at compile
time** — it does not link stdio's wasm — so this is boot noise, not a process blocker. Peer-owned.

## Plan
1. **P0** rebuild `semio-s-plugin-process` for `wasm32-wasip2`; fix whatever no longer compiles.
2. **P1** regenerate both example fixtures with real `stockPayload` + `stepPayloads` via the documented
   fixture-regeneration technique (real `process_working_scene_to_snapshot` + `print_dsl()`, never hand-transcribed),
   and add assertions that bite (non-degenerate stock solid, non-empty timeline).
3. **P2** implement the seven step-scoped mutations against `step_payloads` + re-minted children; retire the
   `UNOBSERVABLE` list and regenerate the committed mutation vectors.
4. **P3** audit the four panels and the viewer for empty renders.
5. **P4** boot :6022, drive the browser, confirm every window is non-empty and the commands round-trip.

## Machine note
The shared cargo target dir is heavily contended (peer sessions running `cargo build -p semio-s-plugin-cad`,
workspace `cargo check`, two `cargo test` runs). Builds block on the target-dir lock; poll, do not kill.

---

# 📅️ 2026-09-02 — P0 reached: the crate compiles again

## What was broken beyond F1–F5, and what fixed it
A peer session ran a repo-wide mechanical sweep overnight (`🦀️component.rs` → `🦀️.rs`, leaf files
consolidated into their owning directory) on top of the serde → `ToValue`/`FromValue` migration
(commit `f394df99d4`). It left the process plugin non-compiling in five distinct ways. All are now fixed:

| # | Symptom | Cause | Fix |
| --- | --- | --- | --- |
| 1 | 16 × `E0255 … defined multiple times` | each `🧬️mutations/<verb>/🦀️.rs` kept a `use …::<verb>::<Type>;` line **and** now defines that type | removed the 16 self-imports |
| 2 | `couldn't read …/👥️presence/🧬️schema/🦀️component.rs` | stale `include_str!` after the rename | → `🦀️.rs` |
| 3 | 26 × `E0433/E0432 cannot find 'mutation' in <verb>` | the `mutation` submodule was flattened away by the sweep; every call site still said `<verb>::mutation::<Type>` | rewrote all 26 call sites to `<verb>::<Type>` |
| 4 | 2 × `E0046` missing `DESCRIPTORS`/`descriptor` | `protocol::Mutation` grew those items; the hand-written config + presence impls never got them — **the identical defect the sourcing ticket fixed** | added both, modelled on `🪵️sourcing`'s config impl (provisional owner paths, same as sourcing's precedent) |
| 5 | 3 × `E0277 … serde::Serialize is not satisfied` for `SemioBrepSnapshot` / `SemioFlowSnapshot` / `JsonValue` | the peer moved stdio's snapshot types off serde onto `ToValue` | the two content-addressed child handles now hash `ToValue::to_value(content)`; the json export serializer builds one `serde_json::Value` from the document's `ToValue` tree and uses it for both the snapshot and the byte export |

Two peer-owned breakages had to be repaired in place to get past them (both mechanical rename fallout,
both blocking every s plugin, not just this one):
- `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/🦀️.rs:39` — `#[path = "../../🦀️testkit.rs"]` → `"../../🧪️testkit/🦀️.rs"`.
- `✏️s/🔌️plugins/🗄️stdio` — four example assets moved into per-example directories
  (`🖼️assets/📊️example.csv` → `🖼️assets/🧪️example/📊️.csv`, and the gltf/gif/tiff equivalents) but the
  `include_str!`/`include_bytes!` call sites still named the old paths; repointed the 8 Rust references.
  The remaining `asset://…` references in stdio's `🥒️.feature` files are left for their owner.

## Verified
```
$ RUSTC_WRAPPER="" cargo check -p semio-s-plugin-process --lib --target wasm32-wasip2
EXIT=0   errors: 0
```
That is the plugin's real component surface (`wasm32-wasip2`), and it is the target the dev app loads.

## Still blocked on a peer
`cargo check -p semio-s-plugin-process` **native** is red with exactly 3 errors, none of them ours:
`semio-framework-plugin-host` still declares `decode_wire_dsl<T: serde::de::DeserializeOwned>` /
`encode_wire_dsl<T: serde::Serialize>` while `dsl::from_dsl_value`/`to_dsl_value` now require
`FromValue`/`ToValue`, and `semio_framework::kernel::PresenceUpdate`
(`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️presence.rs:77`) has no `FromValue`.
Converting it means adding the value-derive dependency to the ui-contract crate and cascading the
derives through `OwnPresence`/`PeerMark`/`SurfaceId` — squarely inside the peer's live workstream, so
it is left to them and polled rather than fought.

**Consequence:** `cargo test -p semio-s-plugin-process` cannot run yet, so the two regeneration tests
that were authored earlier are still unrun and their outputs still unpasted:
- `regenerate_example_fixtures` → `🗣️example.dsl.semio` + `PROCESS_3D_PLATE_EXAMPLE_TEXT` (fixes **F2**)
- `regenerate_step_mutation_vectors` → the seven step mutations' committed vectors (finishes **F3**)

The wasm component build (`plugin process`) does not need native, so the browser pass proceeds ahead of it.

## 🧨️ The link blocker, and why the plugin was actually unbuildable
`bun ./📜️script.ts plugin process` failed with **0/6 crates producing a .wasm**. The cause was not the
plugin: `rust-lld` **SIGSEGVs** (`lld::wasm::ElemSection::writeBody()`) while linking
`semio-framework-os`'s **cdylib** for `wasm32-wasip2`. That crate is `crate-type = ["cdylib", "rlib"]`
(long-standing, for its wasm-pack browser build), so cargo links its cdylib for every crate that depends
on it — and the peer's `ToValue`/`FromValue` fan-out grew it past whatever LLD limit that section hits.
The same crash blocked `cargo test --target wasm32-wasip2` too, so it closed BOTH routes to a runnable test.

**Fix, and it is a real cleanup rather than a workaround:** `semio-s-plugin-process` declared
`semio-framework-os` as a dependency and **never used it** — zero references to `semio_framework_os` in the
whole plugin (only `semio_framework_os_kernel`, a different crate). `🪵️sourcing` does not declare it at all.
Dropping the unused dependency (`✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/Cargo.toml:47`) removes the
crashing cdylib from this plugin's wasm link graph entirely. `📐️cad` and `🌀️procedural` still declare it and
will keep hitting the crash until they either use it or drop it too — worth telling their owners.

Second peer-owned link blocker, untouched: `semio-s-plugin-stdio`'s own component still overruns
`functions count exceeds limit of 1000000`, so `🔌️plugin-modules/stdio/` has no descriptor. Process only
consumes stdio's TYPES at compile time and does not link its wasm, so this is boot noise for us.

## Test surface repaired
`cargo check -p semio-s-plugin-process --lib --tests --target wasm32-wasip2` went from 69 errors to 0.
All of it was catch-up with peer-owned framework changes, none of it weakened a test:
- `protocol::testkit::assert_*` silently resolved to the WRONG module — the kernel root glob-reexports both
  `os_pack::*` (`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs:303`) and `os_spr::*` (`:304`) and both
  export a `testkit`. The mutation-law helpers live in `os_spr`. Call sites now say
  `protocol::os_spr::testkit::…`. **~21 files repo-wide still use the ambiguous bare path** — an upstream fix.
- Those helpers are `async fn`; several call sites never awaited them, so the law assertions were
  constructed and dropped without ever running. They now `.await`.
- `VcsArtifactApp`/`ArtifactStore::new`/`dispatch` and the store round-trip helpers became async; the test
  code now awaits them (and two `assert_undo_redo_round_trip` calls that were silently no-ops now run).
- Stale post-sweep paths: `include_str!("../../🔣️oracle.json")` → `"../../🧪️oracle/🔣️.json"`;
  `SnapshotRetirementFactory::retire_snapshot(…, partial)` → `retire(…, Arc::new(partial))`.

## Defects found by review of the unverified work, and fixed
An adversarial read of the code authored before any of it could run turned up three real bugs:
- `insert_step_mutations` inserted at `cursor + 1` and set the cursor to that same index, so adding a step
  marked an untouched step resolved and left the NEW step pending. Now inserts AT the cursor and advances to
  `cursor + 1`. (`🧬️schema/🦀️.rs`)
- `remove_step_mutations` used `cursor >= removed_index`, so deleting the first UNRESOLVED step pulled the
  cursor back and un-resolved a step the deletion never touched. Now strict `>`.
- `Process3dArtifactPreparation::close_step` compared `grant.maximum_bytes` against the constant
  `PROCESS3D_DOCUMENT_GRANT_BYTES` instead of the mutation's measured footprint, so a large mutation could
  report releasing more bytes than were granted. Now compares against `bytes`, matching sourcing.
Three tests were added for the two cursor builders, which had none.

## 🔍️ The inspection panel was not blocked after all
Every process3d panel doc comment (and `📐️cad`'s, `🌊️flow`'s, `🗒️note`'s, `🧩️puzzle`'s) says the per-item
inspector is unreachable because "`ArtifactEditor::render` carries no `InteractionView` parameter". **That prose
is stale.** The framework grew `ArtifactEditor::render_with_request_context`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, grep the name), which is handed an
`InteractionView`, and `VcsArtifactApp::render` calls **that** override unconditionally — `A::render` is never
the live path. `🌀️procedural` already uses it and renders a populated inspector.

So process3d now overrides it too, reads `interaction.selection(PROCESS3D_INTERACTION_DOMAIN)` (`"geometry"`),
and threads the selected ids to the inspection panel, which resolves them against the document and renders real
fields: the stock's solid variant + dimensions + pose, a step's origin/measure/parameters, or a machine's
capabilities and their parameters. The empty state survives only as the genuine no-selection fallback, and the
test that asserted "always empty" is replaced by four that assert the real content.

Four other plugins carry the same stale comment and the same always-empty inspector — worth telling their owners.

## 🚧️ Why the fixtures are still unregenerated
Two routes to running the crate's tests, both were blocked:
- **wasm**: `cargo test --target wasm32-wasip2` with `CARGO_TARGET_WASM32_WASIP2_RUNNER=wasmtime` builds and
  links fine, but the test binary is a COMPONENT that imports `semio:framework/pure@1.0.0` (`now-ms`, …), so
  plain `wasmtime run` cannot instantiate it: *"a matching implementation was not found in the linker"*. Running
  the suite this way would need a host adapter that does not exist.
- **native**: blocked by `semio-framework-plugin-host`, which still declared
  `decode_wire_dsl<T: serde::de::DeserializeOwned>` / `encode_wire_dsl<T: serde::Serialize>` while
  `dsl::from_dsl_value`/`to_dsl_value` had already moved to `FromValue`/`ToValue`. Every type that flows through
  those two helpers (`HostMutationRosterEntry`, `HostArtifactMutationPlanRequest`/`Result`) **already** carries
  both derives, so the bounds were simply not updated with them. Fixed here, plus the one `PresenceUpdate` decode
  — that type lives in `🖱️ui/🧬️contract`, a crate with no value-derive dependency, so rather than cascading
  derives through `OwnPresence`/`PeerMark`/`SurfaceId` into a peer's live workstream it now decodes through the
  documented, still-live `impl From<&DslValue> for serde_json::Value` bridge.

## ✅️ F2 closed — both example fixtures regenerated
With `semio-framework-plugin-host` green, `cargo test -p semio-s-plugin-process --lib -- --ignored regenerate_`
ran both authoring tools:
```
running 2 tests
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 330 filtered out; finished in 0.05s
```
Output (real `process_working_scene_to_snapshot` + `print_dsl()`, never hand-transcribed) installed:
- `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` — **11 lines now, was 8**: `stockPayload` (339 chars) and
  `stepPayloads` (2245 chars, four real steps) are present. A 3.0 × 0.2 × 0.3 timber beam with a crosscut, a
  lap-joint pocket, a dowel bore and the dowel itself.
- `PROCESS_3D_PLATE_EXAMPLE_TEXT` — same eleven fields, a 1.2 × 0.8 × 0.02 plate with a four-hole bolt pattern
  and `resolvedUpTo = 2`, so the demo opens mid-timeline and the stepper has somewhere to go.

**A geometry defect was caught before regenerating.** The authored scenes placed tools outside the stock:
`box_prim` is ORIGIN-CENTRED (`🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs:415-421`), so a 3.0-wide beam posed at
`x = 0` spans `[-1.5, 1.5]` — but the crosscut sat at `x = 2.7`, and three of the four plate holes sat outside
the plate entirely. Those steps would have been silent CSG no-ops: the timeline would have looked populated
while the mesh never changed. Poses corrected, the bore now breaks a real surface instead of hiding inside the
solid, and the dowel protrudes so the additive step is visible.

## ✅️ F3 closed — the seven step-mutation vectors regenerated
`regenerate_step_mutation_vectors` produced real before/mutation/after/diff/outcome quintets, replacing vectors
whose committed content was an empty diff with `before == after`. The seven fixture directories were named
`…-and-changes-nothing`, an assertion that is no longer true; renamed to describe the real effect
(`accepts-a-rip-cut-step-and-inserts-it`, `…-and-removes-it`, `…-and-applies-it`, `…-and-replaces-it`,
`…-and-reorders-them`) with all 10 referencing files updated.

## 🧵️ The migration was declared but never took effect — and the testkit could not have caught it
Running the suite after the fixtures landed surfaced a hard abort with
`interactive-job.catalog-authority`: *"tool proof catalog must exactly join migrated generated declarations
to live concrete factories"*, with the decisive detail `migrated={}` beside a full 33-entry `generated_ids`.

Root cause: `AppBuilder::action_interactive_job(id, …)`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5166-5172`) only mutates `self.actions`. **Thirty-two of
this app's thirty-three tool ids are declared as COMMANDS, not actions**, so every one of those calls was a
silent no-op — the builder neither errors nor warns on an id it cannot find. `migrated_tool_ids()` (`:12058`)
reads actions + app_commands + mode_commands and saw an empty set, so `tool_job_registration` (`:19140`)
rejected every proof row. The 33 calls are replaced by one `.interactive_jobs(InteractiveJobClassification::Migrated)`
sweep, which covers actions, window actions, commands and mode commands alike. Note the TS route audit passes
either way — it reads the SOURCE declarations, so it could not see that the builder discarded them.

Two more contracts had to be honoured before any of this was observable:
- **`testkit::app()` used the registry-less `new_app`**, whose own docstring says it is "for store-only tests"
  and that "UI and typed command dispatch fail closed until a real registry and exact factory are supplied".
  An app that declares tool proofs cannot be constructed that way at all. It now goes through
  `new_app_with_registry` with the real manifest, and binds the live runtime instance id so typed dispatch
  matches `meta("local")` — otherwise every typed command fails `interactive-job.live-instance`.
- **A registry-backed `VcsArtifactApp` asserts on `Drop`** that its artifact store reached the terminal-empty
  shallow shell, which only `close_registered_fixture_app` produces. Rather than editing dozens of tests, the
  fixture is now a `Process3dApp` newtype with `Deref`/`DerefMut` and a `Drop` that retires the stores, so
  every call site still writes `&mut app`.

## ⏱️ A silent build-budget kill, not a compile failure
`bun ./📜️script.ts plugin process` reported `plugin catalog build summary: 0/1 crate(s) produced .wasm` with
NO compiler error anywhere in 16 931 lines of output. The single decisive line was
`error: spawnSync cargo ETIMEDOUT`: `buildBudgetMs()`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1233`) caps every plugin
cargo invocation, and on a machine already running a second build the wasm compile ran past it. Re-run with
`SEMIO_BUILD_BUDGET_MS=5400000`. Worth knowing: a plugin build that "fails" with no diagnostic is this, not
your code.

## 📊️ Final suite state — 260 passed, 1 failed
`cargo test -p semio-s-plugin-process`

**The one red test is blocked by a peer, not by this work.**
`export_brep_out_returns_step_text_structured_payload` was passing for the WRONG reason before: with the old
empty fixture `replay_process` returned nothing, so `export_process3d_model` took its early `Ok(None)` branch
and the caller fell back to a structured JSON payload — the test named "returns step text" never once exercised
the STEP codec. The real fixture makes it reach the codec, which needs a `step` row in the format registry.
Registering stdio's rows is the established idiom (`✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/🖼️export-media/🦀️.rs:73-74`),
and the test now does exactly that — but `semio_s_plugin_stdio::manifest::stdio_format_descriptors()` itself
currently returns
`PluginAssemblyError { code: "stdio.definition", message: "s.stdio.gltf executable mapping keys diverge from schema registrations" }`
(`✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:466-468`): gltf's declared codec/mutation/inference ids no longer match
what `gltf_inference_services()` + `GltfMutation::kinds()` produce. That is stdio-owned and inside the peer's
live gltf work. The call is left in place because it is the correct fix; faking descriptors here would be worse.

**Two fixture-lifecycle contracts learned the hard way**, both now encoded in the testkit:
- A registry-backed `VcsArtifactApp` asserts on `Drop` that its artifact store reached terminal-empty. The
  framework's `close_registered_fixture_app` caps the drain at 64 turns of one item (`🔌️plugin/🦀️.rs:6699-6709`);
  this app — 33 tool jobs plus five stores — outgrows that, so the pump lives in the fixture's own `Drop`.
- That `Drop` must never panic while the thread is ALREADY unwinding: a second panic in a destructor aborts the
  whole binary and hides the assertion that actually failed. It now asserts only when not already panicking,
  which is how the export failure became readable at all.

`vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed` passes in the full suite but
fails in isolation — it depends on publication-lease state another test leaves behind. Pre-existing latent
ordering coupling, surfaced (not caused) by the fixture now being registry-backed: a bare `VcsArtifactApp::new`
can no longer be constructed for an app that declares tool proofs.

---

# 📅️ 2026-09-05 — re-verification of the closed waves after three days of peer churn

Everything below was re-read against today's tree, not taken from the earlier notes.

## ✅️ P2 still holds — the seven step verbs are genuinely implemented
Each of `🌱create-step`, `🗑️delete-step`, `🏷️rename-step`, `🔘change-step-enabled`, `🧷change-step-origin`,
`📐replace-step-measure`, `🔀reorder-steps` clones `base.step_payloads`, edits it, and returns
`MutationOutcome::new(process3d_step_timeline_diff(base, steps))` — a real diff that re-mints the
`steps`/`tool_solids` children. Verified per-verb in each `🔺️diff/🦀️.rs`.

Two things worth recording because they look like regressions and are not:
- **`mutation.no-op` still appears in eight diffs.** It is no longer the "unimplemented" marker it was; it is
  now the *benign guard* for a mutation whose new value equals the old one (rename to the same label, toggle to
  the same flag, reorder to the same index). The distinguishing check is the string: the old marker read
  `"…pending a link resolver for the composed steps child."`, and that phrase now has **zero occurrences**
  anywhere in the plugin.
- **`UNOBSERVABLE` is now `&[]`** (`🧪️tests/🌷️mutate-process3d-1/🦀️.rs:58`) — all sixteen mutation kinds carry a
  committed vector that moves the document.

## 🧨️ A live-turn cost hazard the fixture fix introduced
`🪟️windows/🪚️workpiece/🦀️.rs:106-113` documents that `processed_mesh` "builds a fresh kernel session, replays
every enabled step as a real CSG boolean, tessellates and remaps face groups", that `processed_volume` "replays
the identical sequence again", so **an uncached turn pays for the whole process twice** — and that the host
re-drives the plugin until every surface publishes, multiplying that by every continuation.

Before this ticket the shipped fixtures parsed to an EMPTY scene, so that whole path was free. Regenerating them
with a real four-step timeline is what first made a turn expensive. That makes the 8 ms
`INTERACTIVE_STEP_CEILING_US` branch a genuine candidate for the 09-02 `runtime live cleanup faulted` symptom —
and, importantly, one this ticket's own P1 could have *caused* rather than merely revealed. The `Process3dPreviewCache`
memo (structural `PartialEq` on the scene) is the existing mitigation; it protects repeat turns, not the first.

This is a hypothesis with a clear discriminator, not a conclusion — see `🧪️runtime-verification.md` for the
twelve fault sites and why the fault message alone cannot tell "too slow" from "wrong ABI".

## ✅️ P3 holds — the inspection panel is genuinely wired to live selection
`Process3dPlayApp::render_with_request_context` (`✏️editor/🦀️.rs:1644-1654`) overrides the framework default,
reads `interaction.selection(PROCESS3D_INTERACTION_DOMAIN)` (`"geometry"`, defined `✏️editor/🦀️.rs:46`) and
threads the ids into `process3d_render_body`. The framework method still exists on the trait
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26844-26854`) with a default that discards `interaction`,
so the override is load-bearing. `🔍️inspection::render` resolves each id against the stock, the machines and
`step_payloads`, and only falls back to `empty_state` on no selection or an unresolvable id. The other three
panels have no empty state at all — they always emit their sections.

## 🕳️ NEW gap found: the 3D viewport can never show a selection
`process3d_render_body` (`✏️editor/🦀️.rs:1315-1326`) passes `selected_ids` to **exactly one** body —
`PROCESS_3D_PLAY_BODY_INSPECTION` (`:1323`). The workpiece window (`:1319`) is called as
`workpiece::render(doc, config)`, with no selection argument. Downstream,
`evaluated_preview_payload` (`🪟️windows/🪚️workpiece/🦀️.rs:104-105`) hardcodes the single instance's
`"selected"` and `"hovered"` to `Bool(false)`.

Consequence: selecting a step or the stock in `🗿️artifact` (which does declare
`.interaction_domain(PROCESS3D_INTERACTION_DOMAIN)`, `🗿️artifact/🦀️.rs:93`) updates the inspection panel but
**never highlights anything in the 3D view**. Selection is one-way.

The doc comment that explains this away — `🪚️workpiece/🦀️.rs:65-67`, "unreachable at this `render` boundary
(`ArtifactEditor::render` carries no `InteractionView` — a known SDK gap)" — is **stale for the same reason the
inspection panel's was**: `render_with_request_context` exists and this app already overrides it. The selection
is in hand at `:1652`; it simply is not forwarded past `:1319`.

Fix shape (deliberately NOT applied yet — see below): forward `selected_ids` into `workpiece::render`, set the
instance's `selected` from whether `fixture.stock_id` is in the selection, and **extend `Process3dPreviewCache`'s
freshness key to include the selection** — it currently keys on `scene`/`resolved_up_to`/`label` only
(`🪚️workpiece/🦀️.rs:123`), so a selection-dependent payload would otherwise be served stale from the memo.

**Sequencing note.** This is held back on purpose. The open question this ticket must answer first is whether a
core rebuilt against today's framework still shows `runtime live cleanup faulted`. Introducing new plugin code
into that build would confound the experiment — a fresh failure could then be either the stale-core hypothesis
or my own new code. Verify the rebuilt core first, then apply this on a known-good baseline.

**Not ours, worth telling the owners.** The identical stale "no `InteractionView`" comment appears in 50+ places
across `📸️remodel`, `📐️cad`, `🌍️gis`, `🕸️dag`, `🎥️shooting`, `🪐️space`, `🧩️puzzle`, `🖍️draw` and others. Each is a
plugin whose viewport may have the same one-way selection.

### Correction on the selection gap — which channel is actually empty
`🪚️workpiece::render` **does** populate `MeshView::selection_json`, via
`process3d_selection_json(config.active_utility())`. That field is not the object-highlight channel: it carries
the marquee/selection **tool** (which utility is active — select/cut/drill/attach), the same slot `📸️remodel`
fills with `world3d_selection_json("rectangle", &[], None)`.

Object highlight travels on a different channel — the per-instance `"selected"` / `"hovered"` booleans inside
`instances_json` — and those are the hardcoded `Bool(false)` at `🪚️workpiece/🦀️.rs:104-105`. So the gap is real
but narrower than "the window gets no selection at all": the window is handed the active tool and never the
selected ids, because `process3d_render_body:1319` does not forward `selected_ids` to it.

That also means the fix does not need a new framework channel — `MeshView` already carries everything required;
only `instances_json` has to be built against the live selection, with the preview-cache key extended to match.

---

# 📅️ 2026-09-05 — checkpoint: what is settled, what is not

## ✅️ Settled by re-verification against today's tree
| Wave | Verdict | Evidence |
| --- | --- | --- |
| P0 crate compiles | holds | `cargo check -p semio-s-plugin-process --target wasm32-wasip2` → 0 errors |
| P1 fixtures | holds | decoded fixture: stock box **3.0 × 0.2 × 0.3** "Timber Beam", `resolvedUpTo=null`, **4 enabled steps** (crosscut / lap-joint-cut / dowel-drill / dowel-attach) |
| P2 seven step verbs | holds | each clones `step_payloads`, edits, returns `process3d_step_timeline_diff`; `UNOBSERVABLE` is now `&[]`; `"pending a link resolver"` has 0 occurrences |
| P3 inspection panel | holds | `render_with_request_context` override at `✏️editor/🦀️.rs:1644` threads live `"geometry"` selection |
| stdio gltf descriptor blocker | cleared by owner | fixed in `03100691d5` |

## 🆕️ Found here, not previously known
- **The 3D viewport can never show a selection.** `process3d_render_body:1319` calls `workpiece::render(doc, config)`
  with no selection argument (only `:1323`, the inspection panel, receives `selected_ids`), and
  `evaluated_preview_payload` hardcodes `"selected"`/`"hovered"` to `false`. `MeshView::selection_json` IS
  populated but carries the active *tool*, not the selected ids. Fix shape is recorded above; **not applied**,
  deliberately, so it cannot confound the stale-core experiment.
- **Two stdio rename-drift bugs, fixed here** (`📇️registry/🦀️.rs:257` and `:923`, `🧊️obj`→`🗽️obj`). These were
  blocking three sessions' builds, not just this one; semio-89 confirmed the fix unblocked their `sourcing` build.

## ⛔️ Not done: P4 live verification
The app has never been observed rendering a populated window in this session. Cause is environmental, and
specific: a Codex-driven applier rewrites files inside Vite's watch graph every few minutes (**8 restarts**
observed), each restart costs minutes on a box at load 90–170, and the shell needs several uninterrupted minutes
to boot ~20 WASM plugins. The serving window has so far been shorter than the boot.

**The honest state of the original question is therefore: still open.** Whether the empty windows of 09-02 were
a stale core or a framework fault has NOT been settled, because no rebuilt core has yet been produced — the
build has been blocked in turn by the emoji-rename race, an sccache stall, and lock contention with peers.

### The one substantive thing to carry into the next session
`🧑️‍💻dev`'s `%CPU` and a bound port are both unreliable signals here, and three separate wrong conclusions in this
session came from trusting them:
- a bound `:6022` with `curl /` returning `200` **while Vite was mid-restart and serving no modules** — check the
  entry module (`/🟦️.ts`), never `/`;
- a cargo at 0 % CPU that was working (live `rustc` child) vs one that was stuck (idle `sccache` child);
- a build log with 0 type errors that had **never type-checked** (0 warnings ⇒ aborted during expansion).

---

# 📅️ 2026-09-05 (13:20) — the selection fix, designed properly

Re-reading `🪚️workpiece/🦀️.rs` changed the shape of this fix, and the naive version would have been a
performance regression.

## ❌️ The naive fix is wrong
The obvious change — thread `selected_ids` into `workpiece::render` and add the selection to
`Process3dPreviewCache`'s freshness key — **would recompute the CSG replay on every selection change**.
`build_preview_cache` calls `processed_mesh` (fresh kernel session, every enabled step replayed as a real CSG
boolean, tessellation, face-group remap) *and* `processed_volume` (the same sequence again). Keying that memo on
selection means clicking a step pays for the whole process twice, on a path already suspected of crossing the
8 ms `INTERACTIVE_STEP_CEILING_US`. That would make the very fault this ticket is chasing *more* likely.

## ✅️ The right split
Selection affects only one boolean in a small JSON object; it does not affect geometry at all.
`evaluated_preview_payload` currently returns `(meshes_json, instances_json)` and both go in the memo, which is
what couples them. Split them by cost:

| value | cost | where it belongs |
| --- | --- | --- |
| `meshes_json` | CSG replay + tessellation | **stays memoized**, keyed on `scene`/`resolved_up_to`/`label` as today |
| `instances_json` | one small object: position, rotation, scale, label, `selected`, `hovered` | **built per render**, outside the memo |
| `volume` | second CSG replay | stays memoized |

Concretely:
1. `evaluated_preview_payload` → `evaluated_meshes_json(scene, resolved_up_to) -> String` (memoized) plus a free
   function `instances_json(fixture, selected: bool) -> String` (cheap, per-render).
2. `Process3dPreviewCache.payload: (String, String)` → `meshes: String`.
3. `pub fn render(fixture, config)` → `render(fixture, config, selected_ids: &[String])`, computing
   `let selected = selected_ids.iter().any(|id| id == &fixture.stock_id);`
4. `process3d_render_body` (`✏️editor/🦀️.rs:1319`) passes `selected_ids` to the `PROCESS_3D_PLAY_BODY_MAIN` arm,
   exactly as `:1323` already does for the inspection panel.

`hovered` stays `false`: there is no hover source at this boundary, and inventing one is out of scope. The stale
doc comment at `🪚️workpiece/🦀️.rs:65-67` gets corrected at the same time.

**Test to add first (this is the TDD order):** a case asserting that with `selected_ids = [stock_id]` the
rendered instances JSON contains `"selected":true`, and with `[]` it contains `"selected":false` — and, to pin
the performance property that motivated the split, that both calls share one memo entry (the mesh is not rebuilt).

## Sequencing
Not yet applied. A baseline `cargo test -p semio-s-plugin-process --lib` is queued behind the shared build lock;
the fix lands after that returns, so a regression is attributable. The previously recorded baseline is
**260 passed / 1 failed**, the single failure being `export_brep_out_returns_step_text_structured_payload`,
which was blocked on stdio and should now pass given the `🗽️obj` repairs made here.
