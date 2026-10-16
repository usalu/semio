# Result TOOL-GRAPH-MACHINE-SPLITS (WP-F, prefix `tgm`)

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no git-modifying command. Brief: `📓️EXECUTOR-RULES.md`, `📓️WAVE5-BRIEF.md` (Wave 6), `📓️RESOLUTION-ORACLE.md`, `WAVE7-CANDIDATES.md` (WP-F). Tools: `tgm-tools/`. Logs and sandboxes: `🗑️generated/tgm-*` (nothing deleted). `M` = `🧰️framework/🔨️modules`.

## 0. Verdict

DONE for all six targets, static evidence only. No compiler ran (Wave 6: the launcher is refused at the headroom gate, so no `bun repo cargo` call was made). The six oversized roots are cut into 34 cohesive child files (largest child 983 lines, largest root 483), all public paths unchanged, no plugin file edited, six TS source-text readers re-pointed (three tool-machine, three tool-run). One plugin test reads the framework tool-run root as text and now needs a one-line re-point by its owner (section 9).

| gate | result |
| --- | --- |
| patched full-resolution differential, baseline snapshot vs live tree, 6 crates x lib / `--test` / `wasm32-unknown-unknown` / `wasm32-wasip2` / `--test` on both wasm targets (+ `lib+testing`, `test+testing` for layout-run) | **onlyNew 0, onlyOld 0 in every configuration** (section 4.1) |
| seeded sensitivity | each crate: a removed glob re-export or a re-privatised moved item is caught (E0425/E0422/E0432/E0603), section 4.2 |
| lossless line multiset (blank lines ignored, whitespace trimmed) | 0 unexplained removed, 0 unexplained added in all six; every added line is a header docstring, a `use super::*;`, a mount line or a `pub(crate)` widening (section 4.3) |
| `//#region` markers | identical multiset and balanced per file in all six |
| rustfmt (nightly 2026-07-20, `skip_children`) | all 40 changed or new files parse |
| `#[path]` / `include!` literals | all resolve, longest repo-relative path 103 bytes |
| duplicate mounts | 0 new (each new file is mounted exactly once) |
| public top-level item set (exact `pub`) before vs after | identical in all six (97, 66, 24, 36, 42, 33 items) |
| definition scan on the installed tree | 0 private crossings (2 intentional tuple-struct markers, section 4.4) |
| taxonomy (`frs-tools/taxcheck.ts`, real `matchDirectoryKind`) | 43 directories checked, 0 unresolved, 34 new ones all `crate-module-children` or `members-of-*` |

## 1. Targets confirmed

Cargo `[lib] path`, package stubs and mounts (checked in `Cargo.toml` and with `git grep -F '#[path'` over every `.rs` file outside the ticket tree, each hit resolved to an absolute path):

| file | crate | lib root | note |
| --- | --- | --- | --- |
| `M/⏯️tool-run/🦀️.rs` | `semio-framework-tool-run` | package stub `📦️packages/🦀️rust/🦀️.rs` (`#[path = "../../🦀️.rs"] mod component;` + explicit `pub use component::{...}` list) | |
| `M/🛠️tool-machine/🦀️.rs` | `semio-framework-tool-machine` | stub (`pub use component::*;`) | |
| `M/🔄️machine/🦀️.rs` | `semio-framework-machine` | stub (`extern crate self as machine;` + `pub use component::*;`) | |
| `M/🕸️graph/🗣️dsl/🦀️.rs` | `semio-framework-graph-dsl` | the file itself (`path = "../../🦀️.rs"`) | |
| `M/🕸️graph/⚙️engine/🦀️.rs` | `semio-framework-graph-engine` | the file itself | |
| `M/🕸️graph/⏯️layout-run/🦀️.rs` | `semio-framework-graph-layout-run` | stub (`pub use component::*;`) | feature `testing` |

