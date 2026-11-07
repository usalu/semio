STATUS: RED. hub-9 (rc 1): 4 red crates (decl-app, guest-contract, fem-2d-model, procedural-2d-editor-presence), 6 errors; ext.sh 3 running. Earlier:  hub-8 (rc 1, first full compile pass): single red crate semio-os-io (38 errors, first io-mechanism/🦀️.rs:540 E0308, receive_nested/RetainedWorkOutcome API mismatch), claimed by runtime 12:22; every downstream crate unchecked. Earlier:  hub-7 (rc 1) refused pre-compile at semio-s-artifact-procedural-2d-editor source cap (routed to proc); no per-crate compile census exists yet. Previous:  hub-6 (rc 1): past decl-app, refused at semio-s-artifact-flow-flow-editor (crate source ~400 KB vs ~262 KB cap; flow/coordinator decide split or closure drop). ext-2 bim (rc 1): framework errors pack-json:135 (fixed), sqlite-snapshot receiving:5, ui-contract-text receive:6 (statik, FIXED per claims file); remaining 20 ext crates queued behind a saturated fleet slot queue, UNVERIFIED.

# 1105b hub lane (compile integrator)

## Fixed
- Authorizer source closure overflow (files=1831, bytes 16788269/16777216; hub-1.txt, runtime-lower1.txt). Root cause: repo `commands/setup` imported the whole os/mcp barrel (~285 KB) just for `ensureMcpBinary`. Binary-path/staging section moved to `os/mcp/stdio/binary/mod` (`🌉️mcp/📡️stdio/📦️binary/🟦️.ts`); setup, mcp barrel, rust/ts package scripts, 4 mcp tests, resolvemcpbinarypath test and the staging tests repointed. Proof: hub-2 passed source admission (`admission-ok` touched). bun: resolvemcpbinarypath 18/19 (1 needs SEMIO_SCRIPT_PROVIDERS, env only), staging publication 19/19, staging 27/28 (1 fails on dev/🟦️.ts text, peer edit, not mine).
- Stray `ose();}` tail in staging publication test removed.
- Cargo manifests left behind by the crate-name rename sweep (hub-3/hub-4: `semio-framework-ui-contract`, `semio-framework-os-kernel` not in workspace.dependencies): 67 dependency keys in 21 manifests renamed to the new workspace names (script `C/1105b-tools/hub/stale.py`, mapping `C/🗑️generated/1105b/hub-renames.json`; sources already use the new idents). guest-contract now `semio-ui-contract = { workspace = true }`.

## Open
- hub-2 `hdiutil resize -size 98304m .cache/b/r/fixed.dmg` status 22: owned by lane disk (a99ac1d46b8fc8259); later runs passed storage.

## Runs
hub-1 (authorizer), hub-2 (hdiutil), hub-3 (guest-contract ui-contract key), hub-4 (os-kernel key), hub-5 (queued). Extension script: `C/1105b-tools/hub/ext.sh <round>` (21 crates, sequential, rc `ext-<round>.rc`).

## Verdicts
- hub-5 / ext-1-flow-extension-bim (C/🗑️generated/1105b/ext-1-flow-extension-bim.txt): Rust crate Source refused, semio-plugin-decl-app, reason files, remainingFiles 0. Every extension depends on decl-app, so ext.sh aborted after the first crate. Rerun hub-6 + ext.sh 1 after runtime's split.
- hub-6 (C/🗑️generated/1105b/hub-6.txt): flow-editor Source refusal, routed to flow (aefa042877b1cdaff), who confirms it needs a multi-crate split.
- ext-2-flow-extension-bim (C/🗑️generated/1105b/ext-2-flow-extension-bim.txt): rc 1, framework value-API errors above.
- hub-7 (C/🗑️generated/1105b/hub-7.txt): Source refusal procedural-2d-editor (flow-editor admitted now). Per-crate error table: none yet, admission aborts before compile.
- Static gate census (hub+ext roots, 1657 crates, normal cfg; C/🗑️generated/1105b/hub-census.json, roots C/1105b-tools/hub/roots.txt): ONLY semio-s-artifact-procedural-2d-editor is over cap (339543 B, 29 files, 4010 lines) -> routed to proc. flow-editor is now under cap. Test-cfg not measured by this tool.

## hub-8 red-crate table (C/🗑️generated/1105b/hub-8.txt, errs.py)
| crate | errors | first | owner |
|---|---|---|---|
| semio-os-io | 38 | 🚪️io/🧭️io-mechanism/🦀️.rs:540 E0308 | runtime (claimed 12:22) |
Downstream of os-io is not reached until it compiles; rerun hub-9 then.

