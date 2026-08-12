# Remaining Work Map (censused 2026-08-12 13:30–14:30)

Evidence base for the finishing plan
`/Users/ueli/.claude/plans/finish-semantic-mutations-overhaul-melodic-horizon.md`.
Read together with `📓️taxonomy.md` (verb table) and `📓️derivation-rules.md` (per-facet recipe).

## Facet census (107 total)

| bucket | count | notes |
|---|---|---|
| migrated + clean | 14 | writer, procedural2d, procedural3d, flow, gisterrain, gismap, vcs, sequence, fem◻2d, fem🧊️3d, forms, layout, cad, draw |
| migrated + legacy dirs left | 18 | mathematical, animate-present, shooting, demonstrator-playground, architect-program, process-process3d, lowpoly, reasoning-wires, norm×9 (iso16757, vdi3805, din4108, din16798, en1990–en1995) |
| untouched | 69 | stdio 53, norm 5, block×3, puzzle×3, note, dag, energy, raster, sourcing |
| odd shape | 6 | playbook, imperative, remodel, trinity-rewrite, trinity-jack, space-home |

Legacy dirs survive because each plugin's `📦️glue.rs` `#[path]`-mounts the OLD dir names and
facet agents were denied glue edits. Migrated facets that couldn't touch glue either self-wired
inline `#[path]` in the dispatch file (animate) or repurposed an old dir's contents (norm's
`📄set-snapshot` now holds live `ChangeAnnex`; architect packed 266 variants into 72 noun dirs).

## Compile-broken crates (this ticket's own fallout)

Dangling glue mounts to deleted triad dirs — hard `E0583`, not warnings:

| crate | dangling mounts |
|---|---|
| `semio-s-plugin-writer` | `✍️set-text`, `📄set-snapshot` (glue.rs:96–112); real dirs are `✏️edit-text`, `🏷️rename-writer`, `🔗change-uri`, `🌐change-language`; dispatch imports those mods, declared nowhere |
| `semio-s-plugin-vcs` | `📛set-title`, `📝set-notes`, `🔢set-counter`, `🚦set-status`; ALSO needs added `🔺️diff` mounts for `add_tag`/`remove_tag` (leaves call `super::diff::diff`) |
| `semio-s-plugin-flow` | `📄set-snapshot`, `📐set-layout`, `🔗synapses`, `🧩widgets` |
| `semio-s-plugin-sequence` | 8 dirs: `{steps,edges}-{add,remove,move,patch}` |

New triad dirs exist and are complete in all four; repair is mechanical remounting. Dispatch enums
already reference glue-declared sibling mods (`super::<slug>`). Wave-2 reports for flow/writer
contain exact glue patches.

## CollectionMutation debt in migrated leaves (37 files)

Pattern to kill: manufacturing a throwaway `protocol::CollectionMutation` purely to feed a
`*_delta_from_collection_mutation` helper.

| plugin | count | files |
|---|---|---|
| gis (gismap) | 12 | all `🔺️diff`: `🆕create-{position,route,region}`, `🗑delete-*`, `🔁replace-*-data`, `🔀reorder-*` |
| flow | 8 | all `🔺️diff`: create/delete/replace-widget, reorder-{widgets,synapses}, connect/disconnect-widgets, update-synapse-endpoints |
| animate | 6 | 5 `🔺️diff` + `🎞tiles/🦠️mutation` |
| architect | 4 | `🔀adjacencies` full triad + `🧵traces/🦠️mutation` |
| layout | 4 | `📄pages`, `📖stories`, `🔗links`, `🌱create-page` `🦠️mutation`s |
| process | 2 | `📋steps/🦠️mutation`, `🔀reorder-steps/🦠️mutation` |
| shooting | 1 | `📦assets/🦠️mutation` |

Plus ~9 facet-level `🔺️diff/📝️text/🦀️component.rs` files that DEFINE the
`*_delta_from_collection_mutation` engines — delete once callers are gone.

**Target pattern** (already used by clean facets): the `🔺️diff` leaf takes `(payload, base)` and
returns the facet's `*Diff` built from its own sparse `*Delta` structs (`added`/`removed`/`set`/
`modified`/`reordered`), `None` for untouched collections; never applies-then-captures. Reference
leaves: `🎬️sequence/…/🌱create-step/🔺️diff` (literal `SequenceStepsDelta` construction),
`🎬️sequence/…/🗑️delete-step/🔺️diff` (reads `base` for severed-edge cascade),
`📋️forms/…/➕add-step/🔺️diff` (early `Default` return on id collision),
`🌀️procedural/…/🎛set-widget/🔺️diff` (facet-local `diff_fixture_from_helpers` composer).

