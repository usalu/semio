# 📓️ S2-AGNOSTIC — artifact-agnostic history editing (G7 labels, G8 editability gate, G12 acceptance harness)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, WP S2-AGNOSTIC (design §16.2, §16.3; gaps G7, G8, G12 of `📓️resume-gap.md`).
Nothing was committed; no ticket or goal was touched. Scratch: `🗑️generated/s2-agnostic/`.

## Session 2 — 2026-10-01 (12:20–13:30, resumed 16:35)

Status at 21:50: **G7 done** (plugin lib compiles; strict label gate 0 findings repo-wide), **G8 done** (editability gate 0 findings
repo-wide; kernel laws 31/0; strict payload lint fixed and down to 11 genuine routed findings; inputs lint needs `schema generate`),
**G12 harness mounted and compiling** (`app::history_edit_acceptance`), wired into 26 plugin crates; per-plugin runs in progress (§7) —
the usage cuts at ~13:30 and ~18:00 and peer compile breaks (gltf pack, ui wgpu component) stalled the first runs.
The sections below are updated at every milestone.

### 1. G7 — generic localized labels (design §16.2)

Decision: the bound IS the default. `ArtifactApp::Mutation`, `ArtifactEditor::Mutation` and `ArtifactViewer::Mutation` are now
`protocol::SemanticMutation<Self::Snapshot> + …` (the end-state ratchet the `SemanticMutation` docstring announces), so every
history label is `SemanticMutation::label` (en + de) by construction. `ArtifactApp::mutation_label`, `ArtifactEditor::mutation_label`,
the `EditorApp` forward and the `None` default are **deleted**; there is no op-text fallback left to reach.

| Change | Path |
|---|---|
| bounds tightened, `mutation_label` deleted (3 traits + `EditorApp` forward) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (label-resolution regions only) |
| history row `leaf_label` = `SemanticMutation::label` of the effective op, else the edit's own first forward (undone edits too, which had no applied op and fell back to op text) | same file, `CommandView` builder |
| `backfilled_edit_label` takes the first op's label; document rows pass `SemanticMutation::label`, config rows a localized "Change settings / Einstellungen ändern" (config lanes are not semantic; previously `print_op`) | same file |
| `NoConfigMutation` gains an (uninhabited) `SemanticMutation<NoConfig>` impl | same file |
| `time_travel_mutation_label` = `SemanticMutation::label` (was `mutation_label().unwrap_or(print_op)`) | `🔌️plugin/⏪️time-travel/🦀️.rs` |
| 10 per-app overrides deleted | writer, shooting, fem 2d, fem 3d, layout, drawing, note, puzzle 2d `✏️editor/🦀️.rs`; **plus stdio html + md**, which a peer added at 12:52/12:55 after the trait method was gone (the new gate caught both) |
| 2 toy overrides deleted | `🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs`, `🔌️plugin/🧪️tests/🧪️history-label-reload/🦀️.rs` |
| fixtures made semantic | `🔌️plugin/🖥️host/🧫️fixtures/🧩️component/🧬️schema/🧬️mutations/🦀️.rs` (uninhabited impl); `🔌️plugin/🧪️tests/🧩️composition/🦀️.rs` (`SemanticMutation<ComposedParentSnapshot>` for `RecursiveFixtureMutation`) |

Blast-radius check: 322 app impls enumerated (`🗑️generated/s2-agnostic/app_mutation_types.py`, `app-mutation-types.json`); every
real editor/viewer names a `#[derive(Mutations)]` aggregate (or the puzzle bridges, which already implement `SemanticMutation` for the
play snapshots); only the 4 fixtures above needed an impl.

### 2. Gates (repo test domain, wired like `schema mutation-inputs`)

New subcommands of `bun ./📜️script.ts schema …` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts`
(regions `🦀️RustSources`, `🏷️MutationLabels`, `✏️MutationEditability`), both also folded into `test schema`:

- **`schema mutation-labels`** (code `schema-mutation-label`): every `fn label` of a `MutationKind`/`CompositeMutationKind`/
  `SemanticMutation` impl must be `LocalizedLabel::native(en, de)` (both non-empty) or a forward; findings `labelOverride` (an app
  `fn mutation_label`), `labelLocaleInvariant` (`LocalizedLabel::data`), `labelLocaleEmpty`, `labelOpText` (`print_op`, also any
  `LocalizedLabel::data(…print_op…)` anywhere — the history fallback), `labelUnresolved`. Proc-macro templates and `🧪️tests` sources skipped.
- **`schema mutation-editability`** (code `schema-mutation-editability`): enumerates every `#[derive(Mutations)]` aggregate (leaf verdict
  `editable` / `foreign` = descriptor `composition: composite` / inert phases of `#[mutation_leaf(payload = …)]`) and every hand-written
  `impl Mutation<S> for T` (reason `forwarding` (all five payload accessors), `lane`, `fixture`, `empty`, `unexposed` (no app names it), else
  finding `aggregateHandwritten`); generic aggregates (`aggregateGeneric`, no emitted law) and variants without a leaf descriptor
  (`leafUnresolved`). `--json` lists every leaf verdict and hand-written aggregate.
