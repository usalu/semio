# Result CENSUS4-PKG-43 (prefix `c4p43`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `O` = `🧰️framework/🛍️products/💻️os/🔨️modules`. Tools: `F/c4p43-tools/` (descendant of `c4p37-tools`, `c4p35-tools` and `c4p39-tools`). Logs: `F/🗑️generated/c4p43-*` (nothing deleted). Sandbox root: `/Users/ueli/.cache/c4p43/` (`stage/`, `stage-frag/`, `sbx/`, `cen/`, `orc/`).

## 0. Verdict

**DONE (static evidence; the real `bun repo cargo` check is queued).** Last update 2026-10-17 15:10.

- All 7 files of Package 43 (8,883 lines) are split: 7 wiring roots (11 to 70 lines) plus 33 children (largest 735 lines, 5 in include fragments). Lossless by line multiset (live tree byte-equal to the staged tree), region markers balanced per file (40 of 40 files), test names unchanged (93 = 93 in the unit fragments), `rustfmt` parses 40 of 40 files, every child mounted exactly once, the scoped patched differential is identical for all six crate groups in every configuration on the stage and on the live tree, the definition scan is clean, taxonomy resolves 33 of 33, RGI emoji, sibling emoji unique, longest repo-relative path 150 bytes.
- Every crate stays inside the source gate (canonical `readRustSourceClosureV1`, lib and `cfg(test)`). The binding case is `semio-framework-os-directory`: 3,958 -> 3,983 of 4,000 lines (23 of 32 files). This slice cut the schema into only 3 children to keep the growth to 25 lines; the crate has 17 lines of headroom left and needs its own crate carve next (section 7).
- Ten source-text readers, pins and registries were re-pointed (section 4); the descriptor-digest law passes 8 of 8 before and after, the channel ingress consumer probe passes on the three files before and after.
- No `COMPILE ADMITTED` line in `📓️FLEET.md` at 15:03 (the fleet mutex `🗑️generated/cargo-mutex.lock` was free), so the real check is recorded as a command (section 8).

## 1. Found state (fresh slice, 2026-10-17 14:00)

- No earlier c4p43 work: no `c4p43-tools`, no `🗑️generated/c4p43-*`, no RESULT file.
- The 7 files were byte-identical to `HEAD b84bf526c7b` (git blob hashes equal) and had the census line counts. Pre-edit snapshots: `F/c4p43-tools/orig/<tag>.rs`, hashes in `F/c4p43-tools/orig/SHA256`. The installers asserted per target that the live file still equalled its snapshot.

| tag | file (tail under `O/`) | lines before | SHA256 (first 16) | owning crate | mount chain |
| --- | --- | ---: | --- | --- | --- |
| inf | `💡️inference/🦀️.rs` | 1,097 | `dc44dcebd32d41aa` | `semio-framework-os-inference` | the file is the lib root (`[lib] path = "../../🦀️.rs"`) |
| dir | `📇️directory/🧬️schema/🦀️.rs` | 1,859 | `0237d163ab9cc5af` | `semio-framework-os-directory` | `#[path] pub mod` of `📇️directory/🦀️.rs` (lib root) |
| hcd | `📡️spr/📜️history/⚙️codec/🦀️.rs` | 1,120 | `fbf02d58c2574970` | `semio-framework-os-history-codec` | the file is the lib root |
| plw | `📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs` | 1,105 | `e5fdb65c9aefc6a2` | `semio-framework-os-spr` | `#[cfg(any(test, feature = "protocol-laws"))] pub mod protocol_laws` of `📡️spr/🦀️.rs` |
| ccd | `📡️spr/🧵️channel/🗜️codec/🦀️.rs` | 1,205 | `181976aaf127243a` | `semio-framework-os-channel` | `mod codec` of `📡️spr/🧵️channel/🦀️.rs` (lib root) |
| cdc | `📡️spr/🧵️channel/🧩️command-decode/🦀️.rs` | 1,109 | `b630eaa5288c0072` | `semio-framework-os-channel` | `mod command_decode` of the same root |
| cut | `📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs` | 1,388 | `0836cdd3a328dc0a` | `semio-framework-os-channel-tests` (companion) | `#[path = "../🔬️unit/🦀️.rs"] mod tests;` of the companion root |

Baselines recorded before the first edit (all kept in `F/🗑️generated/`): census (`c4p43-census-before.txt`, `c4p43-census-own-before.jsonl`), the channel ingress consumer law (`c4p43-law-channel-ingress-before.log`: 1 pass, 1 fail, inherited), the descriptor-digest law (8 pass), the crate-boundaries law (44 pass, 1 timeout at the default 5 s), the hub check files (not runnable as bun tests: 0 tests).

