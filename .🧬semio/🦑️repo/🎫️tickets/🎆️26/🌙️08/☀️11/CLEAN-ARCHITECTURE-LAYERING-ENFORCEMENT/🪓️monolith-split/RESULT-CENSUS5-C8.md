# Result CENSUS5-C8 (prefix `c5c8`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no cargo/nx/`bun repo`. `F` = this folder. `PL` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`. Tools: `F/c5c8-tools/` (descendant of `c5c-tools`; predecessor snapshots re-verified by `shasum -a 256 -c`, all six files of package 8 equal their live files before the first edit). Logs: `F/🗑️generated/c5c8-*` (nothing deleted). Scratch: `/Users/ueli/.cache/c5c8/` (`stage`, `stage-frag`, `sbx`, `orc`, `pos-work`, `nm`).

## 0. Verdict

**Package 8: 5 of 6 files split and installed (static evidence); 1 file refused (`pai`, an indivisible trait impl).**

| tag | file (tail under `PL/`) | before | after: root + children (max child) | mechanism | crate |
| --- | --- | ---: | --- | --- | --- |
| mfs | `🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs` | 1,336 | 45 + 7 fragments (268) | `include!` fragments (flat test module, 20 tests to 20) | `semio-framework-plugin-surface-fixture` |
| pai | `🧵️instance/🔌️plugin-app-impl/🦀️.rs` | 1,728 | **unchanged, refused** | one `impl PluginApp for VcsArtifactApp` of 1,612 lines | `semio-framework-plugin-instance-app` |
| pub | `🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🦀️.rs` | 1,008 | 56 + 5 (404) | `#[path]` modules; the single `impl TypedCtx` (lines 89 to 1008 of the snapshot) was cut into 4 `impl` blocks at method boundaries | `semio-framework-plugin-instance-typed-ops` |
| rc | `🧵️retained-command/🦀️.rs` | 1,021 | 60 + 3 (657) | `#[path]` modules | `semio-framework-plugin-retained-command` |
| ch | `🪆️child/🦀️.rs` | 1,097 | 53 + 5 (315) | `#[path]` modules | `semio-framework-plugin-child` |
| wc | `🪟️window/🎚️config/🦀️.rs` | 1,013 | 51 + 4 (364) | `#[path]` modules | `semio-framework-plugin-window-config` |

24 new folders (`<emoji>️<kebab>/🦀️.rs` next to each root), all RGI (`rgi.mjs` bad=0) and resolved by the live taxonomy (`tax.ts`: 24 dirs, 0 unresolved). Longest repo-relative path 198 bytes (limit 240), longest absolute 171 chars.

### Refusal: `pai` (`🔌️plugin-app-impl`)
- Lines 117 to 1728 are one `impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp<A, M>` with about 100 methods; lines 3 to 115 are imports, 4 free functions and one test helper (113 lines, nothing to gain).
- Rust allows exactly one impl of a trait for a type, so the block cannot be cut at method boundaries as the inherent `impl` blocks of `pub` could. A lossless cut needs a macro wrapper (`macro_rules!` bodies invoked inside the impl) or a new method layer; both change code, not only wiring, and the brief forbids that.
- Long-term cut (design change, needs its own ticket): split the `PluginApp` trait of the plugin SDK into role traits (maintenance and close, typed-operation receipts, local interaction query, transactions, document archive, media) so each impl lands in its own file.
- The crate holds 9 files, 241,072 bytes (0.92 of the gate), so no cut can be added to it before the bytes drop; the file stays on the census list.

### Decisions
- **rc** was first cut into 5 children following the code regions, then re-cut into 3 (`🔖️checkpoint-and-work-contract`, `🧵️retained-command-job`, `🧯️raw-allocation-close-oracle`): the completion-rejection law slices the `//#region 🧵️Job` region (`sourceStart` to the first `//#endregion 🧵️Job`) of the root, so the whole Job region (payload, phase, job record, all three impls) must stay in one file. The first install was rolled back with `uninstall.py rc` and replaced.
- **rc** `🧯️raw-allocation-close-oracle` carries `#[cfg(any(test, feature = "artifact-app-testing"))]` on its `mod` and `pub use` (spec key `cfg_mod`), so a non-test build has no empty module with an unused `use super::*;`.
- Moved private items, fields and inherent methods are widened to `pub(super)` only when their name occurs in another file of the scope (`widen2.py`; lists `/Users/ueli/.cache/c5c8/stage/*.widen.tsv`: ch 5, pub 2, rc 45, wc 24).
- `pub`'s root keeps `use`/wiring only; `pub(crate) use stall_witness_and_unit_advance::*;` re-exports the two crate-visible helpers.

## 1. Evidence (all on the live tree after the install)