- Codes registered in `SCHEMA_DIAGNOSTIC_CODE_TABLE` (`🧪️test/🟦️.ts`) and the protocol enum (now at `🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json`,
  moved there by a peer with my two codes).
- nx targets `@semio-tech/repo-test-domain:test-schema-mutation-{labels,editability}[-census]` and `:test-mutation-history-gates`
  (`📋️project.json`, `📜️script.ts`), five `launch.json` rows (orders 900.05591–900.05595 after `test-schema-mutation-payloads-census`).
- Language-agnostic corpus + third-party oracle: `🧪️test/🧫️fixtures/🧫️mutation-labels/🔣️.json` (12 cases, every verdict) checked by the token
  gate AND an independent **tree-sitter-rust** AST oracle (`web-tree-sitter` 0.20.8 + `tree-sitter-wasms` 0.1.13, installed via the root
  devDependency `@nxlv/python`); editability fixture `📡️spr/🎮️command/🧫️fixtures/🧫️mutation-editability/🔣️.json` checked by the TS gate
  (static) and the Rust law (runtime). Test: `🧪️test/🧪️tests/🧪️mutation-history-gates/🟦️.ts`.

### 3. G8 — runtime editability + label law (Rust)

- `📡️spr/🎮️command/🦀️.rs` (re-exported from `📡️spr/🦀️.rs`): `mutation_input_schema_failures::<P, M>()` (one JSON-object payload schema per
  leaf descriptor), `mutation_label_failures::<P, M>(&ops)` (no empty label cell in any locale × terminology), and
  `mutation_payload_round_trip_failures` now also fails an op **without** an input schema that still rebuilds from its payload (an inert op
  must refuse — the "declared + tested" reason).
- `🗣️dsl/✨️derive/🦀️.rs`: the emitted `semio_payload_law_<aggregate>` runs all three, so every plugin crate's lib tests check every
  witnessed leaf for schema, label and payload round trip with no wiring.
- Kernel tests `📡️spr/🎮️command/🧪️tests/🧪️mutation-payload/🦀️.rs`: `every_witness_reaches_its_editability_verdict` (fixture parity),
  `every_leaf_publishes_one_payload_schema`, `the_label_law_reads_every_locale_of_every_operation`, `an_operation_without_a_schema_refuses_its_payload`
  (hand-written `Unpublished` negative aggregate).
- Puzzle 2d/3d/5d bridges (`impl Mutation<Value>` and `impl Mutation<Puzzle*PlaySnapshot>`, 6 impls) now also forward
  `from_payload_value` and `conflict_target` (the play-snapshot bridge — the LIVE app type — silently dropped every op's conflict target and
  could not build witnesses).

### 3b. G8 extension — every leaf input resolves through the runtime resolver (coordinator, 02:50; S2-CONTROLS' forms finding)

