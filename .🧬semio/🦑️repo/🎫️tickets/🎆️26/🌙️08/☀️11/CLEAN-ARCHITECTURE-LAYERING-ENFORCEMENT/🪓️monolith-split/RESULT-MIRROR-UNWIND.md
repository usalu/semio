# 🪞️ Result: Mirror Unwind (mwu)

Executor: Sonnet (slice MIRROR-UNWIND, prefix `mwu`). Tools: `mwu-tools/`. Logs: `🗑️generated/mwu-*`. Sandboxes: `/Users/ueli/.cache/mwu/`.

## 0. Verdict

IN PROGRESS (22:20). Engine built and unit-tested (12 tests); html artifact applied, gate re-run pending; other artifacts staged only.

## 1. Classification (initial census, before the engine runs)

Compat re-export lines = `pub use semio_s_artifact_*::…` in a lib root or `🪞️mirror/🦀️.rs` (regex census, lower bound; the engine counts exact foreign edges).

| artifact | crates | crates with mirror file | mirror-file lines | lib-root lines | decision |
|---|--:|--:|--:|--:|---|
| 🔌️plugins/◻️2d | 80 | 14 | 617 | 483 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/☁️las | 7 | 1 | 5 | 66 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/♻️rewriting | 10 | 4 | 114 | 122 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/⚖️en1990 | 17 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/⚡️din18599 | 15 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/✒️writer | 9 | 4 | 90 | 129 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/➗️equation | 11 | 4 | 75 | 114 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🌀️generation2d | 26 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🌊️flow | 13 | 5 | 200 | 196 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🌍️en1997 | 15 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🌐️html | 7 | 1 | 5 | 58 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🌦️epw | 8 | 1 | 5 | 62 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🌬️din16798 | 18 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🌿️vcs | 8 | 3 | 64 | 99 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🎒️zip | 9 | 2 | 18 | 110 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🎞️gif | 10 | 2 | 44 | 78 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🎥️mp4 | 9 | 1 | 9 | 64 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🎥️shooting | 13 | 7 | 295 | 198 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🎨️svg | 11 | 4 | 47 | 106 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🎪️playground | 5 | 1 | 11 | 60 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🎬️presentation | 14 | 3 | 76 | 208 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🎬️sequence | 6 | 2 | 20 | 104 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🎵️mp3 | 8 | 1 | 6 | 58 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🏋️en1991 | 24 | 0 | 0 | 2 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🏔️gisterrain | 7 | 2 | 18 | 76 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🏗️ifc | 15 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🏛️en1992 | 15 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🏛️program | 18 | 2 | 51 | 187 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🏠️home | 8 | 2 | 21 | 86 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🏢️model | 1 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🏭️vdi3805 | 17 | 0 | 0 | 1 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/💠️lowpoly | 22 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/💬️bcf | 6 | 1 | 19 | 53 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/💾️binary | 6 | 2 | 10 | 56 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📇️iso16757 | 19 | 0 | 0 | 1 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📊️csv | 7 | 2 | 20 | 48 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📋️forms | 16 | 5 | 136 | 161 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📏️layout | 26 | 0 | 0 | 5 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📐️cad | 37 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📐️step | 21 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📑️tsv | 7 | 1 | 5 | 51 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📕️xlsx | 10 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📖️pdf | 63 | 24 | 793 | 463 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📖️playbook | 7 | 1 | 26 | 110 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📜️docx | 10 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📜️procedure | 7 | 1 | 12 | 119 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📝️md | 7 | 2 | 24 | 53 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📰️xml | 10 | 2 | 19 | 113 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📷️png | 11 | 1 | 16 | 82 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📸️jpg | 8 | 2 | 40 | 84 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📸️remodeling | 56 | 0 | 0 | 8 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📼️avi | 7 | 1 | 11 | 55 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/📽️pptx | 10 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🔊️wav | 7 | 1 | 9 | 56 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🔋️model | 69 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🔌️jack | 13 | 1 | 36 | 114 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🔌️wires | 10 | 1 | 17 | 88 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🔤️txt | 7 | 2 | 19 | 50 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🔩️en1993 | 18 | 0 | 0 | 1 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🔲️grid2d | 8 | 5 | 119 | 80 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🔺️stl | 7 | 1 | 15 | 69 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🕸️dag | 9 | 2 | 27 | 123 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🖊️dwg | 28 | 0 | 0 | 1 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🖋️dxf | 8 | 2 | 17 | 56 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🖍️drawing | 35 | 0 | 0 | 1 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🖐️5d | 45 | 12 | 697 | 355 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🖨️raster | 22 | 0 | 0 | 6 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🖼️bitmap | 10 | 3 | 80 | 150 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🖼️tiff | 8 | 2 | 24 | 75 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🗂️curation | 7 | 4 | 80 | 92 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🗒️note | 16 | 9 | 255 | 232 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🗜️deflate | 8 | 1 | 21 | 51 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🗺️gismap | 16 | 5 | 300 | 121 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🗽️obj | 7 | 1 | 8 | 57 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🧊️3d | 95 | 4 | 143 | 115 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🧊️generation3d | 89 | 0 | 0 | 1 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🧊️gltf | 51 | 8 | 282 | 286 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🧊️process3d | 21 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🧩️en1994 | 15 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🧱️din4108 | 18 | 0 | 0 | 1 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🧱️grid3d | 8 | 2 | 85 | 101 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🧱️ply | 7 | 1 | 6 | 66 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🧾️json | 8 | 3 | 29 | 84 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🧿️semio | 180 | 1 | 16 | 784 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🪐️space | 5 | 0 | 0 | 31 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🪟️bmp | 9 | 2 | 16 | 68 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🪨️en1996 | 23 | 0 | 0 | 2 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🪵️en1995 | 23 | 0 | 0 | 1 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🪶️en1999 | 14 | 0 | 0 | 0 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |
| 🔌️plugins/🫨️en1998 | 17 | 0 | 0 | 1 | (b) re-point consumers to owners, delete; root-level explicit facade API kept (a) |

