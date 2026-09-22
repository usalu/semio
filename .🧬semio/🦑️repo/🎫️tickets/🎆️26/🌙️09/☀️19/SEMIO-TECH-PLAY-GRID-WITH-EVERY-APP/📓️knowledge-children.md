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

---

## 5. 2026-09-22 (successor session after the coordinator restart)

### 5.1 State found
`pass2-1.txt` — the predecessor's ONE batched verification run of the second pass — never completed
(360 bytes, last line `Compiling serde`), so every second-pass number in §4 above was **unverified**.
`git diff --stat` of the three plugin dirs showed 9 STAGED-but-uncommitted files (last commit
`ef2210a4`, 21:41) and NO half-applied edit: the architect editor/window-ownership tests re-pointed at
`definition.actions`, two adjacency fixtures' `"weight": 0` → `0.0`, the `added`-before-`patched`
ordering fix in `🧬️schema/🔺️diff/📝️text/🦀️.rs`, `connect_and_disconnect_adjacency_round_trip` starting
from a cleared adjacency list, the writer archive-door `&mut *app` reborrows, and the removal of the
temporary `zz_dump_adjacency_fixtures` dumper.

### 5.2 BROWSER VERDICTS — run, not claimed (`🗑️generated/knowledge-children/panes/`)
Headless Chromium (`--use-angle=metal`), one page, against the 03:04 activation on `:6033`
(`🗑️generated/knowledge-children/probe-panes.mjs`, `probe-animate-inspection.mjs`).

| pane | shell | console/page errors | content |
|---|---|---|---|
| `#architect` | `data-shell-ready=architect`, 1070 nodes | **0** | ✅ Adjacency tree (`Reception`, `Waiting`, `PAIRS`, `Reception ↔ Waiting [Required]`), Register window with both elements, Graph + Report windows. Example picker on **Demo**. |
| `#writer` | `data-shell-ready=writer`, 514 nodes | **0** | ✅ the curated demo body is VISIBLE: `MATCH (a:Piece)-[r:Connection]->(b:Piece)` / `WHERE a.name = "core"` / `RETURN a.name, b.name`. Example picker on **Demo**. One UI diagnostic in the status bar (§5.3). |
| `#animate` | `data-shell-ready=animate`, 452 nodes | **0** | ⚠️ the deck IS loaded — Inspection reads `Schema animate.presentation` / **`Tiles 15`**, and the Artifact panel lists all 15 `tile-rN-cM` rows with their crops — but the Tile-editor CANVAS is blank (§5.4). |

So the topic's own defect (composed `s.stdio.semio` derived children load EMPTY) is **fixed in the
browser** for all three panes: the persisted payload fields survive the archive door and the panes
boot their curated `demo`.

### 5.3 writer — the demo body lints as invalid (secondary, NOT a content failure)
The status bar shows `unexpected character '-'`. That string is produced by the FRAMEWORK core DSL
lexer, `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🔍️lexer/🦀️.rs:674` — the writer editor lints a
`languageId = "jack"` body with the core semio-DSL grammar, which has no `-` token, while the crate's
canonical jack sample (repeated verbatim in six test leaves) is
`MATCH (a:Piece)-[r:Connection]->(b:Piece) …`. The document renders correctly; only the diagnostic is
wrong. Pre-existing gap, newly VISIBLE because the pane no longer boots an empty document. Owner:
whoever owns the jack grammar / the writer language-service selection, not this topic.

### 5.4 animate — the tile canvas is blank because the demo FIGURE is not served
Proven by the probe's response log: `GET http://localhost:6033/🖼️bauteilbörse.png` → **`200 text/html`**
(the Vite SPA fallback), i.e. the file does not exist. `default_figure_tile_source()`
(`✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🦀️.rs:71`) points every tile crop at
`/🖼️bauteilbörse.png`; the same path is referenced by the `🎤️presentation` product's own tests, and no
`bauteilbörse` image exists anywhere in the repo. The sibling `/🖼️assets/👆️cursor/**.svg` requests DO
resolve (served from `🧰️framework/🔨️modules/🖼️assets`), so the host serves an asset tree — the figure is
simply missing from it. Fix (not taken here, it needs an authored asset): ship the demo figure as an
animate subset asset under `🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/` and
point `default_figure_tile_source` at its transport URL, so the deck is never invisible.

### 5.5 Source fixes landed this session (compile/run verification queued, see §5.7)
1. **writer, PRODUCTION** — `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs`:
   `JsonIntoWriter` parsed the payload into a `JsonSnapshot` and then fed
   `store::ArtifactDsl::print_dsl(&json)` — the s.stdio.json ARTIFACT TEXT, whose first line is the
   `semio stdio.json.dsl v1` preamble — back into a JSON reader. That is exactly the observed
   `JsonIntoWriter: unexpected byte 115 at offset 0` (115 = `s` of `semio`) that fails BOTH
   `writer_into_json_round_trips_through_json_into_writer` and
   `json_into_writer_round_trips_a_real_snapshot`. Now decodes the snapshot's VALUE
   (`json.to_pack_value()` → `dsl::os_pack::json::to_string`), which is what the leaf's own doc
   comment always described. `WriterSnapshot` is all-`String`, so the value hop is lossless.
2. **animate, TEST** — `✏️editor/🧪️tests/🔬️unit/🦀️.rs`: `command_ids_are_unique` asserted `18`.
   `git show 025ec86a42` proves the 18th row was `setLocale`, which was removed when the locale moved
   to the window config; `every_command()` and the wire-keyword table were already reduced to 17 and
   only this literal was missed. Both now read one `PRESENTATION_COMMAND_ROW_COUNT = 17`.
3. **animate, FIXTURE** — `✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json` was stale on BOTH
   axes: kebab-case dispositions (`migrated`, `batch-only-pending-rewrite`) where the law reads
   PascalCase, and only 3 migrated rows where the crate now declares 16
   (`ANIMATE_PRESENTATION_RETAINED_TOOL_IDS` + `…_PUBLICATION_CONTRACTS`). Regenerated against the
   crate's own contracts (lanes `Artifact`/`Config`/`HostOnly`, `exportVideoFromDeck` stays
   `BatchOnlyPendingRewrite` with its blocker), matching the passing sibling shape in
   `🪐️space/🗿️artifacts/🏠️home/…/🧫️retained-command-limits/🔣️.json`.
