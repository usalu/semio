# Packet report — `📸️remodel` (Tier C, 22,138 LOC, single directory)

Target: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine` (only
one engine dir in this plugin — confirmed by `find`).

## Destination per region

| region | source | destination | why |
|---|---|---|---|
| `RemodelEngine` struct + impl | root `⚙️engine/🦀️component.rs` | **DELETED** | 0 external refs anywhere in the plugin, no trait impl — the same fossil-of-`ArtifactEngine` shape as `Block2dEngine` |
| `io_registry` module (`ComposerEntry` table + 8 export composers) | root `⚙️engine/🦀️component.rs` | `🚪️io/🦀️component.rs` | rule 5 |
| `next_remodel_id` | root, `🔖️Ids` | `🧬️schema/🦀️component.rs`, new `🔖️Ids` region | rule 3 (pure id generator) |
| `stage_display`, `video_codec_from_label` | root, `🔖️EngineMapping` | `🧬️schema/🦀️component.rs`, new `🔖️Codecs` region | pure document-type→value, **no** app/engine dependency (see below) |
| `build_engine_params`, `map_engine_stage`, `camera_world_position`, `camera_pose_preview`, `watertight_snapshot`, `build_qc_snapshot`, `raster_to_png_asset`, `video_codec_to_artifact`, `describe_video_probe` | root, `🔖️EngineMapping` | **new** `🎛️apps/📸️remodel/⚙️engine/🦀️component.rs` | each takes/returns a photogrammetry-engine type — see deviation below |
| `mesh_data_to_semio_mesh`, `mesh_to_ply_bytes`, `mesh_to_las_bytes`, `remodel_mesh_from_document`, `remodel_png_export` | root, `🔖️Exporters` | `🚪️io/🦀️component.rs`, new `🔖️Exporters` region | rule 5 (codec/serializer dispatch onto stdio) |
| `remodel_io`, `remodel_photos_in_port`, `remodel_mesh_out_port` | root, `🔖️Io` | `🎛️apps/📸️remodel/🦀️component.rs` | rule 4 (`AppIo`/`MediaPortSpec`) |
| `payload_from_data_url`, `decode_still_image` | root, `🔖️Payloads` | `🎛️apps/📸️remodel/🦀️component.rs` | all 3 real consumers are app-side; no schema type in the signature |
| 10 topic files (`🌟️feature`/`🌫️dense`/`🎥️video`/`🏃️motion`/`🏭️reconstruction`/`📷️camera`/`📸️sfm`/`🖼️images`/`🗺️geo`/`🥽️mesh`) | whole subdirs | `🎛️apps/📸️remodel/⚙️engine/<topic>/🦀️component.rs` | rule 7 + deviation below — pure photogrammetry algorithms are the app's own compute engine |
| Tests | every region above | travel with their subject | rule 8 |

## ⚠️ Deviation 1 (the important one): where do the ten pure-algorithm topic files go?

The packet's region map has no bucket for "large pure-algorithm library that touches neither
`RemodelSnapshot` nor any app type" — the five prior packets (block2d, flow, writer, sequence,
norm+block) were all small enough not to have one. `📸️remodel` does: 10 files, 21,538 of the
22,138 LOC, confirmed by grep to reference **neither** `io_registry`/`AppIo`/`RemodelSnapshot`/
`RemodelEngine` **nor** any type from any other plugin (0 cross-plugin refs both directions).

The master ticket's own D6 doctrine sends "pure algorithm, called by <2 plugins" one level up to
`✏️s/🔨️modules/<name>/⚙️engine/` — but that is the **top-level** `✏️s/🔨️modules` area (a distinct
`"s-module"`-role owner with its own `Cargo.toml`, confirmed against `🔣️taxonomy.json`'s
`ecosystems`/`roles`/`areas` keys and against the *only* two real precedents in the repo,
`🌊️flow/🔨️modules/🧮️compute` and `🔱️trinity/🔨️modules/🔌️jack/{🐚️shell,🧠️lsp}` — both are separately
packaged tools with their own manifest, not loose files sharing the plugin crate). Creating a new
crate + registering it as a workspace member is exactly the "dangling workspace member" hazard
`📌️important.md` calls out as catastrophic (repo-wide `cargo` breakage), and it is **outside**
`✏️s/🔌️plugins/📸️remodel`, which this packet's hard rules forbid leaving.

**Decision: the ten topic files move to `🎛️apps/📸️remodel/⚙️engine/`** — the app's own engine slot,
which already existed as a taxonomy-legal empty stub (confirmed: `appChildDirs`/`appComponentDirs`
both list `⚙️engine`; `find` showed 0 files there before this packet). Reasoning:

1. It is **inside** the plugin boundary — no new crate, no workspace edit, zero blast radius beyond `📸️remodel`.
2. It matches the ticket's own thesis verbatim: *"An app has an engine... behaviour belongs to the app that edits the artifact."* The ten files together **are** the `RemodelPlayApp`'s reconstruction pipeline — SFM, dense stereo, mesh fusion, video decode — not a service shared by any other plugin.
3. It forced a **second, correct** move: every function that bridges a document/schema type to one of these engine types (`build_engine_params`, `camera_pose_preview`, `watertight_snapshot`, `build_qc_snapshot`, `raster_to_png_asset`, `video_codec_to_artifact`, `describe_video_probe`, `map_engine_stage`) **cannot** live in `🧬️schema/💡️inferences/` as the generic map would suggest, because that would make the artifact depend on the app — the one rule the original file's own docstring already stated ("artifacts must never depend on apps"). These nine functions moved to the new `🎛️apps/📸️remodel/⚙️engine/🦀️component.rs` instead. Two functions in the same original region — `stage_display` and `video_codec_from_label` — take **only** document types with no engine dependency, so those two correctly went to `🧬️schema/`.

This is the one place later Tier-C/B packets should read before repeating the "derived compute →
schema/inferences" rule blindly: **check every EngineMapping-shaped function's signature
individually — if it names an engine/app type, it is not schema-safe.**

## io_registry shadow hazard — checked, not assumed

`📸️remodel/🗿️artifacts/📸️remodel` was already flagged in `📓️io-registry-shadow-list.md` as carrying
a shadowing `io_registry` (artifact root, `&'static [&'static ComposerEntry]`) over the real one
(`&'static [ComposerEntry]`). Two call sites needed re-qualifying after the real one moved:

