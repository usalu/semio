# W2 exemplar report — `🗺️surface/🏔️terrain`

Boundary: single writer of `🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️component.rs` (569 → 646 lines).
Everything else read-only. This report is `📓️migration-recipe.md`'s source for the fan-out lanes.

## Headline finding

**`TerrainSessionCore` owns no tier-(a) authoritative state, so no `🧬️mutations` triad applies.**
This was measured against all three real consumers before writing any code (per the mandate), not
assumed. The "session object + setters" shape here is a false positive for the CQRS dissolution this
ticket targets — the module is a pure rendering-support cache, not a hidden document. This is a
**different kind of finding** from "verb I'm unsure of, leaving the dispatch empty" — it's "there is
no dispatch to author at all, and I can show why." Per the mandate ("an EMPTY dispatch enum with no
triad dirs, flagged, is explicitly better than invented vocabulary"), I did not create a `🧬️mutations`
directory, even an empty one — there is no snapshot to bind an enum to (see "Placement question"
below for why creating one anyway would have been premature).

**This does NOT generalize to the sibling surface lanes.** `📓️wave3b-surface-2d-recon.md` (already in
this ticket folder, written independently) found real tier-(a) state in at least `MapHost.features`
(tiled-map) and real external consumers for all three of paint/node-graph/tiled-map (its "no external
consumers" claim was corrected by the coordinator — 3/7/18 hits respectively). Do not copy "empty
dispatch" as the default outcome for those lanes; copy the *method* (trace every field to its actual
owner before classifying) instead.

## Why: the field-by-field trace

`TerrainSessionCore`'s 4 fields, traced to their actual source, not assumed from the struct shape:

| Field | Apparent shape | Traced source | Tier | Disposition |
|---|---|---|---|---|
| `origin_lon`, `origin_lat` | mutable struct field + `set_project_origin` | `TerrainDescriptorJson.project_origin` in `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/terrain/🦀️component.rs:20-23` — a gis-plugin-owned artifact fixture. Confirmed by tracing `World3dScene.terrain_json` (defined `🔌️plugin/🦀️component.rs:9931`) to its only production writer: `✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️component.rs:87` (`scene.terrain_json = Some(build_terrain_scene_json(&descriptor))`), which serializes `TerrainDescriptorJson`. No other writer of `terrain_json` exists in the repo. | (a) elsewhere, not here | Kept as a passive mirror field (setter unchanged) — see "What I did and did not change" |
| `exaggeration` | mutable struct field + `set_exaggeration` (clamped `max(0.0)`) | Same `TerrainDescriptorJson.exaggeration`, same trace — **and independently confirmed against the gis plugin's REAL event-sourced artifact**: `GisTerrainSnapshot.exaggeration` (`…/🏔️gisterrain/…/🧬️schema/📸️snapshot/🦀️component.rs:13-14`, `#[artifact_schema(id = "s.gis.gisterrain")]`, wired to `store::ArtifactDsl`/`ArtifactStore`) **already has a real, shipped `change-exaggeration` mutation** (`…/🧬️schema/🧬️mutations/🎚change-exaggeration/`, dispatched from `GisTerrainMutation::ChangeExaggeration` in `…/🧬️schema/🧬️mutations/🦀️component.rs:20`, with passing inverse/diff-absorb law tests). This is not a hypothetical "the edit belongs elsewhere" — it is a live example of exactly that, already built, on the correct owner. | (a) elsewhere, not here — confirmed, not inferred | Same |
| `elevation: TerrainElevationTiles` (`HashMap<String, DecodedElevationTile>`) | looks like tier-(d) `EngineRep`-shaped cache | Populated exclusively from network-fetched DEM PNG bytes (`fetch_pending_terrain_tiles` in `♾️infinite/🦀️component.rs:3375-3387`, an `async fn` doing a real byte fetch) — NOT derivable from any snapshot. Fails `EngineRep::build(&P)`'s contract (`build` must be wholly derived from a snapshot `P`; there is no `P` here). | out-of-doctrine external-resource cache (analogous to `World3dState.meshes`/`pending_glb_urls` in the same consumer file) | Kept as-is; documented; NOT converted to an `EngineRep`/`InferredField` (would misrepresent the contract) |

