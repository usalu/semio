# Seam Map: plugin Rust tier 2 monoliths (puzzle 5d editor, puzzle 2d editor, bim model, architect registers)

Mode: read-only audit. No source file was edited. No git, cargo, nx or build was run. Analysis helpers (`items.py`, `coupling.py`, chunk specs) live only in the session scratchpad. All line numbers refer to the files as audited on 2026-10-15; re-verify anchors before each cut (section 6 of each file block).

Base path used below: `✏️s/🔌️plugins/` = `/Users/ueli/Documents/semio/✏️s/🔌️plugins/`.

## 0. Summary

| ID | File (under `✏️s/🔌️plugins/`) | Lines | Active lines | Crate (lib root) | Verdict |
|---|---|---|---|---|---|
| A | `🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | 9,141 | 7,729 | `semio-s-artifact-puzzle-5d` (`🖐️5d/🦀️.rs`) | Split. 8 children. Parent keeps about 1,050 active lines (including PlayApp, see A.4). `RetainedCommands` (3650-7730, 4,081 lines) must be cut in three places. |
| B | `🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | 4,059 | 3,171 | `semio-s-artifact-puzzle-2d` (`◻️2d/🦀️.rs`) | Clean up first: lines 106-308 are an empty region shell (189 blank, 14 marker lines, no code). Then split `retained` and `session`; `kit` and `interaction` are optional. |
| C | `🏙️bim/🗿️artifacts/🏢️model/🦀️.rs` | 7,817 | about 100 wiring/declaration lines | `semio-s-artifact-bim-model` (this file is the lib root) | Not a code split. The file is an inline mount manifest (about 2,700 `#[path]`, 1,589 `#[cfg]`, 652 `pub mod` in the mutation block). Re-home each subtree into its owning existing component. |
| D | `🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🦀️.rs` | 7,727 | 6,266 | `semio-s-artifact-architect-program` (`🏛️program/📦️packages/🦀️rust`, lib root `🏛️program/🦀️.rs`) | Split along register families. The families are mutually independent (only `kernel` is shared). Needs taxonomy registration first, because `🗄️registers` has no admitting children rule. |

Cross-cutting findings (details in each block):
1. Taxonomy registration is the gating item for D and for any new child under `🗄️registers/`. For A and B the names below are admitted by `members-of-editor` or the `retained` kind.
2. Two plugin-to-plugin dependency edges point the wrong way: 5d (and the 5d editor file) import 3d editor crates; the 2d package depends on 3d crates. Details in A.9 and B.7.
3. Source-text readers exist for A and B: their unit tests `include_str!("../../🦀️.rs")` and assert marker strings in the editor file. Moving code breaks those assertions unless they are re-pointed. Ownership fixtures also name the editor file (`owner_file`, publication-authority fixture).
4. `libWiringLineBudget` is 150 lines in the taxonomy. C's root cannot meet it unless its mounts move out.

---

## 1. Taxonomy facts that decide the names

Source: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`. Keys checked by reading the JSON; no validator run.

| Rule (key) | Content | Consequence |
|---|---|---|
| `surfaceChildDirs` | 🎭️modes, 🎮️commands, 📌️panels, 🎚️config, 👥️presence, 🫧️transient, 🗣️terminology, 🌉️wasm, 📚️examples. Required: 🎭️modes, 🎮️commands, 🎚️config, 👥️presence, 🫧️transient | Admits the existing 5d/2d/bim editor children. |
| `semanticDirectoryMemberKinds/members-of-editor` (owner kind `editor`, source registry, 21 names) | ⏳️precompute, 🌍️world, 🎨️chrome, 📺️renderer, 🖌️session, 🖼️canvas, 🗂️catalog, 🗺️maphost, 🛠️options, 🧠️precompute, 🧪️fixtures, 🧭️view, 🧵️session, 🧵️simulation-session, 🧵️reconstruction-session, 🎛️chrome, 🔮️inference, 🕹️interaction, 🧩️entities, 🧰️kit, 🧵️gestures | Source of the proposed editor child names for A and B (🧭️view, 🧩️entities, 🗂️catalog, 🕹️interaction, 🧵️session, 🧵️gestures, 🧰️kit). |
| `semanticDirectoryKinds/retained` | emoji 🧵️, slug `^retained$`, parentKindIds `["schema","editor"]` | Admits `✏️editor/🧵️retained/`. |
| `semanticDirectoryKinds/editor` | emoji ✏️, slug `^(editor\|s)$` | Kind of the editor directory. |
| `semanticDirectoryKinds/snapshot` (📸️), `engine` (⚙️), `schema` (🧬️), `tests` (🧪️) | No parentKindIds restriction | 📸️snapshot, 🧬️schema and 🧪️tests under editor are admitted by kind. `⚙️engine` is admitted by kind but contradicts the taxonomy comment `_standardsSubsetsComment` ("Subsets own schema, IO, and examples — never an engine"). Not proposed. |
| `semanticDirectoryMemberKinds/members-of-commands` (owner `commands`, 841 names) | Command folder names | 10 of the 55 existing 5d command folders are not listed: ↔️set-grid-spacing, ✋️set-lod-manual, 📉️set-lod-depth-variable, 📥️import-snapshot, 🗂️open-import-snapshot, 🗨️open-add-part-dialog, 🤏️set-grip-show, 🤖️set-lod-automatic, 🧭️set-grip-direction, 🧱️set-chunk-size. In 2d, the group folders 🎛️runtime and 🧊️document under `🎮️commands/` are not listed. Drift to verify. |
| `semanticCollections` key `🎮️commands` (kind command) with `semanticManifestFileKindId = json` | Taxonomy comment: manifest members are in exact bijection with direct child directories | Open question: 5d and 2d `🎮️commands/` roots have no `🔣️.json` manifest. Moving Work structs into `🎮️commands/<cmd>/` (option A.5b) may trigger this rule. Verify before choosing that option. |
| `schemaChildDirs` | 📸️snapshot, 🔺️diff, 🧬️mutations, 💡️inferences | Only four fixed schema children. |
| `semanticDirectoryMemberKinds/members-of-schema` (owner `schema`, 63 names, source registry) | Includes 🗄️registers and 🧱️kernel | `🗄️registers` and `🧱️kernel` are admitted children of `🧬️schema/`. No change needed for D's parent or kernel. |
| `registers` kind | Not present in `semanticDirectoryKinds`. No member list has owner `registers`. | Children of `🗄️registers/` have no admitting rule. Registration required (section D.8). |
| Existing `🗄️registers/🗂️catalog/` | Admitted only through `os-scale-generated-catalog` (owner `generated`) and `members-of-editor` (owner `editor`). Neither applies to `registers`. | Verify. Not a precedent for new children. |
| `subsetChildDirs` | 🧬️schema, 🚪️io, 📚️examples, 👁️viewer, ✏️editor | Subsets may hold only these children. |
| `artifactChildDirs` / `newArtifactChildDirs` | 🧬️schema, 🚪️io, 📚️examples, 🔨️modules / 🏅️standards | Artifact roots also contain 📦️packages, 🧪️tests, 📖️stories, 🤖️generated, 🛂️manifest. These are not in the lists here; presumably fixed contracts (verify). No split in this ticket needs a new artifact-root child. |
| `libWiringLineBudget` = 150; `packageGlueGrammar.rust` roles declaration / registration / bootstrap / thin-delegation, `maxDelegationStatements` 32 | Lib-root wiring budget | Applies to package entries. Whether it applies to artifact lib roots `🦀️.rs` under `🗿️artifacts` was not verified. C's root is far above 150 today. |
| `rustEntryPathRules` | `#[path]` resolution is cumulative (nested `pub mod X { … }` resolves against the enclosing base; `#[path = "."]` keeps the base). Under a `mod x;` in a non-inline file, `#[path]` is relative to that file's directory. | Governs C's re-homing (C.5) and all `#[path]` rewrites. |
| `mutationDirectoryPattern` = `^.+\uFE0F[a-z][a-z0-9]*(?:-[a-z0-9]+)+$` | Mutation directories are kebab-named | C's 232 mutation folders are direct children of `🧬️mutations/`. |

