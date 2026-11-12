# Zero-touch oracles of the staffage test suite

Result: all 24 files that use an external oracle run on this clean machine without Homebrew, apt, sudo or hand-downloaded binaries. Core four files: before 0 tests ran (4 files failed to load; 21 tests declared), after 22 of 22 pass (one test added). The 20 python-oracle files: 675 passed / 7 failed before and after, the same 7 (product state, not tooling); see section 5.

## 1. Inventory (who called what, for what)

| Test file | Program (before) | Purpose |
|---|---|---|
| paper-dimensions | `rsvg-convert -i shape-figure -x/-y` | rasterise one SVG element at 10 px/mm and compare its pixel height |
| zip-reopen | `unzip -t`, python zipfile, and the product CLI calling `rsvg-convert` | ZIP integrity; frame rasterisation for the PNG sequence |
| pdf-reopen | `qpdf --check`, `pdfinfo -box`, `pdftoppm -r 254`, `pdftotext`, reportlab | structure, metadata, MediaBox, raster measurements, text extraction, AFM metrics |
| frame-timing | `ffprobe` (-show_streams/-packets/-count_frames), `ffmpeg` (decode, null encode), product CLI calling `rsvg-convert` and `ffmpeg` | probe of MP4/WebM, alpha decode, export |
| 20 files via `python()` | `uv run --with <unpinned names>` | shapely, numpy, scipy, svgelements, pymupdf, lxml, ezdxf, reportlab oracles |

Not in scope: the physics oracles (`☁️condensation`, `🏗️cantilever`, `🍃️penman-monteith`, ...) use CoolProp, anastruct, pyet etc. only to write fixtures offline; no test runs them.

## 2. Choices and reasons

- One owned lock: `S/🧪️tests/🧰️support/🐍️oracles/{pyproject.toml,uv.lock}`. `uv.lock` pins every wheel with sha256 for macOS arm64/x64, Linux x64/arm64 and Windows x64 (`required-environments`, `no-build = true`, `exclude-newer`). Hash enforcement verified by tampering a hash in a copy: `uv run --locked` aborted with "Computed ... sha256" mismatch. (`uv run --with-requirements` did NOT verify hashes, so it is not used.)
- Three dependency groups, selected per call (`python(script, set, args)`): `geometry` (shapely, numpy, scipy, svgelements), `documents` (pymupdf 1.26.5, pikepdf, pypdfium2, pillow, lxml, ezdxf, reportlab<5), `media` (av, imageio-ffmpeg). `null` runs the standard library only (zipfile).
- Replacements: qpdf -> pikepdf (bundles libqpdf); pdfinfo -> pikepdf; pdftoppm and pdftotext -> pypdfium2 (PDFium, a second independent engine); rsvg-convert -> PyMuPDF/MuPDF (resvg-py has no Python 3.9 wheel and would need Rust, rejected); ffprobe -> PyAV; ffmpeg -> the static build in the imageio-ffmpeg wheel (libx264 and libvpx-vp9 verified); unzip -> python zipfile.
- Interface (all in `S/🧪️tests/🧰️support/🐍️.ts`): `python`, `rasteriseSvg`, `inspectPdf`, `pdfText`, `renderPdf`, `probeVideo`, `ffmpegExecutable`, `ffmpegBytes`, `oracleToolPort`. Oracle scripts: `🖼️raster.py`, `📕️pdf.py`, `🎞️video.py` next to the lock.
- ffmpeg for the product: the helper publishes the pinned executable as `SEMIO_TOOL_FFMPEG`, the registry's first locator, so the product's own `repoToolPort` (`repo tools exec ffmpeg`) runs unchanged against it. `rsvg-convert` has no pinned source, so `oracleToolPort` answers it with the MuPDF rasteriser given the same arguments (product call line itself is not run against librsvg).

## 3. Environments (nothing inside the worktree)

- Python environments: machine cache of the repo, one per group: macOS `~/Library/Caches/semio/staffage-oracles/<set>`, Windows `%LOCALAPPDATA%\semio\Cache\staffage-oracles\<set>`, Linux `$XDG_CACHE_HOME/semio/staffage-oracles/<set>` or `~/.cache/semio/staffage-oracles/<set>` (set through `UV_PROJECT_ENVIRONMENT`). uv's wheel cache is uv's own default (`~/.cache/uv` / `%LOCALAPPDATA%\uv`). Only macOS was exercised; the Linux/Windows paths are untested code.
- I had first created three venvs inside the ticket folder (329 MB); they were deleted on the coordinator's order. `T/🗑️generated/oracle-tools/` holds only small reports.
- uv is resolved by the helper from the registry's `provisioned` locator without any graph read: `SEMIO_TOOL_UV`, then `<repo>/.cache/b/t/m/native/uv/<version>/<host>-<digest>/uv`, then `~/.local/bin`; `repo tools exec` remains the fallback. `UV_PYTHON_DOWNLOADS=never` (the catalog policy) is set for direct calls.

