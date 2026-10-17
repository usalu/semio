# Result CENSUS4-PKG-48 (prefix `c4p48`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. Tools: `F/c4p48-tools/` (descendant of `c4p44-tools`). Logs: `F/🗑️generated/c4p48-*`. Sandboxes: `/Users/ueli/.cache/c4p48/`.

## 0. Verdict

**DONE** (static; real compile queued). All six files are split into 70 cohesive children in 70 new folders (`#[path]` modules, largest child 250 lines, roots 57 to 102 lines), installed 15:19 and re-checked on the live tree: losslessness exact 6 of 6 (missing 0, every extra line a docstring, `use super::*;`, `#[path]`/`mod`/`use` wiring, region marker or declared impl header/brace line), region markers balanced per file (the unsplit `🗂️codebase` root had one stray `//#endregion`, repaired by one declared `//#region`), rustfmt parse clean, taxonomy 70 of 70 resolved, RGI 70 of 70, every folder mounted exactly once, sibling emoji unique and different from the root's, longest repo-relative path 167 UTF-8 bytes, census 0 of 6 over the gate (lib and test), public API item sets equal (6 of 6), scoped patched differential **identical** (lib and `--test`: 12 of 12 comparisons, 0 differences, warnings identical), full-crate typeck oracle: 0 privacy, 0 trait-scope, 0 other non-stub rows before and after (row delta is stub-layer emulation only, section 5). Two source-text readers were re-pointed to the module tree (section 3). No `COMPILE ADMITTED` line in `📓️FLEET.md` (checked 15:26), so the launcher check is queued (section 8).

## 1. Found state (fresh slice)

No earlier c4p48 work existed. Six files re-measured equal to the census (8,404 lines), snapshots in `F/c4p48-tools/orig/` with hashes in `orig/SHA256`:

| tag | file (tail under `🧰️framework/🛍️products/🦑️repo/🔨️modules/`) | lines | SHA256 (first 16) | owning crate |
| --- | --- | ---: | --- | --- |
| reg | `🎛️dashboard/🎮️registry/🦀️.rs` | 1,385 | `2dccd1b3f0f39c43` | `semio-framework-repo-dashboard-registry` |
| gol | `🎯️goals/📦️packages/🦀️rust/🦀️.rs` | 1,195 | `1cfd84cbe647b09b` | `semio-framework-repo-goals` |
| rsc | `🔗️graphql/🏗️repo-schema/🦀️.rs` | 1,264 | `4a8ea78ed4916f0c` | `semio-framework-repo-graphql-schema` |
| fsc | `🔗️graphql/🗄️fs-context/🦀️.rs` | 1,100 | `edc03848b5a311f9` | `semio-framework-repo-graphql-fs` |
| crd | `🖥️server/🎛️coordinator/📦️packages/🦀️rust/🦀️.rs` | 2,227 | `aecf92347d462ca4` | `semio-framework-repo-coordinator` |
| cbs | `🗂️codebase/📦️packages/🦀️rust/🦀️.rs` | 1,233 | `3b023f53669f6a86` | `semio-framework-repo-codebase` |

## 2. What was cut