4. **writer, TEST** — `🚪️io/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs`:
   `writer_snapshot_and_mutation_owners_retire_one_exact_field_per_grant` asserted 9 released fields;
   the retirement cursor's `take_field` ladder is 10 phases since `WriterSnapshot::text` landed
   (5 snapshot strings + 5 child-reference strings).

### 5.6 Machine notes (why no native numbers yet)
- 01:31–02:24 (`pass2-2.txt`): stalled at 293 compiled units for 30 min — 15 cargos ALL sampled inside
  `cargo::core::compiler::prebuild_lock_exclusive → LockManager::lock → flock`, 0 rustc, 0 writes under
  the shared build dir for 10 min. Killed ONLY my own cargo; rustc reappeared within 5 s.
- 02:26 relaunch (`pass2-3.txt`): `EXIT=143` — the peer's 02:27 fleet cut (coordinator's 02:40 note).
- 02:28–03:28 (`pass2-4.txt`): parked at 12 units behind the play activation; cut on the coordinator's
  instruction and requeued through the new fleet-wide native mutex.
- Current run: `🗑️generated/knowledge-children/pass2-5.txt`, queued as `…-knowledge-children` in
  `/tmp/semio-play-native-test.queue` behind raster / cad-content / design.
  `run-tests.sh` now wraps every invocation in `📜️native-test-mutex.sh` with
  `CARGO_TARGET_DIR=$T/🗑️generated/knowledge-children/target`, `CARGO_BUILD_JOBS=4`.

### 5.7 NATIVE NUMBERS — second pass verified (`🗑️generated/knowledge-children/pass2-5.txt`, 04:2x)
One batched run through the new fleet native mutex (`📜️native-test-mutex.sh knowledge-children`),
private `CARGO_TARGET_DIR`, `CARGO_BUILD_JOBS=4`, `-- --test-threads=4`, `EXIT=101`.

| crate | §4 (pass 1) | **pass2-5 (measured now)** |
|---|---|---|
| `semio-s-artifact-architect-program` | 13 failed of 2085 | **1 failed / 2084 passed** |
| `semio-s-plugin-architect` | 1 failed (`descriptor_is_fresh`) | 1 failed — still `descriptor_is_fresh` |
| `semio-s-artifact-animate-presentation` | 26 FAILED + SIGABRT | **14 FAILED** + SIGABRT (of 324) |
| `semio-s-plugin-animate` | 1 failed (`descriptor_is_fresh`) | **3 passed / 0 failed — GREEN** |
| `semio-s-artifact-writer-writer` | 35 FAILED + SIGABRT | **24 FAILED** + SIGABRT (of 174) |
| `semio-s-plugin-writer` | 1 failed (`descriptor_is_fresh`) | **6 passed / 0 failed — GREEN** |

The coordinator's describe pass refreshed the animate and writer descriptors (`descriptor_is_fresh`
green on both); **architect's `🛂️.descriptor.semio` is still stale** — `🗑️generated/activate.request/architect`
touched at 03:55.

#### architect — ONE red left
`editor::architect::component::unit_tests::interaction_select_stamps_the_picked_element_as_selected_in_the_document_panel`
— "the picked element must be stamped selected by the framework wrapper" (`🔬️unit/🦀️.rs:449`): after a
`framework_verb("interactionSelect", …)` the document panel's rendered tree carries no `"selected":true`.
Fleet-brief stale-test bucket 5. Everything the second pass changed (mounted newtype + settled dispatch,
`addElement`/`removeElement`/`setAdjacencyField` reclassified from `view_action` to `.mutation(…)`, the
`nodeGraphViewport` absent-viewport default, the `definition.actions` roster reads, the regenerated
adjacency fixtures and the `added`-before-`patched` apply order) is now GREEN.

#### the two SIGABRTs (both truncate their binary, so 14 / 24 are PARTIAL counts)
- animate: `retained_presentation_envelope_caller_cancels_and_zero_grant_closes_without_output` —
  `close_presentation_registry` panics "Presentation registry did not reach terminal empty within the
  fixed fixture ceiling" (100 000 bounded close steps), then
  `PresentationEnvelopeMaterializeRegistry::drop` asserts (`💾️binary/🦀️.rs:1533`) → double panic → SIGABRT.
  The *fault*-path abort §4.2 reported is gone; the CANCEL path is the one that never closes now.
- writer: `writer_edit_history_decoder_uses_begin_mutation_and_faults_malformed_input` — "Writer mutation
  array reached Drop before every exact mutation owner was published or cursor-retired"
  (`💾️binary/🦀️.rs:767`) + "Writer edit decode reached Drop before exact publication or bounded
  retirement" (`:1045`). `retire_writer_edit_authority` runs on every exit path of `drive_writer_edit`,
  so the REFUSAL path leaves an owner the bounded close ladder cannot reclaim.
Both are fleet-brief bucket 2 (owners-installing retirement guard) inside retained close cursors.

### 5.8 architect live-host verdict — the derived children DO materialise
`📓️audit-visual-2.md` marks `#architect` "content-degraded — still stuck on the Waiting !… !… placeholder".
Re-probed on the 03:04 activation (`probe-architect.mjs`, `probe-architect-dom.mjs`,
`probe-architect-graph.mjs`, `probe-architect-registers.mjs`; artefacts in `🗑️generated/knowledge-children/panes/`):
1. `data-shell-ready=architect`, **0** console `[error]`, **0** `[pageerror]`, and NO
   `refused|rejected|Incomplete|closure|authority` line anywhere. A failed `genesis_*_child_pack` would
   refuse the replacement with `document-archive-replacement.closure-rejected` — it does not appear.
2. The pane shows the demo document: Adjacency window `Reception`, `Waiting`, `PAIRS`,
   `Reception ↔ Waiting [Required]`; Register window with both elements; example picker on **Demo**.
   So `setActiveExample("demo")` → `Effect::LoadDocument` → archive door completes on the live host.