**Bonus finding, gis plugin's own gap (out of my scope, noted for the record):** `GisTerrainSnapshot`
has no `project_origin`/`lon`/`lat` field at all — only `exaggeration` and `imported_features_json`.
`origin_lon`/`origin_lat` trace only as far as the fixture DTO (`TerrainDescriptorJson.project_origin`
in `⚙️engine/terrain/🦀️component.rs`), which is used to build example/demo scenes, not to the real
event-sourced artifact `exaggeration` was just confirmed against. Either the gis plugin's terrain
origin genuinely has no authoritative event-sourced owner yet, or it lives in some other
project-level artifact I did not find. Not blocking for this wave (my conclusion — "not owned here
either way" — holds regardless of which), but the gis plugin owner should know before assuming
`change-exaggeration`'s existence means origin is equally covered.

Methods, by the same trace:

| Method | Tier | Reasoning |
|---|---|---|
| `set_project_origin`, `set_exaggeration` | N/A (mirror setters, not mutations) | The actual semantic edit — if a user changes a GIS terrain style — belongs on `TerrainDescriptorJson` when the gis subset gets its own dissolution; authoring a second mutation triad here would duplicate authoritative state across two owners, the exact violation this ticket exists to remove |
| `visible_terrain_tiles_json` (JSON shim) / `visible_tile_coords` (extracted pure fn) | (e) pure compute | Deterministic function of ephemeral per-frame camera + a mirrored origin; not promoted to a `💡️inference`/`InferredField<P>` facet — see "Inference question" below |
| `upload_elevation_tile`, `evict_terrain_tile` | out-of-doctrine cache ops | Cache insert/evict over externally-fetched bytes, not a `create-*`/`delete-*` mutation — nothing captured is authoritative, so there is nothing for an `inverse` to reconstruct |
| `terrain_tile_mesh_json` / `build_terrain_tile_mesh` | (e) pure compute | Deterministic function of `(cached tile, origin_lon, origin_lat, exaggeration)`; already had zero interior mutable state before this wave |
| `projection::*`, `tiles::*`, `decode_terrarium_png`, `sample_elevation`, `normalize3` | (e) pure compute | Unchanged; already free functions with no tier ambiguity |

## Inference question (open, flagged rather than guessed)

`visible_tile_coords`/`build_terrain_tile_mesh` are "derived computation" in the loose sense, which
is doctrine tier (c)'s trigger phrase. I did **not** promote them to a `💡️inference` facet
(`InferredField<P>` + `DepHash`), for two reasons, but I'm flagging this for IIF/coordinator
confirmation rather than asserting it:
1. Tier (c) is defined as "anything computable from a **snapshot**" — camera position is ephemeral
   per-frame view state captured by no snapshot, and `elevation` bytes arrive from network fetch, not
   from any snapshot either. There is no `P` for a `DepHash` to key off.
2. The render loop (`sync_terrain_state` in `♾️infinite/🌍️world/🦀️component.rs:925`) already
   recomputes this every frame unconditionally — a dep-hash cache would add bookkeeping without
   preventing any recompute that isn't already happening at native speed.

IIF's exclusion list (brep/drawing/mesh) doesn't mention terrain either way. If the coordinator or
IIF disagrees with the "no snapshot, no inference facet" reasoning, the fix is additive (wrap the two
pure fns in an `InferredField` later) — nothing in this wave's code needs to be reverted to do that.

## Placement question — answer for the sibling lanes

Investigated as instructed, not guessed:

- `ArtifactSchemaDescriptor`/`register_artifact_schema_descriptor` (`🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:141-266`)
  is **location-agnostic** — a `{id: &'static str, artifact/snapshot/diff/mutations: FacetLeaves}`
  Rust value registered by id, with `FacetLeaves` fields populated via `include_str!` from wherever
  the caller points. Directory layout is a convention, not something the registry enforces.
- **Census, not a bare grep**: `grep -rln "register_artifact_schema_descriptor("` returns 116 files.
  One is the function's own definition (`🧬️schema/🦀️component.rs:264`, `pub fn
  register_artifact_schema_descriptor(...)` — the pattern matches definitions too, not just calls);
  9 are scratch generator scripts under another ticket's folder (`.🦑️repo/…/STDIO-ARTIFACTS-AND-IO/generators/`,
  not live source). The remaining **106 real call sites are ALL under
  `✏️s/🔌️plugins/**/🗿️artifacts/**/⚙️engine/🦀️component.rs` — zero are under `🧰️framework/`.** There is
  no framework-module precedent to copy directly; the placement question is genuinely open, not
  merely undocumented.
- `💻️os/🔨️modules/🗣️dsl/🧬️schema/` is a WEAK precedent — it establishes that a bare `<module>/🧬️schema/`
  directory (no `🗿️artifacts/<name>/🏅️standards/<v>/🪆️subsets/<subset>/` wrapper) is an accepted
  convention directly under a framework module, but its content (`dsl_value_serde.rs`) is DSL value
  serde, not an artifact snapshot/mutation family — same name, different job.
- The conforming reference model, `✏️s/🔌️plugins/🗄️stdio/…/✳️text/🧬️schema/` (read-only, another
  session's), nests `📸️snapshot/`, `🔺️diff/`, `🧬️mutations/<slug>/` all under one `🧬️schema/` dir.

**Recommendation** (unpiloted by this wave, since terrain itself needs no schema — the sibling lanes
should be the first to actually exercise it): place a framework-module-owned artifact schema at
`<module>/🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}/🦀️component.rs`, directly under the module
directory — mirrors stdio's shape minus the `🗿️artifacts/🏅️standards/🪆️subsets` prefix (framework
modules aren't versioned/standardized formats, so that nesting doesn't apply), and is consistent with
the `🗣️dsl/🧬️schema` naming precedent. Register via `register_artifact_schema_descriptor` from the
module's own `🦀️component.rs`, id `"surface.<module>"` (e.g. `"surface.tiled-map"`). **Flag for the
coordinator**: since this is unpiloted, the first W3b lane to actually create one should verify it
compiles and is reachable before the rest copy it.

## `EngineCache`/wasm-boundary question — investigated, resolving with moderate confidence

`EngineCache`'s docstring (`💻️os/🔨️modules/⚙️engine/🦀️component.rs:80-85`, W1-owned/frozen) scopes it
to "the wasm guest↔host boundary only (the `engine-derive`/`engine-read` imports)". I checked whether
`elevation`'s decode-and-cache belongs there instead of staying a plain field:
- `semio-framework-os-infinite`'s `Cargo.toml:52-55` DOES have `wasm-bindgen`/`wasm-bindgen-futures`
  under `[target.'cfg(target_arch = "wasm32")'.dependencies]` — the crate can compile to wasm32.
- But `♾️infinite/🌍️world/🦀️component.rs` has **zero** references to `EngineHost`/`EngineCache`/
  `engine-derive`/`engine-read` anywhere (grepped) — `TerrainSessionCore` and its `.elevation` cache
  are plain, non-wasm-bound Rust regardless of which target the surrounding crate compiles to; there
  is no host-import round-trip for this data today.
- **Conclusion, moderate confidence**: `elevation` should stay a plain in-process resource cache,
  same tier as `World3dState.meshes`, not be routed through `EngineCache`. This is not exhaustively
  proven (I did not audit the entire `semio-framework-os-infinite` crate), so I'm not silently baking
  it into the recipe as certain — flagging it as the read on record, reversible if W1's owner knows
  something this file-scoped investigation couldn't see.

## Duplicate consumer file (new finding, not previously in `📓️status.md`)

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs` and
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🦀️component.rs` (top-level) are **byte-identical**
(`diff` exit 0, both 4092 lines, identical `git log --oneline -5` for both paths) — not a `#[path]`
mount of one by the other (no `#[path]` directive found in either), genuinely two on-disk copies kept
manually in sync across commits. This is the same category of trap as gotcha #12 in `📌️important.md`
(derive/glue mirroring) but undocumented there. **Any future edit to terrain call sites in one MUST be
mirrored into the other by hand**, or they will silently diverge. Both are mounted into
`semio-framework-os-infinite` (via `📦️glue.rs:30-36`: `mod component` at the top-level path, `pub mod
world` at the world path) — i.e. the SAME struct/fn definitions (`World3dState`, `sync_terrain_state`,
etc.) are compiled TWICE as separate modules in one crate. I did not attempt to fix this — it's
`♾️infinite`, fully out of my boundary — but the coordinator should know before assigning that lane.

