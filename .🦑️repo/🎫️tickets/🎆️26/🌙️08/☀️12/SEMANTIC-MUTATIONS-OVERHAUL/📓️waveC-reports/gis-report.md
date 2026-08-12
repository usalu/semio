# FacetReport — gis (Wave C)

## facet
`gis` plugin — two artifacts: `🗺️gismap` (12 mutations, `gis.gismap`) and `🏔️gisterrain` (2
mutations). Wave-R (`r2a-gis-shooting-report.md`) had already rewritten all 15 triad leaves (12
gismap `🔺️diff` + 3 `↩️inverse`) to drop `protocol::CollectionMutation`; gis compiled clean and
passed 170/170 lib tests before this session started. This session's job was the remaining funnel/
directory/glue/config debt the fanout brief flagged.

## status
`done`. `cargo check -p semio-s-plugin-gis` confirmed clean (0 errors). `cargo test -p
semio-s-plugin-gis --lib` CONFIRMED **171 passed; 0 failed** (up from Wave-R's baseline 170 — this
session's 2 new config tests plus the removed `Snapshot`-based ones net +1). Two earlier attempts
this session were blocked by an unrelated concurrent `semio-framework-os-kernel` compile break
(ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`, confirmed mid-edit — see **gates**); the
third attempt, after that window closed, ran clean.

## mutationsCreated
None — gismap's 12 and gisterrain's 2 semantic variants were already real (Wave 1/2 + Wave-R).
This session did directory/glue/config/schema-file/emoji trueing only, no new mutation vocabulary.

## genericVariantsRemoved
None left in the artifact mutation enums (already clean). Two whole-config `Snapshot { config }`
sentinels removed from the app-level config ratchet scope (outside the 107-facet census, but in
this ticket's step-3 remit): `Gis2dConfigMutation::Snapshot` and `Gis3dConfigMutation::Snapshot`.

## emoji table (facet-scoped uniqueness, per artifact)
**gismap** (12 triads) — pre-existing quadruple collision (`🆕`×3, `🗑`×3, `🔁`×3, `🔀`×3, one
emoji kept per verb-group, the other two reassigned):

| slug | before | after |
|---|---|---|
| `create-position` | `🆕` | `🆕` (kept) |
| `create-route` | `🆕` | `🛣️` |
| `create-region` | `🆕` | `🌐` |
| `delete-position` | `🗑` | `🗑` (kept) |
| `delete-route` | `🗑` | `✂️` |
| `delete-region` | `🗑` | `🧹` |
| `replace-position-data` | `🔁` | `🔁` (kept) |
| `replace-route-data` | `🔁` | `♻️` |
| `replace-region-data` | `🔁` | `🔄` |
| `reorder-positions` | `🔀` | `🔀` (kept) |
| `reorder-routes` | `🔀` | `🧭` |
| `reorder-regions` | `🔀` | `🔃` |

**gisterrain** (2 triads) — `🎚change-exaggeration`, `📥change-imported-features`: already unique,
untouched.

## Directory + glue trueing
- Renamed the 8 duplicate-emoji gismap triad dirs (table above) — `mv` on disk (no logic files
  touched, only the leading-emoji byte(s) of the dir name; `SemanticDescriptor.kind` is unaffected
  since it's compared against the dir stem with emoji stripped).
- Updated `📦️glue.rs`'s 8 corresponding `#[path]` mount blocks (`create_route`, `create_region`,
  `delete_route`, `delete_region`, `replace_route_data`, `replace_region_data`, `reorder_routes`,
  `reorder_regions` — 3 `#[path]` lines each, mutation/diff/inverse) to point at the renamed dirs.
  gismap's dispatch-variant-to-triad-dir mapping was already real (glue.rs already had per-triad
  mounts, not inline self-wiring — unlike layout/mathematical, gismap needed no `LeafWiring`
  removal, just the rename follow-through).
