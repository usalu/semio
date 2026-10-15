# Result: hub directory (A) and hub inference runtime (B) split

Executor: Sonnet, foreground, no sub-agents. Seam map: `SEAM-MAP-HUB-REPO-SURFACE.md` sections A and B. Paths: `hub/` = `🌎️hub/`.

Status: both files are split (pure structural, public paths stable). **No compile was possible** (see section 7): verification is the full-resolution rustc differential, losslessness, region markers, path resolution, rustfmt, emoji admission and a definition scan. No scratch cargo workspace, manifest, private target directory or `RUSTC` override was created or used (the coordinator's new fleet rule); the only compiler runs were `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null` oracles, so there was nothing to delete.

## 1. Line counts (raw / active = non-blank, non-comment)

| File | Raw | Active |
| --- | --- | --- |
| A parent `hub/📇️directory/🦀️.rs` (was 4,355 / 3,488) | 240 | 156 |
| `🧾️model` | 631 | 463 |
| `🎟️capabilities` | 812 | 674 |
| `🔤️wire` | 431 | 352 |
| `🕰️events` (new, not in the map) | 54 | 33 |
| `🧭️decider` | 244 | 198 |
| `🛠️service` | 862 | 750 |
| `🔌️port` | 328 | 193 |
| `🔀️dispatch` | 804 | 697 |
| B parent `hub/💡️inference/🏃️runtime/🦀️.rs` (was 3,956) | 135 | 94 |
| `🧾️approval-contract` | 417 | 294 |
| `🌍️gis-map-binding` (new, GIS child) | 125 | 114 |
| `🔐️committer` | 927 | 844 |
| `🧱️document-preparation` | 289 | 284 |
| `🔁️commit-turns` | 404 | 397 |
| `🔎️checkpoint-verify` | 249 | 242 |
| `📕️commit-and-close` | 371 | 362 |
| `🧭️job-types-and-port` | 192 | 164 |
| `🚧️checkpoint-test-gate` | 158 | 140 |
| `🏭️inference-runtime` | 370 | 332 |
| `🛣️routes` | 402 | 364 |

Every child is far below 2,500 active lines; the largest is `🔐️committer` at 844. Longest new path is 109 bytes. The backends (`🪶️sqlite`, `🐘️postgres`, `🌐️neo4j`) and the two test files are untouched and still mounted from the A parent.

## 2. What was done

**A.** Children live under `hub/📇️directory/<folder>/🦀️.rs`, mounted from the parent at the position of the code they replaced with `#[path = "<folder>/🦀️.rs"] mod <name>;` plus `pub use <name>::*;`. The parent keeps the docs, the inline `error` module, the shared `use`/const/`ArtifactCasSweep*`/`ProjectionRebuildControl` block, the backend mounts and the test mounts. `model` was an inline module; its body is now the file (dedented 4 spaces; `pub mod model;` in the parent, `super::SessionCapability` etc. still resolve because the child is still directly under `directory`). Children use `use super::*;` (same convention as `RESULT-PLUGIN-INSTANCE-HOST-SPLIT.md`), except `events`, which imports its three `os_directory` names explicitly. `capability_type!` and its uses stay together in `capabilities`. Deviations from the map: the `Events` region is its own child (`🕰️events`, 54 lines) instead of being folded into `decider`, because it is the leaf the cycle resolution needs (section 3); `capped_socket_session_binding` (after the Service `//#endregion`) went into `service` with the rest of the range.

**B.** Eleven children (the map's ten plus the GIS child) under `hub/💡️inference/🏃️runtime/`. The committer `impl` was cut at method boundaries into `committer` (new .. resume_recovered_checkpoint, lines 505-1426 of the original), `document-preparation` (prepare_retained_document, build_assembly, restored_stores), `commit-turns` (drive_turn .. drive_abandoned_request), `checkpoint-verify` (finish_preflight .. verify) and `commit-and-close` (commit_retained .. close). Each lower band re-opens `impl RetainedGisMapApprovalCommitterV1 {` and closes it; the first band closes it with `}`. Every `super::` path inside moved code became `super::super::` (the children are one module deeper; 44 lines, including the doc link), so `inference` siblings (`schema`, `sqlite`, `command`, `wal`, `catalog`, `authorization`) resolve exactly as before. The parent keeps imports, the seven consts, the helper fns (`approval_stamp_v1`, `document_key`, `fresh_now_ms`, `parse_frontier_chain_hash`, `approval_actor`, `approval_mutation_id`, `reader`) and the test mount. Deviation: the map names the gate child `🧪️checkpoint-test-gate`, which repeats the sibling `🧪️tests`; it is `🚧️checkpoint-test-gate`. The gate module and its re-export keep `#[cfg(feature = "integration-fixtures")]` (the items keep their own cfg too). Re-export globs are `pub use` where a child has `pub` items, a private `use` for `gis-map-binding` (all `pub(super)`), and none for the four impl-only children (a glob there is an unused-import warning).

**Visibility.** Only `pub(super)`, never wider, applied by `dirinf-tools/widen.py` (name-based superset: a private top-level item, named struct field, tuple-struct field or inherent method is widened when its name occurs as an identifier, including format-string captures, in any other file of the module tree, the sibling children, the parent, and the test/backend files that are descendants of the module). A: 43 definitions (decider fns 6, wire fns 4, service fields 14 and members 5, ...). B: 193 (committer fields 39 and members 25, gis-map-binding items 19 and fields 25, job types fields 21, ...). The list is regenerable: `dirinf-tools/rebuild.sh` then the report files `widen-*.report` in the scratch dir. A dry run on the final trees proposes 0 further widenings.

## 3. The `artifact_authority` cycle

Measured. `directory` -> `artifact_authority`: the shared use block (chunk_cas, creation) plus `wire` (checkpoint ids, creation validation, CAS publication, creation receipt) and `service` (CAS sweep, `OperationContext`, `AuthorityError`, `VerifiedCheckpointPublisher`). `artifact_authority` -> `directory` (non-test): `error::{DirectoryError, DirectoryResult}` (4 files under `🌱️creation`), `NewDirectoryEvent` (`🌱️creation/🧬️schema`), `model::{SocketSessionBindingStatus, SpaceRole}` and `{DirectoryService, HubDirectory, HubVerifiedCheckpointPublisher}` (both in `🌱️creation/🧑‍🏭️service-v1`). Test-only: `crate::directory::ARTIFACT_CAS_*` (chunk-cas unit test), `crate::directory::MemoryArtifactProjection` (creation unit test).

Direction chosen: `artifact_authority` is lower, because the forward edges are 52 lines of authority types that cannot move without relocating `chunk_cas`/`creation`.

Resolved:
- `error` (inline in the parent), `model` and the new `🕰️events` are leaves with no dependency on `artifact_authority` (model reaches only back to the four capability types it embeds through `super::`; `events` imports three `os_directory` names and nothing else). Everything `artifact_authority` needs from `directory` except the service edge below lands in these three, and the paths are unchanged (`directory::error::*`, `directory::model::*`, `directory::NewDirectoryEvent` via the parent glob).
- The one call site that went through the facade was updated: `hub/🗿️artifact-authority/🌱️creation/🧬️schema/🦀️.rs:175` now names `crate::directory::events::NewDirectoryEvent` (`events` is `pub(crate) mod`). All other `crate::directory::error::` / `::model::` call sites already named the leaf path.

**Not resolved (needs a public-path change, so out of scope here).** `ArtifactCreationServiceV1` (`artifact_authority::creation::service`, re-exported as `artifact_authority::creation::ArtifactCreationServiceV1`) is built on `Arc<DirectoryService>` and calls `HubDirectory` methods; it is an upper-layer service living in the lower module. It cannot take an item of `directory::{service, port}` into `artifact_authority` (that would be absurd) and moving it to `directory` changes its path (consumers: `🏗️bootstrap/🌱️artifact-creation`, `🏗️bootstrap/🧠️state`, `🏗️bootstrap/🦀️.rs`, `🧪️tests/🔬️bin-unit`; the bootstrap agent owns those). Follow-up: move `service-v1` under `directory` (or above both), re-point those four files, drop the `pub use service::ArtifactCreationServiceV1;` re-export in `🌱️creation/🦀️.rs:17`. Together with the two test-only edges this is the complete remaining cycle.

## 4. GIS leak (recorded, relocation is a follow-up)

`🌍️gis-map-binding` groups the GIS-concrete top-level items that could be cut cleanly: the thirteen `GisMap*` type aliases to `semio_s_artifact_gis_gismap`, `..._stdio_semio_drawing`, `..._stdio_semio_value_*`, `GisMapDocumentStoresV1`, `GisMapCommitIdentityV1`, `GisMapCommitOperationV1`, `GisMapClosingStoresV1`, `deterministic_map_inference` and `gis_map_inference_preview`. It is the first thing a hub-side GIS plugin crate takes. The domain is more pervasive than one child can hold because the methods that use it sit inside the committer `impl`; mentions of `GisMap`/`gis_map`/`semio_s_artifact` per file: gis-map-binding 34 / 11 crate paths, committer 182 / 8, commit-turns 136, commit-and-close 77, checkpoint-verify 69 / 1, document-preparation 62 / 4, inference-runtime 44 / 8, approval-contract 46 / 2, routes 20 / 3, job-types-and-port 12 / 1, checkpoint-test-gate 0, parent 4. The real extraction seam is the existing ports `GisMapApprovalCommitterV1` and `GisMapApprovalIngressAuthorityV1` (`approval-contract`), with `db::Database`/`DbBackend` concretes still to be put behind a port.

## 5. Source-text readers

Searches: `/usr/bin/grep -rn --exclude-dir=node_modules --exclude-dir=.🧬semio --exclude-dir=target --exclude-dir=.git -F` for `📇️directory/🦀️` over `🧰️framework ✏️s 🌎️hub 📜️script.ts` ran to the end of `🌎️hub` (log `🗑️generated/dirinf-readers-A.log`; `📜️script.ts` at the repo root was the last argument and was not reached, but the hub package's own script was searched directly). The B pattern (`inference/🏃️runtime`, `🏃️runtime/🦀️`) reached `🧰️framework` and part of `✏️s` before I stopped it (`dirinf-readers-B.log`); every hit there is the plugin `⚛️reactor/🏃️runtime` (a different file) except the fixture below; the hub side of B was searched directly (separate-segment `"🏃️runtime", "🦀️.rs"` form, which the map's pattern missed). Hits that mention `📇️directory/🦀️.rs` in the framework and `✏️s` are the framework's own `🔨️modules/📇️directory` module, not the hub file.

`RESULT-SOURCE-TEXT-READERS.md` does not exist, but the helper it was meant to document does: `readModuleTreeSource` / `listModuleTree` in `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts` (parent first, then `#[path]` mounts in declaration order, `🧪️tests` mounts excluded). Re-pointed with it:

- B, `hub/📦️packages/🦀️rust/📜️script.ts`: the four runtime reads in `proveGisMapApprovalUndoFixture`, `proveGisInferenceRetainedRuntimeFixture`, `proveGisInferenceCheckpointControlFixture` and `proveGisMapProposalApprovalFixture` now use `readModuleTreeSource(...)`.
- A: for the directory file the helper would also pull in the three backends (they are `#[path]` mounts of the parent, and the same gates read the backends separately, where first-occurrence `indexOf("fn ...(")` and negative checks would change meaning). I added `readHubDirectorySource(repoRoot)` next to `orderedDirectoryPublicationOracle` in `hub/📇️directory/📣️publication/🧪️tests/🧾️ordered-append-broadcast/🟦️.ts` (parent plus own children via `listModuleTree`, sqlite/postgres/neo4j filtered out). It replaces the read in that oracle and the three reads in `📜️script.ts` (`proveRetainedShortAdmin` ~3025, `proveInviteRedemptionTransaction` ~16613, `proveDirectorySpaceJourneyV1` ~17083); the script imports it with the oracle.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🧫️fixtures/🔣️.json` (`rustReceivers`, files that must parse and receive the digest): the runtime parent no longer mentions the digest, so it was replaced by its nine children that do; `wire`, `service` and `decider` were added next to the directory parent.

Left alone (not in scope, listed): `hub/🧪️tests/🔬️bin-unit/🦀️.rs:1053` has a doc comment naming `📇️directory/🦀️.rs` for `decide_verified_checkpoint`, which now lives in `🧭️decider`; comments in `🪶️sqlite`, `🐘️postgres`, `🌐️neo4j` cite `📇️directory/🦀️.rs`'s `//#region 🔖️Backends`, which is still in the parent. `📦️packages/🦀️rust/📋️project.json` already globs `{workspaceRoot}/🌎️hub/**/*.rs`, so the new children invalidate the cache.

## 6. Verification actually run

| Check | Command or method | Result |
| --- | --- | --- |
| Full-resolution differential | `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --crate-name semio_hub --emit=metadata -o /dev/null --error-format=short -W unused-imports [--test] [--cfg 'feature="integration-fixtures"'] 🦀️.rs` run from `hub/📦️packages/🦀️rust/` (the lib root; the bin does not mount either file) against a sandbox holding the original files (`dirinf-tools/mksbx.py`), and against the real tree after install. Messages compared as multisets with line numbers stripped and child paths mapped back to the monolith. | native 1,517 vs 1,518 errors, `--test` 3,033 vs 3,034, feature 1,589 vs 1,590, test+feature 3,096 vs 3,097 (3 / 1 / 3 / 1 warnings, identical). In every configuration the only difference is one additional `E0433 cannot find module or crate directory`, which is the explicit `use directory::os_directory::{...};` line in the `events` leaf. Everything else is identical: no new E0412/E0422/E0425/E0423/E0603/E0432. |
| Oracle sensitivity | Same differential with widening switched off | 317 / 354 new E0422/E0425 (private structs and fns across children), so the oracle does see top-level visibility. It cannot see fields and methods; those rest on the name-based definition scan and the idempotent dry run (0 pending). |
| Losslessness | `dirinf-tools/verify.py`: multiset of stripped non-blank lines, `pub(super) ` and `super::super::` undone | A: 0 lost except the two removed lines `pub mod model {` and its `}`. B: 0 lost. Extras are only mount lines, docstrings, `use super::*;`, one explicit `use` in `events`, and four re-opened `impl ... {` / `}` pairs. |
| Region markers | counts of `//#region` / `//#endregion` | A 32 / 21 before and after, B 1 / 1 before and after; the markers moved with their regions. |
| Path resolution | every `#[path]`, `include!`, `include_str!` in the 21 files | 31 checked, 0 missing. |
| rustfmt | `rustfmt +nightly-2026-07-20 --edition 2021 --emit stdout` on each new file | A: parent and all 8 children parse. B: 6 of 11 children parse; the 5 that fail (committer, document-preparation, job-types-and-port, inference-runtime, gis-map-binding) fail on `<_ as ArtifactPack + ArtifactStoreSnapshot>::decode_pack`, which is already invalid on `HEAD` (rustc reports the same 8 `expected one of (, ::, <, or >, found +` parse errors on the untouched original; lines 1112, 1246, 1454, 1485, 2795, 3224, 3275, 3382 of the original). Not mine, not fixed (no behaviour change). The hub library does not parse today because of it. |
| Emoji admission | bun `/^\p{RGI_Emoji}$/v` (with or without U+FE0F) | 17/17; all folders end in U+FE0F, unique among siblings, each already used as a folder prefix elsewhere (`git ls-files`). |
| Ordered-append oracle (TS) | `orderedDirectoryPublicationOracle(repoRoot)` via bun, before and after | `9` before, `9` after (reads the split tree; backends excluded). |
| Literal/slice equivalence for the `script.ts` gates | every string literal of the seven touched `prove*` functions counted in the original vs the concatenated tree; the two B marker slices checked for order | Gate-relevant literals identical; the only differences are words in the new docstrings, the four re-opened `impl` headers and `🦀️.rs` path strings. Both B slices keep start before end. |
| `[DEBUG]` | grep | none. |

## 7. Not verified, and why

- **No cargo/rustc build or test of `semio-hub`.** The compiler-ingress source-boundary gate refuses the framework crates; the oracle's extern-crate errors hide type checking, so field/method privacy (E0616, E0624), trait-in-scope for method calls, and `private_interfaces` are checked only by the definition scan.
- **The `script.ts` gates and `bun repo run os-hub:*` could not run end to end.** `bun repo run` dies in graph construction (`Canonical native dependency owner required: 🧰️framework/🔨️modules/◻️2d/⚙️engine/...`, a peer's change); a harness that exports the gate functions from a copy of `script.ts` fails at baseline for unrelated reasons (missing fixture `🌎️hub/📇️directory/🧪️tests/🏛️retained-short-admin/🔣️.json`, stale schema catalog). The checks in section 6 stand in for them. `dirinf-tools/gates-make.py` and `gates-run.sh` re-run them once those two blockers are gone.
- The descriptor-digest tree-sitter test was not run.
- The bin (`os-hub`) was not compiled; it reaches both modules only through `semio_hub::directory::*` / `semio_hub::inference::runtime::*`, which are unchanged paths.

## 8. Risks and finishing steps

1. First real compile: expect only `pub(super)` gaps in places the name scan could not see (a method called on a value whose type is never named in the other file is covered, since the scan is by name only; a path-qualified use through a trait is not). Fix by adding `pub(super)` at the definition; never by qualifying paths. Expect `unused_imports` warnings from the parent `use` blocks that now only serve children, and `private_interfaces` warnings.
2. The glob re-exports (`pub use <child>::*`) would show E0659 only under real resolution; the differential reports none, and the duplicate-name scan across the 21 files found none (apart from the `cfg(unix)`/`cfg(windows)` pair of `inherited_inference_checkpoint_file`, which stay in one file).
3. After the gate blockers clear: `bun repo cargo --store=shared --configuration=prototype -- check --manifest-path 🌎️hub/Cargo.toml -p semio-hub --lib` (+ `--tests`, + `--features integration-fixtures`), then the five `os-hub` check commands (`directory-ordered-publication-check`, `invite-redemption-transaction-check`, `retained-short-admin-check`, `gis-map-proposal-check`, `space-journey-check`).
4. The `<_ as A + B>` parse errors in the runtime must be fixed by whoever owns that sweep before anything compiles.

## 9. Files

Edited: `hub/📇️directory/🦀️.rs`, `hub/💡️inference/🏃️runtime/🦀️.rs`, `hub/🗿️artifact-authority/🌱️creation/🧬️schema/🦀️.rs` (one path), `hub/📇️directory/📣️publication/🧪️tests/🧾️ordered-append-broadcast/🟦️.ts`, `hub/📦️packages/🦀️rust/📜️script.ts` (4 + 3 reader lines and one import; the peer's concurrent edits in that file were left intact), the descriptor-digest fixture `🔣️.json` above. Created: the 8 directory and 11 runtime child files listed in section 1. Tools (originals, generators, oracle, verifier): `🪓️monolith-split/dirinf-tools/` (`orig-directory.rs.txt` sha256 `3085a23b...`, `orig-runtime.rs.txt` sha256 `bf12e52f...`, both equal to `HEAD`); regenerate and rebuild with `rebuild.sh` / `buildall.sh`. Logs kept in `🗑️generated/`: `dirinf-readers-A.log`, `dirinf-readers-B.log`, `dirinf-ts-baseline.log`. Scratch sandboxes were deleted.

## Hardening (patched oracle)

Wave 2, Sonnet executor, 2026-10-15. Full write-up: `RESULT-HARDEN-WAVE2.md`. No repository source file was edited; no regression was found.

**Method and baseline.** `rustc ... --crate-name semio_hub --crate-type lib --emit=metadata -o /dev/null --error-format=json` from `🌎️hub/📦️packages/🦀️rust/🦀️.rs` (the lib root is copied into the sandbox so `../../` stays inside it); module files copied with the patches applied identically to baseline and split (0 `extern crate` and 0 external globs in this tree, so the patch is a no-op here and the unpatched run gives the same totals). Module counts 82 (baseline) vs 101 (split): 8 directory + 11 runtime children. One baseline sandbox holds both originals at once: `orig-directory.rs.txt` and `orig-runtime.rs.txt` (both equal to `HEAD`). Identical fixes applied to the baseline as in the children: the 8 invalid qualified paths `<X as pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot>::decode_pack` rewritten to `<X as pack::snapshot::ArtifactPack>::decode_pack` (checked against the children: lossless baseline vs split family leaves only the removed `pub mod model {` line), and `🗿️artifact-authority/🌱️creation/🧬️schema/🦀️.rs:175` reverted to `crate::directory::NewDirectoryEvent` (the split points it at `directory::events::`). The two families fold to `FAMILY0` (directory) and `FAMILY1` (runtime).

| Config | Baseline / split errors | Non-extern | Differing keys |
|---|---|---|---|
| native | 1,538 / 1,539 | 34 / 34 | 1 |
| `--test` | 3,059 / 3,060 | 83 / 83 | 1 |
| `integration-fixtures` | 1,610 / 1,611 | 36 / 36 | 1 |
| `--test` + `integration-fixtures` | 3,122 / 3,123 | 85 / 85 | 1 |

The one difference in every configuration is `E0433 cannot find module or crate directory` at `📇️directory/🕰️events/🦀️.rs:3` (`use directory::os_directory::{DirectoryActor, DirectoryEventBody, Hlc};`). It is the unresolved external crate `directory`, a Cargo dependency (`Cargo.toml:67`, package `semio-framework-os-kernel`); the parent imports the same three names at `🦀️.rs:50`. This is the single extra line the first differential in section 6 also reported, now visible at its true size next to the other 1,537 / 3,058 errors, all identical.

**Fixes.** None needed. The test-config cfg imports, `super::super::` depths and `include!` paths of the 19 children all resolve: the split family contains no unresolved in-crate name that the baseline does not also have.

**Definition scan.** Directory family (9 files) and runtime family (12 files) scanned separately inside the original module subtree (the backends and tests are descendants of the directory parent, so they count): fields 14 private / 122 narrowed, members 24 / 62, top-level items 54 / 96 in total. Candidates: 0 members, 0 items, 0 tuple constructors in both. This includes the committer `impl` that is cut into five bands: every private member of one band used from another band is already `pub(super)`. Seeded: stripping 30 `pub(super)` items in `🔐️committer` adds 80 oracle errors (72 E0433, 4 E0425, 4 E0422); additionally stripping its 39 narrowed fields makes the scan report 146 member rows and 11 item rows (0 on the real tree). Stripping the single `pub(super)` item of `🛠️service` changed nothing, which shows that widening is not load-bearing.

**Residual.** Trait-in-scope for methods called on `directory`/`pack` values and `private_interfaces` are not produced by rustc here (no typeck, all externals unresolved); the lib was only analysed, never compiled by cargo. The `ArtifactCreationServiceV1` cycle of section 3 is unchanged. Tools: `harden2-tools/` (`ho.py` target `hub`, `hub-dir-base.rs`, `hub-rt-base.rs`, `hub-schema-base.rs`).