## Notice from GENERATOR-TESTKIT (gtk, 22:15)

gtk edits the crate-tree generator in these places only: `🏗️authoring/📦️crate-tree/🟦️.ts` (`carveWrites` is restructured: mirrors and file texts are computed before the manifests, a `planCarveWiring` step feeds the crate and facade manifests; new exports `loadExternalGates`, `registerBuildOwnerRows` now syncs), `📝️render/🟦️.ts` (`patchCargoLock` prunes stale lock entries, new `syncBuildOwnerRows`), the new module `🧰️testkit/🟦️.ts`, and tests in `🧪️tests/📦️crate-tree/🟦️.ts` (`describe("test-support wiring")`). gtk does not touch `🧫️sandbox/🟦️.ts` or `🪞️unmirror/`. Please keep these when you edit `🟦️.ts`/`📝️render/🟦️.ts`; re-read them before every edit.
For the unwind: a consumer that stops re-exporting through a mirror and names an owner's gated item directly (`#[cfg(any(test, feature = "testkit"))]` items of the owner) needs `features = ["testkit"]` on its dev-dependency row of the owner, and the owner needs `testkit` declared. `gtk-tools/gtk-replay.ts <report.json> <artifact-dir>...` (read-only) derives that wiring from the live crates and lists MISSING/EXTRA rows per crate; run it on an artifact after an unwind to find the dev-dependency rows to add. The rule set is in `RESULT-GENERATOR-TESTKIT.md` section 3.

## 2. Engine (what actually runs)

- `L/🏗️authoring/📦️crate-tree/🪞️unmirror/` (L = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`): `🧵️use-tree` (use-statement scan/render), `🔍️resolver` (module trees from `#[path]` mounts, explicit/glob/re-export resolution, foreign-path tracking with a `cut` flag), `🟦️.ts` (planner), `📝️manifest` (dependency add/remove). Tests: `L/🧪️tests/🪞️unmirror/🟦️.ts` (12 pass).
- Decision rule per `pub use` edge into another carved crate: root-level explicit leaf of an artifact facade (a crate that has carved children below its artifact dir, e.g. `semio-s-artifact-stdio-contract`) = designed API (a), kept once and re-pointed to the owner; everything else = mirror (b): deleted, every consumer (private import, glob, inline path, other crate) re-pointed to the crate that owns the item; globs become explicit imports of the names the file uses; emptied nest modules are deleted; rename aliases (`pub use X as FormQuestion`) are removed by renaming the identifiers in the consumers.
- Tools: `mwu-tools/run.ts <stage|apply> <tag> <artifact dirs…>` (engine + manifest edits + dependency-cycle check), `mwu-tools/gate.sh` (`--check-applied --base=dir:`), `mwu-tools/debug.ts` (resolve a path).

## 3. Non-mirror aliases handled by hand

| item | decision | state |
|---|---|---|
| TS `ProceduralSnapshot as Generation2dArtifact/Generation2dSnapshot`, `parseProceduralSnapshot as parseGeneration2dArtifact` and the 3d trio (6) | removed; the exports and 5 consumers use the owner names | done (10 files) |
| `pub use semio_framework_plugin_testing_caller as original_caller` (3 files) | removed; call sites use the crate name | done |
| `pub(crate) use …sqlite_snapshot_testing_backing as snapshot_test_backing` / `backing_observer` (7 files) | removed; call sites use the crate name | done |
| `pub use semio_framework_os_kernel as __kernel`, `…value as __value` (plugin root, plugin testing, window, store io, os root) | KEPT: `$crate::__kernel` macro-hygiene re-exports, downstream crates of the macros do not depend on the crate | kept |
| `pub(crate) use crate::{wire_fixture as wire, …}` (surface-fixture) | pending | |
| `Form*` aliases (39 lines, forms plugin) | handled by the engine rename alias rule when the forms artifact is unwound | pending |