## 2. What was cut (before -> after, children in lines)

| tag | before | root after | children |
| --- | ---: | ---: | --- |
| `inf` | 1,097 | 65 | 🔑️dep-hash-and-failures 151, 🎯️inferred-field-and-config 119, 🗄️cache-and-session 251, 🔄️field-update 184, 🧮️cursor 125, 🏁️driver 258 |
| `dir` | 1,859 | 70 | 📖️events-commands-and-admin 608, 🪟️views-and-document-open 735, 🛰️execution-target-artifacts-and-stream 468 |
| `hcd` | 1,120 | 39 | 📝️text-grammar 258, 🧱️field-and-message-codecs 134, ✏️op-and-edit-payloads 298, 🔀️transition-and-conflict-payloads 289, 🗂️offset-index 157 |
| `plw` | 1,105 | 54 | 🎲️seeded-generators 262, 📼️history-log-laws 232, 🧮️operation-and-mutation-laws 269, 🔁️dag-convergence-and-frame-laws 148, 🏛️policy-merge-and-corpus-laws 185 |
| `ccd` | 1,205 | 24 | 🧩️wire-combinators-and-writers 242, 📤️command-encoder 278, 📚️archive-and-pack-codecs 176, 📥️command-decoder 140, 🎞️frame-codec 388 |
| `cdc` | 1,109 | 19 | 📦️archive-decode 273, 🛣️route-and-media-decode 186, 🧾️command-owner 231, 🧭️command-cursor 433 |
| `cut` (include fragments) | 1,388 | 11 | 📨️app-command-laws 315, 🎞️app-frame-laws 242, 🚫️codec-refusal-laws 299, 📚️fixture-corpus-laws 413, 🗃️archive-load-host-laws 122 |

Total 8,883 lines before; after 9,151 lines: 282 in the seven roots plus 8,869 in the 33 children (the growth of 268 lines is wiring, docstrings and re-opened region markers). Each child sits next to its root as `<emoji>️<kebab>/🦀️.rs`.

### Mechanism and decisions

- **Item-level splitter** (`c4p43.py`, descendant of `c4p37.py`): whole items (with attributes and docs) move; each child starts with a `//! <emoji> ... See [`super`].` docstring and `use super::*;` (dropped for `inf` dep-hash-and-failures and `plw` seeded-generators and operation-and-mutation-laws, which use nothing from the root: the first oracle round showed `unused import: super::*`), the root keeps its `use` lines, the existing `#[path]` mounts (plan, values, tests, the five schema modules) and the `#[path = "<folder>/🦀️.rs"] mod x;` plus `pub use x::*;` (or `pub(super) use`, plain `use`) at the position of the first moved item. No qualified path, compatibility re-export or alias was introduced.
- **Regions**: items moved with the region stack they sat in; every child is balanced, empty region pairs in the root were pruned. A region that spans several children (`Views`, `Payloads`, `Laws`, `Codec`, `PagedAppCommandDecode`) is re-opened with its original opener and closer text in each child. `hcd` had one orphan opener (`//#region 🧪️Tests` at EOF, never closed): one `//#endregion 🧪️Tests` line was appended so the root is balanced (the only added line that is not wiring or docstring).
- **Visibility**: `widen2.py` makes a moved private item, field or inherent method `pub(super)` when its name occurs in another file of the split or of the module tree: inf 63, dir 1, hcd 9, plw 25, ccd 37, cdc 56 insertions (conservative, lexical). For `ccd` and `cdc` (`nest`) the original `pub(super)` items (`malformed`, `document_archive_into_fields`, `TRANSACTION_PREPARED_OPS_MAXIMUM` and three struct fields) became `pub(in super::super)` because the files are one level deeper than before; the channel root keeps `use codec::{document_archive_into_fields, malformed}` and `use command_decode::TRANSACTION_PREPARED_OPS_MAXIMUM` unchanged. Negative control: removing that `pub(in super::super)` from `malformed` in the sandbox produced 37 new errors (33 E0425, 4 E0432).
- **`dir` in three children**: the crate sits at 3,958 of 4,000 active lines. The first cut had four children (3,991 lines); merging the first two (events, commands, administration) saved 8 lines. The module doc (`//!`) and the five existing `#[path] pub mod` submodules (session authority, space artifact creation, document index, check-in, canonical checkpoint pair, browser actor) stay in the root.
- **Test files**: `cut` (93 `#[test]` and `async_test` fns) uses five `include!` fragments spliced into the same module, so test paths, visibility and `use super::*;` stay as before; nothing is widened. `include_str!` literals get one more `../` (13 lines). Each fragment starts with a `///` line whose first character is the unique emoji. `plw` is a laws library (no `#[test]`), so it uses `#[path]` children like the source files.
- **Pre-existing finding in `cut`**: `include_str!("../../../../🧑‍💻dev/🔖️channel-version/📌️pin/🔣️.json")` resolves to a file that exists in the live tree; the sandbox oracle reports the same missing-file error for both sides (the message text is normalised).

