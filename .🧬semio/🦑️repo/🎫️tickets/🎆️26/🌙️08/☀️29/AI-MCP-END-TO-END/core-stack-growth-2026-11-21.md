# Paid Core Operand and Frame Stack Growth

The former scalar interpreter replaced the operand or frame vector with capacity `len + 1` whenever it encountered a new peak. Each replacement moved the complete live prefix and released the old capacity. The targeted native regression failed before the change at 8,976 bytes of operand relocation for peak depth 34. That is 561 live `Value` entries relocated on this host. The same exact-growth sequence applies to the former frame stack path.

Both paths now double a full stack when credit permits. The spare extent is bounded by the host vector limit, the remaining allocation credit after frame children, and the available release credit. A tight grant can still fund the minimum required capacity. A replacement pays for the exact new allocation and initialized prefix moved, then releases the full old allocation. Frame growth remains part of frame admission. Operand growth is its own paid step and can succeed before an unfunded instruction retries with the same operand values and program counter. The change does not alter instruction fuel, other opcode quotes, or global budgets.

The schema-validated fixture reaches depth 34 on both stacks and returns 42 in the repository interpreter, Wasmtime, and Bun's WebAssembly implementation. Native assertions compare each heap allocation and release with its receipt, bound total relocations below twice peak storage, deny one-byte-short copy, birth, and release grants, and exercise constrained growth, a zero-capacity instruction retry, cancellation at peak, and complete retirement. The tested constrained growth schedule is 1, 2, 3, 6, 12, 24, 48 slots. It moves 48 entries per stack through the peak, as compared with 561 exact-growth relocations.

Five existing native laws used the total source grant as the scalar execution grant for turns that deliberately loaned smaller physical credit. They failed validation before planning instructions. Their test callers now expose the remaining live turn credit in the execution input. The native job-export law also read a removed top-level `parameters` field and used one two-`i32` signature for every job. It now builds each author-described export in Wasmtime and compares its type with the actual selector. Five signatures take 20 `i64` words and `CloseJobs` takes 19. All six return one `i64`.

All 21 Rust interpreter unit tests passed in the pinned native relink, including the Wasmtime oracles. Six focused Bun source laws and strict TypeScript checking of the SDK-independent stack test passed. Wider Bun source coverage exposed separate preexisting contract mismatches. The channel descriptor expects a 19-word `Checkpoint` signature where Core declares no parameters. The job source descriptor also expects 20 words for actor failure publication where the actual Core and guest runtime use 21. Canonical Cargo ingress was unavailable under the local 16 GiB storage floor. The narrow relink used the existing cached target with the repository's pinned toolchain. Its targeted pre-change failure was captured before the shared output path was reused, so no full pre-change native comparison remains.

The source ABI mismatches recorded above were subsequently resolved in the test fixtures. [Owned export fixture audit](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/owned-export-fixture-audit-2026-11-21.md>)

The fixtures establish a host VM scaling improvement. They do not attribute the prior real MCP Input cost to stack growth or qualify a rebuilt gateway.

## Files

- [🏗️instance/▶️invocation/🦀️.rs](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/🦀️.rs>)
- [🏗️instance/▶️invocation/🧬️schema/🔣️.json](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/🧬️schema/🔣️.json>)
- [🏗️instance/▶️invocation/🧪️tests/🧫️fixtures/🔣️.json](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/🧪️tests/🧫️fixtures/🔣️.json>)
- [🏗️instance/▶️invocation/🧪️tests/🦀️.rs](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/🧪️tests/🦀️.rs>)
- [🏗️instance/▶️invocation/🧪️tests/🟦️.ts](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/🧪️tests/🟦️.ts>)
- [🏗️instance/▶️invocation/🧭️control/🧪️tests/🦀️.rs](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/🧭️control/🧪️tests/🦀️.rs>)
- [🏗️instance/🧪️tests/🦀️.rs](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/🧪️tests/🦀️.rs>)
- [🧪️tests/🔬️unit/🦀️.rs](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🧪️tests/🔬️unit/🦀️.rs>)
- [📦️packages/🦀️rust/📋️project.json](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/📦️packages/🦀️rust/📋️project.json>)
- [📦️packages/🦀️rust/📜️script.ts](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/📦️packages/🦀️rust/📜️script.ts>)
- [Pinned test relink](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/core-stack-growth-native.py>)
