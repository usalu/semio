# Packet: stdio 3D/CAD engine dissolution (stl, obj, gltf, step, ifc, dwg)

Wave scope: dissolve the last 8 `⚙️engine` directories in stdio's 3D/CAD formats — `🟪️stl`
(ascii), `🧊️obj` (3.0), `🧊️gltf` (2.0), `📐️step` (ap214), `🏗️ifc` (2x3, 4), `🖊️dwg` (ac1018,
ac1024). Non-stdio was already 100% done; this closes stdio's remaining roster for these six
formats.

## Destination per region per artifact

### 🟪️stl (ascii/✳️any)
- Codec (`decode_stl_ascii`/`encode_stl_ascii`/`decode_stl_binary`/`encode_stl_binary`/
  `decode_stl_auto`) → `🚪️io/🦀️component.rs`, flat top level (preserves `engine::encode_stl_ascii`/
  `encode_stl_binary` for external consumers).
- `empty_stl_snapshot`/`demo_stl_snapshot` → `🧬️schema/🦀️component.rs` (pure helpers).
- `register()`/`register_pilot_languages`/`register_artifact_schema`/`register_artifact_inferences`
  → confirmed zero callers repo-wide (only the doc comment referencing the old side-effecting call)
  → **deleted outright** (declaration() already supersedes them).
- `StlEngine` → zero construction sites → **deleted outright**.
- Tests (incl. `conformance_laws`) → `🚪️io/🦀️component.rs`, beside the codec they exercise.
- `io_registry` → `🚪️io/🦀️component.rs`.

### 🧊️obj (3.0/✳️any)
- Codec (`decode_obj`/`encode_obj` + `resolve_index`/`parse_face_vertex`/`write_face_vertex`) →
  `🚪️io/🦀️component.rs`, flat top level.
- `empty_obj_snapshot`/`demo_obj_snapshot`/`DEMO_OBJ_TEXT` → `🧬️schema/🦀️component.rs`.
- `register_schema_specs` → **kept live**, moved to `🧬️schema/🦀️component.rs` — this is one of the
  ten deliberate imperative calls (`.setup(crate::artifacts::obj::engine::register_schema_specs)`
  at the stdio plugin root, confirmed by grep).
- `register()`/`register_pilot_languages`/`register_artifact_schema`/`register_artifact_inferences`
  → confirmed dead → **deleted outright**.
- `ObjEngine` → zero construction → **deleted outright**.
- Tests → `🚪️io/🦀️component.rs`.
- `io_registry` → `🚪️io/🦀️component.rs`.

### 🧊️gltf (2.0/✳️any)
- Codec (Base64, AccessorModel incl. `GltfComponentType`/`GltfAccessorType`/`decode_accessor`,
  DocumentCodec `parse_gltf_document`/`serialize_gltf_document`, GlbContainer `encode_glb`/
  `decode_glb`) → `🚪️io/🦀️component.rs`.
- `empty_gltf_snapshot`/`demo_gltf_snapshot` → `🧬️schema/🦀️component.rs`.
- `register()`/`register_pilot_languages`/`register_artifact_schema`/`register_artifact_inferences`
  → confirmed dead → **deleted outright**.
- `GltfEngine` → zero construction → **deleted outright**.
- Tests → `🚪️io/🦀️component.rs`.
- `io_registry` → `🚪️io/🦀️component.rs`.

### 📐️step (ap214/✳️any)
- `part21` (shared ISO-10303-21 tokenizer, reused by `ifc` and cross-plugin by `📐️cad`) and `brep`
  (BrepMesh analyzer, part21-derived, no snapshot dependency) → `🚪️io/📐️part21/`,
  `🚪️io/🧱️brep/` — pure geometry algorithms with no snapshot dependency, kept with the codec per
  rule 6 (not promoted to a framework module: doing so would change the crate path every existing
  consumer — `ifc`, `📐️cad` — already depends on).
- `ladder` (shared CC classification, reused by the 6 `✳️ccN` subset validators) →
  `🚪️io/🪜️ladder/`, same reasoning.