Splitter: `F/c4p48-tools/c4p48.py` (descendant of the `c4p44` splitter), specs in `F/c4p48-tools/specs/`. Children are `<emoji>️<kebab>/🦀️.rs` next to their root, wired in the parent with `#[path = "<folder>/🦀️.rs"] mod <snake>;` and `pub use <snake>::*;` (or `pub(crate) use` / plain `use` by the best visibility of the child's named items; no re-export for a child that holds only `impl` blocks). Each child starts with an `//!` docstring whose first character is the folder emoji, then `use super::*;` (omitted for `🪪️command-identity` of the registry and `⏱️config` of the coordinator, which use no name of their parent; the glob was an unused import in the first oracle run). Large inherent and trait `impl` blocks are split by member (`split_impls_h`): the impl header is repeated verbatim for each group of members, the closing brace and header lines are the declared `pre_extra` of the lossless check. Region markers that sat between two members inside an impl are hoisted out of the impl, so every moved item is region-balanced.

| tag | File | Before | After: root + children (max child) | Children (lines) |
| --- | --- | ---: | --- | --- |
| reg | `🎛️dashboard/🎮️registry` | 1,385 | 76 + 12 (214) | `🪪️command-identity` 186, `🧾️declarations` 105, `📋️facts` 88, `🧬️model` 118, `⚖️run-policy` 69, `🛂️tokens-and-validation` 161, `📚️registry-record` 44, `🧶️build-and-derive` 214, `🔍️lookup-and-search` 102, `🚀️resolve-and-launch` 211, `🩺️check-and-json` 74, `🪶️entry-draft` 51 |
| gol | `🎯️goals/📦️packages/🦀️rust` | 1,195 | 70 + 11 (187) | `🚨️errors` 67, `🪪️id-scheme` 103, `🗄️store-and-codec` 187, `🧩️management-and-emission` 170, `🏛️goals-aggregate` 44, `✨️goal-create` 99, `✏️goal-change-and-close` 78, `🔄️goal-reopen-delete-and-sync` 117, `🌱️tree-seeds` 96, `🌳️tree-build` 147, `🖨️tree-render` 91 |
| rsc | `🔗️graphql/🏗️repo-schema` | 1,264 | 57 + 8 (250) | `🗺️type-vocabulary` 209, `🧱️common-schemas` 80, `🗃️repo-and-codebase-schemas` 140, `📜️section-and-statute-schemas` 123, `🤝️interaction-goal-and-todo-schemas` 102, `🎫️ticket-and-contributor-schemas` 140, `⌨️inputs-and-operations` 250, `🧾️build-and-sdl` 177 |
| fsc | `🔗️graphql/🗄️fs-context` | 1,100 | 67 + 10 (214) | `📡️emitter-and-management` 61, `🧭️analyze-scope` 59, `🏠️context-core` 91, `🎯️goal-and-file-support` 152, `🔧️refactor-operations` 65, `🔎️context-queries` 149, `✏️goal-todo-and-draft-mutations` 56, `🎫️ticket-mutations` 150, `🧱️contributor-and-structure-mutations` 128, `🔄️management-sync` 214 |
| crd | `🖥️server/🎛️coordinator/📦️packages/🦀️rust` | 2,227 | 102 + 19 (229) | `⏱️config` 56, `🔢️event-encoding` 202, `🛡️durability` 84, `🗄️event-store` 193, `🔐️store-append` 229, `♻️store-recovery` 151, `🧾️projection-records` 160, `🧠️projection` 89, `📚️repository` 81, `✍️repository-records` 59, `🔭️repository-projections` 116, `📨️http-messages` 65, `🌐️http-server-core` 62, `🎫️ticket-routes` 105, `📥️diff-ingest` 178, `📡️event-routes` 63, `🧵️service-loop` 162, `🪝️webhooks` 167, `🚀️clock-ids-and-main` 54 |
| cbs | `🗂️codebase/📦️packages/🦀️rust` | 1,233 | 91 + 10 (216) | `🙈️generated-folders-and-bundle-labels` 38, `🏠️codebase-handle` 46, `🏗️technology-detection` 216, `🚫️ignore-rules` 81, `🚶️walk` 80, `🪪️artifact-ids` 137, `🧭️scope-and-helpers` 167, `📚️codebase-context` 70, `🧱️bundle-folder-and-file-aggregates` 189, `🌳️section-definition-breach-and-tree-aggregates` 181 |

Totals: 8,404 lines in 6 files became 463 lines in 6 roots plus 70 children. No new crate, test target or command was added, so no manifest, build-owners row or launch route changed. The children sit next to the roots (for example `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🧶️build-and-derive/🦀️.rs`).

Specific handling (each verified in the staged and live text):
- **Pinned items stay in the parent.** The test mounts (`#[cfg(test)] #[path] mod tests;` in rsc, fsc and crd), the `mod publication` path mount with `pub use publication::graph_file;` of the registry (its relative `../../📚️library/…` path is unchanged because the parent did not move), the `pub use` re-exports and the imports stay where they were. Public paths are unchanged.
- **Regions.** Original pairs per file (reg, gol, rsc, fsc, crd, cbs): 9, 9, 24, 3, 11, 15 region and 9, 9, 24, 3, 11, 16 endregion markers; new: 15/15, 14/14, 25/25, 12/12, 21/21, 21/21. A region that spans several children gets a balanced marker pair in each child that holds a part.
- **cbs marker repair.** The unsplit file opened `📚️Context` once and closed it twice (line 863 inside the `CodebaseContext` impl and line 1224). The `📚️Context` end marker at 863 and the `🧱️Aggregates` pair, plus the `🪪️Artifact Ids` end marker, sat inside impl bodies. The `pre` step hoists them out of the impls and re-opens `📚️Context` once before the free functions `descend` and `find_section`, which the stray end marker at 1224 was closing. No marker line is lost; the one added line is `//#region 📚️Context` (declared, tag `marker` in the lossless log).
- **Widening.** Lexical, name-based `pub(super)` insertions for private items, fields and inherent members whose name occurs in another file of the split or in the in-crate test file (`widen2.py`, conservative): reg 156, gol 11, rsc 28, fsc 16, crd 92, cbs 11. Trait-impl methods are never widened. No original `pub(super)` or `pub(in …)` existed (the two `pub(crate)` items of rsc keep their meaning). No qualified path was used to fix visibility.
- **No `include_str!`/`include_bytes!`** exists in any of the six files, so no literal was re-based.
- **Module names.** Folder emoji are unique among siblings and differ from each root's docstring emoji and from the existing sibling folders (`📦️packages`, `🧪️tests`, the tests sub-folders); all are RGI and already used as folder prefixes (at least 2 occurrences, `dirs.py`).

## 3. Source-text readers

`git grep -F` over the repo (excluding tickets and markdown) for every file path and module path; plus the TS tests inside the six module trees.

| reader | before | after |
| --- | --- | --- |
| `🎛️dashboard/🧪️tests/♻️reuse/🟦️.ts` (line 67) | `readFileSync(join(root,"🎮️registry/🦀️.rs"),"utf8")`, then `definition(registry,"launch")` extracts `fn launch` | `readModuleTreeSource(join(root,"🎮️registry/🦀️.rs"))` with a new `module-tree` import beside the existing `rust` syntax import |
| `🎛️dashboard/🧪️tests/🎮️registry/🟦️.ts` (line 60) | `readFileSync(join(dashboard,file),"utf8")` over `🎮️registry/🦀️.rs` and `📚️inventory/🦀️.rs` for `graph_file` and the absence of `cache/build/graph.json` | `readModuleTreeSource(join(dashboard,file))` with the `module-tree` import |

Probe `c4p48-tools/probe-tree.ts` (log `c4p48-probe-tree.log`): the registry module tree has 13 files; the extracted definitions of `fn launch` (3,579 chars), `fn process` (14,506), `fn member` (500) and `fn parameters` (6,262) are **string-equal** to the ones from the unsplit snapshot; the tree of the registry and of the inventory holds `graph_file` and not `cache/build/graph.json`, as the unsplit text did. The two edited tests were not run (they spawn the `semio` binary or `rustc`; both files are also being edited by peers, see `git status`); the probe covers their read expressions.

Not readers: `📋️project.json` files (no source globs), `Cargo.toml` `[lib] path` entries (root file paths unchanged), `📋️build-owners.json`, `🔒️dependencies.json` (crate roots), `🧊️execution` fixtures (synthetic trees with their own `🎮️registry/🦀️.rs` text), `📚️library/…/📥️intent` fixtures (manifest paths), `📚️inventory/🦀️.rs` (mounts only `🎮️registry/🧪️tests/🔬️unit/🦀️.rs`, unchanged), docstring `@see` mentions in sibling dashboard modules. No compatibility re-export: the parents hold `#[path] mod x; <vis> use x::*;` wiring and genuinely own items only.

## 4. Verification (commands, real results)

Tools in `F/c4p48-tools/`, logs in `F/🗑️generated/c4p48-*`; all long runs used `ulimit -n 65536` and `nice -n 10`.

| Check | Command | Result |
| --- | --- | --- |
| Install precondition and apply | `python3 c4p48.py stage <tag>`; `python3 c4p48.py install <tag>` (six tags, `c4p48-install.log`) | each live file equalled its snapshot (the installer refuses otherwise); `installed reg gol rsc fsc crd cbs` |
| Lossless (live) | `python3 live.py reg gol rsc fsc crd cbs` (`c4p48-live.log`) | 6 of 6 `bytes-equal-to-stage` and `LOSSLESS OK`: missing 0 |
| Structure | `python3 check.py reg gol rsc fsc crd cbs` (`c4p48-check-live.log`) | `CHECKS OK`: every child mounted exactly once, doc emoji equals folder emoji, unique among siblings and different from the root's, region counts equal, no `[DEBUG]`, longest child 250 lines, longest path 167 bytes |
| Private declarations | `python3 defscan.py <tag>` (stage) | 0 cross-file leftovers for all six (11, 13, 2, 4, 27, 11 private declarations scanned); `python3 modclash.py …`: `modclash OK` |
| Public API | `python3 pubapi.py reg gol rsc fsc crd cbs` (`c4p48-pubapi.log`) | `PUBAPI OK`: pub item sets (40, 32, 13, 4, 36, 12) and pub method sets (25, 15, 5, 18, 30, 35) equal before and after; every child with pub items is `pub use`d |
| rustfmt | `rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --emit stdout` over the 76 staged files (`c4p48-rustfmt-stage.log`) | 0 `error` lines (a deliberate syntax error gives 1, so the probe works) |
| Taxonomy, RGI, siblings | `python3 dirs.py`; `bun rgi.mjs newnames.txt`; `bun tax.ts newdirs.txt` | `dirs 70, emoji 52`, no sibling clash, no rare emoji; `bad=0`; `dirs 70, unresolved []` |
| Scoped patched differential, staged | `python3 oracle.py all all` (`c4p48-oracle-stage1.log`, `-stage2.log`) | stage1: errors identical, 2 new warnings `unused import super::*` (reg, crd); both children set to `glob=False`; stage2: `ORACLE all CLEAN` |
| Scoped patched differential, live (final) | `C4P48_LIVE=1 C4P48_NORM=1 python3 oracle.py all all` (`c4p48-oracle-live.log`) | `ORACLE all CLEAN`: 12 of 12 comparisons (six crates, lib and `--test`), 0 only-orig, 0 only-new, warnings identical; per-crate error counts equal (reg 110, gol 37, rsc 2, fsc 121, crd 3, cbs 32) |
| Census (canonical `readRustSourceClosureV1`) | `bun census-own.ts <root>` (pre on the sandbox with the unsplit files, post on the live tree); `zsh census.sh post` (`c4p48-census-post.log`) | 0 of 6 over the gate; table below; the canonical `census.ts` rows equal the own rows |
| Wiring budget law | `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` (library module, `c4p48-wiring-budget.log`) | 2 pass, 1 fail; the ten breaches are plugin crates of peers under `✏️s/🔌️plugins`, none in `🦑️repo` |
| Registry readers | `bun probe-tree.ts` | see section 3 |

Census (files / bytes / active lines; ratio = worst of files/32, bytes/262,144, lines/4,000; before = pre run on the unsplit snapshots, after = live):

| crate | lib before | lib after | cfg(test) before | cfg(test) after |
| --- | --- | --- | --- | --- |
| `semio-framework-repo-dashboard-registry` | 6 / 168,075 / 1,463 | 18 / 173,160 / 1,588 (0.661) | same as lib | same as lib |
| `semio-framework-repo-goals` | 1 / 47,614 / 1,196 | 12 / 50,632 / 1,281 (0.375) | same as lib | same as lib |
| `semio-framework-repo-graphql-schema` | 1 / 52,411 / 1,263 | 9 / 54,904 / 1,285 (0.321) | 3 / 64,424 / 1,301 | 11 / 66,917 / 1,323 |
| `semio-framework-repo-graphql-fs` | 1 / 59,059 / 1,099 | 11 / 62,664 / 1,201 (0.344) | 3 / 60,345 / 1,128 | 13 / 63,950 / 1,230 |
| `semio-framework-repo-coordinator` | 1 / 92,887 / 2,220 | 20 / 99,224 / 2,389 (0.625) | 2 / 96,992 / 2,316 | 21 / 103,329 / 2,485 |
| `semio-framework-repo-codebase` | 1 / 54,874 / 1,234 | 11 / 57,864 / 1,307 (0.344) | same as lib | same as lib |

The binding limit is bytes for the registry (it includes the large `📚️library/🏗️build/🕸️graph` file through its `#[path]` mount), files for the others (at most 21 of 32); nothing crosses a limit.

## 5. Full-crate typeck oracle (extra evidence)

`posrun.py` (the privacy and type oracle `F/pos-tools/pos.py` with a stub layer for everything outside the family, sandbox `/Users/ueli/.cache/c4p48/pos-work`), family `families/c4p48.json` = the six crates, no base revision: `pre` ran on the unsplit live tree before the install, `post` after it. Commands: `cd F/c4p48-tools && ulimit -n 65536 && PYTHONHASHSEED=0 POS_PREFIX=c4p48 python3 posrun.py run families/c4p48.json --cfgs lib,test --label {pre,post} --no-base` (`c4p48-pos-pre.log`, `c4p48-pos-post.log`, about one minute each), then `python3 posdiff.py c4p48-pre c4p48-post lib test` (`c4p48-posdiff-1.log`).

| cfg | errors pre / post | privacy | trait-scope | resolve | type | other | canary |
| --- | --- | --- | --- | --- | --- | --- | --- |
| lib | 1,721 / 1,716 | 0 / 0 | 0 / 0 | 13 / 19 | 46 / 45 | 0 / 0 | `E0308 E0616 E0624` reached on both sides |
| test | 1,725 / 1,720 | 0 / 0 | 0 / 0 | 13 / 19 | 46 / 45 | 0 / 0 | reached |

No new `E0603`, `E0616`, `E0624` or unresolved name of the real code is introduced. The row delta (58 rows only in `pre`, 53 only in `post`, identical in lib and test) is stub-layer emulation, two classes: (1) 47 `E0407` rows on each side about the stubbed external trait `RepoContext`/`Tr` in `fs-context` and about `Deserialize` in the registry; the stub reports the trait-position consequences per file, and the `impl … for` sites now sit in children that reach the parent's `use` only through `use super::*` (same class as PKG-44); (2) 6 post-only `E0425` rows in `semio-framework-repo-goals` (`cannot find function … in module identity`, `cannot find type ModelError in module model`) against 11 pre-only `E0061`/`E0277`/`E0308`/`E0599` rows that type through the same stubbed calls. The alias names `identity` and `model` come from the parent's `use semio_framework_repo_identity as identity;` and `use semio_framework_repo_model as model;`; a rustc experiment (`use std::collections as col; mod child { use super::*; … col::HashMap … }`, analysis-only, output under `/Users/ueli/.cache/c4p48/exp`) compiles, so a real build resolves them through the glob import. Not covered: borrowck, trait coherence against the real externals, lints; the real check in section 8 covers them.

## 6. Foreign errors and observations

- `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts`: breaches only in plugin crates of peers (section 4).
- The plain scoped differential is blind below the unresolved external crates (reg 110, gol 37, fsc 121, cbs 32 unresolved-import errors in both trees); the stub-layer oracle (section 5) is the privacy evidence for those four, the plain differential is the deep evidence for rsc (2 errors) and crd (3 errors).
- `git` printed `error: daemon terminated` on read-only calls (fsmonitor noise). No `ENFILE`/`EMFILE` occurred.
- `♻️reuse/🟦️.ts` is a staged new file (`A`) and `🎮️registry/🟦️.ts` a staged modification of peers; my edits (one import and one expression each) are in the working tree only.

## 7. Files

- Edited (6 roots, under `🧰️framework/🛍️products/🦑️repo/🔨️modules/`): `🎛️dashboard/🎮️registry/🦀️.rs`, `🎯️goals/📦️packages/🦀️rust/🦀️.rs`, `🔗️graphql/🏗️repo-schema/🦀️.rs`, `🔗️graphql/🗄️fs-context/🦀️.rs`, `🖥️server/🎛️coordinator/📦️packages/🦀️rust/🦀️.rs`, `🗂️codebase/📦️packages/🦀️rust/🦀️.rs`.
- Created (70 children, one `🦀️.rs` per new folder): the folders of section 2, next to their roots.
- Edited outside the package (readers, section 3): `🎛️dashboard/🧪️tests/♻️reuse/🟦️.ts` and `🎛️dashboard/🧪️tests/🎮️registry/🟦️.ts`.
- Tools, kept: `F/c4p48-tools/` (`c4p48.py` splitter, `specs/*.py`, `oracle.py`, `live.py`, `check.py`, `defscan.py`, `modclash.py`, `globuse.py`, `pubapi.py`, `dirs.py`, `census.sh`, `census-own.ts`, `tax.ts`, `rgi.mjs`, `probe-tree.ts`, `posrun.py`, `posdiff.py`, `families/c4p48.json`, `newdirs.txt`, `newnames.txt`, `orig/` snapshots with `SHA256`). Do not re-stage after install: the specs read the original snapshots, but `install` refuses when the live file differs from them.
- Logs, kept in `F/🗑️generated/`: `c4p48-oracle-{stage1,stage2,live}.log`, `c4p48-install.log`, `c4p48-live.log`, `c4p48-check-live.log`, `c4p48-rustfmt-stage.log`, `c4p48-census-own-{pre,post}.log`, `c4p48-census-post.log`, `c4p48-pubapi.log`, `c4p48-probe-tree.log`, `c4p48-wiring-budget.log`, `c4p48-pos-{pre,post}.log`, `c4p48-posdiff-1.log` with the `c4p48-c4p48-*` diagnostic dumps.

## 8. Compile queue (not admitted)

No `COMPILE ADMITTED` line in `📓️FLEET.md` at 15:26 and no `bun repo cargo` call was made by this slice. The six crates are members of the root workspace (no `--manifest-path`). One batched call under the compile mutex of `📓️WAVE5-BRIEF.md`:

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-repo-dashboard-registry -p semio-framework-repo-goals -p semio-framework-repo-graphql-schema -p semio-framework-repo-graphql-fs -p semio-framework-repo-coordinator -p semio-framework-repo-codebase --lib --tests --keep-going --message-format=short
```

If a build shows a missing `pub(super)`, widen that single definition line in the child (never with a qualified path).
