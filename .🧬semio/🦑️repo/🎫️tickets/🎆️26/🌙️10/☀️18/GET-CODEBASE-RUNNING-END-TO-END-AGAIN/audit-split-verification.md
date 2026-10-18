# 🔍️ Audit: monolith split verification (CENSUS5 F, G, C7, C8, C9, E)

Read-only static audit of the splits reported in `RESULT-CENSUS5-{F,G,C7,C8,C9,E}.md` (ticket `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`). No build, no cargo, no TypeScript run, no git modifying command. Nothing in the repo was edited.

## 0. Method and path prefixes

- Original = the pre-split snapshot `monolith-split/c5*-tools/orig/<tag>.rs` (the snapshot the cut was made from). Snapshot versus `git show HEAD:<path>` is reported per file (D5).
- New tree = the live root plus every file reachable from it through `#[path]`, `mod x;` and `include!` (closure). Pre-existing mounts are in both closures and cancel out. New children = closure files absent from the original closure.
- Compared line multisets (whitespace-normalised) after removing blank lines, `mod`/`use`/`pub use`/`#[path]`/`include!` wiring lines.
- "Vis-only" = a line that differs from the original only by an inserted `pub(super)`, `pub(crate)` or `pub` token (field or item visibility widened for the split).
- "Rebased" = `include_str!`/`include_bytes!` literal gained a `../` because the file moved one folder deeper; the resolved target was compared and is equal.
- Test count = lines starting with `#[...test...]` (covers `#[test]` and `#[...::async_test]`), over the closure.

Prefixes (repo-relative):
- `PL` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`
- `H` = `PL/🖥️host`
- `OD` = `🧰️framework/🛍️products/💻️os/🔨️modules`
- `LY` = `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`
- `ARCH` = `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff`

## 1. Verdict summary

- Content: lossless for all 24 Rust files. Every original line is present in the new closure, except three declared changes (D4) and the rebased `include_str!` paths. Visibility widening accounts for the rest of the `missing` count.
- Wiring: every new child is mounted exactly once (27 of 27 roots checked).
- Tests: `#[test]`-style counts equal before and after in all 24 files.
- Largest split child: 817 lines (`ej` `📖️expand-document`). No split root or child exceeds 1,000 lines.
- FAIL, rule "no `pub use` compatibility re-export": 77 glob re-exports `pub use <child>::*;` (one of them `pub(crate)`) were added to split roots, plus one `export type { ProgramDiff };` in the architect leaf (D1, D2).
- FAIL, rule "no file > 1,000 lines remains in each package": `pai` was refused and still has 1,728 lines, and five other files above 1,000 lines remain in the plugin/host/window-config packages (D3).
- WIT split (E item 13): PASS.
- Architect `🔺️diff` split (E item 14): content PASS, one compatibility re-export (D2).
- E item B (proctor source directories) is not a monolith split and was not audited.

## 2. Per-file verdict table