Gap: the runtime resolver (`registered_input_schema_document`, used by the history editor's input reader and its payload validator)
only sees a scope's four fixed facets and named exports, so forms' `replace-block`/`create-block`/`create-step` (`$ref` into
`s/forms/forms/definition.json`) answered `timeTravel.schema-unavailable`. Generic fix (no forms special case):

| Piece | Path |
|---|---|
| `MutationLeaf::PAYLOAD_SCHEMA_DOCUMENTS` and `Mutation::INPUT_SCHEMA_DOCUMENTS` (row-aligned with `INPUT_SCHEMAS`), defaulted | `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs` |
| `#[derive(MutationLeaf)]` resolves every absolute `$ref` of the payload schema transitively against the `$id`s of every `🧬️schema` JSON document of the leaf's plugin (`🔌️plugins/<p>`) or framework module (`🔨️modules/<m>`), cached per compiler process, and `include_str!`s them; `#[derive(Mutations)]` collects them per leaf | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs` (`mutation_leaf_referenced_documents`, `mutation_schema_search_root`, `mutation_schema_document_index`, `mutation_schema_document_references`) |
| `register_referenced_schema_documents` / `registered_referenced_schema_documents` (idempotent catalogue) + re-export | `🧰️framework/🔨️modules/🧬️schema/📇️registry/🦀️.rs`, `🧬️schema/⚛️component/🦀️.rs` |
| the resolver also searches the published documents | `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` `registered_input_schema_document` |
| every app instance publishes its aggregate's documents at construction; a composed member's when its history-edit owners open | `🔌️plugin/🦀️.rs` `with_registry_on_bus`, `🔌️plugin/⏪️time-travel/🦀️.rs` member visitor |
| puzzle bridges forward `INPUT_SCHEMA_DOCUMENTS` | puzzle 2d/3d/5d `🧬️schema/🧬️mutations/🦀️.rs` |

Gates: (1) runtime — `history_edit_acceptance::input_schema_resolution_failures::<A>()` / `assert_input_schemas_resolve` (every leaf: the
input reader accepts it, every referenced `$id` resolves, the payload validator compiles, on a registered instance exactly as in
production); the macro now emits a second test `history_edit_inputs_resolve` in every wired crate. (2) static — `schema mutation-editability`
gains the finding `leafReferenceUnpublished` (TypeScript twin of the derive's search: plugin/module tree, plus framework/hub scopes).
Repo-wide: **6 findings**, all stdio xml base leaves (`set-declaration`, `set-doctype`, `insert-element`, `remove-element`, `set-attribute`,
`set-text`) whose payload schemas use a relative `$ref: "snapshot.json#…"` (§9). Gate tests 28/28 (`bun test ./…/🧪️mutation-history-gates/🟦️.ts`),
including the forms leaves resolving through their plugin tree and failing without it.
Compile: `semio-framework-schema-registry` ✓ and `semio-framework-os-kernel-dsl-derive` ✓ (`cargo check`, 02:58/03:02); `semio-framework-schema`,
`semio-framework` and everything above wait on a peer's `RecordSpecProducer` refactor of os-kernel (25 errors at 03:05, none in my files).
Order lesson (coordinator, rule 26): two callers landed ~1 min before their callees (registry re-export, derive helper) and broke
activation #8; every later edit wrote callees first.

### 4. G12 — cross-plugin acceptance harness

`🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs` (to be mounted as `app::history_edit_acceptance` under `cfg(any(test, feature =
"artifact-app-testing"))`): `assert_history_edits_end_to_end::<A, M>(plugin, manifest, fixtures)` reads the app's committed quintets
(`🦠️mutation` + `📸️snapshot/⬅️before` + `🎯️outcome: applied`, or `{mutation, before, after}` records), keeps editable ops, loads the
`before` document through `load_document_text`, seeds the leaf (plus one same-base downstream edit when one applies cleanly), and drives the
real verbs: begin → input (a schema-valid change derived from the editor's own `ActionArgDef`s: numbers by step/bounds, booleans, options,
vector axis, free text) → accept → Report replay to a `ready` review → finalize → commit overwrite; a second instance commits a new
alternative (active + scoped supersession). Both heads must equal a fresh fold of the edited log (decoded from the store's superseding
input); the edited row's label must be non-empty in en and de and never the op text. Data that cannot exercise the flow skips to the next
case; a broken mechanism fails at once; the panic names plugin, leaf and case. Macro `history_edit_acceptance_law!(plugin, Editor, manifest,
"../..")` wires one test per crate. Status: **WRITTEN BUT UNVERIFIED** (see §6).

### 5. Results so far

| Command | Result |
|---|---|
| `cargo check -p semio-framework-plugin --lib --tests` (12:44–12:50) | **lib compiled** (G7 bounds + call sites); lib-test target blocked by a peer: `🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/**` deleted in the worktree (old `1standard` dir present) |
| `cargo test -p semio-framework-os-kernel --lib -- mutation_payload_tests mutation_laws_fixture semio_payload_law` (13:03) | did not compile: peer half-rename of `🧫️mutation-laws/🧬️mutations/🌐️add-counter-then-notify-foreign` (refs renamed, dir not) and peer `🏪️store/🧪️tests/🧪️tool-transaction` errors; **no error in my files** |
| `bun test …/🧪️mutation-history-gates/🟦️.ts` | **27 pass, 0 fail** (12 corpus cases × token gate + tree-sitter oracle, coverage law, editability parity, hand-written reasons) |
| `schema mutation-labels --census` (repo-wide, 2:30 min) | 3049 native + 29 forwarding of 3082 leaf labels; 7 findings before fixes → html/md overrides deleted, config fallback replaced, fixtures/uninhabited/derive templates handled |
| `schema mutation-labels` strict, repo-wide (16:51) | **exit 0 — 3054 native and 29 forwarding of 3083 leaf labels; 0 findings** |
| `schema mutation-editability --json` strict, repo-wide (16:53) | **exit 0 — 0 findings**: 3081 leaves, **3077 editable**, 4 `foreign` (kernel counter fixture only), 140 inert phases; hand-written 107 = 81 lane, 11 fixture, 6 forwarding (puzzle bridges), 5 empty, 4 unexposed |
| `cargo check -p semio-framework-plugin --lib --features artifact-app-testing` (16:47–16:53) | **Finished, exit 0** (harness compiles; its one unnecessary-qualification warning fixed afterwards) |
| `CARGO_INCREMENTAL=0 … cargo test -p semio-framework-os-kernel --lib -- mutation_payload_tests mutation_laws_fixture semio_payload_law` (17:31) | **31 passed, 0 failed** — the 4 new laws + 10 emitted `semio_payload_law_*` (now with schema + label + inert checks) |
| `schema mutation-inputs` strict, repo-wide (16:55) | exit 1, 108 findings: 104 catalogue staleness after the path-budget renames (46 catalogued paths no longer exist, 58 new leaves uncatalogued — needs the coordinator's `schema generate`), 4 genuine (§9) |
| `schema mutation-payloads` strict, repo-wide | 185 findings → **lint fix** (below) → **11 genuine findings** (§9); 3081/3082 leaves witnessed, 5108 clean payloads |
| `bun test ./…/🧪️mutation-payload-parity/🟦️.ts` after the lint fix | 42 pass, 0 fail |

**Payload-lint fix** (`mutationPayloadParityReport`, same orchestration file): the REPO-PATH-BUDGET renames left several leaf directories
of one artifact sharing an emoji-stripped stem (`☀️change-scene-sun`, `💡️change-scene-sun`, …) and fixture directories shortened
differently from their leaf (`🅱️change-infiltration` vs `🅱️change-infiltration-temperature-term-coefficient`); the lint mapped fixtures to
leaves by the emoji-stripped stem, so 174 fixtures landed on the wrong leaf. It now tries the exact segments (emoji included), then a unique
prefix-shortened leaf, then the stem.

### 6. Next (in order)

1. Mount the harness (one line in `🔌️plugin/🦀️.rs` beside the time-travel test mounts), compile `semio-framework-plugin`.
2. Re-run both repo gates; re-run kernel tests once the peer renames settle.
3. Wire `history_edit_acceptance_law!` into each artifact crate, run them one at a time (swap 96 %), fill §7.
4. Run strict `schema mutation-inputs` and `schema mutation-payloads` repo-wide.

### 7. Per-plugin acceptance results

Wiring: one `semio_framework_plugin::history_edit_acceptance_law!("<plugin>", <Editor>, <manifest fn>, "../..")` line after the crate's
`*_manifest_for_tests` fn (ticket input `🧪️s2-agnostic-wire-acceptance.py`, idempotent): puzzle 2d, gis terrain, layout, note, shooting,
raster, lowpoly, norm en1990, block 2d, procedural generation2d, vcs, remodel, writer, mathematical equation, animate presentation,
architect, process3d, reasoning wires, forms, cad, playbook, imperative, energy, trinity rewriting, dag, space index, fem 2d, fem 3d,
sequence, puzzle 3d, puzzle 5d, sourcing curation, demonstrator playground (33 crates) plus 15 stdio editors (48 test targets). The
macro emits a plain `#[test]` driven by the harness's own `block_on_acceptance`, so a crate needs no async test macro.
Command per crate (plugin crates are in the `✏️s/Cargo.toml` workspace):
`CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s2-agnostic cargo test --manifest-path ✏️s/Cargo.toml -p <crate> --lib -- history_edits_end_to_end --nocapture`.

| Plugin crate | Result |
|---|---|
| `semio-s-artifact-vcs-vcs` (22:02–22:16, `-j 2`) | **PASS** (1 passed, 0 failed): leaf `rename-vcs` (case `✏️rename-vcs/🧪️retitles-the-document`), drafted `/newTitle`, one downstream edit replayed, overwrite and new-alternative heads equal the fresh fold, row "Rename vcs to "…" / VCS in "…" umbenennen" |

| `semio-s-artifact-puzzle-2d` (`--features component-app-assembly`, 22:35–22:42) | **PASS** (1 passed, 0 failed): leaf `change-node-anchor` (case `⚓change-node-anchor/⚓️derives`), drafted `/newAnchor = "fixed"`, one downstream edit replayed, both heads equal the fresh fold, row "Change node "…" anchor / Anker von Knoten "…" ändern" |

| `semio-s-artifact-norm-en1990` (22:45–23:28) | **PASS**: `change-reference-period-years`, `/newReferencePeriodYears = 2.0`, downstream replayed, row "Change reference period / Bezugszeitraum ändern" |
| `semio-s-artifact-block-2d` (23:28–23:30) | **PASS**: `rename-node-kind`, `/newName`, downstream replayed, row "Rename node kind to … / Knotenart in … umbenennen" |
| `semio-s-artifact-note-note` (23:30–23:37) | **PASS**: `change-grid-opacity`, `/newOpacity = 0.76`, downstream replayed, row "Change grid opacity to Some(0.76) / Rasterdeckkraft auf Some(0.76) ändern" (the leaf prints `Some(…)` in its label — routed, §9) |
| `semio-s-artifact-layout-layout` (23:40–23:51) | **PASS**: `change-page-width`, `/newWidth = 241`, downstream replayed, row "Change page "page-1" width / Breite von Seite "page-1" ändern" |
| `semio-s-artifact-shooting-shooting` (23:55–00:08) | **PASS**: `drag-assets`, `/dx = 4.1`, downstream replayed, row "Drag 3 assets / 3 Assets ziehen" |
| `semio-s-artifact-procedural-generation2d` (`--features component-app-assembly`, 00:50–01:16) | **PASS**: `create-generation`, `/generation/id` (nested object input), downstream replayed, row "Create generation "Taller" / Erzeugung "Taller" erstellen" |
| `semio-s-artifact-remodel-remodeling` (01:17–01:36) | **PASS**: `change-stream-sync`, `/newSyncOffsetMs = -15.5`, downstream replayed, row "Change stream … sync offset to -15.5ms / Synchronisationsversatz … ändern" |
| `semio-s-artifact-gis-gisterrain` (23:54) | BLOCKED (peer): `🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:103` argument mismatch at the time — rerun pending |
| `semio-s-artifact-raster-raster`, `semio-s-artifact-lowpoly-lowpoly` (00:36–00:50) | BLOCKED (peer, transient): plugin lib `match` on `&NoConfigMutation`/`NoPresenceMutation`/`NoTransientMutation` at 11590/11691/11781 mid-edit; the plugin compiled again by 00:50 (generation2d passed) — rerun pending |
| `semio-s-artifact-writer-writer` (01:36–01:45) | BLOCKED (peer): `🚪️io/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs:314` `MutationEnvelope` misses field `line` — rerun pending |
| `semio-s-artifact-mathematical-equation` (01:45–02:03) | BLOCKED (peer): `🧬️schema/🧬️mutations/🦀️.rs:131` `protocol::to_dsl_value` not found — rerun pending |
| `semio-s-artifact-animate-presentation` (02:26–02:39) | FAIL → **harness fixed, rerun pending**: "none of the 0 editable fixture case(s)": every animate fixture outcome is `rejected` or `no-op`, none `applied`. The harness now seeds every case whose outcome is no refusal (a no-op an edit can turn into a change). |

Score so far: **9 PASS** (vcs, puzzle 2d, norm en1990, block 2d, note, layout, shooting, procedural generation2d, remodel), 0 mechanism
failures, 6 blocked/pending reruns. Wired and not yet run: process3d, reasoning, forms, cad, playbook, imperative, energy, trinity rewriting,
dag, space index, architect, fem 2d, fem 3d, sequence, puzzle 3d, puzzle 5d, sourcing, demonstrator, and 15 stdio editors (html, bcf, csv,
tsv, pdf, docx, md, xml valid + base, pptx, txt, json i-json + base, semio mesh + brep; ticket input `🧪️s2-agnostic-wire-acceptance-stdio.py`,
manifest given as a non-capturing closure over `create_*_editor()`).

The crates run through ticket input `🧪️s2-agnostic-run-acceptance.sh` (one gated cargo at a time, `-j 2`), results in
`🗑️generated/s2-agnostic/acceptance-results.tsv`. Cargo is on coordinator HOLD (rule 26) since 02:45.

Two earlier attempts were SIGKILLed (exit 137 at 21:37 and 21:59, swap 17.5/18.4 GB); runs now use `-j 2`.

### 8. Non-editable leaves and aggregates, with reasons

- **Composite leaves** (`may_emit_foreign_steps` is unconditionally `true` for `#[derive(CompositeMutation)]`): only the kernel fixture's
  `add-counter-twice`, `add-counter-four-times`, `add-counter-then-notify-foreign`, `add-counter-sequence`. No plugin leaf is composite today.
- **Inert phases**: 140 `Restore` variants of `#[mutation_leaf(payload = Apply)]` leaves — `GltfMutation` 120, `SvgMutation` 9,
  `XmlMutation` 6, `JsonMutation` 5 (each leaf's `Apply` phase stays editable). `input_schema() == None`; the payload law now proves they
  refuse their payload, and the time-travel editor marks them not editable.
- **Lanes** (81 hand-written config/presence/transient/window/draft mutations): not document history; time travel never edits them.
  By owner: puzzle 12, procedural 7, wfc 6, block 6, fem 5, architect 4, cad 4, draw 4, lowpoly 3, remodel 3, space 3, sourcing 3, flow 3,
  vcs 2, sequence 2, process 2, forms 2, layout 2, raster 2, note 2, hub 2, gis 1, os 1.
- **Fixtures** (11 hand-written test aggregates): `PublicationMutation`, `CausalAddOp`, `DemoMutation`, `RetainedTextMutation`, `WitnessOp`,
  `Unpublished` (my negative), `ColdCounterMutation`, `RecursiveFixtureMutation` ×2, `Add`, `HashMutation`.
- **Uninhabited** (5): `NoConfigMutation`, `NoPresenceMutation`, `NoTransientMutation`, the host component fixture's `Mutation`, `ChildrenTestMutation`.
- **Unexposed** document aggregates (no app names them): `ProbeMutation` (`🌉️mcp/🏠️workspace`), `NativeSocketProbeMutation` (wgpu renderer),
  `CollectionMutation` and `SpaceMutation` (`🪐️space/🗿️artifacts`).

### 9. Routed findings

- **S2-TEXT (or whoever owns stdio html/md editors)**: added `fn mutation_label` overrides at 12:52/12:55 after the trait method was removed;
  deleted by me (compile-atomic). Do not re-add: labels come from the leaf. The gate `schema mutation-labels` fails on any override.
- **S2-TAX / path-budget**: `🔌️plugin/🧫️fixtures/🛰️declaration-channels/1️⃣standard-1/**` deleted in the worktree (blocks the plugin lib tests);
  `🌐️add-counter-then-notify[-foreign]` half-renamed in `📡️spr/🎮️command` (blocks the os-kernel lib tests).
- **S2-W1G**: `🏪️store/🧪️tests/🧪️tool-transaction/🦀️.rs` does not compile (`MemberStoreOwner<WitnessOp>`, `impl Future` compared).
- **Coordinator / design**: composites are never editable because `#[derive(CompositeMutation)]` answers `may_emit_foreign_steps() = true`
  unconditionally; a local-only composite (no `call_foreign`) could be made editable by a declared `#[composite(local)]` that the planner
  enforces. No plugin uses composites today, so this is latent.
- **Config lane labels**: config edit rows without a verb now read "Change settings / Einstellungen ändern"; a semantic config label needs
  `ConfigMutation: SemanticMutation` (out of scope).
- **Coordinator**: run `schema generate` — 104 of the 108 `schema mutation-inputs` findings are catalogue staleness after the path-budget renames.
- **norm-3 (en1995)**: `change-member-role` / `change-member-support` `x-semio-ui.options` label values the enum does not declare, and the
  `insert-member`/`change-member-*` fixtures use role/support values outside the leaf enums (8 payload findings).
- **media (wav)**: `set-snapshot/🔊️resamples` fixture `snapshot.chunkOrder[2]` matches no branch (2 payload findings).
- **raster / S2-STROKES**: `🖌️paint-stroke` has no committed wire witness.
- **os config**: `attach-local-folder` `/folder` widget `text` cannot edit its value; **stdio semio kit** `set-snapshot` `/snapshot/schema` has no label.
- **W3-GLTF / SQLite rollout**: new `🧊️gltf/…/📸️snapshot/📦️pack/🦀️.rs` (16:45) does not compile (`to_value` on `Option<…>`, 16 errors) — blocks
  every stdio-dependent plugin crate's tests (puzzle 2d included).
- **S2-W1E (ui wgpu component)**: `🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs` failed at 17:27 (`wgpu::layout`, `wgpu::stepper`), mid-edit.
- **W2-W-text (stdio xml base)**: six leaves (`set-declaration`, `set-doctype`, `insert-element`, `remove-element`, `set-attribute`,
  `set-text`) `$ref` the relative `snapshot.json#…`, which resolves against the leaf's own `$id` to a document nobody publishes — use the
  absolute `https://json.schemas.assets.semio-tech.com/s/stdio/xml/1.0/base/snapshot.json#…`.
- **S2-DRAW (note)**: `change-grid-opacity`'s label formats an `Option` with `{:?}` ("Change grid opacity to Some(0.76)" / "Rasterdeckkraft
  auf Some(0.76) ändern"), seen by the acceptance law's row check.
- **animate**: no committed fixture of `🎬️presentation` has an `applied` outcome (all `rejected`/`no-op`); add at least one applying quintet.
- **Plugin crates live in the `✏️s/Cargo.toml` workspace** since 11:16: run them with `--manifest-path ✏️s/Cargo.toml` from the repo root.

## Session 3 — 2026-10-02

Successor S3-AGNOSTIC (coordinator `⚪b7db773a…`, fleet rules 1–31). Scratch: `🗑️generated/s3-agnostic/`. Private test target
`…/⚡️cache/cargo/target-nde-s3-agnostic`. Status is updated at every milestone (newest first in S3.0).

### S3.0 Status log

- 12:40 coordinator item §20.4 (D4) done: G7 gate extended to hand-written emission labels (S3.5); gate tests 37/0; repo-wide gate
  red with 220 `labelHandwritten` findings (owners routed). `cargo check -p semio-framework-plugin --lib --features artifact-app-testing`
  **exit 0** (12:02, 138 warnings, none in the harness). Acceptance batch a1 (vcs, block 2d, dag, animate) running.
- 11:40 gates re-run (S3.1), routed findings sent to `main` (S3.4), harness extended (S3.2), wfc 2d / draw / flow wired (S3.3);
  `cargo check -p semio-framework-plugin --lib --features artifact-app-testing` queued behind the build gate (33 rustc busy).
- 11:05 repair check: every G7/G8/G12 piece of session 2 is on disk and consistent (harness mounted at `🔌️plugin/🦀️.rs:7279`
  under `cfg(any(test, feature = "artifact-app-testing"))`; derive `PAYLOAD_SCHEMA_DOCUMENTS`/`INPUT_SCHEMA_DOCUMENTS` + helpers;
  registry `register_referenced_schema_documents`; manifest resolver; runtime publication sites `🔌️plugin/🦀️.rs:24758`,
  `⏪️time-travel/🦀️.rs:859`). The 07:56 / 08:11 mtimes of `🔌️plugin/🦀️.rs` and `🗣️dsl/✨️derive/🦀️.rs` are peer edits outside
  my regions. 48 crates wired (§7 list). Not wired yet: wfc, flow, draw (whole plugins) and secondary artifacts.

### S3.1 Gates re-run (strict, repo-wide)

| Command (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`) | Result |
|---|---|
| `bun ./📜️script.ts schema mutation-labels` (11:01–11:02) | exit 1, **1 finding**: `labelOverride` — a peer re-added `fn mutation_label` to trinity rewriting's `ArtifactEditor` impl at 05:41 (`♻️rewriting/…/✏️editor/🦀️.rs:701`; the trait method no longer exists, so it was also an E0407). **Deleted** (labels come from the leaf). 3055 native + 29 forwarding of 3084 leaf labels |
| `bun ./📜️script.ts schema mutation-editability` (11:03–11:05) | exit 1, **6 findings** (all stdio xml base `leafReferenceUnpublished`); 3078/3082 leaves of 224 aggregates editable, 4 composite (kernel fixture), 140 inert phases, 107 hand-written aggregates |
| `bun ./📜️script.ts schema mutation-payloads` (11:05–11:11) | exit 1, **10 findings** (8 norm en1995, 2 stdio wav); 5131/5136 payloads + feature rows meet their leaf schema, 198 negative witnesses, **3083/3083 leaves witnessed** (raster `paint-stroke` now witnessed) |
| `bun ./📜️script.ts schema mutation-inputs` (11:11–11:12) | exit 1, **21 findings** (12 xml-family `prologPosition` labels, 2 en1995, 2 din18599, 2 vdi3805, 1 lowpoly, 1 os config, 1 trinity `leafUncatalogued`); 5511/5530 inputs of 3041 leaves carry a UI descriptor. The 104 catalogue-staleness findings of session 2 are gone (central `schema generate` ran) |
| `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-history-gates/🟦️.ts` (11:13) | **28 pass, 0 fail** |

Root cause of the 6 editability findings, corrected from session 2: the xml leaves reference `diff.json` absolutely; it is the DIFF document
(`📰️xml/…/🧱️base/🧬️schema/🔺️diff/🔣️.json:10,16,40`) that uses the relative `$ref: "snapshot.json#…"`. Relative references are not part of
the framework's schema contract (`🧬️schema/✅️validator/🦀️.rs:462` `resolve_reference` resolves only exact `$id`s; 10 relative refs exist
repo-wide, none other on a mutation leaf), so the fix is plugin-local (absolute `$id`) — routed.

### S3.2 Harness changes (`🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs`, my file)

- **Composed-child seeding gap (S2-GRAPHS, dag)** fixed generically: a fixture case's `📸️snapshot/⬅️before/🗣️.dsl.semio` is used verbatim
  (the only form a composed parent's owned children survive in; the JSON value form only holds the child handle), and when no committed
  case exercises the flow the law falls back to **derived cases** — every committed operation (whatever its own outcome; a refusal on its
  fixture document may apply elsewhere) on every document the app ships: `A::initial_snapshot()` and `ArtifactEditor::examples()` (a DSL
  body verbatim, a JSON body decoded). This also covers apps whose committed cases are all refusals/no-ops (animate) without a fixture.
- **Head comparison** is now value AND document text (`print_dsl`): a composed parent's value form equals across instances even when its
  owned child content differs; a mismatch names the first differing DSL line.
- Seeding faults are typed (`AcceptanceSeedFault::{Base, Operation(index)}`): a case whose own op does not seed skips its downstream
  variants at once, a document that does not load skips every case on it; a seed op that leaves no applied operation (a no-op on that
  document) is a skip, not a mechanism failure. Failures are returned and the cases retired before the panic (no drop of a fail-closed op
  during unwinding); the skip list in the panic is capped at 40 lines.
- Macro: `history_edit_acceptance_law!` passes `<$editor as ArtifactEditor>::examples()`; the call signature of the 48 wired crates is unchanged.

### S3.3 Wiring

- New: wfc ◻️2d (`✏️editor/🧪️tests/🔬️unit/🦀️.rs`, + dev-dependency `semio-framework-plugin` with `artifact-app-testing` in its
  `Cargo.toml` — the crate's own drag-tool test already used `artifact_app_laws` and could not compile alone), draw drawing (`mod context`),
  flow (`mod context`). Every one of the 34 plugins now has at least one wired editor (51 crates).
- Runner: ticket input `🧪️s3-agnostic-run-acceptance.sh <crate>…` (rule-30 gate, private target `target-nde-s3-agnostic`, `-j 4`,
  filters `history_edits_end_to_end history_edit_inputs_resolve`), results `🗑️generated/s3-agnostic/acceptance-results.tsv`.

### S3.4 Routed (SendMessage `main`, 11:35)

xml diff relative `$ref` (6 editability); xml-family `prologPosition` label (12 inputs, schema edited by a peer 10:27 today); norm en1995
options/enum + fixtures (2 inputs + 8 payloads); norm din18599 labels (2); norm vdi3805 option labels (2); wav `chunkOrder[2]` (2 payloads);
lowpoly `apply-paint-stroke` color widget on an RGBA int array (1); os config `attach-local-folder` text widget (1); trinity override
(deleted) + `schema generate` for `delete-working-nodes`; note `Some(0.76)` label.

### S3.5 G7 extension — hand-written emission labels (design §20.4, D4)

`schema mutation-labels` gains the finding class **`labelHandwritten`**: an `Emit::commit(mutations, label)` call (literal or computed
label, `Emit` qualified and/or turbofished) or an `Emit { … description: <anything but None> … }` struct literal (shorthand
`description` included; destructuring patterns `… } =` / `… } =>` excluded). Code: `rustEmitLabelSites` in region `🏷️MutationLabels` of
`🧪️test/🧬️schema/📋️orchestration/🟦️.ts` (also the census now counts only leaf-label traits). Corpus
`🧪️test/🧫️fixtures/🧫️mutation-labels/🔣️.json` +4 cases (literal commit, computed + qualified + turbofished commit, described literals incl.
shorthand, and a passing case: struct definition, `Default` impl, `-> Emit {`, `commit_transaction`, `mutations`, `description: None`,
destructuring); the tree-sitter-rust oracle reads `call_expression`/`struct_expression` (not patterns) independently. Planted-violation test
in `🧪️tests/🧪️mutation-history-gates/🟦️.ts`: a temp tree with a planted commit label + described literal yields exactly those two
diagnostics (test sources skipped), the leaf-labelled rewrite yields none.

| Command | Result |
|---|---|
| `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-history-gates/🟦️.ts` (12:3x) | **37 pass, 0 fail** (the repo-reading editability fixture test now has a 60 s timeout; it timed out at 5 s once under load ~110) |
| `bun ./📜️script.ts schema mutation-labels` (12:3x) | exit 1, **220 findings, all `labelHandwritten`** (leaf labels themselves: 0 findings) |

Remaining sites (routed to `main`): `Emit::commit` — lowpoly 1 (`🗑️object/🦀️.rs:46`), norm 2 (`📇️registry/🧬️contract/🖥️app-surface/🦀️.rs:1583,1591`);
draw's 10 are already gone. `Emit { description }` — stdio 178 (🧿️semio 34, 📖️pdf 22, 📐️step 14, 🏗️ifc 10, 📜️docx 8, 🖼️tiff 7, svg / xml / jpg /
pptx 6, png / wav 5, gif / mp4 / dwg 4, xlsx 3, 2 each las / zip / mp3 / bcf / csv / tsv / avi / stl / dxf / obj / gltf / ply / json / bmp /
📇️registry `✏️editing`, 1 each html / epw / md / txt), wfc 30 (◻️2d 7, 🖼️bitmap 7, 🧊️3d 8, 🧱️grid3d 5, 🔲️grid2d 3), energy 6, gis gismap 1,
puzzle 3d 1 (`✏️editor/🦀️.rs:5412`), runtime 1 (`🔌️plugin/🦀️.rs:30238` "Set Active Example"). Suggested end state (coordinator): delete
`Emit::commit` and `Emit.description` once the sites are gone.