None of the six is mounted from `🔌️plugin/**` or from anywhere else but its own package stub: 40 `#[path = "...(⏯️tool-run|🛠️tool-machine|🔄️machine|🗣️dsl|⚙️engine|⏯️layout-run)..."]` hits in the repo were resolved, 0 point at one of the six targets. The hits under `🔌️plugin/**` (`👷️job/🛬️lifted/⏯️tool-run`, `📜️declaration/⏯️tool-run`, `🪝️mounts/👥️wp3-owners` -> `🔌️plugin/⏯️tool-run/🦀️.rs`, `🔌️plugin/🛠️tool-machine/🦀️.rs`, `🧵️instance/**/📥️staging/**`) are the plugin crates' own, different files with the same module names. No plugin file was edited and none needs an edit. No `include!`/`include_str!` points at any of the six roots.

## 2. What changed

Method: `tgm-tools/split.py` cuts line ranges of the snapshot root into `<emoji>️<kebab>/🦀️.rs` children, rewrites the root with `#[path = "<folder>/🦀️.rs"] mod <name>;` plus `pub use <name>::*;` at the place of the first cut line, and widens visibility (section 3). Inline `mod x { ... }` blocks become file modules with the same module name and parent, so their hierarchy (and `super::`/`crate::` paths) is unchanged. All specs are in `tgm-tools/specs/`.

| file | before | root after | children (lines) |
| --- | ---: | ---: | --- |
| `M/⏯️tool-run/🦀️.rs` | 2,420 | 182 | `🚦️lifecycle` 424, `🧮️step-progress` 220, `🗄️trace-store` 329, `⏱️tick-writer` 352, `🔤️codec` 424, `📐️definition` 278, `🏷️actions-labels` 260 |
| `M/🛠️tool-machine/🦀️.rs` | 1,971 | 483 | `🌊️gesture` 463, `🫧️scrub` 377, `🕸️node-graph-edit` 233, `⌨️typing` 443 |
| `M/🔄️machine/🦀️.rs` | 1,781 | 361 | `🌐️host` 202, `🔎️inspect` 67, `🧠️kernel` 613, `💾️persist` 102, `🎬️runtime` 178, `🔁️step` 100, `🧭️testing` 158 |
| `M/🕸️graph/🗣️dsl/🦀️.rs` | 2,256 | 134 | `🔍️queryable` 265, `🔌️wire` 175, `🌳️syntax` 409, `🧭️language-service` 687, `🧮️parser` 305, `⚡️executor` 311 |
| `M/🕸️graph/⚙️engine/🦀️.rs` | 1,876 | 204 | `🗄️storage` 690, `🪟️views` 485, `🧰️support` 274, `🔖️kinds` 130, `🌊️max-flow` 128 |
| `M/🕸️graph/⏯️layout-run/🦀️.rs` | 1,828 | 76 | `📖️vocabulary` 171, `🕸️input-graph` 173, `🎛️config` 221, `🏃️job` 983, `🔮️testing` 234 |

Cut decisions:

