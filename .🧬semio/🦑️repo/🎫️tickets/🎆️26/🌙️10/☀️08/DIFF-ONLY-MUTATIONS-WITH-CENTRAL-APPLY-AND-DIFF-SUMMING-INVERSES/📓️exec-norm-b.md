# 📓️ exec-norm-b — en1993, din4108, din16798, en1990, en1998 (192 kinds)

Status: **WRITTEN BUT UNVERIFIED — nothing compiled, no Rust test run.** Every `cargo check` attempt died before reaching the norm crates
(foundation RED: `semio-framework-value` `ErasedSnapshotRetirement` half-change, then `ui-scene`/wgpu `close_step` errors; fine-grain-lock
deadlock killed one run; gate waits of 30-45 min). Foundation still RED at last read (`SnapshotRetirementStep` unresolved import in os-store hydration).
Run when `foundation.status` is GREEN, per artifact workspace (`cd ✏️s/🔌️plugins/📕️norm/🗿️artifacts/<artifact>` then
`gate cargo check --target wasm32-wasip2`, then `cargo test --lib mutations`; en1993 also needs the new `protocol-laws` dev-dep, already added).
Verified without cargo: `bun ./📜️script.ts verify mutation-outcome-law` prints **0** breaches in the five artifacts (incl. R15 per-leaf sum-law);
the Python apply oracle (`T/🗑️generated/norm-b/oracle_apply_diff.py`) applies all **210** committed diff vectors to their before-snapshots and reaches every after-snapshot;
the 210 diff fixtures and the four diff schema twins use the positional wire shape.

## Design (final)

All five diff types now sit on the framework's shared POSITIONAL list delta (wave 5, AMB-1; no `order`, no `after` anchors; fw-spine promoted norm-a's registry module to `protocol::list_delta!` / `protocol::row_patch!` and migrated the five diff files, `commit_onto` now takes the capability; the leaves only use `Delta::{insertion,removal,relocation,modification}` and were not affected):
`<A>Diff { scalars: Option<T>…, <coll>: <A><Row>Delta { removed: [{id, index}], inserted: [{index, row}], moved: [{id, from, to}], modified: [{id, patch}] } }`; a row patch is
sparse (`Option` per field, nested positional delta per owned list: din4108 zone→windows, element→layers; en1998 building→systems/storeys/members).
`apply` assigns field-wise through `commit_onto` (no clone-and-write of a list), `absorb` is field-wise + the module's base-free absorb, `DiffAlgebra::{inverse,between,is_empty}` concrete.
Deleted: `list_wrap!` + 16 `En1993*List`, `apply_in_place` ×33 (din4108), `apply_fields`/`apply_to_artifact` (en1993/en1990/en1998), the `artifact: Option<Box<Artifact>>` diff field (en1993/en1998/din16798), `En1990StringList`, all `Din*List` wrappers.
Leaves build their delta declaratively: `Delta::insertion(clamped_index, row)`, `Delta::removal(&base.list, index)`, `Delta::modification(&row.id, Patch { f: Some(v), ..Default::default() })`, replace = `removal` absorb `insertion` at the same index, layer reorder = `Delta::relocation(&element.layers, from, to)`; din16798 remove-by-id resolves the base index first. Inverse leaves were already concrete absolute setters / index insert-remove (restore at the ORIGINAL index) — unchanged except en1990 (below).
`apply_<artifact>_mutation` and `inverse_<artifact>_mutation` live in `🚪️io/🦀️.rs` (`pub mod mutation_bridge`, re-exported from the mutations aggregate; gate R9), as do io `Document::mutate/absorb` and tests: `protocol::apply_diff`.
Only id-less rows keep a positional edit script: **en1990 `effects`** (`MemberEffect` has no key; a repeated member/action pair is legal) — a small private row engine in the diff file whose `invert_rows` simulates over `Cow` (no ref-param clone, gate R12).