**Two kernel bridges cannot be fixed leaf-side**: flow's dispatch
(`from_framework_mutation`/`to_framework_mutation`, `🧬️mutations/🦀️component.rs:15,60–110`)
and space-home's app bridge both bridge framework modules (`🔨️modules/🌊️flow/🌿️vcs`,
`🔨️modules/🪐️space`) that are themselves `CollectionMutation`-shaped. Serial framework step.

## App / glue funnel debt (migrated plugins)

Handlers constructing variants that no longer exist. Highlights:

- **writer**: `setSnapshot`/`setSnapshotJson` routes + 4 handlers (`🎛️apps/✒️writer/🦀️component.rs`
  :96,98,207,452,454,492,648; `🎮️commands/✍️text` :48–151).
- **animate**: `PresentMutation::Tiles(CollectionMutation::{Add,Remove,Patch})` ×7 across
  `🀄️tile`, `⌨️engagement`; `SetSnapshot` reset in `🖼️source` :66; app-local `NoMutation` command.
- **architect**: macro-generated catalog CRUD (`🗂️catalog/🦀️component.rs` :523,530,578,587,664),
  analysis/graph/element commands, `📤️exchange` import → `SetSnapshot` :48,105, tests :568,599.
- **process**: `setSnapshot` route + `📄️artifact` handlers, 8 collection sites in
  `🪜️step`/`🔎️inspector`/`🛠️workshop`, `🪵️stock`/`📤️media` snapshot loads, 12 wasm-bridge tests.
- **shooting**: `setSnapshotJson` route + `🗃️fixture` handlers, 11 collection sites in
  `📦️asset`/`📷️shot`/`🎥️camera`.
- **layout**: `LayoutMutation::{Pages,Stories,Links}(CollectionMutation::…)` in `✏️author`
  :133,156,209,219.
- **lowpoly**, **reasoning** (`🧬️example` :27), **flow** (app-local `FlowNodeGraphEditOp::SetSnapshot`),
  **vcs** (app-local `NoMutation` command struct — cosmetic).
- **gis**: comments only (already documents the ban). **demonstrator**, **mathematical**: clean.

**Beyond `🎮️commands`**: 141 `.rs` files under `✏️s` outside `🧬️mutations`/`🎮️commands` still hit
the banned tokens (app roots, modes/windows/panels, `⚙️engine`, `🌉️wasm`, artifact roots, all 15
norm apps, cad `🎚️config`), plus 69 `.ts` files. The current policy scan sees none of these.

Glue files still declaring `pub mod set_snapshot`/`no_mutation`: 19 plugins (stdio alone has 53
such blocks).

## stdio (53 facets, 0% migrated)

36 artifact families; multi-standard: gif(87a/89a), ifc(2x3/4), pdf(1.4/1.7), dwg(ac1018/ac1024);
`🧿️semio` has 14 subsets. **Every** facet still leads with `NoMutation` + `SetSnapshot`; **zero**
use the derive; 52 of 53 have exactly one triad dir (`📄set-snapshot`) — `💾️binary`/`📝️text` are
codec dirs, not triads.

- `🧿️semio ✳️image` is the exception: 12 extra triad dirs exist but are NOT mounted in stdio's
  glue (dead scaffolding) and their leaves are apply-and-capture — rewrite, don't trust.
- `🧿️semio ✳️any` is a 13-way union dispatch delegating to the sub-subset enums → migrate LAST.
- stdio's glue comment states per-variant triad dirs are optional scaffolding; a facet CAN be
  migrated with zero glue edits via inline `#[path = "."] pub mod <slug> { … }` — but per the
  finishing plan's design resolution 2, real dirs + real glue mounts are the end state, so
  sub-lane agents emit `sharedFileRequests` and the stdio funnel agent applies them.

Size buckets (variants excluding NoMutation/SetSnapshot): trivial 0–3 → pdf1.4(0), mp3, wav,
ifc2x3, dwg-ac1018, deflate, binary-raw, dwg-ac1024(4); small 4–8 → csv, md, txt, tsv, stl, bmp,
xml, tiff, pptx, semio-video, ply, html, xlsx, svg, semio-{model,audio,object}; medium 9–15 →
mp4, ifc4, step, gif87a, las, docx, epw, zip, bcf, pdf1.7, jpg, avi, semio-{animation,image,
workflow,presentation,mesh,cad,document}; large 16+ → png(15), dxf(18), gif89a(19),
semio-drawing(16), obj(20), semio-brep(21), gltf(22).