---

## 2. A: `✏️editor/🦀️.rs` of the 5d puzzle (9,141 lines)

### A.1 Owner, targets, mounting

- Crate: `semio-s-artifact-puzzle-5d`, manifest `🖐️5d/📦️packages/🦀️rust/Cargo.toml`, lib root `🖐️5d/🦀️.rs` (`[lib] path = "../../🦀️.rs"`). Crate paths `crate::editor::puzzle5d::…` resolve from that root.
- Targets: native (`cargo test`) and wasm (`cfg(all(target_arch = "wasm32", not(target_env = "p2")))` gates present in the package manifest). The file has 7 `#[cfg(` lines. Wasm-vs-native gates are not in this file; verify that the moved children keep their gates.
- Mounting: 5d root `🖐️5d/🦀️.rs` lines 2164-2166:
  `pub mod editor { pub mod puzzle5d { #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"] mod component; pub use component::*; …`.
  The editor file is therefore `crate::editor::puzzle5d::*` through a glob re-export.
- Existing children of the editor directory (precedent, all admitted): 🌉️wasm, 🎚️config (+🧬️schema), 🎭️modes (+✏️edit), 🎮️commands (55 command folders), 👥️presence (+🧬️schema), 📌️panels (⚙️settings, 🔍️inspection, 🗿️artifact, 🛍️catalogue), 📚️examples (🎬️demo-session), 📸️snapshot, 🗣️terminology (+🧪️tests), 🧠️precompute (+📐️geometry, 🖌️brush, 🪣️fill, 🧪️tests), 🧪️tests (7 mounts), 🪟️window (+🧬️schema, 🧪️tests; `🪟️window` is not admitted by any rule found; verify), 🫧️transient.

### A.2 Section table (line ranges, inclusive)

| Lines | Content (main items, line of item) | Active | Goes to |
|---|---|---|---|
| 1-14 | `//!` file doc | 0 | P |
| 15-16 | `#[path="📸️snapshot/🦀️.rs"] pub mod snapshot;` | 2 | P (path unchanged) |
| 17-113 | `use` block (crate paths, `semio_framework_*`, one cross-plugin `use` at 42) | ~97 | P |
| 115-274 | `//#region Constants` (PUZZLE5D_* consts 116-185, LazyLock examples 191-197, reserved sizes 200-208, `puzzle5d_action` 210, `puzzle5d_interaction_select` 217, `Puzzle5dFreshIds` 223, `next_scoped_id` 265) | ~150 | entities |
| 276-809 | `//#region Document`: Puzzle5dGrip2d 279, Grip3d 290, Grip 303, PartAnchor 315, Fastener 323, Part2d 349, Part3d 374, Part 389, TargetVolume 409, Document 425, empty/from_json/example docs 445-469, value helpers 476-557, paste mutations 557, grip/mesh/geometry fns 577-785 | ~540 | entities |
| 811-1018 | `//#region Scene` (Puzzle5dScene 817, InteractionSnapshot 837 + impl 844, scene_from_projection 936/942, scene mode/gumball fns 949-997) and `//#region Engine` (engine_grip_kind 1015) | ~134 | view |
| 1020-1153 | `//#region Labels` (kind catalog label 1023, part display label 1053, next part label 1090/1117) | ~100 | catalog |
| 1155-1227 | `//#region InferredKinds` (inferred part kind rows 1161, inferred kind rows 1211) | ~50 | catalog |
| 1229-1230 | `//#region SelectionFlags` / `//#endregion` (empty shell) | 0 | delete the empty shell |
| 1232-1311 | `//#region Distribution` (kind ids 1233, uniform weights 1249, normalise 1257, ensure 1290, sum 1308) | ~70 | catalog |
| 1313-1412 | `//#region CopyPaste` (copy/paste helpers 1315-1411) | ~90 | interaction |
| 1414-2864 | `//#region ReservedJobs`: stage enums and owners, selection scan (1476-1581), clipboard work and job (1582-1851), paste job (1852-2169), import job and decoders (2171-2863) | ~1,355 | session |
| 2866-3017 | `//#region ContextMenu` (Puzzle5dContextSelection 2879 + impl 2887, reserved actions 2947, menu items 2958) | ~150 | interaction |
| 3019-3214 | `//#region Puzzle5dCommand`: `macro_rules! puzzle5d_command_variants` 3032 (invoked once at 3104), `OpBinary` impl 3202, tool job ids 3190 | ~190 | interaction |
| 3216-3450 | `//#region ActionContext`: Puzzle5dActionCtx 3220 + impl 3260, notice emit 3390, copy/cut fragment fns 3396-3414, interaction part ids 3438, `ui_label` 3447 | ~220 | interaction |
| 3452-3649 | `//#region PlayApp`: `with_puzzle5d_app` 3460, `Puzzle5dPlayApp` 3466 + impl 3468 and 3558, `dispatch_puzzle5d_action` 3584 | ~180 | P (recommended; see A.4) or interaction |
| 3650-3951 | `//#region RetainedCommands` head: `PUZZLE5D_RETAINED_TOOL_IDS` 3651, window ids 3736, payload schema 3778, extent/reduce 3780-3805, nonce 3806, `Puzzle5dWindowCommandWork` 3812-3911, `Puzzle5dExportWork` 3913-3951 | ~195 | retained |
| 3952-5876 | Work structs: Transform 3953-4131, KindWeight 4132-4410, FocusSelection 4411-4599, PatchPart 4600-4773, PatchFastener 4774-4924, EditFastener 4925-5081, RetargetFastener 5082-5343, ProximityConnect 5344-5614, PatchGrip 5615-5795, DeleteFastener 5796-5876 | ~1,693 | gestures |
| 5877-7589 | Work structs: AddNode 5878-6053, AddBrushPart 6054-6346, BoardEvents 6347-6872, CreateFastener 6873-7112, SetActiveExample 7113-7448, RegisterBrushMesh 7458-7589 (`//#region` markers at 7450 area) | ~1,721 (with 7731-8005) | kit |
| 7590-7730 | `Puzzle5dRetainedCommandJobFactory` 7590 + `ArtifactOwnedToolJobFactory` impl 7645; `//#endregion RetainedCommands` 7730 | ~140 | retained |
| 7731-8005 | `//#region StorePreparation`: one-item store preparation factories and impls 7734-8000 | ~270 | kit |
| 8007-8124 | `//#region ToolProofs`: retained proofs 8011-8110, host configuration proofs 8111-8122 (both carry `owner_file` literals at 8016 and 8116) | ~115 | retained |
| 8126-8643 | `impl ArtifactEditor for Puzzle5dPlayApp` (dispatch-only) | ~480 | P |
| 8645-9111 | `//#region Manifest`: part-kind options, `create_puzzle5d_app` 8782 | ~440 | P |
| 9113-9141 | `//#region UnitTests`: `#[cfg(test)] #[path]` mounts 9116-9141 (unit 9117, agent-lane 9121, target-volumes 9127, retained-wiring 9132, host-admission 9136, gesture-kinds 9140) | ~12 | P (see A.5 for per-child test moves) |

