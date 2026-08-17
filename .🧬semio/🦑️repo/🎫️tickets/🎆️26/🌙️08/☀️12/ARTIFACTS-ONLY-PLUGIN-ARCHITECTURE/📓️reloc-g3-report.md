# Reloc G3 — `declaration()`/`pilot_languages()` out of `⚙️engine` for 💠️lowpoly, 🖨️raster, 📸️remodel

Scope: relocate `pub fn declaration()` (and its private helper `pilot_languages()`) out of each
artifact's `⚙️engine/🦀️component.rs` into that artifact's root `🗿️artifacts/<a>/🦀️component.rs`,
alongside `artifact_kind()`. `pilot_languages` stays **private** (not `pub`) in all three sites — no
new public API surface was introduced. `⚙️engine` directories are left in place; only the two
functions moved out.

Census: `grep -rln "fn declaration" "✏️s/🔌️plugins/<plugin>"` returned exactly **one** hit per plugin
(one artifact each, one `declaration()` each). No plugin had zero declarations to skip.

---

## 💠️lowpoly (crate `semio-s-plugin-lowpoly`)

**Move-both held cleanly** — `declaration()`'s body was already fully qualified
(`crate::artifacts::lowpoly::…`) except for the call to `pilot_languages()` itself, and
`pilot_languages()`'s own body only referenced fully-qualified `crate::…`/`dsl::…` paths. No deviation.

Site moved:
- `declaration()` + doc comment: FROM
  `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:78-98`
  TO
  `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️component.rs` (new `//#region 🔖️Register`, inserted
  after `//#endregion 🔖️ArtifactKind`, before `//#region 🧪️Tests`) — now at line 281 (`declaration()`
  itself at line 297).
- `pilot_languages()`: FROM engine file lines 105-166 TO the same new region in the root file
  (private `fn`, unchanged body).
- Kept **in place** in the engine file (per instruction, region stays, only the two functions
  leave): `artifact_schema_registered()` (was lines 100-103) — the `//#region 🔖️Register` /
  `//#endregion 🔖️Register` wrapper in the engine file now contains only that one function.

Call site updated:
- `✏️s/🔌️plugins/💠️lowpoly/🦀️component.rs:15`:
  `crate::artifacts::lowpoly::engine::declaration()` → `crate::artifacts::lowpoly::declaration()`.

## 🖨️raster (crate `semio-s-plugin-raster`)

**Deviation** (per instruction §3, reported explicitly): `declaration()`'s body called
`io_registry::entries()` **unqualified**, resolving to the `pub mod io_registry { … }` that lives
*inside the same engine file* (`⚙️engine/🦀️component.rs:752-868`, module path
`crate::artifacts::raster::standards::v1::engine::io_registry`, confirmed against `📦️glue.rs`
line 38-39). That module is not part of the two-function move (only `declaration()` +
`pilot_languages()` travel), so the unqualified call would not resolve from the artifact root.
Fixed by qualifying it at the new call site:
`.composers(crate::artifacts::raster::standards::v1::engine::io_registry::entries())`.
`pilot_languages()`'s own body was already fully qualified — no other deviation.

Site moved:
- `declaration()` + doc comment: FROM
  `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:21-37`
  (was the entire `//#region 🔖️Register` … `//#endregion 🔖️Register`, containing nothing else) TO
  `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️component.rs`, new `//#region 🔖️Register` inserted
  after `//#endregion 🔖️ArtifactKind`, before `//#region 🧪️Tests` — `declaration()` now at line 240,
  with the `io_registry::entries()` call qualified as described above.
- `pilot_languages()`: FROM engine file lines 39-100 TO the same new region in the root file
  (private `fn`, unchanged body).
- The engine file's `//#region 🔖️Register` wrapper is now gone entirely (it held only these two
  functions); `pub mod io_registry` and everything else in the engine file (SemioBridge,
  MediaExport/Import, Io, ArtifactEngine, Tests) is untouched.

Call site updated:
- `✏️s/🔌️plugins/🖨️raster/🦀️component.rs:15`:
  `crate::artifacts::raster::engine::declaration()` → `crate::artifacts::raster::declaration()`.

## 📸️remodel (crate `semio-s-plugin-remodel`)

**Move-both held cleanly** — same shape as lowpoly: `declaration()`'s `io_registry::entries()` call
was already fully qualified as `crate::artifacts::remodel::standards::v1::engine::io_registry::entries()`
(unlike raster, this one was never local-unqualified), and `pilot_languages()`'s body was already
fully qualified. No deviation.

Site moved:
- `declaration()` + doc comment: FROM
  `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:28-44`
  (the entire `//#region 🔖️Register` … `//#endregion 🔖️Register`, containing nothing else) TO
  `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs`, new `//#region 🔖️Register` inserted
  immediately after `//#endregion 🔖️ArtifactKind` (before the `pub use …RemodelMutation` line) —
  `declaration()` now at line 48.
- `pilot_languages()`: FROM engine file lines 46-107 TO the same new region in the root file
  (private `fn`, unchanged body).
- The engine file's `//#region 🔖️Register` wrapper is now gone entirely; the rest of the 1000+ LOC
  engine file (camera/geo/images/mesh/reconstruction/sfm/video topic re-exports, MeshBridge, Ids,
  Tests) is untouched.

