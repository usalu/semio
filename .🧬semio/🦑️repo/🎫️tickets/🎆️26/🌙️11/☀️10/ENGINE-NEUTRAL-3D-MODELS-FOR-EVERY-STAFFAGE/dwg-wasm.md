# DWG writer without the plugin framework, wasm build and verification

`S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `D` = `S/🔨️modules/📤️export/🖊️dwg`, `C` = `D/📦️packages/🦀️rust`, `T` = this ticket folder. No git command that modifies anything; no `repo run/show/dependencies/dashboard`; the four broken framework crates and `🛍️products/`, `✏️s/` were not touched.

## Result

`dwg-reopen` is **green** (5 passed, was 2 passed / 1 failed). `staffage_dwg.wasm` is built by the crate's own target, and the DWG parts of `multiview-sheet`, `axonometric-exports` and the CLI export are verified on real files.

## What the crate depended on, and what it owns now

Before: `serde`, `serde_json` and four plugin crates of `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/…/🔟ac1024` (geometry, snapshot, snapshot-objects, write: 26 600 lines pulling in the plugin framework). The crate itself only mapped the scene to that writer's `DwgDrawing`. Now: **no runtime dependency at all** (`[dependencies]` is empty); the dev-dependencies are the oracles `acadrust 0.5.5`, `ezdwg-core 0.19.2` and `serde_json`. Owned inside `C` (about 2 000 lines, one module per concern, each a `🦀️.rs`):

| Module | Owns |
|---|---|
| `🔣️json` | strict RFC 8259 reader (escapes, surrogates, depth limit) |
| `🗂️scene` | `ModelScene` validation: metres only, layer names (no `<>/\":;?*\|,=` or control characters, at most 255), ACI 1 to 255, lineweight, at least two points per polyline, positive text height, case-insensitive duplicate layers merged (a conflicting colour is refused) |
| `🧮️bits` | B, BB, BS, BL, BD, DD, RC/RS/RL/RD, H, CMC, OT, TU, MS, MC; the three parallel streams (data, string, handle) with the string-size words and the presence flag; record framing with CRC-16 |
| `🔐️checksum` | CRC-16, CRC-32, the page Adler variant, the magic sequence, the page-header mask |
| `🗜️lz77` | LZ77 encoder (literal prefix, distance-1 runs, terminator): a zero page of 29 696 bytes becomes about 130 bytes |
| `🧱️objects` | the database: 9 table controls with standard records (layer 0, style Standard, linetypes ByBlock/ByLayer/Continuous, APPID ACAD, DIMSTYLE Standard, VPORT *Active), `*Model_Space` and `*Paper_Space` with BLOCK/ENDBLK, layouts Model and Layout1, the named object dictionary with its 8 dictionaries, MLINESTYLE Standard, ACAD_PLOTSTYLENAME with its placeholder, one layer per scene layer, the entities |
| `📚️sections` | AcDb:Header (about 250 variables, insertion units 6, metric, extents from the scene), Classes, Handles (object map in chunks of at most 2 032 bytes), SummaryInfo, AppInfo, FileDepList, RevHistory, AuxHeader, ObjFreeSpace, Template (MEASUREMENT 1) |
| `📦️container` | 0x100 file header, masked file header data with CRC-32, masked 32-byte data page headers, system pages (section map, page map), second header copy |

## DWG version and entity coverage

**AC1024 (AutoCAD 2010)**, the version the TypeScript contract, the tests and the README already promised. Why it is a good target: its container (R2004 pages, LZ77, no Reed-Solomon pages as in R2007) and its objects (R2010 compact object types, string and handle streams) are completely described in the Open Design Specification for .dwg files (read from the ODA PDF, 270 pages: bit codes, section 4 container, 5.8/5.9, 9, 13 to 23, 27); every reader of the test set accepts it; maintenance release 0 avoids the extra size field of later maintenance releases. Where the specification is silent or unclear, details were cross-checked against the source of acadrust's writer and reader (a port of ACadSharp, itself verified against AutoCAD files). **Not verified:** AutoCAD, the ODA tools and LibreDWG (not available here, no unpinned downloads).

