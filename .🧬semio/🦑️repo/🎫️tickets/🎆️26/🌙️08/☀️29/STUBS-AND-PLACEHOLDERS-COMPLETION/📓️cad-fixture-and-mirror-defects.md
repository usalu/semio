# CAD fixture regeneration + iso16757/procedural3d TS-mirror defect sweep

Three independent items. Status: (B) and (C) done and verified; (A) in progress — cargo build
stuck behind heavy concurrent workspace contention (see below), report will be updated in place
once the real `cargo test` output lands.

## (A) CAD `default_example_dsl_round_trips` — IN PROGRESS

File: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs`

### Root-cause finding

`git diff fd01661f06 515271bf60 -- .../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` shows the fixture
was mechanically migrated from the pre-rewrite grammar (inline `shape-geometry`
vertices/edges/wires/faces/shells/solids brep + `objects`/`building-objects`/…  tables) straight to
the new `CadSnapshot` field names, but the migration **dropped the shape/building model
composition** instead of translating it: `shapeModel=[]` / `buildingModel=[]` (i.e. `None`) even
though the pre-rewrite text had real `objects`/`building-objects` content. Per the new schema
(`🧬️schema/📸️snapshot/🦀️component.rs:19-51`), `shape_model`/`building_model`/etc. are now composed
`s.stdio.semio.model` **child handles** (`Option<CadModelChild>`), not inline geometry — the brep
topology moved to a separate composed artifact and can no longer live in `CadSnapshot` at all. So
"regenerate via print_dsl" cannot mean "paste the old geometry back in verbatim"; it means
constructing a `CadSnapshot` value that correctly populates the child-handle slots the new schema
actually has.

`references_by_model_definition_id` (site-photo reference) and `nodes` (root node) in the
currently-committed fixture already decode correctly and match `testkit::sample_scene()`'s content
byte-for-byte (`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️component.rs:483-497`) — only the two model
child slots are missing.

### Fix applied

Added a throwaway capture test next to the ignored test in the same file, to regenerate the fixture
text via a real `print_dsl` call (never hand-transcribed) instead of hand-editing the `.semio` file:

```rust
#[semio_framework_async_macros::async_test]
#[ignore = "scratch: run with --ignored --nocapture to capture the regenerated fixture text"]
async fn scratch_capture_default_example_text() {
    panic!("\n-----BEGIN-----\n{}-----END-----\n", print_dsl(&sample_scene()));
}
```

`sample_scene()` sets `shape_model`/`building_model` to real child handles (`sample_model_child`)
and is already proven round-trip-safe by the existing (non-ignored) `cad_scene_round_trips_through_dsl_document`
test, so reusing it avoids inventing new, unverified content for the demo fixture.

### Blocked on

`cargo test -p semio-s-plugin-cad` has been running for 45+ min at ~0% CPU (both this attempt and an
earlier plain `--ignored` run of the pre-existing test) — matches the known "Concurrent Cargo
Workspace Churn" pattern (another session's in-progress build holding a shared lock; a sibling
`semio-framework-surface` cargo test from a different session was observed running concurrently).
Waiting it out rather than assuming this is a bug in my change. **This section will be filled in
with the real captured text, the final diff, and real `cargo test` pass/fail output once the build
actually runs** — no test-passing claim is made until then.

---

## (B) `📓️iso16757` mutation-verb TS mirrors

Scope: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/*` leaf
files only (`🦠️mutation`, `↩️inverse`; left `🔺️diff` and the union-root `🟦️component.ts` alone — the
latter is being actively filled by a concurrent agent per the brief, confirmed live via
`git status --porcelain` showing it and ~30 sibling non-TS files under active `M` during this
session).

### Named defect 1 — `RenameProduct` payload missing `id`

- Rust: `🛏️rename-product/🦠️mutation/🦀️component.rs:8-11` — `pub struct RenameProduct { pub id: String, pub new_name: String }`.
- TS (before): `🛏️rename-product/🦠️mutation/🟦️component.ts` — `export interface RenameProduct { newName: string; }` (no `id`).
- **Confirmed real defect.** Fixed: added `id: string;`.
- Same defect, same fix, found on spot-check: `🚿rename-product-group/🦠️mutation/🦀️component.rs:8-11` (`id`+`new_name`) vs its TS interface, which also lacked `id`.

### Named defect 2 — `DeleteProductInverse` aliased to `DeleteProduct` instead of `CreateProduct`

