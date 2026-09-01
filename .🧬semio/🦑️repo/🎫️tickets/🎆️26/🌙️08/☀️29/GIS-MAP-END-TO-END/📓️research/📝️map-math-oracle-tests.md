# Web-Mercator / tile-selection oracle tests for `s.gis` tiled-map

## Recon findings

- `🦀️component.rs:279-345` (`mod projection`) and `:328-465` (`mod tiles`) are the target math:
  `lonlat_to_world`/`world_to_lonlat` (Web-Mercator normalized to `WORLD_HALF = 1`), `tile_world_rect`,
  `visible_tiles`/`visible_tile_cursor`/`tile_key`/`tile_key_ancestors`/`tile_retention_keys`. All of
  these are `pub fn` inside `pub mod projection` / `pub mod tiles`, and both mods are reachable from
  outside the crate via `semio_framework_surface::tiled_map::{projection, tiles}` (the crate's
  `📦️glue.rs:17-18` mounts `component.rs` as `pub mod tiled_map`). `MapHost` and its `pub fn new`,
  `set_size`, `set_camera`, `fit_world_camera`, `wheel_screen`, `pointer_down/move/up_screen`,
  `pick_raster_tile_zoom`, `pick_vector_tile_zoom` (all `pub`, camera/viewport fields `pub`) are also
  reachable. `active_map_lod`, `viewport_lon_span_degrees`, `resolve_map_lod_index_from_span`,
  `GIS_MAP_LOD_MAX_SPAN_DEG`, `GIS_MAP_LOD_TILE_Z`, `MAX_VISIBLE_TILE_REQUESTS` are crate-private
  (`fn`/`const` without `pub`) — reachable ONLY from `component.rs`'s own `mod tests`.
- The repo's `🧪️oracle/🔣️.json` + `🗿️artifacts/🏅️standards/.../🪆️subsets` convention (referenced by the
  ticket, and by `SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING`) is **Protocol v2**, a specific
  machine for one thing: a document's typed runtime-mutation vocabulary (create/delete/reorder/replace
  on an artifact's schema), checked outcome-class by outcome-class against a qualifying third-party
  oracle. `tiled-map` is a `🧰️framework/🔨️modules` surface module, not a `🗿️artifacts` case owner — it
  has no standard/subset tree and no mutation vocabulary, so this machinery does not apply here.