- `declaration()`'s `.composers(...)` call (artifact root, was `standards::v1::engine::io_registry::entries()`) → `crate::artifacts::remodel::standards::v1::subsets::any::io::io_registry::entries()`
- the shadow module's own internal alias `use ...::engine::io_registry as v1;` → `use ...::subsets::any::io::io_registry as v1;`

Verified after the edit: `grep "io_registry::entries()"` → exactly one call site, fully qualified;
`grep "[^:]io_registry::"` outside `crate::...::io::io_registry`/`mod io_registry` declarations →
zero. The relocated `io_registry` module body itself needed **no** internal path changes — every
reference inside it (`crate::artifacts::remodel::io::import::...`, `...io::export::...`,
`...schema::RemodelComposer`/`RemodelBuilder`) was already written fully qualified from crate root
against paths that remain valid at the new home.

## Call sites updated

20 references to the old `…::engine::…` path, across 9 files (verified by the "before" census run
before any edit):

| file | refs | fix |
|---|---:|---|
| `🎛️apps/📸️remodel/🦀️component.rs` | 5 (`decode_still_image` import, `remodel_io()`×2, `remodel_photos_in_port()`, `remodel_mesh_out_port()`, `images::encode_png`/`ImageRgba8`×2) | now-local calls (import deleted, symbols defined in-file) or repointed to `crate::apps::remodel::engine::images` |
| `🎛️apps/📸️remodel/🎮️commands/🚀️reconstruction/🦀️component.rs` | 1 import line, 8 symbols | split 3-way: `crate::apps::remodel::engine::{...}` (6), `crate::apps::remodel::decode_still_image` (1), `crate::artifacts::remodel::schema::next_remodel_id` (1) |
| `🎛️apps/📸️remodel/🎮️commands/📥️ingest/🦀️component.rs` | 1 import line, 8 symbols | split 4-way: `crate::apps::remodel::engine::{...}` (4), `crate::apps::remodel::{decode_still_image, payload_from_data_url}` (2), `crate::artifacts::remodel::schema::{next_remodel_id, video_codec_from_label}` (2) |
| `🎛️apps/📸️remodel/📌️panels/📄️artifact/🦀️component.rs` | 1 | `crate::artifacts::remodel::schema::stage_display` |
| `🎛️apps/📸️remodel/🎮️commands/🎯️calibration/🦀️component.rs` | 1 | `crate::artifacts::remodel::schema::next_remodel_id` |
| `🎛️apps/📸️remodel/🎚️config/🦀️component.rs` | 1 (doc comment only, not compiled) | text updated for accuracy |
| 8 of the 10 topic files (all but `📷️camera`, `🖼️images` — pure leaves) | 8 (sibling cross-refs) | prefix `crate::artifacts::remodel::engine::` → `crate::apps::remodel::engine::`, mechanical, verified 0 remaining via grep |
| `🗿️artifacts/📸️remodel/🦀️component.rs` | 2 (`declaration()` composers call, shadow module's `use ... as v1`) | fully qualified to the new `io` location (see hazard section above) |
| `📦️packages/🦀️rust/📦️glue.rs` | 1 (shim `pub mod engine { pub use super::standards::v1::engine::*; }`) | **deleted** (no compatibility shims permitted) — replaced with a new `apps::remodel::engine` wiring block mirroring the old `artifacts::remodel::standards::v1::engine` block exactly (same 10 `pub mod` + root) |

Post-edit structural verification (must be 0, is 0):
```
grep -rn "artifacts::remodel::engine\|standards::v1::engine\|subsets::any::engine" --include="*.rs" .
find . -path "*🗿️artifacts*" -name "⚙️engine" -type d
grep -rn "RemodelEngine" --include="*.rs" .
```
All three: zero matches.

## Assertion-count arithmetic (not eyeballed — `git show 20252aa16d:<path>` vs current disk)

`20252aa16d` (flag 496) is the last commit before this packet's own auto-commit (flag 497) touched
`📸️remodel`, confirmed by `git log --oneline` on the artifact schema path.

Per-file `assert(_eq|_ne)?!` occurrence count, before (11 files, old engine dir) vs after:

| file | before | after | note |
|---|---:|---:|---|
| root `⚙️engine/🦀️component.rs` | 28 | — (dissolved) | redistributed below |
| `🌟️feature` | 79 | 79 | unchanged move |
| `🌫️dense` | 46 | 46 | unchanged move |
| `🎥️video` | 154 | 154 | unchanged move |
| `🏃️motion` | 37 | 37 | unchanged move |
| `🏭️reconstruction` | 24 | 24 | unchanged move |
| `📷️camera` | 36 | 36 | unchanged move |
| `📸️sfm` | 64 | 64 | unchanged move |
| `🖼️images` | 62 | 62 | unchanged move |
| `🗺️geo` | 29 | 29 | unchanged move |
| `🥽️mesh` | 94 | 94 | unchanged move |
| **10-topic subtotal** | **625** | **625** | exact |
| new `🎛️apps/📸️remodel/⚙️engine/🦀️component.rs` (`raster_to_png_asset` test) | (part of root's 28) | 4 | |
| `🚪️io/🦀️component.rs` new regions (`mesh_to_ply`/`mesh_to_las`/`png_export` tests) | (part of root's 28) | 11 | |
| `🎛️apps/📸️remodel/🦀️component.rs` new `🔖️IoTests` region (`remodel_io` test) | (part of root's 28) | 10 | isolated via region-scoped grep |
| `🧬️schema/🦀️component.rs` new regions | (part of root's 28: 3 for `next_remodel_id`) | 8 | 3 relocated + **5 new** (`stage_display_covers_every_stage` ×1, `video_codec_from_label_recognizes_common_aliases` ×4) |
| **root subtotal relocated** | **28** | **4+11+10+3 = 28** | exact — every original root assertion accounted for |
| **grand total** | **653** | **653 preserved + 5 added = 658** | zero lost |

Test-function count: before 267 (`#[test]` fns: 261 across the 10 topics + 6 in root), after 269
(261 unchanged + 6 relocated + 2 new: `stage_display_covers_every_stage`,
`video_codec_from_label_recognizes_common_aliases` — added because those two pure functions had no
test coverage in the original file).

One test needed a genuine content change, not just a move: `png_export_round_trips_a_stored_texture_asset`
originally built its fixture PNG via `remodel_image::encode_png` (the `images` engine, then still
artifact-side). Since `images` now lives app-side and this test lives in `🚪️io/`, keeping that call
would have made an **artifact test depend on the app** — the same layering rule this whole packet
exists to enforce. Fixed by swapping the fixture for an arbitrary base64 byte string (the function
under test never validates PNG-ness, only passes stored bytes through), preserving all 3 of its
original assertions unchanged.

## Compiler verification — honest status: BLOCKED, not green, not red-on-us

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-remodel --all-targets
```
Run 4 times (structural edits, +2 immediate retries, +1 after other work). All 4 runs stop at the
**same single error**, entirely inside `semio-s-plugin-stdio`, before `semio-s-plugin-remodel` is
ever reached (`grep "semio-s-plugin-remodel"` on the full log → 0 hits — my crate is never
compiled):

```
error: couldn't read `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/.../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs`: No such file or directory (os error 2)
error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error
```

**(c) upstream** — `semio-s-plugin-stdio`, subset `✳️mesh`, path
`🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/...` — entirely inside `🗄️stdio`, a directory
this packet's hard rules explicitly forbid entering. Confirmed pre-existing and unrelated by
`git log --oneline -3` on that exact path: last touches are flags 495/493/490, all before this
packet's own auto-commit (497), and this exact failure mode (dangling `#[path]` mount after a
mutation-vocabulary rename, `subsets::brep`/`subsets::mesh`) is already documented in
`📓️packet-manifest.md`'s "stdio REGRESSED" section as another session's live work, explicitly not
to be touched here.

Because the real compiler was never reached for my crate, I did the next best thing instead of
claiming an unearned green: brace-balance-checked every file I wrote or edited (all 6: balanced),
confirmed every new/relocated symbol's type exists and every new crate-level path
(`semio_framework`, `semio_framework_os`, `semio_s_plugin_stdio`, `base64`) is a real dependency in
`📦️packages/🦀️rust/Cargo.toml`, and re-read the 10 moved topic files' headers to confirm the
`sed` prefix rewrite left them byte-clean otherwise.

**This is reported as `blocked-churn`, not `green`.** A real `cargo check` result for
`semio-s-plugin-remodel` is still owed once `🗄️stdio` compiles again.

**Run 5** (final, right before writing this report) changed shape entirely — different missing
file (`💡️inferences/🛰️component.proto`, still `subsets::mesh`), plus two new `error[E0425]`
(`SemioMeshSnapshot` not found) / `error[E0308]` (mismatched types), 603 warnings. This is
independent confirmation the failure is a **live, moving target in another session** (UCAS's
in-flight `subsets::mesh` vocabulary work), not a static bug — my crate is still never reached
(`grep "semio-s-plugin-remodel"` on this run's log → 0 hits either).

## Files touched

**Created:** `🎛️apps/📸️remodel/⚙️engine/🦀️component.rs`

**Moved (directory, content unchanged except one prefix rewrite):** `🎛️apps/📸️remodel/⚙️engine/{🌟️feature,🌫️dense,🎥️video,🏃️motion,🏭️reconstruction,📷️camera,📸️sfm,🖼️images,🗺️geo,🥽️mesh}/🦀️component.rs` (from `🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/`)

**Updated:**
- `🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `🎛️apps/📸️remodel/🦀️component.rs`
- `🗿️artifacts/📸️remodel/🦀️component.rs`
- `📦️packages/🦀️rust/📦️glue.rs`
- `🎛️apps/📸️remodel/🎮️commands/🎯️calibration/🦀️component.rs`
- `🎛️apps/📸️remodel/📌️panels/📄️artifact/🦀️component.rs`
- `🎛️apps/📸️remodel/🎮️commands/📥️ingest/🦀️component.rs`
- `🎛️apps/📸️remodel/🎮️commands/🚀️reconstruction/🦀️component.rs`
- `🎛️apps/📸️remodel/🎚️config/🦀️component.rs` (doc comment only)

**Removed:** `🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/` (entire directory, 22,138 LOC)

## Concurrent-churn observations

`semio-s-plugin-stdio` red (`subsets::mesh`, `os error 2` on a dangling `#[path]` mount) — matches
the pattern `📓️packet-manifest.md` already logged for `subsets::brep` ("third instance of this
pattern today, after `✳️drawing` twice"). Not fixed, not touched, per hard rule and per that
document's own warning that two sessions already talked themselves into "fixing" this and were
wrong both times.

## What I could not verify

A real `cargo check -p semio-s-plugin-remodel --all-targets` result — blocked upstream by `🗄️stdio`,
4 attempts, same single error every time, 0 errors ever attributed to a path this packet touched.
