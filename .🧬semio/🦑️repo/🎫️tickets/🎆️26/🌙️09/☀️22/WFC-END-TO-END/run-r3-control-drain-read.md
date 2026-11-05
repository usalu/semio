# Native R3 Control Drain and GC Review

Read-only review requested by Root. No source, process, instrumentation, policy or queue was changed.

## Accepted-operation settlement

The actual implementation is `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`.

- Lines 212-220: `track(owner, run)` registers one promise in that owner and each actual Scope parent. The completion handlers remove that exact promise from those sets.
- Lines 223-224: `settle` first awaits its callback, then `drain` closes admission and awaits every accepted pending promise before setting the scope nonlive. A callback failure is retained locally until drain completes, so absence of a displayed error does not alone prove that every callback is still executing.
- Lines 355-358: a narrowed control has its own pending set and a parent Scope/control. A zero-millisecond request adds no deadline; it retains the original time/cancellation lineage.
- Line 369: `withRequestedIssuedResourceControlV1` tracks the whole child settlement on its parent, while the child drains accepted work admitted on that child. The wrapper promise is not inserted into its own child pending set.
- Line 373: only operations explicitly admitted through the issued-control port are tracked there. OS process ancestry alone is not this promise custody.

This is an intentional join of accepted asynchronous work, not a general wait for every descendant OS process.

## Observed unit-cache sidecars are detached

The concrete launch is `.../⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts:117-129`.

`startUnitCacheCollectorV1` uses raw Node spawn with detached=true, stdin ignored, stdout/stderr directed to the cache gc.log, and windowsHide=true. It calls child.unref and returns a synchronous started/failed value. The environment is newly whitelisted (lines 113,119-120), so it does not inherit the original Script process invocation envelope or an issued resource control. No exit promise or withIssuedResourceOperation is created here.

Therefore the observed long-lived GC sidecar PIDs are NOT, merely because their parent is 54688, pending accepted operations of the configuration zero-time scope. Their presence cannot explain that scope draining.

The pressure timer at lines 155-157 calls void recheck().catch(...), is unrefed, and clears when its outer body settles. The launcher establishes that timer around the whole Cargo lane at `🏃️process/🦀️cargo/🚪️launcher/🟦️.ts:131-133`, before the configuration reader exists. Its relieve closure passes no resource control. A separate collect-now branch can await in-process GC (lines 142-143), but that is different from waiting for the detached sidecar and cannot be inferred from a sidecar PID.

## What configuration reader 50 establishes

The configuration function at `.../⚙️configuration/🟦️.ts:55` holds its shared reader over initial rechecks, the whole requested-control callback, final rechecks and retirement. That requested callback includes dependency preparation and the launcher's inputs-ready callback.

On normal successful return, its outer accepted-operation drain therefore cannot occur before the inputs-ready callback: that callback is inside the body that must finish first. However an early callback failure could be retained while another accepted operation drains, and a nested recheck scope could itself be draining. The shared-reader lease and normal process facts do not expose private pending sets or exact await positions. They cannot distinguish those generic cases from awaited I/O.

Conclusion: the specific hypothesis that the observed detached GC sidecars keep configuration-reader accepted-operation settlement open is disproved by the launch/control boundary. A different pending accepted operation is not proved or ruled out by current normal facts. No runtime cause or hang is asserted. Original handle 4662 and all frozen source are preserved.