## 4. Files changed

- `🧰️framework/🔨️modules/🏃️process/🧰️tools/🔣️.json`: uv capacity rules (`uv python*`, `uv run*`, `uv pip*` as `tool-query`, 512 MB; default stays `dependencies`, 6144 MB). Reason: each oracle call was admitted as a 6144 MB job and sat 15+ minutes in the queue. ffmpeg entry unchanged.
- New: `S/🧪️tests/🧰️support/🐍️.ts`, `🐍️oracles/{pyproject.toml,uv.lock,🖼️raster.py,📕️pdf.py,🎞️video.py}`.
- `S/🧪️tests/🧰️support/🧰️.ts`: uv resolution, `tool(..., environment)`, `requireTool("uv")` only; removed `python`, `toolBytes`, the rsvg/qpdf/ffprobe probes.
- Tests: `📕️pdf-reopen`, `📐️paper-dimensions` (+`.feature`), `🗜️zip-reopen` (+`.feature`), `🎞️frame-timing` (+`.feature`), and the 20 python-oracle tests (import of `python` from `🐍️.ts`, set name instead of package list; `ORACLE_PACKAGES` constant removed).
- Root `package.json` and `bun.lock`: NOT touched.
- Scripts kept in T: `oracle-tools-switch.py`, `oracle-tools-svg.ts`, `oracle-tools-pdf.ts`.

## 5. Counts (direct vitest, `--maxWorkers=2`; `repo tools exec vitest` queued for minutes at the capacity gate)

- Core four, before: 4 files failed to load (module-level "required tool not available"), 0 of 21 declared tests ran. After: paper-dimensions 9/9, pdf-reopen 5/5, zip-reopen 2/2, frame-timing 6/6 (includes the new missing-encoder test) = 22/22.
- 20 oracle files, before: 682 tests, 675 passed, 7 failed. After (same files plus the core four): 704 tests, 697 passed, 7 failed. The 7 are identical before and after and are product/fixture state of other agents, not tooling: plan-projection (2), export-performance digest (1), multiview-sheet ISO 5456-2 symbol (1), three-views (3).

## 6. Restated assertions

- pdf-reopen: `qpdf --check` text -> `problems` empty (pikepdf `check_pdf_syntax` plus decoding every stream, the two things qpdf checks); `pdfinfo` regexes -> exact fields: title equals, subject regexes, version "1.7", pages 2; MediaBox now compared as exact floats (plus origin 0,0). The old regex expected "Line drawing, dark / Strichzeichnung, dunkel", which the product no longer writes; updated to the current "Dark line / Dunkle Strichzeichnung". Raster and text facts unchanged but produced by PDFium instead of poppler (pixels 800x500 +-1, figure 17.8 to 18.2 mm, colours, hatch variety, text containment all hold).
- paper-dimensions: librsvg `-i shape-figure` PNG height -> MuPDF render of that element alone, height of pixels at least 50% covered, within the same 1 px tolerance.
- zip-reopen: `unzip -t` dropped; `testzip()` (CRC of every member), stored entries, names, PNG signatures and manifest asserted as before.
- frame-timing: ffprobe values -> PyAV: codec, pix_fmt, `r_frame_rate` (PyAV `base_rate`, falling back to guessed rate), decoded frame count (replaces `nb_read_frames`), container duration, packet pts spacing, `alpha_mode` tag; ffmpeg decode and null-encode run the pinned ffmpeg. Added: named "Tool ffmpeg not found" propagates out of `runExport`.
- python oracles: same facts, pinned versions.

## 7. Left for the human dev

1. ffmpeg stays `pin.state: system` in the catalog. A pinned per-platform source with digests exists (the imageio-ffmpeg wheels in `uv.lock`), but the catalog has no provision kind that unpacks a PyPI wheel; it needs installer code in `🛍️products/🦑️repo` (out of my scope). Until then the product export needs system ffmpeg; tests are independent of it. The named error ("Tool ffmpeg not found ... brew install ffmpeg") comes from the registry and is covered by the new test at port level.
2. Product code still calls system `rsvg-convert` (`🖨️preview`, `🎬️cli`); no pinned librsvg source exists. Candidate: swap the product Rasterizer to a pinned implementation.
3. Defect to note: `repo tools exec` (even `--light uv`/`vitest`) waits at the capacity/graph gate; a provisioned tool should resolve without it. The helper now bypasses it for uv.
4. Windows and Linux paths (cache dir, `uv.exe`) were not run; wheel availability for all five platforms is guaranteed by the lock. On Python 3.9 reportlab is capped below 5 (5.x needs 3.10).
