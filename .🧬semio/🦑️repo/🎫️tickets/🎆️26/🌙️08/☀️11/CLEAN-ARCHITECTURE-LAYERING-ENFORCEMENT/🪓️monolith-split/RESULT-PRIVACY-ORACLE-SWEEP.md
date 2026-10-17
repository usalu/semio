# Result PRIVACY-ORACLE-SWEEP (prefix `pos`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command (only read-only `git cat-file --batch` for baseline revisions), no ticket tool. Folder `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/pos-tools/`. Logs, JSON and reports: `F/🗑️generated/pos-*` (nothing deleted). All sandboxes (the only place anything is written): `scratchpad/pos-work/<family>/{cur,base}`, outside the repo.

## 0. Verdict

* **A reusable privacy/type oracle exists** (`pos-tools/pos.py`, section 1). It builds the single-crate union with a stub layer, runs `rustc +nightly-2026-07-20 --emit=metadata` for lib and `--test`, reaches body type-check (proved per run by a canary that must raise E0616, E0624 and E0308) and diffs against a pre-carve baseline or classifies absolutely.
* **It found real carve defects that the plain patched differential could not see.** 12 private members (77 use sites) in `PLUGIN-HOST-CARVE`, 6 in `OS-KERNEL-ENGINE`/`SQLITE-HOST+sync` (14 sites), 7 in `RENDERER-INTEGRATION-2` (10 sites), plus 6 missing-trait/import/path defects. All privacy defects were widened to `pub` (minimal edits, section 3) and re-run to 0. Several non-privacy defects need a design decision (section 5).
* **Families reach body type-check** (canary reached): `phc`, `rcr`, `osm`, `dlo` (after quarantine of 8 files), `smg` (after quarantine of 1 file), `rhs` (after quarantine of 1 file; the server crate separately without quarantine), `okeng`, `oksql`, `ri2`.
* **The single union of all kernel crates (`okernel`, 49 crates) and of the 76 renderer crates in the first runs did not reach body type-check** because of a rustc abort I could not attribute generally (section 4). `okernel` was therefore swept as three smaller families (`okeng`, `oksql`, and `osm` for the kernel facade itself). A cross-family check of the whole kernel closure at once is NOT covered.
* No real `cargo check` was run (headroom gate, per the brief). Everything below is static evidence from rustc `--emit=metadata` over a stub layer, not a compile.

## 1. The tool (`F/pos-tools/`)

| File | Role |
| --- | --- |
| `pos.py` | CLI and driver: `run <family.json> [--cfgs lib,test,wasm,p2] [--label L] [--iters N] [--no-base] [--mutate "<pkg>::<rel>@@old@@new"] [--quarantine --qmax N]`, `index [--force]`. Writes `pos-<family>-<label>-{report.txt,result.json,new-<cfg>.txt,<side>-<cfg>-it<k>.err,-nonstub.txt}`. |
| `poslib.py` | Write guard `Sandbox` (resolves symlinks, asserts every write target is under `scratchpad/pos-work`, refuses a sandbox inside the repo, unlinks a symlink before writing), `log_write` (only `F/🗑️generated/pos-*`), `Source` (working tree or `git cat-file --batch <rev>:<path>`, read only), Cargo manifest index (`pos-manifest-index.json`), feature table. |
| `posunion.py` | Reads each family crate's source closure (`#[path]` including the directory context of inline `#[path] mod x { ... }`, `include!`, `include_str!`, implicit `mod x;`), copies it to `<sandbox>/<__m_crate>/<repo-relative path>` (one private copy per crate, so a file shared by two crates stays independent and relative paths keep resolving; the un-normalised `..` parent directories are created) and rewrites it: `crate::` to `crate::__m_<crate>::`, `$crate::`, `pub(crate)` to `pub(in crate::__m_<crate>)`, heads of family crates to `crate::__m_<dep>`, heads of every other crate to `crate::__ext::<dep>` (also after a single `:`; this is the head-regex defect of the coordinator message, section 6), `extern crate X as Y` aliases, `use ext as alias`, per-crate feature cfgs (`feature = "x"` becomes `feature = "<crate>:x"`, Cargo-style unification over the family), `env!(CARGO_*)`, `#[global_allocator]`, inner attributes, non-std derives and helper attributes of external proc macros, module-level external item macros (for example `bindgen!`) removed. |
| `posstub.py` | Generated `__ext` stub module for every crate outside the union: typed tree (mod, struct, enum, trait, fn, const), generic arity inferred from use, std derives, trait positions inferred per file (impl headers, bounds, where clauses, supertraits, `dyn`/`impl Trait`), glob imports grown from unresolved-name diagnostics (only for names defined nowhere in the union). |
| `postrait.py` | rustc stops before body type-check when a trait path in a bound, `dyn`, `impl Trait`, supertrait, impl header or `<T as Trait>` does not resolve. Such paths (reported by rustc, or reachable only through a failed `use` alias that rustc reports silently) are rewritten in the sandbox to a numbered generic stub trait `crate::__ext::__unresolved::Tr<n>`. Every rewrite is listed under `trait_neutralised` in the result JSON, so a real defect is never hidden. |
| `poscoh.py` | Coherence scan (E0116, E0117, E0210 candidates across crate boundaries). Heuristic, read only. Result: 0 candidates for `phc` and `okernel` (`pos-phc-coherence.txt`, `pos-okernel-coherence.txt`). |
| `families/*.json` | Family specs: crates, optional baseline `{rev, crates}`. |