Active-line count (non-blank, non-comment, from `coupling.py`): P 884, entities 538, view 134, catalog 247, interaction 662 (or 662 minus PlayApp when moved to P), session 1,355, retained 495, gestures 1,693, kit 1,721. Total 7,729.

### A.3 Proposed children (names admitted, all under `✏️editor/`)

Each child is `✏️editor/<name>/🦀️.rs` with `#[path = "<name>/🦀️.rs"] mod <snake>;` and `pub(crate) use <snake>::*;` in the editor file. Emoji copied from taxonomy.

| # | Folder | Admitted by | Line sets | Active | Content |
|---|---|---|---|---|---|
| C1 | `🧩️entities` | members-of-editor | 115-809 | 538 | Constants, document model, grip/part/fastener geometry, mesh resolution |
| C2 | `🧭️view` | members-of-editor | 811-1018 | 134 | Scene projection, interaction snapshot, gumball, engine grip kind |
| C3 | `🗂️catalog` | members-of-editor | 1020-1311 | 247 | Labels, inferred kind rows, kind distribution |
| C4 | `🕹️interaction` | members-of-editor | 1313-1412, 2866-3450 | 662 | Copy/paste, context menu, `Puzzle5dCommand` macro and enum, action context |
| C5 | `🧵️session` | members-of-editor | 1414-2864 | 1,355 | Reserved clipboard, paste and import jobs |
| C6 | `🧵️retained` | kind `retained` (parent editor) | 3650-3951, 7590-7730, 8007-8124 | 495 | Retained tool-id tables, window/export work, job factory, proofs |
| C7 | `🧵️gestures` | members-of-editor | 3952-5876 | 1,693 | Transform, kind-weight, focus, patch, fastener, grip and proximity work |
| C8 | `🧰️kit` | members-of-editor | 5877-7589, 7731-8005 | 1,721 | Authoring work (add node, brush, board events, create fastener, active example, register mesh) and store preparation |

Parent `✏️editor/🦀️.rs` keeps 1-114 (uses), 8126-9141 (ArtifactEditor impl, Manifest, test mounts) and, in the recommended variant, the `PlayApp` block 3452-3649. The coupling run put PlayApp in C4; with the recommended variant the parent carries about 884 + PlayApp active lines (about 1,050) and C4 drops to about 480. Expected parent size is about 1,250 lines including wiring.

Sizes: all children are between 134 and 1,721 active lines. C2 and C3 are small. Merge them into C1 only if the taxonomy reviewer prefers fewer folders; the names are admitted either way.

Rejected alternatives:
- `🧠️precompute` (already a child) is not re-used. The brush/fill helpers stay where they are.
- Moving every Work struct into its `🎮️commands/<cmd>/🦀️.rs`: the header of the monolith says behaviour lives in `🎮️commands/<group>/🦀️.rs`, which is the stronger taxonomy fit. Only 6 mappings are clear by name: Export → 📤️export-snapshot, KindWeight → ⚖️set-kind-weight, FocusSelection → 🎯️focus-selection, ProximityConnect → 📡️proximity-connect, AddBrushPart → 🖌️add-brush-part, RegisterBrushMesh → 📋️register-brush-mesh. This option is blocked until the `🎮️commands` manifest question (section 1) is answered, and it needs dispatch verification in `PUZZLE5D_RETAINED_TOOL_IDS` (3651). Not used in the extraction plan below.

### A.4 Coupling and visibility

From `coupling.py` (token-based, over-approximate; generic tokens such as `new`, `args`, `snapshot`, `label`, `row`, `default` are noise):

