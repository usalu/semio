# Real manim on macOS, zero-touch: native cairo, consumer tests, renders

`C` = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring`, `P` = `C/📦️packages/🐍️python`, `N` = `P/🧱️native`, `T` = this ticket folder. Logs: `T/🗑️generated/python-manim/` (delete at ticket close). No `repo run/show/dependencies/dashboard`, no git command that modifies anything, no Homebrew/sudo, nothing outside the machine cache and the git-ignored `.cache/b/t/m` partition.

## Verdict

Real manim 0.22.0 now imports on this macOS arm64 host with the real pycairo 1.29.2 built against cairo 1.18.6 (no stub): `python -I -c "import cairo, manim"` prints `1.29.2 1.18.6 0.22.0`. The consumer tests, the course-coverage guards, all 22 clip imports (en, de) and two real manim renders pass. The course `📜️script.ts` provisions it by itself (`render --help` verified through it).

## 1. How the environment is declared (found)

- `P/pyproject.toml` + `P/uv.lock`: Python `==3.12.*`, `manim==0.22.0` (PyPI latest is 0.22.1, same `pycairo<2.0` requirement), `numpy==2.5.4`, group `test`.
- Native needs of manim 0.22.0 in the lock: only `pycairo 1.29.2` (meson build, needs pkg-config and cairo development files). `manimpango 0.7.1` has wheels for macOS x64/arm64 and Windows (they bundle pango), no Linux wheel (Linux builds it from the sdist against system pango). `pycairo` has wheels for Windows only (win32, win_amd64, win_arm64); macOS and Linux build the sdist.
- What the other platforms use today (unchanged): Linux/devcontainer = system packages `libcairo2-dev pkg-config libpango1.0-dev` (and equivalents) from the repo bootstrap `🐚️.sh` and `.devcontainer/Dockerfile`; Windows = the pycairo and manimpango wheels of `uv.lock`; macOS = Homebrew in `🐚️.sh` (which this task rules out).

## 2. Options and choice

- (a) wheel: rejected. PyPI has no macOS or Linux wheel of pycairo in any release (1.26.1 to 1.29.2 checked, Windows wheels only); manim 0.22.0 and 0.22.1 both require `pycairo`; no wheel-shipped package provides the `cairo` module (`cairocffi` is a different module name, `pycairo-binary` does not exist). Bumping the lock does not help.
- (b) pinned conda-forge packages unpacked by a pinned `micromamba`, pycairo built by uv from the locked sdist: CHOSEN.
- (c) cairo from source tarballs: rejected. cairo, pixman, freetype, fontconfig, glib, libpng, expat, icu, pcre2 and friends is 20+ tarballs with meson/autotools builds, needing a pinned meson, ninja and perl/python on the host: more pins, slower, more to go wrong than 26 binary packages, and Xcode CLT has no pkg-config.

Why (b): conda-forge publishes SHA-256 and MD5 for every package, so the whole closure can be pinned by exact URL and digest; relocation (prefix paths in `.pc` files and binaries) is done by micromamba, not by hand; the result is a plain prefix with `pkg-config` and `cairo.pc`, so uv builds pycairo from the sdist hash-locked in `uv.lock` unchanged (the lock is untouched). Nothing is installed for Linux/Windows.

## 3. What is fetched, from where, with which digests

Manifest (single source of truth): `N/🔣️.json`.

- `micromamba 2.10.0` single binary from `https://github.com/mamba-org/micromamba-releases/releases/download/2.10.0-0/`:
  - `micromamba-osx-arm64` sha256 `c547639f5caf742236641c362746abb4a3505774753ecd08fe341643670c4c01` (15635200 bytes)
  - `micromamba-osx-64` sha256 `8421462a1ef1df9f7acd114f9e2a24c1332ce4f4e29f44465b4acb44e2755172` (17501440 bytes)
  - both equal the digest the release API reports and the digest of the file I downloaded (arm64 and x64 both downloaded and hashed).
