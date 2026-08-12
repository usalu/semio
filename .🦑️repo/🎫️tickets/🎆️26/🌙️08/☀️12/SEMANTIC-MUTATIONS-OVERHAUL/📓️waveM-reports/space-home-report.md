# FacetReport — `🪐️space` / `🏠️home`

## facet
`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`

## status
**done** (Rust logic + wiring complete; gate commands executed by me were repeatedly blocked by
foreign framework churn unrelated to this facet — see `gates` below. Coordinator confirmed this
facet done from a working-tree check independent of my own gate attempts.)

## mutationsCreated
| slug | verb | entity | superseded |
|---|---|---|---|
| `change-catalog-generation` | `change` | `catalog-generation` | `SetCatalogGeneration` (renamed 1:1, same field) |

## genericVariantsRemoved
- `NoMutation` — dropped outright (no replacement; `MutationKind::inverse` now returns `Vec::new()` where nothing needs undoing — not applicable here since the sole variant is always meaningful).
- `SetSnapshot { snapshot: SHomeSnapshot }` — dropped outright, no replacement. No app command in this plugin ever constructed it (confirmed by grep before deleting); whole-doc replace was never wired for `s.home`, so there was nothing to reroute to `ArtifactStore::reset`/`HostEffect::LoadDocument`.

## filesTouched

**Created** (new `🔢️change-catalog-generation/` triad, 6 files):
- `🧬️mutations/🔢️change-catalog-generation/🦠️mutation/🦀️component.rs`
- `🧬️mutations/🔢️change-catalog-generation/🦠️mutation/🟦️component.ts`
- `🧬️mutations/🔢️change-catalog-generation/🔺️diff/🦀️component.rs`
- `🧬️mutations/🔢️change-catalog-generation/🔺️diff/🟦️component.ts`
- `🧬️mutations/🔢️change-catalog-generation/↩️inverse/🦀️component.rs`
- `🧬️mutations/🔢️change-catalog-generation/↩️inverse/🟦️component.ts`

**Removed**:
- `🧬️mutations/🎛set-catalog-generation/` (old facade triad, 6 files: mutation/diff/inverse × .rs+.ts)
- `🧬️mutations/📖️component.grammar.semio` (dead top-level grammar, unregistered — real one lives under `📝️text/`)

**Updated**:
- `🧬️mutations/🦀️component.rs` — dispatch enum shrunk to `pub enum SHomeMutation { ChangeCatalogGeneration(ChangeCatalogGeneration) }`, `#[derive(dsl::DslEnum, dsl::Mutations)]`, hand-written `apply_shome_mutation`/`inverse_shome_mutation`/`impl Mutation` deleted; added `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` tests.
- `🧬️mutations/📝️text/🦀️component.rs` — dropped `apply_shome_mutation, inverse_shome_mutation` from the `pub use` re-export list (deleted, no longer exist).
- `🧬️mutations/📝️text/📖️component.grammar.semio` — rewritten from generic `stdio.json` boilerplate to real `change-catalog-generation` grammar (this is the one actually registered via `dsl::register_language`).
- `🧬️mutations/💾️binary/🦀️component.rs` — tests updated to build via `change_catalog_generation(...)`.
- `📦️glue.rs` — replaced the `set_catalog_generation` mod block with `change_catalog_generation`, pointing at the new triad paths.
- `🎛️apps/🏠️home/🎮️commands/🏙️studio/🦀️component.rs` — 2 call sites (`created_studio_emit`, `import_space::handle`) rerouted `SHomeMutation::SetCatalogGeneration{..}` → `change_catalog_generation(..)`.
- `🎛️apps/🏠️home/🎮️commands/🗂️vfs/🦀️component.rs` — 1 call site (`delete_virtual_file_system_node::handle`) rerouted the same way.
- `🏅️standards/🔖️1/⚙️engine/🦀️component.rs` — test updated to build via `change_catalog_generation(5)`.
- `🧬️schema/🔺️diff/🦀️component.rs` — removed `SHomeDiff.artifact: Option<Box<SHomeArtifact>>` (the whole-doc-replace field that only `SetSnapshot` ever populated).
- `🧬️schema/🔺️diff/📝️text/🦀️component.rs` — removed the `artifact`-replacement branches from `apply_to_artifact`/`MutationDiff::apply`/`absorb`, and deleted the now-orphaned `diff_set_snapshot` helper.
- `🧬️schema/🔺️diff/🔗️component.graphql`, `🔣️component.json`, `🛰️component.proto`, `🟦️component.ts` — removed the `artifact`/`SHomeArtifact` field to match the Rust struct (JSON/proto trimmed the field; TS mirror trimmed the field + its now-unused import).
- `🧬️schema/📸️snapshot/📝️text/🦀️component.rs` — no change needed (never referenced `SetSnapshot`).

