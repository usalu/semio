# Canonical Note Scalar Core Replay

The replay instantiates the original Note actor Core module from the captured component and executes the Bun transcript with the public scalar `CoreInstance` API. The replay uses each function signature to type its scalar arguments and expected result. It verifies the clock imports in order, including the operation binding and destination pointer for every operation-clock response. It compares the memory size after every export, verifies every captured memory range, and requires both SRE1 Input receipts to match byte for byte.

The source-built interpreter uses Rust optimization level 3 and existing pinned native dependencies, with a one-billion-instruction limit per export and a one-million-instruction step size. The driver checks the SHA-256 of the complete component against the captured identity and reads only the Core range into memory. The 25-call transcript includes 3,319 operation clock imports, five monotonic clock imports, a 601-byte host write and 19 byte checks. The replay uses the transcript as its state, result and host-call oracle.

Run from the repository root with `repo tools exec bun -- <ticket>/owned-input-core-replay.ts 3000000000`. The driver accepts an optional per-export instruction limit. Pass `--replay` to reuse the already linked executable after verifying the component digest again. It builds only the current interpreter source and replay executable, with the cached native dependency closure accessed read-only. Component, transcript, local link outputs and process logs are under the ticket's generated directory.

With the initial one-billion per-export limit, the first 20 calls matched the transcript. `prepare_actor` required 650,754,221 scalar instructions, clocked no imports and took about 29 seconds after instantiation. The first Input exhausted the limit after matching 2,406 of its 3,221 captured clock imports. A second run used a three-billion limit with the same executable, component and transcript.

The three-billion run matched all 25 export results, memory sizes, 3,324 ordered clock imports and 19 exact byte ranges, including both SRE1 receipts and the unchanged wire around both Input calls.

| Export | Calls | Scalar instructions | Clock imports |
| --- | ---: | ---: | ---: |
| `prepare_actor` | 1 | 650,754,221 | 0 |
| `channel_version` | 1 | 785 | 0 |
| `admit_actor_context` | 8 | 2,297,589 | 63 |
| `alloc` | 1 | 601 | 0 |
| `admit_actor_input` | 2 | 1,348,232,590 | 3,261 |
| `dealloc` | 12 | 8,479 | 0 |

The first Input executed 1,347,018,726 scalar instructions and 3,221 clock imports in 84.957 seconds. The second executed 1,213,864 scalar instructions and 40 clock imports in 56.139 milliseconds. The same two receipts report 24,533 guest work units in the Bun run, giving roughly 54,956 scalar instructions per guest work unit across Input under the captured loan. The eight-minute Wasmtime attempt reached its Cranelift compilation wall limit before instantiation, leaving Wasmtime fuel unmeasured. The scalar result measures the interpreter's `CoreInstance::step` accounting under the Bun clock and loan. Paid host dispatch, memory traffic, availability and the live issuer use separate accounting.

Pass `--sample` alongside the three-billion limit to compile a separate testkit executable. It observes the current function and instruction byte offset once per 32,749 scalar instructions during the first Input only and reports the ten most frequent sites and functions. The sample represents counted scalar instructions. The stripped Core module preserves no function names, so the report identifies samples by Core function index and instruction-stream offset.

The sampled build reproduced every call's scalar fuel, clock count, memory size, and byte checks. It observed 41,131 sites across 16,944 distinct function and instruction offset pairs. The hottest function accounted for 7.02% of samples. The ten hottest functions accounted for 32.75%, and the hottest instruction site accounted for 0.27%.

| Function | Samples | Function at sampled site | Byte offset | Samples at site |
| ---: | ---: | ---: | ---: | ---: |
| 58406 | 2,889 | 56229 | 18 | 110 |
| 56229 | 1,917 | 58825 | 7 | 104 |
| 136155 | 1,760 | 56229 | 28 | 103 |
| 57361 | 1,574 | 56229 | 11 | 101 |
| 121031 | 1,460 | 56229 | 4 | 100 |
| 58281 | 885 | 58825 | 0 | 100 |
| 59049 | 832 | 56229 | 0 | 95 |
| 58825 | 812 | 58825 | 2 | 95 |
| 136158 | 735 | 56229 | 8 | 94 |
| 10679 | 606 | 56229 | 29 | 94 |