| check | result |
| --- | --- |
| losslessness | `modsplit.py lossless` ch, pub, rc, wc: `missing 0`, every extra line is wiring (`#[path]`, `mod`, `use child::*`), a child docstring, `use super::*;`, balanced region markers (rc 3/3 regions, 3/3 endregions) or the declared `impl` pair of `pub`. `chk.py live mfs`: spliced sequence equal to the snapshot, `gone=0 extra=0`, 20 tests to 20 |
| structure | `vmod.py live ch pub rc wc`: children mounted once, docstring emoji first/equal to the folder/unique, path lengths, brace depth, rustfmt parse, include targets (rc 1/1), `#[test]` multiset, module-name clash: `VMOD OK`. rustfmt `--edition 2024` parse of all 34 touched files: 0 errors. `sibcheck.py`: no sibling emoji clash |
| definition scan | `defscan.py` ch 7, pub 4, rc 9, wc 8 private declarations scanned: 0 cross-file leftovers. `ambig.py live`: no child name is also exported by a root glob. `modclash.py`: OK |
| scoped patched differential | `oracle5.py` (original restored in a sandbox, lib and test): error counts equal: ch 76/78, pub 264/264, rc 278/380, wc 230/319, mfs 630/640; warnings equal for pub, rc, wc, mfs. ch has one extra warning `unused import: super::*` in `🧭️member-keys-and-paths`: oracle artifact, the child uses `store::MEMBER_OPEN_IDENTITY_BYTES`, and `store` is the root's `extern crate ... as store;` alias that the oracle strips. Log `c5c8-oracle-live.log` |
| typeck oracle | `posrun.py` family `c5c8`, labels `pre` (before the install) and `post`: typeck canary (E0308, E0616, E0624) reached before and after; privacy 0/0, trait-scope 0/0, resolve 38/38 (lib) and 40/40 (test), type 166/166 and 201/201, other 27/27. `posdiff.py`: 4704/4704 lib rows and 5080/5080 test rows; the only diff is 5 rows per cfg whose stub trait name changed (`MutationDiff` and `ChildContentRetainers` to `Tr`). Logs `c5c8-pos8-pre.log`, `c5c8-pos8-post.log` |
| formatting debt | `fmt.py` hunks original/new: ch 35/33, pub 43/47, rc 55/54, wc 58/57; changed-line multisets identical (pub: the 4 impl-split boundaries) |
| census (files / bytes / active lines, lib; test closure in brackets) | mfs 15 to 22 / 156,015 / 1,894 (0.688); pub 8 to 13 / 131,272 / 1,608 (0.501); rc 8 to 10 / 145,631 / 1,826 (test 19 to 22 / 183,274 / 2,299: 0.699); ch 1 to 6 / 67,628 / 1,112 (0.281); wc 7 to 11 / 194,777 / 2,890 (test 9 to 13 / 222,257 / 3,229: 0.848). Every closure inside the gate (worst 0.848). Log `c5c8-final-checks.log` |

## 2. Source-text readers re-pointed

`bun` could not resolve its workspace dependencies during the slice (`/node_modules` is a dangling symlink into the rebuilt `.cache/b/d/m`). Tests were run through `c5c8-tools/bt.sh`: `bun --tsconfig-override=tsconfig.override.json test`, a path map of the 293 `@semio-tech/*` workspace packages plus a scratch `node_modules` (ajv 8.20, web-tree-sitter 0.20.8, tree-sitter-wasms 0.1.13, smol-toml, decimal.js, typescript) copied from the bun install cache into `/Users/ueli/.cache/c5c8/nm`. The baseline of `📦️units` reproduced exactly (7 pass, 6 fail before and after).