- Verified `cargo check -p semio-s-plugin-gis` is 0 errors after the rename (confirms the paths
  resolve and nothing else referenced the old dir names — grepped the whole plugin for each old
  emoji+slug combination, zero hits).

## Config semanticization
Both `Gis2dConfigMutation` (`🎛️apps/◻2d/🎚️config`) and `Gis3dConfigMutation`
(`🎛️apps/🧊️3d/🎚️config`) carried a whole-config `Snapshot { config }` variant every other
variant's `backwards()` returned. Removed both; `diff()`/`inverse()` rewritten per-variant.
`Gis3dConfigMutation` (3 scalar fields, no maps) was direct. `Gis2dConfigMutation` has two
`BTreeMap<String, V>` fields (`layer_visibility`, `layer_stroke_scale`) where a missing key already
meant "default" (per the pre-existing `layer_visible()` helper) — a naive per-field inverse that
just re-inserts the OLD value would fail to restore an ABSENT key exactly (inserting the default
value explicitly is not the same map state as no entry at all), so `diff()` for
`SetLayerVisibility`/`SetLayerStrokeScale` was changed to REMOVE the map entry when the written
value equals the field's default (`true` / `1.0`) rather than storing it explicitly — this makes
"missing" and "explicitly default" the same map state, which is what makes the inverse (re-emit
the same variant with the old value read from `base`, defaulted via `unwrap_or`) byte-exact even
when the pre-operation map had no entry for that key. Added a dedicated test
(`gis2d_config_layer_stroke_scale_backwards_restores_an_absent_entry`) proving this.

## TS mirrors
Added 42 non-stub `🟦️component.ts` files: 36 for gismap (12 triads × 3 leaves — the brief's "gis's
12 migrated triads have none at all" was accurate, now fixed) + 6 for gisterrain (2 triads × 3
leaves). Same shape as layout's (real per-field `interface`, `Kind` const, `declare function` for
diff/inverse).

## Schema description files
Rewrote both artifacts' `📝️text/📖️component.grammar.semio` (+ `💾️binary/📡️component.protocol.semio`,
`📝️text/{🔗️component.graphql,🔣️component.json,🛰️component.proto}`) from the stale
`"schema" SP "stdio.json"` placeholder to one grammar alternative / one `record … tag N` / one
input-type-or-message per real mutation slug. gismap: 12 records tagged 1..12 in dispatch-enum
order (create/delete/replace/reorder × position/route/region). gisterrain: 2 records.