[Bun oracle](owned-input-jit-2026-11-21.md) · [Wasmtime comparison](owned-input-wasmtime-2026-11-21.md) · [Scalar replay source](owned-input-core-replay.rs) · [Replay driver](owned-input-core-replay.ts)

## Canonical WFC Cross-Check

The second owner uses the same replay harness with `--wfc`, the independently qualified component and the recorded 602-byte WFC wire. The scalar engine matched all 25 export results, memory sizes, all 19 byte ranges and both SRE1 receipts. It measured 1,795,912,917 instructions for `prepare_actor`, matching the independent descriptor-emission receipt.

| Input call | Instructions | Imports | Scalar elapsed |
| --- | ---: | ---: | ---: |
| First | 1,347,003,403 | 2,499 | 74.257 s |
| Second | 1,213,124 | 35 | 76.852 ms |

The first Input's instruction count differs from Note by 15,323, around 0.00114% of the observed work. The two owners share the host-side Input and guest-side receiving machinery. The membership of the largest instruction interval is under investigation.

## Note Input From the Live Grant

The synthetic SDK session's exact wire candidate matches the gateway's measured 681-byte length and FNV-1a fingerprint. Its request carried the host's effective semantic limit after separately reserved cleanup, transit and infrastructure allotments. The independent Bun issuer retained its own Boot receipts, reused the live Note operation identity and verified all 19 words of the reconstructed first Input grant before executing it. Bun returned the unchanged 681-byte wire after two receipts. The first Input took 275.739 ms and spent 21,590 guest work units. The second took 1.644 ms and spent 3,265 units.

The native scalar engine independently matched 25 export results, 19 exact memory ranges and both SRE1 receipts in that Bun transcript. The first Input required 1,352,007,195 instructions and 3,158 clock imports in 68.129 seconds. The second required 1,212,401 instructions and 29 clock imports in 45.191 milliseconds. Relative to Note's neutral fixture, the live-grant reconstruction changes first-Input instruction count by 4,988,469, about 0.37%.

- [Reconstruction source and scope](guest-input-work-audit-2026-11-21.md)

## Empty Poll Control

The same exact Note component accepted the checked-in empty 284-byte Poll and returned the original wire after two SRE1 receipts. The first Input spent 12,750 guest work units. The scalar replay verified both receipts and every output byte, and measured 525,525,103 instructions in the first Input and 1,212,506 in the second. Omitting the event and its fields reduced first-Input instructions by 821,493,623 relative to the 601-byte Note fixture, about 61.0%. Field count, field shapes and policy literals differ between the fixtures. This comparison demonstrates input-dependent receiving cost and leaves the specific per-field attribution to the source profile.

The checked-in 480-byte Note InstanceOpen with no capability grant used 1,100,969,135 first-Input instructions and 1,212,816 second-Input instructions. All 25 calls, 19 exact byte ranges and both receipts matched. The 480-byte and 601-byte wires also use different numeric policy limits. The measured 246,049,591-instruction gap includes that difference and the additional capability record.

## Live Input Cost Between Clock Imports

The exact reconstructed live Note replay with the trace-enabled native interpreter consumed 3,236,403 instructions through clock import 64, 127,454,278 through import 512, 378,525,954 through import 1,024 and 828,827,245 through import 2,048. The largest single interval, ending at import 2,342, contained 2,206,883 instructions, or 0.163% of first Input. Only 153,552 instructions followed the last of its 3,158 clock imports. The replay matched every return, memory extent, clock invocation and captured byte again. Its cost accumulates across the receive path.
