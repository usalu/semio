# Result: puzzle 5d and 2d editor split

Executed 2026-10-15 by the Sonnet executor. Seam map: `SEAM-MAP-PLUGIN-RUST-TIER2.md` sections A and B. Pure structural split: no behaviour change, no public renames.
`E5` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`, `E2` = the same path under `◻️2d`.

## 0. Status

Both editor files are split and installed. No compile was possible (compiler-ingress gate), so verification is the full-resolution rustc differential plus static checks (section 7). Everything marked "not verified" in section 8 still needs a real `bun repo cargo ... check` once the gate admits the crates.

## 1. Line counts

Active = non-blank, non-`//` lines.

| File | Before | After (total) | Active |
| --- | --- | --- | --- |
| `E5/🦀️.rs` (parent) | 9,141 | 1,703 | 1,388 |
| `E5/🧩️entities` | | 699 | 539 |
| `E5/🧭️view` | | 212 | 135 |
| `E5/🗂️catalog` | | 293 | 248 |
| `E5/🕹️interaction` | | 690 | 498 |
| `E5/🖌️session` | | 1,455 | 1,356 |
| `E5/🧵️retained` | | 224 | 181 |
| `E5/👆️gestures` | | 1,928 | 1,694 |
| `E5/🧰️kit` | | 1,991 | 1,722 |
| 5d total | 9,141 / 7,729 active | 9,195 | 7,761 |
| `E2/🦀️.rs` (parent) | 4,059 | 1,357 | 1,082 |
| `E2/🕹️interaction` | | 264 | 173 |
| `E2/🧰️kit` | | 327 | 268 |
| `E2/🧵️retained` | | 1,259 | 1,075 |
| `E2/🖌️session` | | 671 | 589 |
| 2d total | 4,059 / 3,171 active | 3,878 | 3,187 |

Largest child: 1,722 active lines (limit 2,500). Longest new path: 157 UTF-8 bytes repo-relative (180-185 absolute); limit 240. Each child starts with a `//! <folder emoji>` docstring that links the parent with `[`super`]`, then `use super::*;`; no comments were added inside definitions.

## 2. Created and edited files

Created (each `🦀️.rs`): `E5/{🧩️entities,🧭️view,🗂️catalog,🕹️interaction,🖌️session,🧵️retained,👆️gestures,🧰️kit}`, `E2/{🕹️interaction,🧰️kit,🧵️retained,🖌️session}`.
Parents wire children with `#[path = "<folder>/🦀️.rs"] mod <snake>;` and a glob re-export: `pub use` for `entities view catalog interaction` (5d) and `interaction` (2d), plain `use` for the children that export nothing above `pub(super)` (picked by the highest visibility a child holds, so no "glob doesn't reexport" warning).

Edited: the two parents; the two `🧪️tests/🔬️unit` and two `🧪️tests/🧵️retained-wiring` test files; `🔣️taxonomy.json`; `📖️source-access/🟦️.ts`; root `📜️script.ts`; `🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts`; fixtures `⏳️completion/🔣️.json` and `🔏️publication-authority/🔣️.json` (details in 5).

## 3. Deviations from the map (forced by gate evidence)

1. **The retained trio stays in the parent** (5d: `PUZZLE5D_RETAINED_TOOL_IDS`, `Puzzle5dRetainedCommandJobFactory`, ToolProofs; 2d: tool-id tables, `Puzzle2dRetainedCommandJobFactory`, ToolProofs). The tool-job proof gate (`📜️script.ts` `toolJobProofCatalogFailures`) requires `proof.sourceFile === proof.ownerFile`, the `impl ArtifactEditor` in that same file and a local `factory_type` struct. Moving them would break that law. Consequence: the two `owner_file` literals per crate are **unchanged** and still name the file that holds them; `🧵️retained` holds the window/export work and retained plumbing instead (5d 181, 2d 1,075 active).
2. 5d `Puzzle5dCommand` region went to `🕹️interaction` as the map says; this is safe only because of the gate change in 5 (the census reads a split root as one text).
3. 2d `ToolProofs` stay in the parent (not in `session`), for the reason in 1. 2d `RetainedCommands` endregion marker was re-emitted in the parent right after the factory; a `//#region 📜️ToolProofs` start marker and a `//#endregion 🧵️ReservedJobs` were added to balance markers that were already unbalanced.
4. Empty region shells deleted: 5d `SelectionFlags` (2 marker lines); 2d lines 106-317 (`Constants Scene BoardReads TargetRegions Clipboard BoardHostSync UiScopes Puzzle2dCommand`, 16 marker lines plus blanks). Verified by script that 106-308 hold nothing but `//#` markers and blanks; the 2d `Puzzle2dCommand` shell (310-316) is empty too (the map said about 6 active lines). A repo grep found no gate or doc reading those region names (only ticket history and unrelated files).
5. 2d: three `use` lines that opened the `ReservedJobs` region (old lines 2492-2494) are used by `retained` code too, so they were hoisted verbatim into the parent's `use` block.
6. Test mounts (`unit`, `agent-lane`, `target-volumes`, `retained-wiring`, `host-admission`, `gesture-kinds`) stay mounted in the parent, so no relative `#[path]` was rewritten (all `#[path]` mounts moved into children: none).
7. Sibling `🧵️` clash: `session`, `retained`, `gestures` all proposed `🧵️`. Chosen: `🧵️retained` (forced by kind `retained`, parent `editor`), `🖌️session` (already in `members-of-editor`, precedent `💠️lowpoly/.../✏️editor/🖌️session`), `👆️gestures` (new).

