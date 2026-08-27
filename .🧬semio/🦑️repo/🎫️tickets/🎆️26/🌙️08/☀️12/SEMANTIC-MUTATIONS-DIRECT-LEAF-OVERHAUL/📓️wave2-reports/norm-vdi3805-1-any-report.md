# Wave 2 — `norm/vdi3805` (standard 1, subset any) — semantic mutations migration

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Derivation

`Vdi3805Snapshot` root fields (`manufacturer_file`, `catalog`, `edition_profile`, `correction_as_of`,
`strict_mode`, `index`, `geometry`, `curves`, `limits`) were mapped per `📓️derivation-rules.md`'s
recipe into 19 semantic mutations, deleting the single generic `SetSnapshot { snapshot }` variant
outright (banned per `📓️taxonomy.md`):

**Document-root** (rule 1 — scalar/facet split):
- `update-manufacturer-file` (facet: the norm's `010` header record's 7 fields are always authored
  together)
- `change-correction-as-of` (scalar)
- `change-strict-mode` (scalar)
- `update-limits` (facet: `SecurityLimits`'s 4 fields are one security policy)

**`edition_profile: BTreeMap<String, EditionProfileChoice>`** (name-keyed override map, rule 2 —
mirrors iso16757's `change`/`remove-part-number-input` map-upsert pattern):
- `change-edition-profile` (upsert), `remove-edition-profile` (clear override)

**`catalog.products: Vec<CatalogueProduct>`** (id-keyed by `identity.article_number`, rule 2):
- `create-product`, `delete-product`, `rename-product` (title), `replace-product-configuration`
  (whole `Configuration` swap)
- `catalog.index: CatalogIndex` is persisted state that mirrors `catalog.products` 1:1
  (`CatalogIndex::from_catalog`'s per-product mapping) — every product mutation keeps it in
  lockstep via `mutations::catalog_index_entry_for`/`extract_dn` helpers rather than letting it
  drift, since both fields are `#[state(persistent)]` on the same snapshot.

**`geometry: BTreeMap<String, ParametricGeometry>`** (id-keyed, rule 2/7):
- `create-geometry`, `delete-geometry`, `resize-geometry` (bbox extent), `add-geometry-connection`/
  `remove-geometry-connection` (the `Vec<ConnectionPoint>` field, addressed by the connection's own
  stable `id`, never by index), `replace-geometry-parameters` (whole tuning-map swap)

**`curves: BTreeMap<String, CharacteristicCurve>`** (id-keyed, rule 2):
- `create-curve`, `delete-curve`, `replace-curve-points`

Deliberately NOT modeled (kept out of scope, no gesture in the current engine/app for them):
`geometry.parameters`'s individual keys (folded into the whole-map `replace-geometry-parameters`
instead — see rule 6's guidance to prefer a targeted `replace` over inventing structure not
justified by a real edit gesture), product `accessories`/`components` `Vec` members (present in the
schema but never populated/read anywhere in `⚙️engine`, so no domain gesture to derive a verb from).

## Implementation

- Dispatch enum rewritten: `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]`
  `#[mutations(snapshot = Vdi3805Snapshot, diff = Vdi3805Diff, schema = "s.norm.vdi3805")]` — every
  variant a single-field tuple. **Note for future fan-out agents**: the worked example in
  `command/component.rs`'s `MiniMutation` fixture and this ticket's task brief both say
  `dsl_derive::Mutations`, but `semio-s-plugin-norm` (like every consuming plugin crate) does not
  depend on the `dsl_derive` proc-macro crate directly — only `semio-framework-os-kernel` does, and
  `📦️glue.rs` aliases it as `dsl`/`store`/`protocol`/`vcs`. The derive must be invoked as
  `dsl::Mutations` (re-exported from `🗣️dsl/🦀️component.rs`: `pub use dsl_derive::{..., Mutations};`).
  Sibling facet `iso16757` (already migrated, same crate) uses the same corrected path.
- 19 triad leaves (`🦠️mutation`/`🔺️diff`/`↩️inverse`), each self-wired via `#[path = "."]` blocks in
  the dispatch file's `🔖️LeafWiring` region (facet-local, not `📦️glue.rs` — that file is
  plugin-shared and out of this ticket's edit scope).
- `Vdi3805Diff`'s existing shape (`Option<WholeField>` per top-level snapshot field, already present
  before this migration) was reused as-is — every new mutation's diff sets exactly one field to a
  freshly-computed whole value built directly from `base` + the payload (never apply-then-capture at
  the snapshot level). No `Vdi3805Diff`/apply/absorb changes were needed.
- Hand-rolled `OpText`/`OpBinary` for the new 19-variant enum in `📝️text/🦀️component.rs` (`#[derive(Mutations)]`
  only generates `Mutation`/`SemanticMutation`, never the wire codecs) — one keyword per verb,
  `keyword key=value ...`, structured payload fields routed through a shared `enc_json`/`dec_json`
  (JSON-in-a-quoted-string) helper since every payload field already derives `Serialize`/
  `Deserialize`. `💾️binary/🦀️component.rs` needed no change (it already just delegates to
  `OpText`/`OpBinary` impls on the mutation type, which now live in `📝️text`).
- Old `📄set-snapshot` triad orphaned in place (doc-comment stub only, matching `iso16757`'s
  precedent exactly) because `📦️glue.rs` still `#[path]`-wires those three files — see
  `sharedFileRequests`.
- `🧬️mutations/🦀️component.rs`'s existing `🧪️Tests` region extended (not a new test file) with a
  `round_trip` helper (diff/inverse law check per mutation) and one test per mutation family (root
  scalars, edition-profile upsert/remove + fresh-key-undo-is-remove, full product lifecycle
  including index lockstep assertions, geometry lifecycle, curve lifecycle, semantic-kinds-count).
  `📝️text/🦀️component.rs`'s existing `🧪️Tests` region extended with `demo_mutation_cases` (one
  representative payload per variant, plus a second `CreateProduct` case exercising `VdiValue`
  nested inside `configuration.parameters`) and the shared `op_text_binary_roundtrip_law` test.
