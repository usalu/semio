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
