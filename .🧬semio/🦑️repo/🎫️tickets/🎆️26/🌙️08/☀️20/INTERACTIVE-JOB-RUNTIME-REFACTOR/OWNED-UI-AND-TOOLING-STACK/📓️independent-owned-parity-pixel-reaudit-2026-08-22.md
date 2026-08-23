# Independent Owned Parity Pixel Re-Audit

Date: 2026-08-22

## Verdict

**ACCEPT** — the repaired private comparator closes the earlier writable typed-array overlap corruption. No blocking defect was found in the owned pixel-comparator packet.

## Re-Audit Scope

Read the governing plan, the first independent rejection, the implementation and overlap-repair reports, and `📊️owned-parity-pixel-differential-2026-08-22.json`. Audited only the owned comparator and dependency-removal packet; no product, manifest, lockfile, or cache mutation was made.

## Overlap And Buffer-Safety Findings

`ownedParityByteRangesOverlap` requires both strict backing-store identity (`left.buffer === right.buffer`) and strict half-open byte-span intersection. It therefore rejects an exact non-empty diff/input alias and forward/backward partial overlap, while permitting end-touching/disjoint same-buffer views and zero-length spans. The guard is after shape/threshold validation and before threshold computation, traversal, or any `diffRgba` write.

The focused suite executes the exact alias, partial-forward reference overlap, partial-backward candidate overlap, disjoint same-buffer view, reference/candidate read-only alias, shared zero-length view, and retained text-edge cases. It passed 9/9. The retained text-edge result is count 2 with the centre anti-alias marker `[255, 192, 0, 255]`; the four retained differential fixtures remain opaque 1, transparent 1, text-edge 2, and scene-gradient 0 with their recorded marker counts.

An additional read-only Bun probe evaluated the exact current comparator source region and confirmed:

- `SharedArrayBuffer` partial diff/reference overlap rejects before mutation;
- disjoint `SharedArrayBuffer` views return 2 and retain the centre AA marker;
- ArrayBuffer/SAB end-touching ranges are non-overlapping and partial ranges overlap, as required by half-open boundaries;
- Bun supports resizable `ArrayBuffer`; a shrunken out-of-bounds input view is rejected by the exact-byte-length validation before output mutation;
- a detached input view is likewise rejected by exact-byte-length validation before output mutation.

The comparator has no `new`, array/object literal, collection, or callback-producing operation in either per-pixel traversal loop; its pre-created marker objects and scalar helper calls do not allocate per pixel. The AA neighborhood traversal is likewise scalar and bounded to 3-by-3.

## Source Boundary

`compareOwnedParityPixels`, its options type, and the range helper are private functions/types; no export or external API/import surface was introduced. Exact scans of the dev script, its `package.json`, and `bun.lock` found zero `pixelmatch` and zero `[DEBUG]` occurrences. A repository live-source/manifest scan excluding ticket history and the historical dependency baseline found no `pixelmatch` occurrence.

## Executed Verification

```text
bun nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache -- -t 'compareOwnedParityPixels'
PASS: 9 passed, 27 skipped, 36 total

bun nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache
PASS: 36 passed

bun install --frozen-lockfile --ignore-scripts --no-progress --no-summary
PASS

bun ./📜️script.ts verify dependencies
PASS: baseline 238, current 140, removed 98, no additions

bun ./📜️script.ts verify dependencies list js --format json
PASS: 77

bun ./📜️script.ts verify dependencies list rust --format json
PASS: 63

bun ./📜️script.ts verify dependencies parity js
PASS: undeclared imports 0, lock mismatches 0
```

The observed dependency total is exactly **77 JavaScript + 63 Rust = 140**.

## Browser Residual

No live React-versus-WGPU browser screenshot sweep was run or accepted in this audit. The implementation and repair reports correctly retain it as a P3 Worker/Wasm browser-lane residual; this report does not falsely claim browser screenshot parity.

## Blockers

None for the owned comparator/dependency packet. The browser screenshot sweep remains a separate residual, not an unperformed passing gate.