- Rust: `🌸delete-product/↩️inverse/🦀️component.rs:9-14` — inverse constructs
  `Iso16757Mutation::CreateProduct(CreateProduct { product: ..., index: Some(position) })` from BASE,
  i.e. the inverse of a delete is a **create**, never another delete.
- TS (before): `🌸delete-product/↩️inverse/🟦️component.ts` — `export type DeleteProductInverse = DeleteProduct;` — wrong shape (`{id}` instead of `{product, index?}`), and semantically backwards (would replay the delete instead of undoing it).
- **Confirmed real defect.** Fixed: `export type DeleteProductInverse = CreateProduct;` (imported from the create-product leaf).
- **Found identical bug on all three delete-siblings** (same "undo re-creates from BASE" pattern in Rust, same wrong self-aliased TS):
  - `🌹delete-product-group` → fixed to mirror `CreateProductGroup` (rust: `🌹delete-product-group/↩️inverse/🦀️component.rs:9-14`).
  - `🌺delete-property-definition` → fixed to mirror `CreatePropertyDefinition` (rust: `🌺delete-property-definition/↩️inverse/🦀️component.rs:9-14`).
  - `🌻delete-subject` → fixed to mirror `CreateSubject` (rust: `🌻delete-subject/↩️inverse/🦀️component.rs:9-14`).

### Further drift found on spot-check (payload leaves)

All confirmed against the Rust `🦠️mutation/🦀️component.rs` payload struct in each verb's own directory; nested-type shapes cross-checked against `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🦀️component.rs` (line refs below) and against the artifact's own union-root `🧬️mutations/🟦️component.ts`, which already carries correct mirrors of these same nested types (`Product`/`ProductGroup`/`PropertyDefinition`/`Subject`/`CatalogueValue`/`PartNumberRule`/`ExchangeProcess`/`SelectionConstraint`) — imported from there rather than re-declared, to stay DRY and avoid re-diverging from a second copy.

| Verb (payload) | Rust shape | TS before | TS after |
|---|---|---|---|
| `🌱change-part-number-input` | `key: String, new_value: CatalogueValue` | `{ newValue: string }` (missing `key`, wrong type) | `{ key: string; newValue: CatalogueValue }` |
| `🌿remove-part-number-input` | `key: String` | `{}` (empty) | `{ key: string }` |
| `🍂replace-part-number-rule` | `new_rule: PartNumberRule` (tagged enum, `component.rs:703-708`) | `{ newRule: string }` | `{ newRule: PartNumberRule }` |
| `🍃change-exchange-process` | `new_exchange_process: ExchangeProcess` (unit enum, `component.rs:687-696`) | `{ newExchangeProcess: string }` | `{ newExchangeProcess: ExchangeProcess }` |
| `🌼change-selection-series` | `new_series_id: Option<String>` | `{ newSeriesId: string }` (not optional) | `{ newSeriesId?: string }` |
| `🛁add-selection-constraint` | `constraint: SelectionConstraint` (`component.rs:371-374`) | `{}` (empty) | `{ constraint: SelectionConstraint }` |
| `🛋️remove-selection-constraint` | `index: usize` | `{}` (empty) | `{ index: number }` |
| `🍁create-product` | `product: Product, index: Option<usize>` (`component.rs:269-277`) | `{}` (empty) | `{ product: Product; index?: number }` |
| `🍀create-product-group` | `product_group: ProductGroup, index: Option<usize>` (`component.rs:192-196`) | `{}` (empty) | `{ productGroup: ProductGroup; index?: number }` |
| `🌾create-property-definition` | `property_definition: PropertyDefinition, index: Option<usize>` (`component.rs:229-238`) | `{}` (empty) | `{ propertyDefinition: PropertyDefinition; index?: number }` |
| `🌵create-subject` | `subject: Subject, index: Option<usize>` (`component.rs:592-600`) | `{}` (empty) | `{ subject: Subject; index?: number }` |

### Further drift found on spot-check (inverse leaves, beyond the two named delete-cases)

Read every inverse's Rust body (not just its payload struct) — several undo a mutation with a
*different* mutation kind, which the TS side was aliasing to the wrong (self-referential) type:

