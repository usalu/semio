# Runtime Resident and Handback APIs — Wasm Compilation R42

Command: `bun x nx run @semio-tech/ui-runtime-rs:check-wasm --skip-nx-cache`.

```text
Finished `dev` profile [unoptimized] target(s) in 11.24s
Finished `dev` profile [unoptimized] target(s) in 3.80s
NX Successfully ran target check-wasm for project @semio-tech/ui-runtime-rs
```

Exit 0. The existing target checks wasm32-wasip2 and wasm32-unknown-unknown. Raw: `🧪️member-runtime-resident-handback-wasm-r42-2026-08-27.txt`. This compiles the current shared permit and typed nonblocking handback entry APIs. It does not compile or execute the larger Plugin/Kernel/WIT producer join, nor instantiate a consumed package.
