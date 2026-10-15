# Result: plugin instance (A) and plugin host (B) split

Executor: Sonnet 5.5. Date: 2026-10-15. Seam map: `SEAM-MAP-PLUGIN-MANIFEST-HUB.md` sections A and B.
Paths: `plugin/` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`.

## 0. Status

Both files are split, pure structural, no public renames. **No compile was possible**: every `bun repo cargo ... check` died in the compiler-ingress gate (`Rust crate Source refused before compiler: semio-framework (Rust crate Source boundary exceeded before compiler byte allocation)`; `semio-framework` has no `rust-source-boundary` metadata and its manifest file alone is 465 KB against the 262144-byte default). Verification is therefore static (section 7): the coordinator's rustc name-resolution differential is identical to the baseline for A (lib, `--test`, wasm32-wasip2) and B (lib, `--test`), plus lossless multiset, rustfmt parse, path and emoji checks. Everything in "Not verified" (section 8) still needs a real `cargo check` once the gate admits the crates.

## 1. Line counts

Active = non-blank, non-comment lines.

| File | Before | After (total) | Active |
| --- | --- | --- | --- |
| A parent `plugin/🧵️instance/🦀️.rs` | 10,927 | 1,256 | 985 |
| A `🚪️ingress` (impl band 1) | | 2,344 | 1,973 |
| A `📨️dispatch` (impl band 2a, history/preview/emit) | | 1,657 | 1,148 |
| A `🖼️render` (impl band 2b, composition/interaction/render/media) | | 1,115 | 881 |
| A `🧷️command-routes` (impl band 3a, reserved routes/action/command) | | 1,009 | 872 |
| A `🧾️typed-operations` (impl band 3b, typed-op publication/typed command/config/history panel) | | 1,844 | 1,528 |
| A `🔌️plugin-app-impl` (free items, 2nd inherent block, `PluginApp` impl) | | 1,742 | 1,478 |
| B parent `plugin/🖥️host/🦀️.rs` | 7,295 | 838 | 490 |
| B `🎭️guest-runtime` | | 2,202 | 1,554 |
| B `🔀️post-turn-relay` | | 2,373 | 2,106 |
| B `🧭️routers` | | 1,901 | 1,337 |

Deviation from the seam map (per the coordinator's "every child at or below about 2,500 active lines" rule): map children X2 (2,762) and X3 (2,843) are each cut in two at a member boundary (cuts at original lines 5196 and 7310, moved up over the member's doc and attribute lines). Every child is at most 2,106 active lines.

## 2. Created and edited files

Created (all start with a `//! <emoji>` docstring, `use super::*;`, then the code; folder names `<emoji>U+FE0F<kebab>`, emojis RGI and already used as folder prefixes in the repo, unique among siblings; longest path 150 bytes):

- `plugin/🧵️instance/🚪️ingress/🦀️.rs`, `📨️dispatch`, `🖼️render`, `🧷️command-routes`, `🧾️typed-operations`, `🔌️plugin-app-impl` (each `🦀️.rs`).
- `plugin/🖥️host/🎭️guest-runtime/🦀️.rs`, `🔀️post-turn-relay`, `🧭️routers`.

Edited:

- `plugin/🧵️instance/🦀️.rs`: lines 1204-9148 and 9166-10904 replaced by five `#[path = "<folder>/🦀️.rs"] mod <snake>;` lines for the impl bands plus `mod plugin_app_impl;` and `pub use plugin_app_impl::*;`. Kept in the parent: imports, structs, helper fns, the two `include!`s (1201-1202, last line), the `Drop` impl, the nine existing `#[path]` mods. Impl bands re-wrap the original header `impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {`; the `Send + 'static` second block and the `PluginApp` impl stay together in `🔌️plugin-app-impl`.
- `plugin/🖥️host/🦀️.rs`: `#[path] mod guest_runtime; pub use guest_runtime::*;` and the same for `post_turn_relay`, `routers`. Kept in the parent: lines 1-601 (mods, shared engine, metrics, session, reports), the dangling doc plus `RuntimeMetricsPublisher` and `ArtifactSession`/`MutationReports` regions, the trailing test mods and the `HostWithStore<ActorHostState>` impl, the `include!` at the old line 596 and the `activation` cfg gate.
- Source-text tests (section 5).

