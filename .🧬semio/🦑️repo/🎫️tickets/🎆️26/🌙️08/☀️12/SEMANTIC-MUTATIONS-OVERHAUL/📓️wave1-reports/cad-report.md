# Wave 1 — CAD facet report

Facet: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-cad`

## What changed

Deleted the old 14-variant `CadMutation` (struct-variant `Set*`/`Patch*`/`Add*`/`Remove*` generics,
including the banned `SetSnapshot` whole-document-replace variant) and replaced it with a 24-variant
closed semantic vocabulary, derived from `CadSnapshot`'s shape per `📓️derivation-rules.md`. Every
variant is now a single-field tuple wrapping a `🧬️mutations/<kind>/🦠️mutation` payload struct that
implements `protocol::MutationKind<CadSnapshot, CadMutation>`, with real handcrafted `🔺️diff`
(sparse `CadDiff` construction) and `↩️inverse` (reconstructed from `base`, never from post-state)
leaves. The dispatch enum itself now derives `dsl::Mutations` (`#[mutations(snapshot = CadSnapshot,
diff = CadDiff, schema = "cad.cad")]`), which generates `impl protocol::Mutation` and `impl
protocol::SemanticMutation` — the old hand-written `impl Mutation<CadSnapshot> for CadMutation`
match-dispatch block is gone.

### New vocabulary (24 mutations, from 14 old generic variants)

| New semantic mutation | Verb | Replaces |
|---|---|---|
| `create-object` | create | `AddObject` |
| `delete-object` | delete | `RemoveObject` |
| `rename-object` | rename | `PatchObject{label}` |
| `change-object-typology` | change | `PatchObject{typology}` |
| `change-object-visible` | change | `PatchObject{visible}` |
| `change-object-locked` | change | `PatchObject{locked}` |
| `move-object` | move | `PatchObject{origin}` |
| `rotate-object` | rotate | `PatchObject{orientation}` |
| `scale-object` | scale | `PatchObject{scale}` |
| `replace-object-geometry` | replace | `PatchObject{mesh_url,extent,solid_handle}` (bundled — no live gesture ever set these fields independently) |
| `drag-objects` | drag | `TranslateObjects` (bulk relative offset) |
| `rotate-objects` | rotate | `RotateObjects` (bulk relative compose) |
| `scale-objects` | scale | `ScaleObjects` (bulk relative compose) |
| `replace-pane-objects` | replace | `SetPaneObjects` |
| `create-node` | create | `AddNode` |
| `delete-node` | delete | `RemoveNode` |
| `rename-node` | rename | `RenameNode` (verb unchanged, payload shape changed) |
| `change-reference-hidden` | change | `PatchReference{hidden}` |
| `change-reference-locked` | change | `PatchReference{locked}` |
| `change-reference-width` | change | `PatchReference{width_world}` |
| `move-reference` | move | `PatchReference{origin}` |
| `replace-reference-media` | replace | `PatchReference{source_url,media_kind,orientation,scale,opacity}` (bundled — no live gesture ever set these independently) |
| `replace-references` | replace | `SetReferences` |
| `change-active-model-definition` | change | `SetActiveModelDefinition` |
| — (none) | — | `SetSnapshot` **deleted, no replacement** — whole-document replace is banned from the mutation enum per taxonomy |

`CadObjectPatch`/`CadNodePatch`/`CadReferencePatch` survive as **internal diff-construction glue
only** (moved into the dispatch `🦀️component.rs`'s `🔖️InternalPatches` region) — never again a
mutation's own payload, matching the taxonomy's "option-bag `Patch` structs may survive as
diff-fragment helpers" carve-out.

## SetSnapshot removal — mechanism

`CadPlayApp::whole_document_operation` now falls back to the framework trait's default (`None`) —
CAD's own override that used to build `SetSnapshot` is deleted. The three real call sites that used
to emit `SetSnapshot` (import_media's `document:in` port, `import-cad-file`'s spatial-JSON-payload
branch, `set-active-example`'s load-example branch) now build a new shared helper,
`apps::cad::reset_document_effect(&CadSnapshot) -> HostEffect`, which returns
`HostEffect::LoadDocument { pack, spr }` — the framework's existing sanctioned non-history
whole-document-swap effect (`store::create_document_envelope` + `store::print_document_spr` over a
fresh, edit-free envelope, `pack` via `ArtifactPack::encode_pack`). This is a `HostEffect` in
`Emit.effects`, not an `artifact_mutations` entry, so it never enters undo history — matching
taxonomy's "goes through `ArtifactStore::reset`, entirely outside the `Mutation` enum" ruling.

## Wire format

`CadMutation` keeps `#[derive(..., dsl::DslEnum, dsl::Mutations)]` (both derives on the same enum —
`dsl::DslEnum`'s codegen already special-cases single-field tuple variants, delegating the whole
`RecordSpec`/value to the inner payload's own `DslField` impl, so the OpText/OpBinary/grammar
mechanics in `📝️text/🦀️component.rs` needed **zero logic changes**). Every payload struct got an
explicit `#[dsl(keyword = "<kind>")]` (the derive's default keyword is `None`, not the struct's own
kebab name, which silently drops the leading grammar keyword from `print_op` — caught by the
`cad_mutation_print_op_round_trips_every_variant_as_one_line` test failing, then fixed).

## Files changed

