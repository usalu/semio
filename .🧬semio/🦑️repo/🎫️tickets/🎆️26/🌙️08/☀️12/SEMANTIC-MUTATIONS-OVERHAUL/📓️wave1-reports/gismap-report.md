# Wave 1 — `gismap` facet report

Facet: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-gis`

## Finding: facet was already fully migrated

On inspection, this facet's dispatch enum, all twelve triad leaves, every emit call site inside the
package, and the facet's law tests already match the target shape described in
`📓️derivation-rules.md` and `📓️taxonomy.md`. No generic vocabulary (`SetSnapshot`, `NoMutation`,
`CollectionMutation<` in a public enum, bare whole-object `Set*`) remains anywhere reachable from
`GisMapMutation`. Since the working tree is shared and concurrently edited, this was presumably
completed by another session before this task started; I verified rather than re-did the work, and
made no code changes.

### Vocabulary present (matches the derivation recipe for 3 sibling id-keyed `Vec<MapFeature>`
collections — `positions`/`routes`/`regions`, each with only `id` + opaque `data`, no separate
scalar fields, no name/key, no display order beyond position):

- `create-position` / `delete-position` / `replace-position-data` / `reorder-positions`
- `create-route` / `delete-route` / `replace-route-data` / `reorder-routes`
- `create-region` / `delete-region` / `replace-region-data` / `reorder-regions`

`GisMapSnapshot` (`🧬️schema/📸️snapshot/🦀️component.rs`) has no document-level scalars, no
relationship/edge collection, and no hierarchy field, so no `rename-gismap`/`change-*`/
`connect`/`move-to-*` verbs are needed — the 12-mutation vocabulary above is the complete, closed
set for this snapshot shape.

### Shape verified against the `MiniMutation` reference fixture

- `🧬️mutations/🦀️component.rs`: dispatch enum `GisMapMutation` derives
  `dsl::Mutations`(`= dsl_derive::Mutations` re-export) with
  `#[mutations(snapshot = GisMapSnapshot, diff = GisMapDiff, schema = "gis.gismap")]`; every variant
  is a single-field tuple wrapping a `🦠️mutation` leaf payload. No hand-written
  `apply`/`diff`/`inverse` dispatch remains (`apply_gis_map_mutation`/`inverse_gis_map_mutation` are
  thin passthroughs to `vcs::apply_mutation`/`operation.inverse`, not hand-rolled match dispatch).
- Each `🦠️mutation/🦀️component.rs` payload implements `protocol::MutationKind<GisMapSnapshot,
  GisMapMutation>` with a real `const SEMANTICS: SemanticDescriptor`, and `diff`/`inverse` DELEGATE
  to the sibling `🔺️diff`/`↩️inverse` leaf functions (checked `create-position`, `delete-position`,
  `delete-route`, `replace-region-data`, `reorder-positions` directly; the remaining 7 follow the
  same generated-looking pattern).
- `🔺️diff` leaves build `GisMapDiff` sparsely via the shared internal
  `CollectionMutation`/`features_delta_from_collection_mutation` engine (documented in wave0 as the
  sanctioned INTERNAL use — never surfaces in the public enum) — never apply-then-capture, never a
  snapshot clone.
- `↩️inverse` leaves reconstruct from `base` (pre-state): `create-*` inverses to `delete-*` using the
  id captured in the payload itself; `delete-*` inverses reconstruct a `create-*` from the removed
  base item (`Vec::new()` if the id is already absent); `replace-*-data` inverses to a `replace-*-data`
  carrying the old `data` from `base` (empty if the id is absent); `reorder-*` inverses to a
  `reorder-*` back to the prior index (empty if the id is absent).

### Emit call sites (inside package)

`⚙️engine/🦀️component.rs` (feature-collection diffing → `CreatePosition`/`DeletePosition`/
`ReplacePositionData` and the route/region equivalents) and
`🎛️apps/◻2d/🎮️commands/🗺️features/🦀️component.rs` (`ReplaceRouteData`) already construct only the
semantic variants. Grepped every `GisMapMutation::` construction site in the package — all resolve
to one of the 12 semantic variants; none construct a generic/banned shape.

### Law tests already present in the facet's existing `#[cfg(test)] mod tests` (no new test files
added — none needed)

`🧬️mutations/🦀️component.rs`'s existing test module already calls
`protocol::testkit::assert_mutation_inverse_law` for `create-position`, `delete-route`, and
`reorder-routes`, and `protocol::testkit::assert_mutation_diff_absorb_law` for `create-position` and
`replace-region-data`, plus hand-written round-trip/inverse-of-missing-id coverage for every
collection. This satisfies recipe step (f)'s "two or three most important new variants" bar.

### Step (g) — grammar/binary protocol (not blocking, left as-is)

`🧬️mutations/📝️text/📖️component.grammar.semio` and
`🧬️mutations/💾️binary/📡️component.protocol.semio` are a generic `stdio.json` header +
`payload = OCTET+` pass-through, matching the sibling `🏔️gisterrain` facet's identical shape
(explicitly documented in `🧬️mutations/💾️binary/🦀️component.rs`'s file docstring: "matching
`🏔️gisterrain`'s sibling facet's identical shape"). Per-mutation-record listing was not added since
that would diverge from the established sibling-facet convention without a clear win; flagging this
as a follow-up decision for whoever runs the grammar/protocol-listing pass across all facets
consistently, rather than doing it ad hoc here.

## Files touched

None — this report file only. No source edits were needed.

## Verification

- `cargo check -p semio-s-plugin-gis` — clean (only pre-existing warnings across the crate,
  unrelated to `gismap`'s mutations facet).
- `cargo test -p semio-s-plugin-gis --lib` — 155 passed; 0 failed; 0 ignored.