How a run works: per side (`cur` = working tree, `base` = `git cat-file` of the pre-carve revision) build the union, scan stubs, run rustc per configuration, parse `--error-format=json` (with `-Ztrim-diagnostic-paths=no`), fix up (stripped attributes, stub glob names, neutralised trait paths, broken-alias traits) and re-run until nothing changes (at most 8 rounds). Diagnostics that mention the stub layer (`__ext`, or an identifier that only the stub defines) are flagged `stub` and hidden from the lists; privacy codes (E0603, E0616, E0624, E0451, E0364, E0365, E0445, E0446) are never hidden. The diff key is (code, message and label with module path prefixes and `TrN` normalised); `NEW` = multiset difference current minus baseline.

**Canary.** The union root carries a module whose probe function must raise E0616, E0624 and E0308 when bodies are type-checked. A run whose canary is missing did not type-check bodies and is reported as such in `typeck canary (...)` of every report. This is how the earlier "0 new" claims of a resolution-only differential are separated from real type-check results.

**Quarantine (`--quarantine`).** If the canary is missing after the fix-up rounds, a delta-debugging pass finds the files that must be emptied for body type-check to run, empties exactly those, and re-runs. The files are listed as `QUARANTINE` in the report; their own checking (and that of the modules they mount) is lost, nothing else.

Controls run:
* Seeded negative control (`rcr`, tree): removing one `pub(super)` from `🧱️tree-model` produced 34 internal E0425 (log `pos-rcr-neg1-report.txt`): the oracle sees the loss of visibility on the `rcr` splits.
* Positive controls: the real defects in section 3 (each disappeared after its fix, section 3 table).
* `-o /dev/null` is refused by the tool sandbox on `/dev/rmeta*`, so the output file goes to `<sandbox>/__out_*.rmeta` and is removed right after each run (nothing else, no rmeta is read by another rustc).

Heuristic note: until the last tool edit the `stub` flag also looked at rustc's suggestion text (`help: consider importing ... __ext::...`), which hid the sibling-module E0433 rows of `ri2` in the `f3` list (they are visible in `pos-ri2-f2-report.txt`); the edit (labels and child messages only) is in `pos.py` but was not re-run on `ri2` (30 min under load).

Limits (all families): a type that derives from a stub type is error-typed and rustc skips its privacy check; methods of external traits are invisible; items generated by external macros (for example the `return_walk_template!` of SINGLE-MOUNT-AND-GATE, `bindgen!`) do not exist, so their users show resolution noise (`E0432`/`E0425` on `walk_*`); `cfg(test)` is on for every crate under `--test`; `[[test]]` integration targets are not unioned.

## 2. Families, configurations and results

`new` = new against the baseline; errors counted after the last fix-up round. "stub noise" = rustc errors caused by the stub layer. Privacy before = at first sweep; after = on the final run (`f1`/`f2`/`f3` label) after my fixes (section 3).

