# 🔬️ Site Size Audit for CDN Redeploy

Read-only audit of `.cache/b/p/m/vite/release/5dc1b077e86de2aa/out` (2026-11-22). Nothing in the repo or the output was modified.

## 1. Headline

- Raw output is **2.6 GB** in 3,479 files. The artifact manifest `.semio-artifact.json` lists 3,478 of them. Only the manifest itself is unlisted.
- **Plugin wasm is 2.33 GB of the 2.6 GB** (37 `.wasm` files). Gzip brings the plugin wasm to about 18 to 22 percent of its raw size (demonstrator 281 MB to 62 MB, energy 268 MB to 47 MB). A CDN that serves `Content-Encoding: gzip` or `br` transfers far less than 2.6 GB. Confirm the CDN compresses `.wasm`, `.js`, `.pbf`, `.json` and `.svg`.
- Real on-disk reductions are modest: about **23 MB** from the `mesh/` duplicate tree and about **44 MB** from content duplicates. The largest unreferenced candidate is **about 123 MB of images** that no built text file references, pending a binary-level check.

## 2. Size by top-level directory

| Directory | Size | Files | Notes |
|---|---|---|---|
| `🔌️plugin-modules` | 1.7 GB | 424 | 11 plugin `.core.wasm` (118 to 281 MB each), 1 vendor `.bin` (8.5 MB) |
| `🧩️extension-modules` | 573 MB | 757 | 37 extension `.core.wasm` (24 to 69 MB each), 63 `.js`, interfaces |
| `🖼️assets` | 170 MB | 1,380 | images 98 MB, metabolism 43 MB, fonts 20 MB, icons 2.9 MB, logos 2.6 MB |
| `assets` | 97 MB | 81 | build bundles, KaTeX fonts (59), `semio_puzzle_bg` wasm 72 MB |
| `vt` | 69 MB | 279 | vector tiles `.pbf`, zoom 0 to 10 |
| `mesh` | 22 MB | 93 | GLB, exact content twins of `🖼️assets` |
| `osm` | 8.0 MB | 198 | raster tiles `.png`, zoom 0 to 10 |
| `renderer-modules` | 3.3 MB | 10 | compiler fonts, icon pack, package assets |
| `cad-assets` | 2.6 MB | 233 | includes `cad-assets/AGENTS.md`, which should not be shipped |
| `infinite-assets` | 836 KB | 4 | includes `🗺️site.pdf` |
| `♻️mit-bestand` | 368 KB | 10 | BBSR, UDK, BMWSB, LUH, zukunft-bau logos (dark and light) |
| `.semio-artifact.json` | 272 KB | 1 | manifest |
| Root HTML and icons | about 20 KB | 9 | `index.html`, `404.html` and `🌐️.html` are identical |

## 3. Thirty largest files

