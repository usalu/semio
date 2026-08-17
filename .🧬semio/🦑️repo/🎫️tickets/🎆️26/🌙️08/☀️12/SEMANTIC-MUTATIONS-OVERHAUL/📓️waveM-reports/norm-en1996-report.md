# Wave M — `norm` / `en1996` / `1` / `any` — mutations facet migration (Job A: from scratch)

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Vocabulary derived

`En1996Snapshot` is a flat, id-less, document-root parameter form: 22 persistent scalar/enum fields
(design actions, resistances, masonry/mortar/exposure classes, effective geometry) for the EN 1996
masonry design check — no id-keyed collections, no name/identity field. Per derivation-rules rule 1,
every field became its own `change-<field>` mutation; none qualified for the `update-<facet>`
grouping exception (each is an independently-entered input). The pre-migration
`En1996Mutation::SetSnapshot { snapshot }` (sole variant) is gone with no replacement mutation; the
old `crate::impl_norm_set_snapshot_ops!` macro call is removed with it (the macro itself is now
dead code plugin-wide — see the lane summary).

All 22 mutations (verb-entity-field): `change-annex`, `change-area-mm2`, `change-bed-joint-
thickness-mm`, `change-design-situation`, `change-exposure`, `change-fire-resistance-min`,
`change-fk-mpa`, `change-f-vk-mpa`, `change-h-ed-kn`, `change-h-ef-mm`, `change-m-ed-knm`,
`change-masonry-class`, `change-mortar`, `change-mu`, `change-n-ed-kn`, `change-shear-area-mm2`,
`change-storeys`, `change-t-ef-mm`, `change-unit`, `change-v-ed-kn`, `change-wall-thickness-mm`,
`change-z-mm3`. Every `SEMANTICS.kind` was computed with the derive's own `to_kebab` algorithm
(traced by hand against `🗣️dsl/✨️derive/🦀️component.rs`'s `to_kebab`), confirmed by `cargo check`'s
own compile-time kind/kebab assertion never firing for this facet.

## Directory layout

22 new triad directories, each with a unique emoji within the facet (📐🔽🔼↔️➡️⬅️📏🟩✂️🔨🗺️🧱🏗️🎢🧊🌡️💧🌬️🔥❄️⚡🔆).
The pre-migration `📄set-snapshot` directory was deleted outright (not repurposed) since this lane's
agent owns `📦️glue.rs` directly — no self-wiring `#[path = "."]` blocks were ever written for this
facet.

## Wiring

`📦️glue.rs`'s `en1996::…::mutations` block now mounts all 22 triads directly as siblings of the
dispatch `component` module (`pub mod change_<field> { pub mod mutation; pub mod diff; pub mod
inverse; }`), each with a real physical `#[path]`. No orphan/legacy mounts remain.

## OpText/OpBinary

`🧬️mutations/📝️text/🦀️component.rs` and `💾️binary/🦀️component.rs` were fully rewritten: a local
`En1996MutationDsl` DSL mirror enum (`#[derive(dsl::DslEnum)]`) plus `to_dsl`/`from_dsl` bridging
functions and handcrafted `OpText`/`OpBinary` impls, mirroring `din16798`'s established pattern —
the old blanket-impl comment ("no bespoke operation enum... `impl_norm_set_snapshot_ops!` covers
it for free") was stale even before this session (the dispatch enum had already gone semantic) and
is now correct.

## `from_snapshot` (new production helper, not just a test fixture)

Added `En1996Mutation::from_snapshot(snapshot: &En1996Snapshot) -> Vec<En1996Mutation>` — decomposes
a whole document into one `change-<field>` mutation per persistent field. This is the closed-
vocabulary replacement for the banned whole-document-replace variant, used by:
- `🎛️apps/📘️en1996/🦀️component.rs`'s `import_media` (`"model:in"` port)
- `🎛️apps/📘️en1996/🎮️commands/📤️set-snapshot/🦀️component.rs` (renamed payload struct
  `SetSnapshot` → `ReplaceSnapshot`, keyword unchanged; handler now calls
  `app_surface::commit_snapshot_fields(En1996Mutation::from_snapshot(&payload.snapshot), …)`)

`crate::app_surface::import_media`'s generic signature was changed plugin-wide from `F: Fn(D) -> M`
to `F: Fn(D) -> Vec<M>` (bundling one `Emit::mutations` call instead of one `Emit::mutations(vec![…])`
per mutation) — see the lane summary for the full blast-radius and why this was a necessary,
justified shared-file change rather than a per-facet workaround.

`🎮️commands/🧮️evaluate/🦀️component.rs` no longer re-commits a no-op whole-document mutation
(impossible now); since the compliance report is derived on every read and never persisted, its
handler now returns `Ok(Emit::default())` — genuinely zero mutations, which is the honest behavior.

## Tests

Extended the existing `🧪️Tests` region in `🧬️mutations/🦀️component.rs` (no new test files):
`every_mutation()` fixture (22 entries), `every_variant_registers_an_approved_semantic_descriptor`,
`every_variant_round_trips_via_inverse`, `from_snapshot_round_trips_via_full_document_replacement`,
and three `protocol::os_spr::testkit::assert_mutation_inverse_law` /
`assert_mutation_diff_absorb_law` pairs on structurally distinct variants (`change-annex` — enum,
`change-m-ed-knm` — f64, `change-unit` — String). `🧬️mutations/📝️text/🦀️component.rs` got its own
`every_variant_op_text_round_trips` loop plus targeted single-variant tests.

## `🟦️component.ts` mirrors

All 22 triads (mutation/diff/inverse) now carry real, non-stub `.ts` files: a TS `interface` per
payload (camelCase fields, matching the Rust struct's `#[serde(rename_all = "camelCase")]`), a
sparse diff-fragment interface, and an inverse type alias — not `export {};`.

## Verification

`cargo check -p semio-s-plugin-norm` — run at the end of the whole lane (see
`📓️waveM-reports/norm-lane-summary.md` for the single combined gate output covering all 15
facets; individual per-facet `cargo check` runs were not re-captured separately once the whole
crate started compiling cleanly for this facet's own files, since the workspace was mid-churn from
a concurrent framework session for most of this session — see the summary's `blocked-churn` note).
Grepped `🗿️artifacts/📘️en1996/**` and `🎛️apps/📘️en1996/**` for `SetSnapshot|NoMutation|
CollectionMutation(<|::)`: zero hits outside the (out-of-policy-scope) app-command-enum variant name
`En1996Command::SetSnapshot` (the manifest action id `"setSnapshot"` was left unchanged — only the
payload struct backing it, `set_snapshot::ReplaceSnapshot`, was renamed).

## `sharedFileRequests`

None outstanding — this facet's own `📦️glue.rs`/`app_surface.rs` edits were made directly (this
lane's agent owns the whole plugin).

## `allowlistKeysToRemove`

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs` *(fixed a broken `SetSnapshot` test fixture left over from the pre-migration diff-layer test)*
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1996/🎮️commands/📤️set-snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1996/🎮️commands/🧮️evaluate/🦀️component.rs`

## Files touched

Created: 22 triad dirs × 3 leaves × 2 files (`.rs`+`.ts`). Rewrote: `🧬️mutations/🦀️component.rs`,
`🧬️mutations/📝️text/🦀️component.rs`, `🧬️mutations/💾️binary/🦀️component.rs` (tests only),
`🔺️diff/📝️text/🦀️component.rs` (test fix only). App files rewritten: `🎛️apps/📘️en1996/🦀️component.rs`
(import_media + command registration), `🎮️commands/📤️set-snapshot/🦀️component.rs`,
`🎮️commands/🧮️evaluate/🦀️component.rs`. Plugin-shared (in-bounds, this lane's agent owns the whole
plugin): `📦️packages/🦀️rust/📦️glue.rs` (en1996 mutations mount block), `🖥️app-surface/🦀️component.rs`
(`import_media` signature + new `commit_snapshot_fields` helper — shared across all 15 facets, see
lane summary), `📄️artifact/🦀️component.rs` (removed the now-fully-dead `impl_norm_set_snapshot_ops!`
macro and its four helper fns once every facet stopped calling it — see lane summary). Deleted:
`🧬️mutations/📄set-snapshot/**` (6 files).