- Heaviest cross edges: entities → session 24 names, entities → kit 24, entities → P 18, entities → interaction 17, entities → gestures 17. Then gestures → P 16, interaction → P 15, retained → P 15, session → gestures 14, session ↔ kit 15/6, gestures ↔ kit 11/5.
- Hub names: `puzzle5d_editor_projection` (44 uses from entities), `PUZZLE5D_INTERACTION_DOMAIN` (28), `Puzzle5dPart` (25), `Puzzle5dDocument` (20), `PUZZLE5D_GRANULARITY_PART` (19), `Puzzle5dFastener` (16), `Puzzle5dGrip` (10), `puzzle5d_grip_full_id` (11), `puzzle5d_resolve_number_edit` (10).
- `Puzzle5dPlayApp` (144 mentions across gestures, kit, retained, session): it should stay in P (with the dispatch function), not in interaction.
- `Puzzle5dCommand` (53 mentions from gestures, kit, retained): keep it in interaction; it is the command vocabulary. Its macro (3032) and its one invocation (3104) must stay in the same child.
- Private names used across children (need `pub(super)` or `pub(crate)` after the move): entities 17, interaction 13, session 21, retained 18, gestures 23, kit 20, P 6. Blanket rule (as in the shell example): prefix private items and fields that another chunk uses with `pub(super)`. Keep the `crate::` paths unchanged.
- Cycles (session ↔ kit, gestures ↔ kit, entities ↔ everything) are legal Rust because children glob-import from the parent. They are a design smell, not a build blocker.

### A.5 Path rewrites, tests, macros, readers, wrong-direction

- Relative paths in the file: only `#[path = "📸️snapshot/🦀️.rs"]` (15) and the 6 unit-test mounts (9116-9141) plus `#[path]` attributes in the file's test wiring. Children sit one folder deeper, so any relative `#[path]` moved into a child needs one more `../` only if it is relative. Crate-absolute `crate::…` paths (395 occurrences) do not change.
- Test modules: `unit` (9117) remains in P (its source reader needs care, see below). `retained-wiring` (9132) moves to C6. `gesture-kinds` (9140) moves to C7. `target-volumes` (9127) and `host-admission` (9136) stay in P.
- Macro hazard: `macro_rules! puzzle5d_command_variants` (3032) is textually scoped. Its definition and its single invocation (3104) are in the same child (C4). Keep them together and keep the `#[derive(...)]` attribute with the enum it generates.
- Source-text reader (blocks extraction until re-pointed): `🧪️tests/🔬️unit/🦀️.rs:468` `let source = include_str!("../../🦀️.rs");` resolves to the editor file. Lines 479-485 assert marker strings: `struct Puzzle5dBoardEventsWork`, `Puzzle5dBoardEventsStage::ScanEdge`, `self.scan_one(source)?`, `puzzle5d_transform_tool_commit("applyBoardEvents"`. These markers live in C8 (kit) and C7 (gestures) after the split. Re-point the reader to the concatenation of the children (or assert on the child file), or move the assertion into the child's own test.
- Ownership literals: `owner_file: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/…"` at 8016 and 8116 (in C6). Update them to the child file. `🧪️tests/🧫️fixtures/🔏️publication-authority/🔣️.json` (line 338) names `🗿️artifacts/🖐️5d/…/✏️editor/🦀️.rs` as a source. Update to the owning child.
- Wrong-direction dependencies (plugin naming other concrete plugins):
  - line 42: `use semio_s_artifact_puzzle_3d_editor::Puzzle3dInstanceOperationOwner;`
  - line 634: `semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionMotion`
  - line 3265 and 4018: `semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord`
  - line 3295: `semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionsLink`
  - line 8219: `semio_s_artifact_puzzle_3d_editor_modes::windows::main::utilities::brush::run_effects`
  - `🖐️5d/📦️packages/🦀️rust/Cargo.toml` (workspace deps 84-123) depends on the 3d crates.
  The 5d editor must not name concrete 3d crates. The shared selection/brush contracts need a framework-neutral home that both 5d and 3d depend on. This is not a split concern and is not fixed by the split; it is recorded here.
- Source-text reader grep (run): `/usr/bin/grep -rln -F` for the relative tails returned no hits outside the file's own folder except the fixture and `owner_file` lines above.

### A.6 Extraction sequence (bottom-up, so line numbers above stay valid)

Before the first cut: remove the empty shell at 1229-1230 (`SelectionFlags`), keep a byte copy of the original outside the repo (scratchpad) for the diff check.