**Deleted** (14 old triad dirs): `➕️add-object`, `➖️remove-object`, `🩹patch-object`,
`↕️translate-objects`, `🔄rotate-objects` (recreated), `↔️scale-objects` (recreated),
`🖼️set-pane-objects`, `🖼️set-snapshot`, `➕️add-node`, `➖️remove-node`, `🏷️rename-node` (recreated),
`📎set-references`, `🎯set-active-model-definition`, `🩹patch-reference`.

**Created** (24 new triad dirs, each with `🦠️mutation`/`🔺️diff`/`↩️inverse` `🦀️component.rs` + `.ts`
stub facades): `➕create-object`, `🗑delete-object`, `🏷rename-object`, `🏗change-object-typology`,
`👁change-object-visible`, `🔒change-object-locked`, `📍move-object`, `🔃rotate-object`,
`📏scale-object`, `🧊replace-object-geometry`, `🫳drag-objects`, `🔄rotate-objects`,
`↔scale-objects`, `🖼replace-pane-objects`, `➕create-node`, `🗑delete-node`, `🏷rename-node`,
`👁change-reference-hidden`, `🔒change-reference-locked`, `📏change-reference-width`,
`📍move-reference`, `🖇replace-reference-media`, `📎replace-references`,
`🎯change-active-model-definition`.

**Rewritten**:
- `🧬️mutations/🦀️component.rs` — dispatch enum, internal patch types, shared helpers
  (`set_pane_objects_delta`, `transform_objects_diff`, `quat_mul`, `quat_from_axis_angle`), tests
  (`every_mutation`, inverse-restores-base law over all 24 variants, semantic-descriptor
  registration law, plus 3 new `protocol::testkit::assert_mutation_inverse_law` /
  `assert_mutation_diff_absorb_law` law tests: `rename-object`, `drag-objects`,
  `change-reference-hidden`).
- `🧬️mutations/📝️text/🦀️component.rs` — dropped the pinned pre-migration byte-fixture test (wire
  format legitimately changed, greenfield); kept the generic round-trip test unchanged.
- `🧬️mutations/💾️binary/🦀️component.rs` — store-level tests updated to new variant constructors;
  dropped the `SetSnapshot`-through-store test (no replacement — the mutation no longer exists).
- `🔺️diff/📝️text/🦀️component.rs` — `whole_artifact_diff_...` test now builds the `CadDiff.artifact`
  fragment directly (that `CadDiff` field itself is untouched; only its former mutation source is
  gone); `object_collection_diffs_absorb_into_one_apply` updated to new constructors.
- `📸️snapshot/🦀️component.rs` — removed the now-dead `impl dsl::DslField for Box<CadSnapshot>`
  (existed solely for `SetSnapshot`'s payload).
- `📦️packages/🦀️rust/📦️glue.rs` — replaced the 14-module mutations sub-tree with the 24-module one.
- `🎛️apps/📐️cad/🦀️component.rs` — added `reset_document_effect`; rewrote `patch_objects_mutations`/
  added `object_field_mutation` to build semantic mutations instead of `CadObjectPatch`;
  `whole_document_operation` override deleted; `import_media`'s `document:in` branch now builds
  `reset_document_effect` directly; fixed the `import_cad_file_action_accepts_spatial_json_...`
  test to assert on `HostEffect::LoadDocument` instead of a mutation.
- `🎛️apps/📐️cad/🎚️config/🦀️component.rs` — one docstring comment updated (no code change).
- `🎛️apps/📐️cad/🎮️commands/{🕸️node,🧱️object,📥️io,🖼️reference,🗺️model-definition,🔄️transform}/🦀️component.rs`
  — every construction site updated to the new tuple-variant mutations; `patch-cad-play-reference`
  and `patch-object`/`patch-selection` commands rewritten to build semantic mutations per field
  instead of a `CadReferencePatch`/`CadObjectPatch`.

## Not done (deferred, non-blocking per the recipe's step g)

`📖️component.grammar.semio`, `💾️binary/📡️component.protocol.semio`,
`🔺️diff/💾️binary/📡️component.protocol.semio`, `🔣️component.json`, `🛰️component.proto`,
`🔗️component.graphql`, and the `.ts` facade stubs still describe the OLD vocabulary — they are
`include_str!`'d as opaque documentation constants (never parsed/type-checked by Rust), so this is
stale documentation, not a compile or runtime bug. Flagged for a follow-up pass; left untouched
here given the scope of the Rust migration itself.

## Verification

- `cargo check -p semio-s-plugin-cad` — clean (only pre-existing warnings in files this ticket
  didn't touch: `composer` unused import, an elided lifetime, an unused doc comment on a macro-
  generated impl, a dead field in `CadEngine` — none introduced by this change).
- `cargo test -p semio-s-plugin-cad --lib` — **132/132 pass**, up from the pre-change 129 (3 new law
  tests added; 2 tests initially broke from the rewrite and were fixed: the missing
  `#[dsl(keyword=...)]` wire-format regression, and the `SetSnapshot`-based import test rewritten to
  assert on `HostEffect::LoadDocument`).

## Files touched (created/updated/removed)

See "Files changed" above for the full list; all paths are under
`✏️s/🔌️plugins/📐️cad/{🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/**,
🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/{🔺️diff/📝️text,📸️snapshot}/🦀️component.rs,
📦️packages/🦀️rust/📦️glue.rs, 🎛️apps/📐️cad/**}`.
