# 🧩️ Snapshot Stdio DWG and STEP Editor Repairs (2026-11-24)

Tree: `/Users/ueli/.sinc/play`. Log: `.cache/b/r/m/components/runs/wrMkAas/l/cargo-release-00-b2.log` (wasm32-wasip2, release).
Nothing was compiled: no cargo or rustc was run. The fixes are checked only by reading the code.

Live (`/Users/ueli/Documents/semio`) has the **same** defects in every file here, apart from one unrelated newer-shape line in the step editors. Nothing was ported from live. Every fix repairs the snapshot against its own neighbouring crates.

## A. `semio-s-artifact-stdio-dwg-editor` (ac1024) and `semio-s-artifact-stdio-dwg-editor-ac1018`

### Causes
1. In the viewer `🪟️main/🦀️.rs`, line 3 lost its `//!` marker. The prose line is a parse error, so module `main` fails, and the viewer `🦀️.rs:92` reports `BODY_KEY` / `render` / `definition` / `WINDOW_KIND_ID` as missing. Both versions had it.
2. The editor `🦀️.rs:195/199` used `semio_s_artifact_stdio_dwg_model::…::io::{export,import}`. The model's `v_ac1024::io` module only mounts `🚪️io/🦀️.rs` (composition, registry, binary, text, sqlite), not the byte codec. In the snapshot, `encode_dwg`/`decode_dwg` live in the crates `semio-s-artifact-stdio-dwg-write` / `semio-s-artifact-stdio-dwg-read`, at `standards::v_ac1024::subsets::any::io::{export::encode_dwg, import::decode_dwg}`. The ac1018 examples, `🧪️tests/🖊️mutate-dwg-ac1024`, and the semio drawing/mesh/brep/cad io crates already use those paths. Neither editor manifest listed these crates.
3. ac1018 only: the edit and view main windows `use semio_s_artifact_stdio_dwg_snapshot_drawing::…::preview`, but the ac1018 manifest listed `semio-s-artifact-stdio-dwg-snapshot-drawing` only under `[dev-dependencies]`. This gave the error `cannot find module or crate semio_s_artifact_stdio_dwg_snapshot_drawing`. The ac1018 `DwgSnapshot` re-exports ac1024's `DwgSnapshot`, so the ac1024 codec and preview type-check for ac1018.

### Files changed
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs`: restored `//!` on line 3.
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs`: restored `//!` on line 3.
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/✏️editor/🦀️.rs`: codec paths now point to `semio_s_artifact_stdio_dwg_write` / `semio_s_artifact_stdio_dwg_read`.
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/✏️editor/🦀️.rs`: same change.
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs`: the same stale `dwg_model::…::io::import::decode_dwg` path, found by the sweep (test-only).
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml`: added `semio-s-artifact-stdio-dwg-read` and `semio-s-artifact-stdio-dwg-write` (both are workspace deps).
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml`: added `-dwg-read` and `-dwg-write`, and moved `-dwg-snapshot-drawing` from dev-deps to deps.
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/Cargo.lock`: added `-dwg-read` and `-dwg-write` to both editor entries, so a `--locked` build stays consistent. No new packages were added.

There is no dependency cycle: `dwg-read` and `dwg-write` do not depend on either editor.

## B. `semio-s-artifact-stdio-step-cc1-editor` … `-cc6-editor`

### Cause
The six editors are **separate files**, not a shared `#[path]` source. Each `✏️editor/🦀️.rs` had `fn import_media` twice in `impl ArtifactEditor`, at lines 199 and 204. The bodies were identical (`import_media_as_load::<Self>(port, media)`), and only the second carried the `/// 📂️ Opening a natural file …` doc. Both signatures match the trait, because `ArtifactMutationOutcome<M,C,D,MediaError>` = `Result<Emit<M,C,D>, MediaError>`. The fix keeps the documented definition, which is the same shape as the dwg sibling editors, and removes the undocumented copy.

### Files changed
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/✏️editor/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/✏️editor/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/✏️editor/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/✏️editor/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/✏️editor/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/✏️editor/🦀️.rs`

Live's only other difference in these files is `main::render(doc.snapshot, view_state.locale)`. That targets a newer `main::render` signature, so it was not ported. The snapshot line compiled without error.

## Sweep across `🗄️stdio`
- **Doc-comment prose without a marker:** I ran a perl heuristic over every `🦀️.rs`: a line after a `//!`/`///` line that is prose with no code token. Run against live, it flags exactly the two dwg viewer files. Run against the snapshot after the fixes, it flags nothing.
- **Duplicate `fn import_media(` / `fn export_media(` in one file:** none remain. There are also no duplicated `natural_file_*` fns, and no duplicated impl fn names in any touched file.
- **`…::io::{export,import}::{encode,decode}_*` paths:** all remaining paths resolve.
  - dwg: all now point to read/write.
  - `gltf_model` and `pdf`: these crates mount `export`/`import`, and they compiled.
  - The one `semio_s_artifact_stdio_pdf_mut_family_11::…::io::import::decode_pdf` use is in the pdf 1.4 `🔭️canvas` file. That crate is a path-preserving mirror of the pdf artifact root, and the pdf crates compiled, so I did not treat it as a defect.
- The log has only 4 `cannot find export/import in io` errors, all from the two dwg crates.

## Open risks
- Not compiled: the warnings-as-proof check and type-check are still pending the release build.
- New deps and the lockfile: the release build uses a generated components workspace under `.cache`. If its lockfile or manifest is generated from a stale copy, it must be regenerated to pick up the new `dwg-read`/`dwg-write` and ac1018 `dwg-snapshot-drawing` edges.
- The other failing crates in the same log (`semio-hub-space-app`, raster-model, sequence-schema, shooting-mutations, space-home editor/examples) are out of scope and were not touched.