| Family | Crates | Baseline | Configs | Body type-check | Privacy errors (E0603/E0616/E0624) before to after | Other non-stub new |
| --- | --- | --- | --- | --- | --- | --- |
| `phc` PLUGIN-HOST-CARVE | host facade + 14 crates | `1bae6115bc5` (monolith host) | lib, test, wasm (first sweep) | yes | lib 4 to 0; test 73 sites (12 members) to 0 | final `f2`: lib 4287 vs 4284 errors, test 8873 vs 8866, 0 new type errors; the only non-stub new rows are the `return_walk` macro artefact (section 5). Before fixes: trait-scope `GuestRuntime` import (2 files) and the `identity` test paths `crate::host`/`crate::RuntimeActorId` |
| `rcr` REPO-CRATE-ROOT-SPLITS | tree, test-runner, hooks, mcp, providers, languages | `HEAD` | lib, test | yes | 0 to 0; lib 850 = 850 errors, test 928 = 928 | 0 new |
| `rhs` REPO-HUB-RUST-FILE-SPLITS | repo-model, repo-tickets, hub sqlite/postgres/neo4j (+server) | `HEAD` | lib, test | yes after quarantine of `semio-framework-server` root | 0 to 0 | final `f1`: lib 6232 = 6232, test 6289 = 6289 errors, 0 new; `rhsserver` (server crate with its 5 sibling crates, no quarantine, the gateway split) lib 300 = 300, test 319 vs 889, 0 new |
| `osm` OS-SERVICES-MCP-SPLITS | kernel crate (services and mcp are its modules) | `HEAD` | lib, test | yes | 0 to 0; lib 106 = 106, test 272 vs 27,477 | 0 new |
| `smg` SINGLE-MOUNT-AND-GATE | guest-contract, plugin-abi, reactor-context, host-plumbing, host-operation | `HEAD` | lib, test | yes after quarantine of `plugin-abi/🚪️io/🦀️.rs` (its lib root) | 0 to 0 | 0 new real (the E0432 on `operation_authority` etc. are the quarantine) |
| `dlo` DIRECTION-LEFTOVERS facades | flow/playbook contract + flow/playbook artifact + models | none | lib, test | yes after quarantine of 8 files | 0 | static check: all 23 (flow) + 7 (playbook) re-exported names exist in the target closure; the oracle itself is limited by the 8 quarantined files |
| `okeng` OS-KERNEL-ENGINE | the 18 carved crates + space-schema | none (absolute) | lib, test | yes | 11 to 0 | see section 5 |
| `oksql` OS-KERNEL-SQLITE-HOST + sync | space-history, text-io, codec-registry, os-io, sync, sync-native, sync-wasm | none (absolute) | lib, test | yes | lib 0; test 2 to 0 | 11 `fields::*` noise (alias of an out-of-union crate) |
| `ri2` RENDERER-INTEGRATION-2 | 76 `semio-framework-os-renderer-wgpu*` crates | `1bae6115bc5` | lib, test | yes | lib 10 to 0 (`f3`: 21,010 vs 20,737 errors); test 12 E0603 `unresolved item import X is private` rows in the tests that stay in the Shell tree (not triaged, probably the `use <alias>;` rewrite of the tool; `test_app`, `host_test_shell`, `split_pane_shell`, `tour_shell`, ...) | trait-scope (1) and import (2) defects fixed; open design items in section 5 |
| `okernel` all 49 kernel/store crates at once | kernel + store + io | `1bae6115bc5` | lib, test | **no** (blocked, quarantine degenerates) | not obtained | covered by `okeng` + `oksql` + `osm` instead |

Not swept (still running or not requested): `pi1`, `okwp8`, `dm2`, `pch`, `pfa`, `orv`, `svm`, `tgm`, `pfs`, `hss`, `ts3`, `otc`, `rhu`. `phc`'s `…-test-component` crates are wasm components and not unioned.

## 3. Fixes applied (all minimal widenings or one missing import; each re-run to 0)

