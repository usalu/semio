# Owned Parity Pixel Comparator

Date: 2026-08-22

## Outcome

The private `pixelmatch` call in the dev screenshot parity path is replaced by the owned, non-exported `compareOwnedParityPixels` implementation in the existing `PixelCompare` region. The production import, direct dev-manifest dependency, and sole lockfile resolution are removed. Dagre, P3/P8 sources, Diagram sources, shared Worker sources, and Cargo metadata were not touched by this packet.

The accepted dependency boundary is now exactly **140 = 77 JavaScript + 63 Rust**. The dependency freeze reports 238 baseline dependencies, 140 current dependencies, 98 removals, and no additions.

## Implementation

`compareOwnedParityPixels` provides the behavior required by the existing React-versus-WGPU diagnostic path:

- exact RGBA byte-length, dimension, multiplication-range, and threshold validation;
- zero-allocation rejection before the first write when the diff view's half-open byte range overlaps either read input on the same `ArrayBufferLike`;
- deterministic row-major traversal with no per-pixel object or array allocation;
- alpha compositing over white before perceptual RGB comparison;
- the existing `0.1` screenshot threshold intent;
- bounded local antialias classification using the surrounding 3-by-3 neighborhood;
- mismatch, antialias, and muted-context diff pixels under an owned diagnostic palette;
- an identity fast path that still emits the complete muted diff image.

The caller remains private to `compareParityRegion`; no generic pixelmatch compatibility surface or production export was introduced.

## Permanent Tests

Nine in-source tests cover:

1. identical pixels and exact muted diff output;
2. the exact below/above-threshold boundary;
3. alpha compositing and invisible RGB changes;
4. shared-edge antialias classification, including the opt-out branch;
5. a material high-contrast edge shift;
6. exact alias, forward/backward partial overlap, disjoint same-buffer views, read-only input alias, empty views, no-mutation rejection, and retained text-edge stability;
7. malformed lengths, dimensions, overflow, and threshold values;
8. a 640-by-360 identity fixture without per-pixel allocation regressions;
9. fixed representative opaque, transparent, text-edge, and scene-gradient fixture results.

## Overlap Repair

The first independent audit rejected the packet because a writable diff view could overlap an unread portion of either input through the same backing buffer. Earlier row-major diff writes could then mutate later input pixels and change both the mismatch count and source bytes.

The repaired comparator compares backing-buffer identity and the exact half-open `[byteOffset, byteOffset + byteLength)` spans before any diff write. It rejects any non-empty intersection between the diff and either read input. It deliberately permits reference/candidate overlap because they are read-only, permits disjoint views backed by the same buffer, and treats empty spans as non-overlapping. The check allocates nothing and leaves all non-overlapping comparison and marker behavior unchanged.

## Differential Evidence

Temporary differential fixtures imported `pixelmatch@7.2.0` only while establishing the owned contract. They were removed before the dependency and production scans. The retained evidence is in `📊️owned-parity-pixel-differential-2026-08-22.json`.

| Fixture        | Legacy mismatch count | Owned mismatch count | Owned mismatch markers | Owned AA markers |
| -------------- | --------------------: | -------------------: | ---------------------: | ---------------: |
| opaque         |                     1 |                    1 |                      1 |                0 |
| transparent    |                     1 |                    1 |                      1 |                0 |
| text-edge      |                     2 |                    2 |                      2 |                1 |
| scene-gradient |                     0 |                    0 |                      0 |                0 |

This corrects the initial experimental comparator, which did not preserve three representative legacy mismatch counts. The final implementation retains exact legacy count parity for all four fixtures and fixes those expectations permanently in the in-source test.

## Dependency Reconciliation

The manifest row and lock resolution were reconciled only through Bun:

```text
bun install --lockfile-only --ignore-scripts --no-progress --no-summary
Saved lockfile
```

The resulting `bun.lock` delta is exactly the dev-workspace `pixelmatch` dependency row and the sole `pixelmatch@7.2.0` package resolution. `pngjs` remains because it is independently used.

## Verification

```text
bun nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache -- -t 'compareOwnedParityPixels'
PASS after overlap repair: 9 passed, 27 skipped

bun nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache
PASS after overlap repair: 1 file, 36 tests

bun install --lockfile-only --ignore-scripts --no-progress --no-summary --frozen-lockfile
PASS: Saved lockfile

bun ./📜️script.ts verify dependencies
PASS: baseline 238, current 140, removed 98, no new dependencies

bun ./📜️script.ts verify dependencies list js --format json | jq 'length'
PASS: 77

bun ./📜️script.ts verify dependencies list rust --format json | jq 'length'
PASS: 63

bun ./📜️script.ts verify dependencies parity js
PASS: undeclared imports 0, lock mismatches 0
```

The dev project exposes no `typecheck`, `lint`, or `format` Nx targets. Consequently, the owned regions were checked directly:

```text
bunx prettier --check --range-start=199547 --range-end=211975 <dev-script>
bunx prettier --check --range-start=292170 --range-end=301694 <dev-script>
bunx prettier --check <dev-manifest>
PASS: all owned ranges/files formatted

(bunx eslint -f json <dev-script> || true) | jq <owned-region-filter>
PASS: owned PixelCompare regions: 0 lint findings

bunx tsc --noEmit --strict --types node,vitest <dev-script> | <owned-region-filter>
PASS: owned PixelCompare regions: 0 TypeScript findings
```

The whole legacy script is not a clean standalone lint/type unit: ESLint reports eight pre-existing findings outside the owned ranges, and standalone TypeScript lacks the repository's Bun/import-meta ambient configuration. The whole-file Prettier check likewise reports unrelated formatting outside the owned ranges. None is introduced or changed by this packet; no false whole-project pass is claimed.

Exact post-removal scans passed:

- no `pixelmatch` or `[DEBUG]` reference in the dev script, dev manifest, or `bun.lock`;
- no `pixelmatch` reference in production JavaScript/TypeScript manifests or sources outside ticket history;
- zero `pixelmatch` entries in the JavaScript dependency list;
- post-format full dev test is 36/36 after the overlap regression was added.

## Safe Boundary And Residual

The source and dependency packet is complete and safe for a fresh independent audit. The browser screenshot sweep against the live React and WGPU renderers remains an explicit residual owned with P3's real Worker/Wasm browser lane. It should confirm end-to-end visual acceptance under the existing screenshot threshold, but it does not block the completed source comparator, representative differential parity, or the exact 140-dependency boundary.