## sharedFileRequests
None. The framework `🪐️space` app (`WorkflowSnapshot`/`WorkflowMutation`) flagged as a possible boundary in the brief turned out not to intersect `SHomeMutation` at all — it's a fully separate `ArtifactApp` with no `🗿️artifacts` node of its own in this crate, so I never touched it.

## allowlistKeysToRemove
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (no more `NoMutation`/`SetSnapshot`)
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` (no more whole-artifact `artifact` field)
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🎮️commands/🏙️studio/🦀️component.rs`, `…/🗂️vfs/🦀️component.rs` (no more `SetCatalogGeneration` struct-literal construction)

## gates
- `cargo check -p semio-s-plugin-space` — **NOT observed as a clean pass by me.** I ran it 4 times across the session; every attempt failed before reaching this crate, blocked by unrelated concurrent churn in `semio-framework-plugin` (`missing field member_edits in initializer of UndoGroup`) and later `semio-framework-os` host (`missing fields dialect and migrated_from in initializer of ArtifactEnvelope`, `LocalizedLabel`/`Vec<String>` mismatches in `🖥️host/🦀️component.rs`) — verbatim errors captured in `.txt` scratch files in this ticket folder (`waveM-space-home-cargo-check-*.txt`). Per the coordinator's later instruction I stopped running cargo entirely and am deferring this gate to the coordinator's consolidated pass.
- `cargo test -p semio-s-plugin-space --lib` — not run, same reason.
- `bun ./📜️script.ts policy` — **run once, output reviewed** (`waveM-space-home-policy.txt`). 23,942 pre-existing high-priority breaches repo-wide; none of the new categories are attributable to this change — `mutation-migration/triad-completeness`, `artifact-schema/facet-completeness` etc. fire on `🏠️home` due to a pre-existing checker limitation with the `🏅️standards/🪆️subsets` nesting that affects nearly every facet in the repo identically (91/273 instances respectively, same pattern before and after my edit). I did find and fix one real self-inflicted breach: `taxonomy/emoji-prefix` on my own new `🔢change-catalog-generation` dir (missing U+FE0F) — corrected to `🔢️change-catalog-generation`.

## lawTests
- `assert_op_line_round_trip` — 1 (the sole variant).
- `dispatch_registers_semantic_descriptors` — verifies all kinds' verbs are in `APPROVED_VERBS` and `kinds().len() == 1`.
- `assert_mutation_inverse_law` — 1 (`change_catalog_generation`).
- `assert_mutation_diff_absorb_law` — 1 (`change_catalog_generation`, two sequential changes).
- None of these were executed by me (no `cargo test` run) — added per the brief's Step 7 but unverified pending the coordinator's consolidated pass.

## deviations
- Left the top-level `🧬️mutations/🔗️component.graphql`/`🔣️component.json`/`🛰️component.proto`/`🟦️component.ts` untouched — they already correctly described `{schema, catalogGeneration}` as the mutable field set (matches `sequence`'s established convention of listing the full persistent field set here, not per-variant).
- Left `🧬️mutations/💾️binary/📡️component.protocol.semio` (+`.abnf`/`.ksy`/`.spicy`) as generic envelope framing — confirmed this matches `sequence`'s own (reference, compiling) facet exactly; the framing is genuinely generic infrastructure, not a per-variant leftover.
- TS triad mirrors are real (non-stub) minimal interfaces/functions, deliberately self-contained (no cross-taxonomy-path imports) to avoid unicode-path import fragility; not type-checked by me (`bunx tsc` not run).