`🖊️dxf/…/🔺️diff` already has a local `named_apply()` helper — the only pre-existing
named-collection engine in stdio.

## Remaining non-stdio untouched facets

| facet | dispatch variants | triads |
|---|---|---|
| norm en1996/en1997/en1998/en1999/din18599 | `SetSnapshot` only | 1 each |
| block ◻2d | SetNodeKind SetPresentation SetHandleKind RemoveHandleKind SetHandle RemoveHandle SetCompatibilityRule RemoveCompatibilityRule SetAttribute RemoveAttribute SetAuthors SetCamera2d SetMeta SetSnapshot | 13 |
| block 🧊️3d | SetObjectKind SetRepresentation RemoveRepresentation SetVortexKind RemoveVortexKind SetVortex RemoveVortex SetCompatibilityRule RemoveCompatibilityRule SetAttribute RemoveAttribute SetAuthors SetCamera3d SetMeta SetSnapshot | 14 |
| block 🖐️5d | SetPartKind SetPart2d SetPart3d SetRepresentation RemoveRepresentation SetGripKind RemoveGripKind SetGrip RemoveGrip SetCompatibilityRule RemoveCompatibilityRule SetAttribute RemoveAttribute SetAuthors SetCamera2d SetCamera3d SetMeta SetSnapshot | 17 |
| puzzle ◻2d | SetNode RemoveNode SetEdge RemoveEdge SetMeta SetSnapshot | 5 |
| puzzle 🧊️3d | SetObject RemoveObject SetAttraction RemoveAttraction SetTargetVolume RemoveTargetVolume SetReference RemoveReference SetMeta SetSnapshot | 9 |
| puzzle 🖐️5d | SetPart RemovePart SetFastener RemoveFastener SetMeta SetSnapshot | 5 |
| note | SetGridVisible SetGridSpacing SetGridSubdivisions SetGridOpacity SetSnapEnabled SetSnapGridSpacing SetPencilWidth SetEraserRadius SetBlocks PutAsset RemoveAsset SetSnapshot | 12 |
| dag | Nodes Edges SetNodes SetEdges SetSnapshot (no dsl derive at all) | 5 |
| energy model | NoMutation SetSnapshot | 2 |
| raster | AddLayer RemoveLayer PatchLayer MoveLayer SetSnapshot | 5 |
| sourcing curate | SetSnapshot only | 1 |

**Norm remainder is strictly simpler than the migrated 9**: en1996(22 flat scalars), en1997(22),
en1998(~39), en1999(26), din18599(13 incl. one nested `MonthlyClimate` struct) — no collections
at all, so per-field `change-*` only (the `din16798`/`din4108` shape; `en1990` needed extra
insert/remove/reorder for its `q_k: Vec<…>`). All 5 also carry
`crate::impl_norm_set_snapshot_ops!(…)` macro calls to remove.

## The 6 odd facets

| facet | shape | migration |
|---|---|---|
| space-home | `#[derive(…Default…, dsl::DslEnum)] SHomeMutation { NoMutation, SetCatalogGeneration{value}, SetSnapshot{snapshot} }` + hand-written apply/inverse; 1 triad dir | trivial warm-up: drop both banned variants, `SetCatalogGeneration` → `change-catalog-generation`, adopt derive |
| trinity-jack | `TrinityGraphMutation` (no derive): CreateNode DeleteNode CreateEdge DeleteEdge Rename Reposition SetDataProperty ClearDataProperty SetFixture; hand-written dispatch fns; 10 semantic dirs | kill `SetFixture` (→ store.reset), `Rename`→`rename-node`, `Reposition`→`move-node`, `SetDataProperty`→`change-data-property`, adopt derive, dissolve hand dispatch |
| trinity-rewrite | `RewriteRuleMutation { SetState{state: RewriteSnapshot} }` — SetSnapshot in disguise | derive real vocabulary from `RewriteSnapshot` (before_fixture_json, lhs_json, rhs_json, parameter_bindings map, rule_layout map) → ~5 `change-*` + map upsert/remove |
| remodel | `#[derive(dsl::DslEnum)] RemodelMutation` — 20 `Set*` variants, 20 matching dirs whose leaves are apply-only | rename to `change-*`/`replace-*`, swap `DslEnum`→`Mutations`, rewrite 20 apply leaves into diff/inverse pairs |
| imperative | **struct** `ImperativeMutation { path_ref, collection: protocol::CollectionMutation<String, Step, Dictionary> }`; 1 dir `✂️step-collection` | biggest structural rewrite of the six: real enum with path-addressed create/delete/move/update-step |
| playbook | no enum — re-exports framework kernel `PlaybookMutation` (`🔨️modules/📖️playbook/🦀️component.rs:283`, already semantic: AddStep RemoveStep MoveStep AddBlock RemoveBlock MoveBlock UpdateBlock UpdateStep UpdatePlaybookTitle) with a bridging `impl Mutation`; 9 semantic dirs whose leaves are apply-and-capture shims | serial framework step (kernel enum gains `MutationKind` + derive), then plugin-side leaf rewrite |

