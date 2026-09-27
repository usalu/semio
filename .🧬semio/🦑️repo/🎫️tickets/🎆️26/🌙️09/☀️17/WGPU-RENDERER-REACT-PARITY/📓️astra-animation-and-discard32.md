# Animation Clock and Discarded-Frame Rearming

Updated 2026-09-27, Europe/Berlin.

## Scope

This packet covers GPU animation clock precision and the independent retained UI / Shell clock owners. It does not establish full renderer parity or a successful current WASM build.

## Findings and Changes

- GPU uniforms previously converted Unix epoch seconds to f32. At current epoch magnitude, adjacent animation frames collapse onto the same float. Shader kinds 6, 7 and 9 use periodic phases of 1.6 or 3.2 seconds.
- The renderer now samples the monotonic job clock, takes the common 3,200,000-microsecond remainder before conversion, and supplies a precise finite f32 phase. An unavailable clock yields the initial phase.
- A neutral schema and fixture cover unavailable time, frame increments, both shader periods, wrap boundaries, one-year uptime, and large clocks. Native assertions also cover u64::MAX.
- Actual Chromium Web Animations API progress independently matches all 20 fixture phase/period combinations. The native test is written but has not yet run.
- Retained UI clock advancement mutates accepted input state before the candidate frame completes. Its UI-only deadline publication remains before ACK, so a discarded candidate does not leave an expired consumed deadline in the host.
- Tutorial advancement also mutates live Shell state before sealing the candidate. Immediately after tutorial_tick, the renderer now publishes the complete Shell deadline minimum. This preserves both tutorial and chrome work across a later candidate discard.
- UI and Shell retain separate mailbox atomics. A partial update cannot erase the other owner. ACK refreshes both and the host installs their minimum.
- The native discard law now explicitly consumes a due UI deadline, advances that owner, discards the frame, and verifies that the next earlier Shell deadline survives without an immediate redraw. It then advances Shell and verifies the remaining UI deadline.

## Validation

- GPU browser oracle: **2 passed**, one file, Vitest 4.1.10, 7.59 seconds. Log: `🗑️generated/astra-runtime/clock30/animation-oracle-1.log`.
- Native clock/discard assertions: pending. The already-running NativeShell3 compilation may predate these additions; no pass claim.
- WASM11: failed. The first renderer Cargo pass completed in 14m28; the subsequent Trunk Cargo pass failed with seven UI errors while the caret packet was being edited. The wrapper omitted diagnostics. A coherent source checkpoint and a fresh build are required before runtime conclusions.
- Shader animation wake scheduling is being audited separately. Correct phase precision alone does not prove an idle renderer wakes to animate.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🖌️render/⏱️schedule/🧫️fixtures/🎞️animation/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/⏱️schedule/🧬️schema/🎞️animation/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⏱️gpu-animation-clock/🟦️.ts`
- The renderer's WGPU deadlines, frame-job tests, native deadline tests, Draw/Tutorial phases, and TypeScript test include list.

## Accepted Animation Scheduling

The prepared worker now records animation demand while its existing bounded Draw/Overlay measurement visits each actual UiInstance. It checks the shared loading, waiting and introducing constants; finished/static primitives do not request animation. This also covers direct mutations of the public draw buckets and overlay-routed primitives without trusting helper-side metadata or adding a second traversal.

The immutable packet carries that receipt to presentation acknowledgement. Only a completed accepted presentation updates the host's AcceptedAnimationClock. A separate keyed ANIMATION deadline preserves an already-future wake across unrelated redraws, rearms after a consumed due wake or delayed turn, and clears on an accepted static packet. A discarded candidate cannot change the last accepted demand. The obsolete OsHost CaretBlink stub, its unconditional one-shot wake, and four tests of that unused policy were removed; actual focused-caret ownership is being supplied by the UI packet.

The neutral fixture now includes primitive classification and an accepted/discarded presentation sequence. Native tests cover 12 packet cases across normal/overlay buckets and packet overlay lists, plus host deadline replacement and preservation of the other control clock.

Updated oracle result: **3 passed**, one file, **6.17 seconds**. The added Chromium law observes actual Web Animations ownership across present/discard transitions. Log: `🗑️generated/astra-runtime/clock30/animation-oracle-2.log`.

UI3 failed before assertions in three tooltip test compilation errors (two fixture paths and one missing constant import). Sol corrected all three. Full UI4 is now running, including the new prepared-animation law. NativeShell3 remains pending; it does not select the latest animation/tutorial laws. WASM12 is running. No source change here has yet received fresh runtime acceptance.

## Build 12 / Native 3 Integration Corrections

WASM12 failed before artifact publication in 8m59. Two text-selection indices were incorrectly constructed with DslValue::from(u64); they now use the existing exact-integer DslValue::uint constructor. Three caret consumers called a crate-private document lookup; Sol is repairing them against the intended public document identity interface. The animation getter was present in source but absent from the UI dependency compiled earlier in that build, so a subsequent coherent build is required.

NativeShell3 failed before assertions in 21m41 with the same production errors and new test-integration errors. Root corrected the frame-job test's crate-qualified discard call and removed the obsolete caret member from its host-retirement fixture. Sol owns the tooltip/tutorial include corrections and new focused-scene test identities. A direct Nx-scoped cargo check with short diagnostics is running to expose any remaining precise spans.

All renderer global caret state, its per-frame phase, and the all-surfaces NodeGraph blink broadcast are removed. Current constructors, browser startup, and native fixtures use the exact accepted caret API being integrated by Sol. This is source integration; no replacement-caret runtime success is claimed yet.