| # | Tag | Live file (prefix) | Before | Root | New children (max) | Vis-only lines | Content missing | Tests before/after | Wiring | Glob re-exports added | Verdict |
|---|---|---|---:|---:|---|---:|---|---|---|---:|---|
| 1 | F:su | `PL/🖥️host/🧵️shard/🧪️tests/🔬️unit/🦀️.rs` | 1656 | 28 | 9 (263) | 0 | 0 (6 rebased `include_str!`) | 50 / 50 | 9 of 9 once | 0 | PASS |
| 2 | F:dd | `PL/🗃️document/🦀️.rs` | 1856 | 87 | 6 (468) | 15 | 0 | 0 / 0 | 6 of 6 once | 5 | PASS, D1 |
| 3 | F:ir | `PL/🧠️interpreter/🏃️runtime/🦀️.rs` | 1202 | 89 | 6 (355) | 98 | 1 declared (D4) | 17 / 17 | 6 of 6 once | 4 | PASS, D1, D4 |
| 4 | F:ip | `PL/🧠️interpreter/🦀️.rs` | 3172 | 119 | 11 (438) | 176 | 1 declared (D4) | 18 / 18 | 11 of 11 once | 4 | PASS, D1, D4 |
| 5 | F:ab | `PL/🧪️tests/🔬️app-app-builder/🦀️.rs` | 1034 | 11 | 4 (325) | 0 | 0 | 62 / 62 | 4 of 4 once | 0 | PASS |
| 6 | G:he | `H/⚡️effects/🦀️.rs` | 1432 | 78 | 5 (484) | 40 | 0 | 15 / 15 | 5 of 5 once | 5 | PASS, D1 |
| 7 | G:hg | `H/🎭️guest-runtime/🦀️.rs` | 1848 (index, D5) | 94 | 7 (388) | 25 | 0 | 11 / 11 | 7 of 7 once | 6 | PASS, D1 |
| 8 | G:hr | `H/🧭️routers/🦀️.rs` | 1924 | 104 | 6 (445) | 67 | 0 | 38 / 38 | 6 of 6 once | 6 | PASS, D1 |
| 9 | G:hs | `H/🧵️shard/🦀️.rs` | 2667 | 89 | 9 (562) | 57 | 0 (1 rebased) | 0 / 0 | 9 of 9 once | 6 | PASS, D1 |
| 10 | G:hc | `H/🧪️tests/🔬️guest-cold-relay/🦀️.rs` | 1012 | 20 | 6 (217) | 0 | 0 (3 rebased) | 26 / 26 | 6 of 6 once | 0 | PASS |
| 11 | C7:sc | `LY/⚙️engine/🎬️scene/🦀️.rs` | 1428 | 76 | 8 (282) | 44 | 0 (1 rebased) | 41 / 41 | 8 of 8 once | 4 | PASS, D1 |
| 12 | C7:ej | `LY/⚙️engine/📤️export/🔣️json/🦀️.rs` | 1456 (D5) | 55 | 4 (817) | 23 | 0 | 0 / 0 | 4 of 4 once | 3 | PASS, D1 |
| 13 | C7:ex | `LY/⚙️engine/📤️export/🦀️.rs` | 2524 (D5) | 60 | 5 (640) | 150 | 0 | 41 / 41 | 5 of 5 once | 4 | PASS, D1 |
| 14 | C7:exu | `LY/⚙️engine/📤️export/🧪️tests/🔬️unit/🦀️.rs` | 1322 | 6 | 3 (471) | 0 | 0 | 41 / 41 | 3 of 3 once | 0 | PASS |
| 15 | C7:ed | `LY/🦀️.rs` | 1999 (D5) | 267 | 8 (342) | 53 | 0 | 40 / 40 | 8 of 8 once | 6 | PASS, D1 |
| 16 | C8:mfs | `PL/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs` | 1336 (D5) | 45 | 7 (268) | 0 | 0 (9 rebased) | 21 / 21 | 7 of 7 once | 0 | PASS |
| 17 | C8:pai | `PL/🧵️instance/🔌️plugin-app-impl/🦀️.rs` | 1728 | 1728 | none | n/a | n/a | n/a | unchanged (sha equals HEAD) | 0 | REFUSED, still > 1,000 (D3) |
| 18 | C8:pub | `PL/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🦀️.rs` | 1008 | 56 | 5 (404) | 2 | 0 | 0 / 0 | 5 of 5 once | 1 (`pub(crate)`) | PASS, D1 |
| 19 | C8:rc | `PL/🧵️retained-command/🦀️.rs` | 1021 (D5) | 60 | 3 (657) | 45 | 0 (1 rebased) | 12 / 12 | 3 of 3 once | 3 | PASS, D1 |
| 20 | C8:ch | `PL/🪆️child/🦀️.rs` | 1097 | 53 | 5 (315) | 5 | 0 | 0 / 0 | 5 of 5 once | 5 | PASS, D1 |
| 21 | C8:wc | `PL/🪟️window/🎚️config/🦀️.rs` | 1013 | 51 | 4 (364) | 24 | 0 | 6 / 6 | 4 of 4 once | 4 | PASS, D1; package still has D3 file |
| 22 | C9:dc | `OD/📇️directory/🔌️client/🦀️.rs` | 2404 | 108 | 8 (561) | 41 | 2 `}` (inline `native`/`browser` module bodies moved to files, declared) | 0 / 0 | 8 of 8 once | 6 | PASS, D1 |
| 23 | C9:dcu | `OD/📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs` | 1295 | 14 | 3 (463) | 0 | 0 | 37 / 37 | 3 of 3 once | 0 | PASS |
| 24 | C9:dd | `OD/🗣️dsl/✨️derive/🦀️.rs` | 1405 | 26 | 5 (386) | 23 | 0 | 1 / 1 | 5 of 5 once | 5 | PASS, D1 |
| 25 | E:wit | `PL/🧬️schema/📜️.wit` + 16 sibling `📜️*.wit` | 1662 (HEAD) | 24 | 16 files | n/a | 0 (1,569 non-blank, non-`package` lines equal) | n/a | 17 `package` lines, no duplicate `interface`/`world` name, `world actor` in root | n/a | PASS |
| 26 | E:arch | `ARCH/🟦️.ts` + `ARCH/🧾️program-diff/🟦️.ts` | 1012 (HEAD) | 872 | 1 (147) | n/a | 0 (1 extra docstring) | n/a | 1 of 1 once | 1 (`export type`) | PASS, D2 |

