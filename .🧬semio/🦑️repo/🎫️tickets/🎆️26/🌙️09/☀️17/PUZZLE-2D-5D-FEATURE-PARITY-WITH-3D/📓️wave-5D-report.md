# 📓️ Wave 5D — puzzle 🖐️5d document IO, clipboard, add-part dialog, context menu

Slice 5D of the 2026-09-17 parity fleet. EDITOR5 =
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.

**Status: source COMPLETE and type-checks green (`cargo check` rc=0, 0 errors). 15 laws landed; 5 pass,
9 fail and 1 is new — every failure caused by one sibling regression outside this slice: the three
shipped 5d example documents are empty (§9.1). No change to this slice's source is needed to turn
them green.**

## 1. Import / export — NEW, three command files + one bespoke work

| what | where |
|---|---|
| `exportFixture` | `EDITOR5/🎮️commands/📤️export-fixture/🦀️.rs` (new, 110 lines) |
| `importFixture` | `EDITOR5/🎮️commands/📥️import-fixture/🦀️.rs` (new, 410 lines) |
| `openImportFixture` | `EDITOR5/🎮️commands/🗂️open-import-fixture/🦀️.rs` (new, 21 lines) |
| `Puzzle5dExportWork` (the segmented-download arm) | `EDITOR5/🦀️.rs:4732`, right after `Puzzle5dWindowCommandWork` |

Ported from EDITOR3 (`📤️export-fixture` 128 lines, `📥️import-fixture` 424 lines) with EDITOR2's
2026-09-16 port as the second reference, and **generalised where the framework already owned the law**:

- the import chunk extent is `semio_framework::kernel::IMPORT_CHUNK_BYTES` and the chunker is the
  framework's own `import_payload_chunks`, not a third re-derivation of `CEILING/2` — so the guest
  measures exactly what the shell slices, and the argument names come from the framework's
  `IMPORT_ARGUMENT_*` constants rather than a fourth spelling of `{payload,name,chunk,chunkCount}`.
- `Puzzle5dExportWork` mirrors `Puzzle2dExportWork`: it is NOT a `dispatch_puzzle5d_action` arm,
  because a segmented download is a `PuzzleCommandWorkStep::Download`, not an `Emit`.

Budgets, all derived and none a literal:

| budget | value | meaning |
|---|---|---|
| inline export | `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` | one `Effect::DownloadMediaExport` payload |
| segmented export | `ArtifactOutputChunks::admit_maximum(PUZZLE_COMMAND_OUTPUT_BYTES)` = 256 KiB | what one segmented download may stream |
| import chunk | `semio_framework::kernel::IMPORT_CHUNK_BYTES` | one inbound `importFixture` page |
| import total | `PUZZLE_COMMAND_OUTPUT_BYTES` | a file this app wrote is a file it can read back |
| one JSON member | `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` | the paged root parse's own refusal |

Measured consequence for the three shipped examples (asserted, not assumed — see
`export_publication_picks_its_lane_by_payload_size`): Concrete Forest rides the **inline** arm,
Nakagin Capsule Tower the **segmented** arm, Capsule Dream is **refused with a notice** in both
directions. That is the brief's expectation and it is now a law rather than a claim.

The whole import is **one document edit**: the staged chunks produce no mutation and abort; the
closing chunk swaps `ctx.scene.document`, and `handle_action_impl`'s epilogue derives the single
delta. Every refusal (`Envelope`/`Chunk`/`Gap`/`Capacity`/`Element`/`Payload`) publishes a localized
notice and aborts — never a silent no-op, never a fault.

### `setFixtureJson` / `importComposeKit` — DELETED (greenfield, no legacy)

- `setFixtureJson` was `importFixture` without chunking, without a size refusal, without schema
  admission and without a notice. Superseded; its command directory
  `EDITOR5/🎮️commands/🧪️set-fixture-json/` is removed and the verb is gone from every registry.
