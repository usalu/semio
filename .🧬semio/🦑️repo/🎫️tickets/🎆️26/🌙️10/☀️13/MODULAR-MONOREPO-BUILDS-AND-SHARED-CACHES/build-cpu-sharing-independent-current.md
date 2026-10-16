# Build CPU Sharing Independent Audit

Source-only review, 2026-10-15. No tests, processes, images, runtime probes or implementation edits.

## Concrete Unpaid Inherited-Capacity Path

`framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts:57-59` trusts environment `SEMIO_COMPILER_CAPACITY_HELD="true"` plus a valid integer `SEMIO_COMPILER_PARALLELISM`, and returns a grant with no-op release. It acquires neither the actual host gate nor actual core leases, and checks no live enclosing grant identity. An arbitrary independent caller can therefore claim inherited capacity while another process already owns all host capacity, and can reuse that claim after any purported parent exits.

This reaches a supported repository route: `library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts:123` skips capacity acquisition and its broker/jobserver construction when the inherited marker is true. `library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts:44` preserves these incoming environment fields while overriding toolchain fields. The launcher still retains cache/compiler writer custody, but those storage locks are not proof of paid CPU capacity; other compiler paths and processes can consume CPU concurrently.

Require an authentic live inherited grant across supported process boundaries, or have independent callers acquire actual capacity. A freely authored boolean/count is not custody. The nested-child design must avoid reacquiring a parent's already-paid cores while proving that parent remains live through child settlement.

## Actual Paid Path and Limits

Without the inherited marker, independent all-host requests use actual exclusive host gate custody. Requested counts use a shared gate, serialized picker and exclusive per-core leases (`capacity/🟦️.ts:61-80`). Two unmarked processes cannot each independently acquire all cores at once. Partial acquisition failures/cancellation release collected cores and gate; supported launcher line 134 releases its grant in finally after awaited execution. Queue polling observes cancellation/deadline, progress and continuation. These facts qualify the inspected admission protocol, not every runtime's actual thread count or all host resources.

## Small Actual Regression Plan

Extend `capacity/🧪️tests/🟦️.ts`, whose existing tests already execute real competing claims, Python SQLite lock observations and crashed owner recovery. Existing command is `bun repo run @semio-tech/framework-process:test-compiler-capacity` (`.vscode/launch.json:12510`), with the existing process script `test compiler-capacity` route at line 11166.

Hold one actual all-host exclusive grant in a bounded test directory. In a separately owned child, invoke the production capacity API with only the inherited boolean/count and requested count one. Use a ready/gate protocol, not busy CPU load, to demonstrate whether it returns while the exclusive owner is live. Independently query the actual host/core SQLite lease files to prove the parent still owns exclusive capacity and the child acquired no core. Expected fixed behavior is refusal without an authentic parent token; a genuine parent-issued inherited child must succeed without a second reservation, drain before parent release, and refuse reuse after parent retirement. Include cancellation of a waiting independent claim and escaped inherited reuse. Schema/Ajv validates closed declarations; SQLite supplies an independent reservation oracle.

No regression was run here. The existing tests exercise honest marker forwarding from paid grants, but do not establish authenticity of a caller-supplied marker.
