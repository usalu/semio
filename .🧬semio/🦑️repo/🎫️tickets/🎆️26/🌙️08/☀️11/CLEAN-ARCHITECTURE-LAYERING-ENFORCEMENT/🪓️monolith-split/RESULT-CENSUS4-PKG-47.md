# Result CENSUS4-PKG-47 (prefix `c4p47`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `O` = `🧰️framework/🛍️products/💻️os`. Tools: `F/c4p47-tools/` (descendant of `c4p43-tools`, `c4p39-tools`). Logs: `F/🗑️generated/c4p47-*` (nothing deleted). Stable sandbox root (survives a scratchpad wipe): `/Users/ueli/.cache/c4p47/`.

## 0. Verdict

**DONE** (static evidence; the real `bun repo cargo` check is queued, no `COMPILE ADMITTED` line in `📓️FLEET.md`). Last update 2026-10-17 15:2x.

- All 7 files of Package 47 (8,493 live lines) are split into cohesive taxonomy children: 4 module-style splits (`#[path]` child modules plus `pub use child::*`: workflow graph, shell schema, codec ABI, plugin host: 33 children) and 3 flat test splits (`include!` fragments: dsl unit, neural unit, host unit: 19 children). 8,848 lines after (wiring, docstrings, repeated region markers); roots 18 to 187 lines, largest child 625 lines (`🪢️shell-command-value-bridge`).
- Lossless: the module splits are line-multiset lossless (missing 0, extras are only wiring, docstrings, markers; widened lines compared with their `pub(super)` prefix removed); the three test splits are exact (sequence-equal to the snapshot after splicing every fragment back and reversing the `../` rebase). `#[test]` fn multiset identical (24, 46, 33). Region markers balanced in every file. Every child mounted exactly once. 52 of 52 new folder names RGI and resolved by the live taxonomy. Longest path 165 UTF-8 bytes.
- Gate: all six owning crates `libCompilable` and `testCompilable` with 0 oversized dependencies in the full-workspace census (worst ratio 0.875: `semio-framework-os-dsl-tests` test closure, 28 of 32 files).
- Differential: the typeck-level scoped oracle (`pos`, section 3) compiled the new children (1,596 lib and 4,017 test diagnostic rows sit inside them) and reaches the canary (E0308, E0616, E0624) before and after. Privacy, trait-scope, resolve, type and other non-stub row counts are equal before and after (0, 0, 88, 40, 75 lib; 0, 0, 191, 103, 159 test). The row-level diff is **not** fully identical: lib has 7 extra rows, test has the same 7 extra rows plus 5 rows that only changed the stub trait name (`DslIdiom` to `Tr`); all 12 come from how the oracle's stub layer learns external trait names behind a `use super::*` glob (section 3.3, with a rustc control). Reported, not hidden.
- Three source-text readers handled (section 4): the codec-abi unit test that read the split root as text now reads the eight children plus the root; one stale `lineHint` of the interactivity-audit allowlist re-pointed; the shell schema registry reader needed no edit because `schema_registry` stays in its root.
- No compatibility re-exports, aliases or shims; no crate, manifest, launch route, owner row or lock changed.

## 1. Found state (fresh slice)

- No earlier c4p47 work: no `c4p47-tools`, no `🗑️generated/c4p47-*`, no RESULT file.
- Re-measured before any edit (the census line counts are older than the live tree: four test/schema files were touched by a peer sweep between the census and now). Pre-edit snapshots: `F/c4p47-tools/orig/<tag>.rs`, hashes in `F/c4p47-tools/orig/SHA256`.

