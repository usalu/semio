# ⏱️ Measurements: Process Plugin Edit → Browser Loop

Host: mac, 10 cores, 32 GiB, load 27-46 during the whole session, 9 of 12 fleet workers refusing (no headroom),
swap 12-17 GiB. All builds go through `repo cargo --store=shared --configuration=prototype`.

## Where the time goes today (monolithic component)

| step | observed | source |
|---|---|---|
| `repo cargo` pre-phases before cargo starts (source admission of 948 crates, generated sources, graph) | 685 s on the first call, about 10 min on repeats | `🗑️generated/build-warm1.log`, `timings-attempt1.txt` |
| local admission | 1 of 10 cores, rustc memory budget 0.0 GiB, no fleet offload (`[repo cargo] units: 332 compiled here, 0 offloaded`) | same |
| shared unit cache for these wasm32-wasip2 prototype units | 0 hits, 209 misses, every miss `differs in env:CARGO_ENCODED_RUSTFLAGS` | same |
| cold build of the Process component closure | 8,255 s (2 h 17 min) for 766 of 948 crates, then `semio-framework-actor` failed on a peer's half-landed edit (`close_unique_plain` missing) | same |
| Process ticket's own loop (hub build plus browser scenarios) | "about 60-90 min including the queue"; "each attempt also rebuilds 400-800 crates"; thin-LTO final link 9 GB, cancelled 7 times by the warden | ticket 26/07/23 GET-PROCESS-WORKING-END-TO-END `status-2026-11-26-afternoon.md` |

## Why the final step is whole-program work

The prototype profile (`--config profile.prototype.lto="thin"`, `strip="symbols"`, `codegen-units=16`, `opt-level="s"`)
makes cargo compile every dependency of the cdylib bitcode-only: the members of
`.cache/b/r/m/u/l/p164a12/wasm32-wasip2/prototype/build/semio-framework-pack/*/out/libpack-*.rlib` start with `BC C0 DE`
(LLVM bitcode), not `\0asm`. So every plugin edit ends in thin-LTO code generation of the whole framework closure
(about 750k functions, 380-610 MB components, 3-9 GB link peak; see ticket 26/11/17 `🎈️component-bloat.md`).
A split boundary therefore must not consume LTO-only rlibs: the split lane builds the component root as an `rlib`, so
cargo compiles the closure with object code once, and later plugin edits only recompile the plugin crates.

## Size attribution

`size-attribution.py <component-or-core.wasm> <out-prefix>` charges code bytes to the instantiating crate from the name
section. Run on the split lane's framework image and plugin module once they exist; results below.

(filled in by the split runs)

## Before/after

(filled in by the split runs)