- The convention that DOES apply to a framework module's pure function (confirmed working precedent:
  `🎠️kernel/🧪️tests/satisfy-version-requirements/{🥒️.feature,🟦️.ts}`, oracle `semver`) is the
  domain-neutral one, driven by `🔣️taxonomy.json`'s `testsDirName`/`testFeatureFileKindId`/
  `testAdapterFileKinds`/`testFixturesDirName` and auto-discovered by
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🔌️nx-plugin.mjs` (`createNodesV2: ["**/*.feature", …]`)
  into one nx project per `<owner>/🧪️tests/<case-slug>/🥒️.feature`, with `test`/`test-oracle`/
  `test-subject`/... targets generated automatically — **no project.json edit needed**. No prior case
  uses a Rust adapter this way (only TS, and only inside the mutation-protocol tree for Rust), so
  wiring a `🦀️.rs` case-adapter through the full plan/scenario coordinator for this case is
  unprecedented and its `oracleHostPackages` resolution (needed for a non-artifact owner) is unverified
  — **not attempted**, to avoid shipping something that looks wired but silently no-ops.
- Third-party oracle actually available offline: **none** pre-installed (`mercantile`/`pyproj` were
  not in `.venv`; no JS Mercator/tile package in `node_modules`). Network to PyPI is reachable, so
  `mercantile` (pure Python, zero runtime deps, canonical Mapbox slippy-tile library implementing the
  published EPSG:3857 forward/inverse projection and OSM/XYZ tile numbering) was added as a **test-only**
  dependency: `uv add --group test mercantile` → `pyproject.toml`'s `[dependency-groups] test` gained
  `"mercantile>=1.2.1"` (one line; `uv.lock` updated). This is exactly what CLAUDE.md requires
  ("test/dev-only oracle is fine and required") and does not touch any runtime dependency.

## What was built (and what is directly runnable/verified right now)

1. **Fixture** — `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/web-mercator-tile-oracle/🧫️fixtures/🔣️vectors.json`
   47 vectors: 10 `projection` (lon/lat → worldX/worldY, including antimeridian and >MAX_LAT clamp
   cases), 9 `tileNumbering` (lon/lat/z → x/y), 12 `tileBounds` (z/x/y → west/south/east/north,
   including the two derived from every `tileNumbering` entry so both arrays are mutually
   cross-checked), 16 `lodBands` (span deg → lodIndex/tileZ, specification vectors — see below).
   `worldX`/`worldY`/tile numbers/tile bounds are all generated FROM `mercantile`, independently of
   this repository's Rust (`mercantile.xy()` gives EPSG:3857 meters; normalizing by `sphereRadius * pi`
   gives exactly `projection::lonlat_to_world`'s `WORLD_HALF = 1` convention — the algebra is spelled
   out in the fixture's own `oracle.rationale` field and in the `.feature` file).
2. **Feature spec** — `…/web-mercator-tile-oracle/🥒️.feature`, tagged `@oracle-mercantile`, one
   scenario per fixture category plus the requested invariants (cursor-anchored zoom, pan round-trip,
   zoom round-trip, visible-tile-count bound), written in the same style as
   `🎠️kernel/🧪️tests/satisfy-version-requirements/🥒️.feature`.
3. **Python oracle adapter** — `…/web-mercator-tile-oracle/🐍️.py`. Re-derives every `projection`/
   `tileNumbering`/`tileBounds`/`lodBands` value from `mercantile` (or, for `lodBands`, from the
   fixture's own transcribed `lodConstants`) and asserts agreement with the frozen fixture. **Run for
   real:**
   ```
   $ .venv/bin/python3 "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/web-mercator-tile-oracle/🐍️.py"
   PASS: mercantile 1.2.1 agrees with all 47 frozen fixture vectors
   ```
4. **Rust integration test (real, new file, does not touch either locked file)** —
   `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/tests/🗺️tiled_map_mercator_oracle.rs`. Cargo
   auto-discovers any `.rs` under a crate's `tests/` dir with zero manifest changes. It reads the SAME
   `🔣️vectors.json` (via `CARGO_MANIFEST_DIR`-relative path) and asserts, through the crate's real
   public API:
   - `projection::lonlat_to_world` matches every `projection` vector (tol `1e-9` on world units).
   - `tiles::visible_tiles` — pointed at each `tileNumbering` fixture's lon/lat via a real
     `Camera`/`Viewport` (a sub-pixel viewport at extreme zoom, so the production windowing code can
     only resolve the single containing tile) — matches mercantile's XYZ tile number. This exercises
     the actual tile-selection pipeline, not a reimplementation of the bucketing arithmetic.
   - `projection::tile_world_rect` + `world_to_lonlat` on its corners matches every mercantile
     `tileBounds` bbox (tol `1e-6`°).
   - `MapHost::wheel_screen` cursor-anchored zoom invariant, `pointer_down/move/up_screen` pan
     round-trip invariant, wheel-in-then-out zoom round-trip invariant, and a visible-tile-count bound
     (`<= 256`, matching the private `MAX_VISIBLE_TILE_REQUESTS` — see caveat below) via
     `pick_raster_tile_zoom`/`pick_vector_tile_zoom`.
   **Verification status: attempted, inconclusive under contention.** `cargo test --package
   semio-framework-surface --test 🗺️tiled_map_mercator_oracle -- --nocapture` was run for real but did
   not finish inside this session: `ps aux` showed at least one OTHER concurrent `cargo test -p
   semio-framework-surface` (no test filter) already holding the shared `target/` build lock — almost
   certainly one of the two other sessions this ticket says are live-editing
   `🗺️tiled-map/🦀️component.rs`/`TiledMapHost/🟦️component.tsx` right now, testing their own in-flight
   change to the same crate. This is exactly the "Concurrent Cargo Workspace Churn" situation (shared
   target dir, another session's build/test holding the lock for the length of ITS run) — not a defect
   in the new test. **Re-run when the workspace is quiet:**
   ```
   cargo test --package semio-framework-surface --test 🗺️tiled_map_mercator_oracle
   ```
   Every symbol the test imports (`tiled_map::projection::*`, `tiled_map::tiles::visible_tiles`,
   `tiled_map::{MapHost, MAP_CAMERA_ZOOM_MIN}`, `tiled_map::canvas::camera::{screen_to_world, Camera,
   Viewport}`, `tiled_map::Point`) was individually confirmed `pub` and reachable by reading
   `🦀️component.rs`, `📦️glue.rs`, the `infinite_canvas` camera module and the `📐️geometry/⚙️engine`
   `Point`/`Rect` types (`Point` derefs to `kurbo::Point` for `.x`/`.y`; `Rect` exposes `x0()/y0()/x1()/
   y1()` — its inner `kurbo::Rect` field is `pub(crate)`, not reachable directly). I could not get a
   green `cargo test` run inside this session; treat the test as **written and self-consistent by
   inspection, not yet confirmed by a passing run** until re-executed on a quiet workspace.
5. **`mod tests` diff (NOT applied — `🦀️component.rs` is locked by two other live sessions this
   ticket).** Adds 3 tests needing crate-private symbols the external integration test cannot reach:
   `lod_band_selection_matches_frozen_specification_vectors` (checks `resolve_map_lod_index_from_span`
   + `GIS_MAP_LOD_TILE_Z` against the fixture's `lodBands`), `active_map_lod_tracks_the_same_bands_as_
   the_span_resolver` (cross-checks `active_map_lod` against the same resolver at several spans),
   `visible_tile_count_never_exceeds_max_visible_tile_requests` (same bound as the external test, but
   against the real private `MAX_VISIBLE_TILE_REQUESTS` constant instead of a duplicated literal).
   Ready-to-apply unified diff (insert point: end of `mod tests`, immediately before its closing `}` at
   the file's current line 5247):

   ```diff
   --- a/🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs
   +++ b/🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs
   @@ -5243,6 +5243,62 @@
            let viewport = Viewport { width: 4000, height: 4000, dpr: 1.0 };
            let cover = super::projection::cover_zoom_for_viewport(&viewport);
            assert!(super::clamp_map_zoom_for_viewport(0.0, &viewport) >= cover);
        }
   +
   +    // #region 🔖️MercatorOracleFixture
   +    // 🌐️ `lodBands`/`lodConstants` are specification vectors, not third-party-checked — no external
   +    // reference exists for this repository's own `GIS_MAP_LOD_MAX_SPAN_DEG`/`GIS_MAP_LOD_TILE_Z` band
   +    // scheme. `projection`/`tileNumbering`/`tileBounds` in the same fixture ARE checked against the
   +    // `mercantile` third-party library, at `../📦️packages/🦀️rust/tests/🗺️tiled_map_mercator_oracle.rs`
   +    // (external integration test, reachable only through this crate's public surface) — these two
   +    // files intentionally read the identical fixture so oracle and subject compare the same inputs.
   +    // @see 🧪️tests/web-mercator-tile-oracle/🥒️.feature
   +    const MERCATOR_ORACLE_FIXTURE: &str = include_str!("🧪️tests/web-mercator-tile-oracle/🧫️fixtures/🔣️vectors.json");
   +
   +    #[test]
   +    fn lod_band_selection_matches_frozen_specification_vectors() {
   +        let fixture: serde_json::Value = serde_json::from_str(MERCATOR_ORACLE_FIXTURE).expect("fixture json");
   +        for entry in fixture["lodBands"].as_array().expect("lodBands array") {
   +            let span = entry["spanDeg"].as_f64().expect("spanDeg");
   +            let expected_idx = entry["lodIndex"].as_u64().expect("lodIndex") as usize;
   +            let expected_tile_z = entry["tileZ"].as_u64().expect("tileZ") as u32;
   +            let idx = super::resolve_map_lod_index_from_span(span);
   +            assert_eq!(idx, expected_idx, "lod index for span={span}");
   +            assert_eq!(super::GIS_MAP_LOD_TILE_Z[idx], expected_tile_z, "tile z for span={span}");
   +        }
   +    }
   +
   +    #[test]
   +    fn active_map_lod_tracks_the_same_bands_as_the_span_resolver() {
   +        let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
   +        for &span_probe in &[180.0, 50.0, 20.0, 8.0, 2.0, 0.6, 0.2, 0.05] {
   +            // Reconstruct a camera whose viewport_lon_span_degrees is approximately span_probe by
   +            // scaling zoom from the world-fit camera; exact span reproduction isn't needed, only that
   +            // active_map_lod's band agrees with resolve_map_lod_index_from_span for whatever span the
   +            // camera actually produces.
   +            let cam = super::projection::default_world_camera(&viewport);
   +            let scale = 180.0 / span_probe.max(1e-6);
   +            let scaled = Camera { x: cam.x, y: cam.y, zoom: cam.zoom * scale };
   +            let span = super::viewport_lon_span_degrees(&scaled, &viewport);
   +            let expected_idx = super::resolve_map_lod_index_from_span(span);
   +            let lod = super::active_map_lod(None, &scaled, &viewport);
   +            assert_eq!(lod.id, super::GIS_MAP_LODS[expected_idx].id, "active_map_lod band for span={span}");
   +        }
   +    }
   +
   +    #[test]
   +    fn visible_tile_count_never_exceeds_max_visible_tile_requests() {
   +        let viewport = Viewport { width: 1920, height: 1080, dpr: 2.0 };
   +        for zoom in [super::MAP_CAMERA_ZOOM_MIN, 5_000.0, 500_000.0, super::MAP_CAMERA_ZOOM_MAX] {
   +            let mut host = super::MapHost::new();
   +            host.set_size(viewport.width, viewport.height, viewport.dpr);
   +            host.set_camera(0.0, 0.0, zoom);
   +            let raster_z = host.pick_raster_tile_zoom();
   +            let vector_z = host.pick_vector_tile_zoom();
   +            assert!(visible_tiles(&host.camera, &host.viewport, raster_z).len() <= MAX_VISIBLE_TILE_REQUESTS);
   +            assert!(visible_tiles(&host.camera, &host.viewport, vector_z).len() <= MAX_VISIBLE_TILE_REQUESTS);
   +        }
   +    }
   +    // #endregion 🔖️MercatorOracleFixture
    }
    // #endregion 🔖️Tests
   ```
   Not compiled (can't touch the file). Every symbol it uses (`resolve_map_lod_index_from_span`,
   `GIS_MAP_LOD_TILE_Z`, `GIS_MAP_LODS`, `active_map_lod`, `viewport_lon_span_degrees`,
   `MAX_VISIBLE_TILE_REQUESTS`, `MapHost`, `MAP_CAMERA_ZOOM_MIN/MAX`) is already used the same way
   elsewhere in the existing `mod tests`, so the access pattern is proven; only this new combination is
   unverified. Apply with `git apply` or by hand once the other two sessions are done with the file,
   then run `cargo test -p semio-framework-surface`.
6. **Dependency registration** — `pyproject.toml`: `mercantile>=1.2.1` added to `[dependency-groups]
   test` via `uv add --group test mercantile` (also updated `uv.lock`). Test-only, not reachable from
   any production path.
7. **`launch.json` registration** — Phase 3 finding: `@semio-tech/framework-surface-rs:test` (which
   now covers the new integration test) is an EXISTING nx target
   (`🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📋️project.json`, running `bun ./📜️script.ts test`
   → `runCargoTestBudgeted(["semio-framework-surface"], …)`) that had **no launch.json entry at all**.
   Added to `.vscode/🧩️launch.seed.jsonc` (the seed the dev server regenerates `.vscode/launch.json`
   from), immediately after the last `"4_gate"` entry (`⚖️gate🧪️test🧹️gc`, order `425`), matching the
   `⚖️gate🏪️store🧪️test` → `bun x nx run @semio-tech/framework-os-kernel:test` precedent exactly:
   ```json
   {
     "name": "⚖️gate🗺️surface🧪️test",
     "type": "node-terminal",
     "request": "launch",
     "command": "bun x nx run @semio-tech/framework-surface-rs:test",
     "cwd": "${workspaceFolder}",
     "presentation": { "group": "4_gate", "order": 425.1 }
   }
   ```
   Did not touch the currently-generated `.vscode/launch.json` itself (it is being actively
   regenerated this session per the ticket's own note); the seed change is what "survives
   regeneration" per the ticket's own instruction.
8. **`🖥️launch.ts` registry** — inspected; it only generates the dynamic `"3_dev"` playground-preview
   section from discovered dev configs, not the hand-authored `"4_gate"` test gates (which live only in
   the seed `.jsonc`). No change needed there for this gate.

## Explicitly not done, and why

- **Full Protocol-v2 / nx auto-discovery wiring of the `.feature` case** (`test-oracle`/`test-subject`/
  `test-parity` nx targets that `nx-plugin.mjs` would auto-generate for
  `…/tiled-map/🧪️tests/web-mercator-tile-oracle/🥒️.feature`) was NOT attempted end-to-end. The
  coordinator that turns a `.feature` + adapters into a `plan.json` for the Python/Rust hosts needs an
  `oracleHostPackages` entry resolved from "the nearest contributor at or above the case owner" for a
  Rust subject adapter, and `🗺️surface`/`🗺️tiled-map` have no `🧪️oracle/🔣️.json` today. Adding one
  blind, without a working example of a non-mutation-protocol Rust case adapter anywhere in the repo to
  copy, risked shipping a case that LOOKS registered but silently produces zero coverage (the exact
  "Taxonomy Filename Drift Blinds Discovery" failure mode). The `.feature` file and fixture are in
  place and nx should already list a project for it (verify with `bun nx show projects | grep
  tiledmap`, predicted name `test-framework-modules-surface-tiledmap-f7a0e3-web-mercator-tile-oracle`
  — recompute the hash if the case slug changes: `sha256("🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map")`
  truncated to 6 hex chars), but its `test-subject`/`test-oracle` targets are unlikely to produce real
  Rust-side results without that `oracleHostPackages` entry. The two verified, working test surfaces
  (Python oracle script + Rust `cargo test` integration test) do not depend on this machinery at all.
- **`mod tests` diff not applied** — `🦀️component.rs` is being actively edited by two other sessions
  this ticket names; edited a copy in `/private/tmp/…/scratchpad/component.rs.patched` instead and
  diffed against an untouched copy to produce the diff above.
- **`cargo test` not confirmed green** — blocked by concurrent cargo lock contention from another
  session's own `cargo test -p semio-framework-surface`, observed via `ps aux` during this run, not a
  property of the new test.

## Files touched

- `pyproject.toml`, `uv.lock` — added `mercantile>=1.2.1` to the `test` dependency group.
- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/web-mercator-tile-oracle/🧫️fixtures/🔣️vectors.json` (new)
- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/web-mercator-tile-oracle/🥒️.feature` (new)
- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/web-mercator-tile-oracle/🐍️.py` (new)
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/tests/🗺️tiled_map_mercator_oracle.rs` (new)
- `.vscode/🧩️launch.seed.jsonc` — added `⚖️gate🗺️surface🧪️test`.
- **Not touched**: `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs`,
  `…/TiledMapHost/🟦️component.tsx` (both locked by other live sessions this run), `.vscode/launch.json`
  (actively regenerated this session).
