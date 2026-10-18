# Result C6STORE (census 6 packages P38 to P42, rust os/store)

Executor `c6store` (Sonnet 5.5, foreground, no sub-agents, no modifying git command, no build). `C` = this folder. Tools: `C/c6store-tools/` (descendant of `c6db1-tools`/`c6db2-tools`). Sandboxes and stage trees: `/Users/ueli/.cache/c6store/` (outside the repo). Nothing was compiled; every claim below is static evidence.

## 0. Verdict

**DONE (static evidence) for packages P38, P39, P40, P41 and P42: 27 of 27 files split.** 46,333 lines became 146 cohesive children (146 new folders, all RGI, all resolved by the live taxonomy: `tax.ts` 146 dirs, 0 unresolved). No file is over 1,000 lines any more: largest child 659 lines (`hyd` `step`, one 649-line method that cannot be cut without rewriting it), then 618 (`b05`), 575 (`core`); every other child is at most 530 lines. The source gate holds for every owning crate (`census-store.ts`, canonical `readRustSourceClosureV1`): worst ratio 0.906 (`tests-05`, 29 of 32 files), then 0.882 (`sync-native` test, active lines, unchanged by the split), 0.86 (`sync-tests-1`).

## 1. Per file (before to after, lines)

| tag | file (tail under `os/🔨️modules/🏪️store/`) | before | root | children (max) | mechanism |
| --- | --- | ---: | ---: | --- | --- |
| live | `⚙️engine/🎞️live/📥️lifted/🏪️store` | 2,670 | 94 | 8 (516) | `#[path]` modules, 2 impl blocks cut into 4 + 3 segments |
| core | `⚙️engine/🧱️core/📥️lifted/🏪️store` | 2,637 | 102 | 7 (575) | modules, 1 impl cut |
| sunit | `🔄️sync/🧪️tests/🔬️unit` | 2,858 | 35 | 10 (519) | two-level `include!` fragments (7 + the 908-line `actor_tests` module cut again into 3) |
| b06 | `🧪️tests/🌴️bin-06/🔬️unit` | 1,499 | 12 | 6 (366) | `include!` fragments |
| elift | `⚙️engine/📥️lifted/🏪️store` | 2,508 | 114 | 7 (475) | modules, 2 impls cut into 6 |
| cos | `📦️codec/🧬️owned-schema` | 2,621 | 90 | 8 (522) | modules (crate root) |
| sync | `🔄️sync` | 2,180 | 312 | 9 (377) | modules (crate root, inline `fixtures` stays) |
| b03 | `🧪️tests/🌈️bin-03/🔬️unit` | 2,489 | 13 | 7 (513) | fragments |
| cfg | `🎚️config/📥️retained` | 1,055 | 27 | 3 (468) | modules, 1 impl cut |
| cmd | `📬️command` | 1,105 | 55 | 4 (421) | modules (crate root, test allocator static stays) |
| rpl | `🔁️replay` | 1,179 | 74 | 4 (429) | modules (crate root) |
| wac | `🔄️sync/🌐️wasm/🎭️actor` | 1,051 | 30 | 4 (332) | modules, `nest`, 1 impl cut in 3 (region edges) |
| rd | `🔗️read` | 1,100 | 91 | 3 (366) | modules (crate root) |
| str | `🦀️.rs` (store facade) | 1,035 | 538 | 2 (318) | the two `#[macro_export]` macro groups moved behind `#[macro_use] mod` |
| dgw | `🧩️composition/🗄️durable-group/📜️wire/📥️lifted` | 1,125 | 68 | 5 (418) | modules |
| osf | `📦️codec/🧬️owned-schema/🧱️fields/📥️lifted` | 1,485 | 63 | 4 (453) | modules |
| nac | `🔄️sync/🖥️native/🎭️actor` | 2,475 | 67 | 8 (508) | modules, `nest`, `ArtifactActor` impl (1,416 lines) cut into 6 with `Hub` region closed and reopened at 2 cuts |
| dg | `🧩️composition/🗄️durable-group` | 2,138 | 88 | 6 (424) | modules (crate root) |
| b05 | `🧪️tests/🌅️bin-05/🔬️unit` | 1,861 | 25 | 4 (618) | fragments (macro `with_fixture_identity!` stays in the root) |
| b04 | `🧪️tests/🌌️bin-04/🔬️unit` | 2,000 | 13 | 7 (352) | fragments |
| iot | `🚪️io/📝️text` | 1,433 | 73 | 4 (466) | modules |
| smi | `🧩️composition/👥️space-member/📥️lifted` | 1,211 | 86 | 4 (512) | modules |
| dgt | `🧩️composition/🗄️durable-group/🧪️tests/🔬️unit` | 1,293 | 19 | 5 (313) | fragments |
| b01 | `🧪️tests/🏙️bin-01/🔬️unit` | 1,368 | 12 | 5 (329) | fragments |
| ssr | `🧪️tests/🧪️supersede-replay` | 1,450 | 11 | 6 (361) | fragments |
| hyd | `🧾️document/📜️history/💧️hydration` | 1,264 | 38 | 3 (659) | modules, 1 impl cut before the 649-line `step` |
| eph | `🫧️ephemeral` | 1,243 | 61 | 3 (504) | modules (crate root) |