Coverage = what the TypeScript `ModelScene` can contain (`ModelLayer`: polylines and texts): **LWPOLYLINE** (open and closed, 2D points, no bulge or width; written as class-based type 503 with the class `LWPOLYLINE`, because the specification lists LWPLINE among the varying object types) and **TEXT** (single line, height, insertion point, style Standard, Unicode string stream), each on a layer with ACI colour and the nearest standard lineweight, entity colour and lineweight by layer. `ModelScene` has no hatches, solids, arcs or circles (the depiction draws the fills of the views as polylines), so none are written. The multiview sheet and the axonometric scene are plain `ModelScene`s.

## Build target and artefact

* `C/📋️project.json` (new): project `@teaching/staffage-dwg-rs`, target `wasm` (`repo:script`, `cache: false`, `class: side-effecting`, `reason: mutates-workspace`, the value 139 other targets use; the file validates against the project schema) and target `test`.
* `C/📜️script.ts` (new, no framework imports): `repo ./📜️script.ts wasm` runs `repo cargo --store=shared --configuration=<SEMIO_BUILD_MODE: release, else prototype> -- build --lib --target wasm32-unknown-unknown --message-format=json-render-diagnostics`, reads the artefact path from cargo's JSON and installs it as **`D/staffage_dwg.wasm`**, next to `D/🟦️.ts`, where `DWG_WASM_URL` looks. `repo ./📜️script.ts test` runs the crate tests through the same ingress. The artefact is git-ignored (`**/*.wasm`, rule 105) and **no artefact is checked in** (the repo checks none in: its other wasm packages are built by their `📜️script.ts wasm` into binding folders or the build store). `Cargo.lock` of the crate is ignored by the repo rule for nested locks; build output stays in the `repo cargo` store `.cache/b/r/…` (git-ignored). Sizes: 150 KB (release), 1.8 MB (prototype). The wasm output equals the native CLI output byte for byte (checked on the sheet fixture).
* Zero-touch for the tests: `ensureDwgWasm()` (`S/🧪️tests/🧰️support/🧰️.ts`) runs the target when the file is missing or older than a source of the crate, under a cross-process lock (two parallel vitest workers made two `repo cargo` runs collide: "Previous lane custody is unresolved").
* Direct cargo (documented in the README): `cargo build --release --lib --target wasm32-unknown-unknown` with the pinned nightly, then copy `target/wasm32-unknown-unknown/release/staffage_dwg.wasm` to `D/`.
* Infra finding: within one agent lane `repo cargo` refuses to alternate `--configuration=prototype` and `release` (after a prototype run the release run fails with "Previous lane custody is unresolved"); hence the script uses one configuration from `SEMIO_BUILD_MODE`. This is a limitation of the repo's lane retention, not of the crate.

## Independent readers and what each verified

| Reader | Where | Verified |
|---|---|---|
| **acadrust 0.5.5** (Rust, port of ACadSharp) | `cargo test` `dwg_reopen`, `dwg_format` | version AC1024, insertion units 6, layer colours and lineweights, LWPOLYLINE count, closed flags and coordinates to 1e-6, TEXT string, height and position, an empty scene, a 20 000-point polyline and a 3 000-character text (two string-size words), the sheet fixture; its bit reader decodes 4 000 pseudo-random bit codes (B, BB, BS, BL, BD, DD, RC, RS, RL, RD, H, CMC, OT) as written; its AC18 decompressor inverts our compression; its CRC-16/32, Adler, magic sequence and mask equal ours |
| **ezdwg-core 0.19.2** (Rust, independent parser) | `cargo test` `dwg_format` | opens the container, version AC1024, object map with one entry per written handle (fixture, sheet and a 4 000-polyline scene over several pages and handle chunks), every record parses with a valid size and CRC, header variables (INSUNITS 6, LUNITS, LUPREC, TEXTSIZE, LTSCALE), class map (500 ACDBDICTIONARYWDFLT, 503 LWPOLYLINE) |
| **ezdwg 0.19.2** (Python wheel of the same project, locked in the oracle set `dwg`) | TypeScript tests via `🧪️tests/🧰️support/🖊️.ts` | version, units, header extents, layers with colour and lineweight, layouts, every LWPOLYLINE and TEXT with its layer and colours: the model fixture, the sheet fixture, 2 000 polylines, all 4 reference assets in both projections (8 sheets), the axonometric scenes of 4 subjects, the CLI exports `--format sheet-dwg` and `--format dwg --viewpoint axonometric` |
| serde_json | `dwg_format` | the owned JSON reader accepts and rejects what serde_json does, with equal values |

