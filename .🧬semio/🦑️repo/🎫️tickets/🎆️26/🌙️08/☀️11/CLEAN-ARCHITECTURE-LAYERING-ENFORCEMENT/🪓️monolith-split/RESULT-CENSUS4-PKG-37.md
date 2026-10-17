# Result CENSUS4-PKG-37 (prefix `c4p37`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `M` = `🧰️framework/🔨️modules`. Tools: `F/c4p37-tools/` (descendant of `c4p35-tools`). Logs: `F/🗑️generated/c4p37-*` (nothing deleted). Stable sandbox/stage root (survives a scratchpad wipe): `/Users/ueli/.cache/c4p37/` (`stage/`, `sbx/`, `orc/`, `pos-work/`).

## 0. Verdict

**DONE (static evidence; real compile not admitted).** All 8 files are split into 37 cohesive children in 37 new folders (largest child 965 lines, parents 34 to 425 lines). Lossless 8 of 8 on the live tree, regions balanced per file (45 of 45 files), rustfmt parses 49 of 49 staged files, every child mounted exactly once, taxonomy 37 of 37, RGI emoji, sibling emoji unique, path length max 118 bytes. The scoped patched differential on the live tree is identical in every group and configuration (native, `--test`, wasm32-wasip2 lib/test, and for the editor also wasm32-unknown-unknown lib/test). The census keeps all eight crates inside the source gate, lib and `cfg(test)` (the one binding case is `semio-framework-value-intrinsic`, test closure exactly 32 of 32 files, which is why its codec file got one child). Five source-text readers were re-pointed to module-tree reads. No `COMPILE ADMITTED` line existed in `📓️FLEET.md` at 09:1x, so the real `bun repo cargo` check is queued (command in section 8).

## 1. Found state (fresh slice)

- No earlier c4p37 work: no `c4p37-tools`, no `🗑️generated/c4p37-*`, no RESULT file.
- The 8 files were byte-identical to `HEAD b84bf526c7b` (git blob hashes equal) and have the census line counts (1,090 + 1,073 + 1,583 + 1,119 + 1,373 + 1,006 + 1,395 + 1,074 = 9,713). Pre-edit snapshots: `F/c4p37-tools/orig/<tag>.rs`, hashes in `F/c4p37-tools/orig/SHA256`.

| tag | file (tail under `M/`) | lines | SHA256 (first 16) | owning crate | mount chain |
| --- | --- | ---: | --- | --- | --- |
| tt | `⏪️time-travel/🦀️.rs` | 1,090 | `64e045d6ee7c90bd` | `semio-framework-time-travel` | `mod component` of the glue root `📦️packages/🦀️rust/🦀️.rs` |
| trc | `⏱️trace/🦀️.rs` | 1,073 | `96af319a38717f6b` | `semio-framework-trace` | `mod component` of the glue root |
| edt | `✍️editor/🦀️.rs` | 1,583 | `f8f13fd97290a001` | `semio-framework-editor` | `mod component` of the glue root |
| abi | `🌉️abi/🦀️.rs` | 1,119 | `6fcd4d8ece87e47` | `semio-framework-abi` | the file is the lib root (`[lib] path = "../../🦀️.rs"`) |
| cod | `🌱️value/🔁️codec/🦀️.rs` | 1,373 | `5233efc45731096a` | `semio-framework-value-intrinsic` | `mod codec` of `🌱️value/🌳️intrinsic/🦀️.rs`, itself `pub mod value` of the glue root |
| omap | `🌱️value/🧬️retained-clone/🗺️ordered-map/🦀️.rs` | 1,006 | `4040796f50185f51` | `semio-framework-value-retained-map` | `pub mod ordered_map` of the glue root |
| dec | `🎒️pack/🌱️value/🛬️decode/🦀️.rs` | 1,395 | `7021224686dc2a6c` | `semio-framework-pack-record-retained` | the file is the lib root |
| ret | `🎒️pack/📐️format/🎮️retained/🦀️.rs` | 1,074 | `eb35574e2bc4fbcd` | `semio-framework-pack-format-retained` | the file is the lib root |

None of the 8 files holds a `#[test]` function (their tests live in mounted `🧪️tests/**` files that stay mounted from the parent), so no test name can move.

## 2. What was cut

