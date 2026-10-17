# 🧹 Result RUST-LEFTOVERS (prefix `rsl`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. Run 2026-10-16 21:46 to {{END}} CEST.
`F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/rsl-tools/`. Logs and tables: `F/🗑️generated/rsl-*`. Pre-edit copies of every file I changed: session scratchpad `rsl-pre/<repo path>`.
Short names: `FAC` = `🧰️framework/📦️packages/🦀️rust/🦀️.rs` (facade crate `semio-framework`), `S` = `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store`.

## 0. Verdict

| Target | Result |
| --- | --- |
| F-04(a) re-point facade alias consumers | **Done outside the skip zones; 4 of the 8 aliases deleted.** 157 files / 702 path uses re-pointed (OK zone: 0 uses left), 120 manifests got 126 `path` dependencies, 161 `Cargo.lock` files were hand-edited. Deleted from the facade: `ui`, `abi`, `platform`, `tool_run`. Kept because 47 files in the skip zones (`🔌️plugin/**` 27, `📺️renderer/**` 20) still name them: `action_bus` (18 uses), `history_edit` (46), `onboarding` (189), `interaction` (10). A re-runnable tool finishes them in one command when `pi2`, `rpv` and `rw2t` are done (section 7). |
| F-11(a) derive root of `os-store-space-schema` | **Verified complete.** Scan over the whole os tree: 111 files use a bare `dsl::` path, 0 lack a root (section 3). One neighbouring defect found and fixed: see (d). |
| F-11(b) `os-store-engine` size, decision D3 | **Done.** `advance_apply_batch` (+ `spend_item_close_grant`), `ingest_remote_merge` and the pure `edits_for_ids` moved to the new crate `os-store-engine-bundle`. Engine 254,589 -> **199,055 bytes** (target 230,000), 3,452 -> 2,771 active lines; bundle **67,182 bytes**, 937 lines. Oracle: 0 new errors, the 5 `LaneItemReceipt` rows of the old tree fixed (section 4). `begin_apply_batch_owned` was already homed in `StoreCursor` (`engine-core`) and needed no move. |
| F-11(c) text-pinned TS scanners | **Partial.** Shared reader `readStoreSourceText()` + language-agnostic law added; 8 pinned readers re-pointed; results per file in section 5. The rest are stale against the new API (not relocations) or live in the skip zone. |
| F-11(d) derive root of `diff_text` / `diff_binary` / `CompositeMutation` | **Scanned, one real defect fixed.** 44 crates use these derives, all above the kernel except `semio-framework-os-command-tests` (4 `#[composite]` fixtures, no kernel dependency): dependency added. |
| F-11(e) `pub use pack::*` in `os-pack` and siblings | **Not done, sized.** About 550 path uses in about 170 files; `pack_rt` adapters and the scalar-witness schema stay by design (section 6). |
| F-11(f) kernel `🎒️pack/⚠️error` exports | **Verified.** No source imports the five names from `⚠️error` any more (section 6). |

Everything below is static or oracle evidence except the one real `cargo` probe in section 8.

## 1. Found state

* The facade (`FAC`) carried the aliases `ui`, `abi`, `action_bus`, `platform`, `interaction`, `history_edit`, `tool_run`, `onboarding` plus `io_schema`, `sqlite_snapshot`, `mesh_io`. At the start 647 uses reached seven aliases through `semio_framework::<alias>::` (the 640 `ui` uses of the old audit were already gone: the consumers name `manifest`) and another 318 `action_bus` uses came through the glob `pub use semio_framework::*;` of the plugin crate (`semio_framework_plugin::action_bus::`): 965 in total, 702 re-pointed and 263 left in the skip zones.
* `os-store-engine`: 254,589 bytes, `ingest_remote_merge` 276 and `advance_apply_batch` 301 lines inside the composer; a peer was editing the same file at the same time (`PendingCommandReport` encapsulation, 47 lines). Every cut below re-read the file right before writing and asserted its anchors.
* The engine crates had never been type-checked: the pre-edit oracle run listed real defects in the file (missing import of `LaneItemReceipt`, 5 sites), fixed in section 4.

## 2. F-04(a): facade re-pointing

### 2.1 What was re-pointed (tools `facade_plan.py`, `facade_apply.py`, `facade_self.py`)