- 26 conda-forge packages per host (`osx-arm64`, `osx-64`, from `https://conda.anaconda.org/conda-forge/<subdir>/<archive>`), each with sha256, md5, size and build string: solved with `micromamba create --override-channels -c conda-forge --dry-run --json --platform <host>` for specs `cairo>=1.18,<1.19 pkgconf zlib freetype expat` (the last three are the `-devel` halves that carry `zlib.pc`, `freetype2.pc`, `expat.pc`; without them meson reported them missing). Result: cairo 1.18.6, pixman 0.46.4, fontconfig 2.18.3, libfreetype/freetype 2.14.3, libpng 1.6.59, libglib 2.90.0, libexpat/expat 2.9.0, libzlib/zlib 1.3.2, icu 79.1, pcre2 10.47, libcxx 23.1.5, pkgconf 3.0.10, libiconv, libintl, libffi, bzip2, font packages. Full rows with digests are in `N/🔣️.json` (not repeated here).
- Order of operations (`N/🟦️.ts`, `provisionNative`): download over https to a `.part` file, refuse unless SHA-256 and size equal the pin (a mismatch deletes the file and throws before anything is unpacked), then rename; micromamba then installs from a local `file://` explicit list (with the pinned md5) so it never fetches anything itself, with `--no-rc`; afterwards every Mach-O file whose signature the relocation invalidated is re-signed ad hoc with the system `codesign` (micromamba 2.10.0 left 8 binaries/dylibs with invalid signatures, e.g. `pkgconf.bin` was killed with SIGKILL on arm64). Idempotent: a marker with the manifest fingerprint makes the second run network-free. Progress (per decile) and cancellation (`AbortSignal`, SIGINT/SIGTERM in CLI mode) are supported.
- Refresh procedure: `T/python-manim-lock.py` rewrites the package rows from dry-run solves (kept as an input script).

Where it lives:

| What | macOS | Linux | Windows |
|---|---|---|---|
| micromamba binary | `<repo>/.cache/b/t/m/native/micromamba/2.10.0/darwin-arm64-c547639f5caf7422/micromamba` (the repo tools partition, git-ignored, next to `uv`) | not used | not used |
| downloads, extracted packages, prefix | `~/Library/Caches/semio/teaching-python/native/{downloads,pkgs,mamba,prefix}` | not used | not used |
| python env and interpreter | `~/Library/Caches/semio/teaching-python/env`, `teaching-python-runtimes` (already there) or `SEMIO_TEACHING_ENV` | unchanged (system cairo/pango packages) | unchanged (wheels) |

The cache root function also resolves `%LOCALAPPDATA%\semio\Cache` and `$XDG_CACHE_HOME/semio` but `hostSubdir` returns nothing for them, so on Linux and Windows `provisionNative` returns before any IO (asserted by the test below).

Catalog: `🧰️framework/🔨️modules/🏃️process/🧰️tools/🔣️.json` got a `micromamba` tool (pin version and digests read from the manifest rows, `provisioned` locator in the tools partition, `SEMIO_TOOL_MICROMAMBA`). `repo tools list` parses it and reports `micromamba 2.10.0 provisioned /Users/ueli/Documents/semio/.cache/b/t/m/native/micromamba/2.10.0/darwin-arm64-c547639f5caf7422/micromamba`. I did not run `repo tools setup micromamba`: the `provision.installer` field points to `N/🟦️.ts` (CLI mode), but I could not verify that the registry's `bootstrap-asset` setup runs an installer of that kind; the course `📜️script.ts` calls it directly instead.

Wiring: `C/📜️script.ts` calls `provisionNative()` before spawning uv and adds `PATH` (prefix `bin`), `PKG_CONFIG`, `PKG_CONFIG_PATH` and `LDFLAGS=-Wl,-rpath,<prefix>/lib` to the environment; uv then builds pycairo (`otool -l` of the built `_cairo.cpython-312-darwin.so`: rpath is the prefix `lib`, `@rpath/libcairo.2.dylib`). The first `uv sync` build of pycairo took 3 s. The build backend wheels (meson-python, meson, ninja) are resolved by uv's build isolation from PyPI and are not pinned or hash-verified by `uv.lock`; this is the remaining unpinned input (a `build-constraint-dependencies` entry would need a lock refresh, not done).

## 4. Test results (macOS arm64, real manim; the earlier numbers are from `dwg-python.md`, with a stubbed cairo)