| # | Size | Path (under `out/`) | Class |
|---|---|---|---|
| 1 | 281.1 MB | `🔌️plugin-modules/🎪️demonstrator/semio_hub_demonstrator_component.core.wasm` | plugin wasm (expected) |
| 2 | 267.7 MB | `🔌️plugin-modules/🔋️energy/…core.wasm` | plugin wasm |
| 3 | 162.3 MB | `🔌️plugin-modules/🌊️flow/…core.wasm` | plugin wasm |
| 4 | 158.3 MB | `🔌️plugin-modules/📐️cad/…core.wasm` | plugin wasm |
| 5 | 154.2 MB | `🔌️plugin-modules/🏭️process/…core.wasm` | plugin wasm |
| 6 | 152.5 MB | `🔌️plugin-modules/🪵️sourcing/…core.wasm` | plugin wasm |
| 7 | 149.7 MB | `🔌️plugin-modules/📖️playbook/…core.wasm` | plugin wasm |
| 8 | 142.5 MB | `🔌️plugin-modules/🧩️puzzle/…core.wasm` | plugin wasm |
| 9 | 118.2 MB | `🔌️plugin-modules/🌀️procedural/…core.wasm` | plugin wasm |
| 10 | 97.8 MB | `🔌️plugin-modules/🌍️gis/…core.wasm` | plugin wasm |
| 11 | 91.8 MB | `🔌️plugin-modules/🏗️fem/…core.wasm` | plugin wasm |
| 12 | 72.5 MB | `assets/semio_puzzle_bg-3b5yXvu-.wasm` | app wasm |
| 13 | 69.4 MB | `🧩️extension-modules/⚙️playbook-module-procedural/…core.wasm` | extension wasm |
| 14 | 32.2 MB | `🧩️extension-modules/🧊️flow-extension-brep/…core.wasm` | extension wasm |
| 15 | 27.4 MB | `🧩️extension-modules/🎨️flow-extension-draw/…core.wasm` | extension wasm |
| 16 | 26.1 MB | `🧩️extension-modules/🏘️flow-extension-bim/…core.wasm` | extension wasm |
| 17 | 26.0 MB | `🧩️extension-modules/📚️flow-extension-dictionary/…core.wasm` | extension wasm |
| 18 | 25.9 MB | `🧩️extension-modules/🧮️flow-extension-math/…core.wasm` | extension wasm |
| 19 | 25.9 MB | `🧩️extension-modules/📃️flow-extension-list/…core.wasm` | extension wasm |
| 20 | 25.9 MB | `🧩️extension-modules/🔤️flow-extension-primitive/…core.wasm` | extension wasm |
| 21 | 25.9 MB | `🧩️extension-modules/🔀️flow-extension-logic/…core.wasm` | extension wasm |
| 22 | 25.9 MB | `🧩️extension-modules/📝️flow-extension-text/…core.wasm` | extension wasm |
| 23 | 24.6 MB | `🧩️extension-modules/🏢️cad-extension-aec-building/…core.wasm` | extension wasm |
| 24 | 24.3 MB | `🧩️extension-modules/🧇️sourcing-module-slabs/…core.wasm` | extension wasm |
| 25 | 24.3 MB | `🧩️extension-modules/🪓️process-extension-wood/…core.wasm` | extension wasm |
| 26 | 24.3 MB | `🧩️extension-modules/🔩️process-extension-metal/…core.wasm` | extension wasm |
| 27 | 24.3 MB | `🧩️extension-modules/🤖️process-extension-robotic/…core.wasm` | extension wasm |
| 28 | 24.3 MB | `🧩️extension-modules/🏙️process-extension-concrete/…core.wasm` | extension wasm |
| 29 | 24.3 MB | `🧩️extension-modules/🪜️sourcing-module-beams/…core.wasm` | extension wasm |
| 30 | 24.3 MB | `🧩️extension-modules/🔥️cad-extension-aec-building-energy/…core.wasm` | extension wasm |

Next are a 14.4 MB GIF (`🖼️assets/🖼️images/🎚️parameters/🔄️Rotation-parameter.gif`) and a 12.7 MB PNG (`nakagin-capsule-tower.png`). GIFs and PNGs do not gzip, so they transfer at full size.

## 4. Map tiles

The bundle `assets/🌐️-C2odevWr.js` hard-codes `tileUrlTemplate:"/osm/{z}/{x}/{y}.png"` and `vectorTileUrlTemplate:"/vt/{z}/{x}/{y}.pbf"`. Tiles are referenced, so they are not orphaned. The files are all shipped regardless of whether the map is opened.

| Zoom | `vt` files | `vt` size | `osm` files | `osm` size |
|---|---|---|---|---|
| 0 | 1 | 84 KB | 1 | 8 KB |
| 1 | 4 | 812 KB | 1 | 8 KB |
| 2 | 16 | 14 MB | 1 | 8 KB |
| 3 | 64 | 18 MB | 1 | 12 KB |
| 4 to 6 | 3 | 1.8 MB | 3 | 0.1 MB |
| 7 | 4 | 1.6 MB | 4 | 0.2 MB |
| 8 | 12 | 3.8 MB | 12 | 0.6 MB |
| 9 | 35 | 9.2 MB | 35 | 1.6 MB |
| 10 | 140 | 20 MB | 140 | 5.4 MB |
| **Total** | **279** | **69 MB** | **198** | **8.0 MB** |

Zoom 9 and 10 hold 175 tiles and about 36 MB. The low zooms 2 and 3 hold 80 `.pbf` tiles and about 32 MB, at about 230 to 280 KB per tile.

## 5. Unreferenced candidates

**Method.** Each file's basename was searched for in all built `.js`, `.mjs`, `.html`, `.css`, `.json`, `.d.ts`, `.md`, `.txt` and `.csv` files, excluding the asset trees. The `.semio-artifact.json` manifest was excluded because it lists every file. Basename matching is coarse. A file counts as referenced if its name appears anywhere.

**Result.** 999 of the 1,380 files in `🖼️assets` (123 MB) have a basename that no text file references.