Wave-2/3: inverse rows are stored in reverse (store replays `.rev()`); every fold in the leaf, fixture and differential tests now replays `.iter().rev()`. en1990 `change-<list>` kinds invert to explicit `Remove`×n (index 0) + `Insert`×m rows (no whole-collection restore; stored reversed); `change-bridge-sls` has no insert/remove kinds in the vocabulary — it keeps the self-setter inverse (**open: needs insert/remove-bridge-sls kinds to drop the restore shape**).
Middle-row law tests: `🧬️mutations/🧪️tests/🔬️middle-row/🦀️.rs` ×5 (remove/insert at index 1 of every top-level list, din4108 layer/window/reorder), asserting the sum law and that the inverse re-inserts at index 1 — programmatic padding, **no committed middle-row fixture bundles** (open).
No `set-snapshot`/`from_snapshot`: deleted the `📤️set-snapshot` + `🎨️set-active-example` command dirs, their `#[path]` mods, `app_commands!` rows, `tools:` lists, `ActionDefinition`/describe/destructive/interactive registrations, `import_media(port, media)` without wrap, the `from_snapshot` mutation differ and its tests, editor tests (undo/redo now a concrete `SetField`), results-window test priming; examples load through the catalogue route.
`EDIT_RULES` (`🧬️mutations/🧭️edit-rules/🦀️.rs`) per artifact, generated from the leaf payloads: en1993 1 set / 16 insert / 16 remove; din4108 32/5/5; din16798 37/2/0; en1990 11/6/6; en1998 12/8/8; all four editor handlers resolve through it.
Limitations (feature loss to review): fields without a concrete setter kind are not settable through `setField` any more (en1993 rows are replaced via `update-*-inputs`, so only `annex` has a rule); din16798 `remove-zone`/`remove-vent-system` address by id, which `RemoveItemRule` cannot express (no remove rule); `update-*`/`reorder-layers` have no value-tree path.

Behaviour changes: inserts reject nothing new (duplicate-id checks already existed); setter diffs no longer emit a whole list; `remove-zone`/`remove-vent-system` remove the first row with the id.

## Tests and fixtures

