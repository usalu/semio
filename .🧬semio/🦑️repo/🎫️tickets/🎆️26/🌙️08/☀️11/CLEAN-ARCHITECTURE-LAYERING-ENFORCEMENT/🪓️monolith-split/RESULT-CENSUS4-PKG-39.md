# Result CENSUS4-PKG-39 (prefix `c4p39`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `T` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests`. Tools: `F/c4p39-tools/`. Logs: `F/🗑️generated/c4p39-*` (nothing deleted). Stable stage and oracle sandbox root (survives a scratchpad wipe): `/Users/ueli/.cache/c4p39/` (`stage/` staged tree, `c4p39-work/` oracle sandboxes). Baseline = the working tree at 2026-10-17 08:3x (equal to the snapshots).

## 0. Verdict

**DONE** (static; real compile queued). All six files are split into 45 cohesive `include!` fragments in 45 new folders (largest child 495 lines, roots 19 to 37 lines), installed 08:41 and re-checked on the live tree: losslessness exact (sequence equal) 6 of 6, test-fn multiset identical, region pairs balanced per file, every `include_str!` target identical, rustfmt parse clean with no new formatting debt, taxonomy 45 of 45 resolved, longest path 226 UTF-8 bytes, census 0 over the gate, scoped patched differential **identical** (lib and test). No `COMPILE ADMITTED` line in `📓️FLEET.md` (checked 08:46), so the launcher check is queued (section 6).

## 1. Found state (fresh slice)

- No earlier c4p39 work: no `c4p39-tools`, no `🗑️generated/c4p39-*`, no RESULT file.
- None of the six files is under a `🧊️wgpu` target folder (they sit in `T/<law>/🦀️.rs`), so none is peer-held. Each is a module mounted with `#[path]` from one Shell test bin crate (lib root `🧰️framework/…/🎯️targets/🧊️wgpu/<bin>/🦀️.rs`, crate `semio-framework-os-renderer-wgpu-<bin>`); the module uses `use super::*;`, so the test paths are `<mod>::<test>`.
- Pre-edit snapshots (live files equal the census counts, 9,662 lines): `F/c4p39-tools/orig/<n>.rs`, hashes in `F/c4p39-tools/orig/SHA256`. File 6 had 2,037 lines instead of the census 2,032: a peer edit of 2026-10-17 00:19 that is part of the snapshot.

| n | file (`T/` + ...) | lines | `#[test]` | bin crate | SHA256 (first 16) |
| --- | --- | ---: | ---: | --- | --- |
| 1 | `🎨️wgpu-theme-editor-and-accessibility/🦀️.rs` | 1,180 | 37 | `shell-tests-chrome-4` | `27bde73af1806f05` |
| 2 | `🔗️hub-projection-workspace/🦀️.rs` | 1,411 | 21 | `shell-tests-docs-2` | `0ca97eb15cb6ab1c` |
| 3 | `🔬️wgpu-panel-anchor-model/🦀️.rs` | 1,262 | 50 | `shell-tests-windows-1` | `35ddab71ddb92e75` |
| 4 | `🔬️wgpu-shell-chrome-parity/🦀️.rs` | 1,274 | 34 | `shell-tests-chrome-3` | `97d9f7de7bc74df3` |
| 5 | `🔬️wgpu-shell-input/🦀️.rs` | 2,498 | 75 | `shell-tests-input-1` | `dbe0692a1de85233` |
| 6 | `🧪️wgpu-time-travel/🦀️.rs` | 2,037 | 41 | `shell-tests-docs-1` | `489b3760cbc3f3e9` |

- Source-text readers: none read these files. Searched with `git grep -F` per law name over the repo: the only hits are the `#[path]` mounts of the six bins and comment-only mentions in TS tests (`🧪️tests/⌨️window-scope`, `🔎️renderer-twins`, the hub collaboration test) and a Rust doc comment. The renderer laws that read source text (`shell_source()`, `repo_text`) read the Shell module tree with `🧪️tests` mounts excluded, so the new fragments are invisible to them by design; the four `repo_text`/`shell_source` call sites inside these files moved unchanged. No reader was re-pointed and no registry (source-access roots, callers corpus, lib-wiring ratchet) names these files.

## 2. What changed

