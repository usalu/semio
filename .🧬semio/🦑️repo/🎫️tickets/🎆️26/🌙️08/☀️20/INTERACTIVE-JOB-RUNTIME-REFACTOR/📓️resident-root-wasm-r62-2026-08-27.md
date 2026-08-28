# Resident Root — Wasm Compilation R62

Canonical command: `bun x nx run @semio-tech/ui-contract-rs:check-wasm --skip-nx-cache`, using the existing master Cargo target. Exit 0.

The checked target's existing script runs wasm32-wasip2, wasm32-unknown-unknown, then wasm32-wasip2 with typegen. Actual completion footers:

```text
Finished `dev` profile [unoptimized] target(s) in 1.26s
Finished `dev` profile [unoptimized] target(s) in 1.82s
Finished `dev` profile [unoptimized] target(s) in 1.00s
NX Successfully ran target check-wasm for project @semio-tech/ui-contract-rs
```

Raw output: `🧪️member-ui-resident-root-wasm-r62-2026-08-27.txt`. This compiles the shared resident permit, 64-slot canonical root binding, deferred affine return, and typed final-root release for both Wasm targets. It does not instantiate or run a consumed browser/component package.