- Every `✅apply` leaf test calls `protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law` (en1993 49, en1998 29, din16798 41, din4108 43, en1990 30 — via `committed_inputs` helper, call inline in the leaf).
- `🔺️diff/🧪️tests/🔬️unit` ×5: `between`/`inverse`/`absorb`/empty laws via `assert_diff_algebra_*` / `assert_mutation_diff_absorb_law` over a document changed in every collection.
- 210 committed `🔺️diff/🔣️.json` regenerated by the independent Python writer `T/🗑️generated/norm-b/fixtures.py`/`fx_all.py` (computes the positional delta from mutation + before only) and checked by `oracle_apply_diff.py` (an interpreter of norm-a's apply rule: validate ids at base indices, slots, survivors fill in base order, then patches); the Rust vector tests have not compared them yet.
- Follow-ups (wave 6): (1) `🔬️middle-row` committed bundles (mutation, padded three-row before, after, diff, outcome) for all 38 delete/move kinds (en1993 16, din4108 6 incl. nested remove-layer/remove-zone-window and reorder-layers 1 to 2, din16798 2, en1990 6, en1998 8), each with a leaf test module `🧪️tests/🔬️middle-row` (committed diff, after, inverse restores, outcome, sum law) mounted from the fixture module, and registered in the oracle manifest, the feature apply table, the Python `VECTORS` and Rust `ROWS`; derived by `middle_fixtures.py`, checked by `oracle_apply_diff.py` (248 vectors, 0 failures) and by the shared independent Python engine (`py_engine_check.py`: apply and inverse of all 38, 0 failures). (2) din16798 remove-by-id: `crate::mutations::resolve_edit` (edit-rules file) resolves `removeItem` on `zones` / `ventSystems` (`zones`, `zones[n]`, `zones[id=...]`) to the by-id kinds before falling back to `EDIT_RULES`; the remove-item editor handler uses it; no feature loss. (3) Insert payload `index` is `Option<usize>` in all 37 insert leaves (absent = append, past-the-end still clamps with `mutation.clamped`): leaf structs, diffs (`unwrap_or(usize::MAX).min(len)`), insert inverses (remove at the landing index), remove inverses (`Some(index)`), leaf JSON schemas (not required, `integer|null`), TS twins (`normWireOptional(normWireNullable(..))`), din4108 text DSL variants, hand-written text codecs (absent `index=` parses to None; hand binary codecs go through the JSON value so `Option` needs no change); `inserting_without_a_position_appends_the_*` tests added to each middle-row unit test file. (4) `change-bridge-sls` confirmed: a replace kind (carries the whole new list) whose inverse is the same kind carrying the base list, so a concrete setter, not a restore; its diff still replaces every row (removed all, inserted all) rather than a sparse patch. (5) `bun ./📜️script.ts schema generate` ran (3658 scopes); `schema check` shows the same diagnostic classes as untouched norm artifacts (graphql export-incomplete, ref-unresolved), more of them for the larger diff export lists.
- Not done: committed append-without-index (`index` absent) bundles; differential inverse rows for the middle-row vectors (extra rows are registered `mutate-` only, like dupe/noop); the generator scripts' diff template (`diffgen2.py`) still emits the old registry macro names, so `write_diff()` must not be re-run.
- Dev-dependency `semio-framework-os-kernel` + `protocol-laws` added to en1993 `Cargo.toml` (others already had it).
- Diff schemas: `🔺️diff/{🔣️.json,🟦️.ts,🛰️.proto,🔗️.graphql}` ×5 regenerated (`schemagen2.py`); the repo-wide `📓️schema-catalog.md`/`🔣️schema-catalog.json` listing was not regenerated.

## Per-kind classification (before → after)


### 🔩️en1993

| kind | before | after |
|---|---|---|
| `↕️update-through-thickness-inputs` | clone-and-write, whole-list diff | positional replace (`removed` at base index + `inserted` at same index) |
| `✨️update-stainless-inputs` | clone-and-write, whole-list diff | positional replace (`removed` at base index + `inserted` at same index) |
| `➕️insert-bridge-fatigue` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➕️insert-cold-formed-member` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➕️insert-crane-runway` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➕️insert-fatigue-detail` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➕️insert-fire-exposure` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➕️insert-joint` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➕️insert-load-case` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➕️insert-material` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➕️insert-member` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➕️insert-member-action` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➕️insert-pile` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➕️insert-plated-panel` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➕️insert-section` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➕️insert-silo-shell` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➕️insert-tension-component` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➕️insert-tower-leg` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➖️remove-bridge-fatigue` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-cold-formed-member` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-crane-runway` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-fatigue-detail` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-fire-exposure` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-joint` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-load-case` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-material` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-member` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-member-action` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-pile` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-plated-panel` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-section` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-silo-shell` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-tension-component` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-tower-leg` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `⬜️update-hss-inputs` | clone-and-write, whole-list diff | positional replace (`removed` at base index + `inserted` at same index) |
| `🌉️update-bridge-inputs` | clone-and-write, whole-list diff | positional replace (`removed` at base index + `inserted` at same index) |
| `🌍️change-annex` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `🏗️update-crane-inputs` | clone-and-write, whole-list diff | positional replace (`removed` at base index + `inserted` at same index) |
| `📊️update-member-properties` | clone-and-write, whole-list diff | positional replace (`removed` at base index + `inserted` at same index) |
| `🔁️update-fatigue-inputs` | clone-and-write, whole-list diff | positional replace (`removed` at base index + `inserted` at same index) |
| `🔥️update-fire-inputs` | clone-and-write, whole-list diff | positional replace (`removed` at base index + `inserted` at same index) |
| `🔩️update-bolt-inputs` | clone-and-write, whole-list diff | positional replace (`removed` at base index + `inserted` at same index) |
| `🗼️update-tower-inputs` | clone-and-write, whole-list diff | positional replace (`removed` at base index + `inserted` at same index) |
| `🛢️update-silo-shell-inputs` | clone-and-write, whole-list diff | positional replace (`removed` at base index + `inserted` at same index) |
| `🥶️update-cold-formed-inputs` | clone-and-write, whole-list diff | positional replace (`removed` at base index + `inserted` at same index) |
| `🧱️update-plated-inputs` | clone-and-write, whole-list diff | positional replace (`removed` at base index + `inserted` at same index) |
| `🧲️update-weld-inputs` | clone-and-write, whole-list diff | positional replace (`removed` at base index + `inserted` at same index) |
| `🪢️update-tension-component-inputs` | clone-and-write, whole-list diff | positional replace (`removed` at base index + `inserted` at same index) |
| `🪵️update-pile-inputs` | clone-and-write, whole-list diff | positional replace (`removed` at base index + `inserted` at same index) |

### 🧱️din4108

| kind | before | after |
|---|---|---|
| `↔️change-element-adjacent` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | keyed row field patch (`modified{key, patch}`) |
| `↔️change-thermal-bridge-length` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | keyed row field patch (`modified{key, patch}`) |
| `☀️change-zone-window-g-value` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | nested keyed field patch |
| `⛱️change-zone-window-shading-fc` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | nested keyed field patch |
| `✅️change-bb2-details-conform` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `➕️insert-layer` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | nested positional `moved` row inside a patched parent row |
| `➕️insert-zone` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➖️remove-layer` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | nested positional `moved` row inside a patched parent row |
| `➖️remove-zone` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | positional removal (`removed` id at base index) |
| `🌉️insert-thermal-bridge` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `🌙change-zone-night-ventilation` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | keyed row field patch (`modified{key, patch}`) |
| `🌡️change-layer-lambda` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | nested keyed field patch |
| `🌡️change-t-int-c` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `🌦️change-climate-zone` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `🏠️insert-element` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `🏷️change-element-kind` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | keyed row field patch (`modified{key, patch}`) |
| `🏷️change-layer-application-type` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | nested keyed field patch |
| `🏷️change-layer-compressive-class` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | nested keyed field patch |
| `🏷️change-thermal-bridge-bb2-type` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | keyed row field patch (`modified{key, patch}`) |
| `💧change-layer-mu` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | nested keyed field patch |
| `💧️change-rh-int` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `💨change-has-mechanical-ventilation` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `💨️change-airtightness-n50` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `📈️change-element-delta-uf` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | keyed row field patch (`modified{key, patch}`) |
| `📈️change-element-delta-ug` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | keyed row field patch (`modified{key, patch}`) |
| `📈️change-element-delta-ur` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | keyed row field patch (`modified{key, patch}`) |
| `📏change-zone-window-area` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | nested keyed field patch |
| `📏️change-layer-thickness` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | nested keyed field patch |
| `📐change-element-inclination-deg` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | keyed row field patch (`modified{key, patch}`) |
| `📐change-zone-window-inclination-deg` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | nested keyed field patch |
| `📐️change-element-area` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | keyed row field patch (`modified{key, patch}`) |
| `📐️change-zone-floor-area` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | keyed row field patch (`modified{key, patch}`) |
| `🔀️reorder-layers` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | nested positional `moved` row inside a patched parent row |
| `🔘change-thermal-bridge-psi` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | keyed row field patch (`modified{key, patch}`) |
| `🗂️change-usage` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `🚫️remove-element` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | positional removal (`removed` id at base index) |
| `🚫️remove-zone-window` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | nested positional `moved` row inside a patched parent row |
| `🧊remove-thermal-bridge` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | positional removal (`removed` id at base index) |
| `🧭change-element-orientation-deg` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | keyed row field patch (`modified{key, patch}`) |
| `🧭change-zone-window-orientation` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | nested keyed field patch |
| `🧱change-zone-heaviness` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | keyed row field patch (`modified{key, patch}`) |
| `🧽️change-layer-material-id` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | nested keyed field patch |
| `🪟insert-zone-window` | `apply_in_place(&mut Snapshot)` clone-and-write, 3 whole lists | nested positional `moved` row inside a patched parent row |

### 🌬️din16798

| kind | before | after |
|---|---|---|
| `☀️change-zone-t-op-summer` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `♻️change-vent-heat-recovery` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `⚙️change-vent-system-type` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `❄️change-zone-t-op-winter` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `➕️insert-zone` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➖️remove-zone` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `🆕️insert-vent-system` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `🌀change-cellar-ventilation` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `🌀️change-vent-sfp` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `🌍️change-annex` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `🌙️change-night-setback` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `🌫️change-outdoor-co2` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `🌬️change-vent-design-airflow` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `🎓️change-vent-sfp-class` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `🏃️change-zone-metabolic-rate` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `🏚️change-cellar-area` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `🏞️change-vent-oda-class` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `🏠️change-envelope-n50` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `🏢️change-zone-usage-type` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `🏭️change-zone-pollution-class` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `👔change-zone-clothing` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `👥️change-zone-occupants` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `💡change-zone-illuminance` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `💧️change-zone-rh` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `💨change-zone-air-speed` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `💨change-zone-turbulence` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `💨️change-zone-outdoor-air` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `📅️change-vent-inspection` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `📐️change-zone-floor-area` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `📐️change-zone-vent-method` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `📦️change-envelope-volume` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `🔄️change-theta-rm` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `🔊️change-zone-noise` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `🔗change-zone-vent-system-id` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `🕳️change-vent-duct-leakage` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `🗑️remove-vent-system` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `🛋️change-zone-comfort-category` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `🧭️change-zone-comfort-model` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `🧱change-vent-duct-class` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `🧽change-vent-filter-sup` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `🫧change-zone-co2` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |

### ⚖️en1990

| kind | before | after |
|---|---|---|
| `⏱️change-reference-period-years` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `⚓️change-permanents` | scalar sparse diff (already clean); inverse restores `new_<collection>: base.<collection>.clone()` | positional whole-list script (`removed` all base rows, `inserted` indexed new rows) |
| `⚠️change-consequence-class` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `⛰️change-altitude-m` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `✂️remove-effect` | clone-and-write, whole-list diff; inverse restores `new_<collection>: base.<collection>.clone()` | positional removal (`removed` id at base index) |
| `➕insert-permanent` | clone-and-write, whole-list diff; inverse restores `new_<collection>: base.<collection>.clone()` | positional insertion (`inserted` at index) |
| `➖remove-permanent` | clone-and-write, whole-list diff; inverse restores `new_<collection>: base.<collection>.clone()` | positional removal (`removed` id at base index) |
| `🌉change-bridge-sls` | scalar sparse diff (already clean); inverse restores `new_<collection>: base.<collection>.clone()` | positional whole-list script (`removed` all base rows, `inserted` indexed new rows) |
| `🌋insert-seismic` | clone-and-write, whole-list diff; inverse restores `new_<collection>: base.<collection>.clone()` | positional insertion (`inserted` at index) |
| `🌋️change-seismics` | scalar sparse diff (already clean); inverse restores `new_<collection>: base.<collection>.clone()` | positional whole-list script (`removed` all base rows, `inserted` indexed new rows) |
| `🌍️change-annex` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `🎯change-reliability-class` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `🏋️change-variables` | scalar sparse diff (already clean); inverse restores `new_<collection>: base.<collection>.clone()` | positional whole-list script (`removed` all base rows, `inserted` indexed new rows) |
| `🏗️change-members` | scalar sparse diff (already clean); inverse restores `new_<collection>: base.<collection>.clone()` | positional whole-list script (`removed` all base rows, `inserted` indexed new rows) |
| `🏷️change-project-id` | scalar sparse diff (already clean); inverse restores `new_<collection>: base.<collection>.clone()` | sparse scalar field (unchanged) |
| `👁️change-supervision-level` | scalar sparse diff (already clean); inverse restores `new_<collection>: base.<collection>.clone()` | sparse scalar field (unchanged) |
| `💣insert-accidental` | clone-and-write, whole-list diff; inverse restores `new_<collection>: base.<collection>.clone()` | positional insertion (`inserted` at index) |
| `💥change-accidentals` | scalar sparse diff (already clean); inverse restores `new_<collection>: base.<collection>.clone()` | positional whole-list script (`removed` all base rows, `inserted` indexed new rows) |
| `📅change-design-working-life-category` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `📆change-design-working-life-years` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `📎insert-effect` | clone-and-write, whole-list diff; inverse restores `new_<collection>: base.<collection>.clone()` | positional insertion (`inserted` at index) |
| `📐change-beta-computed` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `📤remove-variable` | clone-and-write, whole-list diff; inverse restores `new_<collection>: base.<collection>.clone()` | positional removal (`removed` id at base index) |
| `📥insert-variable` | clone-and-write, whole-list diff; inverse restores `new_<collection>: base.<collection>.clone()` | positional insertion (`inserted` at index) |
| `🔍change-inspection-level` | scalar sparse diff (already clean); inverse restores `new_<collection>: base.<collection>.clone()` | sparse scalar field (unchanged) |
| `🔗change-effects` | scalar sparse diff (already clean); inverse restores `new_<collection>: base.<collection>.clone()` | positional `Remove*`/`Insert*` script (id-less rows) |
| `🔩insert-member` | clone-and-write, whole-list diff; inverse restores `new_<collection>: base.<collection>.clone()` | positional insertion (`inserted` at index) |
| `🕳️remove-seismic` | clone-and-write, whole-list diff; inverse restores `new_<collection>: base.<collection>.clone()` | positional removal (`removed` id at base index) |
| `🧯remove-accidental` | clone-and-write, whole-list diff; inverse restores `new_<collection>: base.<collection>.clone()` | positional removal (`removed` id at base index) |
| `🪚remove-member` | clone-and-write, whole-list diff; inverse restores `new_<collection>: base.<collection>.clone()` | positional removal (`removed` id at base index) |

### 🫨️en1998

| kind | before | after |
|---|---|---|
| `↪️change-tower-m-rd-nm` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `⚖️change-storey-permanent-gk-n` | clone-and-write, whole-list diff | nested keyed field patch |
| `✅️change-member-detailing` | clone-and-write, whole-list diff | nested keyed field patch |
| `➕️insert-building` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `➖️remove-assessment` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-bridge` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-building` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-foundation` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-retaining-wall` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-silo` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-tank` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `➖️remove-tower` | clone-and-write, whole-list diff | positional removal (`removed` id at base index) |
| `🌉insert-bridge` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `🌚️update-site` | scalar sparse diff (already clean) | sparse scalar field (unchanged) |
| `🏋️change-assessment-rkn` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `💪️change-system-v-rd-n` | clone-and-write, whole-list diff | nested keyed field patch |
| `📎️change-annex` | clone-and-write, whole-list diff | sparse scalar field (unchanged) |
| `📏️change-elevation-regular` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `📏️change-storey-drift-xm` | clone-and-write, whole-list diff | nested keyed field patch |
| `📐️change-storey-stiffness-x` | clone-and-write, whole-list diff | nested keyed field patch |
| `🔧insert-assessment` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `🗼insert-tower` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `🛑️change-bridge-v-rd-n` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `🛢️insert-tank` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `🧭️change-building-plan-regular` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `🧱️change-masonry-wall-ratio` | clone-and-write, whole-list diff | keyed row field patch (`modified{key, patch}`) |
| `🧱️insert-retaining-wall` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `🪨insert-foundation` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |
| `🫙insert-silo` | clone-and-write, whole-list diff | positional insertion (`inserted` at index) |