1. C8 `kit`: move 7731-8005 (StorePreparation) and 5877-7589 (authoring work). Mount the child, add `pub(crate) use kit::*;`. Update the `include_str!` reader in `🔬️unit` to read the children.
2. C6 `retained`: move 8007-8124 (proofs), 7590-7730 (job factory), then 3650-3951 (retained head). Update `owner_file` literals.
3. C7 `gestures`: move 3952-5876. Move `gesture-kinds` mount.
4. C4 `interaction`: move 1313-1412 and 2866-3450 (keep the macro with its invocation). PlayApp (3452-3649) moves to P.
5. C5 `session`: move 1414-2864.
6. C3 `catalog`: move 1020-1311.
7. C2 `view`: move 811-1018.
8. C1 `entities`: move 115-809. Last, because most other chunks depend on it; `pub(super)` items from section A.4 are required here.
9. After each step: `cargo check` for native and wasm32 targets (use the repo's cargo gate, not raw cargo; see project memory), `--tests` build for the 5d crate, and a diff check that concatenation of the child ranges plus the parent equals the original line set, modulo openers, `pub(super)` prefixes and wiring.

Expected end state: parent about 1,250 lines; children 134-1,721 active lines; no child above the 2,500-line target.

---

## 3. B: 2d `✏️editor/🦀️.rs` (4,059 lines)

### B.1 Owner, targets, mounting

- Crate: `semio-s-artifact-puzzle-2d`, lib root `◻️2d/🦀️.rs` (`[lib] path = "../../🦀️.rs"`). Mounted at 2d root `◻️2d/🦀️.rs` lines 1200-1204 inside `pub mod editor { pub mod puzzle2d { … } }`.
- Existing editor children (precedent): 🌉️wasm, 🎚️config (+🧬️schema), 🎭️modes, 🎮️commands (groups 🎛️runtime, 🧊️document), 👥️presence, 📌️panels, 📸️snapshot, 📼️recorder, 🗣️terminology, ⚙️engine, ⏳️precompute, 🧬️schema, 🧮️history, 🪟️window, 🧪️tests (14 entries). The editor file itself has `mod snapshot` (11-12) and `mod recorder` (14-15).

### B.2 Section table

| Lines | Content | Active | Goes to |
|---|---|---|---|
| 1-10 | `//!` doc | 0 | P |
| 11-15 | `pub mod snapshot`, `pub mod recorder` mounts | 4 | P |
| 17-101 | `use` block (crate paths, `semio_framework_*`) | ~80 | P |
| 106-136 | `//#region Constants` (no active code; region markers and blanks only) | 0 | delete the shell |
| 138-308 | Scene (138), BoardReads (222), TargetRegions (231), Clipboard (267), BoardHostSync (286), UiScopes (300); no active code. Verified: 189 blank, 14 marker lines, 0 code lines in 106-308. | 0 | delete the shells |
| 310-316 | `//#region Puzzle2dCommand` (enum/macro-free) | ~6 | interaction |
| 318-465 | ActionContext: `Puzzle2dActionCtx` 322 + impl 362, `ui_label` 462 | ~140 | interaction |
| 467-577 | ContextMenu | ~110 | interaction |
| 579-628 | PlayApp struct 583 and impl 585 | ~50 | P |
| 629-909 | RetainedCommands head: `PUZZLE2D_RETAINED_TOOL_IDS` 633, host tool ids 700, generic tool ids 725, `Puzzle2dRetainedCommandJobFactory` 789-909 | ~280 | retained |
| 910-1232 | StorePreparation: config and artifact store preparation factories and impls | ~267 | kit |
| 1238-1354 | BoardEvents constants (1238), `Puzzle2dWindowCommandWork` 1260, `Puzzle2dExportWork` 1316, `puzzle2d_dispatch_emit` 1355 | ~200 | retained |
| 1355-1569 | Dispatch emit, extents, active example reduce (1540-1569) | ~150 | retained |
| 1570-2175 | Force layout (`Puzzle2dForceLayoutWork` 1645, impl 1682, `macro_rules! pop` 1744 local to a fn, work impl 2003) | ~430 | retained |
| 2176-2489 | Redraw handles work (`Puzzle2dRedrawHandlesWork` 2216, impl 2245, work impl 2387) | ~314 | retained |
| 2491-3159 | ReservedJobs: import constants and decoders 2498-2631, `Puzzle2dImportJob` 2664 and impls 2699/2800/2947, `Puzzle2dClipboardJob` 3014-3159 | ~670 | session |
| 3165-3256 | ToolProofs: retained proofs 3165, host configuration proofs 3243 | ~90 | session (see note) |
| 3258-3705 | `impl ArtifactEditor for Puzzle2dPlayApp` (3258), `impl Puzzle2dPlayApp` (3691) | ~430 | P |
| 3708-4001 | Manifest: `create_puzzle2d_app` 3746, fault notices, flag helpers | ~290 | P |
| 4003-4059 | UnitTests: 9 `#[cfg(test)] #[path]` mounts (unit 4007, agent-lane 4011, clipboard 4017, locks 4023, select-tool-transactions 4029, board-tools 4035, recorded-kinds 4041, select-tool-history 4047, history-edit-runtime 4054) | ~56 | P |

Active lines (`coupling.py`): P 741, interaction 172, kit 267, retained 1,316, session 675. Total 3,171. Empty shell 106-308 contributes 0.

### B.3 Proposed children

| # | Folder | Admitted by | Lines | Active | Content |
|---|---|---|---|---|---|
| D1 | `🧵️retained` | kind `retained` | 629-909, 1238-2489 | 1,316 | Retained tool tables, job factory, window/export work, force layout, redraw work |
| D2 | `🧵️session` | members-of-editor | 2491-3256 | 675 | Reserved import and clipboard jobs, tool proofs |
| D3 | `🧰️kit` (optional) | members-of-editor | 910-1232 | 267 | Store preparation factories |
| D4 | `🕹️interaction` (optional) | members-of-editor | 310-577 | 172 | Command enum, action context, context menu |

Required cut: D1 and D2 (after deleting the empty shell). Expected parent: about 740 active lines + wiring; file below 1,500 lines. D3 and D4 are optional and only reduce parent size slightly. If D3/D4 are used, `🧵️` appears twice among siblings (retained and session); the registry already lists several 🧵 members under editor, so this is admitted; verify with the taxonomy validator.

### B.4 Coupling and visibility

- retained → P: 23 names. Mostly `snapshot`, `Puzzle2dPlayApp`, `Config`, `handle`, `recorder`. `Puzzle2dPlayApp` stays in P.
- kit → session 9, retained → session 8, kit → retained 8 (close-step demand helpers `checkpoint`, `begin_close`, `close_step`, `terminal_is_empty`, `next_close_*`). These helpers belong to the close-step group in retained; keep them with retained or move kit with it.
- P → retained 7 (proofs, job factory). Private names used elsewhere: retained 24, kit 11, session 8, interaction 1, P 5.

### B.5 Path rewrites, tests, macros, readers

- Test modules (9 mounts at 4006-4054) stay in P, as in A.5. Unit `🔬️unit` (4007) includes `include_str!("../../🦀️.rs")` at 411 (the editor file). Its markers (404-406) include `"Puzzle2dForceStage::Emit"`, `"matches!(command.action_id(), \"addNode\")..."`, and negative markers: `!source.contains("forceLayout" => Box::new(crate::retained_command::BoundedFirstStepCommandWork`) and `!source.contains("Puzzle2dActiveExampleWork")`. After the split these markers live in D1 (force layout, redraw). Re-point the reader to the child files.
- `include_str!("../../🎮️commands/📥️import-snapshot/🧪️tests/🧫️fixtures/📄️payload-input/…")` at 1775 of the unit test does not change.
- `owner_file` literals at 3170 and 3248 (ToolProofs, in D2 after the split) must be updated.
- Ownership fixture: `🧪️tests/🧫️fixtures/🔏️publication-authority/🔣️.json` line 7 names `🗿️artifacts/◻️2d/…/✏️editor/🦀️.rs` as the source of `Puzzle2dPlayApp`. Update to the owning child.
- Macro: `macro_rules! pop` (1744) is local to a `fn` inside the force-layout impl; move it with the fn (D1). No other macro in the file.
- `#[cfg(` lines: 11. Keep each cfg pair with its item.

### B.6 Extraction sequence

1. Delete the empty shell 106-308 (verify no references to `//#region Scene` etc. in docs; the markers are the only content).
2. D2 `session`: move 2491-3256. Update `owner_file` literals. Re-point the unit reader if it reads these markers.
3. D1 `retained`: move 629-909 and 1238-2489 (two cuts; anchor 909/910 and 1232/1238).
4. Optional D3 `kit` (910-1232) and D4 `interaction` (310-577).
5. After each step: native and wasm32 `cargo check` through the repo gate, `--tests` build, diff check.

### B.7 Wrong-direction dependencies