| Subtree | Unreferenced size | Files |
|---|---|---|
| `🖼️assets/🖼️images` | 97.8 MB | 109 |
| `🖼️assets/🌱️metabolism` | 21.6 MB | 231 |
| `🖼️assets/🪧️logos` | 2.3 MB | 76 |
| `🖼️assets/📃️list` | 0.9 MB | 17 |
| `🖼️assets/🔣️icons` | 0.3 MB | 503 |

The largest unreferenced files are the 14.4 MB rotation GIF, the 12.7 MB Nakagin PNG, the 9.0 MB sketchpad demo GIF, the set-connection GIFs (8.3, 6.3, 5.4 MB), the translation GIF (7.6 MB), and the dancing GIF (4.4 MB).

**Caveat.** The binary check was not completed. A search of wasm files for the top 20 names timed out. A search of `.semio` and JSON files found the 20 names only in `.semio-artifact.json`. So these files are unused by any text asset, but a wasm could still load them. Confirm before deleting, for example by checking whether the demonstrator wasm or the app's docs pages open them.

## 6. Duplicates (same size and sha256)

- 111 content groups, 1,095 redundant copies, **43.7 MB** wasted.
- **Largest group:** 50 byte-identical GLBs of 753.6 KB each, with different names (`trapezoid-capsule_z`, `placeholder`, `ellipsoid-capsule_*`, and similar). They sit in `mesh/🌱️metabolism` (24), `🖼️assets/🌱️metabolism` (24), `mesh/🥽️mesh` (1) and `🖼️assets/🥽️mesh` (1). This group alone wastes **36.9 MB**.
- **`mesh/` as a whole** (93 GLBs, 22.9 MB) has a content twin under `🖼️assets/`. The main bundle references `/mesh/` 96 times and `/🖼️assets/` 13 times, so both paths are live. Removing `mesh/` means repointing those references, which is a code change.
- **Small dups:** `🟨️.js` (8.6 KB, 32 copies), `semio-framework-*.d.ts` interfaces (about 10 KB, 20 to 23 copies each), `renderer-modules/…🔣️.json` (834 KB, 3 copies), `🌱️metabolism` `bridge.glb` and `base.glb` twins, and the `uvalue.json` attribute definition in two cad model definitions.
- **Top-level:** `index.html`, `404.html` and `🌐️.html` are identical (3,901 bytes each). `favicon.ico` and `🔖️favicon.ico` are identical, and so are `favicon.svg` and `🛡️favicon.svg`. Only the manifest's path count changes.

## 7. Reduction candidates

| # | Candidate | Estimated saving | Confidence and risk |
|---|---|---|---|
| A | Serve plugin wasm, JS, `.pbf` and JSON with gzip or brotli at the CDN | about 2.0 GB transfer (not on-disk) | High. Measured wasm ratio of 18 to 22 percent. Verify CDN config. |
| B | Run `wasm-opt -Oz` and strip debug and name sections from the 37 wasm | unknown, plausibly 10 to 30 percent of 2.33 GB | Medium. Not measured. Needs a build change and a test pass. |
| C | Drop unreferenced image files (after the binary check in section 5) | up to 123 MB | Medium. Heuristic match. Verify at runtime first. |
| D | Remove the `mesh/` duplicate tree and repoint the 96 `/mesh/` references to `🖼️assets/` | 22.9 MB | High on content. Requires a code change. |
| E | Deduplicate the 50 identical 753.6 KB GLBs with one canonical file plus aliases | about 36.9 MB, overlapping D | High on content. Requires loader change. |
| F | Remove `cad-assets/AGENTS.md` from the output | 4 KB | Hygiene. Does not affect AGENTS.md in the repo. |
| G | Keep only `woff2` for KaTeX and drop `woff` and `ttf` fallbacks | about 0.82 MB | High. Modern browsers only. |
| H | Prune vector and raster tiles for areas or zooms the demonstrator never reaches | unknown, up to about 77 MB | Low. Needs a coverage analysis of the demonstrator's map extent. |
| I | Dedupe top-level HTML and favicons | about 15 KB | Trivial. |

Realistic on-disk reduction without changing wasm is about **23 MB (D)** plus **up to 123 MB (C)**, roughly **0.15 GB** of 2.6 GB. Most wire savings come from compression (A).

## 8. Notes for the ticket

- Scratch outputs were kept in the scratchpad and in `$TMPDIR`. The `$TMPDIR` files were removed. Nothing was added to the repo or the output.
- Sizes are apparent sizes from `stat` and `du`. Gzip figures come from `gzip -6` on three files.
- The wasm plugin set has 11 plugins whose `component.js` are all referenced by name in the bundle. Extension wasm and the `🪞️vendor` font bin were not checked for runtime use.