| Verb (inverse) | Rust inverse behaviour | TS before | TS after |
|---|---|---|---|
| `🌱change-part-number-input` | returns `ChangePartNumberInput` (BASE had a value) **or** `RemovePartNumberInput` (BASE had none) — a real union, `↩️inverse/🦀️component.rs:9-14` | `= ChangePartNumberInput` (missing the remove branch) | `= ChangePartNumberInput \| RemovePartNumberInput` |
| `🌿remove-part-number-input` | returns `ChangePartNumberInput` (never `RemovePartNumberInput`), `↩️inverse/🦀️component.rs:9-14` | `= RemovePartNumberInput` (self-aliased, wrong) | `= ChangePartNumberInput` |
| `🛁add-selection-constraint` | returns `RemoveSelectionConstraint` at the append index, `↩️inverse/🦀️component.rs:9-11` | `= AddSelectionConstraint` (self-aliased, wrong) | `= RemoveSelectionConstraint` |
| `🛋️remove-selection-constraint` | returns `AddSelectionConstraint` with the captured value, `↩️inverse/🦀️component.rs:9-14` | `= RemoveSelectionConstraint` (self-aliased, wrong) | `= AddSelectionConstraint` |

Verbs checked and found **already correct** (self-aliased inverse genuinely matches a self-typed
Rust inverse): `rename-product`, `rename-product-group`, `rename-catalogue`, `rename-manufacturer`,
`update-script-limits`, `change-selection-class`, `replace-part-number-rule`,
`change-exchange-process`, `change-selection-series` — left untouched.

### Not fixed — reported, not touched

`🔺️diff` leaves for every create/delete verb (`create-product`, `create-product-group`,
`create-property-definition`, `create-subject`, `delete-product`, `delete-product-group`,
`delete-property-definition`, `delete-subject`, `add-selection-constraint`,
`remove-selection-constraint`, `remove-part-number-input`) are empty `export interface XDiff {}`.
The Rust diff for every one of these is a whole-catalogue swap
(`Iso16757Diff { catalogue: Some(catalogue), ..Default::default() }`) — this artifact's TS "diff
fragment" convention is a hand-picked semantic summary (e.g. `RenameProductDiff { name?: string }`),
not a literal mirror of `Iso16757Diff`, and there is no already-filled create/delete precedent
*within this artifact* to pattern-match the right field name/shape from. Filling these in would mean
guessing new field names with no verifiable source of truth, which risks writing new wrong content
under the guise of a fix. Flagging as a genuine follow-up rather than guessing.

### Verification (real, run)

```
bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution bundler \
  --esModuleInterop --skipLibCheck <21 touched files>
```
Exit code `0`, zero output — ran twice (once right after editing, once again after the concurrent
agent's edits landed on the union-root `🧬️mutations/🟦️component.ts`, to make sure the imports still
resolve against the live file).

### Files touched (21)