- `◻️2d/📦️packages/🦀️rust/Cargo.toml` depends on `semio-s-artifact-puzzle-3d` (line 47) and `semio-s-artifact-puzzle-3d-inferences` (line 19). The package is a concrete plugin depending on sibling concrete plugins.
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs` imports `semio_s_artifact_puzzle_3d` (the only grep hit under `◻️2d`). This is outside the editor file but in the same plugin.
- The editor file itself has no `semio_s_` import.

---

## 4. C: bim model `🏢️model/🦀️.rs` (7,817 lines)

### C.1 Owner, targets, mounting

- Crate: `semio-s-artifact-bim-model`; the file is the lib root itself (`[lib] path = "../../🦀️.rs"` from the package manifest). Lines 1-62: `definition()`, `artifact<A: BimApplication>()`, `BimApplication` trait; 64-66 `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;`.
- The rest is an inline module tree. Counts in the file: `#[path` 2,735, `#[cfg(` 1,589, `pub mod` in the mutation range (381-7290) 652, `#[cfg(test)]` in that range 1,584, `#[derive` 0, `macro_rules!` 0, `semio_s_` 0, `crate::` 0.
- Mutation directories: `🧬️mutations/` has 233 entries (232 directories plus `🦀️.rs`). Each mutation directory has a component `🦀️.rs`; the leaves `🔺️diff`, `↩️inverse` and `🧪️tests/*` are mounted from the root today.
- Precedent for file-level mounts inside a component: `🧬️schema/🧬️mutations/🦀️.rs` lines 8-10 (`#[cfg(test)] #[path = "🧪️tests/🧰️kit/🦀️.rs"] pub mod kit;`). The mutations component is 501 lines and already has 10 mounts.

### C.2 Section table

| Lines | Content |
|---|---|
| 1-62 | Declaration (code, stays in root) |
| 64-66 | Root unit-test mount (stays) |
| 68-7301 | `pub mod standards` (inline, `#[path = "."]`): `v1` 71-7300 with `v1_component` (72-73), `subsets` (75-…), `any` (78-7300) with `any_component` (79-80); inside `any`: `schema` 83-380 (component 84-85; `snapshot` 88-92; `authored` 94-97; `diff` 100-103; `inferences` 106-379 with component 107-108 and 40-odd nested wrappers, e.g. `storey_levels` 111, `element_solids` 277-…, `walls`, `plan_kit`, `columns`, `beams` inside it); `mutations` 381-7290 (component `🧬️mutations/🦀️.rs` mount at 382-384; 232 mutation wrappers with leaf mounts); `io` 7292-7300 (component 7293-7294) |
| 7302-7335 | `pub mod examples` (inline): checks 7305, house 7308-7314, office 7317-7323, demo 7326-7332 |
| 7336-7777 | `pub mod editor { pub mod bim { … } }` (inline): component 7340-7341; children `kit` 7344, `terminology` 7350, `entities` 7356, `interaction` 7362, `utilities` 7368, `gestures` 7374, `chrome` 7380, `presence` 7386, `config` 7392, `transient` 7398, all `#[path]` to existing components; `commands` 7403-7675 (inline, nested mounts); `modes` 7676-7723 (`edit` 7678-…); `panels` 7724-7777 (`options` 7726-…) |
| 7778-7811 | `pub mod viewer { pub mod bim { … } }` (component 7782; `modes/view` 7785-7810) |
| 7812-7816 | `pub mod render` (component 7814-7815) |

### C.3 Admissibility of the target components

All target components are `#[path]` targets that compile today. The editor children `kit` (members-of-editor), `entities` (members-of-editor), `interaction` (members-of-editor), `gestures` (members-of-editor), `chrome` (members-of-editor), `terminology`, `presence`, `config`, `transient` (surfaceChildDirs) are admitted. `utilities` is not in members-of-editor or surfaceChildDirs (windowChildDirs only): verify. No new directory is needed for C.

### C.4 Why this is not a code split