## Policy state

`bun ./📜️script.ts policy` gates on **high priority only** (verified:
`🧰️framework/…/📚️library/📦️packages/🟦️typescript/📦️index.ts:735–745`).

| kind | counts |
|---|---|
| `mutation-migration/semantic-vocabulary` | 42 high (unallowlisted), 80 low (stale entries), 475 medium (bare `Set*`) |
| `mutation-migration/dispatch-coverage` | 105 medium |
| `mutation-migration/ts-mirror` | 1407 low |
| `mutation-migration/triad-completeness`, `…/artifact-engine` | 91 high each — **bogus**, wrong-depth bug |

Four rules share the wrong-depth bug (scan shallow `<artifact>/🧬️mutations`, which does not exist
in the real taxonomy): `policyMutationTriadCompletenessBreaches` (:5308),
`policyArtifactEnginePresenceBreaches` (:5391), `policyMutationImplPresenceBreaches` (:5361,
silently inert), `policyMutationEmojiUniquenessBreaches` (:5455, silently inert, **high**).
Correct deep helper `policyFindAllMutationsDirs` (:5500) already exists and is used by
dispatch-coverage (:5983). Fixing depth wholesale would ignite emoji-uniqueness on migrated
facets — wave-2 agents reused emojis freely (gis `🆕`×3 `🗑`×3 `🔁`×3 `🔀`×3; flow `🔀️`×2; layout `➕`×2).

`policyMutationTsMirrorBreaches` (:6012–6031) only flags EXISTING `export {};` stubs; migrated
triads have no `🟦️component.ts` at all and are invisible to it.

Vocabulary rule regex-tests raw content INCLUDING comments (:5901–5902) — several of the 42 highs
are doc-comment mentions in retired stubs whose dirs are still glue-mounted (delete only after
the glue rewire).

`POLICY_SEMANTIC_VOCABULARY_ALLOWLIST` (:5531–5874) is a flat `Set<string>` of repo-relative
paths, one literal per line — coordinator-batched edits are safe.

Never implemented from the original plan: rule 5 (grammar coverage), rule 6 (DiffAlgebra scope
expansion beyond stdio), rule 7 (artifact-engine rule fix).

## Config / draft mutations (ratchet scope)

39 hand-written `enum *ConfigMutation` under `🎛️apps/**/🎚️config` — outside the 107-facet census
but inside the original ratchet's `ConfigMutation`/`DraftMutation: SemanticMutation` bounds. Some
carry whole-config `Snapshot { config }` variants (e.g. `🌿️vcs/🎛️apps/🌿️vcs/🎚️config/🦀️component.rs`
:104–114). Framework `NoConfigMutation` (`🔌️plugin/🦀️component.rs:3548`, `Noop` variant) and its
`NoDraftMutation` alias (:3608) implement only `Mutation` — 54 apps use `type DraftMutation =
NoDraftMutation;`. `ArtifactApp` bound points: `🔌️plugin/🦀️component.rs:4476–4488`. Plugin-module
test fixtures `DummyMutation`/`TestMutation` (:2679, :7765) also need migrating for the plain
`Mutation` bound.

`MutationMeta.semantic_kind`/`label` exist and are constructed `None` at both sites
(`📡️spr/🎮️command/🦀️component.rs:406,410,781–782,804–805`); store is generic
`ArtifactStore<P, Mutation>` (`🏪️store/🦀️component.rs:2201`), so bounds are addable.

## Concurrent churn (NOT this ticket's)

Framework os/host (`LocalizedLabel`, `AppDefinition.document`), `🛢️db` module refactor, playbook
JSON codecs, missing `📌️panels/📄️document` files (imperative, reasoning, forms, sourcing).
Gates must be per-crate. Churn policy: retry ×3 spaced, then mark `blocked-churn` and requeue;
never fix another session's files; never `cargo update`.