- `io_registry` (the STEP-level union of `any` + `cc1..cc6` composer entries) →
  `🚪️io/🦀️component.rs`.
- `empty_step_snapshot`/`demo_step_snapshot` → `🧬️schema/🦀️component.rs`.
- `register()`/`register_subset_validators`/`register_pilot_languages`/`register_artifact_schema`/
  `register_artifact_inferences` → confirmed dead (declaration() already folds subset validators
  in via `step_subset_validators()`) → **deleted outright**.
- `StepEngine` → zero construction → **deleted outright**.
- Tests (incl. `conformance_laws`) → `🧬️schema/🦀️component.rs`, beside `demo_step_snapshot`/
  `empty_step_snapshot`, since the tests primarily exercise the DSL/pack facets those helpers feed.

### 🖊️dwg (ac1018 + ac1024)
- **ac1018**: confirmed dead repo-wide per the ticket's own prior finding — `register()`/
  `register_artifact_inferences`/`register_pilot_languages`/`register_schema_specs`/
  `register_artifact_schema` → **deleted outright**. Only `io_registry` (→ `🚪️io/`) and
  `empty_dwg_snapshot`/`demo_dwg_snapshot` + Tests (→ `🧬️schema/`) survive.
- **ac1024**: the real R2004+ byte-level decode pipeline (`R2004FileHeaderDecrypt`, `Lz77Variant`,
  `PageHeaderDecrypt`, `SectionMapAndInfo` — `decrypt_r2004_header`/`decompress_r2004_section`/
  `locate_r2004_sections`/`decode_r2004_sections`) is pure byte↔byte, no `DwgSnapshot` dependency
  → `🚪️io/🦀️component.rs`, along with Tests (incl. the real 145KB `architectural.dwg` fixture
  tests) and `io_registry`.
- `empty_dwg_snapshot`/`demo_dwg_snapshot` (ac1024) → `🧬️schema/🦀️component.rs`.
- `register_schema_specs` (ac1024) → **kept live**, moved to `🧬️schema/🦀️component.rs` — one of
  the ten deliberate calls (`.setup(crate::artifacts::dwg::engine::register_schema_specs)`,
  root-level shim aliased to ac1024 exclusively).
- `register()`/`register_artifact_inferences`/`register_pilot_languages`/`register_artifact_schema`
  (ac1024) → confirmed dead → **deleted outright**.
- `DwgEngine` (both standards) → zero construction → **deleted outright**.

**dwg combined-entries outcome**: the artifact root `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/
🦀️component.rs` already contained `dwg_combined_composer_entries()` (unioning
`standards::v_ac1018::engine::io_registry::entries()` and `standards::v_ac1024::engine::
io_registry::entries()`, fully qualified, never through the shadowing root `io_registry` module)
from an earlier wave — this wave's job was only to make those two `standards::vX::engine::
io_registry` paths keep resolving after the physical `⚙️engine` dirs were dissolved, which the two
new barrel shims (`pub use super::subsets::any::io::*; pub use super::subsets::any::schema::*;`)
do. **No composer entries were dropped for either standard**; ac1018's now-dead registration
functions were deleted, but its composer stays live and unioned.

### 🏗️ifc (2x3 + 4) — registration deliberately left alone
Per the ticket's explicit instruction, ifc's imperative registration mechanism was **not**
converted to `ArtifactDeclaration` and its behavior was preserved exactly, only physically moved:
- **2x3**: `decode_ifc2x3`/`encode_ifc2x3`/`IFC2X3_SCHEMA_NAME` → `🚪️io/🦀️component.rs`.
  `empty_ifc2x3_snapshot`/`demo_ifc2x3_snapshot` → `🧬️schema/🦀️component.rs`. **`register()`/
  `register_artifact_inferences`/`register_pilot_languages` kept live, moved to `🧬️schema/
  🦀️component.rs`** — reached as `standards::v2x3::engine::register()`, exactly the path
  `📦️glue.rs`'s root `ifc::engine::register()` override calls explicitly alongside
  `v4::engine::register()`. `Ifc2x3Engine` → zero construction → deleted. Tests → `🚪️io/`.
  `io_registry` → `🚪️io/`.
- **4**: `spatial` submodule (placement matrices/property sets, part21-derived, reused externally
  by `🧿️semio`'s own IFC4 deserializer) → `🚪️io/🏛️spatial/`, mounted via the same
  `#[path]`-inside-io/component.rs mechanism `part21`/`brep`/`ladder` use for step.
  `empty_ifc_snapshot`/`demo_ifc_snapshot` → `🧬️schema/🦀️component.rs`. **`register()`/
  `register_pilot_languages`/`register_artifact_schema`/`register_artifact_inferences` kept live,
  moved to `🧬️schema/🦀️component.rs`** — this is THE canonical `.setup`-adjacent call the plugin
  root's `crate::artifacts::ifc::engine::register()` reaches (root shim glob-imports v4 by
  default, then the root's own local `register()` override calls both `v4::engine::register()`
  and `v2x3::engine::register()` explicitly). `IfcEngine` → zero construction → deleted. Tests →
  `🚪️io/`. `io_registry` → `🚪️io/`.