| tag | file (tail under `O/`) | lines (census → live) | `#[test]` | owning crate (lib root) | SHA256 (first 16) |
| --- | --- | ---: | ---: | --- | --- |
| gr | `🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🕸️graph/🦀️.rs` | 1,186 → 1,186 | 0 | `semio-framework-artifact-workflow-workflow-graph` (lib root) | `7c05a3e3667ad423` |
| shs | `🔨️modules/🖥️shell/🧬️schema/🦀️.rs` | 1,662 → 1,662 | in-file registry law only | `semio-framework-os-shell` (`#[path]` mod of `🖥️shell/🦀️.rs`) | `9aa9bdc29d18d5fa` |
| dsu | `🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs` | 1,129 → 1,124 | 24 | `semio-framework-os-dsl-tests` (`mod tests` of `🥪️companion/🦀️.rs`) | `455511cdc66e66aa` |
| neu | `🔨️modules/🧠️neural/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` | 1,138 → 1,136 | 46 | `semio-framework-os-kernel-neural-engine` (`#[cfg(test)] mod tests`) | `92fd55c0ea773d65` |
| cab | `🖥️host/📡️codec-abi/🦀️.rs` | 1,013 → 1,013 | 0 | `semio-framework-os-host-codec-abi` (lib root) | `77866e302a6ce5e8` |
| ph | `🖥️host/🔌️plugin-host/🦀️.rs` | 1,332 → 1,332 | 0 | `semio-framework-os` (`pub mod host`, via `🖥️host/🦀️.rs`) | `9d2b2fcdc3b90b1f` |
| hu | `🖥️host/🧪️tests/🔬️host-unit/🦀️.rs` | 1,041 → 1,040 | 34 | `semio-framework-os` (`#[cfg(test)] include!` at the end of `ph`) | `f09ae44069fb9b82` |

- Mount facts: `ph` is the `host` module of the `os` host crate and ends with `#[cfg(test)] include!("../🧪️tests/🔬️host-unit/🦀️.rs");`; `hu` is one inline `mod tests { use super::*; ... }`. `cab` ends with `#[cfg(test)] include!("../🧪️tests/🔬️codec-abi-unit/🦀️.rs");` and that test reads `cab` as text (`include_str!("../../📡️codec-abi/🦀️.rs")`): a source-text reader to re-point.

## 2. What was cut (before → after, children in lines)

`O` = `🧰️framework/🛍️products/💻️os`. Children sit next to their root as `<emoji>️<kebab>/🦀️.rs`.

| tag | before | root + children (max child) | children (lines) |
| --- | ---: | --- | --- |
| gr | 1,186 | 60 + 9 (289) | `🤝️media-contract` 28, `🔢️media-enum-ordinals` 165, `🫳️borrowed-record-specs` 100, `📐️media-contract-record` 194, `🔌️media-port` 130, `🧱️graph-model` 102, `✅️validation-and-planning` 120, `🎛️parameters` 289, `📥️inputs` 94 |
| shs | 1,662 | 187 + 9 (625) | `🌉️json-value-bridges` 103, `🧭️anchors-and-layout` 113, `🔌️plugin-runtime-and-overlays` 96, `🔄️sync-merge-and-scope` 70, `📋️shell-state` 174, `🎮️shell-command` 94, `🪢️shell-command-value-bridge` 625, `📣️events-and-errors` 79, `🛰️capabilities` 181 |
| dsu | 1,124 | 25 + 4 (406) | `🧱️dsl-field-and-derived-document-laws` 281, `🌳️scene-and-shape-laws` 193, `🖌️paint-self-ref-and-table-laws` 406, `📦️paged-operation-laws` 231 |
| neu | 1,136 | 18 + 8 (282) | `🔁️dictionary-and-registry-laws` 160, `🔀️wire-channel-and-collect-laws` 98, `🧵️cluster-and-top-level-laws` 132, `🗂️cached-evaluation-laws` 111, `⏱️budgeted-walk-laws` 282, `📚️cardinality-and-schema-component-laws` 133, `🌊️dirty-set-and-wave-laws` 181, `🚦️owned-control-laws` 45 |
| cab | 1,013 | 42 + 8 (347) | `🧬️operation-schema` 100, `📊️progress-records` 57, `🔤️utf8-cursor` 59, `🔎️filter-kinds-cursor` 190, `🪪️normalize-kind-cursor` 93, `🎟️codec-session` 347, `🛰️retained-service` 137, `🚪️direct-service` 62 |
| ph | 1,332 | 63 + 7 (302) | `🔁️host-lifecycle` 302, `🗃️backbone-document` 295, `🩹️workflow-reconcile` 211, `🏪️workflow-store` 131, `🔗️backbone-ports` 81, `📚️space-catalog` 194, `🧩️extension-install` 123 |
| hu | 1,040 | 24 + 7 (217) | `🔌️plugin-registry-and-hot-swap` 217, `🛟️rollback-contributions-and-recovery` 166, `🧰️host-fixtures` 94, `🎭️actor-context-and-store-laws` 125, `🌱️instances-parameters-and-catalog` 101, `📝️dsl-and-op-text-laws` 208, `🧩️extension-install-laws` 126 |