Flat test modules with one `use super::*;` scope: each root is cut into `include!("<folder>/🦀️.rs");` fragments at depth-0 item boundaries (`c4p39-tools/fragsplit.py`, descendant of the PKG-28 tool; cuts in `mkspecs.py`), as PKG-15 and PKG-28 did for flat test roots. A fragment keeps the module identity of every item, so test paths, visibility and the `use` scope are unchanged; nothing is widened and no struct or impl changed module, so there is no moved-visibility surface and no compatibility re-export. The root keeps the inner docs, the `use` lines and the mounts. Cuts follow the `//#region` pairs, so every region pair stays inside one file. Each fragment starts with a `///` docstring whose first character is its folder emoji (unique among siblings and different from the root's). Relative `include_str!` literals of a fragment got one more `../` (57 literals, each resolves to the same absolute target as before).

| # | File | Before | After: root + children (max child) | Children (lines) |
| --- | --- | ---: | --- | --- |
| 1 | `🎨️wgpu-theme-editor-and-accessibility` | 1,180 | 26 + 7 (242) | `🧰️theme-editor-fixtures` 91, `🎚️theme-publication-and-sections` 225, `🔁️theme-tokens-and-leaves` 151, `♿️accessible-names-and-activation` 218, `🪞️mirror-address-liveness` 73, `🧭️settings-strip-and-keybindings` 242, `📐️passive-regions-and-geometry` 175 |
| 2 | `🔗️hub-projection-workspace` | 1,411 | 19 + 8 (260) | `🔗️projection-fixtures-and-live-workspace` 206, `🪢️severable-relay-and-ledger` 130, `📣️replay-refusals-and-agent-offer` 121, `⏱️frame-pumps-and-edit-ledger` 112, `🤝️two-shell-collaboration` 177, `🧭️native-guest-journey` 192, `🏗️creation-door-and-cross-shell-meeting` 218, `🖱️cross-shell-collaboration-and-sign-in-task` 260 |
| 3 | `🔬️wgpu-panel-anchor-model` | 1,262 | 20 + 7 (229) | `🪟️spawned-documents-and-host-panel` 159, `📌️anchor-ids-resize-and-default-dock` 161, `🧱️anchor-geometry-and-state` 202, `🗄️dock-override-round-trip` 162, `⚙️settings-keybindings-hug-and-mobile` 208, `💬️command-form-and-agent-chat` 229, `🌳️tree-units-and-chrome-bands` 142 |
| 4 | `🔬️wgpu-shell-chrome-parity` | 1,274 | 26 + 6 (265) | `📚️fixtures-and-parity-builders` 115, `🎛️example-picker-and-role-groups` 198, `🛑️chords-controls-and-compute-status` 265, `🗣️chrome-i18n-and-switch-order` 163, `🪟️window-scope-laws` 262, `🛟️safe-area-and-incoming-layout` 263 |
| 5 | `🔬️wgpu-shell-input` | 2,498 | 20 + 9 (495) | `🏷️scene-labels-dock-tabs-and-select-wheel` 135, `🌍️retained-world-capture` 283, `⌨️retained-pane-focus-and-keyboard` 296, `🪟️window-publication-journals` 241, `🧭️dock-order-key-events-and-drag` 293, `🖥️display-and-table-transfer` 495, `🎯️canvas-pointer-wheel-and-catalogue-drag` 327, `🔎️quick-search-palette-keyboard` 71, `📷️canvas-camera-ownership` 364 |
| 6 | `🧪️wgpu-time-travel` | 2,037 | 37 + 8 (360) | `🧰️harness-and-recorders` 153, `⏪️band-law-and-chords` 201, `📐️band-layout-and-focus` 287, `🪟️reveal-indicator-and-refusals` 286, `👥️notices-peer-edits-and-finalize` 212, `🎛️staged-editors` 304, `📚️history-body` 360, `📡️reprojection-blur-import-and-context-menu` 221 |

Totals: 9,662 lines in 6 files became 148 lines in 6 roots plus 45 fragments (largest 495). Nothing outside the six files and the 45 new folders was edited (the bin crates mount the same six paths as before). All fragment folders are siblings of the root file inside its law folder, e.g. `T/🧪️wgpu-time-travel/📚️history-body/🦀️.rs`.

## 3. Evidence (all commands run in this session, on the live tree)

| Check | Command (tools in `F/c4p39-tools/`) | Result |
| --- | --- | --- |
| Install precondition | `python3 install.py check` then `apply` | `children 45, roots 6, problems 0` (live files equal the snapshots), `installed 51 files`; installed bytes equal the staged bytes (51 compared, 0 differ) |
| Exact losslessness | `python3 lossless.py` (`c4p39-lossless-live.log`) | `LOSSLESS OK`: per file `gone=0 extra=0 sequence-equal=True`. Splicing every fragment back at its `include!` line (minus its first docstring line, with the `../` rebase reversed) reproduces the snapshot line for line |
| Structure | `python3 verify.py live` (`c4p39-verify-live.log`) | `VERIFY OK live`: `#[test]` fn-name multiset identical per file (37, 21, 50, 34, 75, 41); region markers balanced in every file (each fragment holds whole `//#region`/`//#endregion` pairs); docstring emoji first and unique among siblings; every child mounted exactly once by its root; every folder emoji followed by U+FE0F; brace depth 0 at the end of each file; rustfmt parse of 51 files clean. The 57 removed lines in the multiset are exactly the 57 rebased `include_str!` lines, each with its twin among the added lines |
| Include targets | `python3 incl.py` (`c4p39-incl-live.log`) | `INCLUDES OK`: the multiset of absolute `include_str!` targets equals the original per file (10, 2, 5, 10, 24, 6), 0 missing |
| Formatting debt | `rustfmt +nightly-2026-07-20 --check --config skip_children=true` per file | hunks original = new in every file (19, 5, 13, 3, 33, 35): no new formatting debt |
| RGI and taxonomy | `bun rgi.mjs newnames.txt`; `bun tax.ts newdirs.txt` | `bad=0` for 45 names; `dirs 45, unresolved []` through the live `semanticDirectoryKindId` |
| Path length | in `verify.py` and an inline scan | longest repo-relative path 226 UTF-8 bytes (limit 240), longest absolute 191 chars (limit 256) |
| Census (canonical `readRustSourceClosureV1` test-closure census, `census.sh pre` and `post`, `c4p39-census-{pre,post}-crates.tsv`) | `zsh c4p39-tools/census.sh post`; `python3 crows.py` | 0 of 3,354 unit rows over the gate before and after; lib-test unit (files / bytes / lines, ratio of the worst limit), native: chrome-3 11/176,318/1,309 to 17/177,637/1,333 (0.673 to 0.678), chrome-4 11/196,706/2,013 to 18/198,333/2,041 (0.750 to 0.757), docs-1 8/183,700/2,100 to 16/185,479/2,132 (0.701 to 0.708), docs-2 16/226,336/2,776 to 24/228,104/2,808 (0.863 to 0.870), input-1 14/236,823/2,558 to 23/238,944/2,594 (0.903 to 0.911), windows-1 13/198,770/2,236 to 20/200,249/2,264 (0.758 to 0.764). Bytes bind everywhere; the file count stays at most 24 of 32. wasm32-unknown-unknown and wasm32-wasip2 rows equal native for chrome-3, chrome-4 and input-1; docs-1 (1 file), docs-2 (21) and windows-1 (16) are smaller on wasm because their laws are not compiled there (same before and after) |
| Scoped patched differential | `PYTHONHASHSEED=0 ulimit -n 65536; POS_PREFIX=c4p39 python3 posrun.py run families/c4p39.json --cfgs lib,test --label {pre,pre2,post} --no-base` (pos oracle with the stub layer for everything outside the family; family = the six bin crates; sandbox moved to `/Users/ueli/.cache/c4p39/c4p39-work`); `python3 posdiff.py c4p39-pre2 c4p39-post lib test` | lib: 0 rows on both sides; test: 7,673 rows on both sides, **`POSDIFF IDENTICAL`** (keyed by cfg, code, normalised message, label and crate, paths and lines dropped because the split moves code between files). The typeck canary (E0308, E0616, E0624) is reached in every run, so typeck ran. 5,890 of the 7,673 rows lie in the six laws' files and now in their fragments (e.g. 1,959 in the input-1 law), so the fragments were compiled by the oracle. Absolute classification: privacy, trait-scope and other 0 non-stub; resolve 17 and type 184 non-stub rows exist identically before and after (stub-layer noise, section 6). Determinism: `pre` against `pre2` (two baseline runs) is identical too (`c4p39-pos-pre.log`, `-pre2.log`, `-post.log`, `c4p39-posdiff-post.log`) |
| Definition scan and E0451 literal scan | not applicable | no definition changed module (one `use super::*;` scope per law; `include!` splices the items into the same module), so no private-item leftover and no struct literal can cross a module boundary |
| Duplicate mounts | `git grep -F` per law folder (section 1) and `verify.py` | each root is mounted by exactly one `#[path]` line of one bin (`shell-tests-*`); each root carries each of its fragments exactly once (`include!`), and no other file names a fragment folder |

## 4. Consumers

None edited. The six `#[path]` mounts in the bin lib roots (`🌅️shell-tests-chrome-3`, `🌴️shell-tests-chrome-4`, `🥧️shell-tests-docs-1`, `🥪️shell-tests-docs-2`, `🍎️shell-tests-input-1`, `🏙️shell-tests-windows-1`) still name the same six paths. Registries that list source files by path (taxonomy, build-owners, launch routes, lib-wiring ratchet, source-access roots) hold no entry for these law files.

## 5. Decisions

- `include!` fragments instead of `#[path]` child modules: the files are flat test modules whose 258 test names (`<mod>::<test>`) and exact-law selectors must stay identical, and a child module would add a path segment and require widening dozens of private fixtures. This is the PKG-15/PKG-28 mechanism for flat test roots.
- Docstring marker `///` on the fragment head (it attaches to the first item as a real docstring; `//!` is illegal inside `include!`).
- Cuts only at region boundaries or between items, never inside a `//#region` pair: no region needed re-opening.

## 6. Limits, queue and foreign errors

- No `bun repo cargo` call: `📓️FLEET.md` has no `COMPILE ADMITTED` line (checked 08:46) and the compile mutex `🗑️generated/cargo-mutex.lock` was held by `cad` (08:42). Command for the compile queue (one batched call under the mutex of `📓️WAVE5-BRIEF.md`):
  `cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-renderer-wgpu-shell-tests-chrome-3 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-4 -p semio-framework-os-renderer-wgpu-shell-tests-docs-1 -p semio-framework-os-renderer-wgpu-shell-tests-docs-2 -p semio-framework-os-renderer-wgpu-shell-tests-input-1 -p semio-framework-os-renderer-wgpu-shell-tests-windows-1 --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml`
- The static proof covers name resolution, privacy and (through the canary) type errors on the non-stub side; it does not cover coherence or trait selection against the real externals.
- Foreign errors: none attributable to the split. The 17 resolve and 184 type non-stub test rows are identical before and after (the same rows in `pre`, `pre2` and `post`) and come from the stub layer of the oracle (non-stub-classified rows that the stubbed externals cause).
- Infrastructure notes: no ENFILE/EMFILE and no Bun segfault occurred (`ulimit -n 65536` and `nice -n 10` on all oracle and census runs). The first oracle launch was moved to the background by the shell tool timeout and finished normally; the baseline was therefore repeated as `pre2` (identical to `pre`) and used for the comparison. `git` printed `daemon terminated` during several read-only calls (fsmonitor noise, unrelated).

## 7. Files

- Edited (6 roots): the files in the table of section 2.
- Created (45 fragments, one `🦀️.rs` per new folder): the folders in the table of section 2, next to their root.
- Tools, kept: `F/c4p39-tools/` (`fragsplit.py`, `mkspecs.py`, `specs/`, `install.py`, `verify.py`, `lossless.py`, `incl.py`, `posrun.py`, `posdiff.py`, `census.ts`, `census.sh`, `crows.py`, `tax.ts`, `rgi.mjs`, `outline.py`, `ctx.py`, `families/c4p39.json`, `orig/` snapshots with `SHA256`, `installed.json`).
- Logs, kept in `F/🗑️generated/`: `c4p39-verify-{stage,live}.log`, `c4p39-lossless-live.log`, `c4p39-incl-live.log`, `c4p39-pos-{pre,pre2,post}.log` with the `c4p39-c4p39-*` diagnostic dumps, `c4p39-posdiff-post.log`, `c4p39-census-{pre,post}.log` and the `c4p39-census-*` tables.