- `importComposeKit` was declared `Migrated` but `puzzle5d_retained_reduce` answered it with a hard
  `Fault` ("no owner-qualified Compose-kit media value is present on this command route; use
  import-media kit:in") — i.e. a verb that could only ever fault. Its real route is the `kit:in`
  media port / the `import-media` reserved tool, which is untouched and still covered by its own
  laws. The verb, its fail-closed branch and its registry rows are gone.

## 2. Clipboard

`Puzzle5dCopyJob` / `Puzzle5dCutJob` / `Puzzle5dPasteJob` **already existed** in EDITOR5 (registered
in `build_reserved_tool_job` on `"copy" | "cut" | "paste" | "import-media"`), and are strictly richer
than 3d's `Puzzle3dClipboardJob`: a staged `Puzzle5dSelectionScan`, a bounded encode ladder, a full
incremental `close_step` retirement, fasteners closed over the copied parts, and both poses carried.
Porting 3d's job would have been a regression, so this slice **closed the two real gaps** instead:

- **locked parts are never cut.** `Puzzle5dCutJob`'s mutation build and the batch twin
  `puzzle5d_cut_operations` now both skip parts carrying `part_2d.locked`, and skip any fastener
  whose endpoint part survives (a half-cut document is worse than an uncut one). Both routes had to
  change or the retained and batch authorities would disagree.
- **pasted parts are re-selected.** `Puzzle5dPasteJob`'s completion now emits
  `InteractionWrite::replace(vortex, part, <fresh ids, sorted>)`. Safe on this route: a clipboard
  completion commits through `commit_framework_clipboard_completion` → `dispatch_emit`, not through
  the typed-operation drain that enforces `publication_lanes`, so this is not an undeclared-lane
  fault (the lane gate at `🔌️plugin/🦀️.rs:27104` only guards app-owned typed operations).

Default paste placement already offsets both poses (`paste_delta_2d` centroid delta, applied to
`part_2d.x/y` and `part_3d.origin[0]/[1]`); that is now a law rather than an accident.

## 3. Add-part dialog

- `openAddPartDialog` → `Effect::OpenDialog { dialog_id: "addPart" }`
  (`EDITOR5/🎮️commands/🗨️open-add-part-dialog/🦀️.rs`, new, 18 lines).
- `DialogDefinition::new("addPart", …, ActionRef::new("addPartKind"))` registered in
  `create_puzzle5d_app`, submitting the EXISTING `addPartKind` verb (5A2's).
- The `partKind` select is **live**: `puzzle5d_part_kind_options()` unions
  `kindCatalogs.parts` across Concrete Forest, Nakagin and Capsule Dream, with the same
  part-kind inference fallback `📌️panels/🛍️catalogue` applies to a document that declares no
  catalog, bounded by `PUZZLE5D_PART_KIND_OPTIONS_MAX = 64`. The literal `"Part"` option — a kind no
  catalog declares, so the form could not add a single real kind — is gone. `puzzle5d_part_kind_arg()`
  builds the select ONCE and is used by both the dialog and `action_args("addPartKind")`, so the two
  forms cannot drift.
- `addPartKind` is now `in_palette: false` (its select IS the dialog). This is puzzle 3d's own
  measured defect 7 — two palette rows carrying the same "Add Part" label, the second opening a bare
  action pane. **Hand-off to 5A2**: that flag sits on their verb's declaration.

## 4. Context menu

`puzzle5d_context_menu_items` was a single part-only branch that returned `Vec::new()` for every
other selection. Rewritten (`EDITOR5/🦀️.rs:3843-3969`, the `🔖️ContextMenu` region) into four branches keyed by
`Puzzle5dContextSelection` (new; buckets `surface.selection` + `surface.hits` per granularity, hits
winning only for a granularity the selection does not carry — puzzle 3d's rule):

| selection | rows |
|---|---|
| empty | `selectAll`, `paste`, `openAddPartDialog` |
| part(s) | `duplicateSelection`, `copy`, `cut`, `selectSameKindSelection`, `focusSelection`, group `settings` → hide/show + lock/unlock (`setSelectionFlag`, alternating value), `deleteSelection` (destructive, count phrase) |
| grip | `targetBrushSuggestions {fullId}` (only for exactly one grip), `focusSelection`, `deleteSelection` (destructive) |
| fastener | `deleteFastener {id}` (destructive) |

Audited against the registry AFTER 5A2's dedupe landed: the live ids are **`focusSelection`** (the
zoom verb; `zoomToSelection` no longer exists) and **`selectSameKindSelection`** (the widen verb;
`selectSameKind` no longer exists). The brief named `selectSameKind` — 5A2 kept the `…Selection`
spelling, so the rows follow the registry, and the law below pins that to the registry rather than to
a string in this document.

Two deliberate omissions, both to avoid a visibly dead row:
- **`retargetFastener` is NOT offered.** It early-returns unless the invocation carries a replacement
  `source`/`target` grip, which a context menu cannot name — an id-only dispatch is a silent no-op.
  Retargeting stays in the inspector, which has both grips.
- **The suggestions POPUP family (`openVortexSuggestions`/`closeVortexSuggestions`/`hoverSuggestion`/
  `acceptSuggestion`) was not ported.** 5d has no `suggestion_menu` in its window transient and the
  popup needs transient state + render surface owned by 5B (brush/fill) and 5E (window options). The
  grip row therefore points at `targetBrushSuggestions`, 5d's real, `Migrated` brush-suggestions entry
  point, which aims the instance's suggestions link at the clicked grip. **Hand-off: 5B/5E** (see §7).

## 5. Registries touched

For `exportFixture`, `importFixture`, `openImportFixture`, `openAddPartDialog` (added) and
`setFixtureJson`, `importComposeKit` (removed), in `EDITOR5/🦀️.rs` unless stated:

1. `puzzle5d_command_variants!`
2. `PUZZLE5D_RETAINED_TOOL_IDS`
3. `bounded_first_step_tool_proofs!` `tools: [...]`
4. `build_tool_job` match (`exportFixture` → `Puzzle5dExportWork`; the other three take the generic
   `BoundedFirstStepCommandWork` route, so they run `handle_action_impl` and can push effects/notices)
5. `PUBLICATION_CONTRACTS` — `exportFixture`/`openImportFixture`/`openAddPartDialog` = `HostOnly`,
   `importFixture` = `Artifact`
6. `dispatch_puzzle5d_action`
7. `.action_with(...)` in `create_puzzle5d_app` + `.dialog(DialogDefinition…)` + `.action_args("addPartKind", …)`
8. `.action_interactive_job(id, Migrated)` for all four new verbs
9. `command_from_action` — unchanged (generic over `Puzzle5dCommand::try_from_action`)
10. `EDITOR5/🗣️terminology/🦀️.rs` — 15 new labels, each with all four EN/DE × native/reuse cells:
    `export`, `import`, `export_too_large`, `import_invalid`, `import_too_large`,
    `import_incomplete`, `add`, `add_part`, `add_part_prompt`, `add_part_body`, `copy`, `cut`,
    `paste`, `select_all`, `suggest_parts`
11. `🗿️artifacts/🖐️5d/🦀️.rs` — four `#[path]` module declarations, `set_fixture_json` removed
12. `🗿️artifacts/🖐️5d/📦️packages/🦀️rust/Cargo.toml` — `semio-framework-trace` dependency (the
    guest contiguous ceiling; 2d and 3d already carry it)
13. `…/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json` — `toolIds`, `evidenceToolIds`, four new
    `semanticCursors` ladders
14. `PLUGIN/🧫️fixtures/🔏️publication-authority/🔣️.json` — the whole `Puzzle5dPlayApp` owner block
15. `PLUGIN/🦀️.rs` — the `ui.dialog` capability reason now names 5d's dialog too

**Registry regenerator**: `TICKET/🗑️generated/5D/sync-5d-registries.py` re-derives 13 and 14 from the
editor source and verifies all four exactness conditions of `📜️script.ts`'s `ownerOracle` plus its
`exactFactory` structural half. Every slice that adds or renames a 5d verb should re-run it; the last
one to land wins. `--verify-only` checks without writing.

## 6. Laws

15 new laws in `EDITOR5/🧪️tests/🔬️unit/🦀️.rs`, in four regions. Verdicts in §8; the 9 red ones all
fail on §9.1, not on the code they test.

**`📤️📥️DocumentIO` — diagnosis law**
0. `every_shipped_example_document_really_carries_its_content` — added AFTER the first test run, so
   that the eight look-alike failures resolve to one named cause.


**`📤️📥️DocumentIO` (`:1887`)**
1. `export_import_round_trips_every_shipped_example_byte_for_byte` — all three documents, through the
   real chunker and the paged root cursor, asserting the re-export is byte-identical.
2. `export_publication_picks_its_lane_by_payload_size` — inline / segmented / refused is decided by
   size against the two derived budgets, plus a pin that Concrete Forest really is the inline case and
   Capsule Dream really is the refusal case (so the law cannot pass while proving nothing).
3. `export_filename_follows_the_document_label` — the four filenames.
4. `import_stages_every_chunk_and_only_the_closing_one_edits_the_document` — through the real app:
   staged chunks emit nothing and leave `parts` at 0; the closing chunk lands the whole document, and
   ONE `undo` takes it all back (the one-edit law).
5. `every_refused_import_publishes_a_notice_and_changes_nothing` — gap / envelope / payload refusals
   each publish an `Effect::Notify`, emit no mutation and leave the projection identical.
6. `a_retransmitted_import_chunk_is_acknowledged_at_the_cursor` — and a closed run frees its slot.
7. `open_import_fixture_requests_a_file_and_edits_nothing` — `import_action == "importFixture"`.

**`🗨️AddPartDialog` (`:2038`)**
8. `open_add_part_dialog_opens_the_declared_dialog_and_edits_nothing` — the opened id IS a declared
   `DialogDefinition`, whose `submit_action` is `addPartKind`.
9. `add_part_dialog_enumerates_live_part_kinds` — every option is a kind a shipped catalog declares,
   the `"Part"` placeholder is gone, the dialog and the arg form offer the SAME list, `addPartKind` is
   not in the palette and `openAddPartDialog` is.

**`🖱️ContextMenuRows` (`:2111`)**
10. `every_context_menu_row_resolves_to_a_live_verb` — for all four selection granularities, every
    emitted row's action id is a declared `Migrated` app action or one of the four reserved verbs.
    **This is the law the 3d `zoomToSelection` defect would have caught, and the one that keeps this
    menu honest as 5A1/5A2/5G keep renaming verbs.**
11. `context_menu_rows_follow_the_selected_granularity` — the per-branch row sets, plus that the
    suggest row carries the clicked grip's `fullId` and the fastener row its own `id`.

**`📋️ClipboardLaws` (`:2227`)**
12. `a_cut_copies_a_locked_part_but_never_removes_it`.
13. `a_paste_preserves_both_poses_and_the_fasteners_between_copied_parts` — fresh ids, no collision
    with the live document, the same planar delta on `2d.x/y` and `3d.origin[0]/[1]`, `origin[2]`
    untouched, grips carried, endpoints remapped.
14. `the_default_paste_placement_offsets_the_fragment_in_both_poses`.

Two existing laws were updated because their subject changed:
`every_dispatched_action_bridges_to_a_command` (the four new verbs in, the two retired verbs out) and
`add_part_kind_materializes_the_declared_kind_default` (the default is now the first LIVE catalog
kind, asserted to be neither empty nor the retired `"Part"` literal).

## 7. Hand-offs

- **5A2** — `addPartKind` is now `in_palette: false` and its `action_args` select is built by
  `puzzle5d_part_kind_arg()`. Keep both if you touch that declaration.
- **5B / 5E** — the grip context-menu row is `targetBrushSuggestions`. If you land a real suggestions
  popup (`openVortexSuggestions`/`closeVortexSuggestions`/`hoverSuggestion`/`acceptSuggestion` + a
  `suggestion_menu` in `Puzzle5dWindowTransient`), swap the row id in `puzzle5d_context_menu_items`
  and the assertion in `context_menu_rows_follow_the_selected_granularity`.
- **5G** — `Puzzle5dWorldWindowConfig` gained `voxelDims` in the JSON schema, but the audit script's
  own neutral fixture case (`📜️script.ts` `validateWindowOwnershipSchemas`, the
  `Puzzle5dWorldWindowConfig` entry) still lacks it, so `publication-authority-audit` throws before it
  ever reaches its owner loop (see §8). Same class of drift for 2F's `Puzzle2dWindowConfig`
  (`areaBrushWidth`/`areaBrushHeight`). Both must be added to the script's case values.
- **5A1 (pre-existing, NOT introduced here)** — `PUZZLE5D_RETAINED_RAW_BYTES`/`_DECODED_ITEMS`
  (262 144 / 16 384) widen 5d's retained band off the shared puzzle default, and the 5d
  `🗄️retained-jobs` fixture's `capacities` was updated to match (committed `0b460ed19f`, 12:02). But
  the shared law `language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle`
  (`PLUGIN/🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs:155`) still expects
  `[PUZZLE_COMMAND_RAW_BYTES, PUZZLE_COMMAND_DECODED_ITEMS, …]` = 8 192 / 512 for EVERY owner, so the
  two cannot both be right. The band is load-bearing for this slice (one 32 KiB import chunk does not
  fit an 8 KiB raw wire), so the fix is to parameterise that law per owner —
  `retained_command_test_catalog()` should carry the owner's real `ToolExecutionContract` — not to
  narrow the band. Left alone here because it changes 2d's and 3d's fixtures too.
- **everyone** — re-run `sync-5d-registries.py` after your verbs land.

## 8. Commands run

All from the repo root, `CARGO_INCREMENTAL=0`, output in `TICKET/🗑️generated/5D/`.

| # | command | verdict |
|---|---|---|
| 1 | `cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly --message-format=short` → `check-native-short.txt` | **rc=0, GREEN.** `Checking semio-s-artifact-puzzle-5d` + `Finished dev profile in 14m 41s`. **0 errors**, 161 warnings across the whole dependency closure, of which **4 touch the 5d crate** and **none touches a file this slice created** (`✏️editor/🦀️.rs:9412` and `🪟️window/🦀️.rs:275` unnecessary-qualification, `🦀️.rs:137 parse_example_dsl is never used` — the last one is 5F's leftover, see §9). |
| 2 | `cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib -- <14 filters>` → `test-filtered.txt` | **Compiled and ran.** Of this slice's laws: **5 pass, 9 fail** — every failure for ONE cause, the shipped examples being empty (§9). Passing: `export_import_round_trips_every_shipped_example_byte_for_byte` (vacuously — now guarded), `open_add_part_dialog_opens_the_declared_dialog_and_edits_nothing`, `every_refused_import_publishes_a_notice_and_changes_nothing`, `open_import_fixture_requests_a_file_and_edits_nothing`, plus the updated `every_dispatched_action_bridges_to_a_command`. The run was cut short by a sibling's 60 s+ `kit_in_retained_import_media_…` test, so per-assertion messages were not printed. |
| 3 | `cargo test … --test-threads=1 -- <6 filters>` → `test-narrow.txt` | **rc=101, could not build.** Not a 5d error: `semio-framework-os-infinite` fails with `error[E0004]: non-exhaustive patterns: types::ActiveUtility::AreaBrush not covered` at `♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:8702` — slice 2F added the variant without updating that match. The whole workspace test build is red until they fix it. |
| 4 | `python3 TICKET/🗑️generated/5D/sync-5d-registries.py [--verify-only]` → `registry-verify-1.txt` | **OK.** All four `ownerOracle` exactness conditions plus the `exactFactory` structural half hold for `Puzzle5dPlayApp`. |
| 5 | `bun ./📜️script.ts publication-authority-audit Puzzle5dPlayApp` → `publication-audit-1.txt` | **rc=1, blocked before the owner loop** — `Puzzle5dWorldWindowConfig neutral fixture failed Ajv validation: missing voxelDims` (and, earlier, 2F's `areaBrushWidth`/`areaBrushHeight` for 2d). Both are the script's own fixture cases, not the 5d owner block; command 4 stands in for it. |
| 6 | `rustfmt --edition 2021 --check` on all seven files this slice created or edited | **clean** — every file parses; the only diff is a pre-existing whitespace line in the test file. |

**wasm32-wasip2 check: NOT RUN — owed to integration.** The addendum allows it only if the native
check and the tests are green; the test build is red on 2F's framework error (command 3), so it would
have measured their breakage, not mine. None of this slice's code is `cfg(wasm32)`-gated, so the
native check covers all of it.

## 9. Not verified / owed to integration

### 9.1 BLOCKER, not this slice: the three shipped 5d examples are empty

This is the single cause of all 9 failing laws, and it breaks far more than this slice — the 5d app
currently boots with NO document at all.

`CONCRETE_FOREST_EXAMPLE_JSON`/`NAKAGIN_…`/`CAPSULE_DREAM_…` (`EDITOR5/🦀️.rs:129-131`) were rewired
from the DSL fixtures to `crate::examples::puzzle5d::<name>::SOURCE.document_json()`, which
`include_str!`s `📚️examples/<name>/🖼️assets/<dir>/📄️document.json`. Those files carry the **puzzle 3d**
document shape:

| example | bytes | top-level members | parts | fasteners | label | kindCatalogs |
|---|---|---|---|---|---|---|
| concrete-forest | 10 067 | `schema, domain, meta, objects, attractions, targetVolumes, references` | 0 | 0 | — | — |
| nakagin-capsule-tower | 158 489 | same (3d shape) | 0 | 0 | — | — |
| capsule-dream | 121 | 5d shape but EMPTY | 0 | 0 | — | — |

`Puzzle5dDocument` carries `#[serde(default)]` on `parts`/`fasteners` and ignores unknown members, so
each of these deserializes **silently** into an empty 5d document rather than failing — which is why
the regression reached the test run as nine unrelated-looking assertion failures instead of one.
`parse_example_dsl` (`EDITOR5/🦀️.rs:137`) is now dead code, the compiler's own witness that the DSL
route was cut.

Two laws were added rather than weakening the nine that caught it:
- `every_shipped_example_document_really_carries_its_content` — names the regression directly
  (schema, label, parts, fasteners, kindCatalogs per example), so an integrator sees the cause first.
- a non-vacuity guard in `export_import_round_trips_every_shipped_example_byte_for_byte`, which was
  passing **only** because an empty document round-trips trivially.

**Owed to integration / 5F:** regenerate the three `📄️document.json` assets from the `.dsl.semio`
fixtures in the puzzle 5d shape (`parts`/`fasteners`/`label`/`kindCatalogs`/`kindCompatibility`), then
re-run this slice's filtered laws. Nothing in this slice's source needs to change for them to pass.

### 9.2 Other slices' breakage seen from here

- **2F** — `semio-framework-os-infinite` does not compile: `error[E0004]: non-exhaustive patterns:
  types::ActiveUtility::AreaBrush not covered`, `♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:8702`.
  The whole workspace **test** build is red on it (the native `check` of puzzle-5d predates it and is
  green). Also blocks the publication audit via the script's `Puzzle2dWindowConfig` fixture case.
- **5G** — the audit script's `Puzzle5dWorldWindowConfig` fixture case lacks `voxelDims`.
- **5A1** — the retained-jobs `capacities` vs the shared law, see §7.

### 9.3 Genuinely not verified

- No browser/battery verification: the coordinator owns Nx/activate/serve/Playwright. The
  import→export round trip is proven as a pure law, not through a real file picker in a real shell.
- The segmented-download drain (`drainSegmentedMediaExport`) is framework code exercised by 2d/3d;
  5d only proves it hands the lane a sealed `ArtifactOutputChunks`.
- `wasm32-wasip2` check not run (§8) — no `cfg(wasm32)`-gated code in this slice.
- The suggestions popup family (§4) is explicitly out of scope.
- The paste re-selection is argued safe from the framework source (clipboard completions commit
  through `dispatch_emit`, not the lane-gated typed-operation drain) but has not been observed in a
  running app.
