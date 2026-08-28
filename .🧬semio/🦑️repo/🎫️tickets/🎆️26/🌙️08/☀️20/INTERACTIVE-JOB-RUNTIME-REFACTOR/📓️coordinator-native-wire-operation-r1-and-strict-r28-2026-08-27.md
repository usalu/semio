# Independent Native Wire Intake R1 and Strict R28

## Actual Commands and Results

Independent R2, same focused command, now passes6tests/597skipped/603discovered in9.06seconds,exit0,start18:19:12. Both actual `node:vm` Uint8Array and BigUint64Array public-entry cases execute, and the one-slot wire patch stream is included. The source review finding below is repaired at this checkpoint; mounted transport and whole-result copying remain open.

Both commands used `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false` and the canonical Bun/Nx targets.

`bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedWireOperation'` returned exit0. Four tests passed,597skipped,601discovered,15.07seconds, start18:14:47. The tests compose all eleven native tags with paired surface publication/ACK, hostile/cancel paths, runtime mint rejection and native typed-buffer ownership. This is not a mounted React/WGPU transport or fresh guest test.

```text
> nx run @semio-tech/framework-renderer-react:test-long --args=--run -t OwnedWireOperation
> bun ./📜️script.ts test long --run -t OwnedWireOperation
 Test Files  1 passed | 4 skipped (5)
      Tests  4 passed | 597 skipped (601)
   Start at  18:14:47
   Duration  15.07s (transform 20.85s, setup 0ms, import 26.67s, tests 10.03s, environment 6.23s)
 NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react
```

`bun x nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache` returned exit1 with exactly the seven tutorial joins; no new native-wire diagnostic appeared.

```text
../../../../../../../../../../♻️mit-bestand/🧺️demonstrator/🟦️brand.ts(151,5): error TS2741: Property 'interactionSelection' is missing in type '{ focusedWindowId: string; activeUtilityByWindowId: {}; activePanelTabByGroup: {}; expandedTreeIds: never[]; commandPanelOpen: false; }' but required in type 'TutorialUiSnapshot'.
🧪️index.test.ts(6922,43): error TS2345: Argument of type '{ activeUtilityByWindowId: {}; activePanelTabByGroup: {}; expandedTreeIds: never[]; commandPanelOpen: false; }' is not assignable to parameter of type 'TutorialUiSnapshot'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2102,5): error TS2353: Object literal may only specify known properties, and 'selectionJson' does not exist in type 'TutorialUiSnapshot'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2102,39): error TS2339: Property 'selectionJson' does not exist on type 'PluginViewState'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2153,41): error TS2339: Property 'selectionJson' does not exist on type 'TutorialUiSnapshot'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2153,76): error TS2339: Property 'selectionJson' does not exist on type 'PluginViewState'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2197,141): error TS2339: Property 'selectionJson' does not exist on type '{ readonly kind: "selection"; readonly domainId: string; readonly granularity: string; readonly ids: readonly string[]; }'.
```

## Source Review and Remaining Work

The coordinator read the public per-operation cursor, private intrinsic buffer transport helper and native child-field cursor. Buffer transfer binds the real whole non-shared ArrayBuffer using captured intrinsic getters rather than user-visible buffer/offset/length properties. Native children are the actual BigUint64Array representation, capped at128IDs, with checked safe-number conversion and one scalar per advance. The retained generated type evidence is older than the current SurfaceRef schema and is not full current ABI parity.

A prospective public-boundary gap was routed to the UI owner: `OwnedUiWireOperationCursor` used realm-local `instanceof` before the shared intrinsic admission. A helper-only cross-realm test did not prove public constructor acceptance. The executor's direct public-entry RED reproduced the Uint8Array rejection before its BigUint64Array branch; the repaired source routes unknown values into the private intrinsic authority. Coordinator R2 now independently executes both positive cases.

The coordinator also read `OwnedUiWirePatchCursor`: one pending native operation, exact sequential ordinal, separate retained page receipt, and a paired-surface ACK only after complete input/publication. Cancellation retains the active input/operation/candidate; late committed cancellation advances notification obligations and waits for ACK transfer. A parent transport owner must retain any transferred page/patch receipt across failed sends; taking a receipt is not by itself native acknowledgement.

The next live aggregate is assigned to the UI owner: exact captured activation object plus native-issued guestLifetime, bounded surface-cell scan, one pending lookup/stream, frozen read facade, and explicit host-side close participation. Demonstrator owns PluginRuntime create/destroy/close and will consume that API. Same-activation numeric-ID reuse must keep pending results bound to the original aggregate, never resolve a replacement by name/ID.

Whole WIT lifting/structured cloning, streaming patch ownership, mounted React/WGPU consumers, all15prepared host schemas and14nested JSON fields remain open. Zero-grant and cancellation tests do not certify physical allocation/GC timing. Tutorial transport remains independently owned; no default snapshot/compatibility mapping was inserted to make strict pass.

## Explicit Identity-Domain Boundary

A further native source read confirms `UiNodeId(pub u64)` and `UiRevision(pub u64)` in the contract's `🦀️document.rs`; their allocator/successor currently stop at u64 overflow, not the JS exact53-bit limit. The comment that the smaller limit is unreachable in practice is not a schema proof. Current retained wire rejection above MAX_SAFE_INTEGER prevents rounding but does not implement the full native u64 identity domain. This mismatch is recorded with the UI owner for a consistent schema/implementation decision after the aggregate cutover; no native limit or ID behavior was silently changed.

The coordinator also re-read the six still-RAII-only watchdog consumers in WGPU winit_app and Plugin host shard. Their current code remains unresolved: event/metrics mutate before an exact verdict, metrics work extends outside the inner guard, redraw commits/presents before checking a result, and the two shard guards span awaited calls then admit output without consuming a verdict. Actual synchronous execution slices need measurement; suspended I/O time cannot be disguised as blocking time. No new callback/timing pass is claimed from the UI wire tests.

No cleanup or output publication was performed. All prior reports remain retained; the master ticket stays open.