3. The SHIPPED descriptor carries the persisted payload: the committed
   `✏️s/🔌️plugins/🏛️architect/🛂️.descriptor.semio` contains the demo example's DSL with
   `{id=knowledge-8 name="Clinic Reception Sightlines" …}` and the `Outpatient Waiting` benchmark, i.e.
   the `knowledgePayload`/`benchmarksPayload` fields ARE in the example the pane boots. (It is stale for
   OTHER reasons — `descriptor_is_fresh` — hence the activation request.)

**The audit's verdict is a CANVAS-crop verdict, and the crop is the Graph window.** That window is
`surfaceId="window:architect-graph"`, a `semio-node-graph-host` canvas — NOT a composed child. Measured
defect there, independent of this topic: `🪟️windows/🕸️graph/🦀️.rs::graph_media_json` lays elements out on a
circle with `center_x = 320.0`, `radius = 220.0`, so with the demo's TWO elements the nodes land at
x = 100 and x = **540** (width 108). The Graph window in the play grid measures **469 px** wide
(`getBoundingClientRect` → `[485, 61, 469, 907]`) and `ArchitectGraphWindowConfig::default().viewport`
is `{x: 0, y: 0, zoom: 1}` — so the second node is entirely outside the viewport and only one label
(`Waiting`) plus its two unlabelled ports (`!…  !…`) can ever be in view. The layout must fit the
window (or the default viewport must frame the node bounds). Handing this over rather than fixing it
blind: it is the Graph window's layout contract, not the composed-child path this topic owns.

---

## 6. 2026-09-22 (session 7) — the two SIGABRTs, the graph fit and the demo figure

Successor of the 04:30 session. Predecessor edits verified on disk first
(`git diff HEAD --stat` over the three plugin dirs, 15 files, all coherent).

### 6.1 Native numbers actually measured

Every run through `📜️native-test-mutex.sh knowledge-children`, private
`CARGO_TARGET_DIR`, ONE cargo invocation for all six crates, `--no-fail-fast` before the `--`.

| crate | pass2-5 (04:2x) | pass3-2 (12:58) | **pass3-3 (13:47)** |
|---|---|---|---|
| `semio-s-artifact-architect-program` | 2084 / 1 | 2086 / 1 | **2087 passed / 0 failed — GREEN** |
| `semio-s-plugin-architect` | 1 (`descriptor_is_fresh`) | 1 (same) | 2 / 1 (same) |
| `semio-s-artifact-animate-presentation` | 14 red + SIGABRT | 14 red + SIGABRT | 15 red + SIGABRT (of 324; 187 ok printed) |
| `semio-s-plugin-animate` | GREEN | **3 / 0** | **3 / 0** |
| `semio-s-artifact-writer-writer` | 24 red + SIGABRT (partial) | **138 passed / 35 failed, NO abort** | aborts on its FIRST failing test |
| `semio-s-plugin-writer` | GREEN | **6 / 0** | **6 / 0** |

Logs were `🗑️generated/knowledge-children/pass3-{1,2,3}.txt`; the folder was swept at 16:34 by
another agent, so the numbers above are transcribed here (a tracked file) rather than cited.
`pass3-1` never produced numbers: it died in PEER churn — `semio-framework-plugin` failed to compile
(`pattern does not mention field 'tasks'`, `no field 'reserved' on TaskSlot` ×5) while the
MICROKERNEL-POOLED-ACTOR runtime was mid-landing; the peer fixed both halves within four minutes.

### 6.2 The two aborts — root causes, both in PRODUCTION code, both fixed and proven

**animate — the CANCEL path could never be reclaimed.**
`semio_framework_job::WorkerJobSession::close_step` only advances a session whose phase IS
`SESSION_CLOSE`; against any other phase it answers `Blocked` and changes nothing. `begin_close` is
the only transition into that phase, and *while the worker holds the session it answers `Blocked`
too* — it records the request, wakes the worker, and the worker then hands the authority back as
`SESSION_TERMINAL`/`SESSION_OUTCOME`, **never** as `SESSION_CLOSE`. The handle issued `begin_close`
exactly once and latched `close_started`, so a caller whose first `begin_close` was blocked answered
`Blocked` for every later step: the registry never reached terminal-empty, the fixture's 100 000-step
ceiling tripped, and `PresentationEnvelopeMaterializeRegistry::drop` asserted on top of that panic —
`panic in a destructor during cleanup` → SIGABRT for the whole binary.
Fix: both blocked arms (`retire_session_step`, `close_step`) now re-issue `begin_close` through the
new `reissue_session_close` (idempotent against `SESSION_CLOSE`).
Proof: `retained_presentation_envelope_caller_cancels_and_zero_grant_closes_without_output ... ok`
in `pass3-3` (it had aborted the binary in every previous run).