- Framework testkit (`🧰️framework/.../📡️spr/🧪️testkit`'s `assert_mutation_inverse_law`/
  `assert_mutation_diff_absorb_law`) deliberately NOT wired in: grepped `semio-s-plugin-norm`'s
  `Cargo.toml` and the whole crate — no existing `testkit` dependency/import anywhere (`iso16757`'s
  migration made the same call). Per the recipe's step (e), skipped rather than adding a new Cargo
  dependency; noted here instead. The `round_trip` helper directly exercises the same diff/inverse
  laws.
- Grammar (`📖️component.grammar.semio`) / binary protocol (`📡️component.protocol.semio`) NOT
  updated — still the generic pre-migration stub, matching `iso16757`'s precedent; task step (f) is
  explicitly non-blocking.
- No `.ts` mirrors created for the 19 new triad leaves — matching `iso16757`'s precedent in the same
  wave (only the pre-existing top-level/set-snapshot `.ts` stubs remain, untouched).

## Verification

`cargo check -p semio-s-plugin-norm`: **zero errors anchored inside this artifact directory**,
confirmed by two independent runs after the fix below (27 errors both times, all in `🎛️apps/**`
across 9 already-or-concurrently-migrated norm facets). `cargo check --tests -p semio-s-plugin-norm`
also type-checks clean inside this directory (46 errors, same `SetSnapshot`-in-`apps` pattern, now 5
per facet since the app's own `#[cfg(test)]` blocks add 2 more call sites for `vdi3805`/`iso16757`).

Could NOT run `cargo test` — the crate's non-test code doesn't fully build yet (blocked by the
`🎛️apps/**` fallout below, none of it mine to fix), so none of my new tests' assertions have been
runtime-verified, only type-checked. Flagging this per policy rather than claiming green tests.

**Bug found and fixed during verification** (real, inside this facet, not churn): my first draft
used `dsl_derive::Mutations` per the task brief's worked example, which doesn't compile in a
consuming plugin crate (see Implementation note above) — fixed to `dsl::Mutations`, which dropped
the error count from 94 to 27 (all remaining ones outside this directory).

**Workspace churn observed and outlived** (per house policy, not chased): three separate transient
`semio-s-plugin-stdio` compile failures (an `xml`/`jpg` artifact's `register_artifact_inferences`
wiring flapping under concurrent edits) blocked reaching `semio-s-plugin-norm` entirely on 3 of the
~7 check runs this session; each resolved on retry within the allowed window.

## sharedFileRequests (for the plugin-wide reconciliation pass — NOT edited here, out of boundary)

1. `✏️s/🔌️plugins/📕️norm/🎛️apps/📔️vdi3805/🦀️component.rs`:
   - Line ~107 `import_media`: `Vdi3805Mutation::SetSnapshot { snapshot }` no longer exists. Whole-
     document import is exactly the "no in-history mutation" case per `📓️taxonomy.md` — route
     through `store.reset(...)` / the non-history import path other migrated artifacts use, not a
     mutation.
   - Test `set_snapshot_commits_a_host_backed_report` / `undo_redo_round_trips_through_the_wrapper`
     (same file) construct `Vdi3805Command::SetSnapshot(set_snapshot::SetSnapshot { snapshot: ... })`
     — depends on the `🎮️commands/📤️set-snapshot` command below.
2. `✏️s/🔌️plugins/📕️norm/🎛️apps/📔️vdi3805/🎮️commands/📤️set-snapshot/🦀️component.rs`: `handle()` emits
   `Vdi3805Mutation::SetSnapshot { snapshot: payload.snapshot.clone() }`. This command IS the app's
   "replace the whole document" gesture (its own manifest action `setSnapshot`) — per
   `📓️derivation-rules.md` rule 6, this whole command should be re-pointed at the non-history
   `store.reset` path (or, if the reconciliation pass decides the manifest action itself should
   retire in favor of composing semantic mutations, that's a product decision beyond a single facet
   agent).
3. `✏️s/🔌️plugins/📕️norm/🎛️apps/📔️vdi3805/🎮️commands/🧮️evaluate/🦀️component.rs`: `handle()` recommits
   the current projection unchanged via `Vdi3805Mutation::SetSnapshot { snapshot: doc.snapshot.clone() }`
   purely to trigger recompute — needs a real no-op-safe path (a dedicated `Emit::Recompute`/reconcile
   hook, or simply emitting no mutations at all since `NormHost::evaluate` already recomputes the
   report without a snapshot change) instead of a fake self-mutation.
4. `📦️glue.rs` (`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs`, lines ~252–260): still
   `#[path]`-wires the now-orphaned `🧬️mutations/📄set-snapshot/{🦠️mutation,🔺️diff,↩️inverse}` triad.
   Delete that `pub mod set_snapshot { ... }` block once every norm facet has migrated (same request
   applies to `iso16757`'s identical orphaned block, and presumably every other norm facet as it
   migrates — a single glue.rs edit at the end of the wave covers all of them).

## Files touched

All under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`:

- `🦀️component.rs` (rewritten: leaf wiring, index-sync helpers, dispatch enum, tests)
- `📝️text/🦀️component.rs` (rewritten: hand-rolled `OpText`/`OpBinary`, demo cases, round-trip test)
- `📄set-snapshot/🦠️mutation/🦀️component.rs`, `📄set-snapshot/🔺️diff/🦀️component.rs`,
  `📄set-snapshot/↩️inverse/🦀️component.rs` (orphaned to doc-only stubs)
- 19 new triad leaf directories (57 new files), one per mutation:
  `🏭️update-manufacturer-file/`, `📅️change-correction-as-of/`, `🔐️change-strict-mode/`,
  `🛡️update-limits/`, `🔁️change-edition-profile/`, `➖️remove-edition-profile/`, `📦️create-product/`,
  `🗑️delete-product/`, `🏷️rename-product/`, `♻️replace-product-configuration/`, `🧊️create-geometry/`,
  `🗑️delete-geometry/`, `📐️resize-geometry/`, `🔌️add-geometry-connection/`,
  `✂️remove-geometry-connection/`, `🔧️replace-geometry-parameters/`, `📈️create-curve/`,
  `🗑️delete-curve/`, `📉️replace-curve-points/` (each `🦠️mutation/🦀️component.rs` +
  `🔺️diff/🦀️component.rs` + `↩️inverse/🦀️component.rs`)

Not touched: `💾️binary/🦀️component.rs` (already correct, delegates to `OpText`/`OpBinary`), the
sibling `📸️snapshot`/`🔺️diff`/`💡️inferences` facets, `⚙️engine/🦀️component.rs`,
`🚪️io/🦀️component.rs`, `📚️examples/**`, plugin-root `🦀️component.rs`, `📦️glue.rs`, `🎛️apps/**`.