* Region boundaries decide the cuts: a region moves whole with its `//#region` and `//#endregion` lines. Struct and its impls stay in one child. `Storage` with its `#[value(with = "..._bridge")]` bridge modules, the view-trait impls for `Storage` and `Csr` stay together in `⚙️engine/🗄️storage` because the derive names the bridge modules by string in the scope of the struct.
* `tool-run`: `Limits` and `Identity` stay in the root next to the existing mounts and the `cfg(test)` global allocator, so the root is the contract index. `Definition` and `Panel` share a child (the panel keys belong to the definition vocabulary).
* `tool-machine`: `Yield`, `Transaction`, `Machine` stay in the root; the gesture, scrub, node-graph-edit (NodeDrag, Once, NodeGraphEditRows) and typing domains move.
* `machine`: the seven non-wasm inline modules move 1:1 (their inner `//!` docstring already starts with a unique emoji). `Ids`, `Schema`, `Reexports`, the cfg-wasm `wasm_bridge`/`wasm_smoke` and the test mounts stay in the root. Their `include!("🧪️tests/...")` literals became `include!("../🧪️tests/...")` (7 lines), since `include!` resolves against the including file.
* `dsl`: `queryable` and `wire` (inline modules) move with their docstring hoisted above the `// #region` comment; the wire module's `include!` got the same `../`. The Jack implementation region is cut into syntax (Ast + Lexer), language service (Language + LanguageService + DslIdiom), parser and executor.
* `engine`: one private `use geometry::Point;` was inside the moved `Kinds` region while the test module reaches `Point` through `use super::*`; it is hoisted back into the root (`split.py` hoists every private `use` of a moved range).
* `layout-run`: `Quadtree` and `Job` stay in one child because the job code works on the tree's internals; the `Limits` region with the two private consts `NONE` and `MIN_DISTANCE` stays in the root (children see them as descendants). The inline `pub mod testing` gets a new file docstring; its outer `///` docs and `#[cfg(any(test, feature = "testing"))]` stay on the mount line.
* Module names of the new mounts avoid names that the unit tests generate with `statechart!` (`scrub`, `typing`): they are `scrub_tool`, `typing_tool`, `gesture_drive`, `node_graph_edit`; the other new modules are `lifecycle`, `step_progress`, `trace_store`, `tick_writer`, `record_codec`, `definition`, `actions_labels`, `jack_*`, `graph_*`, `layout_*`. A scan for `<name>::` path uses in the reachable tree finds none (apart from the unchanged `machine` module names and `layout-run`'s `testing`).
* Header docstrings: each new file starts with `//! <emoji> ...`. The emoji is the folder emoji unless the same emoji already opens another docstring in that file (the repo's docstring-emoji-reuse census, design 21.2); then an unused one is taken (`🧩`, `🧱`). Checked with the real `docstringEmojiReuseOfText`: no new reuse. Original roots carried reuse groups already (tool-run 16, tool-machine 29, machine 22, dsl 13, engine 22, layout-run 11); after the split the totals are 7, 15, 25, 6, 15, 4 groups (machine +3 groups, 106 -> 95 openers: the inline module docs now count per file). The one remaining `no-emoji` hit in tool-run and the 9 in machine are inherited lines.

New files: 34 folders, each with one `🦀️.rs`; the list for the final taxonomy sweep is section 7.

Nothing else in the repository changed except the three TS readers (section 5).

## 3. Visibility widening (name-keyed against every other file of the crate)

A moved private item, inherent method or struct field that another file of the same crate names (masked tokens, so comments and strings do not count; methods need a preceding `.` or `::`, fields need `.field` or `Struct { field`) was widened to `pub(crate)`. `pub(super)` in moved text became `pub(crate)`; none occurred. Trait impls and inherent impls were kept next to their struct.

| crate | items | fields | methods | note |
| --- | ---: | ---: | ---: | --- |
| tool-run | 23 | 23 | 3 | `record_codec` helpers (`field`, `spec`, `uint`, `list`, ...) are reached by the tick writer, the trace store and the existing `🎞️tick/**` children; trace-store and tick-writer fields by `🛫️encode`, `🏗️native` and the unit tests |
| tool-machine | 1 | 8 | 2 | `GESTURE_CLOCK_TICK` (read by node-graph-edit), runner/window ledger fields read by the unit laws |
| machine | 0 | 0 | 0 | inline modules were already modules |
| dsl | 5 | 4 | 2 | lexer `Token`, `SpannedToken`, `lex`, `lex_spanned`, parser `Parser::new`/`expect`, `match_patterns` |
| engine | 0 | 11 | 0 | view and flow struct fields read by `Storage` (`.nodes`, `.edges`, `.graph`) |
| layout-run | 3 | 15 | 11 | job and tree internals read by the unit tests and the testing child |

Full lists per crate with the files that name them: `🗑️generated/tgm-<crate>-plan.json`. Two private tuple structs (`ScrubMachine`, `TypingMachine`: private `PhantomData` marker field) are named from the unit laws by type only, never constructed, so they stay private (the definition scan prints them and the grep for `ScrubMachine(` / `TypingMachine(` finds no construction).

## 4. Verification (commands run, real results)

All tools in `tgm-tools/`: `tgmlib.py`, `modtree.py`, `split.py`, `oracle.py`, `verify.py`, `pubsurface.py`, `census.py`, `docreuse.ts`, `docreuse2.ts`, `readers.ts`, `emojicheck.py`, `rslex.py` (copy of the frs lexer), `specs/*.py`.

### 4.1 Differential (`oracle.py <crate> <label> [cfg...]`)

Baseline A = the pre-edit snapshot of the module tree (`🗑️generated/tgm-work/<crate>/snap`), B = a copy of the live tree. Both are patched identically (external glob imports -> `const _: () = ();`, `extern crate` of missing crates removed; 0 such patches were needed in these trees). `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o <scratch .rmeta, deleted> --cap-lints allow [--test] [--target ...]` from the real crate lib root; the multiset key is severity + normalised message + primary label (module qualifiers stripped, file and line dropped). Run after the last Rust edit (`r3`):

| crate | lib | test | wasm32-unknown-unknown | wasm32-wasip2 | test+wasm | test+wasip2 | features |
| --- | --- | --- | --- | --- | --- | --- | --- |
| tool-run | 203 = 203 | 313 = 313 | 203 = 203 | 203 = 203 | 313 = 313 | 313 = 313 | none |
| tool-machine | 161 = 161 | 298 = 298 | 161 = 161 | 161 = 161 | 298 = 298 | 298 = 298 | none |
| machine | 5 = 5 | 56 = 56 | 18 = 18 | 5 = 5 | 69 = 69 | 56 = 56 | none |
| graph-dsl | 154 = 154 | 175 = 175 | 154 = 154 | 154 = 154 | 175 = 175 | 175 = 175 | none |
| graph-engine | 170 = 170 | 197 = 197 | 170 = 170 | 170 = 170 | 197 = 197 | 197 = 197 | none |
| graph-layout-run | 25 = 25 | 55 = 55 | 25 = 25 | 25 = 25 | 55 = 55 | 55 = 55 | `testing`: lib 34 = 34, test 55 = 55 |

Every cell: baseline errors = current errors, onlyNew 0, onlyOld 0 (all remaining errors are the unresolved external crates and attributes that cancel). Earlier runs (`r1`, `r2`) gave the same result per crate; the first tool-machine run found real name clashes (`scrub::Event` of the tests' `statechart!` against my first module name `scrub`), fixed by renaming the modules (section 2) before the final runs.

### 4.2 Seeded sensitivity (`--mutate`, applied only to the sandbox copy of the current tree)

| seed | result |
| --- | --- |
| tool-run, `pub use record_codec::*;` removed (lib) | onlyNew 53: E0425 for the moved codec fns, E0432/E0433 for `ToolRunCodecError` |
| tool-run, `pub(crate) fn encode_body` -> private (lib, test) | onlyNew 3 each: E0425 `encode_body` |
| machine, `pub fn init` -> private (lib, test) | onlyNew 3 / 6: E0603 `init` is private |
| machine, `mod kernel;` -> `mod kernelx;` (lib) | onlyNew 8: E0432/E0433/E0283 |
| tool-machine, `pub use typing_tool::*;` removed (test) | onlyNew 45: E0425 on `TypingContext`, `TypingLedger`, ... (lib is correctly identical: the lib never names them) |
| dsl, `pub use jack_parser::*;` removed (lib) | onlyNew 2: E0425 `parse` |
| engine, `pub struct Interner` -> private (test) | onlyNew 6: E0425/E0433 `Interner` |
| engine, `pub use graph_kinds::*;` removed (test) | onlyNew 8: E0422 `Node`, `Handle`; E0425 `property_bag_from_value` |
| layout-run, `pub(crate) fn pairwise_repulsion` -> private (test, test+testing) | onlyNew 2 each: E0425 |

### 4.3 Losslessness, regions, rustfmt, paths, mounts (`verify.py <crate>`, JSON in `🗑️generated/tgm-<crate>-verify.json`)

* Multiset of stripped non-blank lines, old root vs new root + children. Added lines are exactly: header docstrings 7/4/0/4/5/5, `use super::*;` 7/4/0/4/5/4, mount lines 21/12/14/16/15/14, `pub(crate)` widenings 49/11/0/11/11/29, `include!("../` rewrites 0/0/7/1/0/0 (tool-run / tool-machine / machine / dsl / engine / layout-run); removed lines are only the inline `mod x {` and `}` wrappers (machine 14, dsl 4, layout-run 2) and the lines replaced by a widened version. Unexplained: none. (The layout-run `use super::*;` count is 4 because the inline `testing` child keeps its own `use super::{...}`.)
* Region markers: identical multiset (22, 20, 70, 38, 30, 18) and balanced in every file.
* rustfmt parse: `rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --check <file>`, stderr scanned for `error`; a deliberately broken file is reported (checked); all 8 + 5 + 8 + 7 + 6 + 6 = 40 files parse (rustfmt diffs in untouched original text are not counted).
* Paths: every `#[path]` and `include!` literal of the new and changed files resolves; longest repo-relative path 103 bytes.
* Duplicate mounts: reachable-tree mount counts per target, before vs after: 0 new. Repo-wide `git grep -F '"<folder>/🦀️.rs"'` for the 34 new names finds the new mount line in the own root only, apart from three unrelated same-named folders below other parents (`🎛️dashboard/🌀️daemon/🚦️lifecycle`, `os/🚪️io/🔌️wire`, `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🧰️support`), which resolve to different files.
* Public surface (`pubsurface.py`): the exact-`pub` top-level items (kind + name; `pub use` lines and `pub mod` included, `pub use <child>::*` mounts excluded) of the original root equal those of the new root plus the lines-kind children in all six crates.
* Census (`census.py`, lib closure of the crate itself, test-gated files excluded; the gate counts dependencies too and still refuses these crates through `semio-framework-os-kernel`): tool-run 13 -> 20 files, 225,190 -> 227,178 bytes, 2,709 -> 2,737 active lines; tool-machine 2 -> 6, 96,827 -> 97,835, 1,441 -> 1,457; machine 2 -> 8, 72,100 -> 62,000 (the `testing` module is now correctly gated out), 1,219 -> 1,130; dsl 1 -> 7, 97,700 -> 97,086, 1,899 -> 1,915; engine 1 -> 6, 81,770 -> 82,884, 1,515 -> 1,535; layout-run 2 -> 6, 82,028 -> 71,056 (`testing` gated out), 1,596 -> 1,404. All stay inside 32 files, 262,144 bytes and 4,000 active lines per crate.

### 4.4 Definition scan (`verify.py`, `defscan_remaining_private_crossings`)

Re-run on the installed tree after the apply: every private top-level item, inherent method and struct field of a moved child that another file of the crate still names. Result: empty except the two intentional tuple-struct markers above. This scan and the oracle cannot see field and method privacy through type inference (E0616/E0624 only surface in typeck), so the name-keyed scan is the evidence for those, not a compiler.

## 5. Source-text readers

Tests that read a moved root as plain text break when the text they look for moves. The search covered every tracked `.ts/.tsx/.mjs/.cjs/.js` file (12,694 scanned, `tgm-tools/tsreaders-scan.py`, regex over the six module names followed by `🦀️.rs` across line breaks) plus every TS file inside the six module trees. Six readers were re-pointed to the module tree (`readModuleTreeSource`, `🔨️modules/📚️compiler/📖️syntax/🌳️module-tree`), which returns the root first and then every mounted child:

tool-machine (`M/🛠️tool-machine/🦀️.rs`):
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔗️node-graph-wire-edit/🟦️.ts` (the `"disconnect" =>` arm field)
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts` (`guestFields` per operation arm)
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🧪️tests/🧮️source-census/🟦️.ts` (+ one `import`; the machine-vocabulary regex over `^pub (trait|struct|fn) <word>`)

tool-run (`M/⏯️tool-run/🦀️.rs`):
* `M/⏯️tool-run/♻️retirement/🧪️tests/🟦️.ts` (the five `RetireOwned` derive lines before `ToolRunTickWriter`, `ToolRunProgress`, `ToolRunStep`, `ToolRunStepRing`, `ToolRunTracePage`; + one `import`)
* `M/⏯️tool-run/🎞️tick/⏭️successor/🧪️tests/🟦️.ts` (`pub fn admit_original_successor(`; + one `import`)
* `M/⏯️tool-run/🎞️tick/🎟️admission/🧪️tests/🟦️.ts` (the five `admit_original_*` markers; the now unused `readFileSync` import replaced by the helper import)

Evidence:
* `tgm-tools/readers.ts` (tool-machine) evaluates the tests' own assertions on the pre-edit root text and on the module-tree text: `disconnect` field `synapseId` in both; the arms `move`, `connect`, `disconnect`, `setSlider`, `insertPort` return identical field lists; the 6 vocabulary words are found in both. `DIFFERENCES 0`.
* `tgm-tools/readers-toolrun.ts` (tool-run): old text `[true x5]`, module tree `[true x5]`, **new root file alone `[false x5]`** for the derive regex, the five markers and `admit_original_successor`. So the three tool-run tests would have failed unedited and pass on the tree.
* `bun test` of the three edited tool-run tests (cwd repo root, `🗑️generated/tgm-bun-toolrun-readers.log`): `3 pass 0 fail 54 expect() calls`.

Other readers checked and unaffected: `M/⏯️tool-run/🏛️ownership/🧪️tests` walks every `.rs` of the module for the strings `/🛍️products/`, `/✏️s/`, `/🌎️hub/` (none in the new files); the `🎞️tick/🛬️receive/**` and `🛫️encode` tests read their own sibling children; `📺️renderer/.../♻️frame-close` already reads the tool-run root through `readModuleTreeSource`; `📚️library/🔍️discovery/🧪️tests/🧪️interactivity-tool-run-policy` uses `⏯️tool-run/🦀️.rs` as a virtual path in an in-memory tree; the plugin tests `🔌️plugin/🛠️tool-machine/♻️retirement`, `🔌️plugin/♻️cancellation/📦️lease` and `retained-clone-check` read the plugin crates' own `⏯️tool-run`/`🛠️tool-machine` files; `M/🗣️dsl/🧪️tests/🧫️fixtures/🧱️ownership/🔣️.json` names the file as data only. No Rust code reads a root (`include_str!`, `read_to_string` and `CARGO_MANIFEST_DIR` scans: none).

Test runs not completed: `bun test ./<source-census>` fails at module load with `ENOENT .../🎯️acceptance/🧫️fixtures/🧮️source-census/🔣️.json` (the fixture lives under `🎯️acceptance/🧪️tests/🧫️fixtures/`; `HEAD` has the same line, so this is a foreign pre-existing failure). The vitest run of the two renderer tests (`bunx vitest run` from the repo root, `🗑️generated/tgm-vitest-renderer.log`) never got past collection under machine load (about 45) and ended with `FATAL ERROR: Reached heap limit Allocation failed - JavaScript heap out of memory` (`exit=134`, during its repo-wide directory scan), so those two tests have no real run; `tgm-tools/readers.ts` is the substitute.

## 6. Real checks for the compile queue (not run)

All six crates are members of `🧰️framework/Cargo.toml`. Under the fleet mutex, one launcher call:

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-tool-run -p semio-framework-tool-machine -p semio-framework-machine -p semio-framework-graph-dsl -p semio-framework-graph-engine -p semio-framework-graph-layout-run --lib --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
```

then the same with `--tests` instead of `--lib`, with `--features testing` added for `semio-framework-graph-layout-run`, and per wasm target (`--target wasm32-unknown-unknown`, `--target wasm32-wasip2`) for the six crates (all build for both). Direct dependents to add once the six are green (Cargo.toml grep): the s-artifact crates `◻️2d`, `🔲️grid2d`, `🖼️bitmap`, `🧊️3d`, `🧱️grid3d` (tool-run), the `🀄️wfc` and cad/lowpoly/process3d artifact crates (tool-machine, machine), `semio-framework-machine-derive`. Then `test` on the six crates (`-- test -p <crate> --lib`, layout-run with `--features testing`).

TS: `bunx vitest run` on the two renderer tests; `bun test ./<source-census>` once its fixture path is repaired.

## 7. New folders for the final taxonomy sweep

All 34 resolve with the live taxonomy (section 0). Parent `M/⏯️tool-run`: `🚦️lifecycle`, `🧮️step-progress`, `🗄️trace-store`, `⏱️tick-writer`, `🔤️codec`, `📐️definition`, `🏷️actions-labels`. Parent `M/🛠️tool-machine`: `🌊️gesture`, `🫧️scrub`, `🕸️node-graph-edit`, `⌨️typing`. Parent `M/🔄️machine`: `🌐️host`, `🔎️inspect`, `🧠️kernel`, `💾️persist`, `🎬️runtime`, `🔁️step`, `🧭️testing`. Parent `M/🕸️graph/🗣️dsl`: `🔍️queryable`, `🔌️wire`, `🌳️syntax`, `🧭️language-service`, `🧮️parser`, `⚡️executor`. Parent `M/🕸️graph/⚙️engine`: `🗄️storage`, `🪟️views`, `🧰️support`, `🔖️kinds`, `🌊️max-flow`. Parent `M/🕸️graph/⏯️layout-run`: `📖️vocabulary`, `🕸️input-graph`, `🎛️config`, `🏃️job`, `🔮️testing`. Every emoji is already a folder prefix elsewhere in the repo (`tgm-tools/emojicheck.py`, counts in `🗑️generated/tgm-emoji-counts.txt`) and unique among the siblings of its parent. The list is also in `🗑️generated/tgm-newdirs.txt`.

## 8. Changed files

Edited: the six roots (`M/⏯️tool-run/🦀️.rs`, `M/🛠️tool-machine/🦀️.rs`, `M/🔄️machine/🦀️.rs`, `M/🕸️graph/🗣️dsl/🦀️.rs`, `M/🕸️graph/⚙️engine/🦀️.rs`, `M/🕸️graph/⏯️layout-run/🦀️.rs`) and the six TS readers of section 5. Created: the 34 child `🦀️.rs`, this file, `tgm-tools/`. Pre-edit copies: `🗑️generated/tgm-work/<crate>/snap` (restore: `tgm-tools/split.py <crate> restore`). No Cargo manifest, `project.json`, launch file, registry or taxonomy file was touched; no new test entry exists, so no launch registration was needed.

## 9. Residual risks and hand-offs

* No compiler saw this. The differential proves name resolution, imports, globs and module privacy of paths; it does not prove typeck, trait solving, borrowck, field and method privacy (E0616/E0624, covered by the name-keyed widening and its re-scan) or proc-macro expansion (the `ToValue`/`FromValue`/`RetireOwned`/`statechart!` derives are unresolved in the sandbox, so their expansions were not seen). Highest-risk spots at the first compile: glob chains `pub use child::*` next to `use super::*` (E0659 would show in the differential, none did), the `#[value(with = "..._bridge")]` derive strings in `⚙️engine/🗄️storage`/`🔖️kinds`, and the unit tests of `🏃️job` that now read widened internals.
* Widening is name-keyed and conservative: a few `pub(crate)` markers sit on names that only collide (`graph`, `nodes`, `DEFINITION`); they widen nothing outside the crate.
* The vitest run of the two renderer tests aborted (`exit=134`, node out of heap during its repo-wide directory scan) and no real Rust test could run (no compile window); `tgm-tools/readers.ts` is the substitute evidence for the renderer tests.
* **Hand-off to the owner of `🔌️plugin/**` (pi1 or pwp7):** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run/🌱️operation/🧪️tests/🟦️.ts` line 3 ends with `expect(readFileSync('🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs','utf8')).toContain('admit_original_op')`. That reads the framework tool-run root as text; `admit_original_op` now lives in `M/⏯️tool-run/⏱️tick-writer/🦀️.rs`, so the assertion fails until it reads the tree: `readModuleTreeSource('🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs')` with an import of `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts`. I did not touch it (brief: nothing under `🔌️plugin/**`); the earlier read in the same test (`new URL('../🦀️.rs', import.meta.url)`) is the plugin tests tree's own `🌱️operation/🦀️.rs` and is unaffected. It is the only reader of a framework root found under `🔌️plugin/**`.
* Foreign: `🧮️source-census/🟦️.ts` fails at load with a missing fixture path (pre-existing); `git` prints `error: daemon terminated` (fsmonitor) on every call; the original `⏯️tool-run/🦀️.rs` and the others already had docstring-emoji reuse groups, which this split reduces but does not clear.
* Other hand-offs: none. `⏯️tool-run` and `🛠️tool-machine` of the plugin tree are different files and were not touched.