| reader | edit | evidence |
| --- | --- | --- |
| `PL/🛠️tool-machine/♻️retirement/🧪️tests/🟦️.ts` (wc) | `readFileSync` to `readModuleTreeSource` | 9 pass |
| `PL/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🟦️.ts` (wc) | same | 30 pass, 1 skip |
| `PL/🪟️window/🎚️config/🧬️preparation/🧪️tests/🟦️.ts` (wc) | `Bun.file(...).text()` to `readModuleTreeSource` (import added) | 1 pass |
| `🧰️framework/🔨️modules/🧬️schema/🧩️composition/📥️admission/🧪️tests/🟦️.ts` (ch) | `child=readModuleTreeSource(...)` | the `child` test passes; the caller-rows test fails first on the peer file `➗️equation/🦀️.rs` (known from CENSUS5-B), the ch row checked separately: tree contains `semio_framework_composition_admission::`, no `store::ChildRestoreProjection` |
| `PL/🎮️command/🎟️lease/🧪️tests/🟦️.ts` (pub) | the tree-sitter grammar loop lists the 5 children after the root | 3 pass |
| `PL/🧵️retained-command/🎟️admission/🧪️tests/🟦️.ts`, `PL/🧵️retained-command/🧪️tests/♻️raw-allocation-close/🟦️.ts` (rc) | root read to `readModuleTreeSource` | 2 pass, 2 pass |
| `PL/🧪️tests/🧪️artifact-admission-and-completion-oracles/🟦️.ts` (rc) | `row.source` and `binding.source` read through `readModuleTreeSource` (import added) | probe `c5c8-tools/completion-probe.test.ts`: the rc rows `puzzle` and `writer` and rows `draw`, `puzzle5d-copy`, `puzzle5d-cut` pass; the probe stops at `puzzle5d-paste` (`sourceStart` absent from the peer file in the unsplit tree too) |
| `🧰️framework/🔨️modules/🧵️job/🏃️work/🎟️grant/🧪️tests/🟦️.ts` (rc) | `Bun.file(...).text()` to `readModuleTreeSource` (import added) | the host-demand test fails on the anchor `let demand = match work.work_demands`, which is absent from the snapshot and from `git HEAD` too (stale before the split) |
| `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/📜️script.ts` (rc) | `Bun.file(...).text()` to `readModuleTreeSource` (import added) | not run (needs cargo); the asserted bilingual preview string is present in the tree (`readModuleTreeSource` probe: true) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts`, `.../🎛️commands/✅️verify/🟦️.ts` (rc) | `policyReadFileSafe` to `policyReadModuleTreeSource` for the retained-command root | extracted `toolJobArtifactRetainedCommandExact` (`c5c8-tools/tooljob-law.test.ts`): 39 required tokens, the same 10 missing in the snapshot and in the tree, so the law was stale before the split and is unchanged by it |
| `PL/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts` (rc) | `rawSource` read through `readModuleTreeSource` | the self-test fails earlier on the plugin-root `exact(source)` anchors (reported stale by CENSUS5-A); line not reached |
| `PL/🧪️tests/📦️units/🟦️.ts` (mfs) | `publicCodecTypes` and `pub(crate) count: i32` reads of the `original` file go through `readModuleTreeSource` | the law body up to the closure walk passes (it fails at the foreign `EISDIR`: `#[path="."]` in `🪟️surface-fixture/🦀️.rs`, same as before) |
| `PL/🧪️tests/📦️units/📋️surface-fixture.json` and `🧬️surface-fixture.json` (mfs) | **re-recorded** by `rerecord_mfs.py`: `files[0]` keeps the root path (no functions except the metadata law), the 7 fragments are appended in include order (14 files, schema `minItems`/`maxItems` 7 to 14); 124 function shas | proof per function: text identical (116) or identical after reversing the `../` rebase of an `include_str!` literal (8); assertion total 186 and the 22 laws kept |

Not edited, checked: `🧪️tests/⏳️completion` fixtures and `🧷️companions/🧯️test-support/🧪️tests/🦀️.rs` already read module trees; `#[path]` mounts of the rc root in `🧷️companions/🍎️tests-01` and `🪝️mounts/👥️wp3-owners` keep working because the children mount relative to the root.

## 3. Open risks

1. **Compile not run.** The coordinator must run `cargo check -p <crate> --lib --tests` for the five crates below. Static oracles show equal resolve/type/privacy rows, but they do not model trait coherence or macro hygiene.
2. `pub`: the `impl TypedCtx<...>` header is repeated in 4 impl blocks across 5 files; inherent methods called across children were widened by name (2 members). A missed widening would show as E0624 in the real compile.
3. `rc`: `mod raw_allocation_close_oracle;` is cfg-gated like its item; `retained_command_job` re-exports with `pub use ...::*;` (all items pub or crate-visible as before).
4. **Foreign test failures, unchanged**: units law (`EISDIR`), composition callers law (`➗️equation`), grant host-demand anchor, completion oracle rows `puzzle5d-paste` and `puzzle5d-import`, the tool-job retained-command law and `toolJobLatestWinsSelfTests`.
5. `pai` remains over 1,000 lines until the `PluginApp` trait is split by roles.
6. Incident, repaired: while copying tools I overwrote `F/c5c-tools/orig/SHA256` with the six package-8 lines (a relative `cd` failed). I rewrote the file byte-identically from the earlier read (1,013 bytes) and checked all 14 hashes with `shasum -a 256 -c`: OK. The files of `F/c5c-tools` are otherwise untouched.
7. Peer files modified in the same folders during the slice and not touched: `PL/🧵️retained-command/📸️checkpoint/👣️cursor/🧪️tests/🟦️.ts`, `.../🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts`, `.../🪟️mounted/♻️frontier/🧪️tests/🟦️.ts`.

## 4. Crates the coordinator must compile

`cargo check -p <crate> --lib --tests` for:
- `semio-framework-plugin-surface-fixture` (mfs)
- `semio-framework-plugin-instance-typed-ops` (pub)
- `semio-framework-plugin-retained-command` (rc)
- `semio-framework-plugin-child` (ch)
- `semio-framework-plugin-window-config` (wc)

(Not `semio-framework-plugin-instance-app`: `pai` was not touched.)

## 5. Files

- Created: 24 child files `PL/**/<emoji>️<kebab>/🦀️.rs` (mfs 7, pub 5, rc 3, ch 5, wc 4), `F/c5c8-tools/` (specs `ch,pub,rc,wc`, `rerecord_mfs.py`, `fnrows-text.ts`, `completion-probe.test.ts`, `tooljob-law.test.ts`, `bt.sh`, `tsconfig.override.json`, copies of the c5c tools, `orig/` snapshots with `SHA256`), `F/🗑️generated/c5c8-*`, this file.
- Edited: the five roots (mfs, pub, rc, ch, wc), the readers and records of section 2.
- Untouched: `pai` and every file of packages 5, 6, 7 and 9.