Notes on the table:
- "Vis-only" is the exact-match `missing` count minus the count after removing `pub(super)`/`pub(crate)`/`pub` tokens. Those lines are the widened items and fields that the split needed.
- "Content missing" is the count after removing visibility tokens and rebasing `include_str!`/`include_bytes!` literals. The listed declared cases are explained in section 3.
- Extra lines in the new tree are docstrings (`//!`, `///`), region markers (`//#region`/`//#endregion`), re-opened `impl` headers and braces, and cfg attribute duplicates. All match the declared mechanism in each RESULT file.
- Test counts: the RESULT files state su 49, mfs 20. This audit counts 50 and 21 with the wider `#[...test...]` pattern. Equal before and after in every file, so no defect.

## 3. Declared changes (verified, not defects by themselves)

- D4a `ir`: HEAD `PL/🧠️interpreter/🏃️runtime/🦀️.rs:446` had `let host = include_str!("🦀️.rs").as_bytes();`. It is now `let host = OWNED_RUNTIME_SOURCE.as_bytes();` at `PL/🧠️interpreter/🏃️runtime/📥️snapshot-operation-and-input-records/🦀️.rs:116`. `OWNED_RUNTIME_SOURCE` is a `concat!` of the root and its children (`PL/🧠️interpreter/🏃️runtime/🦀️.rs:15-23`). The engine-identity digest therefore covers all children, not only the root. The identity changes once.
- D4b `ip`: `INTERPRETER_SOURCE` at `PL/🧠️interpreter/🦀️.rs:10` is now a `concat!` of root plus 11 children, with the same one-time digest change.
- Visibility widening: `pub(super)`/`pub(crate)` added to items and fields that moved into children (for example `PL/🗃️document/🧾️store-replacement-vocabulary/🦀️.rs:34` `pub(super) struct ArtifactStoreRefusalCode`). Needed for the cross-child access. Counts per file are in the table.

## 4. Concrete defects

