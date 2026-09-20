# Pointer Modifier Snapshot Packet

Terra identified a remaining input gap after keyboard focus-loss repair: a modifier pressed outside the renderer root is absent from a subsequent canvas pointer gesture because normalized pointer events currently borrow the renderer’s last keyboard state. React receives modifier flags directly on each DOM pointer event.

The clean implementation boundary is to carry required `EventModifiers` on normalized PointerMove, PointerDown, PointerUp and Scroll events. The browser DOM projection must serialize each event’s shift/control/alt/meta snapshot; Rust wire decoding must preserve it; native winit producers must supply their current modifier state; retained latest-pointer/scroll samples must preserve the snapshot through replacement and frame reconstruction. Dispatch updates the aggregate input state from that event before routing. Do not synthesize fake key presses on pointer entry or add optional legacy fields.

Known production paths are engine browser-boot, browser-frame-transport, input-wire Rust, winit-app, renderer retained input reconstruction, plus UI render dispatch and its host/coalescing producer modules. The DispatchEvent variants live in `🧰️framework/🔨️modules/🖱️ui/🖌️render/🖱️dispatch/🦀️.rs`; there are fourteen source/test files with pointer constructors in the renderer/UI search scope, so a full native compile must verify every producer.

Extend the existing language-neutral browser input-wire fixture and schema before implementation. Cover an outside-root held Shift press, Control/Meta selection, modifier release between two coalesced pointer moves, and wheel modifier preservation. The DOM oracle must create actual browser/JSDOM events independently of the wire object. The Rust law must inspect the normalized event and the actual renderer dispatch state, including a deliberately stale positive keyboard modifier that the next pointer snapshot clears.

This packet is planned, not implemented or accepted. Root is preserving the current native/wasm compilation cohort before assigning source mutation.

## Sol implementation — 2026-09-20

The normalized pointer contract now carries a required `EventModifiers` snapshot on `PointerMove`, `PointerDown`, `PointerUp`, and `Scroll`. The contract has no defaulted or optional pointer modifier fields.

The event-time snapshot is preserved across every owned boundary:

- The browser `PointerEvent` and `WheelEvent` producers copy `shiftKey`, `ctrlKey`, `altKey`, and `metaKey` into the UI-isolate input vocabulary.
- The TypeScript frame wire requires the four booleans on pointer and wheel events. Pointer replacement keeps the latest event whole; wheel accumulation sums deltas while replacing position and modifiers with the newest sample.
- Rust `BrowserWireEvent` requires the same four fields and projects them into `DispatchEvent`.
- Both native winit hosts attach their current `ModifiersState` snapshot to pointer, touch, and wheel events.
- `ui_host::PointerMoveSample` and `ScrollSample` preserve modifiers through bounded coalescing. `RuntimeEventCursor` restores them when it reconstructs normalized events.
- `dispatch_normalized_event` uses the event snapshot for pointer move/down/up and writes it into both `AppInteractionState.modifiers` and retained `InputState.modifiers`; scroll does the same before queuing its application. A pointer snapshot with all flags false therefore clears stale positive keyboard state.
- The lower browser-host binary event contract also appends one modifier byte to pointer and wheel payloads and decodes it without synthesizing key events.

The shared language-neutral fixture now covers held Shift on a move and press, Shift release in a later same-identity move, Ctrl+Meta on a selection press, Alt on a middle press, and Ctrl+Alt on a wheel. The Rust twin renders the live normalized event back into that fixture grammar and explicitly refuses a pointer wire object missing modifiers.

### Test-first receipts

The fixture change produced the intended red receipt before production changes: the focused WGPU input-wire suite ran 22 tests with 10 failures, all showing the missing required pointer/wheel modifier fields or lost transport bytes.

After implementation, the focused TypeScript run passed 63/63 across:

- `🧪️tests/🎮️wgpu-browser-input-wire/🟦️.ts`
- `🧪️tests/📨️browser-frame-transport/🟦️.ts`

The browser oracle uses real jsdom `MouseEvent` and `WheelEvent` objects, independently reads their native modifier flags, and verifies the resulting wire values. It also proves a later coalesced pointer move clears the earlier held-Shift snapshot. Rust laws cover the fixture projection, required-field refusal, bounded pointer/wheel coalescing, retained sample reconstruction, native modifier mapping, and actual renderer dispatch state for move/down/up/wheel. Per root ownership, the Rust laws were authored but no Cargo/native/wasm run was launched from this packet; the next root native cohort must supply that receipt.

### Ownership and limits

No keyboard focus-loss/reset lifecycle changed. No fake key transitions, compatibility defaults, retry behavior, scheduling ceilings, or queue capacities were introduced. Runtime behavior is source-complete but is not claimed until root's native build and browser cohort consume this source.