External paths stay stable: `crate::artifact_instance::{MAINTENANCE_STAGES, settle_framework_reserved_admission, settle_framework_reserved_commit, drive_framework_reserved_spawn_like_host (cfg test), FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS, ...}` resolve through the parent's `pub use plugin_app_impl::*;` (glob re-export keeps each item at its own visibility). Host `pub` items (`GuestRuntime`, `WasmtimeRuntime`, `PluginInstanceHandle`, `IoEntryRoute`, ...) resolve through the three `pub use <child>::*;` lines under the lib root's `pub use component::*;`. The `pub(crate) mod actor_bindings` stays inline whole in `🎭️guest-runtime` (the `bindgen!` is not split).

Where the wrong-direction edges the map lists now live (all left untouched):

- `ui_wgpu::wgpu` imports, `tool_run_contract`, `tool_jobs::{time_travel, tool_run}`, `reserved_jobs`, `interaction_topology` imports: still in the A parent's `use` block (inherited by all children through `use super::*`); `is_time_travel_action_id(...)` arm and `tool_jobs` uses at the dispatch head: `🧷️command-routes`; one `ui_wgpu` use inside the `PluginApp` impl: `🔌️plugin-app-impl`.
- `os_vcs` in `GuestRuntime`: trait and impls in `🎭️guest-runtime` (32 mentions), plus the host parent's `pub use os_vcs`-style line (1 mention).
- The `include!`s: A parent (3), B parent (1). Unchanged, paths still resolve because they stay in the parent.

## 3. Path and literal rewrites

