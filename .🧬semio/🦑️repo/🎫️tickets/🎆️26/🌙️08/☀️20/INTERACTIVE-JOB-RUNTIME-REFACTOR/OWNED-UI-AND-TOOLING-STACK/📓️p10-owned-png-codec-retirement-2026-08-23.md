# P10 Owned PNG Codec Retirement — 2026-08-23

## Status

**IMPLEMENTATION GATES COMPLETE; AUDIT PENDING.** This is the bounded Phase 10 dual-run replacement of the OS-dev parity harness's direct `pngjs` codec only. It does not accept Phase 10, claim the zero-dependency end state, or accept any P3, P8, Rust, Compose, Dagre, coordinator, or unrelated dependency work. A separate Terra audit owns acceptance.

The governing plan's Phase 10 zero-dependency rule requires each replacement to dual-run against the outgoing dependency before the old default is deleted. The accepted Terra scout identified this private codec as a one-manifest, one-binding leaf. The implementation kept `pngjs` installed through the differential, proved decoded-pixel parity in real Chromium, and only then removed the direct source/manifest/lock-workspace edge.

## Authorized Surface

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/package.json`
- `bun.lock`
- this implementation report

No Cargo command was run. No Rust, Compose, Dagre, coordinator, ticket metadata, `AGENTS.md`, launch configuration, or additional script was edited by this packet.

## Implementation

The private `createRequire(import.meta.url)("pngjs")` binding and its `OwnedPng` constructor facade were deleted. Three owned helpers now cover the exact private operations in the already-open Playwright page:

1. `decodeParityScreenshot(page, bytes)` decodes browser screenshot PNG bytes with `createImageBitmap`, draws to an off-screen Canvas, and returns owned width, height, and copied RGBA bytes.
2. `cropOwnedParityRgba(image, x, y, width, height)` validates exact image byte length and non-negative safe-integer bounds, copies complete row-major RGBA rows, and never aliases or mutates source storage.
3. `encodeParityDiff(page, data, width, height)` writes exact RGBA through `ImageData`/Canvas and returns `toBlob("image/png")` bytes for the existing diagnostic path.

`compareParityRegion` is now asynchronous only because failure diagnostics are browser-encoded. The existing region-selection order, rounded `x/y`, bounded rounded width/height calculation, per-kind threshold, mismatch ratio, path sanitization, and `diff-<variant>-<path>.png` naming remain unchanged. `compareOwnedParityPixels` remains the comparator, including exact opaque mismatch marker `[255,32,64,255]` and ignored-antialias marker `[255,192,0,255]`. Diagnostic PNG byte identity is intentionally non-contractual; decoded dimensions and RGBA pixels are exact.

Permanent tests retained in the existing `📜️script.ts` cover:

- multi-row crop byte selection;
- lower/upper bounds and malformed source-byte rejection;
- independent crop storage and source-byte preservation;
- real Chromium fixed CSS `rgb(12 34 56)` decoding;
- real Chromium `rgb(20 40 60 / 50%)` alpha decoding;
- transparent screenshot bytes;
- Canvas encode/decode round-trip with opaque, alpha, mismatch-marker, and antialias-marker pixels.

## Required Dual-Run Differential

The outgoing dependency remained installed and bound for the differential test. The actual headless Chromium fixture was a 4×3 transparent page:

- pixel `(0,0)`: fixed CSS `rgb(12 34 56)`;
- pixel `(1,0)`: fixed CSS `rgb(20 40 60 / 50%)`;
- remaining ten pixels: transparent.

Canvas decode and `PNG.sync.read` matched in dimensions and all 48 RGBA bytes. The exact first row was:

```text
[12,34,56,255, 20,40,60,128, 0,0,0,0, 0,0,0,0]
```

The Canvas 2×1 crop at `(0,0)` matched `PNG.bitblt` in all eight bytes. A Canvas-encoded 2×2 diagnostic containing `[12,34,56,255]`, `[20,40,60,128]`, `[255,32,64,255]`, and `[255,192,0,255]` decoded through PNG.js to the same 16 bytes. Semantic mismatches: **0**.

The first differential run exposed a harness-input detail rather than a pixel mismatch: PNG.js `sync.read` calls Buffer's `readUInt32BE` and rejected the Canvas helper's plain `Uint8Array`. The temporary legacy assertion wrapped only that outgoing decoder input with `Buffer.from(encoded)`. The rerun passed 39/39, after which the temporary legacy test, binding, manifest declaration, and workspace lock edge were removed. The permanent browser proof then passed without any `pngjs` source reference.

## Commands and Results

| Command                                                                                                                                                                                                                | Result                                                                                                                                         |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `bunx prettier --write <OS-dev-script>` before the dual run                                                                                                                                                            | PASS; formatted the temporary differential implementation.                                                                                     |
| `bun x nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache` (first dual run)                                                                                                                                | Expected harness correction required: 38 passed, one failed because PNG.js received a plain `Uint8Array`; no pixel assertion failed.           |
| `bun x nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache` (corrected dual run, dependency still present)                                                                                                  | PASS; one file, 39/39 tests; exact legacy/Canvas parity described above.                                                                       |
| `bun ./📜️script.ts test quick -t 'crops complete RGBA rows\|preserves fixed CSS color'` from the OS-dev package                                                                                                        | PASS; two focused permanent tests, 36 skipped.                                                                                                 |
| `bun x nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache` after removal                                                                                                                                   | PASS repeatedly; final run one file, 38/38 tests. The repeated `NO_COLOR`/`FORCE_COLOR` messages are existing non-failing Bun warnings.        |
| `bun install --frozen-lockfile`                                                                                                                                                                                        | PASS; 1,946 installs checked across 1,998 packages, no changes.                                                                                |
| `bun ./📜️script.ts verify dependencies`                                                                                                                                                                                | PASS; baseline 238, current 137, 101 removed, no new third-party identities.                                                                   |
| `bun ./📜️script.ts verify dependencies list js --format json`                                                                                                                                                          | PASS; raw list contains no `pngjs`.                                                                                                            |
| `bun ./📜️script.ts verify dependencies list js --format json \| bun -e 'const rows=await Bun.stdin.json(); console.log(JSON.stringify({javascript:rows.length,pngjs:rows.filter((row)=>row.name==="pngjs").length}))'` | PASS; `{"javascript":74,"pngjs":0}`. Together with the unchanged accepted Rust boundary this is provisional **137 = 74 JavaScript + 63 Rust**. |
| `bun ./📜️script.ts verify dependencies parity js`                                                                                                                                                                      | PASS; 83 manifests, 259 external rows, 110 evidenced, 149 unowned, 0 undeclared imports, 44 lock workspaces, 0 lock mismatches, 5 fixtures.    |
| `rg -n --hidden --no-ignore-vcs 'pngjs\|createRequire\|PNG\.sync\|PNG\.bitblt\|new PNG' <OS-dev-script> <OS-dev-package>`                                                                                              | PASS by absence; exit 1 with no matches.                                                                                                       |
| `rg -n --hidden --no-ignore-vcs '"pngjs"' --glob 'package.json' --glob '!node_modules/**' --glob '!🧰️framework/🛍️products/💻️os/🔨️modules/🧱️compose/**' .`                                                              | PASS by absence; exit 1 with no non-Compose live manifest match.                                                                               |
| `rg -n 'pngjs' bun.lock`                                                                                                                                                                                               | Expected two transitive lines only: `@vitest/browser` dependency at line 2112 and `pngjs@7.0.0` resolution at line 3560.                       |
| `bunx prettier --check <OS-dev-script> <OS-dev-package>` before the final write                                                                                                                                        | Correctly found one post-removal formatting change; `--write` repaired it.                                                                     |
| `bunx prettier --check <OS-dev-script> <OS-dev-package>` final                                                                                                                                                         | PASS; both files match Prettier.                                                                                                               |
| `git diff --check -- <OS-dev-script> <OS-dev-package> bun.lock`                                                                                                                                                        | PASS.                                                                                                                                          |
| `git diff --check`                                                                                                                                                                                                     | PASS across the whole concurrently dirty worktree.                                                                                             |

## Lock and Count Boundary

The OS-dev workspace's direct `pngjs` row is absent. `bun.lock` deliberately still contains the package because `@vitest/browser@4.1.10` depends transitively on `pngjs@^7.0.0`; its `pngjs@7.0.0` resolution must remain until that unrelated dependency boundary is retired. A zero-literal lock scan would therefore be incorrect.

The dependency verifier's current total is exactly the scout's expected provisional boundary: **137 third-party identities = 74 JavaScript + 63 Rust**. This packet claims only the one-identity `pngjs` reduction from 138; it does not attribute or accept the other concurrently present dependency removals.

## Concurrent Tree and Residuals

The worktree and index were already broadly dirty. The authorized OS-dev files and `bun.lock` contained accepted/staged comparator and other dependency-retirement changes before this implementation, and concurrent workers continued staging shared paths. Git cannot provide packet attribution for those pre-existing changes. The packet-local source/manifest absence, exact helper call sites, frozen workspace snapshot, dependency counts, and runtime tests establish this bounded result without assigning unrelated HEAD diff rows to P10 `pngjs`.

A full product parity-server variant was not launched: its React/WGPU P3/P8 inputs are concurrently changing, and the scout permits that command only after those inputs are stable. The codec-specific real-Chromium differential did run against actual Playwright screenshot bytes and is exact. The separate Terra auditor/coordinator may add a stable already-built variant run when those inputs are quiescent.

Residual contractual facts:

- diagnostic PNG bytes may vary with Chromium encoder versions;
- decoded diagnostic RGBA pixels, dimensions, mismatch ratio/threshold, marker pixels, and output path are the contract and are covered;
- `pngjs` remains a transitive lock resolution only;
- Phase 10 and the zero-dependency end state remain open for independent work and audit.
