# 🔍️ Explore — adding a mutation kind / a schema field to `s.energy.model`, generator drift

Sources studied: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END/🐍️generate-mutation-leaves.py`
(10 210 lines), `📓️mutation-kind-template.md`, `📓️mutation-tag-ledger.md`, the real crate files, and the
existing fenestration/surface/material kinds under
`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`.

## 0. HEADLINE — the generator is stale against the current repo and will half-land if run as-is

`generate-mutation-leaves.py:24-25` hard-codes

```python
CRATE = "✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/🦀️.rs"
MOUNT_PREFIX = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
```

and `mount_block()` (script line ~8998) indents the kind-module block at 32/36 spaces. **Neither
matches the live repo.** The real `pub mod mutations { … }` block lives in
`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs` (464 KB, last touched today), starting at line 815,
indented **20/24/28 spaces**, with bare `#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/…"]`
literals (no `../../🗿️artifacts/🔋️model/` prefix — the file already sits at that root). The 46-line
`📦️packages/🦀️rust/🦀️.rs` the script targets is only plugin/editor/viewer wiring; it contains no
`"mutations"` text at all (verified: `grep fenestration` on it returns nothing).

`rewrite_mounts()` does `source.index(opener)` where `opener` is the 28-space-indented
`'#[path = "."]\npub mod mutations {\n'` literal. Against the wrong file this raises
`ValueError: substring not found`. **`main()` calls `rewrite_mounts()` *after* it has already written
every leaf file and every aggregate file** (`🧬️mutations/🦀️.rs`, `🔣️.json`, `🟦️.ts`, `🔗️.graphql`,
`🛰️.proto`, `📖️.grammar.semio`) — so an unmodified run partially lands (leaves + aggregate files
updated) and then crashes before mounting, before the oracle-catalog rewrite, and before the
`🥒️.feature`/adapter/Python-oracle rewrite. `audit()` (which the docs advertise as "fails loudly on
everything … before a single file is written") does **not** check `CRATE`, so this specific failure
mode is not caught by the custodian gate it advertises.

**Before adding the new kinds, fix in the script**: `CRATE` → `"…/🗿️artifacts/🔋️model/🦀️.rs"`,
`MOUNT_PREFIX` → the bare `"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"` (no `../../…` prefix),
and re-indent `mount_block()`'s `one`/`two` from 32/36 to 24/28 spaces to match line 815's real nesting.
Confirm by reading `sed -n '805,840p' "…/🗿️artifacts/🔋️model/🦀️.rs"` again after the fix — the opener
must appear verbatim.

## 1. Running the generator

- **Command**: `python3 "<ticket>/🐍️generate-mutation-leaves.py"` from the repository root (`ROOT =
  os.getcwd()`, script docstring line 10). No env var needed for the generator pass itself;
  `SEMIO_ENERGY_WRITE_FIXTURES=1` is only for the later `cargo test` pass that materializes the five
  fixture JSONs.
- **`audit()` (line 10063)** runs before any file is written and asserts, per kind in `KINDS`:
  emoji not one of the 8 non-kind siblings (`💾️📖️📝️🔗️🔣️🛰️🟦️🦀️`); emoji ends in exactly one U+FE0F;
  directory NFC-normalized; directory matches `taxonomy.json`'s `mutationDirectoryPattern` (read live
  from the file, never copied); ledger number inside one of the 10 `LEDGER_RANGES` (1-99 … 900-999);
  no two kinds share a number/emoji/slug; `camel(slug) == variant`; at least one `✅️` and one `⛔️`
  fixture case; no two cases share a directory name; every message code in `diff`/`inverse` is one of
  the 7 `FROZEN_MESSAGE_CODES` (`mutation.target-missing`, `mutation.no-op`, `mutation.partial`,
  `mutation.clamped`, `mutation.duplicate-id`, `mutation.invariant`, `mutation.cascade`); every case's
  longest fixture-file path stays ≤ `PATH_UTF16_BUDGET = 227` UTF-16 units (measured via
  `utf16_units()`, not `len()` — astral emoji count double).