- A: none needed. No `#[path]`, `include!`, `include_str!` or `CARGO_MANIFEST_DIR` in any moved A range; the nine `#[path]` mods and the three `include!`s stayed in the parent.
- B: every `#[path = "X"]` inside a moved range got a `../` prefix (18 occurrences; test mods stay with the code they test, so they now see the child's private items): guest-runtime 7 (`🏗️instance`, `🔁️lifecycle/🧪️tests`, `📥️cold-pair`, two `🧪️tests/...`, one more `🧪️tests`), post-turn-relay 3 (`♻️publication/🧪️tests`, `🔔️wake`, `🧪️tests/🔬️guest-cold-relay`), routers 8 test mods (`🧪️tests/...`). `include_str!("⚡️effects/🧪️tests/🧫️fixtures/🔣️.json")` in `post-turn-relay` became `"../⚡️effects/..."`. A script resolved all path literals of the three B children to existing files: 0 missing. The moved path-mods keep their own relative paths; their `super` is now the child module, and `use super::*` chains through the child's glob of the parent.

## 4. Visibility widenings (static, superset, not compiler-driven)

Rules: only `pub(super)`; never wider. Scripts: `🗑️generated/plugin-instance-host/widenA.py`, `widenB.py` (plans and sources there; `widenB.plan.tsv` is the applied B plan).

- A: 106 inherent members widened to `pub(super)` (word-match on any use outside the defining child: other bands, the parent, the nine existing `#[path]` mods (time-travel, tool-run, tool-machine, operation-progress, full-operation, group), the three ladders, the three `include!`d files, test-view). By child: ingress 48, dispatch 17, render 17, command-routes 9, typed-operations 10, plugin-app-impl 5 (consts and free fns, e.g. `history_changed_event`, `PUBLICATION_RETURNED_ROOT_ALLOWANCE`, `MAINTENANCE_CHILD_ROOT_STAGE`). One pre-existing `pub(super) fn test_child_admission_state` (original line 2969) was rewritten to `pub(in super::super)` in `🚪️ingress` so it stays visible in the same module it was visible in before (the module above `artifact_instance`).
- B: 110 items widened to `pub(super)`: guest-runtime 59 (18 top-level items incl. `ActorHostState`, `GuestInstanceState`, `cold_pair`, `owned_compiled`; 21 fields; 18 mock/runtime members; 2 `use` lines `use wasmtime::component::Accessor;` and `use ...::{jobs as wit_jobs, reactor as wit_reactor};` because moved files and the parent impl use those names), post-turn-relay 42 (37 members, 3 items incl. `RELAY_JOB_BUDGET`, 2 fields), routers 9. Fields use "owner type and field name both appear in a different file"; members and items use a name match; this is an over-approximation by design and some widenings may be removable.
- Trait impls and `pub`/`pub(crate)` members were not touched.

## 5. Source-text tests

Rust (both now concatenate the parent and the six A children in file order: parent, ingress, dispatch, render, command-routes, typed-operations, plugin-app-impl; assertions unchanged):

- `plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`: new `fn instance_source() -> String` (docstring `/// 🧵️ ...`) with `include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../...")) x7`; the five `include_str!` reads at the old lines 2804-2821 now call it.
- `plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs`: same helper inside `mod typed_command_full_operation_tests`; three `let source = include_str!(...)` became `instance_source()`, and the `concat` with `🔄️full-operation` became `[include_str!(...).to_string(), instance_source()].concat()`.
- I checked in Python that the three anchored slices (`publish_mounted_typed_child_operation_unit` .. `publish_mounted_typed_operation_unit`, `admit_command_json_with_proof` .. `pub async fn new(app: A, actor`, `dispatch_action` .. `dispatch_command(`) are byte-identical to the original after stripping the added `pub(super)`.

TypeScript (the seam map missed these; found by resolving every relative path literal to the A/B root files, plus the list-based readers): updated with `🗑️generated/plugin-instance-host/tsfix.py` (replaces a single read with the concatenation of parent plus children, or expands the file-name list), 13 files:

- A readers: `♻️cancellation/📦️lease/🧪️tests`, `♻️lifecycle/🏪️stores/🧪️tests`, `🎞️media/⚠️turn/🧪️tests`, `🎮️command/🎟️lease/🧪️tests`, `👷️job/🚪️session/🎟️pending/🧪️tests`, `👷️job/🚪️session/🫴️refusal/🧪️tests`, `🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🧪️tests`, `🧪️tests/🧾️document-archive-load-legs`, `🧵️retained-command/🪟️mounted/♻️frontier/🧪️tests`.
- B readers: `🖥️host/⏱️deadline/🧪️tests`, `♻️publication/🧪️tests`, `🎟️context/🤝️admission/🧪️tests`, `🔔️wake/🧪️tests`.
- **Not touched (forbidden area `⚛️reactor/**`) and now failing more than before**: `⚛️reactor/🎟️context/🌉️call/⚖️failure-debit/🧪️tests/🟦️.ts` (baseline 12 pass / 1 fail, now 10 / 3: `original render context reaches the actual instance before tree publication` and `addressed action dispatch forwards original arguments and whole fault through one context` newly fail) and `⚛️reactor/🚪️lifetime/🏗️opening/🧪️tests/🟦️.ts` (baseline 1 / 0, now 0 / 1). Both read only the A parent. The fix is `cd plugin/ && python3 -I <path to 🗑️generated/plugin-instance-host/tsfix.py>` after removing the `reactor/` skip on line `if f.startswith("⚛️reactor/")` (the reactor owner should run it).

## 6. Pre-existing failures (baseline, not mine)

TS baseline taken before any edit (`🗑️generated/pis-ts-baseline.log`) and, for the files I did not baseline up front, with the original parents temporarily swapped back (`pis-ts-baseline2.log`, a swap of a few seconds, restored and `cmp`-verified). After-run: `pis-ts-after.log`. Results are the same before and after for all 13 edited TS test files: owner 0/1 pass, frontier 6/1, cancellation lease 4/0, doc-archive 1/0, stores 5/3, command lease 3/0, media turn 9/0, job pending 3/1, job refusal 5/0, host deadline 0/1, host publication 0/1, host admission 1/2, host wake 2/1 (pass/fail). The failures were already there.

## 7. Verification actually performed

| Check | Command or method | Result |
| --- | --- | --- |
| Native check, A | `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-plugin --message-format=short` (retry loop, 8+ attempts 02:10 to 02:42; `🗑️generated/pis-baseline-native.log` and `.attempts`) | **Not run to rustc.** First `cargo fetch --locked ... cannot update the lock file` (peer adding a member), then a peer's mid-edit `Cannot find module '../../../📦️sdk/🧊️binary/🔎️verification/🟦️.ts'`, then every attempt `Rust crate Source refused before compiler: semio-framework`. Stopped on the coordinator's instruction. |
| wasm32-wasip2 / `--tests` / host native / host `--tests` | same command shape | **Not run** (same gate). |
| `cargo test -p semio-framework-plugin --lib` and the two substring tests | | **Not run** (cannot compile). |
| Name-resolution differential | `rustc +nightly-2026-07-20 -Zunpretty=expanded --crate-name probe --edition 2021 --crate-type lib [--test] [--target wasm32-wasip2] <file>`, original in a symlink sandbox vs. split; compare sorted `^(error\|warning)` header multisets (`🗑️generated/plugin-instance-host/orc.sh`, `mksbx.py`) | A lib 370 errors, identical; A wasm32-wasip2 identical; A `--test`: identical except `E0433 too many leading super keywords` 5 vs 4 `within crate` + 1 `within crate::ingress`, which is the one `pub(in super::super)` line (it errors at the probe's crate root, valid inside the real module tree). B lib 1,386 errors, identical; B `--test` 2,853, identical. Re-run against the final real trees: same results. The probe found two real faults on the way (private imports `Accessor`, `wit_jobs`, `wit_reactor` not visible to moved files), both fixed. |
| Lossless | multiset of non-blank lines, original vs parent + children (after undoing `pub(super)` and the `../` path prefixes) | 0 lines lost. A extras: the six `mod`/`#[path]` pairs, `pub use`, six docstrings, `use super::*;` x6, 4 extra `impl ... {` / `}` pairs of the re-wrapped bands. B extras: three `#[path]`/`mod`/`pub use` triples, three docstrings, `use super::*;` x3. |
| Syntax | `rustfmt --edition 2021 --emit stdout` on every new file and both parents | 11/11 exit 0. |
| Paths | script resolving all `#[path]`, `include!`, `include_str!` in moved B code | 0 missing; A has none. |
| Visibility closure | `widenA.py` dry run on the final A tree | 0 further promotions. (A re-run of `widenB.py` on the final B tree mis-reported because of a bug in its descendant map when run in place; I had run it with `--apply` by mistake, which widened extra items; I restored all three B children from the verified sandbox copy and `cmp`-checked them. Final B state equals the sandbox state that passed the differential.) |
| Emoji admission | bun `/^\p{RGI_Emoji}$/v` on the nine folder emojis | 9/9. |
| Path length | `wc -c` on every new path | max 150 bytes. |
| Source-text laws (TS, 15 files incl. 2 reactor) | `bun test ./<path>` | section 6; reactor regression in section 5. |
| Source-text laws (Rust, 2 tests) | | not run (no compile). |

## 8. Not verified, and the risks

- Method and field privacy (the differential cannot see it). Likely residual errors: a private member or field used from a sibling or from a path-mod of the instance (time-travel and so on), or a B test that reaches a private field of a moved struct. Run the check, then `fixvis.py <json-log> super` (in `🗑️generated/plugin-instance-host/`) inserts `pub(super)` at the definition named by E0603/E0624/E0616/E0451.
- Type privacy: `private_interfaces` warnings, unused imports and `dead_code` from the parent's `use` block are expected to be warnings only; workspace lints were not compiled.
- Glob re-exports `pub use <child>::*;`: no ambiguity reported by the probe, but E0659 would appear only under real name resolution.
- B `ActorHostState` stays reachable to the parent's `HostWithStore<ActorHostState>` impl only through `pub use guest_runtime::*;` and its new `pub(super)`.
- The cfg/target arms (wasm32 at old A 1373, 1384, 7349) moved whole inside `🚪️ingress` / `🧷️command-routes`; compile for wasm32-wasip2 is the one check that matters most for guest plugins.
- Stale line-number comments elsewhere citing the old files are not refreshed.
- Peers' concurrent edit to `plugin/🖥️host/🧪️tests/🔬️unit/🦀️.rs` (`s.stdio.binary` to `framework.io.binary`) is not mine and was left alone.

## 9. How to finish once a compiler is available

1. `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-plugin --message-format=json > log` then `--target wasm32-wasip2`, `--tests`, and the same for `semio-framework-plugin-host` (native, `--tests`).
2. `python3 -I fixvis.py <log> super` until clean; re-run the checks.
3. `-- test -p semio-framework-plugin --lib`, then the targets `plugin-runtime-plugin-builder-contract` and `app-typed-command-full-operation` (find the exact test target names in `plugin/📦️packages/🦀️rust/Cargo.toml`).
4. Optional narrowing of widenings no error requires.

## 10. Artifacts

`🗑️generated/plugin-instance-host/`: `instance-original.rs.txt`, `host-original.rs.txt` (untouched originals), `splitA.py`, `splitB.py`, `widenA.py`, `widenB.py`, `widenB.plan.tsv`, `tsfix.py`, `fixvis.py`, `orc.sh`, `mksbx.py`. Logs: `🗑️generated/pis-baseline-native.log(.attempts)`, `pis-ts-baseline.log`, `pis-ts-baseline2.log`, `pis-ts-after.log`.

## Hardening (full-resolution differential, 2026-10-15)

Full configs, baseline construction, scan tool and residual risks: `RESULT-HARDEN-MANIFEST-PLUGIN-HUB.md`.

**Instance (A)**
- Oracle from the real crate root `🔌️plugin/📦️packages/🦀️rust/🦀️.rs` (crate `semio_framework_plugin`), baseline = sandbox with `🧵️instance/🦀️.rs` = `git show HEAD:` (identical to `instance-original.rs.txt`) with the peer's 7-line `store::ChildRestoreProjection` to `semio_framework_composition_admission::ChildRestoreProjection` substitution. Coordinator patches applied identically to both trees (5 `extern crate` lines removed, 26 external globs such as `use semio_framework_ui_contract::*;` in the instance parent neutralised). Without the glob patch the same oracle printed an equal 1361 / 1361 and was blind.
- Regression found: 12 errors `cannot find child_emit_preparation / window_mutation / window_transient in super` (native 9236 / 9248). Cause: `super::X` paths in code moved one level deeper. Fixed by `super::` to `super::super::` at `🧾️typed-operations/🦀️.rs` lines 657, 659, 660, 891, 897, 1063, 1069, 1241, 1247 (12 occurrences).
- Final, baseline / split, 0 differing keys: native 9236 / 9236, `--test` 20822 / 20822, wasm32-wasip2 9527 / 9527, `--test` wasm32-wasip2 21107 / 21107, all four feature cfgs 10084 / 10084, features + `--test` 20845 / 20845.
- Definition scan over the instance subtree (448 modules): 0 candidates for private fields, methods, items or traits defined in the six children (seeded sensitivity: stripping `pub(super)` from the 17 widened methods in `📨️dispatch` reproduces all 17). Residual: typeck-phase privacy unproven by rustc.

**Host (B)**
- Oracle from `🖥️host/📦️packages/🦀️rust/🦀️.rs` (crate `semio_framework_plugin_host`, native only), baseline = `git show HEAD:` host (identical to `host-original.rs.txt`), 3 `extern crate` lines removed, 0 external globs. Final, baseline / split: native 1334 / 1334, `--test` 2796 / 2796, 0 differing keys. No edit.
- Definition scan: 433 candidate groups, all collisions on common field names (`state`, `owner`, `generation`, ...) and one `Self::new`; the only family-typed access (`ActorHostState` fields in `📥️imports`) is on `AsyncActorHostState`, a different struct. 0 real.

**Task C** (`store::ChildRestoreProjection` to the new crate in `🚪️ingress` and `📨️dispatch`): a peer's deliberate concurrent change, left as is. New crate `semio-framework-composition-admission` created 02:51 to 02:56, Cargo dependency added 02:53 (uncommitted), `store`'s own definition removed (136 deleted lines), non-split files switched identically, children touched 02:53 after the 02:50 split.