| Run | Before | After |
|---|---|---|
| consumer `C/🎨️visuals/🧍️staffage/🧪️tests/🐍️.py` | 61 OK (stub) | 64 OK (61 + 3 new), run twice |
| guards `course_staffage`, `staffage_only`, `media_staffage` | 48 OK (stub) | 48 OK |
| clip imports (22 modules using `_manim_module.staffage`), `T/dwg-python-import-clips.py` | 22 of 22 (stub) | 22 of 22 with `SEMIO_TEACHING_LANGUAGE=en` and `de` |
| render elevation: real chapter `Beat2_PersonenPhiP` of `🔥️heating/🎬️clip/👥️internal-gains` (casts via bun exporter), `T/python-manim-render.py`, 640x360 at 15 fps | not possible | 389 frames, 25.9 s, h264 yuv420p 287278 bytes; 388 of 389 frames differ from the first; 25.6 % non-background pixels in the middle frame; PyAV oracle `S/🧪️tests/🧰️support/🐍️oracles/🎞️video.py probe` agrees (640x360, 15/1, framesRead 389) |
| render axonometric: `T/python-manim-axonometric-scene.py` (no course cast uses `axonometric`, so one scene draws the same asset as `elevation` and `axonometric` through the consumer with an in-memory manifest) | not possible | 30 frames, 9507 bytes, 29 of 30 frames change; last frame viewed: side profile left, isometric figure right |
| non-staffage visuals (`🐍️`, `integration`, `scenes`, `section_consumers`, `sections`) | 5 failures + 7 errors (stub) | real cairo alone: 1 failure + 7 errors; after the fixes below all pass (49 tests, 2 skipped Windows-only font tests) |

Fixes and causes (the 5 raster/caption failures of the stub run disappeared with real cairo; they were stub artefacts):

1. 7 errors, `ValueError: Set SEMIO_TEACHING_LANGUAGE to en or de` (the repo has no default language): `scenes` (3 tests) and `section_consumers` (4) called course code without setting a language, passing only when an earlier test leaked the variable. Fixed by a `setUp` in `C/🎨️visuals/🧪️tests/🐍️scenes.py` and a `setUpClass` in `🐍️section_consumers.py` that patch the environment from a case value; `🔣️section_consumers.json` got `"language": "de"` (`scenes` already had it).
2. 1 failure `test_authored_captions_fit_in_both_languages` (`Beat3_IsolateAirflow`, `en`): the fixture `🔣️integration.json` still held the superseded English caption of the clip `❄️cooling/🎬️clip/📐️system-sizing`; the clip and its own test (`📐️system-sizing/🧪️tests/🔣️.cases.json`) use "Divide the sensible load by air density, ...". Fixture updated to the authored text; German text and the "previous" overlong captions unchanged; test passes (it also checks the 2-line limit).
3. `semio-style` test `test_render_key_names_the_cast_keys_...` (found by the wider run, `AttributeError: 'function' object has no attribute 'cache_clear'`): the test replaced `staffage._source_digest` with a lambda while `casts_in` calls `.cache_clear()` on it. Test now patches with `mock.Mock(return_value=...)` (`C/🎨️visuals/🧪️tests/🎨️semio-style/🐍️.py`).
4. `C/📜️script.ts`: for `test` the environment now sets `SEMIO_TEACHING_LANGUAGE` explicitly (`process.env` value or `en`); without it the language-dependent modules only pass in a lucky combined order. It is a test setting, not a product default; `render` still requires `--language`.

## 5. Axonometric and formats (task 4)

- Enums: `_VIEWPOINTS` in the consumer equals `$defs/Viewpoint` of `S/🧬️schema/🔣️.json` and the cast schema `$defs/Cast/properties/viewpoint`: `elevation, front, plan, axonometric`. `ExportFormat` in the staffage schema lists `svg pdf dxf dwg png-sequence mp4 webm component glb fbx model model-zip`. The consumer has no format enum (it only calls the cast exporter and hashes the catalog asset file bytes), so a catalog `asset.exports` list with `glb fbx model model-zip` cannot be rejected; this is now tested over all 152 real catalog assets.
- Before: no consumer test used `axonometric` (the viewpoint fixture listed three). New in `C/🎨️visuals/🧍️staffage/🧪️tests/🐍️.py` and `🔣️.json`:
  - viewpoint fixture now has `axonometric` (distinct cache key per viewpoint, index accepted);
  - `Schema.test_the_viewpoints_and_export_formats_agree_with_the_staffage_schema`: consumer tuple equals both schemas; every viewpoint passes `entry_issues` and jsonschema validation;
  - `Schema.test_catalog_assets_listing_every_export_format_are_not_rejected`: every `asset.exports` is within the schema enum, `toaster` lists the four model formats, `asset_revision`, `entry_issues`, `cast_key` accept them;
  - `Axonometric.test_the_exporter_draws_the_axonometric_view_and_an_independent_parser_measures_the_bounds_it_reports`: the real bun exporter exports `elevation` and `axonometric` casts of `standing-adult`; svgelements (independent parser) measures the drawn bounds, which must equal `boundsM` of the index minus the 1 mm padding (tolerance 1 mm); the axonometric drawing is at least as tall as `heightM` (the vertical axis is true to scale), wider than the elevation, and byte-different.
- Not added to the course manifest: no clip needs an axonometric cast; adding one would need an authored scene, not a test.

