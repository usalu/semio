# 🧪️ Handcrafted mutation fixtures — `📔️vdi3805`, `📘️en1993`, `📙️din18599`, `📘️en1990`

59 mutation leaves, 59 committed test cases, one per leaf. 54 applied, 5 rejected.
Authoring scripts (kept per CLAUDE.md, one dict of hand-authored content per case):
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/COMPOSE-TO-PUZZLE5D-MIGRATION/🔧️fixtures-vdi3805-en1993-din18599-en1990/`.

## 📊️ Coverage

| tree | leaves | cases | applied | rejected |
| --- | --- | --- | --- | --- |
| `📔️vdi3805/…/🧬️mutations` | 19 | 19 | 19 | 0 |
| `📘️en1993/…/🧬️mutations` | 17 | 17 | 17 | 0 |
| `📙️din18599/…/🧬️mutations` | 13 | 13 | 12 | 1 |
| `📘️en1990/…/🧬️mutations` | 10 | 10 | 6 | 4 |

`bun ./📜️script.ts fixtures lint --by-tree` (from `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust`):

```
🧬️ 115 artifact mutation trees · 1558 mutations · 1260 covered · 298 uncovered
```

None of the four trees appears in the `--by-tree` uncovered list, and zero of the CLI's 320 repo-wide
errors come from `✏️s/🔌️plugins/📕️norm` (all 320 are `no 🧪️tests cases` rows from still-uncovered
trees plus 22 `enum variant has no mutation directory` rows in `🗄️stdio`'s `🧿️semio` subsets).
Because the CLI truncates its printed error list at 40 rows repo-wide, the lint's own rules were
re-implemented scoped to these four trees only —
`🔧️fixtures-vdi3805-en1993-din18599-en1990/📜️verify.py`:

```
vdi3805    19/19 leaves covered · 21 mounted #[path] · 19 mod lines
en1993     17/17 leaves covered · 19 mounted #[path] · 17 mod lines
din18599   13/13 leaves covered · 15 mounted #[path] · 13 mod lines
en1990     10/10 leaves covered · 12 mounted #[path] · 10 mod lines
scoped errors: 0
```

(The `#[path]` count is 2 higher than the `mod` count in each tree: the grouping `#[path = "."]`
plus one occurrence inside the region's own comment.)

## 🔌️ Wiring

`📦️glue.rs` was NOT touched — it is shared by all fifteen norm artifacts and several lanes edit it
concurrently. Each artifact self-wires its own cases in its mutations-root
`🧬️mutations/🦀️component.rs`, in a new `//#region 🧪️FixtureTests` at the end of the file:

```rust
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "<leaf-dir>/🧪️tests/<case>/🦀️component.rs"]
    mod tests_<leaf>_<case>;
    // …one line per leaf
}
```

Precedent for the self-wiring shape:
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/…/🧬️mutations/🦀️component.rs`.

## ✅️ Verification performed

- **`fixtures lint --by-tree`** — four trees fully covered, zero norm errors (above).
- **Scoped lint re-run** — variants ↔ leaves, core file set per case, `🎯️outcome.status` validity,
  `code` on every rejection, `🚫️component.absent` present AND zero bytes on rejections, no rejected
  case also carrying a diff JSON, both snapshot sides present. 0 errors.
- **`include_str!` targets** — every one of the 5 × 54 + 5 × 5 constants resolves to a file that
  exists (checked by `📜️verify.py`, not by eye).
- **`#[path]` mounts** — all 59 resolve; no duplicate `mod` identifiers.
- **JSON validity** — every committed `🔣️component.json` reparses.
- **`rustfmt --edition 2021 --emit stdout`** — 63 files (59 cases + 4 mutation roots) parse clean.
- **NOT run:** `cargo`. No test has been executed; no claim is made that any of them passes.

## 🧬️ Wire shapes — read off the source, not assumed