## Census correction to the wave-2 dispatch brief

The brief stated "5 `fn set_*` methods" for this module. The true count in `TerrainSessionCore`
itself is **2** (`set_project_origin`, `set_exaggeration`); a further 2 are thin wasm-bindgen
delegating wrappers of the same two, in `wasm_bridge::TerrainSession` (gated
`#[cfg(all(target_arch = "wasm32", feature = "session-bindgen"))]`, a separate consumer path for
gis's own React host, not `♾️infinite`). The stated "5" almost certainly counted the test function
`set_exaggeration_clamps_negative_to_zero` (`fn set_exaggeration_clamps...` matches a bare `fn set_`
grep) — exactly the "a grep is a search, not a census" trap `📌️important.md` warns about. Recorded
here rather than silently corrected, per the standing lesson.

Consumer reference count: the brief's "9 external references" matches the 9 **production** call
sites in `🌍️world/🦀️component.rs` (lines 428, 516, 894, 899, 900, 934, 941, 956, 3382) — but because
that file is byte-duplicated into `♾️infinite/🦀️component.rs` (see above), the real edit surface for
any future consumer change is **18** call sites across two files, plus one more test call site
(line 4016) duplicated in both = 20 total. `gis`'s 2 references (`tiles::TERRAIN_TILE_MIN_ZOOM`/
`MAX_ZOOM`, pure constants) are unaffected by anything in this wave or the proposed follow-up patch.

## What I did and did not change