## 3. Verification (commands and real results)

| Check | Command | Result |
| --- | --- | --- |
| Lossless line multiset | `python3 c4p43-tools/live.py inf dir hcd plw ccd cdc` (`c4p43-lossless-live.log`) plus an inline multiset check for `cut` | `LIVE <tag> bytes-equal-to-stage`, `missing 0`, `LOSSLESS OK` for the six; `cut`: bytes equal to stage, missing 0, extra 10 (5 `include!`, 5 `///`). Extras are only wiring (`#[path]`, `mod`, re-export, `use super::*`, docstring, region markers, the `hcd` `//#endregion 🧪️Tests`) |
| Structure | `python3 c4p43-tools/check35.py live` (`c4p43-check-live.log`) | `CHECKS OK`: each child mounted once, first line is a doc whose emoji equals the folder emoji, unique among siblings and different from the root, no clash with an existing sibling folder, folder emoji used as a folder prefix in the repo (770-emoji scan), regions balanced per file, test fn multiset equal (`cut` 93 = 93), no `[DEBUG]`, longest child 735 lines, longest repo-relative path 150 bytes (absolute 178 chars) |
| rustfmt parse | `python3 c4p43-tools/fmt.py live` | 40 files parsed, 0 failures |
| Scoped patched differential, stage and live | `python3 c4p43-tools/oracle.py <group>` (stage) and `C4P43_LIVE=1 python3 c4p43-tools/oracle.py <group>` (live tree), logs `c4p43-oracle-stage-*.log`, `c4p43-oracle-live-*.log` | `ORACLE <group> CLEAN` for inf, dir, hcd, plw, chn (ccd + cdc), cut on the live tree: errors and warnings identical. Configurations: inf/dir/hcd lib, `--test`, wasm32-wasip2 lib and test; plw additionally `feature="protocol-laws"` (native and wasip2); chn additionally `feature="testkit"` (native and wasip2); cut `--test` native and wasip2. Error counts orig = new: inf 111/248, dir 320, hcd 76, plw 84/102/163/37/58/90, chn 529, cut 607. Patches as in `📓️RESOLUTION-ORACLE.md` (external globs replaced; an `extern crate` line is replaced by `const _: () = ();` so that a preceding `#[cfg]` stays attached; sandbox paths and the random `/dev/rmeta` temp name normalised) |
| Oracle sensitivity (negative controls) | removed `pub(in super::super)` from `malformed` (chn) and `pub(super)` from `SplitMix64` (plw) in the sandbox, `C4P43_REUSE=1 oracle.py` (`c4p43-oracle-negative.log`) | chn: `+37` errors (E0425 x33, E0432 x4) plus a new warning; plw `--test`: `+4` E0425 plus `unused import: super::*`; both `DIFFERENCES` (the live tree is untouched by these runs) |
| First-round findings fixed before the install | stage runs | `inf`: `unused import: super::*` in the dep-hash child (`glob=False`); `dir` (and every group with an `extern crate` behind a `#[cfg]`): the oracle patch removed the line and left the attribute dangling (one root parse error, 1 error line); fixed in the oracle (`const _`), all baselines re-run; `cut`: the path text of the unreadable pin file differs between the trees (normalised) |
| Definition scan, module name clash | `python3 c4p43-tools/defscan.py <tag>` and `modclash.py inf dir hcd plw ccd cdc` | 0 cross-file leftovers (inf 20 declarations, dir 2, hcd 31, plw 0, ccd 0, cdc 0 private declarations scanned; the widened ones are already `pub(super)`); `modclash OK`. The only external glob (`pub use semio_framework_os_history_schema::*;` in `hcd`) shares no name with the codec items (11 public names in the schema tree, none defined in the codec): the glob/glob ambiguity that the neutralised oracle cannot see does not arise |
| Taxonomy | `bun c4p43-tools/taxcheck.ts c4p43-tools/newdirs.txt` (`c4p43-taxonomy.log`) | `checked=33 unresolved=0` (23 `crate-module-children`, 10 `members-of-tests`: the five laws children and the five unit fragments) |
| RGI emoji | `node c4p5-tools/rgi.mjs 🗑️generated/c4p43-foldernames.txt` | `bad=0` |
| Census (canonical `readRustSourceClosureV1` via `census.ts`, lib) | `c4p43-tools/census.sh before` / `after` | channel 10 files / 185,915 B / 3,443 lines -> 19 / 189,803 / 3,528 (ratio 0.861 -> 0.882); channel-tests (lib) 1 / 1,145 / 0, unchanged; directory 20 / 186,502 / 3,958 -> 23 / 187,887 / 3,983 (0.990 -> 0.996); history-codec 1 / 54,632 / 1,121 -> 6 / 56,370 / 1,181 (0.280 -> 0.295); inference 3 / 85,220 / 1,431 -> 9 / 87,721 / 1,493 (0.477 -> 0.498); spr (lib) 3 / 68,413 / 1,149 unchanged. None over the gate |
| Own-closure census, lib and `cfg(test)` (`census-own.ts`, live and sandbox) | `bun c4p43-tools/census-own.ts /Users/ueli/Documents/semio` (`c4p43-census-own-{before,after}.jsonl`; sandbox copies `c4p43-census-own-sbx-*.jsonl` equal the live numbers) | test closures: inference 11 files / 2,401 lines -> 17 / 2,463 (crate-boundaries limit 3,000); directory 20 / 3,958 -> 23 / 3,983 (limit 4,000); history-codec 1 / 1,121 -> 6 / 1,181 (limit 2,000); spr 8 / 2,990 -> 13 / 3,040; channel 10 / 3,443 -> 19 / 3,528; channel-tests 17 / 1,459 -> 22 / 1,478. All far below 32 files / 262,144 bytes except the line ratio of directory |
| Descriptor-digest law (reader re-point) | `bun test ./O/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🟦️.ts` | before 8 pass 0 fail (147 expects); after 8 pass 0 fail (156 expects: the schema tree parses file by file) |
| Command-ingress consumer law | `bun test ./O/📡️spr/🧵️channel/🏛️ownership/📥️command-ingress/🧪️tests/🟦️.ts` | before 1 pass 1 fail, after 1 pass 1 fail, logs identical after removing timings (the failing assertion, `declarations.length` 0 expected 1, is inherited and sits before the cases of the three files). The five cases that name the three files run through `inspectCommandIngressConsumer` on the pre-edit snapshots and on the live files with `c4p43-tools/ingress-cases.ts` (`c4p43-ingress-cases.log`): snapshot PASS, live PASS |
| Crate-boundaries law | `bun test --timeout 120000 ./O/🛢️db/🧪️tests/📦️crate-boundaries/🟦️.ts` | before (default timeout) 44 pass 1 fail (the failing test timed out at 5 s); after (120 s timeout) 45 pass 0 fail (the budgets of inference 3,000, history-codec 2,000, directory/channel/spr 4,000 hold) |
| Schema-catalog drift | `bun c4p43-tools/catalog-diff.ts` (`c4p43-catalog-diff.log`) | the tracked catalog is stale at HEAD for many foreign entries (mutation schemas and others, +4 lines); after the one-line pin edit (section 4) no line of this slice is left in the diff |
| SpaceBrowser needle lines | inline check | the first line starting with `    CreateSpace { `, `    CreateInvite { ` and `    ArchiveSpace { ` is the same line in the child as in the unsplit file |