The file has no algorithmic code to move; the bulk is declarations. Splitting it by line range is meaningless. The valid seam is to put each mount in the component file that owns the module (taxonomy rule: mounts belong to their owning directory's component).

### C.5 Re-homing plan (ordered, bottom-up)

1. Mutation leaves: for each of the 232 mutation folders, move its own leaf mounts (`pub mod diff`, `pub mod inverse`, `#[cfg(test)] mod tests_*`, the `#[path]` pairs) into the mutation's own `🦀️.rs`. Rewrite each `#[path]` relative to that component's directory (drop the root-relative prefix `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<mutation>/`). About 5,000 lines move.
2. Mutation-level wrappers (232 × `pub mod X { #[path] mod component; pub use component::*; }`, about 930 lines): move into `🧬️mutations/🦀️.rs`. That file grows from 501 to about 1,430 lines (below 2,500).
3. `inferences` (106-379): move wrappers into `🧬️schema/💡️inferences/🦀️.rs` (exists). About 275 lines.
4. `schema` wrappers (83-105: `snapshot`, `authored`, `diff`) and the `mod mutations` declaration: into `🧬️schema/🦀️.rs` (67 lines today, about 110 after).
5. `io` (7292-7301) into `🚪️io/🦀️.rs`. `examples` (7302-7335) into `📚️examples/🦀️.rs`. `editor` (7336-7777, about 440 lines) into `✏️editor/🦀️.rs` of the bim editor. `viewer` (7778-7811) into `👁️viewer/🦀️.rs`. `render` (7812-7816) into `🖌️render/🦀️.rs`. Each of these components exists (its `#[path]` target compiles today).
6. Root keeps 1-82 (declaration, unit test, `standards::v1::subsets::any` wrappers) and the closing braces. About 100 lines (inside the 150-line budget if it applies).

Path rewrites: every moved `#[path]` string loses the root-relative prefix up to the owner's directory. About 2,700 attributes. Mechanical but large; a script plus `cargo check` per step. Confirm the path base rule with the validator (`rustEntryPathRules`): inside a non-inline file, `#[path]` is relative to that file's directory.

Re-export discipline: `pub use component::*;` in each owning component keeps `crate::standards::v1::subsets::any::schema::snapshot` and the other paths resolving, because the child modules move into the component and the glob re-exports them.

Test modules and readers: `🧪️tests/🔬️unit/🦀️.rs:6` reads `../../🏅️standards/…/🧪️tests/🧫️fixtures/…/🏠️house/📸️snapshot/🔣️.json`. The path is relative to the unit test and does not change. All 1,584 `#[cfg(test)]` attributes move with their mounts.

Macro hazards: none (`macro_rules!` 0 in the file). `#[wasm_bindgen]` 0. `cfg(target_arch)` 0 in the file; the 1,589 cfg attributes are almost all `#[cfg(test)]`.

Wrong-direction dependencies: none found (`semio_s_` 0 in the file).

Verification: native and wasm32 `cargo check` per step through the repo gate; `--tests` build; count `#[path]` before and after (must match, 2,735 total); confirm the root is below the wiring budget.

---

## 5. D: architect registers `🗄️registers/🦀️.rs` (7,727 lines)

### D.1 Owner, targets, mounting

- Crate: `semio-s-artifact-architect-program`. Lib root: `🏛️program/🦀️.rs`.
- Mounting: architect root `🏛️program/🦀️.rs` lines 105-111: `pub mod registers { #[path = "…/🗄️registers/🦀️.rs"] mod component; pub use component::*; #[path = "…/🗄️registers/🗂️catalog/🦀️.rs"] pub mod catalog; }`.
- Architect `🧬️schema/🦀️.rs` (501 lines) imports `use crate::standards::v1::subsets::any::schema::registers::*;` (line 4) and re-exports `RegisterKey`, `RegisterRow` (line 5). Verified: `TraceLink`, `RegisterKey` and `RegisterRow` are defined in `🧬️schema/🧱️kernel/🦀️.rs` (only definition hit among schema children). The registers component consumes them through the kernel import path.
- Sibling precedent: `🗄️registers/🗂️catalog/` (catalog 1,799 lines with `🟦️.ts`). Not admitted by a registers rule (section 1).
- Targets: native and wasm32; the component has one `#[cfg(test)]` mount at 7725-7727 and no `#[cfg(target_arch)]`.

### D.2 Section table

| Lines | Content | Active | Goes to |
|---|---|---|---|
| 1-9 | `//!` doc (65 feature-area registers plus meta/project/governance; points to `registers` and `kernel`); `extern crate` lines | 6 | P |
| 10-31 | `// #region PatchHelpers`: `trait PatchRow` 14, `impl T` 19, `impl Option` 31 | ~20 | P (kept for textual scope) |
| 43-53 | `macro_rules! impl_identified_header` (used 65 times) | 11 | P |
| 55-88 | `macro_rules! impl_patchable` (used 69 times, expands to `RowPatch<…>` impls referencing `PatchRow`, `protocol::`) | 33 | P |
| 90-588 | `// #region SharedEnums`: 29 enums (InfluenceLevel 96 … DeliveryPhase 577) | 468 | kernel |
| 589-1681 | Families F1 (see D.3) | 1,034 | F1 |
| 1682-3214 | F2 | 1,449 | F2 |
| 3215-4859 | F3 | 1,549 | F3 |
| 4860-6281 | F4 | 1,338 | F4 |
| 6282-7727 | F5 (incl. `impl Identified for Governance` 7685, `impl Governance` 7685-7724, `#[cfg(test)] #[path] mod tests;` 7725-7727) | 1,363 | F5 |

Each record is a `struct X` plus `struct XPatch`, with `impl_patchable!` and `impl_identified_header!` invocations. Only one `impl` with logic exists per few records (ProjectDefinition 728, Governance 7685).

Active lines (`coupling.py`): P 65, kernel (enums) 468, F1 1,034, F2 1,449, F3 1,549, F4 1,338, F5 1,363. Total 6,266.

Note: the coupling tool reports `kernel` for the enums chunk (SharedEnums) — the kernel component `🧱️kernel/🦀️.rs` itself is 402 lines and is not part of this file.

### D.3 Proposed families (record grouping by contiguous regions)

Family folder names are candidates. None is admitted today; registration is required (D.8).

| Folder (candidate) | Lines | Records (struct + Patch each) | Active |
|---|---|---|---|
| `🧭️program-structure` | 589-1681 | ProgramMeta, ProjectDefinition, Stakeholder, UserProfile, Activity, Function, ProgramElement, QuantityRequirement, Relationship, Adjacency | 1,034 |
| `🔁️operations-requirements` | 1682-3214 | Process, FlowRequirement, AccessRule, OperationalRequirement, Equipment, Resource, StorageRequirement, EnvironmentalRequirement, HumanFactorRequirement, AccessibilityRequirement, PrivacyRequirement, SafetyRequirement, SecurityRequirement, RegulatoryRequirement | 1,449 |
| `🌐️context-requirements` | 3215-4859 | SiteContext, OrganizationalRequirement, ServiceRequirement, InfrastructureRequirement, InformationRequirement, CommunicationRequirement, WayfindingRequirement, ScheduleRequirement, FlexibilityRequirement, GrowthPlan, SustainabilityRequirement, ResilienceRequirement, CostRequirement, DeliveryConstraint, Risk, Conflict | 1,549 |
| `📋️decisions` | 4860-6281 | Requirement, PriorityRecord, Scenario, OptionEvaluation, Decision, ValidationRecord, PerformanceCriterion, QualityRecord, ArtifactRecord, ChangeRecord, CollaborationRecord, AnalysisRecord, ReportRecord, SearchFilter | 1,338 |
| `🗃️records` | 6282-7727 | StatusRecord, Workshop, Survey, Issue, AuditEvent, TemplateRecord, KnowledgeRecord, BenchmarkRecord, Assumption, ConstraintRecord, ComplianceRecord, ApprovalRecord, MeetingRecord, Governance | 1,363 |

All families are at or under 1,550 active lines (under the 2,500 target). Kernel (enums): 468 active.

Shape of the proposal: the family split is a choice of grouping, not a necessity. Any contiguous grouping works because the families are independent (D.4). The grouping above keeps each family under 1,600 active lines and keeps region markers at cut points.

Emoji check: `🧭️`, `🔁️`, `🌐️`, `📋️`, `🗃️` are distinct from each other and from `🗂️catalog`, the only sibling under `🗄️registers`. Each name must be checked against the repo for an existing emoji reuse before registration (verify).

### D.4 Coupling and visibility

- Families are independent. Cross-family references: none, except through `kernel`. Measured: kernel → F2 13 names, F1 10, F3 6, F5 4, F4 4; F3 → kernel 2; F1, F2, F4 → kernel 1 each; P → F1 1 and P → F5 1 (the `id` token; noise). No family uses another family's type.
- Kernel names used by families (must stay `pub`): `ValidationStatus` (30 uses, all families), `RiskLevel` (18: F2, F3, F5), `AccessLevel` (8), `SeparationKind` (6), `DeliveryPhase` (6), `InfluenceLevel` (4), `IssueSeverity` (4), `EngagementLevel`, `UserCategory`, `ProgramElementKind`, `FunctionKind`, `FlowKind` (1-2 each). All enums are already `pub`.
- Private names needing `pub(super)`: none in the families; P has 1 (the macro-facing `PatchRow` and helper trait items, if moved).

### D.5 Macro hazard (textual scope)

- `impl_identified_header!` (43) and `impl_patchable!` (55) are `macro_rules!` defined in this file. They are invoked 65 and 69 times (`impl_patchable!(` at 79, 635, 734, …). Textual scope applies: a macro is only visible after its definition, in the same module and in child modules declared after it. Keep both macros and `trait PatchRow` in the component file above the `mod` declarations of the families (lines 1-89). Do not move them to kernel without a `#[macro_use]` or `pub(crate) use` re-export (verify).
- The macro bodies reference `PatchRow`, `RowPatch`, `Identified`, `EntityId` and `protocol::ApplyCapability`/`protocol::MutationApplyError`. Families must import these from the parent (`use super::*;`).

### D.6 Tests

- `🗄️registers/🧪️tests/🔬️unit/🦀️.rs`: 4,182 lines, 65 `#[semio_framework_async_macros::async_test] async fn <record>_patch_round_trips()` tests, one per record, plus one local macro `patch_through_table!` (line 3-10). This test file is itself above 2,500 lines: it must be split with the families.
- Mapping by line:
  - F1 tests (stakeholder … adjacency): lines 23-580, 8 tests.
  - F2 tests (process … regulatory): 581-1512, 14 tests.
  - F3 tests (site_context … conflict): 1513-2510, 16 tests.
  - F4 and F5 tests: from `requirement_patch_round_trips` (2511) to the end (4115 for `meeting_record_patch_round_trips`), 27 tests. Split at the F4/F5 boundary (`status_record_patch_round_trips`, verify line).
- Each family's tests move to `<family>/🧪️tests/🔬️unit/🦀️.rs` (the `tests` kind has no parent restriction; the `🔬️unit` folder is the existing name). The `patch_through_table!` macro must be either duplicated (about 11 lines) or moved to a shared test-helper file mounted before the family test mods. Duplicating is simpler.
- No `include_str!` source-text reader in this test file (checked: no `include_str!`, `read_to_string`).

### D.7 Path rewrites

- `#[path]` attributes move with their children. Each family's `mod component;` is at `<family>/🦀️.rs`; the component's parent mount (architect root 105-111) adds the families through `pub use` in `registers/🦀️.rs` (or as `mod` children of the component).
- The `use crate::standards::v1::subsets::any::schema::registers::*;` in `schema/🦀️.rs` stays valid if the families are `pub use`-d from the component.
- Path length check (proposed, measured): `…/🗄️registers/🧭️program-structure/🦀️.rs` is 154 characters (217 bytes); its test file is 169 characters (242 bytes). Under 256.

### D.8 Registration required (where)

1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, `semanticDirectoryKinds`: add a `registers` kind (emoji 🗄️, slug `^registers$`, parentKindIds `["schema"]`). `🗄️registers` is currently admitted only as a memberName of `members-of-schema`.
2. Same file, `semanticDirectoryMemberKinds`: add `members-of-registers` with `ownerKindIds: ["registers"]`, `source: "registry"`, memberNames = the five family folders (and `🗂️catalog` only if it is kept; its admission must be reconciled with `os-scale-generated-catalog`).
3. Tests: families need no new kind (`tests` kind, `🔬️unit` precedent).
4. Until registration, D cannot be cut without violating the taxonomy. The gate command the repo uses (`verify dependencies literal-external` / taxonomy inventory) must be run after the registration; not run here.

### D.9 Extraction sequence

1. Registration (D.8), then verify the taxonomy inventory accepts `🗄️registers` as a `registers` kind with an empty member list.
2. Move SharedEnums 90-588 into `🧬️schema/🧱️kernel/🦀️.rs` (kernel is the documented home of shared primitives per the architect header). Check that kernel does not import registers.
3. Move the F5 block (6282-7727) to `🗃️records/`, with its test mod (2 cuts: records group and the tests mount). Bottom-up, so earlier line numbers stay valid.
4. Move F4 (4860-6281), then F3 (3215-4859), then F2 (1682-3214), then F1 (589-1681). Each cut is at a region marker (anchors 589, 1682, 3215, 4860, 6282).
5. Move the unit tests with their families (section D.6). Remove the single `#[cfg(test)] mod tests;` mount from the component.
6. After each step: native and wasm32 `cargo check`, `--tests` build, diff check (concatenation of child ranges plus parent equals the original, modulo `use`, `pub` and `mod` lines).

---

## 6. Cross-cutting extraction order (recommended across all four)

1. B (2d) first: the shortest path. Remove the empty shell, move `session` and `retained`. This tests the `include_str!` reader fix and the `owner_file` update on a small surface.
2. A (5d): then the same reader and `owner_file` fixes on the larger file. Resolve the 3d wrong-direction edges separately (they are a dependency change, not a split step).
3. D (architect): after taxonomy registration. Families are independent, so each can be cut as its own change.
4. C (bim): last. It needs ~2,700 `#[path]` rewrites and touches 232 mutation components. Do it as a scripted re-homing with one commit per subtree, not per file.

## 7. Open items (not resolved by this audit)

- Taxonomy validator not run: `🪟️window` (5d, 2d), `🧬️schema` under editor (2d), `🗂️catalog` under `🗄️registers`, `🪛️utilities` under bim editor, group folders `🎛️runtime` and `🧊️document` under 2d `🎮️commands`, and the 10 unlisted 5d command names. Each is either a drift or a rule this read did not find.
- Whether `libWiringLineBudget = 150` applies to artifact lib roots. Decides whether C's root must reach about 100 lines in this ticket.
- Whether the `semanticCollections` manifest rule (exact bijection with direct children) applies to nested `🎮️commands/` roots. Decides whether moving command work into `🎮️commands/<cmd>` is allowed without a manifest.
- Exact wasm32 and native cfg gates in the 2d, bim and architect manifests (only the 5d manifest was read for cfg).
- Usage of `semio_s_artifact_puzzle_3d` by the 2d package beyond the one inferences file (not grepped beyond `🧬️schema/💡️inferences`).
- Coupling figures are token-based and over-approximate. They guide the cut; they are not a substitute for `cargo check`.

Report produced by read-only audit. No file in the repository was modified. Helper scripts are in the session scratchpad only.