## 6. Provisioner test

`N/🧪️tests/🟦️.ts` (run `repo tools exec --light bun -- N/🧪️tests/🟦️.ts`, passes): manifest invariants (https conda-forge URLs, 64-hex sha256, cairo and pkgconf and the devel packages per host), host mapping, no IO on Linux/Windows, environment variables, the digest gate with an injected fetcher (accepts matching bytes, rejects tampered bytes and removes the partial file, rejects http, keeps a verified file without network), hash equal to the independent `shasum -a 256`.

## 7. Full course suite (75 modules of `📜️script.ts test`, run one module at a time, 150 to 240 s limit, `T/python-manim-suite.sh`)

First run: 51 modules OK, 24 not. After the language and fixture fixes above 5 more modules pass (`fonts/heading`, `output`, `clip/tests`, `transforms`, `capsule`) and `semio-style` passes after fix 3. Still failing, outside the visuals module and not investigated or changed:
- `repo tools exec --light node` fails (node is `missing` in `repo tools list`; node 24.15.0 is pinned but not provisioned here): browser/presentation tests (about 8 modules: `camera`, `audience`, `geometry`, `interaction`, `presentation`, `browser`, `chapters`, `🐍️` of `clip/tests`).
- `🪟️presentation/🌐️browser/🧪️tests/🧹️staging`: `/var/folders/...` versus `/private/var/folders/...` path comparison in a temp-dir test (a macOS symlink bug in the test or code).
- `bim` (`models/footer` error, `exchange`: `Unknown object: B3`), `excel/live-snapshot` (one glyph-ghost assertion `6 <= 5`).
- 5 modules ran past the limit (`bim` clip tests, `bim presentation`, `grasshopper presentation`, `excel presentation`, `bim consistency lifecycle`); the first attempt at one combined `unittest` process ran 31 min at 100 % CPU without finishing, so the combined run cannot be used as is on this host.
- The staffage consumer `Cancellation.test_an_interrupt_mid_export_kills_the_process_tree...` failed three times while another agent's bun process was saturating the CPU (the 0.5 s interrupt timer races the exporter) and passed in every run on a quiet machine (consumer module 64 OK twice). Timing-sensitive, not changed.

## 8. Per platform

- macOS arm64: verified end to end as above. macOS x64: manifest rows and micromamba digest exist (micromamba x64 downloaded and hashed; package rows from the solver), not executed.
- Linux / devcontainer: unchanged; relies on system `libcairo2-dev pkg-config libpango1.0-dev` from `🐚️.sh`/Dockerfile for pycairo and manimpango sdists; the TS module is a no-op there; not run.
- Windows: unchanged; pycairo and manimpango wheels of `uv.lock`; the module is a no-op; not run.

## 9. Files

- New: `N/🔣️.json`, `N/🟦️.ts`, `N/🧪️tests/🟦️.ts`; in `T`: `python-manim.md`, `python-manim-env.sh`, `python-manim-lock.py`, `python-manim-render.py`, `python-manim-axonometric-scene.py`, `python-manim-probe.py`, `python-manim-suite.sh`.
- Changed: `C/📜️script.ts` (native provisioning, explicit test language); `🧰️framework/🔨️modules/🏃️process/🧰️tools/🔣️.json` (micromamba tool); `C/🎨️visuals/🧍️staffage/🧪️tests/{🐍️.py,🔣️.json}`; `C/🎨️visuals/🧪️tests/{🐍️scenes.py,🐍️section_consumers.py,🔣️section_consumers.json,🔣️integration.json,🎨️semio-style/🐍️.py}`; `T/dwg-python-import-clips.py` (adds the workspace to `sys.path`).
- Untouched: `P/pyproject.toml`, `P/uv.lock`, root `package.json`, `bun.lock`.
- Machine cache and tools partition only for binaries and environments; test output of the suite went to the course's own git-ignored `.🧬semio/🎓️teaching/⚡️cache/tests`.

## 10. Left

1. Pin the pycairo build backend (meson-python, meson, ninja) with hashes (needs `build-constraint-dependencies` plus a lock refresh).
2. Decide whether Linux should use the same prefix instead of apt (the manifest format already allows `linux-64` rows; manimpango has no Linux wheel, so pango would need pinning as well).
3. Provision node (`repo tools setup node`) to unlock the browser tests; fix the `/var` versus `/private/var` comparison; owners of `bim`/`excel` to look at their failures.
4. Verify `repo tools setup micromamba` semantics (kind `bootstrap-asset` with a TS installer) and the macOS x64 and Linux/Windows paths on those hosts.