| Alias | Real producer | Uses re-pointed (OK zone) | Uses left (skip zones) |
| --- | --- | ---: | ---: |
| `action_bus` | `semio_framework_action_bus` | 308 (`semio_framework::`) + 318 (`semio_framework_plugin::`, the glob leak) | 18 (plugin zone) |
| `abi` | `semio_framework_abi` | 48 | 0 |
| `onboarding` | `semio_framework_onboarding` | 21 | 189 (renderer 153, plugin 36) |
| `history_edit` | `semio_framework_time_travel_actions` | 2 | 46 (renderer 15, plugin 31) |
| `tool_run` | `semio_framework_tool_run_actions` | 4 | 0 |
| `interaction` | `semio_framework_interaction` | 0 | 10 (renderer 9, plugin 1) |
| `ui` | `semio_framework::manifest` (the single use also dropped the redundant `::ui::kernel`) | 1 | 0 |
| `platform` | `semio_framework_action_bus::platform` | 0 | 0 |

Total 702 replacements in 157 files (`🗑️generated/rsl-rust-write.txt`). Per file the replacement count had to equal the planned count and the file text had to be unchanged since planning (assertions); `git diff --numstat` shows 447 lines +/- 447, symmetric. Residual scan after the run (`facade_plan.py`): 47 files, all in the two skip zones (`rsl-residual-files.json`).

In the facade root itself (`facade_self.py`): the alias lines `ui`, `abi`, `platform` (`pub use semio_framework_action_bus::platform;`) and `tool_run` are gone; `pub use platform::{PanelVisibility, Platform, PlatformSpec};` became `pub use semio_framework_action_bus::platform::{…}`; `FRAMEWORK_RESERVED_CONTRIBUTIONS` names the two producer crates; the mounted `🛂️manifest/🧪️tests/🧩️framework-reserved` test too; the module list in the crate docstring was rewritten. The root keeps its facade API (`kernel`, `manifest`, the item re-exports) and the aliases `action_bus`, `interaction`, `history_edit`, `onboarding`, `io_schema`, `sqlite_snapshot`, `mesh_io`.

Deleted aliases were checked for hidden users: no `crate::`/`super::`/bare use of `ui`, `abi`, `platform`, `tool_run` in the facade crate, in the glob-importing crates (`semio-framework-plugin`, `semio-framework-os-host`, the `reserved` retained-command file) or in the crates that glob `semio_framework_plugin::*` (bare `tool_run::` in `wp3-owners` is a local alias of `tool_jobs`, bare `ui::` in `app-panel-kit` is a local `use … as ui`).

### 2.2 Manifests (`manifest_plan.py`, `manifest_apply.py`)

120 manifests changed, 126 lines added, 0 removed (`git diff --numstat`): `semio-framework-action-bus` 114, `semio-framework-abi` 5, `semio-framework-tool-run-actions` 4, `semio-framework-onboarding` 2, `semio-framework-time-travel-actions` 1 (125 in `[dependencies]`, 1 in `[dev-dependencies]`, always in the section of the facade or plugin dependency line it follows). All lines are `path` dependencies computed relative to the manifest: no workspace root lists these crates in `[workspace.dependencies]`, so `workspace = true` was not possible without editing 71 workspace roots. Direction: every consumer already depends on the facade (or `semio-framework-plugin`), which depends on the producer, so no edge can close a cycle; `kcf-tools/manifests.ts` on the 120 manifests: TOML parses, every path resolves, no duplicate key (0 problems). `dep_check.py`: for each rewritten file and each owner manifest (reach graph, 31,036 files) every producer crate named in the text is a declared dependency (156 files, 0 missing).

### 2.3 Locks (`locks_apply.ts`, copy of the `orc` propagation)

