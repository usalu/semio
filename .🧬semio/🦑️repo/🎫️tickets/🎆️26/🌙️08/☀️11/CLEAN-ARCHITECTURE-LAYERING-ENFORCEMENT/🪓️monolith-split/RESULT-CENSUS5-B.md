# Result CENSUS5-B (prefix `c5b`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `PL` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`. Tools: `F/c5b-tools/` (descendant of `c5a-tools`). Logs: `F/🗑️generated/c5b-*` (nothing deleted). Sandboxes: `/Users/ueli/.cache/c5b/`.

## 0. Verdict (IN PROGRESS, started 2026-10-17)

- Packages 4, 5, 6 of `MONOLITH-CENSUS-5.md` section 4 (16 files, 26,736 live lines). One package at a time.
- **Package 4: DONE (static evidence)** — 6 files split, installed, verified on the live tree (section 2).
- Package 5: not started. Package 6: not started.

## 1. Found state (fresh slice)

- No earlier c5b work. Snapshots: `F/c5b-tools/orig/<tag>.rs`, hashes in `orig/SHA256`. Tag table: `F/c5b-tools/files.tsv`.
- Live line counts equal the census counts (16 files, 8,933 + 8,920 + 8,883 lines).
- Baseline census of the owning crates (lib and cfg(test) closures): `F/🗑️generated/c5b-census-own-before.jsonl`.

## 2. Package 4 (6 files, 8,933 lines before)

`PL` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`. Live line counts equal the census counts. Pre-edit snapshots `c5b-tools/orig/<tag>.rs`.

| tag | file (tail under `PL/`) | before | root + children (max child) | mechanism | crate |
| --- | --- | ---: | --- | --- | --- |
| rp | `⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs` | 1,819 | 16 + 8 fragments (344) | `include!` fragments (flat test module, 41 tests) | `semio-framework-plugin-reactor-patches` (test) |
| ba | `🏗️builder/🧩️app/🦀️.rs` | 1,572 | 139 + 6 (568) | `#[path]` modules; the 1,096-line `impl AppBuilder` was cut into 4 `impl` blocks at method boundaries | `semio-framework-plugin-app-builder` (crate root) |
| dj | `📜️declaration/👷️job/🦀️.rs` | 1,061 | 77 + 5 (284) | `#[path]` modules; the two `include!` items (`🎟️birth`, the feature-gated laws module) stay in the root | `semio-framework-plugin-decl-app` (`tool_contract`) |
| dc | `📜️declaration/🦀️.rs` | 1,218 | 68 + 5 (288) | `#[path]` modules | `semio-framework-plugin-decl-registry` (crate root) |
| da | `📜️declaration/🧩️app/🦀️.rs` | 1,170 | 159 + 3 (732: the `ArtifactApp` trait) | `#[path]` modules | `semio-framework-plugin-decl-app` (crate root) |
| ds | `📜️declaration/🪟️surface/🦀️.rs` | 2,093 | 185 + 6 (506) | `#[path]` modules (macro and its invocation live together in `🏗️surface-builders`) | `semio-framework-plugin-decl-surface` (crate root) |

Children are `<emoji>️<kebab>/🦀️.rs` next to their root; 33 new folders, all RGI (`rgi.mjs` bad=0) and resolved by the live taxonomy (`tax.ts`: 33 dirs, 0 unresolved). Specs: `c5b-tools/specs/{ba,dj,dc,da,ds}.py` (module splits) and `FRAG["rp"]` in `c5b-tools/specs5.py`.

Decisions
- `da` has an orphan `//#endregion 🔖️Limits` (no opener, already in HEAD). It stays in the root unchanged (the stager treats an unmatched end marker as a plain line); reported, not repaired.
- Moved private items, fields and inherent methods are widened to `pub(super)` only when their name occurs in another file of the scope (`widen2.py`; lists `stage/<tag>.widen.tsv`: ba 12, dj 12, dc 40, da 0, ds 4). `include!`d fixtures and the `#[path]` neighbours count as scope.
- `dj`/`da` roots were edited by a peer after my install (`semio_framework_2d::compute` to `semio_framework_2d_compute` in the `use` lines of the `da` and `ds` roots); the live roots now differ from the snapshot by exactly that line. Not mine, not touched.

Evidence (all run on the live tree after the install)
| check | result |
| --- | --- |
| losslessness | `modsplit.py lossless` for ba, dj, dc, da, ds: `missing 0`, every extra line is wiring (`#[path]`, `mod`, `use child::*`), a child docstring, `use super::*;` or a declared impl-split pair; regions orig/new 0/0 (ba, dj, dc, ds) and endregions 1/1 (da). `chk.py live rp`: spliced sequence equal to the snapshot, `gone=0 extra=0`, 41 tests to 41 |
| structure | `vmod.py live ba dj dc da ds`: every child mounted once, docstring emoji first/equal to the folder/unique/different from the root, path lengths, brace depth, rustfmt parse, include targets (2/2 ba and dj, 0 elsewhere), `#[test]` multiset, module-name clash: `VMOD OK`. `chk.py live rp`: `CHK OK` |
| definition scan | `defscan.py` (private items/fields/members used across files): 0 leftovers for ba, dj, dc, da, ds |
| glob ambiguity | `ambig.py live ba dj dc da ds`: no child item name is also exported by the root's glob sources (`ui_contract::*` 669 names, the 7 local `completion_*` modules, `artifact_inference`) |
| scoped patched differential | `oracle5.py` (original restored in a sandbox, extern crates removed, globs neutralised): error counts equal in lib and test: rp 23/201, ba 75/145, decl-app 411/424, dc 322/344, ds 476/498, warnings equal. The only message differences are the peer rename above (da, ds) and a missing-file path text in rp (an `include_str!` target that is absent in both trees). Log `c5b-oracle-pkg4.log` |
| typeck oracle | `posrun.py` family `c5b4`, labels `pre` (before the install) and `post2`: typeck canary (E0308, E0616, E0624) reached before and after; privacy 0/0 and trait-scope 0/0 in lib and test; lib resolve 3/3, type 40/40; the lib row diff is two rows that only changed the stub trait name (`ArtifactPack` to `Tr`). The test diff contains the include-fragment files (the oracle does not read `include!` files; their names are filled from the parent glob by the patched oracle only after the install, so rp and `app-merge-ui-values` rows exist only in `post2`) and the same stub trait-name renames |
| formatting debt | `fmt.py`: hunk counts original/new ba 8/7, dj 24/24, dc 14/14, da 34/33, ds 50/50; changed-line multisets identical or smaller (only removed blank lines and the peer rename) |
| census (files / bytes / active lines, lib) | rp 1/78,667/1,289 (test 11 to 19 files, 0.812), ba 2 to 8 files (0.409), dc 5 to 10 (0.682), decl-app 15 to 23 (0.755), ds 2 to 8 (0.615); every test closure inside the gate (worst 0.812) |
| source-text readers | `tsedit.py`: the flow retirement law (`✏️s/🔌️plugins/🌊️flow/.../🎮️commands/♻️retirement/🧪️tests/🟦️.ts`) read the roots of `👷️job`, `🧩️app`, `🪟️surface` as text and now reads their module trees; the composition-admission callers law reads `readModuleTreeSource(row.source)`. Probe `c5b-tools/ts-probe.ts`: both assertions hold over the trees (2 caller rows, 5 structs). `bun test`: media turn law 9 pass, tool-machine gesture law 3 pass, job birth law 1 pass, callers law fails first on `➗️equation/🦀️.rs` (peer file, before my rows), flow law cannot load (it imports a missing replication test; not mine) |
