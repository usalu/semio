# Host API Build Integration

The actual scoped renderer prototype compiler stopped in `semio-framework-plugin-host` with 54 errors before reaching the renderer. Its current WIT job interface requires original job source, retained input, absolute deadline, cancellation and raw receipts; old Host methods lack these inputs. Grant-only wake and router-effect closure also lack the genuine caller StepContext required by current job owners. These are real upstream API gaps; no compatibility shim or fabricated budget/context has been added.

## Clock Integration

WasmtimeRuntime now requires its embedding clock. All observed constructors explicitly pass their root or test clock. ActorHostState retains that clock, and the store epoch callback checks both its physical pool timer and unchanged original absolute microsecond deadline. Budget callers in the main Host use `arm_original`; instantiation checks its caller deadline before VM work. The pure deadline owner is included directly by a focused interpreter-package integration target, avoiding source copies and the uncompiled Host dependency.

A closed six-case language-neutral corpus distinguishes submillisecond remaining time, exact expiry, overdue calls, missing clock and a two-millisecond wake. Strict Ajv and independent SQLite produce the same outcomes. The initial new harness failed because Ajv strictRequired rejected an incomplete conditional schema; the first native run then failed on an incorrect test-target relative path. Both harness defects were corrected without weakening expected outcomes. Prototype exact Source/native gate passed in 34.4 seconds; native output confirms exact units, unchanged deadline, exact expiry, missing-clock refusal and overflow-safe maximum deadline. Release exact Source/native gate also passed in 3 minutes 2 seconds. These checks qualify the isolated deadline logic; full Wasmtime epoch/runtime and complete renderer remain unqualified.

The snapshot owner is migrating actual Budget.deadline_us into OwnedGuestInstance resume_snapshot and Wasmtime calls, preserving full failure and pending source. Its separate prototype source/native gate passed in 68 seconds. ComponentDocumentCodec currently receives no per-operation original source/budget through detached Store APIs, so the invalid zero-identity constant cannot be made legitimate by renaming a field.

## Concurrent Work

The native executor owns persistent compiler-recipe targets, the Nx executor owns transactional graph publication after actual ENOSPC and semantic dependency-record reuse, and the web executor owns snapshot/dispatch closure integration. Read-only inspection of the other active architecture chat found its current work on repository dependency enforcement and generic service ownership; no message or mutation was sent to that chat.