## hub-9 red-crate table (C/🗑️generated/1105b/hub-9.txt)
| crate | errors | first | owner |
|---|---|---|---|
| semio-plugin-decl-app | 1 | 🛬️lifted/♻️retirement/🌱️birth/🦀️.rs:19 E0433 value_native_owned | runtime |
| semio-plugin-guest-contract | 2 | original-actor-context/🧊️cold-job:22 E0027 returned_source | runtime |
| semio-s-artifact-fem-2d-model | 1 | 🧬️schema/📸️snapshot/native.rs:99 E0308 | statik |
| semio-s-artifact-procedural-2d-editor-presence | 2 | 👥️presence/🦀️.rs:41 E0053 | proc |

## ext-3-flow-extension-bim (rc 1)
| crate | errors | first | owner |
|---|---|---|---|
| semio-plugin-runtime-ops | 20 | ⚛️reactor/🏃️runtime/🧰️ops/📄️document/🦀️.rs | runtime |
| semio-plugin-reactor-jobs | 4 | ⚛️reactor/💼️jobs/🦀️.rs | runtime |
| semio-plugin-instance-render | 6 | 🧵️instance/🖼️render/📥️staging/🏷️reserved-jobs/🦀️.rs | runtime |
Remaining 20 ext crates are running sequentially (ext-3).

## ext-3 so far (6/21 rc 1, all red from shared framework/runtime crates, tree moving under peers)
bim/brep: plugin-runtime-ops, reactor-jobs, instance-render; dictionary: instance-render, reactor-jobs, runtime-exchange; list/logic: framework-io-sqlite-snapshot (store lane); math: plugin-instance-app, reactor-context. All runtime/store owned, routed. No ext-specific crate error seen.

## hub-10 red-crate table (partial, run still in cargo:check at ~55 min; C/🗑️generated/1105b/hub-10.txt; tool 1105b-tools/gates/redtable.py)
| crate | errors | first / class | route |
|---|---|---|---|
| semio-s-artifact-stdio-docx-base-model | 3 | E0308 sqlite snapshot decode/encode vs RetainedWorkOutcome (ecma-376 base schema io/sqlite/snapshot) | stdio family (koordinator) |
| semio-s-artifact-stdio-pptx-base-model | 9 | E0053/E0107/E0277 same file family, receive 3 generics | stdio family |
| semio-s-artifact-stdio-xlsx-base-model | 9 | same | stdio family |
| semio-s-artifact-stdio-jpg-io-registry | 1 | io registry | stdio family |
| semio-s-artifact-gis-gismap-snapshot | 1 | E0308 sqlite/snapshot 🦀️.rs:34 | value (this lane) |
| semio-s-artifact-puzzle-5d-mut-any-5 | 1 | E0599 RetainedWorkOutcome<Puzzle5dSnapshot,..> at play-codecs:50 | value (this lane) |
| semio-s-artifact-puzzle-2d-text-inverse | 2 | E0046 missing trait item last_turn_work (editor kit 45,252) | value (this lane) |
| semio-s-artifact-procedural-2d-io-text | 1 | E0599 FlowHostCloseRefusal lacks Display (generation2d io text snapshot:233) | proc |
| semio-s-artifact-flow-flow-editor-leaves | 5 | E0631 type mismatch connect-media-ports mirror (flow-eval-tick, node-graph-edit, reorganize) + E0252 MAXIMUM_INTERACTIVE_ROWS defined twice | flow |

### hub-10 FINAL (rc 1, ~1h20m): 13 red crates
fem-2d-editor 1, flow-editor-leaves 5, gis-gismap-snapshot 1 (fixed-unverified by value), procedural-2d-io-text 1, puzzle-2d-text-inverse 2 (fixed-unverified by value: kit last_turn_work), puzzle-5d-mut-any-5 1 (fixed-unverified by value: play-codecs decode), stdio docx-base-model 3, html-snapshot 8, jpg-io-registry 1, mp3-mutations 1, pptx-base-model 9, tsv-snapshot 14, xlsx-base-model 9. Remaining are IoError.cause/RetainedWorkOutcome sqlite-codec ports in stdio (route: stdio family) and fem-2d-editor/flow-editor-leaves/procedural-2d-io-text (route: statik/flow/proc). ext.sh not started: it needs hub green first (each of 21 components re-walks the same shared graph, ~4 min each at best).