- D1 (rule: no `pub use` compatibility re-export). Glob re-exports `pub use <child>::*;` added to the split roots, lines as found in the live file:
  - `PL/🗃️document/🦀️.rs:56,60,64,71,75` (F:dd)
  - `PL/🧠️interpreter/🏃️runtime/🦀️.rs:56,60,64,75` (F:ir)
  - `PL/🧠️interpreter/🦀️.rs:34,38,42,54` (F:ip)
  - `H/⚡️effects/🦀️.rs:56,60,64,68,72` (G:he)
  - `H/🎭️guest-runtime/🦀️.rs:19,23,27(pub(crate)),31,35,42` (G:hg)
  - `H/🧭️routers/🦀️.rs:15,19,23,27,31,35` (G:hr)
  - `H/🧵️shard/🦀️.rs:58,62,66,70,83,87` (G:hs)
  - `LY/⚙️engine/🎬️scene/🦀️.rs:31,35,39,59` (C7:sc)
  - `LY/⚙️engine/📤️export/🔣️json/🦀️.rs:26,30,34` (C7:ej)
  - `LY/⚙️engine/📤️export/🦀️.rs:39,43,47,54` (C7:ex)
  - `LY/🦀️.rs:140,152,156,160,164,168` (C7:ed)
  - `PL/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🦀️.rs:44` (`pub(crate)`, C8:pub)
  - `PL/🧵️retained-command/🦀️.rs:40,44,50` (C8:rc)
  - `PL/🪆️child/🦀️.rs:37,41,45,49,53` (C8:ch)
  - `PL/🪟️window/🎚️config/🦀️.rs:39,43,47,51` (C8:wc)
  - `OD/📇️directory/🔌️client/🦀️.rs:38,42,46,50,54,58` (C9:dc)
  - `OD/🗣️dsl/✨️derive/🦀️.rs:10,14,18,22,26` (C9:dd)
  - Total 77. They keep the old module paths resolvable, which is the compatibility-layer shape the greenfield rule forbids. Verdict: FAIL unless the coordinator confirms a consumer needs the old path. The fix is to re-point the consumers to the child paths and delete the glob re-exports.
- D2 (same rule, architect): `ARCH/🟦️.ts:10` `export type { ProgramDiff };` re-exports the moved root interface so the importer keeps its path. The import is at `ARCH/🟦️.ts:4`. Fix: re-point the importer `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🔺️diff/🔣️json/🟦️.ts` to `🧾️program-diff/🟦️.ts` and drop line 10.
- D3 (rule: no file > 1,000 lines remains in each package). Files above 1,000 lines in the plugin tree now, none of them created by this slice:
  - `H/🔀️post-turn-relay/🦀️.rs` 2,392 (hp; not in the six results; equal to HEAD)
  - `PL/🧪️testing/🦀️.rs` 2,386 (not in the six results)
  - `PL/🧪️tests/🧪️history-edit-acceptance/🦀️.rs` 2,003 (not in the six results)
  - `PL/🧵️instance/🔌️plugin-app-impl/🦀️.rs` 1,728 (C8:pai, refused in RESULT-CENSUS5-C8; unchanged)
  - `PL/🪟️window/🎚️config/📥️retained/🦀️.rs` 1,596 (pre-existing mount of the C8:wc root, equal to HEAD; C8 did not split it, and the wc result's "largest child 364" omits it)
  - `PL/⚛️reactor/🩹️patches/🦀️.rs` 1,361 (not in the six results)
- D5 (process, not split-induced): six snapshots differ from `git HEAD`. `c5b-tools/orig/hg.rs` has 1,848 lines against HEAD 1,843 (the index holds a peer's staged edit, 7 changed lines). `c5c-tools/orig/ej.rs` (10 changed lines), `ex.rs` (32), `ed.rs` (4), `c5c8-tools/orig/mfs.rs` (2) and `rc.rs` (1, 1,021 vs 1,022 lines) differ too. The split is correct against the snapshots, so these changes are peer work carried into the cut. RESULT-CENSUS5-C7 §0 calls its snapshots "the HEAD originals", which is true only for line counts.

## 5. Not verified

- No compile, no `cargo check`, no `bun test`. The TypeScript readers and the WIT consumers (`bindgen!`) were not executed. The coordinator's compile list in each RESULT file still applies.
- The proctor `manifest-closure` change (RESULT-CENSUS5-E item B) is not a monolith split and was not audited.
- `🟦️.ts` source-reader edits in the RESULT files (`readModuleTreeSource` swaps) were not re-checked line by line; only the split files were.
