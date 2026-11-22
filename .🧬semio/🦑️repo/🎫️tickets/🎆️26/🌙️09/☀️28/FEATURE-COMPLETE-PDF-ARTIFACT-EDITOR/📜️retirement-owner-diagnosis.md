# Retirement Owner Diagnosis

The Wasm location log maps the panic to controlled retirement Drop at value/♻️retirement/🎮️controlled/🦀️.rs:389, reached from exchange_generic at reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs:161–173. Drop requires terminal emptiness: no original value, pending cursor, or cursor pages (:148, :326, :387–390).

Generic owns both exchange_close and output_close (reactor/🔄️turn/🔌️extension-invocation-and-command-ingress/🦀️.rs:75). The cancellation path retire_command_ingress steps output and output_close before exchange (:140), and can yield retaining output_close. Normal exchange_generic closes exchange (:170) and the decoded owner (:171), then replaces the entire Generic state (:173), without examining output_close; the failure terminal branch also replaces state (:164). If cancellation has partially populated output_close, this replacement drops a live ControlledRetirement. Exact instruction mapping is needed to identify the concrete dropped member in the recorded trap. All state removal routes must retain the Generic owner until each owned retirement has completed and its close slot is None; pending and refusal must preserve custody.

FailedDecoded handling in runtime/🧰️ops/📥️ingress/🦀️.rs:214–232 preserves the decoded owner on pending or refusal, and returns terminal failure only after owner.terminal_is_empty(). close_exchange_owner (extension-invocation-and-command-ingress/🦀️.rs:83–84) reports success only when both original and closing slots are None. A focused regression should interrupt output retirement with a bounded grant, resume the generic execution or failure path, and verify no state replacement occurs before output_close is empty; also exercise close after the same interruption.

## Compiled Guest Failure Path

The second mounted run retained the worker stack. The signed core is stripped. The core decoder matched the WebAssembly program counters to exact function bodies and located Rust location records in immutable module data. The direct call in function 159437 is in the failed generic command exchange. It leads through the primary error enum switch, the boxed domain drop, the domain wrapper and the controlled retirement Drop assertion. The old command path copied the primary message to a new shell fault and discarded the still-live typed failure.

The live failed-decoded branch now uses the existing ingress retirement driver. Pending and refused work keeps the admitted state on the original lane. The terminal path sends the complete typed failure through the existing original turn return owner. That driver also settles any exchange and output closing owners before moving the command. A fresh signed component and mounted Open are required to verify the correction and surface any primary import failure.

- [Captured guest stack](🗑️generated/mounted-wasm-limits-retirement-stack/pdf-mounted-actor-traffic.json)
- [Core function and source record decoder](📜️pdf-wasm-trap-locations.py)
- [Compiled function locations](🗑️generated/pdf-wasm-trap-locations.log)

## Primary WASM32 Refusal

The warmed Chromium run captured the original domain type as `OriginalCommandPackFailure` before the old component dropped it. Natural Open decodes its `MediaArtifactDescriptor` using the original typed Pack receiver. The default request is 4,294,967,296 allocation bytes. The previous WASM32 conversion saturated this to 4,294,967,295. The retained cursor refuses physical byte ceilings above the target's signed allocation maximum. It refused this descriptor before decoding any PDF content. Both original command Pack receivers now clamp to the signed target maximum, 2,147,483,647 on WASM32. The default remains 4,294,967,296 on 64-bit native targets.

The focused WASM32 law failed with the old conversion at 2,147,483,648 requested bytes. It passed ten limits through both real allocation expressions, the materializer clamp, and the physical cursor refusal condition after the correction. SQLite independently calculated admitted byte ceilings. The fresh signed component build is queued behind host memory pressure.

- [WASM32 law and SQLite comparison](📜️pdf-pack-wasm32-limit.ts)
- [Mounted old-component console](🗑️generated/mounted-wasm-limits-fault-warmed/pdf-mounted-console.json)
