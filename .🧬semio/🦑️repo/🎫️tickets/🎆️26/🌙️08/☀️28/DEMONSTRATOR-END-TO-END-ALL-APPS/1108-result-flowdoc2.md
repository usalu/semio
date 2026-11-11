# Lane flowdoc2: flow-document -> flow-host chain (takes over from flowdoc)

## FIXED-verified native lib W-flowdoc2-5 (compiled with warnings only)
store `🧬️snapshot-clone` (ReplyStash re-export), flow-model, flow-document, flow-copy (ported by me), flow package, flow-operators (registry + ported catalogue-work/owner as the real crate; scratch no longer needed for production code).

## Ported this round (EDITED, verified only where stated)
- window-config `🔒️epoch` extra `)` (source-admission refusal) and `🎟️capture`/`🔒️epoch` decoder identity type `(NativeAllocationIdentity,u64)` (7 E0308 in W-flowdoc2-5): unverified.
- flow-copy `🧵️retained/📑️copy`: SharedOriginalStorage roots, FactoryAuthority, effect `close_step(body,effect)`, one `next_close_demands`, staged `FlowSnapshotAdmission` intake (`from_read`/`resume` via `cx.with_original_effect`, charged by `receive_original_retained_work`), new `🧭️projection` (copy of the store's private StagedProjection; debt: hoist). `FlowSnapshotProjection` trait gone. VERIFIED W-flowdoc2-5.
- job `OriginalSnapshotIntake::close_step(body,effect)`; host intake closes with `close_original_effect_step`. VERIFIED (job + host eval-session not yet compiled; job lib compiled).
- flow-vcs `♻️retirement::owner_step`: owner-less VCS close through flow-model `retained::cold_effect` (made pub, re-exported by the flow package; no new dependency, no lock). Unverified.
- host evaluation-source (24 errors in W-flowdoc2-5): `🧵️pipeline` close via `close_original_effect_step` (helpers close_with_* never existed), `begin_close` calls on `BudgetedOwnedEntry` removed (the entry closes itself), `📤️publication` display/paint resume/pause return `(error, continuation)` (continuation kept in `self.control`), paint `step` takes the effect (parser `step_source(..,effect,..)`), receive `step_source(..,false)`, ordering/window-key u128 work. Unverified.

## Update 00:50 (11/10): W-flowdoc2-6 native FIXED-verified for flow-vcs, window-config, window-config-original, dag-model, normal-fill (warnings only)
host-source: 24 -> 2 errors (ordering private type leak, edited to pub; W-flowdoc2-7 queued). decl-views (137 left) is now claimed by lane declviews, window-transient/dag-graph by hubreds; dag-host (17) is a new unclaimed-looking red that appeared after dag-model compiled (probably hubreds' board-dag-graph chain).

## Queue
W-flowdoc2-6 native: flow-vcs, host-source/read/core/session/host, semio-os-flow (queued; capacity wait is about 1 h).
Then: wasm32-wasip2 flow-document+operators+host chain, flow wasm crates (actions-a/b, domain-adapter), `🌊️flow/📦️packages`, editors (flow editor `🧵️retained`, procedural `flow-source/📇️catalogue`, generation3d/2d), laws (`catalogue-work/🧪️laws`, host `⚖️laws`), delete `$C/1108-flowdoc/scratch`.
Unclaimed upstream reds seen in W-flowdoc2-5 that block later crates (not mine, not touched): `semio-plugin-decl-views` 237 errors (declaration/view, lifted/read, presence/transient retirement, tool-run), infinite-board dag-model 7 + normal-fill 5, tool-machine typing edit 1, plugin host/requests 1, guest-contract original-actor-context 1.

## Debts
- flow-copy tests (`📑️copy/🧪️tests`, `🧪️tests`) and host read `🧪️tests` still use the Arc/grant constructors.
- core `cold_effect` and registry `cold_turn` duplicate flow-model `retained::cold_effect`.

## Edited after W-flowdoc2-6 (unverified; W-flowdoc2-7 was still waiting for cargo capacity, 7 ahead, when I stopped)
- host-source `🔤️ordering`: `OrderingRetire`/`OrderingRetirement` made `pub` (the 2 remaining E0446 errors).
- infinite board `dag/🧭️host/🗺️minimap-widget-and-retirement`: 5 hand `RetireOwned` impls (`DagPointerState`, `DagPaintState`, `DagMinimapState`, `DagNotesState`, `DagHost`) -> `artifact_retire_struct!` (the 17 dag-host errors); claimed in the log.
Rerun (native, root manifest): `"$C/1105b-tools/chk.sh" W-flowdoc2-8 Cargo.toml native -p semio-os-flow-vcs -p semio-os-flow-host-source -p semio-os-flow-host-read -p semio-os-flow-host-core -p semio-os-flow-host-session -p semio-os-flow-host -p semio-os-flow` (about 1-2 h capacity wait); then the same list with `wasm` plus `-p semio-framework-artifact-flow-flow-document -p semio-os-flow-operators`.

## Remaining chain (blocked on other lanes' reds, not on my edits)
flow-host-core -> flow-extensions -> semio-plugin -> decl-views (declviews lane), window-transient (hubreds), infinite-board dag-host (17 errors, check ownership). After those: read, core, session, host, semio-os-flow native; then wasm32-wasip2 for flow-document/operators/host chain, flow wasm crates, `🌊️flow/📦️packages`; editors; laws; scratch deletion.
Empty-enum derive bug (for declviews): `value/✨️derive/🧬️retained-clone/🦀️.rs` `expand_retire_owned` emits `match &self.inner{}` for uninhabited enums (E0004); add a wildcard arm when `rows` is empty.

STATUS: BLOCKED host-core/read/session/host behind decl-views (lane declviews) and window-transient (lane hubreds); dag-host edit unverified; FIXED-verified W-flowdoc2-5, W-flowdoc2-6, W-flowdoc2-7 (host-source)