| artifact | mutation enum representation | payload field case |
| --- | --- | --- |
| `Vdi3805Mutation` | **externally tagged** (no `#[serde]` on the enum) → `{"ChangeStrictMode":{…}}` | snake_case |
| `En1993Mutation` | **externally tagged** → `{"UpdateBoltInputs":{…}}` | snake_case |
| `Din18599Mutation` | `#[serde(tag = "mutation", rename_all = "camelCase")]` → `{"mutation":"changeHT","newHT":118.0}` | camelCase (every payload struct carries its own `rename_all`) |
| `En1990Mutation` | `#[serde(tag = "mutation", rename_all = "camelCase")]` → `{"mutation":"changeResistance","new_resistance_kn":320.0}` | snake_case, **except** `ChangeAnnex` which alone carries `rename_all` → `newAnnex` |

Snapshots are `rename_all = "camelCase"` in all four; nested value types (`ManufacturerFile`,
`CatalogueProduct`, `BoundingBox`, `MonthlyClimate`, `En1990QkEntry`, …) carry **no** rename rule and
therefore stay snake_case on the wire. The only nested type with its own rule is `vdi3805`'s
`VdiValue` (`#[serde(tag = "kind", rename_all = "camelCase")]` → `{"kind":"integer","value":50}`).

All four `<Artifact>Diff` types are `#[serde(rename_all = "camelCase", default)]` with **no**
`skip_serializing_if`, so every committed diff JSON carries every field, `null` for the untouched
ones. `Vdi3805Diff`/`En1993Diff` additionally have an `artifact: Option<Box<…>>` field (also always
emitted as `null`); `Din18599Diff`/`En1990Diff` have had theirs deleted.

## ⚠️ Pinned limitations

### `Option<Option<u32>>` does not survive a JSON round trip
Every one of the four diff types ends in `#[state(presence)] pub selected_check_index:
Option<Option<u32>>`. `None` and `Some(None)` both encode as `null`, so the distinction between
"untouched" and "explicitly cleared" cannot be expressed in the committed JSON. **No case in this
lane writes that field**; it is `null` in all 54 committed diffs and the limitation is stated in each
test's `committed_diff_is_canonical` docstring rather than asserted around.

### Composed-child slots force `mutation.target-missing` on index-addressed mutations (`en1990`)
`En1990Snapshot.q_k` is an `s.stdio.semio.table` CHILD slot. Its live entry list is read by
`en1990_qk(snapshot)`, which looks the handle up in the thread-local `EN1990_QK_SCRATCH` working-scene
cache — a cache that only `en1990_qk_child_from_entries` ever seeds, at runtime. A snapshot decoded
from committed JSON therefore ALWAYS reads back an **empty** entry list, and every index-addressed
variable-action mutation can only answer `mutation.target-missing`. No hand-authored `⬅️before` can
change this. Four leaves are consequently committed as rejected cases, each pinning that guard and
each asserting the `q_k` handle is not re-minted:

- `🐎remove-variable-action` — `refuses-to-remove-action-0-from-an-unseeded-child-slot`
- `🐗reorder-variable-actions` — `refuses-to-move-action-0-to-slot-1-in-an-empty-list`
- `🐮change-variable-action-category` — `refuses-to-recategorise-a-missing-action-0`
- `🦌change-variable-action-value` — `refuses-to-revalue-a-missing-action-0`

`🐴insert-variable-action` is the one index mutation that succeeds against an empty list, so it is
the lane's only applied `q_k` case.

### `en1990`'s `before`/`after` child ids are toolchain-derived content addresses
`en1990_qk_scene_id` is `format!("en1990-qk-{:016x}", DefaultHasher over serde_json::to_string(entries))`.
Rust's `DefaultHasher` is explicitly **not** guaranteed stable across releases, so these two literals
are pinned to the repo's `rust-toolchain.toml` (`nightly-2026-07-07`) and were computed — not
guessed — by compiling a standalone `rustc` program (no `cargo`) against that toolchain:

| entry list | JSON hashed | child id |
| --- | --- | --- |
| `[]` | `[]` | `en1990-qk-7904dd65836c8ff4` |
| `[{category: "Q_snow", value: 12.5}]` | `[{"category":"Q_snow","value":12.5}]` | `en1990-qk-69c0017661d2372c` |

The committed `⬅️before` uses the empty-list address, which makes the four rejections
self-consistent rather than accidental. **A toolchain bump will change both literals** and break
`insert-variable-action`'s three fixtures; recompute them the same way if that happens.