Totals: 8,493 lines in 7 files became 8,848 lines in 7 roots (18 to 187 lines) plus 52 children (largest 625). The growth of 355 lines is wiring, one docstring per child and the region markers that are re-opened in every child (region marker counts per file in section 3.1).

### Mechanism and decisions

- **Module splits (gr, shs, cab, ph)**: whole items (with attributes and docs) move; each child starts with a `//! <emoji> ... See [`super`].` docstring and `use super::*;`; the root keeps its `use` lines and gets `#[path = "<folder>/🦀️.rs"] mod x;` plus `pub use x::*;` (or a private `use x::*;` when the child holds only private items, or no re-export for the impl-only `🪢️shell-command-value-bridge`). Re-export visibility follows the widest visibility of the named items. `🧩️extension-install` is gated twice with the same cfg (the `mod` line and the re-export), because every item of it is `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]`.
- **Visibility** (`widen2.py`): a moved private item, field or inherent method becomes `pub(super)` when its name occurs in another file of the split scope: gr 15 lines, shs 7 (the four `#[value(with = ...)]` bridge modules and three `CommandMeta` fields), cab 73, ph 10 (reports in `F/c4p47-tools/widen/`). The scope includes the files that read the split file's private items: the shell unit-test module, the host unit test (`orig/hu.rs`) and the codec-abi unit test. `pub(super)` items stay out of the public API (the glob re-export caps them).
- **Roots keep what is genuinely theirs**: `shs` keeps `pub mod schema_registry` (113 lines) because `📦️packages/🦀️rust/📜️script.ts` `rustRegistryExportIds` regex-reads `SchemaExport { name: "..." }` rows from that exact file, plus the UI preference re-export and the `#[cfg(test)]` mount; `cab` and `ph` keep their `#[cfg(test)] include!` of the unit tests; `ph` keeps its `// #region host` pair around everything.
- **Flat test splits (dsu, neu, hu)**: `include!` fragments spliced back into the same module (`fragsplit.py`, descendant of PKG-39), so test paths (`<mod>::<test>`), visibility and `use` scope are unchanged and nothing is widened. `hu` is one inline `mod tests { ... }`: its root keeps the header and the nine `use` lines and carries the seven `include!` lines inside the module body (fragments at brace depth 1). The two `#[path] mod controlled_*_refusal;` mounts of `dsu` stay in its root between fragments. Relative `include_str!` literals of fragments got one more `../` (dsu 5, neu 3, hu 1; targets proven equal, section 3). A fragment head is a `///` docstring where its first line is an item, `//` where it starts with a region marker (`hu` DSL and extension chunks).
- **dsu was cut into 4 fragments instead of 6** so its test closure stays at 28 of 32 files (the carving rule asks for 28).
- **neu** has one chunk with a different shape than the first cut: the cache-epoch test sits between the budgeted-walk tests in the original and stays there (`⏱️budgeted-walk-laws`), so the splice reproduces the original sequence exactly.
- No qualified path was added, no compatibility re-export, alias or migration shim.

## 3. Evidence (all commands run in this session on the live tree unless noted)

### 3.1 Structure and losslessness