No tool died with `ENFILE`/`EMFILE` or a Bun segfault (`ulimit -n 65536`, `nice -n 10` on every oracle, census and bun run). The `COMPILE` check was not attempted (no admission).

## 4. Source-text readers and registries

Search method: `git grep -F` of each root's path suffix over `.ts/.tsx/.py/.mjs/.sh/.json/.jsonc/.toml/.rs` (log `c4p43-readers-grep.txt`), a grep for the SHA256 of each original file, and a read of every candidate.

| Reader | What it reads | Action |
| --- | --- | --- |
| `O/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🟦️.ts` | the schema root three times (`not.toContain` of physical bodies, `toContain("pub enum DescriptorDigestError")` which moved to a child, hash transport laws) and parses it with tree-sitter | **edited**: `readFileSync` -> `readModuleTreeSource(new URL(...))` (2 reads), the parse loop walks `listModuleTree` for the schema root; import of `@semio-tech/framework-compiler-ts/syntax/module-tree`; 8 pass before and after |
| `🌎️hub/🧪️tests/📰️event-page-checks/🟦️.ts`, `🔑️directory-authority-checks`, `🛂️space-administration-checks`, `🧭️space-journey-checks` (one line each) | the schema text for `includes`, `replace`-based hostile variants and constant declarations | **edited**: `readFileSync(join(repoRoot, schema.rs), "utf8")` -> `readModuleTreeSource(join(repoRoot, schema.rs))` plus the import where absent (a peer sweep already added or aliased the import in two of them); `bun build --no-bundle` parses all four. The checks are cargo-driven hub scripts, not runnable here |
| `O/📦️packages/🦀️rust/🎛️commands/📇️directory-checks/🟦️.ts` | `rust.includes("pub struct DirectoryEventPageV1")` | **edited**: `readModuleTreeSource` (the file already imported it) |
| `O/📺️renderer/🧑‍🎨engine/🧱️elements/🏘️SpaceBrowser/🧪️tests/🔬️wgpu-unit/🦀️.rs` | `include_str!` of the schema root, then the first `    CreateSpace { ` / `CreateInvite` / `ArchiveSpace` lines (declaration order of `DirectoryCommand`) | **edited**: the `include_str!` literal names the child `📖️events-commands-and-admin/🦀️.rs` that holds `DirectoryCommand` (the needle check is proven above; the Rust test itself is queued) |
| `O/🧑‍💻dev/🔖️channel-version/📇️consumers/🔣️.json` | census of hand-written channel version literals per file: `APP_CHANNEL_VERSION = 8` (1 occurrence, `arbitrary`) was in `🧪️tests/🔬️unit/🦀️.rs`; the census scans every file by `git grep` and fails with `unregistered` and `occurrences` otherwise | **edited**: the consumer path is now `🔨️modules/📡️spr/🧵️channel/🧪️tests/🔬️unit/📚️fixture-corpus-laws/🦀️.rs` (the only fragment that holds the literal; checked with the census patterns on all stage files) |
| `O/📦️packages/🦀️rust/📋️project.json` (`test-command-ingress-consumer` inputs) | Nx input hashing: `channel/*/🦀️.rs` and the unit file only see one level | **edited**: three globs added (`🗜️codec/*/🦀️.rs`, `🧩️command-decode/*/🦀️.rs`, `🧪️tests/🔬️unit/*/🦀️.rs`) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` | SHA256 of the schema root's `🦀️.rs` facet | **edited**: one hash line `0237d163...` -> `d71d3011...` (the value the generator renders; verified by `catalog-diff.ts`), the other foreign drift of the catalog left alone |
| `O/📡️spr/🧵️channel/🏛️ownership/📥️command-ingress/🧪️tests/🧫️fixtures/🔣️.json` (retired-public, private-import, retired-laws on the three channel files) | tree-sitter probes of the files | **no edit**: the root files keep their single `use ::protocol::wire::command_ingress::{...}` declaration and no retired public name exists; 5 of 5 cases pass on snapshot and live |
| `O/🔌️plugin/🛂️describe/🧪️tests/🔬️example-assets/🦀️.rs` | `read_module_tree_source` of the directory schema | **no edit**: already a tree read |
| `O/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json` (entries of inference and history-codec) | crate manifest and entry path, `maximumActiveLines` | **no edit**: roots unchanged; the law passes |
| `🧑‍💻dev/🔖️channel-version/🧪️tests`, `POLICY_SPLIT_SOURCE_ROOTS`, build-owners, launch routes, taxonomy | paths or readers | **no edit**: none of the 7 roots is listed; no crate, test target or command was added or renamed |
| Doc and comment mentions (`SpaceBrowser/🎯️targets/.../🦀️.rs`, `🔌️plugin/🛂️describe/🦀️.rs`) | paths only | **no edit** |

## 5. Files

- Edited (7 targets): `O/💡️inference/🦀️.rs`, `O/📇️directory/🧬️schema/🦀️.rs`, `O/📡️spr/📜️history/⚙️codec/🦀️.rs`, `O/📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs`, `O/📡️spr/🧵️channel/🗜️codec/🦀️.rs`, `O/📡️spr/🧵️channel/🧩️command-decode/🦀️.rs`, `O/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs`.
- Edited (readers, section 4): `O/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🟦️.ts`, `🌎️hub/🧪️tests/{📰️event-page-checks,🔑️directory-authority-checks,🛂️space-administration-checks,🧭️space-journey-checks}/🟦️.ts`, `O/📦️packages/🦀️rust/🎛️commands/📇️directory-checks/🟦️.ts`, `O/📺️renderer/🧑‍🎨engine/🧱️elements/🏘️SpaceBrowser/🧪️tests/🔬️wgpu-unit/🦀️.rs`, `O/🧑‍💻dev/🔖️channel-version/📇️consumers/🔣️.json`, `O/📦️packages/🦀️rust/📋️project.json`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json`.
- Created: the 33 children listed in section 2 (folder list in `F/c4p43-tools/newdirs.txt`).
- Tools, kept in `F/c4p43-tools/`: `c4p43.py` (stager, lossless, installer), `fragsplit.py` and `specs-frag/cut.json` (include fragments), `install_frag.py`, `widen2.py`, `rsitems.py`, `specs/*.py` (one spec per file), `oracle.py`, `sandbox.py`, `check35.py`, `fmt.py`, `defscan.py`, `modclash.py`, `globuse.py`, `testnames.py`, `live.py`, `census.sh`, `census-own.ts`, `catalog-diff.ts`, `ingress-cases.ts`, `taxcheck.ts`, `newdirs.txt`, `orig/` and `orig/SHA256`.
- No `[DEBUG]` log was added.

