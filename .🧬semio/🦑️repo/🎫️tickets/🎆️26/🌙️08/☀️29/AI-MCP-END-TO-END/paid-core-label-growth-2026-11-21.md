# Paid Core Structured Label Growth

A canonical Note call had posted 1,220,726,165 copied control bytes, 574,114,480 allocated control bytes, and 454,255,988 released control bytes before its first Input receipt. The structured instruction path replaced every full label vector with capacity one past its depth. A new label at each depth repeatedly copied the existing prefix and released its allocation.

The structured path now uses the operand and frame stacks’ `StackGrowth` planning and replacement. Type-vector allocations count against the live allocation credit before sizing the label vector. One quote pays for the label and type writes, the initialized prefix moved, the actual new capacity, and the previous capacity released. An unfunded quote leaves the condition operand, the labels, and the program counter in place.

The shared fixture nests 33 typed control instructions inside a function, including an `if` whose condition remains on the operand stack until its label is admitted. Bun, Wasmtime, and the paid interpreter return 21. The source test gives the `if` one-byte-short copy, allocation, and release grants, then runs it with the three-slot growth capped by allocation or release credit. It compares charged capacity and release against the thread’s actual allocator receipts, exercises cancellation at peak depth, and retires the invocation to zero live bytes in both runs.

| Core build | Label slots at peak depth 34 | Label bytes moved |
| --- | ---: | ---: |
| Cached baseline, `13a6a7f356e2df0e` | 34 | 44,880 |
| Pinned native source relink | 48 | 3,840 |

The move volume fell 91.4% in this fixture. The baseline fails the linear relocation assertion. The current source passes the public Source law and the schema-checked Bun suite. The source closure measured 259,189 bytes in 22 files on macOS, Linux, and Windows before the adjacent transfer changes. The prior interpreter testkit relink could not find its cached actor/job testkit identity. The public Source proof used the pinned native interpreter, independent Wasmtime, and existing workspace allocator. Gateway behavior and ordinary retirement credit for large label vectors require separate end-to-end verification.

- [Interpreter control implementation](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/🧭️control/🦀️.rs>)
- [Shared fixture](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/🧭️control/🧪️tests/🧫️fixtures/🔣️.json>)
- [Internal state regression](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/🧭️control/🧪️tests/🦀️.rs>)
- [Public Source and Wasmtime law](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/mcp-core-label-growth-law.rs>)
- [Public Source law runner](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/mcp-core-memory-law.ts>)