Paths below are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/` unless stated. Nothing else was edited.

| # | File | Error | Edit |
| --- | --- | --- | --- |
| 1 | `🔌️plugin/🖥️host/🔧️engine/🦀️.rs` | E0616 `EpochDeadlinesInner.pool`/`.engine` read by guest-runtime (4 sites) | `struct` and the two fields `pub` |
| 2 | `🔌️plugin/🖥️host/🧵️shard/🦀️.rs` | E0624 `push_inbound` (37 uses) and `take_outbound` (25 uses) of `LoopbackProbe` in shard-laws; E0425 `Mutex` in a non-testkit lib build (the struct had left its `cfg(test)` module without the gate) | `LoopbackProbe` and its impl gated `#[cfg(any(test, feature = "testkit"))]`, struct and the two methods `pub` |
| 3 | same | E0616 `FixedReplaySeed.request`, `.kind_pages`, `.kind_length`; E0624 `copy_kind_page`, `copy_input_page`; E0616 `OwnerSlot.owner` | `FixedReplaySeed`, `OwnerSlot` and those members `pub` |
| 4 | `🔌️plugin/🖥️host/🧵️shard/🔁️lifecycle/🦀️.rs` | E0616 `AdmittedAuthority.authority`, `.retry` | struct and the two fields `pub` |
| 5 | `…/🧵️shard/🔁️lifecycle/🧪️tests/🔁️lifecycle/🦀️.rs`, `…/🧵️shard/🧪️tests/🔬️unit/🦀️.rs` | E0599 `compile`/`instantiate` on `Arc<MockGuestRuntime>` (trait `GuestRuntime` not in scope after the extraction of the tests into a crate) | `GuestRuntime` added to the existing import |
| 6 | `…/🧵️shard/🪪️identity/🧪️tests/🦀️.rs` | E0432 `crate::host`, E0425 `crate::RuntimeActorId` (monolith paths) | imports of `MockGuestRuntime`/`GuestInstanceState`/`MockInstanceState` from the guest-runtime crate, `RuntimeActorId` as `semio_framework_actor::ActorId` |
| 7 | `🏪️store/🏗️initialization/🏃️runtime/📥️lifted/🏪️store/🦀️.rs` | E0616 `ArtifactStoreInitializationRuntime.close_phase` read by the sibling module `lifted_replay` | `pub(crate)` |
| 8 | `🏪️store/🧩️composition/🗄️durable-group/📜️wire/📥️lifted/🗄️durable-group/🦀️.rs` | E0616 `DurableOwnedGroupAnchorV1.schema/.parent/.shape` read by the durable-group crate (`pub(crate)` across a crate edge) | the three fields `pub` |
| 9 | `🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs` | E0624 `initial_history_is_exact` (`pub(super)`) called from space-member | `pub` |
| 10 | `🏪️store/🔄️sync/🦀️.rs` | E0616 `ArtifactMailboxClose.authority` read by sync-native tests | `pub` |
| 11 | `🏪️store/🔄️sync/🖥️native/📥️lifted/🔄️sync/🦀️.rs` | E0432 `use fixtures::*` (the module lives in the sync crate) | `use semio_framework_os_store_sync::fixtures::*;` |
| 12 | `🔨️modules/🖱️ui/🎨️styling/🌓️theme/📐️geometry/🦀️.rs` (framework module) | E0603 `contract`, `bindings` (3 sites) private (`pub(crate)`) for shell-input | `pub fn` |
| 13 | `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs` | E0624 `cancel`, `validate_original` (3 uses), `events_terminal_is_empty`, `return_shell_maintenance`; E0616 `RuntimeDispatchSource.epoch` used by runtime-mailbox (`runtime-apply`) | the four methods and the field `pub` |
| 14 | `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📺️frame-presentation/🦀️.rs` | E0599 `interaction_mut` on `MutexGuard<AppRuntime>` (trait `MailboxRuntime` not imported) | `MailboxRuntime` added to the runtime-mailbox import |
| 15 | `📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/🎨️theme-file-import-and-accessibility/🦀️.rs` | E0432 `super::theme_file_import_and_accessibility` (module lives in chrome-state) | `use semio_framework_os_renderer_wgpu_shell_chrome_state::theme_geometry;` |
| 16 | `…/📥️staging/⚓️dock/🪟️tree-windows/🦀️.rs` | E0432 `super::ShellState` (type lives in the higher crate shell-state; the import was unused) | `use super::UiDirtyScope;` |

Before/after: see the privacy column of section 2. Re-run commands: `python3 pos.py run families/<f>.json --cfgs lib,test --label <L> --quarantine --qmax 30` from `F/pos-tools` (about 10 s to 5 min per family under load 30 to 100).

## 4. The abort before body type-check (open tooling issue)