## 4. Taxonomy changes

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: one entry appended to `semanticDirectoryMemberKinds."members-of-editor".memberNames`: `"👆️gestures"` (RGI emoji + U+FE0F, precedent `👆️gesture` elsewhere; existing `🧵️gestures` kept, bim uses it). Admitted without change: `🧩️entities 🧭️view 🗂️catalog 🕹️interaction 🖌️session 🧰️kit` (members-of-editor) and `🧵️retained` (kind `retained`, parents schema/editor). All eight emojis match `/^\p{RGI_Emoji}$/v` (checked with bun) and are unique among siblings in each editor dir.
`bun ./📜️script.ts verify taxonomy report --scope <E5>` and `<E2>`: before 16 / 27 errors, after 16 / 27, the sorted error sets are identical (all pre-existing `directory-kind-unresolved` etc., none on a new child). Inventory grew by 8 dirs / 8 files (5d) and 4 / 4 (2d), so the new children were scanned. I ran no scope without the `👆️gestures` registration, so I cannot show it is needed.

## 5. Re-pointed readers (the map missed most of them)

- Rust unit tests: `E5/🧪️tests/🔬️unit/🦀️.rs` (12 `include_str!("../../🦀️.rs")` reads) and `E2/🧪️tests/🔬️unit/🦀️.rs` (1 read) now call a new `pub(crate) fn editor_source() -> String` (docstring `📚️`) that concatenates every child first and the root last (assertions unchanged; `let source = source.as_str();` keeps `&str` semantics, so the `UnitTests` prefix split still closes production). Both `🧪️tests/🧵️retained-wiring/🦀️.rs` read `super::unit_tests::editor_source()`.
- `⏳️completion/🔣️.json` (`🧰️framework/.../🔌️plugin/🧪️tests/🧫️fixtures`): the four `puzzle5d-copy/cut/paste/import` `reservedCallers` `source` now name `E5/🖌️session/🦀️.rs`; `sourceStart` of paste and import became `"\npub(super) struct Puzzle5dPasteJob {"` / `...ImportJob {` because the widening prefix sits before `struct`.
- `🔏️publication-authority/🔣️.json`: `source` still names the root; both puzzle owners gained `"mounted": [...]` (the split children). `📦️packages/🟦️typescript/📜️script.ts` reads `mounted` children first, then the root (`ownerSource`), type field `mounted?: string[]`. The 3d owner has no `mounted`, so its text is unchanged.
- Root `📜️script.ts`: new `toolJobPluginProductionSources(root)` (a declared split root stands for its module tree and its mounted children are no census entries), used by `toolJobCoverageRun` and `toolJobFactoryProofActivationScan`; `puzzle2d` is now read with `policyReadModuleTreeSource`. `puzzle5dEvidence` already goes through `policyReadRustPolicySource`.
- `📖️source-access/🟦️.ts`: both editor roots appended to `POLICY_SPLIT_SOURCE_ROOTS`.
- Not changed, and why: hub `🔮️ownership/🔣️.json` (stale: 10 of 11 tests fail at baseline with ENOENT on fixture paths and recorded editor hashes already mismatch); `.vscode/launch.json` syntax-check argument (the root file still exists); the `🗀️artifacts` typo path at root `📜️script.ts` line about 6493 (a pre-existing label, not a read).

## 6. Visibility widenings

Static, over-approximate by design. Rule: a private definition in a child whose identifier appears (lexer id token) in any other file of the component scope (parent, other children, the 7 / 13 descendant mounts: tests, snapshot, recorder) becomes `pub(super)`; trait impl members and existing `pub`/`pub(crate)` items untouched. 5d: 361 (58 items, 244 fields, 59 inherent members) = entities 19, interaction 10, session 92, retained 19, gestures 120, kit 101. 2d: 91 (19 items, 58 fields, 14 members) = interaction 1, kit 10, retained 35, session 45. A re-scan of the final texts finds 0 further promotions. Private items of the parent needed no change (children are descendants). No qualified paths were added.
Consequence worth knowing: line-anchored source readers must now allow the `pub(super) ` prefix (the completion fixture was the one that broke).