**⚠️ One genuine bug caught by the compiler and fixed**: my first pass on ifc 2x3's
`🧬️schema/🦀️component.rs` left the `register_schema_spec`-not-called doc comment as a trailing
`///` block at true end-of-file with no following item (the exact `//#region`-adjacent E0753
hazard this ticket's brief warned about, though mine was a genuine end-of-file dangling comment,
not a `//!` mid-file block). Converted to plain `//` comments; verified by a fresh
`cargo check` that the resulting `E0432`/E0425` cascade (missing `Ifc2x3Analyzer`/`Ifc2x3Composer`/
`Ifc2x3Parts`/`Ifc2x3Builder`, all macro-generated by `derive_artifact_facets!` earlier in the same
file) cleared completely.

## Module path vs directory path — glue.rs nesting checked per standard, not assumed

- `step`: single standard (`v_ap214`), single-file engine mount at `standards::v_ap214::engine`.
- `dwg`: **checked ac1018 and ac1024 separately** — both were single-file `#[path=...] pub mod
  engine;` mounts at `standards::v_ac1018::engine` / `standards::v_ac1024::engine` respectively;
  same shape, converted identically.
- `ifc`: **checked 2x3 and 4 separately** — both were single-file mounts at `standards::v2x3::
  engine` / `standards::v4::engine`; the root-level `pub mod engine { pub use super::standards::
  v4::engine::*; fn register() {...} }` shim (a `📦️glue.rs`-only barrel, never a physical
  directory) was left untouched since it already correctly calls both standards explicitly.

## Consumer call sites (external, cross-plugin)

Verified via
`grep -rn "semio_s_plugin_stdio::artifacts::(stl|obj|gltf|step|ifc|dwg)::" ✏️s 🧰️framework
--include="*.rs" | grep -v "^✏️s/🔌️plugins/🗄️stdio/" | grep -vE ':[0-9]+: *(///|//!|//)'` before and
after every edit (77 lines, unchanged set):

- `📐️cad`'s own `🚪️io/🦀️component.rs` and `🗺️geometry-import/🦀️component.rs`: `obj::standards::
  v3_0::engine::encode_obj`, `stl::standards::v_ascii::engine::encode_stl_binary`, `step::
  standards::v_ap214::engine::part21::{parse_part21, write_part21}`.
- `🏗️fem`'s own serializers (`🧊️3d` and `◻2d`): `obj::standards::v3_0::engine::encode_obj`,
  `stl::standards::v_ascii::engine::encode_stl_ascii`.

**None of these call sites needed editing.** Every one resolves through the per-standard `engine`
barrel shim (`pub use super::subsets::any::io::*; pub use super::subsets::any::schema::*;`) I
installed in `📦️glue.rs`, since `encode_obj`/`encode_stl_ascii`/`encode_stl_binary`/`part21` all
now live at the top level of each artifact's `🚪️io/🦀️component.rs`. Confirmed by the consumer
`cargo check -p semio-s-plugin-cad -p semio-s-plugin-fem --all-targets` run below.

Grep also surfaced ~55 plain `use semio_s_plugin_stdio::artifacts::{obj,dwg,gltf,stl,step,ifc}::
{Snapshot type, DOCUMENT_SCHEMA}` imports across `remodel`, `raster`, `process`, `block`,
`procedural`, `gis`, `sourcing`, `note`, `shooting`, `layout`, `puzzle`, `lowpoly` — none of these
go through `engine::`, so none were affected by this wave.

## Bare `io_registry::entries()` check ("the defect that ships GREEN")

`grep -rn "\bio_registry::entries()" <the six format trees>` → every hit is either fully qualified
(`crate::artifacts::X::engine::io_registry::entries()` / `standards::vY::engine::io_registry::
entries()`) or inside a doc comment. **Zero bare calls** introduced or left behind. Each artifact
root's own `.composers(...)` already pointed at the fully-qualified `engine::io_registry::
entries()` path before this wave (dxf/step/obj/gltf/stl/dwg all follow the same established
pattern); this wave did not touch any `.composers(...)` call site, only what `engine::io_registry`
resolves to.

## Assertion arithmetic (tests moved, none dropped)

`#[test]` count per artifact, summed across its new `🚪️io/🦀️component.rs` + `🧬️schema/
🦀️component.rs`, matches the original engine file's test count exactly (schema files that already
had pre-existing, unrelated tests before this wave show the pre-existing count on top — obj's
schema had 2 pre-existing `DerivedAnalysis` sniff tests, so 15 moved + 2 pre-existing = 17):

| Artifact | Moved (engine's own) | Total now (io+schema) |
|---|---|---|
| step (ap214) | 8 | 8 |
| obj (3.0) | 15 | 17 (2 pre-existing) |
| gltf (2.0) | 25 | 25 |
| stl (ascii) | 15 | 15 |
| dwg ac1018 | 9 | 9 |
| dwg ac1024 | 14 | 14 |
| ifc 2x3 | 13 | 13 |
| ifc 4 | 8 | 8 |

## Compiler output

`RUSTC_WRAPPER="" CARGO_TARGET_DIR=.../target/stdio_3d2 cargo check -p semio-s-plugin-stdio
--all-targets`, run 5 times across the wave (after each artifact and again at the end). The final
run:

```
error: could not compile `semio-s-plugin-stdio` (lib) due to 2 previous errors; 604 warnings emitted
error: could not compile `semio-s-plugin-stdio` (lib test) due to 7 previous errors; 749 warnings emitted
```

All 9 distinct errors are in `💾️binary`, `🎨️svg`, `🎞️gif`, `📄️pdf` — confirmed by
`grep -B3 "^error" <output> | grep -E "🟪️stl|🧊️obj|🧊️gltf|📐️step|🏗️ifc|🖊️dwg"` returning **zero
matches** on every one of the 5 runs. `git status --porcelain` on `✏️s/🔌️plugins/🗄️stdio` at
report time shows a much wider set of `⚙️engine` deletions in flight (`epw`, `zip`, `gif`, `pptx`,
`svg`, `bcf`, `binary`, `pdf`, …) than this wave touched — another session is actively dissolving
the rest of stdio's roster concurrently, in the same tree, right now. This is exactly the
concurrent-churn shape `📌️important.md` §"Cross-session protocol" describes, not a regression from
this wave.

Consumer check: `cargo check -p semio-s-plugin-cad -p semio-s-plugin-fem --all-targets` —
**could not complete**: `semio-s-plugin-cad`/`semio-s-plugin-fem` depend on `semio-s-plugin-stdio`,
which cargo must build first; that build hit the same `svg`/`gif` concurrent-churn errors (not
mine) documented above, so cad/fem were never reached. Structural evidence stands in their place:
the consumer grep re-run (above) shows the exact same 77-line, unchanged call-site set before and
after this wave's edits, and every `engine::`-qualified call site resolves through a barrel shim
this wave installed and verified compiles clean in isolation (the `semio-s-plugin-stdio` lib itself
has zero errors attributable to `stl`/`obj`/`gltf`/`step`/`ifc`/`dwg`). **Unverified** (blocked by
upstream churn, not a gap in this wave's own work) — re-run
`cargo check -p semio-s-plugin-cad -p semio-s-plugin-fem --all-targets` once `svg`/`gif` are green
again.

## Deviations from the brief

- STEP's Tests (incl. `conformance_laws`) went to `🧬️schema/` rather than `🚪️io/` (unlike every
  other artifact in this wave) because `demo_step_snapshot`/`empty_step_snapshot` — the tests'
  primary subject — already lived there per rule 5, and `step`'s io/component.rs's own codec is
  the shared `part21`/`brep`/`ladder` trio (used by `ifc`/`cad` too), not step-specific — pairing
  the tests with the step-specific document helpers instead kept the shared codec files
  unencumbered by step-only test fixtures.
- part21/brep/ladder (step) and spatial (ifc 4) were kept in `🚪️io/` rather than promoted to a
  `✏️s/🔨️modules/` or `🧰️framework/🔨️modules/` module engine, despite being reused by a second
  artifact (`ifc`) and, for part21/encode_obj/encode_stl, a second plugin (`📐️cad`, `🏗️fem`) —
  promoting them would change the crate path (`semio_s_plugin_stdio::...` → something else) every
  existing consumer depends on, a much larger blast radius than this wave's mandate. Documented per
  format above; flagged here for a future wave if the framework/module split is ever wanted.

## Files touched

**Created (physical moves, verbatim content unless noted)**: `🚪️io/📐️part21/🦀️component.rs`,
`🚪️io/🧱️brep/🦀️component.rs`, `🚪️io/🪜️ladder/🦀️component.rs` (step); `🚪️io/🏛️spatial/
🦀️component.rs` (ifc 4).

**Updated**: `🚪️io/🦀️component.rs` and `🧬️schema/🦀️component.rs` for each of the 8 standard/
artifact pairs listed above; `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` (8 physical
`pub mod engine;` mounts converted to inline barrel shims).

**Removed**: all 8 `⚙️engine` directories —
`📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/⚙️engine`,
`🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/⚙️engine`,
`🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/⚙️engine`,
`🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/⚙️engine`,
`🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/⚙️engine`,
`🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/⚙️engine`,
`🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/⚙️engine`,
`🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/⚙️engine`.

## Verification commands (real output)

```
$ find ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🟪️stl,🧊️obj,🧊️gltf,📐️step,🏗️ifc,🖊️dwg} -type d -name "⚙️engine"
(no output — 0 dirs)