Children sit next to their parent as `<emoji>️<kebab>/🦀️.rs`, wired with `#[path = "<folder>/🦀️.rs"] mod <snake>;` plus `pub use <snake>::*;` (or a plain `use` when the child has only private items; none when it holds only impls). Each child starts with an `//!` docstring whose first character is the folder emoji, then `use super::*;` (omitted only for the constants-only time-travel child, where it would be unused). Mounts of existing modules (`#[path]` to `🧪️tests`, `♻️retirement`, `🎮️decision`, `⏱️clock`, ...) stay in the parent because a `#[path]` is relative to the declaring file.

| tag | before | parent after | children (lines) |
| --- | ---: | ---: | --- |
| tt | 1,090 | 68 | 🔖️limits-and-codes 65, 🪪️identity-and-session 151, 📨️events 175, 🎬️effects-and-refusals 125, ⚙️reducer 340, 🌍️labels 206 |
| trc | 1,073 | 114 | 🕰️clock 58, 📏️ceilings-and-diagnostics 119, 📊️sample-rings-and-sites 173, 🐕️timers-and-watchdog 209, 📒️overrun-ledger 111, 🧵️thread-roles-and-counters 178, 🛰️operations-and-trace 176 |
| edt | 1,583 | 74 | 🎨️canvas-theme 75, 📐️viewport-and-settings-json 147, 🖊️host-state 127, ⌨️host-impl 780, 🔤️text-geometry 99, 🌐️wasm-session 336 |
| abi | 1,119 | 34 | 🪪️limits-and-identifiers 193, 📨️status-and-messages 187, 🧱️message-codec 232, ⏳️page-transfer 313, 🗃️ledgers-and-port 187 |
| cod | 1,373 | 425 | 📦️container-codecs 965 |
| omap | 1,006 | 44 | 🗂️map-core 131, 🔁️clone-cursor 306, 🔢️bounded-ord 165, 🔎️lookup-cursor 83, ➕️insert-cursor 320 |
| dec | 1,395 | 42 | 🧩️value-tokens-and-parsers 170, 🧭️value-cursor 613, 📜️record-body-cursor 596 |
| ret | 1,074 | 36 | 📄️pages-and-source-state 109, 🚰️source-cursor 220, 🧱️segment-and-varint-cursors 498, ⚓️catalog-and-anchor-cursors 244 |

Specific handling (each verified in the staged and live text):