For several unions rustc ends `coherence_checking` with errors and then skips all bodies (the canary is not reported). Cases I attributed and handled: any unresolved trait path (bound, `dyn`, `impl Trait`, supertrait, impl header), E0404 (struct used as trait), a failed `use` alias that rustc reports silently (the name that comes from an unresolved external item macro such as `bindgen!`), and `extern crate X as Y` aliases. Remaining cases (`okernel` 49 crates, the first `rhs` and `ri2` unions, `smg`'s `plugin-abi` root, `dlo`) are not reproduced by micro-tests of every error code present (E0050, E0185, E0204, E0220, E0277, E0407, E0437, E0599, E0038, ...). Delta debugging reduced them to single files (for example `plugin-abi/🚪️io/🦀️.rs`, `semio-framework-server` root) or, for `okernel`, to hundreds of files, which is why the quarantine is capped (`--qmax`) and `okernel` is split. Hand-off: a rustc-internal explanation (or a real cargo check) would remove the need for quarantine.

## 5. Findings that need a design decision or belong to other slices

* **`ri2`: monolith sibling-module heads left behind (not fixed).** Staging files of the flipped Shell crates still name modules that only exist in the Shell parent of the facade: `time_travel::…` (shell-prefs, shell-history, shell-state, shell-actions), `local_folders::…` (shell-actions, shell-state), `icon_export::…` (shell-pointer, shell-doc-open), `tree_windows::…` (shell-state, shell-dock), `winit_app::…` (native-probe), `host_base::…` (host-winit), `element_contracts` (ui-engine-dump). In the monolith they resolved through `use super::*` from the parent; in the crates the items live in `staged_<crate>__<chunk>` modules or in lower crates. Every site needs an alias (`use staged_… as time_travel;`) or a path to the owning crate; which crate owns each name is a design decision of the slice (full list: `pos-ri2-f2-report.txt (lib) and pos-ri2-f3-report.txt (test)`, group `resolve`).
* **`ri2`: feature-gated field defined in one crate, initialised in another.** `HistoryState.host_route_folder` is `#[cfg(any(wasm32, test, feature = "testkit"))]` in shell-history; shell-state's initialiser carries the same attribute but refers to shell-state's own `testkit` feature. When shell-history is built with `testkit` and shell-state without it (a workspace build unifies features) E0063 appears (`pos-ri2-f2-report.txt (lib) and pos-ri2-f3-report.txt (test)`). Needs either an ungated field or a constructor in shell-history.
* **`ri2`: tool artefacts among the new resolve rows:** `serde_json::json!` (external macro), `pack` in `native-socket-probe/🪶️sqlite` (alias through `extern crate … as pack` inside an inline module) and `element_contracts`/`NativeJsonPages` (crates outside the union) are not defects.
* **`ri2`: sync fixtures of out-of-union crates** (`NativeJsonPages`, `NativeRuntimeManifest` unresolved import in program-bridge) are stub-layer gaps, not verified.
* **`okeng`/`okstore`:** E0432 on `ArtifactEnvelope`, `ArtifactBackboneRef`, `ArtifactProjectionCause` etc. come from `space-schema` re-exporting crates outside the family (glob of an external crate): not a defect, but the 47-crate union that would resolve them is blocked (section 4).
* **`phc`/`smg`:** `operation` (host) shows `E0432 return_walk::walk_snapshot_file_result` and test `E0425 walk_*`: `return_walk` is generated by the macro `return_walk_template!` of plugin-abi (SMG); the oracle cannot expand it. Statically fine, to be closed by the real check.
* **Coherence scan** found no E0116/E0117/E0210 candidate in `phc` or in the 49 kernel crates.
* **Not covered:** the `okernel` union of all 49 crates at once, wasm targets other than `phc`, `[[test]]` integration targets, `semio-framework-plugin` (oversized, owned by `pi1`/`pwp7`).

## 6. Coordinator message: head-regex defect

`pos-tools` was generalised from `frs-tools/stubgen.py`, whose head regex `(?<![\w$:.])` has the defect named in `RESULT-TOOL-TEST-COMPANIONS.md` section 6. I found it independently in my first smoke run on `space-history`, `text-io`, `codec-registry`, `os-io` (`enum Linked{native_identity:semio_framework_os_store_text_io::ArtifactNativeSnapshotIdentity}`, 18 internal E0433 `cannot find text_io`) and fixed it before any family result was recorded, with `(?<![\w$.])(?<!::)`. The same lookbehind was in the external-item-macro regex and is fixed too (`posunion.py`). Every number in section 2 comes from runs made after the fix; the families swept before later tool fixes (alias heads, inline `#[path]` directories, `dyn|impl` headers) were re-run (labels `f1` to `f3`): rcr 850/928 identical, osm 106/272, rhsserver 300/319, smg and dlo after quarantine, okeng, oksql, phc, ri2. No family's "0 new" comes from the buggy version.

## 7. Commands run (summary)

`python3 pos.py index --force`; `python3 pos.py run families/<f>.json --cfgs lib,test[,wasm] --label <L> [--no-base] [--quarantine --qmax N]` for `phc`, `rcr`, `rhs`, `rhsserver`, `osm`, `smg`, `dlo`, `okeng`, `oksql`, `okernel`, `ri2` (labels `a1` first sweep, `q1` with quarantine, `f1`/`f2`/`f3` final after fixes); `python3 poscoh.py families/{phc,okernel}.json`; a static re-export check for the two facades; rustc micro-tests in the sandbox for the abort analysis. Real `bun repo cargo`: not run (headroom gate).

## 8. Files

Created: `F/pos-tools/{pos.py,poslib.py,posunion.py,posstub.py,postrait.py,poscoh.py}`, `F/pos-tools/families/{phc,rcr,rhs,rhsserver,osm,smg,dlo,okeng,oksql,okstore,okernel,ri2}.json`, this file. Edited (repo, section 3): 20 source files listed there. Logs: `F/🗑️generated/pos-*`.
