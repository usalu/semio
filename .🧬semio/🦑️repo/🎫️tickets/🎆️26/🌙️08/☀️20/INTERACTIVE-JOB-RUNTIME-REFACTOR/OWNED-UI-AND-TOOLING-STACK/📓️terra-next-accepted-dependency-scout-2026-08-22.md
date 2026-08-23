# Terra Next Accepted Dependency Scout — 2026-08-22

## Recommendation

**ACCEPT — retire direct `pngjs` `^7.0.0`.** It is the smallest current in-scope external dependency leaf after the accepted **138 = 75 JavaScript + 63 Rust** boundary. It has one direct development manifest edge, one private dynamic binding, and one parity-harness use site. The same Chromium/Playwright browser already used by that harness independently reproduced PNG decode, RGBA extraction, crop-compatible Canvas operations, diagnostic-PNG encoding, and byte transfer without `pngjs`.

This packet is TypeScript-only and file-disjoint from the active P3/P8 Rust work. It excludes `dagre`, `./compose`, Sharp (recorded separately as a DEFER), and all previously removed identities.

## Candidate Selection

| Candidate                                               | Decision     | Evidence                                                                                                                                                                                                               |
| ------------------------------------------------------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `sharp`                                                 | DEFER        | Recorded in `📓️terra-next-dependency-scout-after-language-detector-2026-08-22.md`: a real print artifact dependency whose PDF/PNG visual differential is blocked before execution by missing active-P8 styling tokens. |
| `@napi-rs/canvas`                                       | Not selected | It supplies the PDF.js canvas runtime for the print flow, so it is not a small leaf.                                                                                                                                   |
| `@mdx-js/rollup`, Storybook addons, Tailwind typography | Not selected | Current Storybook/Tailwind configuration imports or configures them; they are not stale edges.                                                                                                                         |
| `pngjs`                                                 | **ACCEPT**   | One dynamic codec binding private to the OS parity diagnostic harness; browser Canvas reproduces its needed observable operations now.                                                                                 |

## Exact Consumers And Source Scan