| Check | Command (tools in `F/c4p47-tools/`) | Result |
| --- | --- | --- |
| Install precondition | `python3 c4p47.py install <gr|shs|cab|ph>`, `python3 install.py check` then `apply` | each asserted the live file equals its snapshot and the child folders do not exist; `children 19, roots 3, problems 0`, `installed 22 files` |
| Module splits lossless | `python3 c4p47.py lossless <tag>` (stage) and `python3 live.py gr shs cab ph` (live, `c4p47-lossless-mod-live.log`) | `LOSSLESS OK` x4, `missing 0`, live files byte-equal to the stage; regions orig/new 6/10 (gr), 30/30 (shs), 3/10 (cab), 15/22 (ph), `#endregion` counts equal |
| Test splits exact | `python3 lossless.py`, `python3 incl.py` (`c4p47-lossless-frag-live.log`, `c4p47-incl-frag-live.log`) | `LOSSLESS OK`: `gone=0 extra=0 sequence-equal=True` for dsu, neu, hu (spliced fragments reproduce the snapshot line for line); `INCLUDES OK`: include_str targets equal (5, 3, 1), 0 missing |
| Structure | `python3 verify.py live` (`c4p47-verify-live.log`), `python3 check.py gr shs cab ph` (`c4p47-check-live.log`) | `VERIFY OK live` (test fn multiset 24 to 24, 46 to 46, 33 to 33, docstring emoji first and unique among siblings, regions balanced, brace depth 0, rustfmt parse); `CHECKS OK` (33 children, each mounted exactly once, doc emoji equals folder emoji and differs from the root emoji, regions balanced, no `[DEBUG]`, longest child 625 lines, longest path 160 bytes) |
| Include targets, module splits | `python3 incl-mod.py` | `INCLUDES OK`: gr 0 to 0, shs 1 to 1, cab 2 to 2 (schema JSON and the test include), ph 1 to 1 literals, same absolute targets, 0 missing |
| Definition scan (private items, fields, members used across files) | `python3 defscan.py <tag>` | 0 cross-file leftovers for gr (20 declarations), shs (3), cab (13), ph (24) |
| Child module name clash | `python3 modclash.py gr shs cab ph` | `modclash OK` |
| Formatting debt | `python3 fmt.py` (`c4p47-fmt-live.log`) | per file: changed-line multiset of `rustfmt --check` (blank-normalised, `pub(super) ` and `../` removed): cab and ph, dsu, neu identical; gr and shs: the only difference is 2 removed blank-line hunks each (less debt); hunk counts original/new gr 22/26 (adjacent hunks no longer merge), shs 8/7, cab 2/2, ph 11/11, dsu 40/40, neu 22/22. `hu`: not comparable: rustfmt never visits `include!`d files, and a depth-1 fragment parsed standalone shows its indentation. The debt a first stage had (extra blank line in `🔤️utf8-cursor`, un-spaced cfg on the `ph` re-export) was fixed in the final state |
| Taxonomy and names | `node c4p5-tools/rgi.mjs c4p47-tools/newnames.txt`, `bun c4p47-tools/tax.ts c4p47-tools/newdirs.txt` | `bad=0` for 52 names; `{"dirs": 52, "unresolved": []}` through the live `semanticDirectoryKindId`; every emoji already used as a folder prefix in the repo (`emo.py`) |
| Path length | in `c4p47.py paths`, `check.py` and an inline scan | longest repo-relative path 165 UTF-8 bytes (limit 240), longest absolute path 144 chars / 193 bytes (limit 256), over all 61 final files (`final-SHA256`) |
| Glob visibility control | `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null /Users/ueli/.cache/c4p47/control/glob_private_use.rs` and `_neg.rs` | positive control (child module with `use super::*` using the parent's private `use` of a trait and a type, re-exported with `use child::*`) compiles, only the `couldn't create a temp dir /dev/rmeta` environment line appears; negative control (the parent import removed) fails with E0404 |

### 3.2 Census (canonical `readRustSourceClosureV1`)

`bun census-own.ts <root>` before (`c4p47-census-own-before.jsonl`) and after (`-after.jsonl`), files / bytes / active lines (ratio of the worst limit 32 / 262,144 / 4,000):

| crate (tag) | lib before | lib after | test before | test after |
| --- | --- | --- | --- | --- |
| `semio-framework-artifact-workflow-workflow-graph` (gr) | 1 / 85,333 / 1,187 (0.326) | 10 / 88,231 / 1,292 (0.337) | same | same |
| `semio-framework-os-shell` (shs) | 4 / 191,188 / 2,047 (0.729) | 13 / 193,666 / 2,116 (0.739) | 6 / 221,144 / 2,493 (0.844) | 15 / 223,622 / 2,562 (0.853) |
| `semio-framework-os-dsl-tests` (dsu) | 1 / 1,945 / 0 | 1 / 1,945 / 0 | 24 / 168,351 / 1,925 (0.750) | 28 / 169,407 / 1,941 (0.875) |
| `semio-framework-os-kernel-neural-engine` (neu) | 2 / 28,417 / 537 (0.134) | 2 / 28,417 / 537 | 8 / 119,476 / 1,889 (0.472) | 16 / 121,217 / 1,921 (0.500) |
| `semio-framework-os-host-codec-abi` (cab) | 2 / 43,081 / 1,013 (0.253) | 10 / 46,450 / 1,095 (0.313) | 4 / 68,438 / 1,457 (0.364) | 12 / 72,544 / 1,550 (0.388) |
| `semio-framework-os` (ph, hu) | 7 / 117,529 / 1,971 (0.493) | 14 / 120,050 / 2,045 (0.511) | 11 / 214,461 / 3,145 (0.818) | 25 / 218,494 / 3,247 (0.833) |

Full-workspace census (`cgc-census.ts`, a copy of `🗑️generated/cgc7-census.ts` that reads the shared manifest list but writes only `c4p47cgc-*`; 2,865 crates, `readerRefusedCrates` 0, 10 oversized lib and 3 test-only, none of them mine): all six crates above have `libOver false`, `testOver false`, `libCompilable true`, `testCompilable true`, `libOverDeps 0`, `testOverDeps 0`, with the same files, bytes, lines and ratios as the table (`c4p47cgc-crate-census.csv`, `c4p47-cgc-census.out`).

### 3.3 Scoped differential (typeck-level `pos` oracle, patched stub layer)

Family `F/c4p47-tools/families/c4p47.json` (the six owning crates; stub layer for everything outside), run with `PYTHONHASHSEED=0 ulimit -n 65536; POS_PREFIX=c4p47 python3 posrun.py run families/c4p47.json --cfgs lib,test --label {pre,post,post2} --no-base`, nice 10, sandbox `/Users/ueli/.cache/c4p47/c4p47-work`, labels `pre` (live tree before any edit), `post` (first install) and `post2` (final tree); diff with `python3 posdiff.py c4p47-pre c4p47-post2 lib test` (keyed by cfg, code, normalised message, label and crate; paths and lines dropped).

- Baseline: lib 1,846 errors, test 5,715; typeck canary (E0308, E0616, E0624) reached in every run; privacy 0 and trait-scope 0 non-stub rows before and after; resolve 88 / 191, type 40 / 103, other 75 / 159 (lib / test) non-stub rows, equal before and after (stub-layer noise of the external crates).
- Determinism: `post` against `post2` is **IDENTICAL** (1,852 lib and 5,721 test rows, `c4p47-posdiff-post-vs-post2.log`).
- The children were compiled by the oracle: in `post2` 1,596 of the 1,856 lib rows and 4,017 of the 5,725 test rows lie in the new child files (22 and 41 children carry rows); in `pre` none do, as expected.
- `pre` against `post2` (`c4p47-posdiff-post2.log`, `POSDIFF 2 cfg differ`): lib +7 rows, test +12 / -5 rows, all in two groups:
  1. 7 x `E0277 the trait bound Op: Tr<P> is not satisfied` at `🗃️backbone-document:226,235` (lib and test), the residue of 9 first-iteration `E0404 expected trait, found struct Mutation` rows in `🗃️backbone-document` (8) and `🔗️backbone-ports` (1) that the oracle neutralises to `Tr`.
  2. dsu: 5 rows `E0407/E0437/E0438 ... is not a member of trait DslIdiom` (before) are the same 5 rows with the trait name `Tr` (after).
  Cause: the oracle learns whether an external name is a trait from trait-position evidence in the file that imports it. `Mutation` (`use protocol::Mutation;`) and `DslIdiom` (`use semio_framework_dsl::DslIdiom;`) are imported in the root and used in a child (or an `include!` fragment, which the oracle does not read as a file) through `use super::*`, so the stub stays a struct, the use is neutralised to the generic stub trait `Tr`, and the same bound or impl error appears under another trait name. It is a limit of the stub layer, not a resolution failure: the rustc control (section 3.1) shows a child resolving a parent-imported trait through `use super::*`, the idiom every child of this fleet's earlier splits uses. The real `bun repo cargo` check (section 6) is the confirmation.

The simple patched differential (`oracle.py`, extern crates removed, external globs neutralised; `c4p47-oracle-stage-*.log` on the stage and `c4p47-oracle-live.log` on the live tree) is `ORACLE <tag> CLEAN` for gr, shs, cab, host (ph and hu together), dsu and neu in every configuration (lib, test, wasm32-wasip2, wasm32-unknown-unknown where the crate builds for it): error counts equal (537, 95/132, 2/3, 225/395/182, 1,317, 7/130), warnings equal. It is weak here because the missing external crates stop rustc early; it proves the mount structure only.

## 4. Source-text readers

Search method: `git grep -F` of each file's path suffix and crate name over `.ts/.tsx/.rs/.py/.mjs/.sh/.json/.toml/.jsonc` (excluding the ticket tree), plus a read of every candidate.

| Reader | What it reads | Action |
| --- | --- | --- |
| `O/🖥️host/🧪️tests/🔬️codec-abi-unit/🦀️.rs` `public_interactive_route_has_no_batch_or_whole_input_capability` | `include_str!("../../📡️codec-abi/🦀️.rs")`, cuts at `//#region 🧪️Tests` and asserts forbidden strings are absent and `RegisteredOsHostFormatResolver`, `FilterKindsStructuralCursor`, `NormalizeKindStructuralCursor` are present in the production text | **edited**: the source is now `[<eight children>, <root>].concat()` of `include_str!` literals (children first, so the root's Tests region still ends the production text); the forbidden strings do not occur in the new docstrings (checked) |
| `📚️library/⚖️laws/🔬️interactivity-audit/🗂️files/🟦️.ts` allowlist entry for `🖥️host/🔌️plugin-host/🦀️.rs` | `lineHint: 1331` pointing at the `include!` of the host unit test (`expectedNeverToMatch: true`, the scanner skips `/🧪️tests/` paths and `lineHint` is not read by any logic) | **edited**: `lineHint: 62` (the `include!` line in the new root) |
| `O/🔨️modules/🖥️shell/📦️packages/🦀️rust/📜️script.ts` `rustRegistryExportIds` | regex over `🧬️schema/🦀️.rs` for `SchemaExport { name: "..." }` rows | **no edit**: `pub mod schema_registry` stays in the schema root |
| `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts` (line 320, 398) and `🌎️hub/🧩️compositions/🗄️stdio/...` (line 832) | `readModuleTreeSource` / `readFileSync` of `O/🖥️host/🦀️.rs` (the host root, not the split file) | **no edit**: `readModuleTreeSource` follows `#[path]` and `include!` mounts (probe `bun /Users/ueli/.cache/c4p47/probe-tree.ts`: host root tree 96,183 bytes, ph root 8 children, cab 9, gr 9, shs 10), and the stdio script only matches `pub mod host;` lines of the host root |
| `📚️library/🔍️discovery/📖️source-access/🟦️.ts` `POLICY_SPLIT_SOURCE_ROOTS` | lists `O/🖥️host/🦀️.rs` and `O/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` | **no edit**: roots that are not split files; they already read as module trees |
| `Cargo.toml` (workspace), `📋️build-owners.json`, `🔒️dependencies.json`, `.vscode/launch.json`, `🧩️launch.seed.jsonc`, `📋️project.json` globs (`**/*.rs`, `🧬️schema/**/*`), `📚️library/🔣️taxonomy.json` | crate roots, manifests, six launch routes, nx inputs, folder-kind names | **no edit**: no crate, target, command, owner row or lock changed; the project inputs are `**` globs and the new folders resolve in the taxonomy |
| Doc mentions (`🏪️store/✉️envelope`, `🪐️space/⚙️engine`, `🧬️schema/🟦️.ts`, `📜️script.ts` comments) | file paths in prose | **no edit** |

## 5. Foreign errors and findings

- Pre-existing, not touched: the 88 / 191 resolve, 40 / 103 type and 75 / 159 other non-stub oracle rows are present before and after the split (external crates behind the stub layer); `rustfmt` debt in `gr` (22 hunks), `cab` (2), `ph` (11) existed before.
- Incident, fixed before the final state: the first re-stage after the install used `mounted()` on the already split root and so treated the new children as "other files" when planning visibility, which over-widened 14 live files to `pub(super)`. It was caught by comparing live against the re-stage, the planner now reads the root text from the snapshot (`specs/lib.py` `mounted(..., orig=)`), and the live files were rewritten from the corrected stage (widened counts 15, 7, 73, 10 again equal the first stage, `live.py` byte-equal to the stage).
- A peer sweep touched the four test and schema files between the census and this slice (the census counts differ from the live counts); the snapshots are the state at 14:3x.

## 6. Compile queue (not admitted)

No `COMPILE ADMITTED` line in `📓️FLEET.md` (checked 15:09) and no mutex folder `🗑️generated/cargo-mutex.lock` at that time. Command for the compile queue (hold the fleet mutex around the single launcher call):

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-artifact-workflow-workflow-graph -p semio-framework-os-shell -p semio-framework-os-dsl-tests -p semio-framework-os-kernel-neural-engine -p semio-framework-os-host-codec-abi -p semio-framework-os --lib --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
```

A `--target wasm32-wasip2` variant of the same call covers the `semio-framework-os` p2 gating of `🧩️extension-install` and the shell and graph crates.

## 7. Files

- Edited (7 roots): the seven files of section 1, now wiring roots (`gr`, `shs`, `cab`, `ph`) or `include!` roots (`dsu`, `neu`, `hu`).
- Edited (2 readers): `O/🖥️host/🧪️tests/🔬️codec-abi-unit/🦀️.rs` (+13 / -2 lines), `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗂️files/🟦️.ts` (1 line).
- Created (52 children, one `🦀️.rs` per new folder): the folders in section 2; the list with their absolute paths is `F/c4p47-tools/newdirs.txt`, the SHA256 of every final file is `F/c4p47-tools/final-SHA256`.
- Tools, kept: `F/c4p47-tools/` (`c4p47.py` stager, lossless, installer; `specs/*.py`; `mkfrag.py`, `fragsplit.py`, `specs-frag/*.json`; `install.py`, `lossless.py`, `incl.py`, `incl-mod.py`, `verify.py`, `check.py`, `live.py`, `defscan.py`, `modclash.py`, `globuse.py`, `members.py`, `testnames.py`, `fmt.py`, `emo.py`, `widen2.py`, `rsitems.py`, `oracle.py`, `posrun.py`, `posdiff.py`, `families/c4p47.json`, `census-own.ts`, `cgc-census.ts`, `tax.ts`, `orig/` snapshots with `SHA256`, `widen/`). Logs: `F/🗑️generated/c4p47-*` (pos runs `c4p47-pos-{pre,post,post2}.log` with the `c4p47-c4p47-*` dumps, `c4p47-posdiff-*`, `c4p47-oracle-*`, `c4p47-census-own-*`, `c4p47-cgc-census.*` and `c4p47cgc-*`, `c4p47-fmt-live.log`, `c4p47-verify-live.log`, `c4p47-lossless-*`, `c4p47-incl-*`, `c4p47-check-live.log`, `c4p47-defscan.log`, `c4p47-modclash.log`); nothing deleted. Stable sandbox/stage root: `/Users/ueli/.cache/c4p47/` (`stage/`, `stage-frag/`, `sbx/`, `c4p47-work/`, `control/`).
- No `[DEBUG]` log was added to the repo; `cgc-census.ts` is a copy of the shared census tool and keeps its progress lines.