**writer — a REFUSAL dropped a live nested owner.**
`WriterEditHistoryAuthority::accept_token` did `self.active.take()` and then `?`-propagated a nested
fault, so the taken `WriterEditActive` was DROPPED on the spot. `WriterMutationArrayAuthority::drop`
(and `OwnedSchemaStringAuthority`'s) assert terminal-empty, so a refused edit raised "Writer mutation
array reached Drop before every exact mutation owner was published or cursor-retired" *inside the
caller's unwind*, and this authority's own Drop assert turned it into the same double-panic SIGABRT.
Fix: every arm now yields `(outcome, retain)`; a refused owner is retained exactly like a pending one
and leaves through this authority's bounded `close_step` ladder, which already handles the faulted
array. Proof: `pass3-2` produced writer's FIRST ever complete `test result:` line (138/35, no abort).

### 6.3 architect — the Graph window now fits its pane (§5.8's handover, closed)

`graph_media_json` laid every program out on a FIXED circle (`radius = 220`, centre `(320, 240)`) and
used the ring point as the node rect's ORIGIN, so a two-element program put its second node's left
edge at x = 540 with width 108 — in a 469 px pane whose `ArchitectGraphWindowConfig::default()`
viewport is `{0, 0, 1}`. Only one label plus two unlabelled ports were ever in view: the audit's
"Waiting !… !…" verdict.

Now (`🪟️windows/🕸️graph/🦀️.rs`): `graph_ring_radius(count)` is the smallest radius whose
neighbour-to-neighbour CHORD clears a whole node box measured on its DIAGONAL plus
`ARCHITECT_GRAPH_NODE_GAP`; the ring centre is anchored so the node bounding box starts at
`ARCHITECT_GRAPH_MARGIN`; and x/y are the box's top-left, not the ring point. Two elements now span
x ∈ [24, 272] — inside the pane, with the default viewport untouched.
Laws: `the_default_viewport_frames_every_node_without_panning` (positive quadrant, inside the
narrowest measured pane, exact margin/extent on the axis where `cos` reaches −1) and
`the_ring_radius_keeps_neighbouring_node_boxes_clear` (2…32 elements, monotone).

### 6.4 animate — the demo figure is a REAL shipped asset now

`default_figure_tile_source()` pointed at `/🖼️bauteilbörse.png`, which exists NOWHERE in this
repository, so every host answered the figure request with its SPA fallback (`200 text/html`) and the
booted deck painted fifteen blank tiles with no console error to explain it — the acceptance red
`📓️play-runtime.md` records against the `animate` pane.

It now names `DEMO_FIGURE_SRC = "/🖼️assets/🖼️images/🏙️architecture/🏘️habitat-67.png"` (2560 × 1707),
a file this repository ships and `semioAssetsVitePlugin` serves — and a release build copies — at
`/🖼️assets/*` for every host that mounts this app: the `semio-tech play` grid, animate's own
playground and the CDN bundle alike. No registry, Cargo-manifest or play-config change is needed,
which is why this beat declaring a per-plugin `static-dir` route. 17 JSON fixtures were rewritten
(`src` + `sourceAspect` 1.3638392857142858 → 1.4997070884592854).
Law: `the_demo_figure_source_names_a_shipped_asset` — the src must sit under the shared asset route
AND resolve to a file on disk, and the declared aspect must be the shipped figure's pixel aspect.
The committed `🖼️assets/🎬️demo/🗣️.dsl.semio` carries a content-HASHED composed-child id, so no hand
edit of it can be correct: a new `zzz_write_demo_example_asset` writer test regenerates it from this
crate's own printer (its law `the_committed_demo_asset_is_the_printers_own_demo_snapshot` reads the
`include_str!` constant of the SAME binary, so the regenerated text is proven by the NEXT run).

### 6.5 Stale laws restated (never weakened)

- **architect `interaction_select_stamps_…`** (fleet bucket 5): the framework stopped writing
  `"selected":true` into a rendered row — `stamp_and_cache_interaction_ui`'s presence-stamping half
  became the render-plane presence OUTBOX. The law now asserts BOTH the one tree-level
  `"interactionDomain":"program"` binding that makes the rows pickable AND that
  `PluginApp::take_pending_presence` carries exactly one `own.selected` entry whose `node_key` is the
  picked element id. GREEN.
- **writer `drain_typed_operations`** (fleet bucket 6, 20 reds): the editor test context re-rolled the
  retained-operation pump and never drained `take_typed_operation_completion`, the composed-result
  outbox or the interaction-query replies — all three of which `has_pending_typed_operations` COUNTS —
  so every command whose operation published no further lane page spun to the 30 s deadline ("Writer
  retained operations did not finish"). It also took ONE page per turn where the framework helper
  drains every presented page. Replaced with `artifact_app_laws::settle_registered_typed_operation`.
- **writer `app_with_jack`** (4 reds): printed a document pack and then dropped the envelope;
  `ArtifactEnvelope::drop` asserts its owners were detached by an app-owned bounded retirement
  authority. New `retire_writer_envelope` runs `crate::spr::writer_document_store_owners()
  .retire_envelope_uninstalled(..)`, the shape `🖨️raster`'s `retire_raster_envelope` already uses.
- **writer `command_surface_has_the_expected_row_count_…`**: literal `21` → `WriterCommand::TOOL_JOB_IDS.len()`
  (the surface is 19 rows since `ast-hover`/`text-hover` dissolved into the framework `ast` domain).
- **writer's two nested window laws**: they filed a BARE node key (`"match"`), which only ever matches
  a TOP-LEVEL container — `TreeWindows::path_of` joins enclosing windowed containers with
  `TREE_WINDOW_PATH_SEPARATOR`. New `ast_window_path` files `root␟match`, the shape raster's
  `nested_key` and cad's nested requests use. (That miss is why the closed law materialised 47 rows
  instead of 0 and the window law read offset 0 instead of 30.)
- **writer's two label laws**: they asserted `"Document"`/`"Camera"`, neither of which is in
  `WriterPlayLabels` (the artifact term has always been `"Artifact"`) and the inspection panel renders
  no camera section — they failed on their own vocabulary, not on a locale regression. Now: native
  English `"Artifact"` present, German `"Artefakt"` absent.
- **writer's three example-id laws**: `document_for_example_id` publishes exactly ONE id
  (`crate::examples::demo::ID`), treats the EMPTY id as "load my default example" (animate does the
  same) and resets to the empty document for anything else. The laws dispatched `"jack"` (never
  published → empty document, so `lint_is_a_view_action_and_example_default_materializes` proved the
  opposite of its own name and its failure then tripped the store Drop witness into a SIGABRT) and
  `""` while expecting the empty fallback. They now dispatch the published id, and the fallback law
  files an id the app genuinely does not publish.

### 6.6 Live panes — the 15:47 activation

The coordinator's chain landed `activate-dev rc=0` at 15:47 and the supervisor restarted :6033 at
15:57. First probe (16:05, `probe-kc.mjs`, headless chromium `--use-angle=metal`):

- **animate — the figure fix is PROVEN live.** The response log records
  `200  http://127.0.0.1:6033/🖼️assets/🖼️images/🏙️architecture/🏘️habitat-67.png` — a real asset body,
  NOT the `200 text/html` SPA fallback the same request answered on every earlier activation. Pane
  reached `data-shell-ready=animate` (452 nodes) with the Demo example selected; zero bad-asset
  responses. `curl` against the same URL returns 200 / 985 708 bytes.
- **architect / writer** did not finish their boot inside 180 s on that first (cold) pass, and every
  later probe (16:11, 16:16, 16:24) ended the same way for ALL THREE panes with one console error:
  `Framework OS boot failed Error: plugin-handle.closed` (`🔌️PluginRuntime/🟦️.tsx:1857`
  `requireOpening`). It is identical across the three panes, carries no plugin-specific code, and
  appears on a pane (animate) that had booted fine minutes earlier — a host-side boot fault on this
  activation under fleet load, not a knowledge-children defect. **Not verified visually this session:
  the architect Graph fit and the writer pane.** The native law pins the graph geometry exactly
  (§6.3); the pane screenshot is the piece that is still owed.

