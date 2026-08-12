# Engine Dissolution — `🏭️process` / `🧊️process3d`

Ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES (#2553).
Dispatch-time baseline commit: **`382ace1b27`**. All "before" figures are read from that commit
(`git show 382ace1b27:<path>`), **not** from `HEAD` — this repo auto-commits, so `HEAD` already
contains this work and is useless as a baseline.

---

## (a) Summary row

| Engine dir (deleted) | LOC before | Destinations |
|---|---|---|
| `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` (5 files) | 1915 | `🧬️schema/🦀️component.rs`, `🧬️schema/💡️inferences/🦀️component.rs`, `🚪️io/🦀️component.rs`, `🎛️apps/🧊️3d/🦀️component.rs` |

Per-file LOC before: main `🦀️component.rs` 1048 · `🪵️wood` 235 · `🔩️metal` 217 · `🤖️robotic` 208 · `🧱️concrete` 207.

Note the real on-disk subdir names carry a variation selector the dispatch brief omitted:
`🔩️metal`, `🪵️wood`, `🤖️robotic`, `🧱️concrete` (not `🔩metal`/`🪵wood`/…). Confirmed by `find` before editing.

---

## (b) Region-by-region destinations

### Main `⚙️engine/🦀️component.rs` (1048 LOC)

| Region / item | Destination | Rule |
|---|---|---|
| `🔖️Plugin` — `register()` | **app** `🎛️apps/🧊️3d`, new `//#region 🔌️Registration` | 6 (live wiring) |
| `🔖️ExampleFixtures` — `TIMBER/PLATE_EXAMPLE_DSL`, `default_document`, `plate_document` | `🧬️schema`, `//#region 🔖️ExampleFixtures` | 3 |
| `🔖️IdGeneration` — `next_step_id` | `🧬️schema`, `//#region 🔖️DocumentHelpers` | 3 |
| `🔖️Io` — `process3d_io() -> AppIo` | **app**, `//#region 🔖️Io` | 4 (returns `AppIo`) |
| `🔖️Catalog` — `GenericCatalog` | `🧬️schema`, `//#region 🔖️Catalog` | 3 (pure data) |
| `🔖️Catalog` — `ContributedMachineCatalog`, `ProcessMachinesTopicPayload`, `sync_process_machine_contributions`, `CONTRIBUTED_MACHINE_CATALOGS`, `LAST_PROCESS_CONTRIBUTIONS_JSON`, `leak_str`, `builtin_installed_catalogs`, `installed_catalogs`, `catalog_machine` | **app**, `//#region 🔧️Behavior` | 7 (mutable global state fed by host contributions — genuinely stateful, not a snapshot projection) |
| `🔖️Catalog` — `find_capability`, `capability_for_measure_kind`, `ValidationContext`, `ValidationFailure`, `parameter_value`, `quantity_value`, `quantity_label`, `validate_capability`, `validation_reason`, `stock_extent`, `validation_context_for_stock`, `measure_for_capability` | `🧬️schema/💡️inferences`, `//#region 🔖️CapabilityValidation` | 2 (pure `&Snapshot`-fragment → value projections) |
| `🔖️KernelReplay` — `hash_value`, `ProcessKernelReplay`, `ProcessKernelMemo`, `prefix_signature`, `solid_for_spec`, `tool_solid_for_measure`, `replay_process`, `processed_mesh`, `processed_volume`, both consts | `🧬️schema/💡️inferences`, `//#region 🔖️KernelReplay` | 2 (derived compute: `Snapshot` → mesh/volume) |
| `🔖️KernelReplay` — `axis_angle_from_up_to` | **app**, `//#region 🔧️Behavior` | 8 (pure math, no snapshot/io coupling; sole caller is `🎮️commands/🌍️world` face-drag) |
| `🔖️MediaImportExport` — `Process3dModelExport`, `export_process3d_model`, `process3d_bytes_from_data_url`, `import_process3d_model` | `🚪️io`, `//#region 🔖️MediaImportExport` | 5 (serializer/deserializer wrappers) |
| `🔖️DocumentHelpers` — `insert_step_mutations`, `remove_step_mutations` | `🧬️schema`, `//#region 🔖️DocumentHelpers` | 3 (pure, no `&mut self`) |
| `register_pilot_languages`, `register_artifact_schema`, `register_artifact_inference` | **app**, `//#region 🔌️Registration` | 6 |
| `🔖️ArtifactEngine` — **`Process3dEngine` struct + `new()` + `into_snapshot()`** | **DELETED OUTRIGHT** | 1 — see below |
| `🚪️DerivedIoRegistry` — `io_registry` mod (`entries()`, 8 export composers, `rebuild_native_snapshot`, all dialect consts) | `🚪️io`, `//#region 🚪️DerivedIoRegistry` | 5 |

### Material subdirs

| File | Destination |
|---|---|
| `⚙️engine/🔩️metal/🦀️component.rs` (217) | `🧬️schema` `//#region 🔖️Catalog` → `MetalCatalog` + `metal_catalog()`; tests → `//#region 🔖️MetalCatalog` |
| `⚙️engine/🪵️wood/🦀️component.rs` (235) | `🧬️schema` → `WoodCatalog` + `wood_catalog()`; tests → `//#region 🔖️WoodCatalog` |
| `⚙️engine/🤖️robotic/🦀️component.rs` (208) | `🧬️schema` → `RoboticCatalog` + `robotic_catalog()`; tests → `//#region 🔖️RoboticCatalog` |
| `⚙️engine/🧱️concrete/🦀️component.rs` (207) | `🧬️schema` → `ConcreteCatalog` + `concrete_catalog()`; tests → `//#region 🔖️ConcreteCatalog` |

These are **pure data** (static machine/capability tables), not process simulation — rule 3, not rule 7.
Each carried a byte-identical private `parameter` / `max_rule` / `min_rule` helper trio; the four copies
were collapsed into one shared trio in `🧬️schema` (CLAUDE.md: "if code is repeated, it MUST be close to
each other"). Each file's free `catalog()` was renamed `<material>_catalog()` on merge to avoid four
colliding names in one module.

### Rule-1 verdict — `Process3dEngine` deleted

- `grep -rn "Process3dEngine" ✏️s 🧰️framework` → **2 hits, both its own definition** in the engine file.
- `trait ArtifactEngine` / `impl … ArtifactEngine for …` → **0 hits** plugin-wide.

No external reference, no trait impl ⇒ deleted outright with both methods, per rule 1. **No exception raised.**

---

## (c) Unqualified paths found and how qualified

The `🧱️block/◻2d` two-`io_registry` hazard **is present here** and was handled:

- `🗿️artifacts/🧊️process3d/🦀️component.rs` has its own thin `io_registry` whose `entries()` returns
  `&'static [&'static ComposerEntry]` — a **different type** from the real one, which returns
  `&'static [ComposerEntry]`.
- Its `use … as v1` previously read `…::standards::v1::engine::io_registry`. Repointed to the real
  registry's new home, **fully qualified**:
  `crate::artifacts::process3d::standards::v1::subsets::any::io::io_registry as v1`.
- Every path moved into a new file was written fully qualified. No bare `io_registry::…` survives in
  either `🚪️io/🦀️component.rs` or the artifact root (`grep -n "\bio_registry::"` → 0 hits in both).

Other requalifications (all call sites, 11 files):

| Old | New |
|---|---|
| `process3d::engine::{default_document, plate_document}` | `process3d::schema::{…}` |
| `process3d::engine::{insert_step_mutations, next_step_id, remove_step_mutations}` | `process3d::schema::{…}` |
| `process3d::engine::{processed_mesh, processed_volume}` | `process3d::schema::inferences::{…}` |
| `process3d::engine::{find_capability, validate_capability, validation_context_for_stock, validation_reason, capability_for_measure_kind, measure_for_capability}` | `process3d::schema::inferences::{…}` |
| `process3d::engine::{export_process3d_model, import_process3d_model}` | `process3d::io::{…}` |
| `process3d::engine::{installed_catalogs, catalog_machine, sync_process_machine_contributions, axis_angle_from_up_to}` | `crate::apps::process3d::{…}` |
| `process3d::engine::{process3d_io, TIMBER_EXAMPLE_DSL, PLATE_EXAMPLE_DSL}` | app-local / `process3d::schema::{…}` |
| plugin root `.setup(process3d::engine::register)` | `.setup(crate::apps::process3d::register)` |

`ProcessKernelReplay` / `replay_process` were widened from private to `pub` (documented inline) so
`🚪️io`'s exporter/importer can reach the real kernel + replayed handle, which the tessellated
`processed_mesh` / `processed_volume` projections cannot supply.

---

## (d) Assertion counts — exact, before vs after

**Before** — engine files at `382ace1b27`:

| File | asserts |
|---|---|
| `⚙️engine/🦀️component.rs` | 29 |
| `⚙️engine/🔩️metal/…` | 7 |
| `⚙️engine/🪵️wood/…` | 7 |
| `⚙️engine/🤖️robotic/…` | 7 |
| `⚙️engine/🧱️concrete/…` | 7 |
| **engine subtotal** | **57** |

**Destinations** — pre-existing at `382ace1b27` vs now:

| Destination | before | after | delta |
|---|---|---|---|
| `🧬️schema/🦀️component.rs` | 0 | 32 | **+32** |
| `🧬️schema/💡️inferences/🦀️component.rs` | 3 | 10 | **+7** |
| `🚪️io/🦀️component.rs` | 0 | 0 | 0 |
| `🎛️apps/🧊️3d/🦀️component.rs` | 44 | 62 | **+18** |
| **subtotal** | **47** | **104** | **+57** |

**Grand total: 57 + 47 = 104 before → 104 after. Zero assertions lost.**

Delta reconciliation against the main file's 29:

- app **+18** = `process3d_io_mirrors_the_declared_artifact_kind` (4) + `…declares_geometry_in_and_brep_out_ports` (10) + `face_drag_orients_box_along_normal` (2) + `face_drag_degenerate_antiparallel_normal_does_not_panic` (1) + `sync_process_machine_contributions_merges_hot_installed_catalogs` (1)
- inferences **+7** = `drill_reduces_volume_below_stock` (1) + `attach_increases_volume_above_stock` (1) + `disabled_step_is_skipped_on_replay` (1) + `cursor_zero_yields_stock_volume` (1) + `box_primitive_spans_from_local_origin_corner` (3)
- schema **+32** = `default_document_parses_timber_example` (2) + `plate_document_parses_and_opens_mid_timeline` (2) + the four material files' 7×4 = 28

18 + 7 + 4 = 29 ✓ (main file) · 28 ✓ (materials) · 57 total ✓

No new test files created; every test landed in an existing file under a named subregion.

---

## (e) Compile status

**UNVERIFIED — build-lock contention, not attempted to completion.**

The mandated command was issued four times against the shared ticket `CARGO_TARGET_DIR`. Runs 3 and 4
never got past `Blocking waiting for file lock on build directory` (≈35 concurrent cargo processes
across the coordinator's fan-out) and were abandoned on instruction. **No green claim is made, and no
red claim is made, for the current tree.**

What *was* observed, from the last run that actually reached rustc (`scratch-cargo-check-2.txt`,
against an intermediate state of my edits) — **4 errors, since triaged**:

| # | Error | File | Mine? | Status |
|---|---|---|---|---|
| 1 | `E0432` unresolved import `…subsets::any::io::io_registry` | `🗿️artifacts/🧊️process3d/🦀️component.rs` | **yes** | **fixed** — I had repointed the artifact root's `as v1` alias before actually creating the `io_registry` module at its new home. `🚪️io/🦀️component.rs` now defines it. |
| 2 | `E0599` no method `export` on `GlbExporter` | `🚪️io/🦀️component.rs` | **yes** | **fixed** — `MeshExporter` trait was in the old engine's `use` list, not carried over. Added. |
| 3 | `E0599` no method `import` on `GlbImporter` | `🚪️io/🦀️component.rs` | **yes** | **fixed** — same cause, `MeshImporter`. Added. |
| 4 | `E0308` `?` type mismatch, expected `JsonValue` found `Value` | `🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs:10` | **no — pre-existing** | **left alone**, see attribution below |

Verbatim, error 4:

```
error[E0308]: `?` operator has incompatible types
  --> ✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/./././././././././././././../../🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs:10:16
   |
10 |         value: serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?,
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `JsonValue`, found `Value`
   |
   = note: `?` operator cannot convert from `serde_json::Value` to `JsonValue`
```

**Attribution — error 4 is another session's in-flight refactor, not mine.** Evidence:

- `git log -3 --oneline -- <export leaf>` → `2564722008` only; `git diff --name-only 382ace1b27 -- <export leaf>` → **empty**. The file is untouched by me and unchanged since my baseline.
- The *import* counterpart at `🚪️io/📥️import/…/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` **has** changed since my baseline, and not by me:
  ```
  -    let mut out: Process3dSnapshot = serde_json::from_value(from.value.clone())
  +    let out: Process3dSnapshot = serde_json::from_value(from.to_serde_value())
  -    deserialize(&JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
  +    deserialize(&JsonSnapshot::from_value(value))
  ```
  i.e. someone is converting `JsonSnapshot::value` from bare `serde_json::Value` to a `JsonValue`
  newtype (`to_serde_value()` / `from_value()` accessors) in `semio-s-plugin-stdio`, has updated
  process3d's import leaf, and has not yet reached its export leaf. Neither leaf is in my blast radius.
- I deliberately did **not** fix it: it sits mid-refactor in another session's lane, and a one-line
  patch from me would race them.

The ticket's stated "known inference-related errors from another session" did **not** reproduce, and
would not have explained error 4 in any case — I verified rather than assumed, and error 4 is a
json/`JsonSnapshot` typing issue, not an inference one.

Because rustc reports all resolvable errors rather than halting at the first, the run that surfaced
these four is meaningful evidence that errors 1–3 were the complete set attributable to me at that
point. **It is not evidence that the current tree compiles** — the fixes for 1–3 have never been
through a completed compile. Please attribute this crate from the central
`cargo check --workspace --all-targets --keep-going`.

---

## (f) Structural verification (compiler-independent) — all clean

```
$ find ✏️s/🔌️plugins/🏭️process -path "*🗿️artifacts*" -name "⚙️engine" -type d
0
$ grep -rn "::engine::\|standards::v1::engine\|subsets::any::engine" ✏️s/🔌️plugins/🏭️process | wc -l
18
```

All 18 `::engine::` hits are unrelated third-party paths, **zero** are artifact-engine leftovers:

- `base64::engine::general_purpose` — 4
- `semio_framework_3d::brep::engine::{block_on, BrepEngineHost, BrepKernel, GeometryHandle}` — 14
- `semio_s_plugin_stdio::…::engine::` (stdio's still-standing engines, out of scope) — 0
- `process3d::engine::` — **0**

Dangling `#[path]` audit on `🏭️process/📦️packages/🦀️rust/📦️glue.rs`:

```
total #[path] mounts: 220
dangling: 0
```

Glue cleanup confirmed: `grep -n "⚙️engine" 📦️glue.rs` → none; `grep -n "mod engine" 📦️glue.rs` → none.
Both the `pub mod engine { … }` taxonomy block (5 `#[path]` mounts) and the
`pub mod engine { pub use super::standards::v1::engine::*; }` back-compat shim were removed. Brace
balance re-checked: 123/123.

`rm -rf` was preceded by `find … -name Cargo.toml` → empty, so no crate was destroyed.

---

## (g) Deviations

1. **`register()` is a `.setup()` free function, not `declaration()`.** The exemplar `🧱️block` has
   migrated to `.artifact(declaration())`; **process3d has no `declaration()` at all**
   (`grep -rn "fn declaration" ✏️s/🔌️plugins/🏭️process` → 0 hits, and `.composers(` / `.artifact(` → 0
   hits each). There was no `.composers(...)` call to point anywhere. Rather than invent that migration
   — out of scope for this packet, and a different ticket's job (ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
   M1/W1d) — I moved the live `register()` verbatim into the app's `//#region 🔌️Registration` per
   rule 6, and repointed the plugin root's existing `.setup(...)`. The registration surface is
   unchanged in behaviour; only its owner moved from artifact to app, which is the thesis.
2. **No `//#region 🔧️Behavior` state machine was invented.** The four material catalogs turned out to
   be static data, not process simulation, so rule 7's "park stateful pieces" applies only to the
   contributed-catalog registry (two `static Mutex`es fed by host topic contributions), which is
   genuinely stateful and now sits in the app's `🔧️Behavior` region. The app-side
   `🎛️apps/🧊️3d/⚙️engine` directory exists but is **empty and unmounted** — left untouched as the
   reserved machine slot, per instructions.
3. **Four duplicate helper trios collapsed into one.** Not a pure move; justified above.
4. **`ProcessKernelReplay`/`replay_process` visibility widened** to `pub` for the io seam. Documented
   inline at the type.
5. **`catalog()` → `<material>_catalog()`** on merge, to de-collide four same-named free functions.

---

## (h) Files touched

Deleted (5):
```
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🔩️metal/🦀️component.rs
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🪵️wood/🦀️component.rs
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🤖️robotic/🦀️component.rs
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🧱️concrete/🦀️component.rs
```
(directory `⚙️engine/` and all 4 subdirs removed)

Modified (13):
```
✏️s/🔌️plugins/🏭️process/🦀️component.rs
✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📦️glue.rs
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️component.rs
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs
✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🦀️component.rs
✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎚️config/🦀️component.rs
✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️component.rs
✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/📌️panels/🔍️inspection/🦀️component.rs
✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/📌️panels/🛍️catalogue/🦀️component.rs
✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/📌️panels/🛠️workshop/🦀️component.rs
✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎮️commands/{🌍️world,📄️artifact,📤️media,🛠️workshop,🪜️step}/🦀️component.rs
```

**Not** mine, despite appearing in `git diff 382ace1b27 -- ✏️s/🔌️plugins/🏭️process`:
`🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` — another
session's `JsonSnapshot` newtype refactor (see §e).

Scratch logs: `scratch-cargo-check-{1,2,3,4}.txt` in this ticket folder.

---

## Verdict

**PASS (structural) / UNVERIFIED (compile)** — engine directory and all 4 material subdirs deleted, 0 artifact-tree `⚙️engine` dirs, 0 `process3d::engine::` references, 0 dangling `#[path]` of 220, all 57 assertions preserved (104→104 across destinations), 3 self-inflicted compile errors found and fixed, 1 pre-existing cross-session `E0308` attributed and left alone; final compile never completed due to shared build-lock contention.
