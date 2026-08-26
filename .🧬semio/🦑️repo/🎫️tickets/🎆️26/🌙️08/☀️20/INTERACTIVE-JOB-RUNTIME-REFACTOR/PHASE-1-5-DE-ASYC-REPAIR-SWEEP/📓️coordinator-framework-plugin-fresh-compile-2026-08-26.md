# Framework Plugin Fresh Compile

## Result

On 2026-08-26 the coordinator ran a fresh, isolated native library compile after the scheduler
payload-ledger and de-async changes:

`CARGO_INCREMENTAL=0 CARGO_TARGET_DIR='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYC-REPAIR-SWEEP/🧪️target-root-framework-plugin-current' cargo check -p semio-framework-plugin --lib`

The command exited 0 after 2 minutes 26 seconds with 0 compiler errors and 168 warnings. This is
evidence for the native framework-plugin library only. The strict zero-warning, all-target, Wasm,
Nx, and end-to-end gates remain open and must not be inferred from this result.

## Scope

- package: `semio-framework-plugin`
- target: native library
- isolated Cargo target directory: yes
- incremental compilation: disabled
- result: compile green, strict-warning gate red by count