hub-10 follow-up (value): docx/pptx/xlsx base-model, jpg-io-registry, mp3-mutations, gismap-snapshot FIXED (renames-28/29 rc 0). Still open from hub-10: html-snapshot + tsv-snapshot (koordinator), fem-2d-editor (statik), flow-editor-leaves (flow, E0252 was jpg), procedural-2d-io-text (proc), puzzle 2d/5d fixes unverified until hub-11.

## hub-11 (W-hub-11, rc 1, ~1h; C/🗑️generated/1105b/W-hub-11.txt) red-crate table
| crate | errors | class | route / status |
|---|---|---|---|
| semio-s-artifact-gis-gismap-gis2d | 1 | E0046 last_turn_work (editor one-item-preparation) | value: FIXED-unverified |
| semio-s-artifact-puzzle-5d-store-preparation | 2 | E0046 last_turn_work (kit store-preparation x2) | value: FIXED-unverified |
| semio-s-artifact-procedural-3d-viewer-presence | 2 | E0053 decode_presence_native signature | value: FIXED-unverified (19 presence impls ported by script) |
| semio-s-artifact-stdio-idml | 10 | `semio_framework_pack::` -> lib is `pack`; decode/encode natives to RetainedWorkOutcome | value: FIXED-unverified |
| semio-s-artifact-stdio-html-snapshot | 8 | sqlite codec port | koordinator |
| semio-s-artifact-flow-flow-unit-tests | 3 | flow tests | flow |
| semio-s-artifact-puzzle-2d-v1 | 8 | E0433 `routes` in `io`; E0603 private module `tool_contract`; E0433 DslValue in semio_framework_dsl; sqlite decode/encode (E0053/E0107/E0277) | UNOWNED -> puzzle/aggregator |
Verification of the four value fixes: W-renames-30 (hub manifest, wasm).

## hub-12 (W-hub-12, rc 1; C/🗑️generated/1105b/W-hub-12.txt)
Only 2 red crates (everything else of the 13 hub packages compiles): gis-gisterrain-inferences 1 (E0308 sqlite snapshot ValueError->failure, fixed by value: .into()), puzzle-5d-puzzle5d 1 (E0603 private tool_contract::ArtifactReservedWork -> plugin_app_close_prelude, fixed by value). Both fixed after the run; proof W-renames-38 targeted check follows. Same private-module import exists (not in hub closure): cad tests geometry-media-and-owners, animate presentation reserved-import, layout reserved-import-job, draw clipboard job.

hub-12 closure: W-renames-41 (rc 0) proves gisterrain-inferences + puzzle-5d-puzzle5d; with W-hub-12's other 11 packages green, all 13 hub packages compile (hub gate GREEN modulo concurrent edits). ext.sh round 4 started.

## ext-4 (W-ext-4-*, 21 components, chk via 🌎️hub-independent workspaces)
Green so far (9 flow ext): bim, brep, dictionary, draw, list, logic, math, primitive, text.
| ext component | red crate | errors | first error |
|---|---|---|---|
| playbook-module-procedural | semio-s-plugin-playbook-procedural | 4 | E0425 `render_question_control` not found (playbook/procedural/🌊️embedded-flow:110); E0277 `?` cannot convert to OriginalTurnFailure (🧵️embedded-evaluation:96,129,183) |
| process-extension-concrete | semio-plugin-retained-command | 3 | E0433 `semio_framework_job_worker::WorkerJobCloseStep` not exported (defined in job/👷️worker/🧵️worker-job-session but not at crate root) |
Remaining ext components still running.

## ext-4/5 FINAL: GREEN
All 21 extension components wasm rc 0: 19 in W-ext-4-*, plus playbook-module-procedural and process-extension-concrete rerun as W-ext-5-* after proc/aggregator fixes. Marker 🗑️generated/1105b/ext-ok written. Hub gate (13 hub packages) green as of W-hub-12 + W-renames-41.

## Pre-freeze items (channel pin, wasm debug 0)