Children are `<emoji>️<kebab>/🦀️.rs` next to their root; module children are mounted with `#[path]` and re-exported by **explicit** lists (`pub use child::{A, B}`; no glob re-export anywhere; non-`pub` items are re-exported privately only when the root or a sibling names them, `#[cfg(test)]` when only tests name them, `#[cfg(any(test, feature = "testing"))]` for the native fixtures). Roots keep file mounts (`#[path]` tests), inline modules whose region markers span a cut (`sync::fixtures`), the `#[global_allocator]` test statics and `use` blocks.

## 2. Decisions and deviations (all declared to the lossless oracle)

- `self::Mutation` in generic bounds (`Mutation: ... + self::Mutation<P>`) names the imported trait; in a child `self::` would be the child module, so every `self::` of a moved line became `super::` (`rebase_self`; 88 occurrences in live 10, core 15, elift 11, rpl 26, iot 19, eph 6, smi 1).
- Private items, fields and inherent methods whose name occurs in another file of the split became `pub(super)` (widen counts: live 18, core 7, elift 36, cos 14, sync 6, cfg 22, cmd 1, rpl 9, wac 19, dgw 14, osf 7, nac 106, dg 16, hyd 6, eph 56). `nest` (original `pub(super)` becomes `pub(in super::super)`) is on for wac and nac.
- **Pre-existing defect fixed (one line):** `dg` held `pub(super) struct DurableOwnedThreeStoreCommitV1` in a crate root; rustc refuses it (E0433 "too many leading `super` keywords within `crate`", reproduced on the unsplit file in the oracle). It is now `pub(crate)`. Declared as `expect_missing`/`expect_extra` in `specs/dg.py`.
- Impl blocks were cut at method boundaries into several `impl` blocks of the same header (`IS` helper in `specs/lib.py`, declared extra lines); where a `//#region` spans a cut (nac `Hub`) the region is closed before the `}` and reopened after the header (balanced markers in every child).
- `str` moved the macros `field_set_mutations!`, `config_record!`, `config_diff!`, `space_members!` into two children mounted with `#[macro_use]` at the macros' original positions. They are `#[macro_export]`, so `$crate::`/crate-root paths and every external `store::space_members!` use are unchanged; no macro is used textually inside the crate.
- `nac` keeps `include!("../🧪️tests/🔬️retained-turn-fixtures/🦀️.rs")` in the root; that file is an inline module with `use super::*` that builds `ActorRunner { ... }`, so its field names were widened and the three names only it needs are re-exported under `#[cfg(any(test, feature = "testing"))]`.
- Two-level fragments: `sunit` pass 1 cut the root into 7 fragments, pass 2 (`sunitm`) cut the 908-line `actor_tests` module body (include fragments inside the module).
- Cannot be cut losslessly: `hyd` `RetainedPersistedDocumentHydration::step` (649 lines) stays whole in its own child (659 lines).

## 3. Evidence (all run on the live tree after the installs)