### 6.7 What remains

1. `animate-presentation` — 15 reds + one abort:
   `retained_presentation_envelope_caller_faults_and_zero_grant_closes_malformed_pack` fails
   `registry.fault(..).is_some()` and then aborts on the registry Drop assert, and
   `envelope_helpers_round_trip` newly fails. Both assert BEFORE any close step, so neither is
   reachable by §6.2's change; the common factor is the worker-session/pool path the peer's
   MICROKERNEL-POOLED-ACTOR runtime is rewriting. The other 13 (manifest/panel/terminology/binary)
   have never printed their messages — the abort truncates the binary. Next step is exactly the run
   queued as `pass4-2.txt`: skip that one test and read them.
2. `writer-writer` — after §6.5 the residue to re-measure is
   `writer_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed` (the live decode
   faults `writer-envelope.snapshot-pack-must-be-scalar`, `💾️binary/🦀️.rs:254`, on an envelope built
   from `empty_writer_snapshot()`; animate's byte-identical guard passes, so the difference is what
   writer's `capture_read` now puts in `initialSnapshot` since the `text`/`document` child reshape),
   `writer_artifact_store_preparation_is_exact_bounded_and_reversible` (2 vs 1),
   `scene_emits_placeholders_selectable_spans_and_newline_gates_for_jack`, and the
   mutation/operation-law reds.
3. `semio-s-plugin-architect::descriptor_is_fresh` — the coordinator regenerated the architect
   descriptor at 15:47, so this should pass on the next run; it needs a wasm `describe`, never a
   native fix.
4. A visual pass on the architect and writer panes once the host's `plugin-handle.closed` boot fault
   clears.

### 6.8 Files changed this session (absolute)

Production:
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs`

Laws / tests:
- `…/🏛️architect/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🧪️tests/🔬️unit/🦀️.rs`
- `…/🏛️architect/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `…/🎞️animate/…/🧬️schema/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs`
- `…/🎞️animate/…/🚪️io/📸️snapshot/📝️text/🧪️tests/🔬️unit/🦀️.rs`
- `…/✒️writer/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `…/✒️writer/…/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`
- `…/✒️writer/…/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs`
- `…/✒️writer/…/✏️editor/🎮️commands/📝️text-edit/🧪️tests/🔬️unit/🦀️.rs`
- `…/✒️writer/…/✏️editor/🎮️commands/🔍️lint-document/🧪️tests/🔬️unit/🦀️.rs`

Fixtures (17 files, `src` + `sourceAspect` only):
`…/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/**/📸️snapshot/**/🔣️.json`
and `…/✏️editor/📌️panels/🔍️inspection/🧫️fixtures/🔣️panels.json`.

> 🗂️ **Scratch relocation (16:47).** The repo workspace cleanup deletes `$T/🗑️generated`, so this
> topic's STATUS.md, run logs, probe scripts and `CARGO_TARGET_DIR` now live under
> `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/play-fleet/knowledge-children` (coordinator's
> brief-v5 16:40 addendum). This tracked file stays the durable record.

### 6.9 Live verdicts CONFIRMED on the 15:47 activation (17:30)

The coordinator's strict acceptance suite is **70/70** on this activation, with `animate`, `architect`
and `writer` all passing — including the SPA-fallback asset guard that `📓️play-runtime.md` had
recorded as a real red against `animate`. The `plugin-handle.closed` boot failures seen at 16:11–16:24
were a transient host fault under load (machine load ~250); they do not reproduce now.

Re-probed `#architect` alone at 17:30 (`probe-kc.mjs`, evidence under
`⚡️cache/play-fleet/knowledge-children/panes7/`):
`outcome=settled`, `data-shell-ready=architect`, **1070 nodes, 0 console errors, 0 page errors, 0
refusals, 0 404/SPA-fallback asset responses**. The pane shows the demo document — Adjacency window
(`Reception`, `Waiting`, `PAIRS`, `Reception ↔ Waiting [Required]`), Register window with both
elements, Report window, example picker on **Demo**.

**The Graph window is fixed, visibly.** `window:architect-graph` measures `[485, 61, 469, 907]` (the
same 469 px pane that used to crop it) and the screenshot now shows BOTH element nodes drawn inside
it, side by side and joined by their adjacency edge, with both labels legible. Under the previous
fixed-radius layout the second node's left edge was at scene x = 540 — off the right side of this
window with the default `{0, 0, 1}` viewport — which is exactly the "Waiting !… !…" crop
`📓️audit-visual-2.md` reported and §5.8 handed over.

### 6.10 pass4-2 (18:03) — four of six crates green, and the REAL reason the other two "abort"

