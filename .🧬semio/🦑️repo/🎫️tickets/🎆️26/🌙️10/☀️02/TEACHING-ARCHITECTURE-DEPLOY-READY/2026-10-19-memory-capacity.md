# Constrained Compiler Memory Capacity

Root's actual private builder observations report physical total8,321,515,520 bytes while `process.constrainedMemory()` reports7,516,192,768 bytes (7GiB). Both production admission broker callers previously supplied totalmem-based budgets, overstating usable total by805,322,752 bytes. No OOM was observed or claimed. The independent initial audit is `2026-10-19-private-build-audit.md`.

## Schema-First Boundary Repair

The existing admission schema now declares closed `MemoryCapacityV1` physical/constraint input DATA and `MemoryShareV1` effective capacity/CPU fraction DATA. Runtime `admissionMemoryBytesV1` uses actual `totalmem()` and `process.constrainedMemory()` by default, returns the minimum for a positive constraint and physical total for zero (unknown/unbounded), and refuses nonpositive/invalid physical or negative/noninteger/nonfinite constraint values. Positive constraints above physical total never manufacture capacity. Numbers are bounded to safe integer bytes.

The existing admission owner is shared by both broker callers. Component-pass uses the probe when its explicit `physicalBytes` observation is absent; existing explicit test inputs remain deterministic. Cargo applies the existing admitted CPU fraction **after** effective memory selection, with the unchanged6GiB reserve, then clamps reconstructed capacity to effective total. This prevents negative headroom below reserve from manufacturing RAM at fractional CPU allocations. Neither the reserve, CPU discovery/grants, broker progress/freeze law nor transient free-memory policy was changed. Telemetry continues recording actual physical `memoryTotalBytes`; it is not relabeled as cgroup capacity.

Exactly two production `startAdmissionBrokerV1` call sites were found across framework Source: native component-pass and repository Cargo launcher. The remaining call is the explicit test harness. Both production broker input expressions were extracted from their actual current TypeScript AST and evaluated with neutral probes, proving default effective bounds, CPU sharing, below-reserve behavior, and unchanged explicit component observations. No Original/Native token or compiler invocation is constructed by these tests.

## Actual Verification

Test-first red: the new neutral test failed because the shared boundary API was absent; `🗑️generated/2026-10-19-memory-red.log` records0pass/1fail. The fixture covers zero unknown, very large unbounded-like constraint, actual smaller private limit, equal/larger limits, a one-byte constraint, actual CPU shares, round-down and below-reserve cases. Ajv validates the language-neutral inputs and SQLite independently computes min/conditional clamp and share results; invalid numeric inputs refuse.

After implementation, focused tests passed **2tests/71assertions** (`2026-10-19-memory-integration.log`). Related pure admission corpus passed **8tests/125assertions** (`2026-10-19-memory-related.log`), including unchanged wire schema, unit identities, d3-array quantiles, prediction, ordering, progress/freeze/EASY vectors and output footprints. Counts overlap; they are execution evidence, not summed unique tests. Tests ran directly with host Bun1.3.14 under the authorized pure-source exception. No raw compiler/installer, managed preparation or private builder operation ran here.

Actual host default probe console output (`2026-10-19-memory-host-probe.log`) observed physical34,359,738,368, process constraint34,359,738,368 and effective34,359,738,368. This validates default binding on native macOS. It does not replace the future genuine private Original check on Linux/Bun1.4.2. For the actual guest DATA, full-CPU memory budget before other charges changes from1,879,064,576 to1,073,741,824 bytes while the6GiB reserve stays exact.

## Handoff

All six paths below already exist in captured Source; no new module/fixture or capture extension is needed. First three are the minimal functional overlay; schema and neutral regressions should travel together before the next Original when possible. Root owns live dependency lock and must wait for its terminal state before any Source overlay. This worker has no private edits, live commands or remaining Source edits. No Native/proctor/image/course gate readiness is claimed.

| Path | SHA-256 |
| --- | --- |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts` | `d326f21ab2b9e0743068a6a741b640e5cf4aa43d2386d15e6885066759c60d82` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts` | `0661a10b079019667ad51842c9289e67a54c8d27618f9bd7f29e6655189eaa5b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts` | `59fa502071de6039d2a9d10349028f16dfffe7066dc751eecfe7d24b3d4cd161` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧬️schema/🔣️.json` | `602bb870e3767ebc1ebe74b40727db9d6bb098babc86dccc581a1ffe623a2c00` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🟦️.ts` | `6b1589b7c03b91bb5dadef3e1df8a9be3040ec1693951b72b2a9f1422a58c695` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🧫️fixtures/🔣️.json` | `b28bb985743e9fb6a7918865663643cf66f295954e7a842514a67af727198e0a` |

Local API contract check: installed `node_modules/@types/node/process.d.ts:1577` explicitly documents OS-imposed `constrainedMemory()` capacity and zero when unconstrained/unknown. Its separately documented `availableMemory()` is transient free memory and is intentionally not used for broker total capacity. This validates the zero sentinel interpretation without inferring it from one host sample.
