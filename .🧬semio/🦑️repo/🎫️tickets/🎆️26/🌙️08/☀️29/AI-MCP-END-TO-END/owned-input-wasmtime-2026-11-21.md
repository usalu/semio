# Canonical Note Owned Input Fuel Replay

The fuel probe replays the canonical Note actor Core module extracted from component SHA-256 `a745c114e0f7e685ad6af51c1c658b6bd6cca590f5249c641d5b78642163c2e3`. It uses the same 601-byte wire and loan sequence as the Bun oracle. The Bun oracle previously observed 24,533 guest-reported work units across two Input receipts. The strict host had reached hundreds of millions of paid VM instructions while its first Input was still open. The ratio between guest-reported work and Wasm instructions cannot be inferred from either observation alone. [Bun oracle results](owned-input-jit-2026-11-21.md)

The opt-in Bun capture records the selected Core module offset and length, each owned export and signed scalar argument, every clock import with its original value, the host wire write, and the exact channel, Boot, Input and SRE1 receipt bytes. The recorder also checks and captures the unchanged wire at the original pointer before and after each Input call. The transcript uses line records `M` for module identity, `C` for a call followed by its clock imports `I`, `W` for a host write and `R` for bytes the guest must match.

The Wasmtime probe instantiates that same Core slice and replays the transcript in order. It requires Wasmtime to consume every clock event with the same import identity and arguments, then compares each scalar result, linear-memory size and byte range. It reports elapsed time and fuel separately for each export, counts each imported function by call and overall, and includes measurements up to the first divergent or trapped call. A complete run requires two Input calls, a single wire write and 19 exact byte checks. At the boundary of each call, a fresh Wasmtime fuel budget defaults to one billion. The JIT compiles serially and the Store limits linear memory to 256 MiB. The driver verifies the entire component digest before launching the replay.

Run each stage from the repository root with `repo tools exec bun -- <ticket>/owned-input-wasmtime.ts --record`, then `--check` for a metadata-only Rust check, `--link` to link against the cached Cranelift-enabled Wasmtime runtime, and `--replay` to measure. Replay accepts an optional per-export fuel cap. Each stage uses the same ticket-local generated directory, and the link uses the pinned Rust compiler and the existing artifact closure read-only. The transcript avoids copying the Core or component and remains under an 8 MiB replay bound.

The Bun capture, Rust metadata check and Wasmtime link passed. The 170,382-byte transcript contains 25 exports, 3,324 clock imports, 19 byte checks and one wire write. Bun observed 3,220 operation-clock reads in the first Input and 39 in the second, with one monotonic-clock read in each. The selected Core is 99,256,445 bytes, including a code section of 89,729,261 bytes. Both serial compilation attempts reached their wall bounds of two and eight minutes before instantiation. During the second attempt the compiler was still using CPU at 6 minutes 43 seconds. The Wasmtime probe produced no fuel sample, so a comparison against the paid host remains open. Both runs used cached Cranelift-enabled Wasmtime variant `1bf474c8efe59f9c`.

The first and second Input loan words follow the wire pointer and its length.

```text
1 1 9  1 86400000000 999992736 47994653 2997887235 2145478208 2147483648 64 65536 16777216 67108864 67108864 64 1000000 86400000000 0
1 1 10 1 86400000000 999971415 47982310 2994207872 2144014920 2146021417 64 65536 16777216 67108864 67108864 64 1000000 86400000000 0
```

Compare the first successful `admit_actor_input` fuel sample to the paid host's VM instructions spent within that one guest call. Exact host clock responses and live paid-host scalar words would make a second comparison possible. The current capture freezes the diagnostic issuer and its Bun clock. Wasmtime fuel and the paid interpreter's physical fuel have separate accounting rules, including treatment of control and bulk operations. A numerical difference is evidence about the combined paths and metering schemes unless per-instruction calibration or a matching live issuer transcript isolates it.

[Bun capture](owned-input-jit.ts) · [Wasmtime driver](owned-input-wasmtime.ts) · [Replay implementation](owned-input-wasmtime.rs)
