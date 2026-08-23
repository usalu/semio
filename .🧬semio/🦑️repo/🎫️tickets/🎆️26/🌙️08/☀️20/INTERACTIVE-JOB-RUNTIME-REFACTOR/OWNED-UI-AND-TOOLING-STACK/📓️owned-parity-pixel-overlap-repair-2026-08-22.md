# Owned Parity Pixel Overlap Repair

Date: 2026-08-22

## Verdict

The typed-array-view corruption identified by `📓️independent-owned-parity-pixel-audit-2026-08-22.md` is repaired. `compareOwnedParityPixels` now fails closed before its first output write whenever `diffRgba` overlaps either read input. Existing mismatch counts, marker policy, allocation behavior, and the exact 140-dependency boundary are retained.

This repair changed only the owned `PixelCompare` implementation/test regions and the two ticket reports. It did not change dependencies, manifests, lock contents, Dagre, Diagram, P3/P8 sources, shared Worker sources, or Cargo metadata.

## Root Cause

Distinct `Uint8Array` views can share one backing `ArrayBufferLike`. The comparator previously wrote each diff pixel while future pixels were still read from the two inputs. If the diff view started inside either input view, a completed output pixel overwrote a later source pixel. The audit's forward-overlap text-edge probe therefore changed the expected count from 2 to 8 and mutated the reference bytes.

## Repair Contract

`ownedParityByteRangesOverlap` uses the three properties that define a typed-array byte span:

- `buffer` establishes shared backing storage, including `ArrayBuffer` and `SharedArrayBuffer` through `ArrayBufferLike`;
- `byteOffset` establishes the half-open span start;
- `byteLength` establishes the half-open span end.

Two views overlap only when they share the exact same backing buffer and both strict half-open intersection inequalities hold. Consequently:

- exact non-empty diff/input alias is rejected;
- partial forward and backward diff/input overlap is rejected;
- disjoint views on the same buffer are accepted;
- reference/candidate alias or overlap is accepted because both are read-only;
- zero-length spans are accepted as non-overlapping.

The guard runs after shape and threshold validation but before threshold computation, traversal, or any diff mutation. It performs constant work and allocates nothing, preserving the no-per-pixel-allocation contract.

## Permanent Regression Coverage

The new in-source test covers all requested authority boundaries:

1. exact `diffRgba === referenceRgba` rejection and unchanged reference bytes;
2. forward partial overlap reproducing the audit geometry and unchanged shared storage;
3. backward partial overlap against the candidate and unchanged shared storage;
4. disjoint reference/diff views on one backing buffer, accepted with count 2 and the retained center AA marker;
5. exact reference/candidate read-only alias, accepted with count 0;
6. one shared zero-length view used for both inputs and output, accepted with count 0;
7. an independent post-rejection text-edge comparison retaining count 2 and the center AA marker.

The existing four representative fixture expectations remain opaque 1, transparent 1, text-edge 2, and scene-gradient 0.

## Verification

```text
bun nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache -- -t 'compareOwnedParityPixels'
PASS: 9 passed, 27 skipped, 36 total

bun nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache
PASS: 1 file, 36 tests

bunx prettier --check --range-start=199547 --range-end=211975 <dev-script>
bunx prettier --check --range-start=292170 --range-end=301694 <dev-script>
bunx prettier --check <dev-manifest>
PASS: all owned ranges/files formatted

bunx eslint -f json <dev-script> | jq <owned-region-filter>
PASS: owned PixelCompare regions: 0 findings

bunx tsc --noEmit --strict --types node,vitest <dev-script> | <owned-region-filter>
PASS: owned PixelCompare regions: 0 findings

bun install --lockfile-only --ignore-scripts --no-progress --no-summary --frozen-lockfile
PASS: Saved lockfile

bun ./📜️script.ts verify dependencies
PASS: baseline 238, current 140, removed 98, no additions

bun ./📜️script.ts verify dependencies list js --format json | jq 'length'
PASS: 77

bun ./📜️script.ts verify dependencies list rust --format json | jq 'length'
PASS: 63

bun ./📜️script.ts verify dependencies parity js
PASS: undeclared imports 0, lock mismatches 0
```

The dev package still exposes no Nx typecheck, lint, or format targets, so the same precise owned-range checks are reported rather than a false whole-file pass. The task explicitly prohibited every Git command; therefore no Git diff command was run. Proportionate change-integrity coverage instead consists of focused Prettier checks on both complete owned regions, the full dev suite, owned lint/type filters, the frozen lock gate, dependency parity, and exact source scans.

Exact scans found no external `pixelmatch` reference and no `[DEBUG]` log in the dev script, dev manifest, or lockfile.

## Safe Boundary

The local overlap blocker is closed and the packet is ready for a second fresh Terra audit. No next dependency packet should mutate the workspace before that audit accepts this repaired 140-dependency boundary. The live React-versus-WGPU screenshot sweep remains the separately documented P3 browser-lane residual.
