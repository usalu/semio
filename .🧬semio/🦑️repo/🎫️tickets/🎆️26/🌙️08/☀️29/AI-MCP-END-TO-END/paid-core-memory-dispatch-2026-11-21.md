# Paid Core memory dispatch

The interpreter previously returned Pending as soon as it admitted a retained memory plan and again after each movement or publication. This exposes host Poll boundaries within short Wasm memory operations even when the active turn can fund them.

The production change stays in `CoreInstanceOpening::advance_scalar`. A successful admission enters the next loop iteration. An existing memory plan records its prior retained progress, advances once, and follows these paths.

- A completed instruction clears the plan, increments VM fuel once, and continues.
- An incomplete instruction with paid movement resumes the plan on the next iteration.
- An incomplete instruction without progress returns Pending with the same retained plan.

Every loop iteration checks cancellation, deadline, and physical fuel. The current `charge` checks the live Source ceiling before each admission, movement, or commit. A movement uses the currently available copy bytes and keeps its existing overlap direction. The final commit advances the guest PC and removes the original operands. The same retirement path handles a retained plan after cancellation or budget exhaustion.

The shared fixture executes three fills, including a zero-length fill, two overlapping copies, a load and memory size. Both Bun's independent WebAssembly engine and the existing Wasmtime oracle read the fixture. The original interpreter regression compares the same output, 25 VM instructions, and aggregate physical effects across a full grant, one retained item per turn, and four VM instructions per turn. It separately checks a no-copy turn against an already retained plan. The preexisting linear-growth test issues a one-item grant to retain the publication boundary it uses to test partial initialization and cancellation.

The repo Bun fixture suite passed all 11 assertions. The public native law compared the original interpreter against Wasmtime with the same fixture and checked the live Source receipts. The cached interpreter baseline failed the one-turn assertion. A pinned Rust relink of the production interpreter passed it.

| Issued turn | Baseline turns | Updated turns |
| --- | ---: | ---: |
| Full retained grant and 26 VM fuel | 15 | 1 |
| One retained item | 42 | 42 |
| Full retained grant and four VM fuel | 16 | 7 |

Each run returned 101, spent 25 VM fuel, and posted 42 retained items, 1,855 copied bytes, 112 bytes of capacity and 50 released bytes. The native verification reused the existing artifact closure read-only and ran the single-crate link with `repo tools exec bun -- <ticket>/mcp-core-memory-law.ts --test-paid-turns --source`. The full repository Rust suite was outside this focused relink.

Both public growth laws passed in the same run. They verified a zero-delta grow without allocation or release budget, repeated growth with exact allocation and retirement receipts, and cancellation while partly initialized pages remained unpublished.