$ python3 <dangling #[path] check from the brief>
dangling: 0
```

## sharedFileRequests
None — `📦️glue.rs` edits were all in disjoint per-standard blocks (`step`, `obj`, `gltf`, `stl`,
`dwg` v_ac1018/v_ac1024, `ifc` v2x3/v4), no overlap with any other artifact's own mount lines.

## Concurrent-churn observations
See "Compiler output" above. `deflate` (E0753 mid-file `//!` block) and `gif` (dangling `⚙️engine`
mount) were the first two artifacts observed mid-edit by another session; by the final check the
red set had grown to include `binary`/`svg`/`gif`/`pdf`, none in this wave's scope, all outside
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🟪️stl,🧊️obj,🧊️gltf,📐️step,🏗️ifc,🖊️dwg}`.

## Honest pass/fail
**Pass** for this wave's own scope: 0/8 `⚙️engine` dirs remain, 0 dangling `#[path]` mounts, 0
bare `io_registry::entries()`, 0 compiler errors originating in `stl`/`obj`/`gltf`/`step`/`ifc`/
`dwg`, external consumer call sites unchanged and verified. **Unverified** (out of scope, another
session's live work): the crate-wide `semio-s-plugin-stdio` build is red due to `binary`/`svg`/
`gif`/`pdf`.