| Surface                     | Evidence                                                                                                                                                                                                                                                                             |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Direct manifest             | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/package.json:29` is the sole non-Compose manifest edge: `"pngjs": "^7.0.0"`.                                                                                                                                 |
| Private binding             | `📜️script.ts:5,56-65` imports `createRequire`, declares unexported `OwnedPng`/`OwnedPngConstructor`, and creates its only `pngjs` binding.                                                                                                                                           |
| Decode and compare          | `📜️script.ts:3666-3685` crops two screenshots, runs the existing owned RGBA comparator, and emits an optional failure diagnostic. `📜️script.ts:4333-4334` decodes two Playwright screenshots.                                                                                        |
| Public API/type exposure    | None. `OwnedPng`, `OwnedPngConstructor`, `compareParityRegion`, `PixelRegionResult`, and `diffPng` are private implementation/report details; no package export or public signature exposes a PNG.js type.                                                                           |
| Dynamic/binding/export scan | The no-ignore scoped scan found only this manifest edge, the private `createRequire(...)("pngjs")` binding, its eight codec operations, and lock entries. No other import, dynamic import, require, namespace binding, export, configuration key, or public API use exists in scope. |

The reported diagnostic contract remains unchanged: a pixel mismatch continues to return the same path/ratio/threshold and, above threshold, writes `diff-<variant>-<path>.png`. PNG byte identity is not contractual; decoded RGBA pixels and the file path are.

## Owned Replacement

Keep the existing pure `compareOwnedParityPixels` algorithm. Replace the private PNG abstraction with three local helpers in the existing `📜️script.ts`:

1. `decodeParityScreenshot(page, bytes)`: pass `Array.from(await page.screenshot())` to the same Playwright page; `createImageBitmap(new Blob(...))`, draw into an off-screen Canvas, then return `{ width, height, data: Uint8Array }` from `getImageData`.
2. `cropOwnedParityRgba(image, x, y, width, height)`: copy the requested RGBA rows into a new `Uint8Array`; retain the existing bounds/rounding calculation exactly.
3. `encodeParityDiff(page, data, width, height)`: `putImageData` into an off-screen Canvas, call `toBlob("image/png")`, return its bytes, and write the same failure path. This makes `compareParityRegion` asynchronous and the region collection awaits it.

This removes `createRequire`, both private PNG.js type aliases, the one dynamic binding, and the direct manifest edge. It adds no dependency or public API. Canvas decoding/encoding happens in the already-open actual target page, so it neither introduces a Node image codec nor expands the runtime to another host.

## Reproduced Browser/Runtime Proof

The following real headless Chromium probe used Bun and the repository-local Playwright cache. It created an 8×6 page colored `rgb(12,34,56)`, captured a Playwright PNG screenshot, decoded it with `createImageBitmap` plus Canvas, re-encoded the returned RGBA through `canvas.toBlob("image/png")`, and received:

```json
{ "decodedWidth": 8, "decodedHeight": 6, "firstPixel": [12, 34, 56, 255], "encodedBytes": 111 }
```

This proves the browser-side replacement's dimensions, exact first RGBA pixel, PNG decode, Canvas crop-capable image path, and diagnostic PNG encoding. It is stronger than an assumed Web API availability check and is isolated from P3/P8 Rust compilation.

## Required Implementation Gates

| Gate                 | Required command / assertion                                                                                                                                                                                                                                                                                                           |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Focused unit tests   | Extend the existing `compareOwnedParityPixels` suite in `📜️script.ts:5057+` with row-crop bounds/byte-preservation and Canvas encode/decode round-trip fixtures; run `bun x nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache`.                                                                                           |
| Browser differential | On fixed CSS-color, alpha, and cropped-pattern pages, compare the current PNG.js baseline and Canvas replacement decoded RGBA buffers exactly; assert the diff artifact decodes to the same mismatch-marker pixels. Then run the existing parity browser command for one already-built variant only after its P3/P8 inputs are stable. |
| Source/API absence   | No-ignore `rg` for `pngjs`, `createRequire`, `PNG.sync`, `PNG.bitblt`, and `new PNG` across the non-Compose product/configuration surface; permit only the transitive lock evidence below.                                                                                                                                             |
| JS packet check      | Re-run the exact JS target above; do not invoke a Cargo task. The pre-change target passed: 1 file, 36 tests.                                                                                                                                                                                                                          |
| Dependency/lock      | `bun install --frozen-lockfile`; `bun ./📜️script.ts verify dependencies`; `bun ./📜️script.ts verify dependencies list js --format json`; and `bun ./📜️script.ts verify dependencies parity js`.                                                                                                                                        |
| Scoped hygiene       | `bunx prettier --check <dev-script> <dev-package-manifest>` and `git diff --check -- <dev-script> <dev-package-manifest> bun.lock`.                                                                                                                                                                                                    |

## Graph Impact And Expected Count

Current verification is clean at **138** total identities and **75** JavaScript identities. Removing this sole direct manifest identity should produce **137 = 74 JavaScript + 63 Rust** with no replacement dependency.

The package's resolved lock node must remain: `@vitest/browser@4.1.10` depends on `pngjs@^7.0.0` at `bun.lock:2113`, resolving `pngjs@7.0.0` at `bun.lock:3561`. The expected lock change is removal of the OS-dev workspace's direct `pngjs` entry at `bun.lock:717`, not a zero-literal lock scan. The implementation acceptance criterion is zero direct manifest/API/source edge from this packet and a frozen, parity-clean lock graph.

## Commands Run

| Command                                                                    | Result                                                                                                                                            |
| -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| Scoped no-ignore `rg` for `pngjs`, `PNG.sync`, `PNG.bitblt`, and `new PNG` | Exactly one manifest, one private binding, eight codec operation lines in `📜️script.ts`, and the described lock edges.                            |
| Headless Chromium Canvas decode/encode probe via `bun -e` and `playwright` | PASS — 8×6, `[12,34,56,255]` first pixel, 111-byte re-encoded PNG.                                                                                |
| `bun x nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache`     | PASS — 1 file, 36 tests.                                                                                                                          |
| `bun install --frozen-lockfile`                                            | PASS — 1,946 installs checked; no changes.                                                                                                        |
| `bun ./📜️script.ts verify dependencies`                                    | PASS — baseline 238, current 138, 100 removed, no new third-party identities.                                                                     |
| `bun ./📜️script.ts verify dependencies list js --format json               | bun -e '...'`                                                                                                                                     | 75 JS identities; `pngjs` exactly once at `^7.0.0`. |
| `bun ./📜️script.ts verify dependencies parity js`                          | PASS — 83 manifests, 260 external rows, 110 evidenced, 150 unowned, 0 undeclared imports, 44 lock workspaces, 0 lock mismatches, 5 lock fixtures. |

## Expected Packet Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/package.json`
- `bun.lock`
- focused tests retained in the existing `📜️script.ts` test suite
- implementation evidence inside this master ticket only.

No product source, manifest, lockfile, coordinator file, or ticket metadata was changed by this scout. This report is its only write.