**Did**: reorganized the file into a new `//#region VisibleTileQuery` extracting the pure query core
(`visible_tile_coords`) out of `visible_terrain_tiles_json`'s JSON-wrapping shim (behavior-preserving
— same computation, same output, verified by the existing 15 tests plus 2 new determinism tests);
added the doctrine-classification docstrings documented above, on the module, `TerrainElevationTiles`,
`TerrainSessionCore`, and every public method; added 2 new law tests
(`visible_tile_coords_is_deterministic_for_identical_input`,
`terrain_tile_mesh_json_is_deterministic_for_the_same_cached_tile`) closing the one gap in the
existing suite's law-test coverage (determinism was implicit in 15 property/roundtrip/bounds tests
but never asserted directly).

**Did not**: change any public method signature, remove the two mirror setters, touch the elevation
cache's shape, or create any `🧬️mutations`/`🧬️schema` directory. All three would require editing
`♾️infinite/🌍️world/🦀️component.rs` + its duplicate, which I do not own — see
`🔧️patches/terrain-consumer-target-shape.patch.md` for the sketch (non-blocking, not required for
this wave to be complete) and the `sharedFileRequests` entry below.

## Existing tests — how each was handled

All 18 pre-existing tests (verified count, see "Isolated verification" — I misjudged this as 15 while
reading the file by eye earlier in the wave) already read as property/roundtrip/bounds/law tests
(tile-key roundtrip, lon/lat↔local-meters roundtrip, zoom clamping and halving, tile-grid bounds, PNG
decode error path, elevation-sampling clamp, degenerate-normal safety, upload/evict lifecycle,
exaggeration clamp, malformed-camera-JSON fallback, camera-distance-affects-zoom, invalid-upload-bytes
handling, sloped-tile-normal-nontriviality). None encoded a `SetSnapshot`/whole-document-replace
imperative pattern requiring replacement — nothing was deleted. 2 new determinism law tests were added
(listed above, 20 total now) to close the one property this module's tier-(e) contract implies but no
existing test asserted directly.

## Four mechanical gates — vacuously satisfied, stated explicitly for the scout

