# Retained Production Reactor Lifecycle

## Current Qualification

The production lifecycle implementation is landed but **not yet native-qualified**. The end-to-end goal remains active. Cold canonical checkpoint pages and the real GIS renderer have not been wired through this lifetime yet.

The registered `@semio-tech/framework-plugin:guest-lifecycle-check` passed its five neutral admission cases with an independent AJV predicate (44529, 71880). The initial source gate47316 was intentionally RED because the real kernel reducer was component-only and inaccessible to native tests. These source checks are not native runtime evidence.

Native exact group53937 / `reactor-lifecycle-native/exact-cargo-laws-rPBwm0/00` reached a toolchain linker failure: macOS `ld` returned EINTR opening already-built Rust archives while linking dispatch macros. The capture reader also got EINTR and left its wrapper alive. After confirming Cargo was absent, root terminated only its identified Bun wrapper40228; the Nx session then ended exit1. No native lifecycle law ran. Warm retry93401 is active on the root-owned `reactor-lifecycle-native-target`.

## Production Changes

- Extracted the actual reducer into `⚛️reactor/🔄️turn/🦀️.rs`; the WIT bridge calls this same reducer. Kernel request-outcome conversion is now available natively.
- Replaced the numeric lifetime and copied in-flight-close globals with a fixed typed registry in `PluginRuntime<PA>`, mounting `GuestLifecycleCell<NativeLifetimeOwner<PA>>`.
- The owner retains the exact native close lease, reserved/active/released participant stages, and the captured request. Foreign close validation and all capacity checks precede native close admission. The duplicate `plugin_destroy_app` call is removed.
- Open preflights actor, instance, quarantine and lifecycle rows before the real factory. Actor failure cannot leave a created live app without its lifecycle. Any occupied modulo slot rejects before advancing the guest serial.
- Captured and Accepted receipts are retained and require exact ACKs. Retired requires native lease terminal emptiness plus exact reactor/patch/pending close completion. Terminal resource release is one participant per bounded step and remains tracked through retry.
- Both native and WIT paths use the same output transaction: lower the owned turn result first, then consume staged lifecycle ACKs after the real clock verdict. An ordinary conversion error leaves the ACK/receipt mounted.
- Command ingress now retains the exact NativeCloseKey and participates in reactor close completion. The close path incrementally retires command pages/cursors without dispatching a closing app.
- UI intent admission/dispatch and task resumption check Live authority. Typed-operation continuation skips mounted non-Live cells; non-Live render work does not drain presence.
- Native compilation exposed a borrowed mounted-render grant escaping TLS. The grant now retains the same Rc-owned patch tracker state through awaited rendering; no second tracker is created.
- A sparse active-close cursor advances real occupied close work rather than wasting 1023 empty turns between cleanup units.

## Native Laws and Launch Ownership

The real runtime laws are included inside the existing private plugin-builder contract module. They use its real synthetic fluent plugin bundle, not a new PluginApp mock. They cover actual Captured/ACK/open actor state, early close refusal, exact single native close generation, Retired terminal proof, bounded final ACK release, foreign lifetimes, modulo collisions, actor setup rejection, and stale ACK after reopen. Existing lifecycle clock/late-release laws run in the same group.

The permanent command lives in the plugin Rust package's `📜️script.ts`, with a corresponding `📋️project.json` target and source/native launch seed411.011/.012. Home regenerated the normal registry and confirmed freshness (59 plugins,60 playgrounds,45 framework packages).

## Full Hub and Fleet Status

Root full Hub27152 was terminal BUILD RED on missing nested `ArtifactHash` imports; Home fixed the import. Warm retries3444/SQP1Wr and71601/SIjgri exposed respectively the native request-outcome cfg and render-grant TLS borrow now corrected. Current full Hub86949 is active on its exclusive `trusted-gis-real-activation/hub-target`; no real-Hub materialization success claimed. Genuine GIS child65957 remains a separate pending native qualification.

Writer38 is GREEN (`aKflPX/00`) and current clean compaction4 is GREEN (`CTTmHs/00`), both DB executable SHA256 `2885e0fc2d788ea28c8c9fdef86242cfa090f133049365fd8693c56f5cca62e0`. The DB agent is diagnosing durable-decision preimage parity before composed/recovery. Checkpoint publication source oracle is GREEN21; its native build was blocked by the shared render-grant compile error, so Home is continuing process/source harness work while root qualifies the repaired core.

## Remaining Audited Work

Do not call the lifecycle or frontend finished. Native and real component receipts remain required. Patch ACK/rejection must bind the issued exact lifetime, patch sequence, surface and revision; typed result ACKs and deferred render/task work need their exact retained identities reviewed. WIT's silent numeric/null projections need explicit refusal. Native open duplicate payload consistency, final ACK retry/tombstones, command-ingress fault injection and mounted render ownership require tests. The folder mirror's stale durable write still needs a real publication fence. Then implement bounded cold-pair pages, transactional UI patch application, and the actual authenticated Hub → GIS child → renderer → shared durable inference journey.