- **Regions.** A region that spans several children gets a balanced marker pair in each child that holds a part (generated from the item's region stack); the parent keeps the markers of regions that still have content (trace `GuestMemory`, `SpanRecord`, `Watchdog`, `Tests`; editor `Wasm`, `Tests`) and the splitter prunes empty ones. Original markers 8/15/5/6/5/0/1/1 became 8/16/9/7/5/0/3/4 pairs.
- **`include_str!` re-based.** `abi`: `include_str!("🧬️schema/🔣️.json")` became `include_str!("../🧬️schema/🔣️.json")` in `🪪️limits-and-identifiers`.
- **`super::` re-based (omap).** `ordered_map` sits one level below the glue root and uses `super::RetainedCloneBorrowAuthority`, `super::RetainedCloneSource`, `super::RetainedCloneSourceTake` (glue-root names). In the children these became `super::super::...` (10 occurrences in 3 children: insert-cursor 6, lookup-cursor 2, bounded-ord 2; the existing `➖️remove` module keeps its own `super::super::` untouched, because its parent module is still `ordered_map`); the parent's own `use super::{...}` is untouched. A scan shows no remaining non-glob `super::` in any child except the declared `super::super::`.
- **Editor browser session.** `🌐️wasm-session` is mounted with the same `#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]` as its items (no empty module and no unused glob on native); the five cfg-gated `use` lines stay in the parent.
- **Macros.** `cod` and `omap` keep every `macro_rules!` with its invocation in one child (macro scope is textual).
- **Derive helper.** `#[value(with = "color_bridge")]` and the inline module `color_bridge` stay together in `🎨️canvas-theme`.
- **Widening.** 3 + 45 + 98 + 48 + 0 + 44 + 49 + 38 = 325 `pub(super)` insertions (items, fields, inherent members) where the name occurs in another file of the crate closure (lexical `widen2.py`, conservative). Trait-impl methods are never widened. No qualified path was introduced to fix visibility.
- **`cod` is one child on purpose.** The first cut had five children; the canonical counter showed `semio-framework-value-intrinsic` `cfg(test)` closure going from 31 to 36 files (gate 32). That cut was rolled back (own files only: five child folders removed, original restored from the snapshot) and replaced by parent (traits, scalars) plus one child with the whole `Containers` region, giving exactly 32 files.

## 3. Source-text readers (re-pointed)

`git grep -F` over the full repo for each file path and for the module directory names (`*.ts`, `*.tsx`, `*.py`, `*.json`, `*.rs`, excluding `.vscode` and `.🧬semio`). Readers of moved text, all fixed:

| reader | before | after |
| --- | --- | --- |
| `M/🧵️job/⏱️budget/🧪️tests/🔬️tool-job-microsecond-budget/🟦️.ts` | `readFileSync(join(base, "../../⏱️trace/🦀️.rs"))` | `readModuleTreeSource(join(base, "../../⏱️trace/🦀️.rs"))` plus import |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⏱️wgpu-ui-turn-budget/🟦️.ts` | `readFileSync(TRACE_RS, "utf8")` (the `SUSTAINED_OVERRUN_QUARANTINE_STEPS` regex) | `readModuleTreeSource(TRACE_RS)`; the now unused `readFileSync` import removed |
| `🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts` (line 340) | `policyReadFileSafe(root, ".../⏱️trace/🦀️.rs")` | `policyReadModuleTreeSource(root, ...)` |
| `🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🟦️.ts` (line 1119) | `policyReadFileSafe(this.root, ".../✍️editor/🦀️.rs")` | `policyReadModuleTreeSource(this.root, ...)` (same call already uses it for its neighbours) |
| `...renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-engine-surface-lifetime/🟦️.ts` (line 21) | `read(".../✍️editor/🦀️.rs")` | `policyReadModuleTreeSource(repoRoot, ".../✍️editor/🦀️.rs")` plus import |

Registering the editor root in `POLICY_SPLIT_SOURCE_ROOTS` was tried and rejected: `policyReadRustSourceEvidence` then throws `conditional external module tests is not canonical` for the existing `✍️editor/⚠️error/🧪️tests/🦀️.rs` mount. The registry is unchanged by this slice.

Probes (bun, repo readers on the live tree):
- Microsecond worker law needles on the trace text: `default_clock_us -> None` regex and `elapsed().as_micros()).ok()` are true for the orig snapshot and for the module-tree text (`F/c4p37-tools/probe-micro.ts`). The whole law returns false for the orig text too (the plugin/job sources moved on), so that is not caused by this slice.
- Editor interactivity law: `pub struct EditorHostRetirement` precedes `impl Default for EditorHost`, and the four needles of the `editorRetirement` slice (`let EditorHost {`, `self.released\n            && self.text.is_empty()`, `self.semantic_tokens.is_empty()`, `self.diagnostics.is_empty()`) are all true on orig and on the module-tree text (1,584 vs 1,755 lines).
- `readModuleTreeSource` / `listModuleTree` resolve all 8 trees (trace 12 files, editor 9, time-travel 10, abi 6, codec 9, ordered-map 9, decode 6, retained 5).

Other hits that are not readers: docstrings and `@see` lines, the `binary-gate.json` dep-info parser example (`🌉️abi/🦀️.rs` is only sample text), `⌨️text-input/🔣️.json` provenance text (names the editor file), the `🎒️pack/⚠️error` cause fixture (`source.path`/`sha256`/`predicate`, no code reads it; its sha256 `9784b967...` was already different from the file's `eb35574e...` before this slice), and the many `⏪️time-travel` matches that belong to the plugin directory `🔌️plugin/⏪️time-travel`, not to this module.

No compatibility re-exports: parents hold `#[path] mod x; <vis> use x::*;` wiring and the file's genuine top-level items; no alias, deprecation or shim was added.

## 4. Verification (commands, real results)

| Check | Command | Result |
| --- | --- | --- |
| Lossless line multiset (live) | `python3 c4p37-tools/live.py tt trc edt abi cod omap dec ret` (`c4p37-live.log`) | 8 of 8 `bytes-equal-to-stage` and `LOSSLESS OK`, missing 0; every added line is a docstring, `use super::*;`, `#[path]`/`mod`/`pub use` wiring, a region marker or a declared cfg line |
| Live structure | `python3 c4p37-tools/check.py ...` (`c4p37-check-live.log`) | `CHECKS OK`: every child mounted exactly once (0 duplicates, 0 missing), doc emoji equals folder emoji, unique among siblings and different from the root's, region counts equal per file, no `[DEBUG]`, longest child 965 lines, longest repo-relative path 118 bytes |
| Region stack | inline Python over the 45 live files | 0 mismatched or unclosed markers |
| rustfmt | `rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --emit stdout <file>` over the 49 staged files before install | 0 `error` lines (a broken control file reports one); `cod` re-run after its re-cut: 0 |
| Private declarations | `python3 c4p37-tools/defscan.py <tag>` on the staged tree | 0 cross-file leftovers for tt, trc, edt, abi, dec, ret; for omap and cod the scan reports trait-impl methods written at column 0 (false positives: `to_value_controlled`, `advance_retirement_demands`, ...; `widened 0` for the cod child) |
| Child module name clashes | `python3 c4p37-tools/modclash.py ...` | `modclash OK` |
| Test names | `python3 c4p37-tools/testnames.py ...` | 0 = 0 `#[test]` functions for every file |
| Scoped patched differential, staged (before install) | `python3 c4p37-tools/oracle.py <tag> lib,test,p2,p2test[,wasm,wasmtest]` (`c4p37-oracle-<tag>-stage.log`) | identical error and warning counts in every group; the first runs showed three textual-only differences (below) |
| Scoped patched differential, live (final) | `C4P37_LIVE=1 C4P37_NORM=1 python3 c4p37-tools/oracle.py <tag> ...` (`c4p37-oracle-<tag>-live.log`) | `ORACLE <tag> CLEAN` for all 8 groups, 0 only-orig and 0 only-new, warnings identical |
| Taxonomy | `bun c4p37-tools/tax.ts c4p37-tools/newdirs.txt` | `dirs 37, unresolved []` |
| RGI / siblings / reuse | `node c4p5-tools/rgi.mjs newnames.txt`; sibling and git-tracked prefix scan | `bad=0`; no clash with existing sibling folders; every emoji already prefixes other folders |
| Census | `zsh c4p37-tools/census.sh after` (canonical, lib) and `bun c4p37-tools/census-own.ts <root>` on the live tree and on the oracle sandboxes `orig` / `new` (lib and `cfg(test)`) | see the table below; 0 of 8 over the gate in lib; `cfg(test)` over the gate only for time-travel bytes, which was over before |

Differential detail. The oracle runs `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null` from the real crate lib root against a sandbox holding the ORIGINAL file, with the `extern crate` and external-glob patches applied identically to both sides, and compares error and warning multisets without path, line and column. Observations that matter:
- The groups `tt`, `edt`, `cod`, `omap`, `dec`, `ret` stop at name resolution (they have unresolved external crates in both trees: 29 to 657 errors per configuration), so privacy and type errors are not reached there; `defscan` and the conservative lexical widener cover that gap, and section 5 adds the full-crate typeck oracle.
- `trc` has no external dependency: its lib configuration reaches the end of analysis (the only error is rustc's `couldn't create a temp dir` at `/dev/rmeta...` for `-o /dev/null`, normalised), with identical warnings (2). That is a full privacy and type pass on the split tree.
- `abi` stops at its single external import (`semio_framework_value`), identical in both trees.
- Three textual differences in the staged runs, all with equal counts and codes, and normalised in the final runs (`C4P37_NORM=1`, which only rewrites printed module paths): the editor's `E0599 from_value_controlled` is printed with the type path `component::scene::EditorScene` instead of `EditorScene`; the ordered map prints `bounded_ord::BoundedOrd` instead of `ordered_map::BoundedOrd` and `module super::super` instead of `super` for the stubbed glue-root name. Two real catches during staging: `cod` first showed `unused import: scalar_codecs::*` (fixed with `reexport=False` for a child with only impls) and `tt` showed `unused import: super::*` for the constants-only child (fixed with `glob=False`).
- One oracle tool defect fixed along the way: `couldn't read` errors of `include_str!` files are now pulled into the sandbox as well (the trace tests read three JSON files and `.cargo/config.toml`).

Census (canonical `readRustSourceClosureV1`; gate 4,000 lines, 32 files, 262,144 bytes). Own-closure rows from `census-own.ts`, before = oracle sandbox `orig`, after = live; the canonical `census.sh after` agrees with the lib rows (files/lines) and adds `worstRatio`:

| crate | lib before | lib after | lib + cfg(test) before | lib + cfg(test) after | canonical worst ratio (lib) |
| --- | --- | --- | --- | --- | --- |
| time-travel | 5 / 73,053 / 1,307 | 11 / 74,694 / 1,353 | 9 / 470,603 / 1,957 | 15 / 472,244 / 2,003 | 0.344 (files) |
| trace | 6 / 91,682 / 2,007 | 13 / 94,361 / 2,079 | 22 / 159,060 / 2,957 | 29 / 161,739 / 3,029 | 0.520 |
| editor | 4 / 64,361 / 1,367 | 9 / 54,776 / 1,411 | 9 / 119,915 / 2,385 | 14 / 110,330 / 2,429 | 0.353 |
| abi | 2 / 45,149 / 1,118 | 7 / 47,109 / 1,150 | 5 / 64,349 / 1,466 | 10 / 66,309 / 1,498 | 0.287 |
| value-intrinsic (cod) | 15 / 157,986 / 2,622 | 16 / 158,380 / 2,640 | 31 / 253,969 / 3,724 | **32** / 254,363 / 3,742 | 0.671 |
| value-retained-map | 5 / 89,890 / 1,161 | 10 / 91,871 / 1,209 | 12 / 140,539 / 1,733 | 17 / 142,520 / 1,781 | 0.350 (bytes) |
| pack-record-retained | 4 / 125,849 / 1,675 | 7 / 127,439 / 1,704 | 13 / 190,527 / 2,290 | 16 / 192,117 / 2,319 | 0.486 (bytes) |
| pack-format-retained | 1 / 53,712 / 987 | 5 / 55,578 / 1,024 | 5 / 101,521 / 1,325 | 9 / 103,387 / 1,362 | 0.256 |

(files / bytes / active lines.) The editor lib bytes shrink because the browser session is `cfg`-gated out of the native closure (the wasm32 closure adds its 336 lines). The time-travel `cfg(test)` closure is over the byte gate before and after (470,603 and 472,244 bytes: its test fixtures), unchanged by the split in kind. Nothing else crosses a limit; the only limit-touching row is `value-intrinsic` `cfg(test)` at 32 of 32 files.

## 5. Full-crate typeck oracle (extra evidence)

`pos.py` (privacy and type oracle, stub layer for everything outside the family; copy in `F/c4p37-tools/pos-tools/` with `SCRATCH` moved to `/Users/ueli/.cache/c4p37`, family `families/c4p37.json` = the 8 crates, base `b84bf526c7b`, cfgs lib and test). Command: `cd F/c4p37-tools/pos-tools && ulimit -n 65536 && POS_PREFIX=c4p37 python3 pos.py run families/c4p37.json --cfgs lib,test --label a3 --iters 3` (log `c4p37-pos-a3.log`, about 17 minutes; two earlier attempts `a1`, `a2` were cut by my own 580 s alarm after iteration 2 and wrote no report). Current tree (all 8 splits installed, `cod` already in its final one-child form) against `HEAD`:

| cfg | errors cur / base | NEW / GONE | privacy | trait-scope | resolve | type | other | canary |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| lib | 2670 / 2670 | 9 / 9 (all stub-layer rows) | 0 | 0 | 0 | 0 | 0 | `E0308 E0616 E0624` reached on both sides |
| test | 4598 / 4598 | 11 / 11 (all stub-layer rows) | 0 | 0 | 0 | 0 | 0 | reached on both sides |

0 new non-stub rows in every category, so no missing `pub(super)` (`E0603`, `E0616`, `E0624`) and no unresolved name is introduced; the canary proves the oracle reaches type and privacy checking. The HIR cull rewrote 2 unresolved trait references on both sides identically (in `🗂️ordered/🌐️wire`, not a file of this slice). Not covered: the wasm32-unknown-unknown editor session (covered by the scoped differential only), borrowck, trait coherence against the real externals, and lints; the real check in section 8 covers them.

## 6. Foreign errors and observations

- `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` (library module): 1 pass, 2 fail, all three offenders are plugin crates of peers (`🗄️stdio/🗿️artifacts/📸️jpg` grew 527 to 551, `🔋️energy/.../🧬️schema` grew 165 to 214, `🗜️deflate/🦀️.rs` is a stale pin). The law does not look at the framework modules of this slice, so it was not changed by it.
- `toolJobMicrosecondBudgetSelfTests()` throws `microsecond exact worker binding: none` on the live tree and `toolJobMicrosecondWorkerExact` returns false for the original trace text too (probe above): the plugin and job sources moved on, unrelated to the trace split.
- `interactivityMountedEngineSurfaceLifetimeSelfTests` cannot complete on the live tree: `policyReadRustSourceEvidence` throws `conditional external module scene_retained_tests is not canonical` for `🗺️surface/🕸️node-graph/📡️scene/🧪️tests/🦀️.rs` (the node-graph source, not the editor). I proved the editor half with the direct probe in section 3.
- The tool-job coverage `🏁️run/🟦️.ts` and the source-access registry carry unrelated working-tree edits of peers; my edit to the first is one expression.
- `git` printed `daemon terminated` during some read-only calls (fsmonitor noise). No `ENFILE`/`EMFILE` or Bun segfault occurred in any run (`ulimit -n 65536` set), so no retry was needed.

## 7. Files

- Edited (8 roots): `M/⏪️time-travel/🦀️.rs`, `M/⏱️trace/🦀️.rs`, `M/✍️editor/🦀️.rs`, `M/🌉️abi/🦀️.rs`, `M/🌱️value/🔁️codec/🦀️.rs`, `M/🌱️value/🧬️retained-clone/🗺️ordered-map/🦀️.rs`, `M/🎒️pack/🌱️value/🛬️decode/🦀️.rs`, `M/🎒️pack/📐️format/🎮️retained/🦀️.rs`.
- Created (37 children, one `🦀️.rs` per new folder): the folders of section 2, next to their roots.
- Edited outside the package (readers, section 3): the five TypeScript files listed there.
- No registry, manifest, `launch.json`, lock or build-owner edit was necessary: no crate, test target or command was added or renamed.
- Tools, kept: `F/c4p37-tools/` (`c4p37.py` splitter with `rebase_super`, `cfg_mod` and `reexport` options, `specs/*.py`, `oracle.py`, `live.py`, `check.py`, `defscan.py`, `modclash.py`, `globuse.py`, `testnames.py`, `census.sh`, `census-own.ts`, `tax.ts`, `probe-micro.ts`, `pos-tools/` copy with the family `families/c4p37.json`, `orig/` snapshots, `newdirs.txt`, `newnames.txt`). Do not re-stage after install: the specs compute their `external` lists from the live parents, which now mount the children.
- Logs, kept in `F/🗑️generated/`: `c4p37-oracle-<tag>-{stage,live}.log`, `c4p37-live.log`, `c4p37-check-live.log`, `c4p37-census-*.log`, `c4p37-census-own-*.log`, `c4p37-tsrun-*.log`, `c4p37-defscan-live.log`, `c4p37-install.log`, `c4p37-pos-*.log`.

## 8. Compile queue (not admitted)

No `COMPILE ADMITTED` line in `📓️FLEET.md` at 09:1x (the mutex folder `🗑️generated/cargo-mutex.lock` was held by `cad`). Exact commands for the compile queue, one batched call under the mutex of `📓️WAVE5-BRIEF.md` (framework workspace members need the manifest path):

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-time-travel -p semio-framework-trace -p semio-framework-editor -p semio-framework-abi -p semio-framework-value-intrinsic -p semio-framework-value-retained-map -p semio-framework-pack-record-retained -p semio-framework-pack-format-retained --lib --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
```

then `--target wasm32-wasip2` for the same set (`--lib`) and, for the browser session of the editor, `-p semio-framework-editor --lib --tests --target wasm32-unknown-unknown`. If a build shows a missing `pub(super)`, widen that single definition line in the child (never with a qualified path).