- **`report_ledger_drift()`** (line 10102) cross-checks `KINDS` against
  `📓️mutation-tag-ledger.md`'s own rows (parsed with a regex, `LEDGER_ROW`) and only **prints NOTEs**
  — it never fails the run. A stale/missing ledger row is "a bookkeeping debt owed by the group that
  landed the kind."
- **`prune_stale_case_directories()`** (line 10133) deletes a `🧪️tests/<case>/` directory no longer
  declared by its kind's spec row. **Kind directories with no spec row at all are left untouched**,
  only reported: `NOTE: …/<dir> has no spec row (another group's in-flight kind, or a stale rename) —
  left untouched`.
- **Idempotency / no clobber of hand-fixed inverses**: `main()`'s fixture-seeding loop is
  `if not os.path.exists(target): write(f"{case_dir}/{relative}", "{}\n")` — the five committed JSON
  files are **only ever created when absent, never overwritten**. Re-running the generator cannot
  wipe a materialized `(before, mutation, after, diff, outcome)` vector.
- **The two hand-fixed inverses are safe — confirmed in the generator's own source, not just on
  disk**: `🪚️delete-surface`'s spec entry (`number=201`, line ~3890) ends its `inverse=` string with
  `// ↩️ The store replays an inverse in REVERSE order … steps.reverse(); steps` (line 3936-3939), and
  `📕️delete-annual-schedule`'s spec entry (`number=912`, line ~7630) ends the identical way (line
  7668-7671). Both fixes live in the **spec table itself**, not as a post-generation hand patch, so a
  re-run reproduces them byte-for-byte — this is exactly the idempotency property the docs claim, and
  it held under direct inspection.
- **What it does NOT regenerate** (hand-edited, generator-blind): the Rust engine (physics) modules
  under `🔨️modules/⚡️simulation/⚙️engine/` (e.g. `🪟️fenestration/🦀️.rs`, `🧠️precompute`, `🧪️sim`,
  `🏛️bestest`); the `Model`/`Fenestration`/`Surface` struct definitions themselves
  (`🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs`); the epJSON exporter/importer
  (`🚪️io/📤️export/…`, `🚪️io/📥️import/…`); the top-level snapshot schema
  (`✳️any/🧬️schema/📸️snapshot/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}` and its `📝️text/💾️binary` codecs);
  the honeybee-energy second-implementation execution oracle
  (`✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🐍️.py`); the committed BESTEST example assets
  (`🧫️fixtures/🏛️bestest-*/`, `🖼️assets/*/🗣️.dsl.semio`) and the separate top-level
  `✳️any/🧫️fixtures/🧬️mutations/<kind>/<case>/` quintets some (older) kinds use (see §1a). None of
  these are in the generator's write list (`main()`, line ~10162 onward).
- **Safe to add ONE new kind**: yes, mechanically — add one `kind(...)` call to the spec table with a
  fresh ledger number/emoji from `📓️mutation-tag-ledger.md` §2/§5, run it, and (after the `CRATE`/
  `MOUNT_PREFIX`/indentation fix above) it rewrites every aggregate surface and the crate mount
  consistently. It is **not** safe to run it blind today without that fix, because it will throw
  before reaching the mount rewrite, the oracle catalog rewrite, or the feature/adapter/python-oracle
  rewrite.

### 1a. A second, unrelated drift found in passing — fixture file location