| check | result |
| --- | --- |
| losslessness, module splits | `modsplit.py lossless`: `missing 0` for 19 of 19; every extra line is wiring, docstring, `use super::*;`, explicit `pub use` lists, marker, declared impl header/brace or the declared `dg` visibility line; regions balanced per child |
| losslessness, fragments | `chk.py live` (pass 1) and `chk-deep.py` (expands both levels): `gone=0 extra(non-marker)=0`, deep line sequence EQUAL and `#[test]` multiset equal for sunit 48, b06 34, b03 57, b05 70, b04 84, dgt 21, b01 39, ssr 28 (381 tests before and after) |
| structure | `vmod.py live`: every child mounted once, docstring emoji first/equal to folder/unique/different from root and from existing siblings, path length (max 194 absolute chars, 226 repo-relative bytes), brace depth 0, rustfmt parse of every file, include targets equal, no `[DEBUG]`: `VMOD OK` 19 of 19 |
| definition scan | `defscan.py`: 0 cross-file private leftovers in 19 of 19 (multi-line impl headers were a scanner blind spot; fixed) |
| glob ambiguity | `ambig.py`: no child item name collides with a root glob source (explicit re-exports shadow globs anyway) |
| taxonomy and names | `rgi.mjs` bad=0 for 146 names, `tax.ts` 146 dirs 0 unresolved, emoji all in the repo folder-emoji set |
| census | `census-store.ts` per crate, lib and test closure: files at most 32 (`tests-05` 29), bytes at most 0.86 of 256 KiB, active lines at most 0.882 of 4,000 (nac test, unchanged) |
| scoped patched differential | `oracle5.py` (original restored in one sandbox, live tree in the other, `rustc +nightly-2026-07-20 --emit=metadata`, lib and `--test`, extern crates and external globs neutralised identically): error and warning multisets equal for all groups; the only message differences were explained: sync `unused import envelope_serde` (used by serde `#[value(... = "envelope_serde::...")]` attribute strings the oracle does not expand), hyd one extra unresolved extern-path error (the impl header with that path now appears twice), dg one fewer error (the fixed `pub(super)` at the crate root) |
| not covered by rustc here | `wac` (wasm32-only), `nac` fixtures under `feature = "testing"`; the real compile has to confirm them |

## 4. Source-text readers (found by path greps over every tracked `ts tsx json py mjs rs`, by relative-literal resolution `relreaders.py`, by string-literal impact analysis `needs-tree.py`)

Re-pointed to the module tree (`readModuleTreeSource`, import added where missing; unused `readFileSync` imports removed):
- Store: `♻️retirement/📦️backing`, `🔁️replay/🛑️cancel`, `🔄️sync/🪪️actor-identity`, `🔗️backbone/♻️retirement`, `🕰️history/📐️planning` (+ `🎟️admission`), `🚪️io/🚫️refusal/📬️outcome`, `🧩️composition` (`📨️emission/📦️owned` + `🌱️birth`, `📬️publication/🤝️group` unit, `🚪️open` `📏️birth` unit, `🚪️open/🧪️tests/🔬️unit/📏️retirement` 18 sites, `🗄️durable-group/🧪️tests/♻️journal`, `🚪️open/📜️history/🏭️factory/🛬️receiving`), `🧪️tests/🔬️unit` (supersede replay), `🧪️tests/🗄️durable-owned-group`, `🧪️tests/🗣️member-dialect`, `🧪️tests/🧪️supersede-replay`, `🧵️canonical-edit/🔬️canonical-error-progress`, `📜️space-history/.../📸️snapshot` (hydration parse and bin-06 parse), `🏗️initialization/.../📥️source` self-test (read and bin-05).
- Outside the store: flow editor retirement, stdio registry editing unit, hub (gis durable assembly, space script, execution-target lease, check-in-process), replication (foreign mutation, input drafts), composition admission, job (retirement context, loan), os typescript script, retained-clone-check, renderer frame-close, plugin reactor-contract-oracles, host member-factory-identity and member-open.
- Policy evidence: `OS_STORE_ROOT_SOURCES` lists every child after its root (str 2, read 3, command 4, replay 4, owned-schema 8, text 4, engine 7, engine-core 7, engine-live 8); `POLICY_SPLIT_SOURCE_ROOTS` gained the sync root, the read root and the bin-05 unit (read through `policyReadRustSourceEvidence`: interactivity audit store sync law, member producer source roles).
- Rust source pins (`include_str!` of split roots): the wasm and native `echo-suppression-pin` and the sync `document-echo-suppression` now include every child then the root through `concat!`.
- Left alone on purpose: rustfmt-only loops over roots (still parse), laws that only check `pub use` lines of the store facade, path-presence checks (`cache-contracts`, `crate-boundaries` fixtures, vcs receiving fixture), the `history-closure-policy` virtual file map, comments.
- Stale before the split (not touched): laws that read the store facade for items that were already carved out of it (`📜️space-history/.../📸️snapshot` lines 79, 92, 175, 232 `sourceTokens`, `🚪️open/.../📏️retirement` lines 498 and 1395, the `store.tests` check of `policyReadMemberStoreProducerSources`).
- **None of the TS laws could be run**: `node_modules` is not installed in this checkout (`bun` cannot resolve `@semio-tech/*`), so these edits are static. Slice-based laws keep working where the child order equals the original order; the order changed only for live (children 3 and 4), replay (children 2, 3, 4) and nac (child 3), and no reader slices across those groups.