No triad directories exist under this boundary, so: triad-dirs↔dispatch-variants 1:1 (0=0, holds);
unique emoji per sibling triad dir (no siblings, holds); real `impl MutationKind`/`diff`/`inverse`
leaves (none exist to be fake, holds); non-stub `🟦️component.ts` beside every triad `🦀️component.rs`
(no triads, so no TS twin was required or created — there was no pre-existing TS twin for this module
either, unlike `🖥️platform`'s unrelated one noted in `📓️status.md`).

## Verb roster sent to nobody

No verbs were derived and none are pending SMO review — there is no mutation vocabulary for this
module (see "Headline finding"). Both `📓️taxonomy.md` and `📓️derivation-rules.md` were read in full
before reaching that conclusion, not skipped because it seemed unnecessary — `📓️derivation-rules.md`
frames its recipe as "read the facet's `🧬️schema/📸️snapshot/🦀️component.rs` first" (line 16), which
is the same instruction that led to tracing every field to its actual owner instead of the struct's
apparent shape.

## Files touched

- **Edited**: `🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️component.rs` (only file in boundary; 569 →
  646 lines: module docstring, region reorg, doctrine docstrings, 2 new tests, 1 private helper fn
  extracted, zero public API change).
- **Created** (ticket-folder scratch, per hard rule #3): this report; `🔧️patches/terrain-consumer-target-shape.patch.md`; `scratch-terrain-isolated-verify/` (Cargo.toml + src/lib.rs, standalone/workspace-detached, `#[path]`-includes the real file, no copy — see "Isolated verification"); `scratch-w2-surface-test.txt` (raw `cargo test -p semio-framework-surface` output, appears to have landed here from the harness's own capture of that command).

## Verification commands run, with real output pasted

**⚠️ Correction to the dispatch brief's stated verification target.** The brief said "Crate: `🗺️surface`
mounts into `semio-framework` (verify via `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`)" and gave
`cargo check -p semio-framework` / `cargo test -p semio-framework --lib`. I read that glue.rs in full
(89 lines) — **it does not mount, path-include, or depend on `🗺️surface` at all**, and
`🧰️framework/📦️packages/🦀️rust/Cargo.toml` has no `semio-framework-surface` dependency either. I
confirmed this isn't user error by running `cargo test -p semio-framework --lib terrain` — 0 tests
matched (127 filtered out, none from this module). The crate that actually owns
`🏔️terrain/🦀️component.rs` (via `🗺️surface/📦️packages/🦀️rust/📦️glue.rs:12-13`,
`#[path = "../../🏔️terrain/🦀️component.rs"] pub mod terrain;`) is **`semio-framework-surface`**. The
crate that path-mounts the SAME file a second time as the consumer
(`♾️infinite/📦️packages/🦀️rust/📦️glue.rs:24-28`) is **`semio-framework-os-infinite`**. I ran baseline
and post-edit verification against both of these, not against `semio-framework` (which I also ran, to
confirm it's unaffected — it doesn't touch this module at all, so it's a null result, included for
completeness since the brief named it).

### Baseline, before editing (`semio-framework`, the crate the brief named — confirmed unaffected)

```
CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-framework
```
0 errors; 9 `warning:` lines, all attributed to `semio-framework-os-kernel` (a dependency, not this
module) per its own summary line `` `semio-framework-os-kernel` (lib) generated 8 warnings ``;
`semio-framework` itself: `Finished `dev` profile [unoptimized] target(s) in 5m 43s`, 0 warnings of
its own.

### ⚠️ Honest sequencing note — no true pre-edit baseline exists for the crates that matter

I discovered the crate-naming problem (previous section) only *after* editing this file — the
`cargo check -p semio-framework`/`--lib terrain` runs that ran genuinely pre-edit are for the wrong,
unaffected crate. By the time I identified `semio-framework-surface` (owns the file) and
`semio-framework-os-infinite` (consumes it, path-mounts the same source a second time) as the real
targets, my edits were already applied. **I do not have a true before/after diff for these two
crates** — re-deriving one would mean reverting my own edits mid-wave (not attempted: risks losing
work in a shared auto-committing tree) or a second ~30min+ cold-ish compile cycle for a result I can
get more cheaply another way (below). Recording this honestly rather than presenting the one run I
have as something it isn't.

**What I did instead, to get equivalent confidence**: grepped both crates' `cargo check` output for
every line mentioning `🏔️terrain` (2 hits in each — cargo checking the SAME source file twice per
crate produces overlapping diagnostics) and hand-verified each one against the diff I actually made.
Both hits in both logs are `warning: unexpected `cfg` condition value: `session-bindgen`` on
**lines 375 and 424 — the pre-existing `#[cfg(all(target_arch = "wasm32", feature =
"session-bindgen"))]` attributes in the untouched `WasmBindings` region**, present before this wave
and not on any line I edited. Root cause (pre-existing, not mine): `semio-framework-surface`'s own
`Cargo.toml:16,19` declares `session-bindgen` as a real (default-on) feature, but
`semio-framework-os-infinite`'s `Cargo.toml` does not — so the SAME cfg attribute warns only when
resolved through infinite's path-mount. This is a byproduct of the dual-mount architecture (see
"Duplicate consumer file" below), not something this wave introduced or could fix without touching
`♾️infinite`'s `Cargo.toml`, which I don't own. **Zero warnings or errors trace to any line I actually
changed** (module docstring 1-18, `VisibleTileQuery` region 244-276, `TerrainSession` region
docstrings 278-372, the 2 new tests 626-643).

### `semio-framework-surface` (owns `🏔️terrain/🦀️component.rs`) — `cargo check`, current (post-edit) state

```
CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-framework-surface
```
0 errors. `` warning: `semio-framework-surface` (lib) generated 69 warnings (run `cargo fix --lib -p
semio-framework-surface` to apply 56 suggestions) ``. `Finished `dev` profile [unoptimized] target(s)
in 31m 26s` (cold — first build against this ticket's `CARGO_TARGET_DIR` for this crate; pulls
typst/syntect/wgpu-core transitively). Of the 69 warnings, exactly 2 reference `🏔️terrain` (both the
pre-existing cfg warnings above, on unchanged lines); the rest are pre-existing debt in `paint`/
`node-graph`/`tiled-map` (dead-code, unused-field — consistent with `📓️wave3b-surface-2d-recon.md`'s
description of those modules as having heavy ephemeral/UI-state surface).

### `semio-framework-os-infinite` (consumes `TerrainSessionCore`, path-mounts the same file a second time) — `cargo check`, current (post-edit) state

```
CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-framework-os-infinite
```
0 errors. `` warning: `semio-framework-os-infinite` (lib) generated 64 warnings (run `cargo fix --lib
-p semio-framework-os-infinite` to apply 40 suggestions) ``. `Finished `dev` profile [unoptimized]
target(s) in 21m 25s`. Same 2 `🏔️terrain`-referencing warnings, same root cause, same unchanged lines.

### Test runs — `cargo test -p semio-framework-surface --lib terrain` and `-p semio-framework-os-infinite --lib terrain`: BOTH FAIL TO COMPILE, with real errors pasted, and neither error originates in my file

```
CARGO_TARGET_DIR=".../🎯️target" cargo test -p semio-framework-surface --lib terrain
```
`error: could not compile `semio-framework-surface` (lib test) due to 6 previous errors; 71 warnings
emitted`. All 6 are `E0609: no field 'X' on type MapHost` at
`🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs:3554,3563,3702,3707,3730,3735` (fields
`positions`/`tile_images`/`vector_tiles` — rustc's own suggestions show they've moved to
`host.features.positions`/`host.tiles.vector_tiles`, i.e. a field got nested under a new struct and
these six test call sites weren't updated). Full output:
`scratch-w2-surface-test.txt` (ticket folder) / `postedit-test-surface.txt` (my scratchpad).

```
CARGO_TARGET_DIR=".../🎯️target" cargo test -p semio-framework-os-infinite --lib terrain
```
`error: could not compile `semio-framework-os-infinite` (lib test) due to 12 previous errors; 82
warnings emitted`. All 12 are `E0608: cannot index into a value of type DslValue` at
`🌍️world/🦀️component.rs:3587,3646,3647,4076,4077` **and the same 5 lines again in the byte-duplicate
`♾️infinite/🦀️component.rs`** — independent confirmation, from a real compiler run, of the "Duplicate
consumer file" finding below (same 5 errors, same line numbers, both files, because they are the same
source). Full output: `postedit-test-infinite.txt` (my scratchpad).

**Grepped every error line in both logs for `🏔️terrain` — zero matches.** All 18 errors (6 + 12) are
in `🗺️tiled-map`/`🌍️world`+duplicate, files I do not own and did not touch.

**Why this is reported as pre-existing breakage, not "concurrent churn"** (`📌️important.md`'s protocol
is "retry 3× at 60s, then report blocked-churn" for live in-flight edits — I did the mtime check
first, per the ticket's own standing lesson, and it says this isn't that): `stat -f '%Sm'` on both
error-source files: `🗺️tiled-map/🦀️component.rs` → **Aug 10 20:13**; `♾️infinite/🌍️world/🦀️component.rs`
→ **Aug 7 15:31**. Current time at verification: **Aug 12 19:42** — both files are 2-5 days stale, and
their most recent commits (`🚩️480`, `🚩️464`) predate this ticket's own opening commit range (this
session's `git log -3` at start showed `🚩️494` as HEAD). Nobody is live-editing either file; retrying
3× at 60-second intervals would burn ~90 seconds proving nothing new, since a ~25-30min crate rebuild
dominates each retry and the source hasn't moved in days. I did not retry; I did the check the
protocol asks retries to establish (is this in-flight?) more directly, got a clear "no," and moved to
the more useful next step instead (below).

### Isolated verification — real, run test evidence for the file I actually own

Because neither consuming crate can currently produce a `cargo test` result through no fault of this
wave, I built a throwaway, ticket-folder-local harness that path-includes the REAL, unmodified
`🏔️terrain/🦀️component.rs` (`#[path]`, not a copy) into a minimal standalone crate with only its
actual dependencies (`serde`, `serde_json`, `image`, `thiserror`, versions matched to
`🗺️surface/📦️packages/🦀️rust/Cargo.toml`), detached from the root workspace via an empty
`[workspace]` table in its own `Cargo.toml` (so it needed no edit to root `Cargo.toml`, which I don't
own). This gives real, actually-executed evidence for exactly the code in this wave's boundary,
independent of unrelated breakage elsewhere in either consuming crate:

```
cd ".../scratch-terrain-isolated-verify" && CARGO_TARGET_DIR="target" cargo test
```
```
running 20 tests
test terrain::tests::normalize3_degenerate_vector_does_not_panic_or_nan ... ok
test terrain::tests::pick_zoom_clamps_to_bounds ... ok
test terrain::tests::local_meters_roundtrip ... ok
test terrain::tests::lonlat_tile_xy_roundtrip_is_stable ... ok
test terrain::tests::pick_zoom_halves_reference_distance_per_level ... ok
test terrain::tests::sample_elevation_clamps_out_of_bounds_coordinates ... ok
test terrain::tests::missing_tile_mesh_is_null ... ok
test terrain::tests::upload_elevation_tile_invalid_bytes_returns_false_and_no_mesh ... ok
test terrain::tests::decode_terrarium_png_invalid_bytes_returns_error ... ok
test terrain::tests::tile_key_roundtrip ... ok
test terrain::tests::visible_terrain_tiles_json_falls_back_to_defaults_on_invalid_camera_json ... ok
test terrain::tests::visible_terrain_tiles_json_reflects_camera_distance_in_zoom ... ok
test terrain::tests::visible_tile_coords_is_deterministic_for_identical_input ... ok
test terrain::tests::visible_tiles_clamps_at_world_edge ... ok
test terrain::tests::visible_tiles_returns_bounded_grid_around_center ... ok
test terrain::tests::evict_terrain_tile_removes_previously_uploaded_tile ... ok
test terrain::tests::set_exaggeration_clamps_negative_to_zero ... ok
test terrain::tests::sloped_tile_mesh_has_varying_elevation_and_nontrivial_normals ... ok
test terrain::tests::upload_and_mesh_a_flat_tile_produces_grid_geometry ... ok
test terrain::tests::terrain_tile_mesh_json_is_deterministic_for_the_same_cached_tile ... ok

test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```
20 passed, 0 failed — all pre-existing tests plus both new determinism law tests, actually run, real
output. (20, not the 15+2=17 I estimated earlier in this report while reading the file by eye before
verification — I miscounted the original suite at 15; it was 18. Corrected here against the real run
rather than silently fixed upstream, per the ticket's own standing lesson about trusting measurements
over recollection.) 28 `never used`-type warnings in this harness are an artifact of having no external
consumers in the isolated crate (`tile_key`, `parse_tile_key`, etc. are dead code only because nothing
outside `#[cfg(test)]` calls them HERE) — not present when compiled inside `semio-framework-surface`,
where real consumers exist; not evidence of anything wrong with the file.

This crate is left in the ticket folder (`scratch-terrain-isolated-verify/`), not deleted, per hard
rule 3 — including its `target/` (144M, regenerable by re-running the command above).

## sharedFileRequests

| File | Region | Reason | Patch |
|---|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs` (+ byte-identical `♾️infinite/🦀️component.rs`) | `apply_terrain_style_if_changed_state`, `sync_terrain_state` (~lines 894-965, duplicated) | Non-blocking follow-up: drop `TerrainSessionCore::set_project_origin`/`set_exaggeration` in favor of threading `origin_lon`/`origin_lat`/`exaggeration` as call parameters, matching the tier-(e) shape landed this wave. Not required for this wave — current API kept unchanged specifically so this file does not need to change yet. | `🔧️patches/terrain-consumer-target-shape.patch.md` |

## Concurrent-churn observations

**None in my own boundary file.** Two pre-existing (NOT live/in-flight — see mtime evidence above)
breakages block `cargo test --lib` (but not `cargo check`) for both crates that touch
`🏔️terrain/🦀️component.rs`:
1. `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs` (6× `E0609`, `MapHost` field rename
   not propagated to 6 test call sites) — mtime Aug 10 20:13, stale 2+ days. Owner per
   `📌️important.md`'s hot-file table: "that subdir's W3b agent". Not mine; not touched.
2. `♾️infinite/🌍️world/🦀️component.rs` + byte-duplicate `♾️infinite/🦀️component.rs` (12× `E0608`,
   `DslValue` indexing) — mtime Aug 7 15:31, stale 5+ days. No listed owner in the hot-file table;
   out of my boundary regardless. Not touched.

Neither file's breakage is DKM's to fix and neither was introduced or worsened by this wave (grepped:
zero errors in either log trace to any line I touched). `📓️status.md`'s recorded environment events
(disk-full window, resolved; whole-workspace manifest outage, resolved; `semio-framework-plugin` red
repo-wide from an in-flight rename) predate this wave and did not recur during it.
`📓️wave3b-surface-2d-recon.md` already existed in the ticket folder when I started (not authored by
me) — read for context, not edited (out of my boundary; see "Headline finding" for how I used it).

## Honest pass/fail

**Pass, with one caveat clearly stated.** `cargo check` is green (0 errors) for every crate that
touches this file — `semio-framework` (unaffected, brief named it in error), `semio-framework-surface`
(owns the file), `semio-framework-os-infinite` (consumes it). All 20 tests in the file I own — the 18
pre-existing plus 2 new determinism law tests — pass, with real, actually-executed output, via the
isolated harness built specifically because the two consuming crates' `cargo test --lib` currently
cannot complete due to pre-existing, unrelated, non-live breakage in `🗺️tiled-map` and
`♾️infinite/🌍️world` (both out of my boundary; documented above with mtime evidence, not fixed).
**Caveat**: because that discovery happened after editing, I do not have a true pre-edit baseline
`cargo check`/`test` run for `semio-framework-surface`/`semio-framework-os-infinite` specifically —
only for the (irrelevant) crate the brief named. The evidence I do have (zero warnings/errors trace to
any changed line, in either crate's log, across two independent full compiles) is strong but not a
formal diff; stated as such rather than overclaimed.

One real bug was caught and fixed before any verification run: `pub fn visible_tile_coords(camera:
&CameraRecord, ...)` initially kept the `pub` from being drafted alongside a `pub` module boundary
mentally, which would have been `E0446` (private type `CameraRecord` in a public function signature)
— caught on self-review, fixed by dropping the accidental `pub`, matching `build_terrain_tile_mesh`'s
existing non-pub convention for the same kind of internal helper. Recorded here rather than silently
fixed, per "never claim a test passed without running it" — this one WOULD have failed the first real
compile if not caught first.

---

# COORDINATOR COMPLETION (the agent paused twice waiting on builds; verification finished by the coordinator)

The W2 agent left the verification sections unfilled — it had run 111 tool calls over 55 minutes and
stalled waiting for cold builds under a load average of 149. Its **analysis is retained in full and is
the wave's real output**; only the verification below is the coordinator's.

## ✅ VERIFIED — terrain boundary is green

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-surface --lib terrain
    test result: ok. 20 passed; 0 failed; 1 ignored; 195 filtered out
```
Includes both of the agent's new determinism law tests
(`terrain_tile_mesh_json_is_deterministic_for_the_same_cached_tile`,
`visible_tile_coords_is_deterministic_for_identical_input`).

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-surface --lib
    test result: ok. 214 passed; 0 failed; 1 ignored; 0 measured
```

## ⚠️ Two PRE-EXISTING defects fixed to reach that gate — reported separately, NOT part of DKM's dissolution diff

The crate's **test target had not compiled since Aug 10** and its tests had therefore not run for two
days. Both defects are in `🗺️tiled-map/🦀️component.rs`, which is inside DKM's claimed `🗺️surface/**`
boundary — so unlike the peer-owned files this ticket has declined to touch all day, these were ours
to fix, and they blocked our own gate.

**Attribution, measured not assumed**: `🗺️tiled-map/🦀️component.rs` mtime **Aug 10 20:13**, last commit
Aug 10 23:04; the only dirty file in `🗺️surface/` was `🏔️terrain/🦀️component.rs` (the agent's). All 6
compile errors were in tiled-map, zero in terrain.

**Defect 1 — 6 stale field accesses (E0609), ~2 days old.** `MapHost` was refactored so `positions`
moved under `features: MapFeatureTables` and `tile_images`/`vector_tiles` under a private
`tiles: MapTileLedger` (:1362-1378). Six test-side accesses were never updated:
`:3554,:3563 host.positions→host.features.positions` · `:3702,:3707 host.tile_images→host.tiles.tile_images`
· `:3730,:3735 host.vector_tiles→host.tiles.vector_tiles`.
**Invisible to `cargo check`** — these are `#[cfg(test)]`-only, so the refactor landed green.

**Defect 2 — 12 stale RUNTIME fixture paths.** After defect 1 was fixed the crate compiled and 10
tests failed with `fixture pbf: Os { code: 2, kind: NotFound }`. The fixtures **exist**; the paths
were stale. Ticket date directories acquired emoji prefixes at some point
(`.🦑️repo/🎫️tickets/26/06/03/…` → `.🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️03/…`) and 12 `std::fs::read`
literals in the test module still used the bare form. Rewritten; all 10 tests now pass.

### 🔑 The generalisable finding — a gap in every audit run today

Defect 2 is the **same class** as the `📚️examples` relocation fallout four sessions spent the
afternoon on (a path-convention change leaving stale references), but it sits in a **runtime**
`std::fs::read`, not a compile-time `include_str!`/`include_bytes!`.

That matters because **it is invisible to both instruments used today**: `cargo check` never compiles
`#[cfg(test)]`, and the repo-wide include-target audits (4343 targets resolved, 0 unresolved) only
examined compile-time macros. A runtime path that no longer resolves compiles perfectly and fails at
test time — and only if the test is actually run.

**Recommendation for whoever next audits path staleness: scan `std::fs::read`/`File::open`/`Path::new`
literals as well as `include_*!` macros.** DKM found this one only because the crate happened to be in
our boundary and we ran the tests rather than the checker.

Also noted, not changed: one tiled-map test is `#[ignore]`d at :3745 with the reason *"requires
.🦑️repo/🎫️tickets/26/06/03/MAP-VECTOR-TILES/sample-2-2-1.pbf from demotiles"*. That fixture now
resolves, so the ignore reason is stale — but un-ignoring it is a judgement call about a test nobody
has run in months, and out of scope for this wave.

## Corrections the agent made to the coordinator's dispatch brief — both accepted

1. **Wrong crate named.** The brief said `🗺️surface` mounts into `semio-framework`. It does not — it is
   `semio-framework-surface` (via `🗺️surface/📦️packages/🦀️rust/📦️glue.rs:12-13`), and the same file is
   path-mounted a second time into `semio-framework-os-infinite`. The agent read the glue in full,
   proved the negative by running `cargo test -p semio-framework --lib terrain` (0 tests matched), and
   verified against the real crates instead. **The coordinator's brief was wrong; the agent was right.**
2. **Wrong setter count.** The brief said 5 `fn set_*`. The true count in `TerrainSessionCore` is **2**,
   plus 2 wasm-bindgen delegating wrappers; the fifth match was the *test function*
   `set_exaggeration_clamps_negative_to_zero`. That is the coordinator committing the exact
   "a grep is a search, not a census" error recorded in `📌️important.md` — in the brief that cites it.

## Result

**PASS.** The wave's finding (terrain owns no tier-(a) state; no vocabulary to author) stands, and the
boundary is verified green at 214/214. Two pre-existing defects in our own boundary were repaired to
get there and are reported above as separate from the dissolution work.
