# Result: repo crate root splits (WP-C, prefix `rcr`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. Tools: `rcr-tools/` (originals in `rcr-tools/orig/`). Logs and sandboxes: `🗑️generated/rcr-*` (nothing deleted).

Paths are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/` unless they start with `.` or `/`. `P` = `<module>/📦️packages/🦀️rust`.

## 0. Verdict

All six crate roots are split and installed. Each root now holds only the header docs, the imports, the crate's `pub use` re-exports, the `🔖️Children` wiring block and its tests mount. Everything else moved verbatim into 6 or 7 cohesive children `P/<emoji>️<kebab>/🦀️.rs`, mounted as `#[path = "<folder>/🦀️.rs"] mod <snake>;` plus `pub use <snake>::*;`.

Static verification is complete and clean (section 4). **No cargo/rustc build or test of any repo crate was run**: the real check is queued (section 6). Nothing outside the six roots was edited, and no reader of these roots exists (section 3).

| Crate (`semio-framework-repo-...`) | Root before | Root after | Children | Largest child (raw / active) | Files in own source closure (incl. unit tests) | Active lines in own closure |
| --- | ---: | ---: | ---: | --- | --- | --- |
| `tree` (`🌳️tree`) | 2,390 | 81 | 7 | `🧱️tree-model` 773 / 608 | 2 -> 9 | 1,973 -> 2,001 |
| `test-runner` (`🏃️test-runner`) | 2,153 | 77 | 6 | `⚙️execution-and-parsing` 518 / 397 | 1 -> 7 | 1,739 -> 1,762 |
| `hooks` (`🪝️hooks`) | 2,142 | 84 | 7 | `🛡️tool-policy` 546 / 492 | 2 -> 9 | 1,815 -> 1,843 |
| `mcp` (`🔌️mcp`) | 2,134 | 58 | 7 | `📨️protocol` 414 / 335 | 2 -> 9 | 1,926 -> 1,953 |
| `providers` (`🧩️providers`) | 1,754 | 54 | 7 | `🐙️github-management` 496 / 441 | 2 -> 9 | 1,555 -> 1,584 |
| `languages` (`🗣️languages`) | 1,710 | 65 | 6 | `📑️section-parsing` 374 / 344 | 2 -> 8 | 1,700 -> 1,723 |

Every child is far below the 1,200-line target. The own-closure numbers stay under the compiler-ingress source limits (4000 active lines, 32 files, 262144 bytes; bytes rose 2-3 KB per crate for the docstrings and mount lines). The longest new path is 150 bytes (limit 240).

## 1. Layout

Children follow the existing regions. A region is never cut; the only cut inside one is the mcp `🧾️Json` region, whose four `use` lines stay in the root (section 2). Line ranges refer to the HEAD roots (`rcr-tools/orig/<name>.rs.txt`, equal to the live files when I started; `git diff --stat` on the six roots shows only deletions of moved lines).

**tree**: `🧱️tree-model` (43-809: Vocabulary, Ports, Records, NodeHelpers), `🩻️monorepo-tree` (811-1258), `🎯️goal-tree` (1260-1509), `⚖️statute-tree` (1511-1727), `🧹️sort-filter-search` (1729-1981: Sorting, Filtering, Search), `🎨️rendering` (1983-2316: Rendering, Mermaid, MermaidLoc), `📌️cache` (2318-2382: Cache, Encoding). Root keeps 1-41 and the Tests region. Ports and Records reference each other, so they share a child with the node helpers every builder uses.