### `din18599`'s `update-climate` is committed as a rejection for the same reason
`update-climate`'s diff builder mints a content-addressed handle via `din18599_climate_scene_id`
(same `DefaultHasher` construction). Rather than commit a second toolchain-derived literal, its case
pins the `mutation.invariant` guard instead: January's `g_h_w_m2` is `-30.0`, which the builder
refuses BEFORE the no-op check. The non-finite half of that predicate cannot be pinned at all — JSON
has no `Infinity`/`NaN` literal — so the negative-irradiance half is the only expressible one.
`din18599`'s committed `climate` handle is a fixture-authored id with no cache entry, exactly
mirroring the documented all-zero cache-miss read.

## 🗂️ Case index

### `📔️vdi3805` (19 applied)
`update-manufacturer-file`/renames-the-header-manufacturer-to-acme ·
`change-correction-as-of`/advances-the-correction-cut-off-to-2025-03 ·
`change-strict-mode`/turns-strict-mode-on ·
`update-limits`/tightens-every-untrusted-input-limit ·
`change-edition-profile`/switches-sheet-8-from-legacy-to-current ·
`remove-edition-profile`/clears-the-sheet-8-legacy-override ·
`create-product`/appends-vlv-80-002-and-its-index-entry ·
`delete-product`/removes-vlv-50-001-and-its-index-entry ·
`rename-product`/retitles-vlv-50-001-and-resyncs-its-index-tags ·
`replace-product-configuration`/reparameterises-vlv-50-001-to-dn-80-and-resyncs-index-dn ·
`create-geometry`/adds-the-geom-valve-80-definition ·
`delete-geometry`/removes-the-geom-valve-50-definition ·
`resize-geometry`/doubles-the-geom-valve-50-bounding-box ·
`add-geometry-connection`/attaches-the-drain-connection-to-geom-valve-50 ·
`remove-geometry-connection`/detaches-the-out-connection-from-geom-valve-50 ·
`replace-geometry-parameters`/rescales-geom-valve-50-to-half-and-adds-clearance ·
`create-curve`/adds-the-curve-dp-pressure-drop-curve ·
`delete-curve`/removes-the-curve-kvs-flow-curve ·
`replace-curve-points`/resamples-curve-kvs-onto-three-points

The four product/index mutations each assert the persisted `catalog.index` moves in lockstep
(`create`/`delete` add or drop the entry, `rename` rebuilds `tags`, `replace-configuration` recomputes
`dn` via `extract_dn`), and `delete-geometry`/`delete-curve` each assert the dangling
`geometry_ref`/`function_refs` are deliberately NOT cascaded.

### `📘️en1993` (17 applied)
`change-annex` plus the 16 `update-<part>-inputs` groups. The `before` is one S355 rolled-member
design sheet with all 74 scalars populated; every case asserts its own group's fields by name and at
least one field of a NEIGHBOURING group stays `null` in the diff. `update-member-properties` is the
widest diff (11 `Some`s of 76 fields); `update-plated-inputs` the narrowest (2).
`update-cold-formed-inputs` pins a negative `cf_psi` carried verbatim, and
`update-through-thickness-inputs` pins that a `0.0 → -20.0` move clears the no-op guard.

### `📙️din18599` (12 applied + 1 rejected)
`change-use-class` · `change-heated-area-m2` · `change-occupants` · `change-h-t` · `change-h-v` ·
`change-internal-gains-w-m2` · `change-solar-gains-kwh` · `change-system-losses-kwh` ·
`change-renewable-kwh` · `change-annual-limit-kwh` · `change-energy-carrier` ·
`change-reference-q-p-kwh` · **`update-climate`/refuses-a-negative-january-irradiance (rejected)**.
Every applied case asserts the composed `climate` child slot stays `null` in the diff — only
`update-climate` may write it.

### `📘️en1990` (6 applied + 4 rejected)
`change-annex` (payload lives in the stale `🐷set-snapshot` leaf directory) ·
`change-permanent-action` · `change-resistance` · `change-consequence-class` (the vocabulary's only
RANGE invariant) · `change-seismic-action` (0.0 sentinel → enabled) ·
`insert-variable-action` (content-addressed handle) · plus the four `q_k` rejections listed above.
