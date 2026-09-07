# Retained Production Reactor Lifecycle

## Current Qualification

The production lifecycle implementation is landed but **not yet native-qualified**. The end-to-end goal remains active. Cold canonical checkpoint pages and the real GIS renderer have not been wired through this lifetime yet.

The registered `@semio-tech/framework-plugin:guest-lifecycle-check` passed its five neutral admission cases with an independent AJV predicate (44529, 71880). The initial source gate47316 was intentionally RED because the real kernel reducer was component-only and inaccessible to native tests. These source checks are not native runtime evidence.

Native exact group53937 / `reactor-lifecycle-native/exact-cargo-laws-rPBwm0/00` reached a toolchain linker failure: macOS `ld` returned EINTR opening already-built Rust archives while linking dispatch macros. The capture reader also got EINTR and left its wrapper alive. After confirming Cargo was absent, root terminated only its identified Bun wrapper40228; the Nx session then ended exit1. No native lifecycle law ran. Retries93401 and58441 ended BUILD RED on test-only type/import errors, now repaired. Build1220 was GREEN but selected zero laws because the requested factory scope was wrong. Runs64687,30740 and32815 reached native assertions: the decoder and then the existing synthetic bundle lacked canonical package/owner declarations. These fixtures now use the first-party DSL decoder and package `semio:test` owning `s.test.synthetic`. Native10 retry87787 is active; no full lifecycle GREEN claimed.

## Production Changes

- Extracted the actual reducer into `⚛️reactor/🔄️turn/🦀️.rs`; the WIT bridge calls this same reducer. Kernel request-outcome conversion is now available natively.
- Replaced the numeric lifetime and copied in-flight-close globals with a fixed typed registry in `PluginRuntime<PA>`, mounting `GuestLifecycleCell<NativeLifetimeOwner<PA>>`.
- The owner retains the exact native close lease, reserved/active/released participant stages, and the captured request. Foreign close validation and all capacity checks precede native close admission. The duplicate `plugin_destroy_app` call is removed.
- Open preflights actor, instance, quarantine and lifecycle rows before the real factory. Actor failure cannot leave a created live app without its lifecycle. Any occupied modulo slot rejects before advancing the guest serial.
- Captured and Accepted receipts are retained and require exact ACKs. Retired requires native lease terminal emptiness plus exact reactor/patch/pending close completion. Terminal resource release is one participant per bounded step and remains tracked through retry.
- Both native and WIT paths now prepare output from a borrowed turn result, consume staged lifecycle ACKs after the real clock verdict, then publish infallibly. An ordinary conversion or late-clock error returns the exact typed patch to its pending slot and leaves the lifecycle ACK/receipt mounted. Effects/presence retry retention is still unqualified.
- Command ingress now retains the exact NativeCloseKey and participates in reactor close completion. The close path incrementally retires command pages/cursors without dispatching a closing app.
- UI intent admission/dispatch and task resumption check Live authority. Typed-operation continuation skips mounted non-Live cells; non-Live render work does not drain presence.
- Native compilation exposed a borrowed mounted-render grant escaping TLS. The grant now retains the same Rc-owned patch tracker state through awaited rendering; no second tracker is created.
- A sparse active-close cursor advances real occupied close work rather than wasting 1023 empty turns between cleanup units.

## Native Laws and Launch Ownership

The real runtime laws are included inside the existing private plugin-builder contract module. They use its real synthetic fluent plugin bundle, not a new PluginApp mock. They cover actual Captured/ACK/open actor state, early close refusal, exact single native close generation, Retired terminal proof, bounded final ACK release, foreign lifetimes, modulo collisions, actor setup rejection, and stale ACK after reopen. Existing lifecycle clock/late-release laws run in the same group.

The permanent command lives in the plugin Rust package's `📜️script.ts`, with a corresponding `📋️project.json` target and source/native launch seed411.011/.012. Home regenerated the normal registry and confirmed freshness (59 plugins,60 playgrounds,45 framework packages).

## Full Hub and Fleet Status

Root full Hub27152 was terminal BUILD RED on missing nested `ArtifactHash` imports; Home fixed the import. Warm retries3444/SQP1Wr and71601/SIjgri exposed respectively the native request-outcome cfg and render-grant TLS borrow now corrected. Current full Hub86949 is active on its exclusive `trusted-gis-real-activation/hub-target`; no real-Hub materialization success claimed. Genuine GIS child65957 ended BUILD SIGKILL in `fresh-process-HwPaH5`; no real child, component, or browser law ran. The signal's underlying cause is unproven.

Writer38 is GREEN (`aKflPX/00`) and current clean compaction4 is GREEN (`CTTmHs/00`), both DB executable SHA256 `2885e0fc2d788ea28c8c9fdef86242cfa090f133049365fd8693c56f5cca62e0`. The DB agent qualified durable decision/recovery15 (`zMvcm3/00`) and journal5 (`ySopjz/00`). The earlier journal10k-turn shutdown miss remains unresolved despite the successful rerun. GIS assembly kernel4 is GREEN; real GIS factory build remains active. Checkpoint publication source/process-harness gate is GREEN22; native process is unqualified. Home is now implementing the durable stale-folder publication fence while holding duplicate full-Stdio compilation.

## Remaining Audited Work

Do not call the lifecycle or frontend finished. Native and real component receipts remain required. Patch ACK/rejection now stages and commits a per-slot exact lifetime/sequence/surface/revision tuple; neutral10 source cases are GREEN74113, native laws pending87787. Read-only audit found no demonstrated in-process typed-result ACK ABA, but found a concrete numeric-only task-resume reuse gap. Deferred render keys and final Released-ACK retries remain open. WIT's silent numeric/null projections need explicit refusal. Native open duplicate payload consistency, final ACK retry/tombstones, command-ingress fault injection and mounted render ownership require tests. The folder mirror's stale durable write still needs a real publication fence. Then implement bounded cold-pair pages, transactional UI patch application, and the actual authenticated Hub → GIS child → renderer → shared durable inference journey.