Oracle set: `🧪️tests/🧰️support/🐍️oracles/pyproject.toml` has the new group `dwg = ["ezdwg==0.19.2; python_full_version >= '3.10'"]`; `uv.lock` pins it with hashes for macOS (both CPUs), Linux (x86_64, aarch64) and Windows (x64). `🐍️.ts` runs it with `--python ">=3.10"` (the other groups still run on 3.9; ezdwg needs 3.10; without a 3.10+ interpreter the dwg tests fail loudly, since the oracle policy forbids downloads). On this host the interpreter found is the 3.14 symlink in `~/.local/bin`.

## Tests, before and after

| Suite | Before | After |
|---|---|---|
| `dwg-reopen` (TypeScript) | 2 passed, 1 failed (`staffage_dwg.wasm` missing) | **5 passed** (missing module named in both languages; wasm plus ezdwg for the model scene; sheet scene and byte determinism; empty, 2 000 polylines, refusals; progress and cancellation) |
| `multiview-sheet` | 25 (DWG only with a fake encoder) | **27** (+ every sheet of the four cases in both projections as a real DWG read by ezdwg; + the real `--format sheet-dwg`) |
| `axonometric-exports` | 14 (no DWG) | **19** (+ four axonometric DWGs read by ezdwg; + the CLI `--format dwg` with progress and cancellation) |
| Rust (`repo ./📜️script.ts test`) | did not compile or run | **11 passed** (`dwg_format` 5, `dwg_reopen` 6) |

Library `tsc -p tsconfig.json --noEmit`: 0 errors. `cargo clippy --all-targets`: no warnings (native and wasm32). The three TypeScript files ran together with two workers (51 passed).

## Progress and cancellation

`writeDwg(scene, control)` checks the signal before the module is used and after it has been resolved, reports 0 of 1 and 1 of 1 with the bilingual label "Encoding DWG" / "DWG wird kodiert" (the CLI passes its control, so `runExport` reports it like the other exporters), and rejects when aborted before or during the encoding (tested, also through the CLI). `DwgUnavailableError` remains only for a genuinely missing or unloadable module; its reason is bilingual and names the build command `repo ./📜️script.ts wasm` (tested: English and German text, `missing` names `staffage_dwg.wasm`).

## README

`S/README.md`: the "unverified / pending" statements are removed (sheet paragraph; the DWG paragraph now states what the writer is, the version and why, the build target, the three readers and that AutoCAD was not available); the oracle list names ezdwg.

## Left

* The files have not been opened in AutoCAD, the ODA tools or LibreDWG. The writer follows the specification and the acadrust reference; only the three readers above verified it.
* The writer emits the minimal set of standard objects (no MLEADERSTYLE, TABLESTYLE, SCALE or visual style objects; empty material, colour and visual style dictionaries); AutoCAD recreates what it needs, which was not observed.
* `repo cargo` cannot alternate prototype and release in one agent lane (see above); the script follows `SEMIO_BUILD_MODE`.
* ezdwg needs CPython 3.10 or newer.
* Files changed: `C/Cargo.toml`, `C/🦀️.rs` and the new modules, `C/📜️script.ts`, `C/📋️project.json`, `D/🟦️.ts` (message, signal check, two duplicate docstring emoji), `S/🧪️tests/🖊️dwg-reopen/*`, `S/🧪️tests/🖊️dwg-format/*` (new), `S/🧪️tests/📑️multiview-sheet/🟦️.ts`, `S/🧪️tests/🧊️axonometric-exports/{🟦️.ts,🥒️.feature}`, `S/🧪️tests/🧰️support/{🧰️.ts,🐍️.ts,🖊️.ts}`, `S/🧪️tests/🧰️support/🐍️oracles/{pyproject.toml,uv.lock,📐️dwg.py}`, `S/README.md`. Scripts kept in `T/🗑️generated/dwg/` (`validate-project.ts`, `wasm-vs-native.ts`); the generated outputs there were deleted.