```
🛏️rename-product/🦠️mutation/🟦️component.ts
🚿rename-product-group/🦠️mutation/🟦️component.ts
🍁create-product/🦠️mutation/🟦️component.ts
🍀create-product-group/🦠️mutation/🟦️component.ts
🌾create-property-definition/🦠️mutation/🟦️component.ts
🌵create-subject/🦠️mutation/🟦️component.ts
🌸delete-product/↩️inverse/🟦️component.ts
🌹delete-product-group/↩️inverse/🟦️component.ts
🌺delete-property-definition/↩️inverse/🟦️component.ts
🌻delete-subject/↩️inverse/🟦️component.ts
🌱change-part-number-input/🦠️mutation/🟦️component.ts
🌱change-part-number-input/↩️inverse/🟦️component.ts
🌿remove-part-number-input/🦠️mutation/🟦️component.ts
🌿remove-part-number-input/↩️inverse/🟦️component.ts
🍂replace-part-number-rule/🦠️mutation/🟦️component.ts
🍃change-exchange-process/🦠️mutation/🟦️component.ts
🌼change-selection-series/🦠️mutation/🟦️component.ts
🛁add-selection-constraint/🦠️mutation/🟦️component.ts
🛁add-selection-constraint/↩️inverse/🟦️component.ts
🛋️remove-selection-constraint/🦠️mutation/🟦️component.ts
🛋️remove-selection-constraint/↩️inverse/🟦️component.ts
```
(all under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`)

---

## (C) `🌀️procedural3d` `FormGeneration` type defect — CONFIRMED AND FIXED

### Evidence

- Rust source of truth: `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️component.rs:344-349`:
  ```rust
  pub struct FormGeneration {
      pub id: String,
      pub name: String,
      pub values: Map<String, Value>,  // serde_json::Map<String, Value>
  }
  ```
  (used via `flow::playbook::FormGeneration` in
  `🧬️mutations/➕create-generation/🦠️mutation/🦀️component.rs:9`.)
- TS before (3 separate inline duplicate declarations, all wrong the same way):
  - `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️component.ts:46`
  - `.../🧬️schema/📸️snapshot/🟦️component.ts:22`
  - `.../🧬️schema/🔺️diff/🟦️component.ts:51`

  all three: `export type FormGeneration = { id: string; name: string; valuesJson: string };`
- TS already-correct precedent (found, not touched):
  `.../🧬️mutations/➕create-generation/🦠️mutation/🟦️component.ts` —
  `interface FormGeneration { id: string; name: string; values: Record<string, unknown>; }`, i.e.
  `Record<string, unknown>` is this repo's established mirror for a Rust `serde_json::Map<String, Value>`
  / `BTreeMap<String, T>` (same convention independently confirmed in
  `🔱️trinity/🗿️artifacts/🔌️jack/…/🌱️create-node/🟦️component.ts` `JackPort.properties` and in
  iso16757's own `Product.parameterValues`/`ProductVariant.parameterValues`).
  A prior session's research note, `.🧬semio/.../STUBS-AND-PLACEHOLDERS-COMPLETION/📓️ts-mirrors-procedural3d.md`
  ("Unfinished / caveats"), had already flagged this exact `valuesJson` mismatch in the diff file as
  a known follow-up — this closes that.

**Confirmed real defect** (not a misreading) — no runtime `.ts`/`.tsx` code anywhere in the repo reads
`.valuesJson` (`grep -rn valuesJson --include=*.ts --include=*.tsx .` → only the type declarations
themselves), so the fix is a self-contained, non-cascading rename.

### Fix

Changed all three duplicate declarations to
`export type FormGeneration = { id: string; name: string; values: Record<string, unknown> };`.

### Consumers grepped and checked

`grep -rln FormGeneration --include=*.ts --include=*.tsx .` → 9 hits, 6 inside `procedural3d`:
1. `🧬️schema/🟦️component.ts` — **fixed** (own declaration).
2. `🧬️schema/📸️snapshot/🟦️component.ts` — **fixed** (own declaration).
3. `🧬️schema/🔺️diff/🟦️component.ts` — **fixed** (own declaration).
4. `🧬️mutations/➕create-generation/🦠️mutation/🟦️component.ts` — already correct (source of the right shape); untouched.
5. `🧬️mutations/➕create-generation/🔺️diff/🟦️component.ts` — imports `FormGeneration` from #4; no change needed, still type-checks.
6. `🧬️mutations/🗑delete-generation/↩️inverse/🟦️component.ts` — imports `FormGeneration` from #4; no change needed, still type-checks.

Blast radius: contained to these 3 files, all pure type declarations (`Record<string, unknown>` is a
superset-compatible widening of the type, no narrowing that could break a consumer). Not sprawling —
no wire-format/snapshot-serialization code depends on the literal `valuesJson` field name.

3 remaining hits are `procedural2d`'s own **independent** copy of the identical bug
(`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/{🧬️schema/🟦️component.ts:34, 🧬️schema/📸️snapshot/🟦️component.ts:22, 🧬️schema/🔺️diff/🟦️component.ts:39}`
— same `valuesJson: string` wrongness, same Rust `flow::playbook::FormGeneration` source of truth,
but a separate artifact, out of this ticket item's explicit scope (`🧊️procedural3d` only). Left
untouched; flagged as a spawn-task follow-up.

### Verification (real, run)

```
bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution bundler \
  --esModuleInterop --skipLibCheck <6 files listed above>
```
Exit code `0`, zero output.

---

## Summary of files changed

- CAD: `.../🧬️schema/📸️snapshot/📝️text/🦀️component.rs` (scratch capture test added; fixture file +
  ignore removal pending real `cargo test` output — see (A)).
- iso16757: 21 TS leaf files under `🧬️mutations/` (list above).
- procedural3d: 3 TS files (`🧬️schema/🟦️component.ts`, `🧬️schema/📸️snapshot/🟦️component.ts`,
  `🧬️schema/🔺️diff/🟦️component.ts`).

## Unfinished

- (A) real `cargo test -p semio-s-plugin-cad` pass/fail pending — build stuck behind concurrent
  workspace lock contention, not a failure of the change itself as far as currently known.
- (B) 11 empty `🔺️diff` leaves in iso16757 create/delete verbs — flagged, not fixed (no verifiable
  source of truth for the intended field shape within this artifact).
- (C) procedural2d carries the identical `FormGeneration.valuesJson` bug in 3 files — out of this
  item's scope, flagged for a follow-up task.
