# Why the Repository Uses a VHD

The responsible feature is **bounded physical repository cache storage**, implemented by the process framework's Native build storage backend and consumed by repository cache budget enforcement. On Windows its fixed-size disk-image implementation is `fixed.vhd`; Linux uses `fixed.img`. The feature is a filesystem capacity boundary for build/dependency/tooling storage, not a virtual machine, security sandbox, or a prerequisite of caching itself.

## Source Evidence

Paths below are relative to C:/git/semio.

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts:9` names cache partitions: rust, tools, graph, dependencies, products, diagnostics, nx and leases. Line 11 gives each partition a physical capacity and filesystem size; line 23 derives filesystem bytes from the partition capacity minus reserved overhead. This identifies the feature above the platform mechanism.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts:63` validates the VHD footer; line 65 requires fixed VHD type 2, exact physical file size of virtual capacity plus 512-byte footer, and the declared maximum. Line 212 selects Linux fixed.img; line 226 selects Windows fixed.vhd and performs allocate/activate/format through the platform owner.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts:47` explicitly states that an ordinary directory has no portable aggregate physical quota. Line 48 intentionally refuses ordinary-directory Native operations. Lines 85-88 require an enforcing backing before native writes and intersect its maximum with the declared policy capacity.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts:308` verifies current backing physical allocation; line 316 verifies the mapped filesystem fits the declared filesystem capacity. These are runtime checks of actual backing, not merely counters in JSON.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts:2` exports the separate journal state/projection implementation. Line 29 imports separate resource-lease primitives. Line 99 applies journal events and accounting. Their algorithms are distinct from mounting a VHD, although current orchestration intentionally requires enforcing backing before using them for Native writes.

## What Removing It Means

Simply deleting the VHD or skipping its mount breaks today's storage contract: dependency setup and routed build/cache operations refuse their missing enforcing backend or missing receipt/mapping. An image may contain live cache data, so deletion also discards that data; it is not just removing an unused metadata file.

Replacing the image backend with ordinary host directories is a feasible architectural redesign, but it removes the hard aggregate filesystem capacity boundary. Logical reservations and periodic directory-size accounting cannot by themselves stop subprocesses, package installers or compiler outputs from consuming more physical disk than declared. Cache speed, task graph selection, reusable artifacts, locks, event journals, cancellation and cross-process coordination can all exist without VHDs; they do not inherently require disk images. The current requirement tying them to Native bounded storage would need deliberate changes, rather than bypassing the checks.

An OS quota or another enforcing storage backend could preserve the capacity invariant without VHDs, if implemented and validated per platform. Such replacement is not currently provided by the ordinary-directory fallback, which expressly refuses enforcement.

Current host limitation: the Windows mount privilege is unavailable, so full mounted production has not been qualified here. No tests were needed or run for this read-only explanation audit.
