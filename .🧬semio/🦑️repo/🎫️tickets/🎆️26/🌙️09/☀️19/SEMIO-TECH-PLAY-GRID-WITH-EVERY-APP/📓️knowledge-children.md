# 📓️ knowledge-children — Architect / Writer / Animate composed-child content

Topic owner: fleet agent `knowledge-children` (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP).
Goal: the `#architect`, `#writer` and `#animate` panes of play must boot their `demo` example with
VISIBLE, correct content; root-cause the "composed `s.stdio.semio` derived children load EMPTY"
defect in production code and prove it with a per-app test through the host document archive door.

## 1. Root causes (all confirmed by running, not inferred)

### 1.1 The child content was never persisted (all three plugins)
`store::ArtifactChild<S>` persists ONLY `(child_id, target)`: its `local_owner` is skipped by
`ToValue`/`FromValue` and reconstructed as `None` on decode
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`, `impl FromValue for ArtifactChild`).
All three plugins kept the composed child's CONTENT somewhere ephemeral:

| plugin | where the content lived | why it vanished |
|---|---|---|
| ✒️writer | `ArtifactChildLocalText` in the handle's `local_owner` | skipped by every codec |
| 🎞️animate | a process-global `PRESENTATION_SCRATCH: HashMap<child_id, (source, tiles)>` | invisible to `ArtifactPack` |
| 🏛️architect | `ProgramKnowledgeWorkingTable`/`ProgramBenchmarksWorkingTable` in `local_owner` | skipped by every codec |

`setActiveExample` builds its document in the guest and hands it to the host as
`Effect::LoadDocument { pack: ArtifactPack::encode_pack(document), … }`. The encode drops the local
owner / scratch key, the host decodes the pack and calls `genesis_*_child_pack` on THAT snapshot, so
the derivation ran against an empty working scene and every composed child materialised EMPTY.

**Fix** (the in-repo precedent from this same ticket — `🏭️process`'s `genesis_process3d_child_pack`
deriving from the persisted `stock_payload`/`step_payloads`): persist the child's content as sibling
`#[state(artifact)]` payload fields on the PARENT snapshot + artifact + diff, and derive
`genesis_*_child_pack` from those fields.

- writer: `WriterSnapshot::text` / `WriterArtifact::text` / `WriterDiff::text`
- animate: `PresentationSnapshot::source` + `tiles` (same on artifact + diff); the process-global
  `PRESENTATION_SCRATCH` cache, `cache_presentation_working_scene`,
  `presentation_working_scene_for_handle` and `presentation_child_handle_and_cache` are GONE
- architect: `ProgramSnapshot::knowledge_payload` + `benchmarks_payload` (same on artifact + diff)

### 1.2 writer: the store initializer copied the snapshot field-by-field and never copied the body
`🚪️io/🧬️mutations/💾️binary/🦀️.rs` rebuilds the initial snapshot through a fixed 9-field string
cursor (`initial_field`/`initial_field_mut`) and retires it through the matching
`WriterSnapshotRetirement` cursor. Neither knew about the body, so even with the body persisted the
document that reached the store still had `text: ""`. Both cursors now carry `text` (10 fields).

### 1.3 writer: `setActiveExample` never knew its own published example id
`WriterPlayApp::dispatch`'s `SetActiveExample` arm matched only `"jack"` / `"dag.jack"` and fell
through to `empty_writer_snapshot()` — but the ONE id this subset publishes is
`crate::examples::demo::ID` = `"demo"`, which is exactly what the playground navbar boots with. Every
boot therefore reset the pane to an empty document. There were TWO copies of the map (the dispatch arm
and the unused `🎮️commands/🧺️set-active-example` leaf); they are now one
(`set_active_example::document_for_example_id`), and it knows `demo`.

### 1.4 the demo documents carried no content to show
- architect `sample_plugin()` had ZERO knowledge and ZERO benchmark rows → the composed children were
  legitimately empty. It now carries one curated knowledge record and one benchmark record.
