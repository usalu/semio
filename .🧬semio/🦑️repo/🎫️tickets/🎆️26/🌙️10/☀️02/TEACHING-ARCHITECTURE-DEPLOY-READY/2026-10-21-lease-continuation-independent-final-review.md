# Lease Continuation and Causal Correction Independent Review

Bounded current static clearance for handoff26077/SHA-25674ff43cf61b2694fdd0c996192f4eb4e93cdec5d0670d09d96701400a3b4a62f. All eight changed physical byte/hash pins matched; no new paths are declared or required by these corrections. No actionable defect identified in the requested slice.

The actual waitForLeaseContinuation retains the existing deadline/abort race, records failure with a separate boolean, and awaits the exact accepted pending callback in finally before unwinding. Late undefined/null/false/zero/empty/Error cannot replace a selected stop. The acquisition finally closes its DB only after waitProgress rejects; admission release occurs outside that awaited acquisition. A callback that never drains therefore retains custody rather than allowing cleanup to race it. This grants no new deadline or authority.

The new actual-function regression imports genuine issuer/control owners, extracts remaining/waitForLeaseContinuation/waitProgress/acquireUngatedResourceLease from authenticated physical Source, reaches a controlled busy transaction, holds onWait, and first observes genuine1000ms child expiry or actual abort. It requires unsettled acquisition and zero DB closes while the callback is held, then exact selected failure identity and callback-drain before DB-close. Fourteen closed rows span both stops and seven late outcomes; strict Ajv validates all. SQL independently checks event order. The fake busy DB establishes caller sequencing, not physical SQLite/kernel-lock proof.

Prior inference blockers are corrected: actual fixture writer control is third argument; default writeSync and causal FD2/offset/count/partial-write/failure ports match owner interface; AST confirms extracted nodes delimiter is exactly one U+000A. The current closed inference fixture validates. Current storage headroom and window extraction origins use physicalSourceOriginV1; renamed partial title asserts deferral after one consumed image window, preserving exact existing atomic title.

Phase coupling: five exact wait titles at120000ms each, phase660000; two exact inference titles at60000ms each, phase180000. Existing storage partial title must be replaced in selected metadata with `a partial retirement defers after one consumed image window`. All six changed TS files have zero static parse diagnostics.

## Exact Reviewed Pins

- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts`: 75284 / `03f0c550dff79a518cd0ed8225f0f6da1bd54a89fd848494763a9ff532600673`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json`: 55489 / `d63832c7174619078c65178f66ece55f67b25bab7199baa1cc81d02610a760ee`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/⏳️wait/🧪️tests/🔣️.json`: 16145 / `bac21d9238072669a26c75b967df8185c3dcee8e6dff403ef556cb1203602d37`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/⏳️wait/🧪️tests/🟦️.ts`: 19969 / `cec29d17ca3894741a122735757115f3f80b8a43788b4cd45a95bbc82fb9c3de`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🟦️.ts`: 20993 / `9ba5f1ae3fb51b674d0551962d036a208700cc2ee093cc36ecd6a4b383a31683`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📥️inference/🟦️.ts`: 11761 / `02b356358ea28d73ee376616dc9c39e94699305d978555384269770914f8e9bf`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/💧️headroom/🟦️.ts`: 8159 / `a9e3e355bea50a8844243c613f49c81a9b9fcf1e7616c541ae1d1978800be6b7`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🪟️retirement-window/🟦️.ts`: 4861 / `749f5ff8ce9c9fce07c1af94728d417aea062a12842bba2cf37e826a866e04a1`.

No Source/test/runtime was evaluated and no Native operation was invoked. Whole-cohort seals and actual runtime outcome remain pending.