`propagateEdges` over all 180 lock files: **161 `Cargo.lock` changed**, 126 edges, 2,005 edge placements, 26 producer blocks copied into stale locks together with their closure (the lock listed the consumer but not the producer: `🗑️generated/rsl-locks-write.log`), 1 dev edge skipped (`🌎️hub/Cargo.lock`: `semio-s-dev-trinity-jack` is not a member). `locks_check.ts` afterwards: 2,005 placements, 161 locks, 0 violations. The list is `🗑️generated/rsl-locks-changed.txt`: 38 stdio, 17 norm, 11 flow, 6 process, 6 wfc, 5 sourcing, 5 puzzle, 4 block, 4 procedural, 3 space, 3 trinity, 3 energy and 56 more under `✏️s/🔌️plugins/**`, `✏️s/🧑‍💻dev/**`, `🌎️hub` and the root. **Each of these workspaces needs `cargo update --workspace --offline --manifest-path <ws>/Cargo.toml` before its first `--locked` check**, as do all stale locks of F-02 (the engine entry of the root lock was already out of step with its manifest before my edit: the manifest declares `job`, `os-directory`, `os-spr`, `os-store-command`, `os-store-ephemeral` and more that the entry lacks, and the entry lists `os-store-codec-registry`, which the manifest no longer has).

## 3. F-11(a) and F-11(d): derive roots

* `dslroot.py` over `🧰️framework/🛍️products/💻️os`: 111 files with 136 bare `dsl::<Derive>` uses. For every (file, owner crate) pair `dsl` is bound by `extern crate semio_framework_os_kernel as dsl;`; all owners depend on the kernel and none is in the kernel's own dependency list, except one test file under `🔌️plugin/🖥️host/🧪️tests/💡️service-operation` (owner `semio-framework-plugin-host-wit`, no kernel dependency; plugin zone, hand-off to `pi2`/`phc`).
* `os-store-space-schema`: the 7 mutation files name `semio_framework_os_dsl::{MutationLeaf, Mutations}` (os-dsl re-exports both derives: `🗣️dsl/🦀️.rs:28`), the manifest declares every root the two expansions name (`replication`/`protocol`, `os-command`, `value`, `dsl-record`, `diagnostic`, `ui-locale`, `schema-state`, dev `os-store-one-item`). `DslArtifact` emits `::semio_framework_dsl_record::…` (checked in `🗣️dsl/✨️derive/🦀️.rs` lines 843-848).
* `diff_text!`, `diff_binary!` and `#[derive(CompositeMutation)]` still emit `::semio_framework_os_kernel::…` (29 paths). Scan of all 89 files that use them: 44 owner crates have the kernel dependency; **`semio-framework-os-command-tests` did not** (4 fixtures `4️⃣add-counter-four-times`, `✌️add-counter-twice`, `🌐️add-counter-then-notify`, `🔢️add-counter-sequence` with `#[composite(…)]`). Fix: `semio-framework-os-kernel` added to the companion manifest and to its entry in the root `Cargo.lock`; no cycle (nothing depends on the companion). A path-root attribute (like `mutation_leaf(contract = …)`) was not added: `fold_plan_diff`/`fold_plan_inverse` live in `os-spr`, the rest in `os-command`/`replication`, so a single override root does not exist.
* Library names, not package names, are the path heads: `semio-framework-replication` is `protocol`, `semio-framework-pack` is `pack`. The oracle caught my first attempt to spell `protocol::` as `semio_framework_replication::` (37 E0433 rows); reverted, the engine files keep `protocol::` and `pack::`. Scans that compare heads with dependency keys (including mine at first) overcount "unbound" heads for these two.

## 4. F-11(b): decision D3 (spill valve)

Tools: `d3_split.py` (cut and transform), `d3_register.py` (manifest and registries), `d3_locks.ts`, `d3_prune.py`, `d3_diff.py`.

### 4.1 Design