## 5. Crates to compile (coordinator)

`cargo check -p <crate> --lib --tests` (via `bun repo cargo --store=shared --configuration=prototype`):

```
semio-framework-os-store-engine-live semio-framework-os-store-engine-core semio-framework-os-store-engine
semio-framework-os-store-owned-schema-decode semio-framework-os-store-owned-schema-fields
semio-framework-os-store-sync semio-framework-os-store-sync-native semio-framework-os-store-sync-tests-1 semio-framework-os-store-sync-tests-2
semio-framework-os-store-config semio-framework-os-store-command semio-framework-os-store-replay semio-framework-os-store-read
semio-framework-os-store-durable-group semio-framework-os-store-durable-group-wire semio-framework-os-store-durable-group-tests
semio-framework-os-store-text-io semio-framework-os-store-space-member semio-framework-os-store-document semio-framework-os-store-ephemeral
semio-framework-os-store-tests-01 semio-framework-os-store-tests-03 semio-framework-os-store-tests-04 semio-framework-os-store-tests-05 semio-framework-os-store-tests-06
semio-framework-os-kernel
```
Plus `semio-framework-os-store-sync-wasm` on `--target wasm32-unknown-unknown` (and wasip2 where the workspace builds it). The `sync-native` test with `--features testing` covers the gated fixture re-exports.

## 6. Open risks

- Compile level: the oracle proves name resolution and structure only; trait-scope, privacy and type errors need the real check. Likeliest failure classes: a missed `pub(super)` widening of a private name used only through a macro-generated path or a string-keyed attribute (`#[value(serialize_with = "...")]`), a duplicate private re-export (E0252) between two children, `unused_imports` warnings on private re-exports used only by derive-expanded code.
- The two-level `include!` inside `mod actor_tests` (`sunit`) relies on `include!` resolving relative to the including fragment file; verified by path arithmetic (`chk.py` include targets exist), not by compiling.
- `hyd` child 659 lines and `b05` child 618 lines are above the 600-line target but below the 1,000-line rule.

## 7. Files

- Edited (27 roots): the files of section 1. Created: 146 `🦀️.rs` children, one per new folder next to its root (list: `C/c6store-tools/newdirs.txt`).
- Edited (readers): section 4. Edited (Rust pins): 3 echo-suppression files.
- Tools, kept: `C/c6store-tools/` (`modsplit.py` with explicit re-exports, `IS` impl splitter, `rebase_self`, `nest`, `external_gated`, unclosed-region tolerance; `frag.py` with keep rows and inner-module scope; `run.py`, `chk.py`, `chk-deep.py`, `vmod.py`, `defscan.py` (multi-line impl fix), `ambig.py`, `oracle5.py` (dynamic groups), `census-store.ts`, `tsrepoint.py`, `tsfix.py`, `needs-tree.py`, `relreaders.py`, `crates.py`, `specs/*.py`, `specs5.py`, `orig/` snapshots with SHA256, `staged-*.json`).