**test-runner**: `🧮️paths-and-identity` (26-202), `🌲️snapshot-and-scope` (204-534), `🗺️planning` (536-979), `⚙️execution-and-parsing` (981-1492; the two reference each other), `🔎️resolution` (1494-1967), `💽️real-world` (1995-2153, the only layer touching the machine). Root keeps 1-24, the five cross-type `*_to_json_text` fns (1969-1993, they combine types of several children, so they are the crate's top-level API) and nothing else.

**hooks**: `🎯️hook-model` (48-227 plus the `🛤️Paths` region 2020-2043, which every reader uses), `🧭️payload-reading` (229-700), `🛡️tool-policy` (702-1241: ToolClassification and BlockingPolicy share `split_command_segments`), `🗺️plan-steps` (1243-1355), `🔶️dispatch-and-formatting` (1357-1753), `📓️session-logging` (1755-1975), `💽️system-ports` (MicroCommit 1977-2018 plus SystemPorts 2045-2136). Root keeps header, imports, the `🔁️Reexports` region and Tests.

**mcp**: `🔏️digest-and-json` (15-88, 93-236), `📨️protocol` (238-645), `📡️event-log` (647-909), `🔐️session` (911-1293), `🚦️routing` (1295-1644), `🗄️repository` (1646-2000), `🚀️entrypoint` (2002-2126; not `🦀️entrypoint`, because the emoji would repeat the sibling file `🦀️.rs`). Root keeps the header region, the four `use` lines (89-92) and Tests.

**providers**: `📐️provider-model` (17-346: ProviderError, ModelPending, McpClientKind), `🏃️process-runner` (348-514), `🔭️provider-interfaces` (516-699: ProviderInterfaces and Ports; mounted with `#[macro_use]`, section 2.5), `🐙️github-management` (701-1190), `🌿️version-control-and-sandbox` (1192-1368), `🎆️editor-providers` (1370-1642), `🎖️registry-and-git-hooks` (1644-1748). Root keeps docs, `#![allow(async_fn_in_trait)]`, the four imports and Tests.

**languages**: `🪪️identity-pending` (12-90), `🗂️language-table` (111-366), `🧩️pattern-matcher` (368-667), `📑️section-parsing` (669-1036), `🧷️definition-parsing` (1038-1344), `🧭️scopes-and-headers` (1346-1702). Root keeps docs, imports, the `🔖️Records` region (the `pub use` of the model types plus `DefinitionRange`, 92-109) and Tests.

Folder emoji: the 36 distinct emoji of the 40 folders are RGI, followed by U+FE0F, already used as folder prefix elsewhere in the repo (3 to 259 uses each, `rcr-tools/verify_fmt_emoji.py`), unique among the siblings of one package dir, and none repeats a sibling file (`📋️`, `📜️`, `🦀️`). Region markers inside the children are untouched; each child starts with a docstring whose first character is the folder emoji.

## 2. How the split was done

1. `rcr-tools/split.py` claims line ranges (every claimed line exactly once, every cut checked to lie between top-level items and never between an item and its attached doc comments or attributes), writes children verbatim, and writes the root from the unclaimed lines plus the wiring block placed after the imports (`anchor` in `spec.py`).
2. **Imports stay in the root.** Each child starts with `use super::*;` (the established fleet convention). A child gets no `use super::*;` when it references no root import and no sibling item (identity-pending, paths-and-identity, digest-and-json); this was checked by identifier overlap and confirmed by rustc's own `unused_imports` lint in the sandbox (0 warnings in the split trees, 0 in the baselines). The root's own mount globs are `pub use` when the child has `pub` items.
3. **Visibility widening, `pub(super)` only, never wider.** Splitting turns module-private into child-private, so private top-level items, struct fields and inherent methods that other files use got `pub(super)` (`rcr-tools/split.py`, name-based superset: top-level names by identifier, fields by `.field` access or struct name plus field name, methods by `.m`/`::m`). Closure over signatures widens the private types a widened signature names. Counts: tree 6, test-runner 0, hooks 5, mcp 46, providers 4, languages 5 (lists: `🗑️generated/rcr-<name>-widen.tsv`). The scan is a superset: some fields share a name with a public field of another struct (hooks `repo_root`, providers `runner`, `issued`), so a few widenings are harmless extras. The unit-test files (`🧪️tests/🔬️unit/🦀️.rs`, `use super::*`) count as "other files".
4. **Two reader lines inside moved code** needed one extra `../` because the code is now one directory deeper: `mcp` `🗄️repository` (`include_str!("../../../🖼️assets/🔣️descriptions.json")`) and `languages` `🗂️language-table` (`include_str!("../../../🧬️schema/🔣️languages.json")`). Both targets exist (path check). No other path changed; the root's tests mount `#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]` is untouched.
5. **providers macros.** `#[dyn_enum]` captures a trait into a hidden `#[macro_export]` macro `__semio_dispatch_<Trait>!` that `dyn_enum_close!` calls by bare name (textual scope, see the dispatch crate docs). Traits `ManagementProvider`, `VersionControlProvider`, `SandboxProvider`, `EditorProvider` are declared in `🔭️provider-interfaces` and closed in later siblings, so that child is mounted first with `#[macro_use]`. `ProcessRunner` and its closer, and `plain_editor_provider!` with its uses, each stay inside one child. I probed the language rule with a toy crate (`rustc +nightly-2026-07-20 --emit=metadata`, output to a scratchpad `.rmeta`, not a repo crate): a macro-expanded `#[macro_export]` macro in a `#[macro_use] mod` is found by bare name in later sibling modules without warnings, and an enum generated by a closer in one sibling is usable from the macro module and a third sibling through `use super::*` glob chains. That probe uses declarative macros, not the real proc macros (section 5).

## 3. Source-text readers and edits outside the six roots

- Searches (`git grep -F`): the six full path tails `<module>/📦️packages/🦀️rust/🦀️.rs`, each module name plus `🦀️`/`rust`/`.rs` in `📚️library` (all `.ts`, `.rs`, scripts; json/md excluded), `include_str!`/`include!` in the six module trees, and every module test adapter (`🧪️tests/*/🟦️.ts`).
- Result: **no reader of these roots exists** in `📚️library` or elsewhere, so no reader line was re-pointed and nothing in `📚️library` was touched. The only mention of a root file is a doc comment `@see ...🔌️mcp/📦️packages/🦀️rust/🦀️.rs` in `⌨️cli/🦀️.rs:26`, which still names an existing file. The package `📜️script.ts` files only drive cargo; `📋️project.json` and `📋️build-owners.json` use the package directory as root, not file lists.
- Edits: the six crate roots, plus the 40 new child files. Nothing else.

## 4. Verification actually run (all against the installed tree unless noted)

| Check | Command (from `rcr-tools/`) | Result |
| --- | --- | --- |
| Baseline | `python3 snapshot.py`, `python3 oracle.py all base` before any edit | snapshots of the 6 module trees; baseline equals current for all 12 configs |
| Patched full-resolution differential, lib and `--test` | `python3 oracle.py all real` (`rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null [--test]` on the lib root of a sandbox holding the original vs a sandbox copy of the real tree; messages compared as multisets, paths and lines stripped; extern-crate globs/`extern crate` patch: 0 occurrences) | IDENTICAL in all 12: tree 143/143 (lib and test), test-runner 54/54, hooks 47/47, mcp 56/56 and 64/64, providers 69/69 and 74/74, languages 37/37 and 39/39. Log `🗑️generated/rcr-oracle-real-normal.log` |
| Deeper differential | `RCR_DEEP=1 RCR_DEEP2=1 python3 oracle.py all realdeep`: same, but the sandbox drops serde attributes/derives and external `use` lines identically on both sides so rustc gets further | IDENTICAL in all 12 (tree 306/319, test-runner 43/43, hooks 276/280, mcp 56/64, providers 75/82, languages 34/38). Log `rcr-oracle-real-deep.log` |
| Oracle sensitivity | same differential on trees staged without the widening (`RCR_NOWIDEN=1`) | mcp 56 -> 115 errors (E0616 private field x32, E0624 private method x13, E0425, E0433); tree 143 -> 258 (E0425 for `text`, `data_of`, `list`, `number`, `nest_ordered`); hooks 47 -> 52. So the oracle sees privacy where rustc reaches type checking (only mcp), and top-level names everywhere |
| Warnings | the same runs print rustc warnings | 0 warnings in the baselines and in the split trees (no unused import, no private-interface lint) |
| Lossless line multiset | `python3 verify.py all real` | 0 lost non-blank lines in all six (after undoing the inserted `pub(super) ` and the two `../` edits). Extras are only mount lines, docstrings, `use super::*;` and the `🔖️Children` region markers (tree +51 lines, test-runner +43, hooks +51, mcp +50, providers +52, languages +43 non-blank) |
| Item set | same script | 0 top-level items lost; the extras are exactly the mounts and `use super::*;` |
| Regions | same script | all original `//#region`/`//#endregion` markers present; only `🔖️Children` is new (one pair per root) |
| Public API | `python3 apicheck.py` | public item sets identical (tree 102, test-runner 88, hooks 88, mcp 72, providers 61, languages 48); 0 top-level name defined in two files; impl blocks 13/14/15/30/24/1 unchanged |
| Definition scan | `python3 defscan.py real` | 0 further widenings proposed (the planner is idempotent on the installed tree) |
| Path/include check | in `verify.py` | 8/6/8/9/8/8 `#[path]`/`include_str!` targets checked, 0 missing |
| Duplicate mounts | `python3 dupmounts.py real` | CLEAN: every target mounted once; each root has original `#[path]` count + number of children; no other tracked source mounts a new path |
| Module-name collisions | in `verify.py` | none (child module names vs top-level items and `use` names) |
| rustfmt | `python3 verify_fmt_emoji.py real` (`rustfmt +nightly-2026-07-20 --config-path rustfmt.toml --edition 2021 --check --config skip_children=true` per file) | 0 parse failures in 46 files. The baseline roots are not rustfmt-clean (diff hunks: 26/18/11/12/2/17); the split trees have 26/17/11/12/2/17, so nothing new |
| Emoji admission | same script, `\p{RGI_Emoji}` via bun | 36/36 distinct emoji OK |
| `[DEBUG]` | none added | |

## 5. Not verified, and why

- **No real compile.** `bun repo cargo` was not run (Wave 6: headroom refusal, static route). The differential cannot resolve external crates (`serde`, `serde_json`, the sibling repo crates, `dispatch_macros`), so type checking stops early everywhere except mcp. Residual risks for the first real check:
  1. Field/method privacy (E0616, E0624) in tree, test-runner, hooks, providers and languages rests on the name scan and the idempotent definition scan, not on rustc. Fix a gap with `pub(super)` at the definition, never with a qualified path.
  2. providers: `#[macro_use]` on `🔭️provider-interfaces`, and `#[dyn_enum]`/`dyn_enum_close!` reaching the children through the root's `use dispatch_macros::{...}` and `use super::*;` (language-level behaviour probed with declarative macros only). Fallback if rustc rejects it: give each of the five children that use a macro its own `use dispatch_macros::{...};` (that adds four expected `E0432 unresolved import dispatch_macros` lines to the sandbox differential, as external-crate noise) or move each trait next to its closer.
  3. `rust_2018_idioms`/clippy warnings from the new `pub(super)` items (`private_interfaces`) are expected to be absent (closure over signatures), but only a real build shows them.
- rustfmt was run in `--check` mode only; I did not reformat anything (the baseline is not clean).
- The Go twins, the module `README.md` files and the taxonomy were not touched (section 7).

## 6. Real-check commands for the compile queue

All six crates are members of the root workspace (`Cargo.toml`), so no `--manifest-path` is needed. Hold the compile mutex for one batched call:

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-repo-tree -p semio-framework-repo-test-runner -p semio-framework-repo-hooks -p semio-framework-repo-mcp -p semio-framework-repo-providers -p semio-framework-repo-languages --lib --keep-going --message-format=short
```

Then the same with `--tests` instead of `--lib`, and once with the direct dependents added (`-p semio-framework-repo-cli -p semio-framework-repo-cli-verbs -p semio-framework-repo-cli-tree-source-and-search -p semio-framework-repo-cli-event-stream -p semio-framework-repo-cli-mcp-verb -p semio-framework-repo-dashboard-repo-domain -p semio-framework-repo-tickets -p semio-framework-repo-graphql-fs -p semio-framework-repo-statutes -p semio-framework-repo-codebase -p semio-framework-repo-move`). The unit tests run through `bun ./📜️script.ts test` in each package dir, or `... -- test -p <crate>` with the same prefix.

## 7. Hand-offs

- **Taxonomy sweep**: the 40 new folders are not registered (same as the repo-cli and graphql carves): under `🌳️tree/📦️packages/🦀️rust/`: `🧱️tree-model`, `🩻️monorepo-tree`, `🎯️goal-tree`, `⚖️statute-tree`, `🧹️sort-filter-search`, `🎨️rendering`, `📌️cache`; `🏃️test-runner`: `🧮️paths-and-identity`, `🌲️snapshot-and-scope`, `🗺️planning`, `⚙️execution-and-parsing`, `🔎️resolution`, `💽️real-world`; `🪝️hooks`: `🎯️hook-model`, `🧭️payload-reading`, `🛡️tool-policy`, `🗺️plan-steps`, `🔶️dispatch-and-formatting`, `📓️session-logging`, `💽️system-ports`; `🔌️mcp`: `🔏️digest-and-json`, `📨️protocol`, `📡️event-log`, `🔐️session`, `🚦️routing`, `🗄️repository`, `🚀️entrypoint`; `🧩️providers`: `📐️provider-model`, `🏃️process-runner`, `🔭️provider-interfaces`, `🐙️github-management`, `🌿️version-control-and-sandbox`, `🎆️editor-providers`, `🎖️registry-and-git-hooks`; `🗣️languages`: `🪪️identity-pending`, `🗂️language-table`, `🧩️pattern-matcher`, `📑️section-parsing`, `🧷️definition-parsing`, `🧭️scopes-and-headers`.
- **Parent (`📚️library`)**: no edit needed, see section 3.
- Foreign errors: none observed (no real compile ran).

## 8. Files

Edited: the six `P/🦀️.rs` roots. Created: the 40 children above, each `P/<folder>/🦀️.rs`. Tools: `rcr-tools/` (`rcrlib.py`, `spec.py` = the split specification, `snapshot.py`, `split.py`, `install.py`, `oracle.py`, `verify.py`, `verify_fmt_emoji.py`, `defscan.py`, `apicheck.py`, `dupmounts.py`, `outline.py`, `regions.py`, `emoji_check.py`, `orig/*.rs.txt`). Regenerate: `python3 split.py all` then `RCR_STAGE=1 python3 oracle.py all x` (stage under `🗑️generated/rcr-work/<name>/stage`, never the repo), `python3 install.py all` (refuses when a live root differs from the original).