## 7. Verification actually run

| Check | Result |
| --- | --- |
| Full-resolution differential, `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null --error-format=short`, from the real lib roots `🖐️5d/🦀️.rs` and `◻️2d/🦀️.rs`, split tree vs a symlink sandbox holding the original editor file; error multisets compared with all editor files folded into one label | 5d native 3157 = 3157, `--test` 6884 = 6884, `--target wasm32-wasip2` 3157 = 3157; 2d native 1359 = 1359, `--test` 1812 = 1812, wasip2 1359 = 1359; only-orig / only-split 0 in all six. First on sandboxes (without and, for `--test`, with the test-file edits), then again on the installed real tree. The only raw diff was one error whose message embeds the sandbox path (pre-existing unreadable fixture include); normalised. |
| Oracle sensitivity | Removing `pub(super)` from one fn and one struct produced +47 errors (E0425 / E0433), so a missing item widening is caught. Field and method privacy are NOT covered (typeck is cut short by the missing external crates). |
| Losslessness (non-blank line multiset, widening prefix removed) | 0 original lines lost except the deleted shells (5d 2 markers, 2d 16 markers); extras are only docstrings, `use super::*;`, mod wiring, and the 2 added 2d markers (ToolProofs start, ReservedJobs end). Installed files equal the generated files byte for byte. |
| `rustfmt --edition 2021 --emit stdout` parse | 0 failures on all new files, both parents, the four test files. |
| Path resolution | 15 (5d) and 16 (2d) `#[path]` attributes resolve; no `include!` in moved code. |
| Tool-job census probe (bun script importing the exported `toolJob*` functions from root `📜️script.ts`; rows, dispositions, proofs, app-owned rows, scan-then-monolith, proof-catalog failures, `toolJobFactoryProofActivationScan`, 5d and 2d reserved-route exactness) | baseline and after outputs are byte-identical (110,716 bytes; 920 failures overall both times, all pre-existing; 78 / 58 puzzle rows; reserved 5d / 2d results equal). `bun ./📜️script.ts verify interactivity tool-jobs` itself dies at baseline in its first step (`shared-action-fixture: copy differs from its production factory contract`), unrelated to this work. |
| Publication authority audit (the script's own functions, run through a bun copy because `bun repo run @semio-tech/puzzle-js:publication-authority-audit` fails in the build graph with `Selected compile source import missing: ...🧊️3d/📖️stories/...semio_puzzle.js`) | Puzzle5dPlayApp passes before and after; Puzzle2dPlayApp and Puzzle3dPlayApp fail before and after (pre-existing); the 55 `production.includes(...)` clauses have the same false-set before and after for both crates. |
| `completionRejectionOracle(repoRoot)` | ok, 10 rows, before and after (after the `sourceStart` fix above). |
| `bun test` `🔍️discovery/🧪️tests/🔬️rust-policy-source-evidence` | 6 pass, 0 fail with the new roots. |
| Emoji / lengths | see 1 and 4. |

Logs and tools are in `🪓️monolith-split/🗑️generated/puzzle-editors/` (`pspec.py`, `psplit.py`, `pverify.py`, `prun*.py`, `pcheck.py`, `pclosure.py`, `pinstall.py`, `gate-probe.ts`, `audit-probe.ts`, `completion-probe.ts`, `orig5d.rs.txt`, `orig2d.rs.txt`, `pe-*.log|json`). Per the new fleet rule: I created no cargo workspace, manifest, private target dir, or `RUSTC` override; the only compiler use was analysis-only `rustc ... -o /dev/null`. The scratchpad sandboxes (`sbx/{o,s,r,n}{2d,5d}`, symlink trees of sources, no `target`) and error files I made were deleted.

## 8. Not verified / risks

- No `cargo check`, no `cargo test`, no wasm build through `bun repo cargo`. Method / field / type privacy (E0616 / E0624 / `private_interfaces`) and glob ambiguity (E0659) appear only under real name resolution. Likely residual: a private field or method used from a sibling through a path the id scan cannot see (type-inferred access is covered only by name match, so the risk is under-widening of nothing found, over-widening elsewhere).
- The Rust source-text tests (`unit`, `retained-wiring`) were compiled by the oracle (`--test`, identical error set) but not executed.
- `toolJobScanThenMonolithRows` detects helper declarations with a line-anchored regex; widened `pub(super) fn` helpers no longer match it. It returns 0 rows before and after, so no change is visible, but the check could miss a future violation.
- The split gate evidence relies on the new `toolJobPluginProductionSources`; any other per-file census over `✏️s/🔌️plugins` Rust files I did not find would see the children as separate files.
- Wrong-direction edges recorded, **not changed**: 5d `Cargo.toml` still depends on the 3d crates (`semio-s-artifact-puzzle-3d*`, `-editor*`, `-inferences`, `-model`, `-mutations*`, `-precompute*`); the 3d imports now live in `E5/🦀️.rs` lines 42 and 781, `🕹️interaction` 505 and 535, `👆️gestures` 71, `🧩️entities` 524. 2d `Cargo.toml` still depends on `semio-s-artifact-puzzle-3d` and `-3d-inferences`; the 2d editor files have no 3d import.
- Stale line numbers in comments elsewhere that cite the old files are not refreshed. `🗑️generated` was left in place.

## Hardening (patched oracle)

Wave 2, Sonnet executor, 2026-10-15. Full write-up: `RESULT-HARDEN-WAVE2.md`. No repository source file was edited; no regression was found.

**Method.** `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null --error-format=json` from the real lib roots (`🖐️5d/🦀️.rs`, `◻️2d/🦀️.rs`) over a sandbox in which every module file is a copy with the patches applied identically to baseline and split: 15 `extern crate` lines of missing crates stripped (they fail with E0463 and silently swallow every path through their alias) and 1 external-crate glob (`🌉️wasm` bridge) replaced by `const _: () = ();`. Baseline = `orig5d.rs.txt` / `orig2d.rs.txt` (equal to `HEAD`) at the editor path, every other file current (the current unit and retained-wiring tests read the children by `include_str!`, present in both). Module counts 386 vs 394 (5d, +8) and 461 vs 465 (2d, +4). Lossless check of baseline vs split family: only the deleted empty region shells (5d 2 markers, 2d 16) are missing.

| Crate | Config | Baseline / split errors | Non-extern | Differing keys |
|---|---|---|---|---|
| 5d | native | 4,276 / 4,276 | 50 / 50 | 0 |
| | `--test` | 8,340 / 8,340 | 189 / 189 | 0 |
| | wasm32-wasip2 | 4,276 / 4,276 | 50 / 50 | 0 |
| | `--test` wasm32-wasip2 | 8,340 / 8,340 | 189 / 189 | 0 |
| 2d | native | 3,710 / 3,710 | 143 / 143 | 0 |
| | `--test` | 9,826 / 9,826 | 508 / 508 | 0 |
| | wasm32-wasip2 | 3,710 / 3,710 | 143 / 143 | 0 |
| | `--test` wasm32-wasip2 | 9,807 / 9,807 | 508 / 508 | 0 |

Sensitivity of the patch: the unpatched run of the same sandboxes gives 5d 3,157 / 3,157 (non-ext 65) native and 6,884 / 6,884 `--test`, 2d 1,359 / 1,359 and 1,813 / 1,813, i.e. the earlier "identical" results were true but blind to 1,119 to 8,013 errors per configuration. Those errors are now compared and identical.

**Fixes.** None needed: no widening, no `super::super::`, no cfg(test) import, no include depth change.

**Definition scan** (fields, methods, assoc consts, traits, struct literals, tuple constructors, top-level items; uses inside the original `editor` module subtree but outside the child scope): 5d 131 private and 244 narrowed fields, 40 / 59 members, 92 / 62 top-level items; 2d 63 / 58, 28 / 14, 75 / 23. Candidates: 0 and 0. Seeded checks in sandbox copies: stripping `pub(super)` from 5 top-level items of `🖌️session` adds 7 oracle errors (5d, 4,276 to 4,283) and 4 (2d, 3,710 to 3,714); stripping it from fields and methods of `🖌️session` makes the scan report 110 rows (5d: 18 fns, 70 fields stripped) and 37 member plus 4 item rows (2d), against 0 on the real trees.

**Pre-existing and identical in baseline and split (not caused by the split; also at `HEAD`).** 2d: 9 x E0603, private `fn` of `📸️snapshot/🦀️.rs` imported by `📼️recorder/🦀️.rs:7` (`new_node_id`, `patched_field_value`, `puzzle2d_placed_handles`, `unique_edge_id`, `unique_node_id`) and by the editor parent line 17 (`puzzle2d_interaction_definition`, `puzzle2d_node_kind_arg`, `sync_host_fixture_content`, `sync_host_runtime_state`); 2 x E0425 in `--test`, `PUZZLE2D_NODE_KIND_OPTIONS_MAX` used at `🧪️tests/🔬️unit/🦀️.rs:1535` and `:1747` without import; one unreadable fixture include (`payload-input/🔣️.json`).

**Residual.** Typeck-level errors (trait in scope, E0616/E0624 beyond the name scan) are still not produced by rustc here; cargo check remains the final word once the gate admits the crates. Tools: `harden2-tools/` (`ho.py`, `spec.json` targets `p5`, `p2`).