## 6. Foreign errors and inherited findings

- **Channel ingress consumer law** fails at baseline and after with the same assertion (`private-import`/`public-export` count of 0 expected 1 on an earlier case); not caused by this slice, and the five cases for the three files pass.
- **Crate-boundaries law**: the test "every production source file is owned by exactly one crate" times out at the default 5 s under load; it passes with `--timeout 120000`.
- **Schema catalog** is stale at HEAD for foreign entries (mutation schema hashes and four lines); regenerate with the schema generator when the owner is ready. Only this slice's own hash line was updated.
- **Hub check files** are cargo-driven scripts, not bun tests; their re-points are proven by the parse (`bun build --no-bundle`) and by the module-tree read semantics (`parent first, children in declaration order`), not by running them.

## 7. Hand-offs

- **`semio-framework-os-directory` is at 3,983 of 4,000 active lines** (ratio 0.996). Any growth of the crate (peer edits to the directory schema, access policy, identity or sign-in modules) pushes it over the gate. A crate carve of the schema (events and commands, views and document open, execution target and artifacts are three cohesive children already) is the next step for the crate-carving slices; a file split cannot create headroom because every child costs about 8 lines of docstring, glob and wiring.
- The schema catalog (`📚️library/🔣️schema-catalog.json`) needs a regeneration for the foreign drift.

