# Paged TypeScript Flood Workspace

## Actual Red and Green

The unmodified public1024-square close probe genuinely failed via repo tools exec --light bun -- test with an explicit ./ file operand:0 pass,1 fail,20 assertions. It held6,291,456 bytes; close demanded4,194,304 bytes for one queue backing under the real1,048,576-byte release grant. Eight funded attempts released0 bytes and left the cursor open.

Schema-first paged/non-square/prefix regressions also failed before production changes:0 pass,4 fail,617 assertions. The replacement passed the entire existing ten-case TypeScript suite plus6 new cases:16 pass,0 fail,13,834 assertions. This is actual TypeScript execution, not Rust validation.

The strengthened1024 case requires the first initialized pixel, proving all directories and pages were held before cancellation. It holds and retires6,297,600 bytes using771 progressing close turns (768 leaves plus3 directories). The first release quote is16,384 bytes, with the unchanged1MiB grant.

## Storage and Bounds

Source: framework pixels/editing/flood/🟦️.ts. The existing accepted bounds remain exactly width/height<=16,384 and total pixels<=16,777,216. No extent is truncated or newly refused.

Each leaf holds up to4,096 pixels: seen/mask leaves<=4,096 bytes and queue leaves<=16,384 bytes. Three fixed-length directories are allocated in distinct funded actions before leaf allocations. Each leaf is then allocated in one funded action. There is no unbounded push/reallocation of a root directory. At the maximum accepted extent, a TypeScript logical directory has4,096 reference slots at8 bytes each,32,768 bytes. Every leaf and directory is independently retirable below1MiB.

All receipts include directory capacity/release, and all allocation/close actions quote metadata copy as well as backing ownership. Grants are checked before effects. Denied grants preserve the next quote and produce zero work/receipt. Allocation failures retain every earlier funded backing for close.

The accounting field explicitly identifies these JSON receipt totals as typescript-logical. Native Vec<Vec<T>> directory metadata differs: e.g.4,096*24=98,304 bytes per directory on64-bit, based on native sizeof(Vec<T>), not8-byte TypeScript logical references. Shared semantic coverage/page-count and quote/receipt/retirement laws must match; physical/logical header totals must not be falsely equated.

## Published Mask API and Rust Handoff

result() returns a completed FloodMask view with length, at(index), and iteration. No terminal N-byte flatten/copy occurs. Each scalar/iterator access checks the cursor is complete and neither canceled nor closing. A held view/iterator revokes after cancellation or beginClose; it exposes no raw page backing.

Rust should mirror the shared4,096-pixel page indexing and fixed bounded outer-directory design using its actual native sizes and paid retained closure. A borrowed native mask view can provide len/get/iter; its lifetime should prevent mutable closure while borrowed. TypeScript uses dynamic revocation. Rust source remains frozen pending the separately owned Native95837 diagnostic; paged TypeScript green is not Rust green.

## Regression Coverage

The neutral paged JSON/schema covers a67x63 non-square comb crossing page boundaries:2 pages,2,175 reached pixels independently reproduced by Graphlib. The tests cover every directory/leaf allocation prefix, every leaf allocator failure, resource-denied advance/close, exact retained/released ownership, maximum accepted directory bounds, pixel-output identity, and held-view cancellation/closure revocation.

Changed owning inputs/tests: flood/🧪️tests/{paged.json,paged.schema.json,paged.test.ts,large-close.json,large-close.schema.json,large-close.test.ts,🟦️.ts}. Existing tests now read scalar/iterable mask values and include paid directory ownership; their pixel semantics are unchanged.

No Rust, Lowpoly fill, host, or browser result is claimed here. Native95837 remains separately monitored in pixels-native-execution.md.

Final settled check: public tsc strict/noEmit for the owning production TypeScript module passed exit0. The complete16-test Bun corpus was repeated after explicit typescript-logical accounting and allocator-test constructor correction:16 pass,0 fail,13,834 assertions, exit0. No additional production changes followed these checks.