- animate `setActiveExample("demo")` loaded `default_snapshot()`, a source figure with NO tile crops →
  an empty tile editor. Added `demo_snapshot()` (the same 3×5 grid `resetGrid` seeds);
  `default_snapshot()` stays tile-less so the fresh/empty document contract is unchanged.
- writer's demo asset carried only the opaque `document=[childId,target]` pair; the body it was minted
  from lived in two `const … QUERY_TEXT` strings in the codec file (the file's own doc comment
  documented that gap). The assets now carry `text=<hex>` and those consts are gone.

### 1.5 architect: `exportRegistersCsv` exported registers `importRegistersCsv` refused
`collect_rows` exports all 68 registers including `knowledge`/`benchmarks`; `upsert_register_row`
errored `unsupported register import: <r>` for everything except 5. With the demo's new rows the
round-trip broke. Added `upsert_knowledge_stub`/`upsert_benchmark_stub` (same honest header-only stub
shape as the pre-existing `upsert_adjacency_stub`), which also re-mint the parent's child handle.

## 2. What was changed (absolute paths)

Framework: **nothing**. Every change is inside the three plugins.

### ✒️writer
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🦀️.rs` — `writer_text`/`writer_text_owner`/`writer_snapshot_with_text`/`genesis_writer_child_pack` read the persisted body
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` — new `WriterSnapshot::text`
- `…/🧬️schema/🦀️.rs` — `WriterArtifact::text` + all three projections
- `…/🧬️schema/🔺️diff/🦀️.rs` — new `WriterDiff::text`
- `…/🚪️io/🔺️diff/📝️text/🦀️.rs` — apply/absorb/`diff_set_text` carry the body with the handle
- `…/🚪️io/📸️snapshot/📝️text/🦀️.rs` — DSL `text=<hex>` line; the two `*_QUERY_TEXT` consts deleted
- `…/🚪️io/📸️snapshot/💾️binary/🦀️.rs` — pack carries the body
- `…/🚪️io/🧬️mutations/💾️binary/🦀️.rs` — **the store initializer + retirement field cursors gained `text`** (9 → 10 fields)
- `…/✏️editor/🦀️.rs` + `…/✏️editor/🎮️commands/🧺️set-active-example/🦀️.rs` — ONE `document_for_example_id` map that knows `demo`
- `…/🧬️schema/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}` and the `📸️snapshot/`+`🔺️diff/` leaves — schema-first leaves updated
- assets: `…/🖼️assets/🎬️demo/🗣️.dsl.semio`, `…/📚️examples/🎬️demo/🖼️assets/🧪️dag-example/🗣️.dsl.semio`
- fixtures: the 4 mutation families' `📸️snapshot/{⬅️before,➡️after}` + `🔺️diff` JSONs
- tests: `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (archive-door law extended), the `edit-text` fixture leaf and `…/🧪️tests/✒️mutate-writer-1/🦀️.rs` (the now-dead local-text seeding removed)

### 🎞️animate
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🦀️.rs` — scratch cache deleted, `presentation_working_scene` reads the snapshot, new `demo_presentation_snapshot()`
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` — `source`/`tiles` fields, `default_snapshot()` (empty) vs `demo_snapshot()` (3×5 grid)
- `…/🧬️schema/🦀️.rs`, `…/🧬️schema/🔺️diff/🦀️.rs`, `…/🚪️io/🔺️diff/📝️text/🦀️.rs` — artifact + diff carry the payload
- `…/🚪️io/📸️snapshot/{📝️text,💾️binary}/🦀️.rs` — DSL `source=`/`tiles=` lines and pack fields
- `…/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs` — `demo` loads the demo deck
- assets: `…/🖼️assets/🎬️demo/🗣️.dsl.semio` regenerated **by this crate's own printer**
- fixtures: 9 mutation families × `{⬅️before,➡️after}` + their `🔺️diff` JSONs
- tests: the archive-door law extended; a new `the_committed_demo_asset_is_the_printers_own_demo_snapshot` law; the 6 fixture leaves now ASSERT the committed payload instead of seeding a process-global cache; the test context is a self-closing `PresentationApp` newtype that binds an instance id and settles every dispatch (see §4)

### 🏛️architect
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🦀️.rs` — payload accessors, genesis from the payload, `sample_plugin()` demo rows
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`, `…/🧬️schema/🦀️.rs`, `…/🧬️schema/🔺️diff/🦀️.rs`, `…/🧬️schema/🔺️diff/📝️text/🦀️.rs` — payload fields + apply/absorb
- the 8 knowledge/benchmark mutation `🔺️diff` leaves — the diff carries its payload beside the handle
- `…/✏️editor/🦀️.rs` — `upsert_knowledge_stub` / `upsert_benchmark_stub`
- `…/🧬️schema/**/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}` (artifact, snapshot, diff)
- assets: `…/🖼️assets/🎬️demo/🗣️.dsl.semio` regenerated **by this crate's own printer**
- fixtures: 533 snapshot + 260 diff fixture JSONs gained the two payload keys; the two `create` families' after/diff regenerated from the crate's own encoder
- tests: the archive-door law extended; the empty-register placeholder test moved to a register the demo really leaves empty

## 3. Evidence — every number below came from a run I made

Logs (all under `$T/🗑️generated/knowledge-children/`). Cargo env for every run:
`CARGO_TARGET_DIR=…/⚡️cache/cargo/target-knowledge-children CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2
RUST_MIN_STACK=33554432 DEVELOPER_DIR=/Library/Developer/CommandLineTools`, `-- --test-threads=4`.

| crate | first run I completed | final | log |
|---|---|---|---|
| `semio-s-artifact-architect-program` | 1872 failed / 2085 (my fixture churn, before the fixture keys landed) | **2072 passed / 13 failed** | `architect-2.txt` → `architect-4.txt` |
| `semio-s-plugin-architect` | 3 passed / 0 failed | 2 passed / **1 failed** (`descriptor_is_fresh`) | `all-1.txt` → `architect-4.txt` |
| `semio-s-artifact-animate-presentation` | 39 FAILED + SIGABRT (324 tests) | **26 FAILED** + SIGABRT | `animate-1.txt` → `wa-1.txt` |
| `semio-s-plugin-animate` | 2 passed / 1 failed | 2 passed / 1 failed (`descriptor_is_fresh`) | `all-1.txt` → `wa-1.txt` |
| `semio-s-artifact-writer-writer` | 36 FAILED + SIGABRT (174 tests) | **35 FAILED** + SIGABRT | `writer-3.txt` → `wa-1.txt` |
| `semio-s-plugin-writer` | 5 passed / 1 failed | 5 passed / 1 failed (`descriptor_is_fresh`) | `all-1.txt` → `wa-1.txt` |

A true pre-edit baseline does not exist for these crates: the batch I launched at 13:59 sat ~2 h on
the shared build-dir lock and only started compiling after the first edits landed (`first-run.txt`
ends in `EXIT=101` on my own half-applied animate edit). The coordinator's `baseline/summary.tsv`
never reached these six crates either.

### The deliverable itself — GREEN, run, not claimed
- `editor::writer::component::unit_tests::demo_example_load_settles_through_the_host_document_archive_door` ... **ok**
- `editor::animate::component::unit_tests::demo_example_load_settles_through_the_host_document_archive_door` ... **ok**
- `editor::architect::component::unit_tests::demo_example_load_settles_through_the_host_document_archive_door` ... **ok**
- `examples::demo::component::example::every_demo_asset_is_the_printers_own_content_addressed_output` (writer) ... **ok**
- `standards::v1::subsets::any::io::snapshot::text::tests::the_committed_demo_asset_is_the_printers_own_demo_snapshot` (animate, NEW) ... **ok**

Each archive-door test now asserts more than "the member is live": it decodes the member
`genesis_*_child_pack` derives FROM THE LOADED SNAPSHOT and asserts it is non-empty and equal to the
loaded content (writer: the document's code-block body == the loaded body; animate: one slide per
loaded tile + a master; architect: one table row per loaded knowledge/benchmark record). Before the
fix these assertions fail — the writer one did, and it is what caught root cause §1.2.

## 4. What is still red, and exactly why

### 4.1 architect — 13 of 2085 (none introduced here)
- 4 × `connect-adjacency`/`disconnect-adjacency` "committed … JSON is not canonical" — fleet-brief
  stale-test bucket 1 (float canonical form): the committed fixture holds `"weight": 0`, the crate's
  encoder emits `0.0`. **Verified unchanged in git** (`git show HEAD:<fixture>` still `"weight": 0`),
  so this predates this topic. Fix = the `Number(0)` vs `Number(0.0)` decision, not a knowledge/benchmark concern.
- `create_stakeholder_obeys_the_inverse_and_absorb_laws` — `mutation.apply.missing-target` on
  `stakeholders.patched[0]`, nothing to do with the composed children.
- 7 × `interactive-job.live-instance` / `artifact store reached Drop without its exact terminal-empty
  shallow-shell witness` / "every window kind must inherit the app-level setActiveExample" — fleet-brief
  buckets 2+3: architect's test context never binds an instance id, never settles, never closes.
  The same fix I applied to animate (§4.2) is the recipe; it is a self-contained follow-up.

### 4.2 animate — 26 of 324 (down from 39; the binary still SIGABRTs)
Fixed here: the context now binds an instance id, settles every `dispatch_typed` and applies the
`LoadDocument` effects, and `PresentationApp` is a self-closing newtype (Deref/DerefMut + Drop →
`close_registered_fixture_app`, skipped while panicking). That cleared every
`interactive-job.live-instance` refusal. What remains is the NEXT layer of the same bucket:
`close_registered_fixture_app` itself reports `registered fixture did not reach its exact
terminal-empty witness` (framework `🔌️plugin/🦀️.rs:7417`), i.e. animate's stores still have no
owners-installing retirement guard (bucket 2, the `♻️retirement` module pattern from block3d /
trinity jack). One test binary abort also remains in
`retained_presentation_envelope_caller_faults_and_zero_grant_closes_malformed_pack`
(`registry.fault(...).is_some()` — the malformed-pack caller no longer reaches `Fault`). Also
`command_ids_are_unique` asserts 17 == 18: a peer added an 18th `PresentationCommand` row that
`every_command()` does not list — peer-owned, not touched.

### 4.3 writer — 35 of 174 (was 36; the ticket's own note says "writer 44 red")
Same two buckets (`interactive-job.live-instance` on the mounted app, and the
`Writer mutation array / Writer edit decode reached Drop before …` retirement guards that SIGABRT the
binary). `writer_into_json_round_trips_through_json_into_writer` /
`json_into_writer_round_trips_a_real_snapshot` fail with `JsonIntoWriter: unexpected byte 115 at
offset 0` (byte 115 = `s`, i.e. the deserializer is handed `.dsl.semio` text, not JSON) — a wrong-input
defect unrelated to the new field.

### 4.4 `descriptor_is_fresh` (all three plugin crates)
`🛂️.descriptor.semio` must be refreshed with `describe`, which is a wasm32 build and therefore behind
the fleet wasm mutex. The mutex is held by a multi-hour peer release build with 9 tickets queued, so
on the coordinator's instruction my queued `cargo check --target wasm32-wasip2` for the three artifact
crates was **cancelled and is UNVERIFIED** — the coordinator's activation compiles the same code.
`touch "$T/🗑️generated/activate.request/{architect,writer,animate}"` is in place for the re-serve.

### 4.5 Known new constraint (fail-closed, strictly better than before)
The writer store initializer refuses a single snapshot string longer than
`WRITER_ENVELOPE_FIELD_BYTES` (4 096) with `writer-store.initializer-initial-field-too-large`. Now
that the body is one of those strings, a writer document over 4 KiB is refused at store
initialisation instead of silently losing its text (which is what happened before). The clean
follow-up is the paging cursor from the `Retirement Cursor Must Page Oversized Strings` recipe.

### 4.6 Not done
Browser verification on `:6033` — the panes cannot show the fix until the changed plugin lanes are
re-activated, which is the coordinator's step. No browser claim is made here.
