# UI Document and Patch Wasm R48

Canonical `@semio-tech/ui-contract-rs:check-wasm` completed exit 0. The existing router checked wasm32-wasip2, wasm32-unknown-unknown, and wasm32-wasip2 with typegen in that order. All three compiler passes completed. This verifies the new exact document comparison and whole-patch owner types compile across those targets, not consumed Wasm runtime execution.

Actual completion output:

```text
15:    Checking semio-framework-ui-contract v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust)
422:    Finished `dev` profile [unoptimized] target(s) in 3.03s
423:    Checking semio-framework-ui-contract v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust)
830:    Finished `dev` profile [unoptimized] target(s) in 3.71s
831:    Checking semio-framework-ui-contract v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust)
1238:    Finished `dev` profile [unoptimized] target(s) in 2.52s
1242: NX   Successfully ran target check-wasm for project @semio-tech/ui-contract-rs
```