New crate **`semio-framework-os-store-engine-bundle`** at `S/⚙️engine/🧳️bundle/` (`🦀️.rs`, `📦️packages/🦀️rust/{Cargo.toml,📋️project.json}`; 108/136/145 path bytes). It depends on `engine-live` (home of `StoreParts`) and `engine-core`, below `engine`. Contents (936 lines): `advance_apply_batch`, `spend_item_close_grant`, `ingest_remote_merge`, `edits_for_ids` as free functions over `StoreParts<'_, P, Mutation>`, and 23 private one-line helpers (`#[inline]`, bodies copied from the composer's delegates, with `&mut self.X` becoming `&mut *store.X`) so that every `self.helper(args)` of the moved bodies keeps its exact argument evaluation order (a direct substitution would take `&mut *store.log` before evaluating an argument that reads `store.log`). `begin_apply_batch_owned` is a delegate of `StoreCursor` already.

`ArtifactStore` keeps the old public signatures as delegates: `advance_apply_batch(&mut self, …) { bundle::advance_apply_batch(&mut self.parts(), …) }`, `ingest_remote_merge`, `edits_for_ids` (still used by the `🌈️bin-03` companion test). The 18 `use` leaves that only the moved bodies needed were removed from the composer (non-traits only: `d3_prune.py`), and the dependency `semio-framework-diagnostic` that only they used.

### 4.2 Registrations

Root `Cargo.toml` member (byte-sorted), `📋️build-owners.json` row `@semio-tech/os-store-engine-bundle-rs`, 6 launch routes in both launch files (inserted behind the `engine-stage` routes, next free order numbers 900.26133 and up), boundary corpus row (`maximumActiveLines` 1500) in `S/../🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json`, engine manifest edge, root `Cargo.lock` package block with its 15 dependencies and the engine edge (the framework workspace lock does not hold the engine). No taxonomy entry: the folder reuses `🧳️` (already a folder prefix elsewhere) and no sibling in `⚙️engine/` uses it.

### 4.3 Census (`🗑️generated/census.ts`, lib closure)

| crate | files | bytes | active lines | overLimit |
| --- | ---: | ---: | ---: | --- |
| `semio-framework-os-store-engine` | 4 | **199,055** (was 254,589) | 2,771 (was 3,452) | false, ratio 0.759 |
| `semio-framework-os-store-engine-bundle` | 1 | **67,182** | 937 | false, ratio 0.256 |
| `semio-framework-os-store-engine-core` / `-live` | 3 / 3 | 184,198 / 190,324 | 2,693 / 2,705 | false |

Test closure (`rsl-tools/tcc-census.ts`, outputs `rsl-tcc-*`): engine lib-test 5 files, 199,152 bytes, ratio 0.76; the bundle has no tests; the facade `semio-framework` lib-test closure 8 files / 249,064 bytes (ratio 0.95, unchanged by me, 58 bytes smaller). The 10 root-workspace members among the touched crates (facade `semio-framework`, kernel, engine, bundle, `os-command-tests`, `os-mcp-catalog`, four `os-flow-wasm*` crates): 0 `overLimit`, 0 `oversizedDeps`. The plugin-workspace consumers of 2.2 are not members of the root workspace, so `census.ts` does not measure them; their sources only got shorter (a `semio_framework_plugin::action_bus` path became 8 bytes shorter).

### 4.4 Typeck oracle (`pos-tools`, prefix `rsl`, stub layer over the 47 store crates + bundle)

| run | config | result |
| --- | --- | --- |
| `pre` (tree before D3, `rsl-rsld3-pre-*`) | lib | canary E0616/E0624/E0308 reached; 517 listed non-stub rows |
| `post2` (after the move, before my import fixes) | lib | **0 added keys, 0 gone keys** against `pre` (`d3_diff.py`): the moved bodies type-check exactly as they did in the composer |
| `post3` (+ `LaneItemReceipt` import in engine and `member_group`, prune) | lib | 0 added, 2 gone keys (3 rows: the `LaneItemReceipt` rows of the engine and `member_group`) |
| `post4` (+ the same import in the bundle) | lib | **0 added keys, 2 gone keys (5 rows: every `LaneItemReceipt` E0422/E0425)**; 512 listed non-stub rows against 517; canary reached; the one remaining row inside the bundle is the stub-layer E0308 at the `admit_retained_clone_progress` `if let Err`, which the composer had in `pre` |
| `fin` against `HEAD` (`b84bf526c7ba`) | lib, test | lib: 20,564 errors (base 20,721), NEW 5, all classified stub-layer noise, **0 non-stub new**; test: 23,294 (base 23,432), NEW 45, **0 non-stub new**; canary reached on both sides |

What this does and does not prove: the oracle type-checks bodies against a stub layer and the borrow checker only runs on bodies without type errors in the stub world; the three `lifetime may not live long enough` rows in the union belong to `engine-core` and `engine-live` and exist before my change. The moved bodies compile exactly as clean (or as stub-noisy) as before; the real `cargo check` of section 8 is what settles E0499/E0502.

Defects found and fixed in passing (all invisible to the earlier static tools): `LaneItemReceipt` was used but not imported in `S/⚙️engine/📥️lifted/🏪️store/🦀️.rs` (4 sites: `apply_one`, two inside `advance_apply_batch`, the return type) and in `S/🧩️composition/📬️publication/🤝️group/🦀️.rs` (1 site); the import was added to both and to the bundle.

## 5. F-11(c): text-pinned TS scanners

`okeng-tools/pins.py` (re-run at the start): 11 files, 94 needles. Two of the 11 only mention the store root in a comment (`⚖️laws/🧩️plugin-consistency/🚪️artifact-io`, `🔍️discovery`). Done:

* `🧰️framework/🔨️modules/🧪️test/📖️source-text/🟦️.ts` gained `readStoreSourceText()`: the store facade root followed by the module files of the carved crates (the directories `pins.py` uses plus every `📥️lifted` folder), tests and packaging folders excluded, byte-ordered. Re-exported through `@semio-tech/framework/test/source-text`. Law added to the module's test (`🧪️tests/🟦️.ts`, fixture key `store` in `🧫️fixtures/🔣️.json`, language-agnostic) with the third-party oracle `fast-glob`: `bun test` 5 pass, 0 fail (was 4/0).
* Re-pointed and measured with `bun test ./<file>` (before -> after):

| File | Change | Result |
| --- | --- | --- |
| `📝️drafts/🧪️tests/🟦️.ts` (replication) | reads the file that holds the needle (`⚙️engine/🧱️core/📥️lifted`) | 0 pass / 1 fail -> **1 / 0** |
| `🌐️foreign/🧪️tests/🟦️.ts` (replication, script) | store path -> `⚙️engine/🎞️live/📥️lifted`, end anchor -> `//#endregion 📬️OneItemPublication`, needle `candidate.edit().forwards[0]` (accessor, stale since an earlier carve) | script failed -> **runs to the end, all asserts pass** |
| `🎮️one-item/♻️close/🧪️tests/🟦️.ts` | `spend_item_close_grant` now read from the bundle (moved by D3), needles `size_of_val(…)` and `matches!(step,RetainedCloneStep::Complete(_))` (compaction drift of earlier passes) | 0 / 1 -> **1 / 0** |
| `🧩️composition/📨️emission/📦️owned/🧪️tests/🟦️.ts` | 4 store-root reads -> `readStoreSourceText()`, trait signature `fn demands(&self, store: &dyn ArtifactStoreCloseHost<P, Mutation>, …)` (decision C1) | 16 / 9 -> 16 / 9: the first failing line of every test moved past the store reads; the 9 remaining laws pin other carves (stdio semio root, `os-services`, plugin facade) |
| `🧵️canonical-edit/🧪️tests/🔬️canonical-error-progress/🟦️.ts` | first store read -> `readStoreSourceText()`; the `begin_next` refusal slice reads the bundle (`advance_apply_batch` moved there). Reading the union for the other five root reads made two more laws fail (they slice by root-only anchors) and was reverted | 9 / 6 -> **10 / 5**; the 5 left pin shapes that changed in earlier carves (`Result<ArtifactStoreOneItemPreparationStep, ValueError>`, `refused_base: Option<SnapshotRead<P>>` now homed in `one-item-owners`) and need a rewrite by the one-item owner |
| `📦️packages/🦀️rust/🎛️commands/🧬️retained-clone-check/🟦️.ts` | 2 reads re-pointed (`engine-core` for `handoff_batch_publication_close`, `one-item-owners` for the `ErasedSnapshotRetirement` impl) | not runnable here (`BundleScript`); both assertions simulated with a 10-line script: pass |
| `🖥️host/📦️packages/🦀️rust/📜️script.ts` | `MemberFactory` slice read from `🧩️composition/👥️space-member/📥️lifted` | static; `pub trait MemberFactory` and `pub enum NoMembers` are in that file |
| `📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts` | `OS_STORE_ROOT_SOURCES` gained the five engine/owner files (the list "scans exactly the production source the single root file held") | the store-side tool-job coverage law (`toolJobStoreBatchPublicationBounded`) returns `false` before and after, simulated with `sim_toolcov.ts`: other drift (plugin root text) keeps it false |

Not touched on purpose: `🔌️plugin/⏪️time-travel/♻️retirement/🧪️tests` (15 of 21 tests fail, 11 needles) and `🔌️plugin/🧪️tests/🔬️tool-job-latest-wins` (2 needles, 0 tests found) are in the `pi2` zone; `✏️s/…/🌊️flow/…/♻️retirement/🧪️tests` cannot load (`Cannot find module …/📡️replication/🧾️wire/♻️retirement/🧪️tests/🟦️.ts`, a missing file unrelated to the store).

## 6. F-11(e) and F-11(f)

* **`pub use pack::*;` in `os-pack`** (`🎒️pack/🦀️.rs:6`): the crate's own items are `pack_rt`, `stored_content_hash` and `record_codec_laws`; everything else a consumer writes after `semio_framework_os_pack::` or `os_pack::` is a name of the framework `pack` crate (library name `pack`). Measured: 255 uses in 61 files via `semio_framework_os_pack::<name>` (record 78, write_varint_u* 40, PackLimits 17, PackRefusal 16, ByteReader 16, …), 18 files with `use semio_framework_os_pack::{…}` groups that mix both kinds, and about 290 more uses through the kernel hub alias `os_pack::` (`record`, `PackRefusal`, `codec`, `json`, `format`). The sweep needs the same plan, apply, manifest and lock pipeline as 2, plus regrouping; it was not started. The F-04 tools are parameterised by an alias table (`facade_lib.PKG`, `facade_apply.REALP`) and can take it.
* **The four `pack_rt` adapters** (`encode_document`, `decode_document`, `encode_record_body`, `decode_record_body`) keep their names: they are the ABI that `dsl_derive` emits (`::semio_framework_os_kernel::os_pack::pack_rt::…`), `orc-defscan-after.txt` allow-lists them.
* **`os.pack.scalar-witness` schema** stays in the os tree: moving it changes schema ids, the schema document index and the catalog (generated files); the schema owner decides.
* **F-11(f)**: `ValueRefusalKind`, `PackTransportCategory`, `PagedRefusalMetadata`, `PagedAllocationMetadata` and `ValueError` were TypeScript imports from `🎒️pack/⚠️error`. No `*.ts`/`*.tsx` file outside the tickets imports any of the five from that module any more (`git grep`); the module exports `PackErrorData`, `packErrorMessage`, `PackError`; the importers use `🌱️value/⚠️refusal`. `PackTransportCategory` is a public enum in the Rust module `🎒️pack/⚠️error/🦀️.rs`; the other four names are not defined there.

## 7. Hand-offs

1. **`pi2` (`🔌️plugin/**`) and `rpv`/`rw2t` (`📺️renderer/**`): finish F-04(a).** 47 files, 34 manifests remain (`🗑️generated/rsl-residual-files.json`, `rsl-handoff-manifest-plan.json`: onboarding 19, time-travel-actions 9, action-bus 6, interaction 4 dependency lines). Commands, from the repo root:
   `python3 F/rsl-tools/facade_plan.py abi,action_bus,interaction,onboarding,history_edit,tool_run,platform,ui` (writes `rsl-plan.json`), then
   `python3 F/rsl-tools/facade_apply.py F/🗑️generated/rsl-plan.json action_bus,interaction,onboarding,history_edit --zone all --write` (one file `🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface` has a `use {semio_framework::action_bus, …}` form to do by hand), then
   `RSL_ZONE=all python3 F/rsl-tools/manifest_plan.py && python3 F/rsl-tools/manifest_apply.py --write`, then
   `cd F/rsl-tools && bun locks_apply.ts --write`, then delete the four aliases from `FAC` (`pub use … as action_bus/interaction/history_edit/onboarding` and their uses in the facade) and rerun the oracle families.
2. **Offline lock refresh**: `cargo update --workspace --offline [--manifest-path <ws>/Cargo.toml]` for the 161 locks of section 2.3 and for the root lock (engine bundle entry, command-tests -> kernel).
3. **Real checks** (section 8).
4. `pi2`/`phc`: `🔌️plugin/🖥️host/🧪️tests/💡️service-operation/🦀️.rs` uses a bare `dsl::` while its owner `semio-framework-plugin-host-wit` has no kernel dependency.
5. **Dead dependency** `semio-framework-source-projection` in `FAC`'s manifest (no use in the facade crate); removing it needs the edge removal in the locks (`propagateRemovals`).
6. One-item owner: the 6 laws of `canonical-error-progress` and the 9 of `emission` (list in section 5) pin shapes that changed in earlier carves.
7. `io_schema` (834 uses, 192 files), `sqlite_snapshot` (53) and `mesh_io` (28) are facade aliases of the same kind; not part of this task, and 135 + 35 + 1 of their uses sit in `🔌️plugin/**`. The F-04 tools take them (`facade_plan.py io_schema,sqlite_snapshot,mesh_io`).

## 8. Verification (commands run and their real results)

| Check | Command / tool | Result |
| --- | --- | --- |
| reach graph | `bun F/rsl-tools/reach.ts rsl-reach.json` | 2,552 manifests, 2,887 roots, 31,036 Rust files |
| plan | `facade_plan.py` | 204 files, 5 without owner (dead draw/raster files, text rewritten only); then 47 residual files, all skip zone |
| dependency completeness | `dep_check.py` | 156 files, 0 manifests missing a dependency |
| manifests | `bun F/kcf-tools/manifests.ts <120 manifests>` | problems 0 |
| locks | `locks_check.ts` | 2,005 placements, 161 locks, 0 violations; bundle and command-tests entries equal their manifests |
| duplicate mounts | `bun F/dm2-tools/modowners.ts F/🗑️generated rsl-modowners.tsv` | 2,558 manifests, 31,047 file-package pairs, **0 files owned by more than one package** |
| paths | max over the 285 touched paths | 224 bytes (limit 240), 143 characters (limit 256); new crate paths 108 to 145 bytes |
| rustfmt | `rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --emit stdout` on the 162 changed `.rs` files | 158 parse; 4 fail with the same message on the pre-edit copies (`generation2d`, `vdi3805`, `remodeling` editors, `puzzle/🧵️retained`): not mine |
| boundary corpus JSON | `python3 -c json.loads` after the edit | parses; launch files parse (`json.loads` on `launch.json`, `Bun.JSONC` not needed for the seed copy edit by anchor) |
| oracle, F-04 | `pos.py run rsl1..rsl7 --cfgs …` | {{F04}} |
| oracle, D3 | section 4.4 | see there |
| real cargo probe | `SEMIO_SCRIPT_POLICY='…' perl -e 'alarm 3300; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-store-engine-bundle -p semio-framework-os-store-engine --lib --keep-going --message-format=short`, mutex held and released | {{CARGO}} |
| TS | `bun test ./<file>` per file | section 5 |

Real-check commands for the compile queue (framework crates are members of the root workspace unless noted):
`bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-store-engine-bundle -p semio-framework-os-store-engine -p semio-framework-os-command-tests --lib --keep-going --message-format=short`, the same with `--tests`; the facade and its consumers in the framework workspace with `--manifest-path 🧰️framework/Cargo.toml`: `-p semio-framework --lib --tests`; plugin workspaces one by one with `--manifest-path ✏️s/🔌️plugins/<plugin>/…/Cargo.toml` after the lock refresh of hand-off 2. Expect, per `PLAN-OS-KERNEL-CRATES.md` 13.1, borrow-split errors in the engine crates that the oracle cannot see.

## 9. Files

Created: `S/⚙️engine/🧳️bundle/{🦀️.rs, 📦️packages/🦀️rust/Cargo.toml, 📦️packages/🦀️rust/📋️project.json}`; `F/rsl-tools/*` (all tools); `F/RESULT-RUST-LEFTOVERS.md`.
Edited (lists: `🗑️generated/rsl-rust-files.txt` 157 Rust files, `rsl-edited-manifests.txt` 120 manifests, `rsl-locks-changed.txt` 161 locks): `FAC`; `🛂️manifest/🧪️tests/🧩️framework-reserved/🦀️.rs`; `S/⚙️engine/📥️lifted/🏪️store/🦀️.rs`; `S/🧩️composition/📬️publication/🤝️group/🦀️.rs`; `S/⚙️engine/📦️packages/🦀️rust/Cargo.toml`; `📡️spr/🎮️command/🧪️tests/🧷️companion/📦️packages/🦀️rust/Cargo.toml`; root `Cargo.toml` (one member), `Cargo.lock`, `📋️build-owners.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, the crate-boundary corpus; the TS files of section 5.
No `[DEBUG]` line was added; the `[DEBUG]` console lines in the TS laws are committed test output.