| crate | pass4-2 |
|---|---|
| `semio-s-artifact-architect-program` | **2087 passed / 0 failed — GREEN** |
| `semio-s-plugin-architect` | **3 / 0 — GREEN** (`descriptor_is_fresh` passes on the coordinator's 15:47 descriptors) |
| `semio-s-plugin-animate` | **3 / 0 — GREEN** |
| `semio-s-plugin-writer` | **6 / 0 — GREEN** |
| `semio-s-artifact-animate-presentation` | 16 red printed, then SIGABRT |
| `semio-s-artifact-writer-writer` | SIGABRT on its first failing test |

The two remaining SIGABRTs are **one defect, present in both crates, and it is not a close-ladder
bug at all** — it is what makes every OTHER red in these two binaries invisible:

> every owner in these two `🚪️io/🧬️mutations/💾️binary/🦀️.rs` files asserts its bounded-close witness in
> `Drop` **unconditionally**. A witness exists to catch a leak on a HEALTHY path. When a test fails its
> own assertion, the thread starts unwinding with a live owner still on the stack; the witness then
> fires *during* that unwind, and a second panic in a destructor is a NON-unwinding abort. One red test
> therefore kills the whole binary, the first real failure is never printed, and the other ~300 tests
> in that binary are lost with it.

pass4-2 shows this exactly: animate aborts out of
`retained_presentation_envelope_publication_retries_backpressure_exactly_once` whose own
`assert_eq!` failed first (`🔬️unit/🦀️.rs:255`), then
`PresentationEnvelopeMaterializeRegistry::drop` (`💾️binary/🦀️.rs:1560`); writer aborts out of
`commit_rename_renames_all_spans_at_the_config_selection` whose own `assert_eq!` failed first
(`📝️text-edit/🧪️tests/🔬️unit/🦀️.rs:85`), then `WriterSnapshotRootRetirement::drop`
(`💾️binary/🦀️.rs:155`).

**Fix (production, both crates):** every `Drop` witness in those two files now reads
`assert!(std::thread::panicking() || (<witness>), "…")` — the exact shape the framework's own
`store::ArtifactEnvelope::drop` already uses (`🏪️store/🦀️.rs:2870`). 7 witnesses in animate, 9 in
writer. A leak on a healthy path still aborts loudly; a leak observed while a test is already failing
no longer replaces that test's message with a process abort.

**Law (one per crate, `a_live_owner_dropped_during_a_panic_unwinds_instead_of_aborting`):** hold a
genuinely live owner (a submitted `PresentationEnvelopeMaterializeRegistry` caller / a freshly begun
`WriterEditHistoryAuthority`), assert it is NOT terminal-empty, then panic inside `catch_unwind` while
it is dropped and require the panic to arrive as an `Err`. Without the guard this law does not fail —
it aborts the process, which is precisely the behaviour it forbids.

This also unblocks the two crates' remaining reds, which have never printed their messages: with the
binary surviving its first failure, `--no-fail-fast` finally reports all of them in one run.

### 6.11 pass5-1 (21:47) — the aborts are GONE, and every red is finally visible

`🗑️…/pass5-1.txt` (now `⚡️cache/play-fleet/knowledge-children/pass5-1.txt`), no `--skip` at all:

| crate | pass5-1 |
|---|---|
| `semio-s-artifact-architect-program` | **2087 / 0 — GREEN** |
| `semio-s-plugin-architect` | **3 / 0 — GREEN** |
| `semio-s-plugin-animate` | **3 / 0 — GREEN** |
| `semio-s-plugin-writer` | **6 / 0 — GREEN** |
| `semio-s-artifact-animate-presentation` | 305 passed / **22 failed**, NO abort |
| `semio-s-artifact-writer-writer` | 156 passed / **19 failed**, NO abort |

Both `a_live_owner_dropped_during_a_panic_unwinds_instead_of_aborting` laws pass. Neither binary
aborts any more, so `--no-fail-fast` reports every red in one run for the first time in this ticket.
`zzz_write_demo_example_asset` also did its job: `🖼️assets/🎬️demo/🗣️.dsl.semio` on disk now carries the
`habitat-67` source and the recomputed `presentation-39a58fcf08cf1060` child id, so
`the_committed_demo_asset_is_the_printers_own_demo_snapshot` goes green on the next build.

#### animate — 22 reds, grouped by cause
1. **Retained envelope publication never reaches Ready** (5):
   `envelope_helpers_round_trip`, `retained_presentation_envelope_materializes_populated_history_in_order`,
   `retained_presentation_envelope_publication_retries_backpressure_exactly_once` (`left: Pending, right: Ready`),
   `retained_presentation_envelope_caller_faults_…` (`registry.fault(..).is_some()` false) and
   `presentation_deck_materializes`. The first four are one symptom: the worker job never publishes.
   Note `presentation_deck_materializes` and three writer reds share the message
   `edit history insertion requires its exact mutation retirement factory` — fleet-brief bucket 2,
   a bare `ArtifactStore::new` in the fixture.
2. **Panels render nothing** (6): `document_lists_seeded_tiles` (no `tile-r0-c0`),
   `catalogue_lists_templates`, `details_panel_reports_schema_and_tile_count`,
   `presentation_semantic_panels_match_the_json_oracle` (`left: Null, right: "animate.presentation"`),
   and the two terminology laws (`"Tile templates"` / `"Kachelvorlagen"`). One suspect: the mounted
   fixture app boots an EMPTY deck, so every panel body renders its placeholder.
3. **Manifest / interactive-job classification** (4): `missing declared operation addTile`,
   `missing declared operation seedGrid`, `export_video_from_deck` refused with
   `interactive-job.not-ui-safe … BatchOnlyPendingRewrite`, and `delete_selection` (0 vs 1).
4. **Media import** (2): `media port 'frames:in' is registered but has no concrete resumable importer`
   (`interactive-job.missing-reserved-builder`).
5. **Fixture canonical form** (2, fleet bucket 1): `Number(1.0)` vs `Number(1)` in
   `🖼️replace-source/…/📸️snapshot` and `✂️resize-tile-crop/…/🦠️mutation`. **Fixed in this session**:
   five committed animate fixtures now write their integral `x/y/width/height/gap/sourceAspect`
   floats as `0.0`/`1.0`, which is what the crate's own encoder emits.
6. `presentation_document_contract_round_trips_children_and_rejects_foreign_owners`:
   `ValueError("missing field source")` — a document-contract fixture predating `PresentationSnapshot::source`.
7. `the_committed_demo_asset_…` — regenerated, see above.

#### writer — 19 reds, grouped by cause
1. **`edit history insertion requires its exact mutation retirement factory`** (5) — fleet bucket 2.
2. **`result.mutations` on a MOUNTED app** (3: `commit_rename_…`, `format_artifact_…`,
   `set_text_action_updates_projection`, all `left: 0, right: 1`) — fleet bucket 3: a mounted app's
   `result.mutations` is always empty; assert receipt lanes / history length instead.
3. **`EditText` inverse/apply do not round-trip** (2): `rename_writer_and_edit_text_invert_to_the_prior_field_value`
   yields `EditText { text: "" }` where the base carries `"old text"`, and
   `edit_text_obeys_the_inverse_and_diff_absorb_laws` restores a snapshot whose composed child id
   (`document-74621fa791e15905`) differs from the base's (`document-c930b9687a2eaa95`) for the same
   empty text. Both are consequences of `WriterSnapshot::text` becoming the persisted body while the
   fixtures/handles still derive from the child handle — the one genuinely PRODUCTION-shaped item left.
4. `writer_edit_history_decoder_uses_begin_mutation_and_faults_malformed_input` — the VALID decode is
   refused with `writer-envelope.mutation-array-entry` at offset 78. Pre-existing and newly visible:
   the abort this session removed was masking this first `.expect`.
5. One each: `writer-envelope.snapshot-pack-must-be-scalar` on the live envelope,
   `RetainedJobPayload requires one-page close…` on the store initializer, a missing
   `language-neutral oracle catalog` file (`🔮️oracles/🔣️.json`, fleet bucket 9),
   `Arc::ptr_eq` on the child-local-text fixture, the read-only viewer scene stamp,
   `scene_emits_placeholders_…`, `writer_artifact_store_preparation_…` (2 vs 1), and
   `writer_window_state_retained_publications_…` ("Writer window operations did not finish" — the
   SAME hand-rolled-pump defect as `drain_typed_operations`, in the window-config leaf's own helper).

### 6.12 Close-out (22:10)

Two further verification runs (`pass5-2.txt` 22:06, `pass5-3.txt` 22:10) could not measure anything:
`semio-framework-plugin` does not compile while the peer's interaction-selection work is mid-landing —
`error[E0583]: file not found for module 'interaction_selection_laws'` (`🔌️plugin/🦀️.rs:22418`, the
module file does not exist yet) plus `cannot find function 'history_row_applied_v1'` (that one the peer
landed at 22:03). Nothing of this topic's is implicated; the two edits made after `pass5-1` and still
unmeasured are the five float-canonical animate fixtures (§6.11 group 5) and nothing else.

**State at hand-over**
- GREEN through a real run log, no aborts: `semio-s-artifact-architect-program` 2087/0,
  `semio-s-plugin-architect` 3/0, `semio-s-plugin-animate` 3/0, `semio-s-plugin-writer` 6/0.
- `semio-s-artifact-animate-presentation` 305/22 and `semio-s-artifact-writer-writer` 156/19 — both
  now report FULLY (the abort that used to hide them is fixed, §6.10), every red triaged in §6.11.
- Live: strict acceptance 70/70 on the 15:47 activation with all three of this topic's panes passing;
  the architect Graph fit re-probed and screenshotted (§6.9); the animate figure served for real (§6.4).
- Next agent: start from §6.11's two grouped lists. The first three animate groups and the first two
  writer groups are ~20 of the 41 reds and are all fleet-brief buckets 1/2/3, i.e. mechanical once the
  framework compiles again.

---

## 7. 2026-09-22 late — clearing the mechanical buckets (source landed, verification blocked)

Written while `semio-framework-plugin` does not compile: the peer's interaction-selection work
declares `mod interaction_selection_laws;` (no `#[path]`) against a directory it created as
`🧪️tests/🕹️interaction-selection-laws/`, and `history_row_applied_v1` is referenced from a scope that
cannot see it. `pass5-2`, `pass5-3` and `pass6-1` all died on those two errors before reaching any
crate of this topic. Everything below is landed source, queued behind that.

### 7.1 Bucket 2 — `edit history insertion requires its exact mutation retirement factory` (8 tests)

`ArtifactStore::new` installs NO owner catalog, and `reserve_edit_history_slot` then refuses every
`Apply`: a bare store can be read but never mutated, undone or closed. Both crates now carry the
owner-installing constructor + self-closing guard that `🕸️dag`'s `new_dag_store`/`OwnedDagStore`
established, next to their existing owner catalogs:

- `🎞️animate/…/🚪️io/🧬️mutations/💾️binary/🦀️.rs` — `new_presentation_store` → `OwnedPresentationStore`
  (`Deref`/`DerefMut`, `close()` walking `close_owned_step` to the terminal-empty witness, `Drop`
  running it unless panicking).
- `✒️writer/…/🚪️io/🧬️mutations/💾️binary/🦀️.rs` — `new_writer_store` → `OwnedWriterStore`, installing
  the crate's own `writer_document_store_owners()` (the same catalog the app installs through
  `ArtifactEditor::build_document_store_owners`), plus the `WriterDocumentEnvelope`/`WriterDocumentStore`
  aliases that were missing.

Fixtures repointed: animate `presentation_deck_materializes` and `document_text_round_trip_with_operation_applied`;
writer `seeded_store` (feeding `writer_document_vcs_replays_text_mutations` and
`writer_document_vcs_undoes_text_mutation`), `writer_document_text_round_trips_through_the_store` and
both `command_envelope_round_trip_holds_for_an_applied_operation` leaves.

### 7.2 Bucket 3 — `result.mutations` on a MOUNTED app (3 tests)

`format_artifact_reformats_jack_query`, `set_text_action_updates_projection` and
`commit_rename_renames_all_spans_at_the_config_selection` asserted `result.mutations.len() == 1`.
A mounted app answers first and publishes afterwards, so `InvocationResult::mutations` is ALWAYS
empty there — the count-of-1 tested the unmounted shape this harness stopped using. Each now asserts
the mounted contract (`result.mutations.is_empty()`, with the operations printed on failure) and
keeps its own projection assertion, which is the real proof the edit landed.

### 7.3 Bucket 1 — fixture float canonical form (2 tests)

Five committed animate fixtures wrote integral `x`/`y`/`width`/`height`/`gap`/`sourceAspect` as
integers where the crate's own encoder emits floats (`Number(1)` vs `Number(1.0)`), failing
`replace-source/no-ops-when-the-source-is-already-identical` and
`resize-tile-crop/rejects-a-zero-width-crop`'s `committed_json_is_canonical`. All five now carry the
encoder's form.

### 7.4 Bucket 9 — the oracle catalog moved (1 test)

`direct_owners_descriptors_surfaces_and_catalog_correspond` read
`🪆️subsets/✳️any/🔣️oracle.json`, which has not existed for some time — the language-neutral catalog
lives in `🪆️subsets/✳️any/🔮️oracles/🔣️.json`. The law failed on `No such file or directory` rather
than on any correspondence it was written to check; it now reads the real file.

### 7.5 `WriterSnapshot::text` consistency (2 tests, the production-shaped pair)

`rename_writer_and_edit_text_invert_to_the_prior_field_value` and
`edit_text_obeys_the_inverse_and_diff_absorb_laws` hand-built a base that set the composed
`document` handle from one body while leaving the PERSISTED `text` field empty. Since `text` became
the body `writer_text` reads, that base is internally impossible: `EditText::inverse` correctly
answers `EditText { text: "" }`, and the inverse law then re-mints the child handle from the restored
text and gets a different content address than the hand-built one (`document-74621fa791e15905` vs
`document-c930b9687a2eaa95`). Both bases — and the three outcome-law bases beside them — now go
through the crate's own `writer_snapshot_with_text`, the single constructor that keeps the field and
its content-addressed handle in step. The PRODUCTION inverse is correct as written; the fixtures were
the stale half, and the doc comments now say why.

### 7.6 Animate document contract (1 test)

`🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json`'s `document` predates
`PresentationSnapshot::source`/`tiles` and decoded with `ValueError("missing field 'source'")`. It now
carries a neutral `source` (the same `/fixture-deck.png`, 3:2, full frame the sibling mutation
fixtures use) and an empty `tiles`, in the encoder's float-canonical form.

### 7.7 Still open, and why they are NOT being guessed at

- **animate panel cluster** (`document_lists_seeded_tiles`, `catalogue_lists_templates`,
  `details_panel_reports_schema_and_tile_count`, `presentation_semantic_panels_match_the_json_oracle`,
  the two terminology laws). The semantic-contract failure is precise — `fields[0]["children"]` is
  EMPTY where a `value` child is expected, i.e. `tree_item_desc`'s value no longer materialises as a
  child node — but the same panel's Debug-rendered output also lacks the schema string when reached
  through `app.render(PRESENTATION_PLAY_BODY_DETAILS, …)`, which is a different symptom. Two shapes
  cannot both be inferred from a log; this needs one compile.
- **animate retained-envelope publication** (`envelope_helpers_round_trip` and the three
  `retained_presentation_envelope_*` publication laws): the worker never reaches `Ready`. Production,
  and the close-ladder work in §6.2 is already in place, so the next step is instrumenting the
  handle's `maintenance_step` state machine — again, needs a compile.
- **animate manifest/interactive-job** (4) and **media import** (2):
  `missing declared operation addTile`/`seedGrid`, `interactive-job.not-ui-safe …
  BatchOnlyPendingRewrite` for `exportVideoFromDeck`, and
  `media port 'frames:in' is registered but has no concrete resumable importer`.
- **writer singles** (7): `writer-envelope.snapshot-pack-must-be-scalar` on the live envelope,
  `RetainedJobPayload requires one-page close…`, `writer_edit_history_decoder_…`'s VALID decode
  refused with `writer-envelope.mutation-array-entry` at offset 78, `Arc::ptr_eq` on the child-local
  text fixture, the read-only viewer scene stamp, `scene_emits_placeholders_…`, and
  `writer_window_state_retained_publications_…` ("Writer window operations did not finish" — the same
  hand-rolled-pump defect as `drain_typed_operations`, in the window-config leaf's own helper).

`pass6-1.txt` is the queued verification of §7.1–§7.6 (16 of the 41 reds); re-run it the moment
`semio-framework-plugin` compiles.

### 7.8 pass6-2 (22:39) — §7.1–§7.6 VERIFIED

`⚡️cache/play-fleet/knowledge-children/pass6-2.txt`, the first run after the peer removed its broken
`mod interaction_selection_laws;`:

| crate | pass5-1 | **pass6-2** |
|---|---|---|
| `semio-s-artifact-architect-program` | 2087 / 0 | **2087 / 0 GREEN** |
| `semio-s-plugin-animate` | 3 / 0 | **3 / 0 GREEN** |
| `semio-s-plugin-writer` | 6 / 0 | **6 / 0 GREEN** |
| `semio-s-artifact-writer-writer` | 156 / **19** | 167 / **8** |
| `semio-s-artifact-animate-presentation` | 305 / **22** | 310 / **17** |
| `semio-s-plugin-architect` | 3 / 0 | 2 / 1 — `descriptor_is_fresh` |

Every item of §7.1–§7.5 is green. `descriptor_is_fresh` regressed for one honest reason: the
`zzz_write_demo_example_asset` regeneration (§6.11) changed `🖼️assets/🎬️demo/🗣️.dsl.semio`, which the
ANIMATE descriptor embeds, so `🗑️generated/describe.request/animate` is touched again — a wasm
`describe`, never a native fix.

### 7.9 Landed after pass6-2 (queued as `pass7-1`, blocked again on the peer)

1. **animate's test `render` helper read the tree off `format!("{:?}", ..)`** — every node KEY and
   every LABEL is a `UiText`, a fixed-capacity buffer whose `Debug` is not its string, so a substring
   search for `"animate.presentation.play.catalogue.templates"`, `"tile-r0-c0"`, `"Tile templates"` or
   the document schema could NEVER match, while a search for an enum variant name like `Canvas2d`
   still did. That single outlier explains four of the six "panels render nothing" reds
   (`document_lists_seeded_tiles`, `catalogue_lists_templates`,
   `details_panel_reports_schema_and_tile_count`, `animate_presentation_labels_resolve_native_by_default`).
   It now projects through `artifact_app_laws::project_and_retire_fixture_tree` like `🏛️architect`,
   `✒️writer` and `🖨️raster` — which also RETIRES the rendered tree the `Debug` route leaked.
2. **`animate_presentation_labels_translate_panels_in_german`** asked for German from a render given
   the DEFAULT `ViewModel`, which resolves native English. New `render_with_view` helper; the law now
   hands the app a German view model.
3. **`writer_window_state_retained_publications_…`** re-rolled the retained-operation pump in its own
   leaf (the same defect as `drain_typed_operations`, §6.5): it never drained the terminal-witness,
   composed or interaction-query outboxes that `has_pending_typed_operations` counts, and took one
   page per turn where a window command publishes a config AND a transient. Replaced with
   `settle_registered_typed_operation`, counting the two window lanes off its receipt.

`pass7-1` could not measure them: `semio-framework-plugin` stopped compiling again with 36 errors
after the peer added a `Send` bound to `impl PluginApp for VcsArtifactApp<A, M>` without carrying it
into their own `artifact_app_laws` helpers (`🔌️plugin/🦀️.rs:7415`, `:7638`, …). Same handling as
before: requeue the identical command when it compiles.