## 8. Compile queue (not admitted)

No `COMPILE ADMITTED` line in `📓️FLEET.md`; the mutex folder `🗑️generated/cargo-mutex.lock` was free at 15:03. Exact command for the compile queue (hold the fleet mutex around the launcher call):

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-inference -p semio-framework-os-directory -p semio-framework-os-history-codec -p semio-framework-os-spr -p semio-framework-os-channel -p semio-framework-os-channel-tests --lib --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
```

then the same with `--features semio-framework-os-spr/protocol-laws` and `--features semio-framework-os-channel/testkit`, and with `--target wasm32-wasip2` for the lib of the five non-test crates. Afterwards the native tests that run or read the split code: `cargo test -p semio-framework-os-channel-tests --lib` (93 async and sync tests in the five fragments, names unchanged), `-p semio-framework-os-spr --lib` (protocol laws), `-p semio-framework-os-inference --lib`, `-p semio-framework-os-history-codec --lib`, and the SpaceBrowser wgpu unit test `every_command_builder_emits_the_shared_canonical_field_order` (reads the schema child by `include_str!`). If a build exposes a missing `pub(super)`, widen that one definition line (the spec and `c4p43.py` reproduce the stage).

## 9. Residual risks

- Not covered by a compiler: borrowck, trait-method resolution through glob re-exports and the lint set beyond `unused-imports` and the default resolution lints (the differential compares all warnings, none new). The differential runs with unresolved external crates (37 to 607 error lines per configuration, identical before and after), so field and method privacy through external types is only covered by the definition scan and the widener; the real compile decides.
- `pub(super)` widening is name-based and conservative (generic names such as `new`, `encode`, `drive` were widened on private items).
- `include!` fragments are invisible to rustfmt; they were parse-checked as files by the fragment splitter and the module-tree walker.