`case_component()` (script line 8296) now emits **colocated** `include_str!("📸️snapshot/⬅️before/🔣️.json")`
etc. (relative to the case's own directory), matching `📓️mutation-kind-template.md` §7. But
`⬇️change-fenestration-sill-height/🧪️tests/✅️raises-the-sill/` on disk contains **only `🦀️.rs`** — no
colocated JSON — and that `🦀️.rs` uses the **older** path shape,
`include_str!("../../../../../🧫️fixtures/🧬️mutations/⬇️change-fenestration-sill-height/✅️raises-the-sill/📸️snapshot/⬅️before/🔣️.json")`,
pointing at the separate top-level `✳️any/🧫️fixtures/🧬️mutations/<kind>/<case>/` tree (which does hold
the real 5-file quintet for many landed kinds, e.g. `⚖️change-material-density`,
`💣️delete-plant-loop`, `🕑️change-refrigeration-system-defrost-schedule`). This means at least some
G1/G2/G3/G4-landed kinds were generated by an **earlier version** of the script (top-level fixtures
tree) and have not been touched since the script moved to colocated fixtures. Re-running today's
script would **seed empty `{}` colocated files** beside those `🦀️.rs` files (since the colocated path
doesn't exist yet there) while leaving the old top-level tree's real content orphaned — the `🦀️.rs`
itself would also get rewritten with colocated `include_str!` paths, which would then read the fresh
empty `{}` seeds instead of the real committed vectors, silently emptying those cases' assertions
until `SEMIO_ENERGY_WRITE_FIXTURES=1` is re-run for them. **Flag this to whoever owns those kinds
before a repo-wide re-run** — it is not something adding one new fenestration kind will trigger by
itself (a fresh kind gets colocated fixtures correctly from a clean state), but a full re-run touches
every kind's case file, including these stale ones.

## 2. Full checklist for a brand-new mutation kind (e.g. `change-fenestration-vertices`)

Per `📓️mutation-kind-template.md` §0-§9 and the generator's own `main()`:

1. **Ledger row** — `📓️mutation-tag-ledger.md` §5, table "Geometry & envelope — 200-299": take the
   next free number in that block (currently ends at 235 `change-fenestration-fin-offset`; next free
   is 236) and a fresh, U+FE0F-suffixed emoji not already in §5 or the 8 reserved siblings.
2. **Spec table entry** in `generate-mutation-leaves.py` — a `g1_window_scalar(...)` (or hand-rolled
   `kind(...)`) call with `number`, `slug`, `emoji`, `record`, `display`, `doc`, `field`, `ty`, `unit`,
   `label`, `probe`, `happy`/`refusal` case tuples. This one call drives every file below.
3. Generator emits, under `🧬️mutations/<emoji><slug>/`: `🦀️.rs` (payload struct + builder +
   `MutationKind` impl), `🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`, `🔣️.json` (leaf descriptor,
   `"binaryTag": null` always), `🧬️.schema.json` (payload JSON Schema, `title == aggregateVariant`),
   and per fixture case `🧪️tests/<case>/🦀️.rs` + 5 seeded `{}` JSON files.
4. **Aggregate `🧬️mutations/🦀️.rs`**: one `pub use super::<module>::{<builder>, <Variant>};`
   (alphabetical, region `🔖️Reexports`), one `<Variant>(<Variant>),` enum arm (ledger order), one
   `"<slug>",` in `KINDS` (same order), one `("<slug>", "<emoji><slug>"),` in `DIRECTORIES` (same
   order), one `wire_probes()` literal.
5. **Aggregate mirrors**, all ledger-ordered: `🧬️mutations/🔣️.json` (`oneOf` ref + `$defs` entry),
   `🧬️mutations/🟦️.ts` (interface + union member), `🧬️mutations/🔗️.graphql` (type + union member),
   `🧬️mutations/🛰️.proto` (`message` + `oneof kind` field **using the ledger number as the protobuf
   field number, forever stable**), `🧬️mutations/📖️.grammar.semio` (one `op = … | <slug>` alternative +
   production).
6. **Crate mount** — one `pub mod <module> { … 4 `#[path]` mounts (component/diff/inverse/tests) … }`
   block, ledger-ordered, inside `pub mod mutations { … }`. **Today this must land in
   `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs`, not the `📦️packages/🦀️rust/🦀️.rs` the script
   currently targets** — see §0.
7. **Oracle registration**, `✳️any/🔮️oracle/🔣️.json` (3 edits): `mutationCatalogs[0].vectors[]` entry
   (`mutationId`, `sourceMutationDirectoryName`, `mutationDirectoryName`, `scenarios[]`),
   `mutationCatalogs[0].kinds[]` slug, `mutationManifests[0].mutations[]` entry (`id`, `capability:
   "energy-model-1-mutate"`, `payloadSchema`, `outcomes`, `productionDispatch`,
   `oracleRequirements`). The generator's `rewrite_oracle_catalog()` does all three from `vectors()`.
8. **Second implementation**, `✳️any/🧪️tests/🏛️mutate-energy-model-1/`: one `VECTOR_ROOTS` entry per
   case (`asset://🧬️schema/🧬️mutations/<emoji><slug>/🧪️tests/<case>`) in `🐍️.py`, one Python function
   written from the payload schema (never transliterated from Rust), one `VOCABULARY` row, one
   `invert()` arm; two `Examples` rows per case in `🥒️.feature`; one `Vector { … }` per case in the
   Rust adapter `🦀️.rs`. The generator's `python_oracle()`/`feature()`/`adapter_rs()` write all three
   files wholesale from `KINDS`.
9. **Fixture materialization**: seed the five case JSONs with `{}` (generator does this automatically
   for a fresh kind), then `SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test -p semio-s-plugin-energy
   <case-name>` (every other test in the case fails on this pass — expected), then re-run `cargo test`
   without the var; all ten per-case laws (`fixtures::assert_*`, defined once in `#[cfg(test)] pub mod
   fixtures` inside the aggregate `🦀️.rs`) must pass against the committed bytes.
10. **Gates**: `bun nx run verify-taxonomy-report`, `bun nx run verify-taxonomy-enforce`,
    `bun nx run mutation-outcome-law`, then `cargo check -p semio-s-plugin-energy --lib --tests` and
    `cargo test -p semio-s-plugin-energy <slug>` (with `RUSTC_WRAPPER=` and a private
    `CARGO_TARGET_DIR`, per §10 of the template — see memory
    `project-shared-cargo-build-dir-fine-grain-locking.md` for why a private `CARGO_TARGET_DIR` is
    itself against house rules; use the shared build-dir instead).

## 3. Full checklist for a NEW OPTIONAL FIELD on `Fenestration`

`Fenestration` is declared at `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs:390-406`:

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct Fenestration {
    pub id: EntityId, pub name: String, pub surface_id: EntityId,
    pub u_value_w_m2k: f64, pub shgc: f64, pub vlt: f64, pub area_m2: f64,
    pub height_m: f64, pub sill_height_m: f64,
    pub frame_conductance_w_k: f64, pub divider_conductance_w_k: f64,
    pub overhang_depth_m: f64, pub overhang_offset_m: f64,
    pub fin_depth_m: f64, pub fin_offset_m: f64,
    pub glazing_construction_id: Option<EntityId>,
}
```

16 fields today, no vertex/polygon field. `ToValue`/`FromValue` are struct-level derives — adding a
field is automatic there (no manual arm to add), **but every call site that reconstructs a
`Fenestration` positionally breaks and must gain the new parameter**:

1. **The struct itself** — add e.g. `pub polygon_m: Option<Vec<[f64; 3]>>,` (immediately usable:
   `Vec<[f64; 3]>` and `Option<T>` are already in the generator's `JSON_TYPES`/`PROTO_TYPES` maps as
   `"Vec<[f64; 3]>"`/`"Option<crate::model::EntityId>"`-shaped entries — a `Vec<[f64;3]>`-of-`Option`
   combination needs one more map entry, `"Option<Vec<[f64; 3]>>"`, added to both dicts if any
   generated mutation payload carries it directly).
2. **Every positional reconstruction site** (grep confirms exactly these, outside the ticket folder):
   - `create_fenestration(...)` builder + its `MutationKind` impl in
     `🧬️mutations/🪟️create-fenestration/🦀️.rs` — the literal in the generator's `g1_create(...)` spec
     call (`literal=` string, ~15 named fields) needs the new field appended (with a sensible
     default/`None` for backward payload compatibility) if `create-fenestration` should be able to set
     it; if not, it can stay implicit `None` in the literal.
   - `🚪️delete-fenestration/↩️inverse/🦀️.rs` — its `undo=` string re-lists every field positionally
     to rebuild the deleted entity; must append the new field.
   - `🪚️delete-surface/↩️inverse/🦀️.rs` — cascades fenestrations hosted on the deleted surface via
     `super::create_fenestration(window.id, …, window.glazing_construction_id)`; must append the new
     field to this call too (found directly in the generator's `inverse=` string for `number=201`).
   - The Python mirrors of both (`py_undo`/`py_literal` dict strings) in the same spec calls.
3. **A NEW mutation kind, if the field needs its own edit verb** (per the ticket's own
   `change-fenestration-vertices` example) — follow the full §2 checklist above; this is the
   recommended path for a polygon that should be independently editable rather than only settable at
   creation.
4. **DSL codec of the snapshot** — checked directly: the top-level snapshot schema
   (`✳️any/🧬️schema/📸️snapshot/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`) treats `"model"` as an **opaque**
   `"type": "object", "x-semio-state": "artifact"` blob (verified: `grep Fenestration` on all four
   files returns nothing), and the `📝️text/🦀️.rs` / `💾️binary/🦀️.rs` codecs under that same
   `📸️snapshot/` directory never mention `model`/`Model` either (both files' only "model" hit is their
   own doc-comment header). **So there is no snapshot-level schema/codec surface to touch for a
   Fenestration field** — `Model`'s own `Serialize`/`Deserialize` + `pack::json`/`ToValue` do the
   whole job, opaque to the snapshot wrapper. Only the field-count law below is model-level, not
   nested-struct-level.
5. **epJSON exporter/importer — the two sites that matter most for a polygon field**, both already
   study "declared convention #2" (a rectangle synthesized from `area_m2`/`height_m`/`sill_height_m`,
   never real vertices):
   - Export: `aperture_rectangle()` in
     `…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/⚡️epjson/🔖️25.2/✳️any/🦀️.rs:199-224` — computes 4
     corners from the host surface's basis + `window.area_m2`/`height_m`/`sill_height_m`. A polygon
     field should make this function **prefer real vertices when `Some`, fall back to the synthesized
     rectangle when `None`** — exactly the shape "optional field, old documents keep working" implies.
   - Import: `decode_apertures()` in
     `…/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/⚡️epjson/🔖️25.2/✳️any/🦀️.rs:455-509` — explicitly
     **refuses non-rectangular apertures** (`"only four-vertex rectangular apertures decode into a
     semio Fenestration"`, line 474) and always constructs `Fenestration { …, glazing_construction_id:
     None }` positionally (line 499) — needs the new field appended, and could stop refusing
     non-rectangular apertures once it has somewhere to put the extra corners.
6. **Committed BESTEST example assets** — `🖼️assets/*/🗣️.dsl.semio` (the ticket brief references
   these; not directly located under the energy plugin subset in this pass, likely elsewhere under a
   demo/showcase artifact) regenerated via `SEMIO_ENERGY_BESTEST_REGENERATE=1`. Confirmed live:
   `🔨️modules/⚡️simulation/⚙️engine/🏛️bestest/🧪️tests/🔬️unit/🦀️.rs:376-422` — "THE generator.
   Deliberately inert unless `SEMIO_ENERGY_BESTEST_REGENERATE` is set," and its assertion messages say
   `"committed fixture for case {case} is stale — rerun with SEMIO_ENERGY_BESTEST_REGENERATE=1"` /
   same for `"committed asset for case {case}"`. Run the bestest unit test suite with that env var set
   after any change to `Fenestration`'s shape or to the exporter's aperture geometry, then re-run
   without it to confirm the committed bytes are stable.
7. **Committed fixture quintets** for every existing fenestration-touching kind
   (`create-fenestration`, `delete-fenestration`, `delete-surface`, `change-fenestration-*`) —
   regenerate via `SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test -p semio-s-artifact-energy-model --lib --
   writes_the_committed_vector_when_requested` (crate name per the ticket brief; the ticket's own
   template says `-p semio-s-plugin-energy` — confirm which crate currently owns these tests before
   running, they may have diverged post-refactor per §0's finding).
8. **Python oracle(s)** — two, not one:
   - `✳️any/🧪️tests/🏛️mutate-energy-model-1/🐍️.py`'s `VOCABULARY` (mutation-semantics oracle,
     generator-owned) — only touched if a new mutation kind reads/writes the field (§3).
   - `✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🐍️.py` (the **honeybee-energy BESTEST
     second-implementation** oracle — a different oracle entirely, not generator-owned, no
     `VOCABULARY` dict in it at all). Its `_add_apertures()` (line 207-219) is the **same declared
     convention #2 rectangle-synthesis** as the epJSON exporter, driven by `_APERTURE_ASPECT`/
     `_APERTURE_SILL_M`, `face.apertures_by_ratio_rectangle(...)`. If real vertices are added to
     `Fenestration`, this function should gain the same "use real geometry when present" branch for
     the two oracles to stay comparable. **The task prompt's phrase "the python oracle VOCABULARY in
     `🔮️oracles/🏃️execution/🐍️.py`" does not match what's on disk — that file has no `VOCABULARY`
     symbol; the mutation VOCABULARY lives in the generator-owned file above.** Flag this back if the
     inspector work depends on which oracle is meant.
9. **Field-count law does NOT need touching for a Fenestration field**: confirmed at
   `✳️any/🧬️schema/💡️inferences/🗃️entries/🧪️tests/🔬️unit/🦀️.rs:14`, `const MODEL_FIELD_COUNT: u32 =
   44;` — the doc comment enumerates exactly `Model`'s 44 **top-level** fields (`name/version/site/
   zones/spaces/surfaces/fenestrations/materials/glazingMaterials/gasMaterials/constructions/…`).
   `fenestrations` is one of those 44 regardless of how many fields `Fenestration` itself has; nested
   struct fields are not counted. Adding a field to `Fenestration` changes nothing here. Adding a
   **new top-level `Model` field** would (and is the one place a mutation-kind addition could
   silently break a hard-coded count if it ever added a full new collection).

## 4. Existing kinds already covering fenestration/surface/material/zone/site edits

All from `📓️mutation-tag-ledger.md` §5 (ledger numbers verified against the actual ledger rows, status
column says `LANDED`):

**Fenestration** (host: `create-fenestration`/`delete-fenestration` carry every field at creation —
see §3 literal):
| kind | # | payload field(s) |
|---|---|---|
| `change-fenestration-u-value` | 215 | `new_u_value_w_m2k: f64` |
| `change-fenestration-shgc` | 216 | `new_shgc: f64` (0..=1) |
| `change-fenestration-vlt` | 217 | `new_vlt: f64` (0..=1) |
| `change-fenestration-area` | 218 | `new_area_m2: f64` |
| `change-fenestration-height` | 230 | `new_height_m: f64` |
| `change-fenestration-sill-height` | 231 | `new_sill_height_m: f64` |
| `change-fenestration-frame-conductance` | 219 | `new_frame_conductance_w_k: f64` |
| `change-fenestration-divider-conductance` | 220 | `new_divider_conductance_w_k: f64` |
| `change-fenestration-overhang-depth` | 232 | `new_overhang_depth_m: f64` |
| `change-fenestration-overhang-offset` | 233 | `new_overhang_offset_m: f64` |
| `change-fenestration-fin-depth` | 234 | `new_fin_depth_m: f64` |
| `change-fenestration-fin-offset` | 235 | `new_fin_offset_m: f64` |
| `bind-fenestration-glazing-construction` | 228 | attaches `glazing_construction_id: EntityId` (the layered stack that supersedes u/shgc/vlt) |
| `clear-fenestration-glazing-construction` | 229 | detaches it |
| `change-fenestration-surface` | 214 | rehosts to a new `surface_id: EntityId` |
| `rename-fenestration` | 213 | `new_name: String` |
| `create-fenestration` / `delete-fenestration` | 211/212 | full entity, see §3 literal |

**No `change-fenestration-vertices` exists** — this is genuinely new territory the ticket's own
`🪟️` example names correctly; `create-fenestration`/`delete-fenestration` are the only kinds that
touch geometry-adjacent state today, and only via the scalar area/height/sill trio, never real
vertices.

**Surface**:
| kind | # | fields |
|---|---|---|
| `create-surface` | 200 | full entity: `id, name, zone_id, class, vertices (Vec<[f64;3]>), construction_id, boundary kind, interzone partner, sun_exposed, wind_exposed, multiplier` |
| `delete-surface` | 201 | `id` (cascades fenestrations + adjacency pairs, restricts on interzone-partner reference) |
| `rename-surface` | 202 | `new_name` |
| `change-surface-zone` | 203 | `new_zone_id` |
| `change-surface-class` | 204 | `new_class` (enum) |
| `replace-surface-vertices` | 205 | `new_vertices: Vec<[f64; 3]>` — the direct precedent for a fenestration polygon field/kind |
| `change-surface-construction` | 206 | `new_construction_id` |
| `change-surface-boundary-condition` | 207 | new outside-boundary kind (+ partner when interzone) |
| `change-surface-sun-exposed` | 208 | `bool` |
| `change-surface-wind-exposed` | 209 | `bool` |
| `change-surface-multiplier` | 210 | `u32` |

**Material** (300-309): `create-material`/`delete-material`/`rename-material` (300-302), then one
scalar kind each for `thickness` (303), `conductivity` (304), `density` (305), `specific-heat` (306),
`thermal-absorptance` (307), `solar-absorptance` (308), `visible-absorptance` (309).

**Construction** (310-315): `create-construction`/`delete-construction`/`rename-construction`
(310-312), `add-construction-layer`/`remove-construction-layer`/`reorder-construction-layers`
(313-315).

**Glazing material / gas material — NO mutation kind exists at all.** `grep -i glazing-material` and
`grep -i GlazingMaterial` across both the ledger and the generator's spec table return nothing.
`Model.glazingMaterials`/`gasMaterials` are two of the 44 top-level fields (added after the original
40, per the `MODEL_FIELD_COUNT` doc comment) but nothing in the current vocabulary can create, delete
or edit an entry in either collection. If the inspector needs to edit glazing/gas material properties
directly (as opposed to only binding a `Construction` via `bind-fenestration-glazing-construction`),
that is new-kind work, following the `create-material`/`change-material-*` shape as the closest
precedent.

**Zone**: `rename-zone` (100), `change-zone-volume` (101), `change-zone-multiplier` (102),
`change-zone-conditioned` (103), `change-zone-floor-area-participation` (104), plus
`create-zone`/`delete-zone` (105/106).

**Site**: `update-site` (3) — one inseparable 5-field facet (`latitudeDeg`, `longitudeDeg`,
`elevationM`, `timeZoneHours`, `northAxisDeg`), all required every time (verified in the generator's
own `update_site()` Python oracle function, line 9607-9621); there is no per-field site kind.