Call site updated:
- `✏️s/🔌️plugins/📸️remodel/🦀️component.rs:15`:
  `crate::artifacts::remodel::engine::declaration()` → `crate::artifacts::remodel::declaration()`.

---

## Verify — four greps, repo-wide over all three plugin directories

```
$ grep -rn "fn declaration" ✏️s/🔌️plugins/💠️lowpoly ✏️s/🔌️plugins/🖨️raster ✏️s/🔌️plugins/📸️remodel
✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️component.rs:297:pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️component.rs:240:pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs:48:pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
```
Exactly one hit per plugin, all at the artifact root, none under `⚙️engine`. ✅

```
$ grep -rn "engine::declaration" ✏️s/🔌️plugins/💠️lowpoly ✏️s/🔌️plugins/🖨️raster ✏️s/🔌️plugins/📸️remodel
(zero hits, all three)
```
✅

```
$ grep -rn "pub fn pilot_languages" ✏️s/🔌️plugins/💠️lowpoly ✏️s/🔌️plugins/🖨️raster ✏️s/🔌️plugins/📸️remodel
(zero hits, all three)
```
Nothing was widened to `pub` — confirmed for all three (each `pilot_languages()` stayed a bare
private `fn` at its new location). ✅

`#[path]` resolution: every `#[path = "…"]` literal in each plugin's
`📦️packages/🦀️rust/📦️glue.rs` was resolved against disk (relative to the glue file's directory) —
all paths exist for all three plugins (nothing in this pass moved or renamed a file, only moved
functions between two already-wired files, so this was a sanity check rather than an expected
failure point).

## cargo check — ONE run per crate, with the mandated override

All three ran as:
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/🎯️target" \
  cargo check -p <crate> --all-targets
```

- **`semio-s-plugin-lowpoly`** — `Finished \`dev\` profile [unoptimized] target(s) in 1m 03s`. 0
  lines matching `^error`. Only pre-existing warnings (unused imports, elided lifetimes,
  unnecessary qualifications, two dead-code fields/methods on `LowpolyEngine`) — none touch
  `declaration`/`pilot_languages`/`artifact_kind`/`io_registry`. **All yours; none upstream-stdio;
  zero errors.** Full log:
  `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/scratch-g3-lowpoly-check.txt`

- **`semio-s-plugin-raster`** — `Finished \`dev\` profile [unoptimized] target(s) in 1m 30s`. 0
  lines matching `^error`. Only pre-existing warnings (unused imports incl. `ArtifactBuilder`/
  `ArtifactAnalyzer`/`SemanticMutation`, elided lifetime, unused `SEMIO_RASTER_EXAMPLE_TEXT` const,
  dead `RasterEngine.artifact` field, unused `app` var) — none touch the relocated functions or the
  qualified `io_registry::entries()` call. **All yours; none upstream-stdio; zero errors.** Full log:
  `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/scratch-g3-raster-check.txt`

- **`semio-s-plugin-remodel`** — `Finished \`dev\` profile [unoptimized] target(s) in 2m 58s`. 0
  lines matching `^error`. Only pre-existing warnings (unused imports, elided lifetime, dead
  `RemodelEngine.artifact` field, unused `inner` var) plus a future-incompat-report notice
  (pre-existing, unrelated to this pass). **All yours; none upstream-stdio; zero errors.** Full log:
  `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/scratch-g3-remodel-check.txt`

None of the three plugins hit the known `semio-s-plugin-stdio` E0599 `--all-targets` failure —
`semio-s-plugin-stdio` was not even rebuilt as part of these three checks (its lib was already
cached), and no error of any kind appeared in any of the three logs. **All three builds are fully
green and VERIFIED**, not the "complete but UNVERIFIED" case.

## apa-status

`apa-status: green` — all three g3 plugins (💠️lowpoly, 🖨️raster, 📸️remodel) relocated, verified
by grep + a real `cargo check -p <crate> --all-targets` with the sccache override, zero errors, zero
newly-`pub` `pilot_languages`. One reported deviation (🖨️raster's `io_registry::entries()` call
needed qualifying, since that module stayed behind in the engine file). `🧬️mutations/**` untouched,
no artifact kind ids renamed, no `Cargo.toml`-bearing directory moved, `📕️norm`/`🧱️block` untouched.

## Files touched

- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️component.rs` (added `declaration()` + `pilot_languages()`)
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (removed same, kept `artifact_schema_registered()`)
- `✏️s/🔌️plugins/💠️lowpoly/🦀️component.rs` (call site)
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️component.rs` (added `declaration()` + `pilot_languages()`, qualified `io_registry::entries()`)
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (removed `//#region 🔖️Register` entirely)
- `✏️s/🔌️plugins/🖨️raster/🦀️component.rs` (call site)
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs` (added `declaration()` + `pilot_languages()`)
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (removed `//#region 🔖️Register` entirely)
- `✏️s/🔌️plugins/📸️remodel/🦀️component.rs` (call site)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/scratch-g3-lowpoly-check.txt` (new, cargo log)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/scratch-g3-raster-check.txt` (new, cargo log)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/scratch-g3-remodel-check.txt` (new, cargo log)