Both artifacts' dispatch-level `🧬️mutations/🦀️component.rs` `include_str!`s its OWN root-level
`📖️component.grammar.semio` (a SEPARATE file from the `📝️text/` one, used by `⚙️engine`'s
inspector via a `dsl`/`op`/`diff` module shim) — for gismap this root copy held a completely
unrelated legacy DSL (`gis.gismap.op` with `set-crs`/`add-point`/`tile-ref` — an old coordinate/
tile-ref grammar, not the CRUD mutation vocabulary at all) and confirmed **dead** (grepped the
whole plugin for `mutations::COMPONENT_GRAMMAR`, zero hits — `⚙️engine` only reads `op::…`, which
the glue.rs shim maps to `mutations::text::…`, never the dispatch file's own top-level consts).
Overwrote both artifacts' root copies with the same real grammar text as their `📝️text/` sibling
for honesty/consistency, since leaving unrelated dead content there is worse than a duplicate.
Did not rewrite `.g4`/`.ebnf`/`.abnf`/`.ksy`/`.spicy` siblings — same reasoning as layout's report
(no working reference exists, rule never gated by policy).

## lawTests
No changes needed — gis's existing test region already covers all 4 verb kinds
(create/delete/replace/reorder) across position/route/region with explicit
`protocol::testkit::assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` calls, plus
full round-trip coverage for all 14 kinds (12 gismap + 2 gisterrain). `DiffAlgebra` is not
implemented for `GisMapDiff`/`GisTerrainDiff` — same complexity/risk tradeoff as layout, flagged
as remaining work, not attempted.

## gates
- `cargo check -p semio-s-plugin-gis`: **0 errors**, confirmed after the directory rename + glue.rs
  fix (~21 pre-existing warnings, unrelated to this session, none in touched files).
- `cargo test -p semio-s-plugin-gis --lib`: **CONFIRMED — `test result: ok. 171 passed; 0 failed;
  0 ignored; 0 measured; 0 filtered out`** (up from Wave-R's 170; this session's config tests net
  +1). Two earlier attempts this session were blocked by `semio-framework-os-kernel` failing to
  compile (`E0753`/`E0063`/`E0308` in `🏪️store/🦀️component.rs`, confirmed the SAME live edit
  window that blocked layout/mathematical — the exact broken line/error set differed between the
  two attempts, proving another session was actively writing code there); the third attempt, after
  that window closed, ran clean with zero failures.
- `bun ./📜️script.ts policy`: ran once, repo-wide (`22158` high-priority breaches total — expected
  for a live, multi-session, 107-facet migration in progress). Zero
  `mutation-migration/semantic-vocabulary`, `…/dispatch-coverage`, or `…/ts-mirror` breaches
  reference `🌍️gis`. The only hits mentioning gis facets are `…/triad-completeness` and
  `…/artifact-engine`, both pre-documented in `📓️remaining-work-map.md` as "bogus, wrong-depth bug"
  and unrelated to this session's changes.

## allowlistKeysToRemove
Full-plugin sweep (`grep -rnE "SetSnapshot|NoMutation|CollectionMutation(<|::)" ✏️s/🔌️plugins/🌍️gis`)
returns zero hits. Two comment-only pre-existing hits this session reworded (banned-token prose,
zero behavior change):
```
✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🦀️component.rs
✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎮️commands/🎨️example/🦀️component.rs
```
(Both already did the RIGHT thing behaviorally — batched targeted create/delete/replace-data
operations with real inverses, never a snapshot swap — the doc comments just still named the
banned token while explaining what the code does NOT do; reworded to describe the same fact
without the literal string.)

## filesTouched
**Updated:**
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎮️commands/🎨️example/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎚️config/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🎚️config/🦀️component.rs`
- gismap schema files: `…/🗺️gismap/…/🧬️mutations/📖️component.grammar.semio` (root),
  `…/🧬️mutations/📝️text/📖️component.grammar.semio`,
  `…/🧬️mutations/💾️binary/📡️component.protocol.semio`, `…/🧬️mutations/📝️text/🔗️component.graphql`,
  `…/🧬️mutations/📝️text/🔣️component.json`, `…/🧬️mutations/📝️text/🛰️component.proto`
- gisterrain schema files: same 6-file set under `…/🏔️gisterrain/…/🧬️mutations/…`

**Created:** 42 `🟦️component.ts` files (36 gismap + 6 gisterrain, one per triad leaf).

**Renamed (directory, 3 files each moved):** 8 gismap triad dirs — `🆕create-route` →
`🛣️create-route`, `🆕create-region` → `🌐create-region`, `🗑delete-route` → `✂️delete-route`,
`🗑delete-region` → `🧹delete-region`, `🔁replace-route-data` → `♻️replace-route-data`,
`🔁replace-region-data` → `🔄replace-region-data`, `🔀reorder-routes` → `🧭reorder-routes`,
`🔀reorder-regions` → `🔃reorder-regions`.

## sharedFileRequests
None — gis's `📦️glue.rs` was in-scope for this Wave-C lane.

## deviations
- `DiffAlgebra` not implemented for either diff type — flagged, not attempted (same reasoning as
  layout).
- `.g4`/`.ebnf`/`.abnf`/`.ksy`/`.spicy` siblings not rewritten — flagged, no working reference
  exists.
- `cargo test` not re-confirmed after this session's edits due to concurrent framework churn —
  reported as `blocked-churn`, not claimed as a pass. `cargo check` IS confirmed clean.