1. Channel pin (audit A5): 70 of 71 tracked `*/🧩️component/🔣️.json` say appChannelVersion 24, authority pin is 25 (`🧑‍💻dev/🔖️channel-version`). These are describe outputs; `repo run workspace:components describe <id> --profile prototype` rewrites them in the activation. No generator verb re-pins without building, so no pre-freeze re-pin is done. Residual risk (unverified): serve-time descriptor-pair validation against a stale pair if describe is skipped for a plugin.
2. Wasm debug 0 (audit D): no consumer needs DWARF (strip-debug guard strips before jco; describe/probe use exports). Applied via `1102-tools/actinfra/patches/prototype-wasm-debug0.py`: profile schema `prototype-wasm` debug 1 -> 0 plus 2 consumption fixture rows. Profile bun tests: 7 pass, 4 fail; failures are fs/path lookups (ENOENT Cargo.toml in the profile dir, browserPackages, sourceOwnership instr path), unrelated to the debug value, so kept. Not done: single-component size proof (build is not cheap under fleet load). Warning: the profile change invalidates wasm caches, so hub/ext re-verification will be cold.

## Profile bun tests classification
The 4 failures were cwd-dependent (tests read relative to process.cwd(), expecting the repo root). Run from the repo root: `bun test ./<profile>/🧪️tests/🟦️.ts` gives 11 pass, 0 fail with debug 0 applied. Nothing broken; no fix needed.

## cold-hub-1 (normal label, cold wasm caches)
RED (2 errors, rest compiled):
- semio-s-artifact-gis-gisterrain-editor: E0046 missing last_turn_work in Gis3dOneItemPreparation. OWNED, FIXED (last_work field, reset/set, fn last_turn_work) in gisterrain editor 🦀️.rs. Re-verify in cold-hub-2.
- semio-s-artifact-puzzle-5d-v1: E0433 cannot find `routes` in `io` at 🪆️subsets/✳️any/🦀️.rs:41 (`io::routes::entries()`); routes is declared in 🚪️io/🦀️.rs:236 (`#[path="📇️routes/🦀️.rs"] pub mod routes;`), both files rewritten Nov 5 05:36; the mirror crate resolves a different `io`. NOT MINE: routed to puzzle owner.

## puzzle-5d-v1 E0433 root cause and fix
`io::routes` is declared in the subset's 🚪️io/🦀️.rs (not inside cfg(test); cfg(test) at :108 only guards `mod tests`). The v1 crate does not mount that file; its 📦️packages/🦀️rust/🪞️mirror/🦀️.rs defines a separate `io` with only text/binary, and `routes` lives in the `semio-s-artifact-puzzle-5d-tests` crate's io (which mounts 🚪️io/🦀️.rs). Fix: mirror `io` now has `pub use semio_s_artifact_puzzle_5d_tests::standards::v1::subsets::any::io::routes;` (v1 already depends on the tests crate). Verify in cold-hub-3 (cold-hub-2 may have started before the edit).

## Value section (value-edit transfer and named receiving)
Native `semio-framework-value-edit --lib` (vedit-5): 15 passed, 0 failed.
- KeyPreservingPersistentSet 4096 B hop: Self was 2464 B (admission 656 inline after boxing, PersistentSet 1344 inline), constructor_demand 5568 B. Fix at the layout: `admission` and `set` are now `Box`ed; creation quotes `capacity_bytes` and reports `retained_capacity_bytes`; each box has its own paid close step (`release_bytes`, `released_bytes`). Hop untouched. (index/♻️transfer/🦀️.rs)
- 256 vs 257 (receive test): the hook was right, the test wrong. Persistent-list work hands children `remaining()` (items already spent), so the hook sees a sub-grant. Test now asserts `0 < items <= original` like the other axes.
- KeyPreservingPersistentSet::new "lost its original ...": not an ordering bug of the close split (fails before any close). Stale tests ran `index.project` once; it is multi-step, so the slot was an unready alias. Loops now run until `ready()`.
- receiving `Option::unwrap` on None: `e.primary` None because the SealedShared field receiver asks the construction recipient for a grant and the test recipient always refused. Test recipient `grant()` now returns a grant clipped to the original explicit grant per axis.
- Temporary [DEBUG] size print removed. Wasm check: vedit-wasm (see claims).

## Cold round (post debug0)
cold-hub-3: GREEN (rc 0). cold-ext-1: 21 components run; reds were flow-dictionary (native-encoding RefCell, fixed by nativetests; rerun cold-ext-2-flow-dictionary rc 0) and cad-aec-structure (transient peer edit in window-config typed-pack-load; rerun cold-ext-2-aec-structure rc 0). All 21 extensions green after reruns; caches are warm for the freeze re-verify.
Confirmed: cold-ext-3-sourcing-windows rc 0 (after runtime restored retained-command imports). All 21 extensions green and cold-hub-3 green after the debug0 cold start; caches are warm for the freeze re-verify (W-hub-13 + wext).
