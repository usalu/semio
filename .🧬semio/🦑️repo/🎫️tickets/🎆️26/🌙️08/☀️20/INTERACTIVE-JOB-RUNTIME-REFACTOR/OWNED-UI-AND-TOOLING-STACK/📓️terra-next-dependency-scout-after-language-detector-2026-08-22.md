# Terra Next Dependency Scout After Language Detector — 2026-08-22

## Selection

**Selected dependency: `sharp` `^0.34.5`** — the smallest viable in-scope direct-runtime leaf after the accepted **138 = 75 JavaScript + 63 Rust** boundary. It is neither `dagre`, `./compose`, nor a dependency already retired. Its implementation surface is one dynamic import and one image-transform chain in the print product, in a TypeScript file disjoint from the active P3/P8 Rust work.

**Recommendation: DEFER implementation until the panel-glass visual differential can be executed against a stable print fixture.** This is an excellent next serialized packet, but it is not stale: it materially generates PNG assets consumed by the second TeX pass. The current focused print test is blocked before reaching this code by an absent active-P8 styling token file, so a behavior-preserving replacement has not yet been demonstrated end-to-end.

## Current Consumers And Public-API Risk

| Item                        | Current evidence                                                                                                                                                                                                                                                                |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Direct manifest edge        | `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/package.json:22` is the sole direct declaration: `"sharp": "^0.34.5"`.                                                                                                                                                  |
| Source import / use         | `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️component.ts:130,158` dynamically imports `sharp` and invokes its sole pipeline. No other product/configuration import, `require`, export, or manifest use was found by the scoped literal scan.    |
| Runtime entry               | `renderPrintPanelGlass` is exported at `component.ts:118` and is invoked internally at `component.ts:269` after the first Tectonic PDF pass, only when the panel manifest has entries.                                                                                          |
| Observable product behavior | The function writes `.semio-panel-glass/<jobname>/<panel>.png` and `.ready`; `🧰️framework/🛍️products/📓️print/🖋️latex/semio-window.sty:1897-1916` conditionally includes those PNGs during the second TeX pass. This is a visual artifact contract, not a `sharp` type/API leak. |
| Exact transform             | Extract a calculated PDF-page rectangle, remove alpha, Gaussian blur, adjust saturation, alpha-composite a solid theme tint, encode PNG with compression level 9, then write one file per panel.                                                                                |

`renderPrintPanelGlass` has no `sharp` type in its signature. Nevertheless, because it is exported and its emitted assets are read by TeX, changing the transform must preserve dimensions, placement, opacity, and visual treatment; byte-for-byte PNG identity is not a valid requirement because encoders differ.

## Removal / Replacement Design

Retain the already-required `@napi-rs/canvas` edge: it is also needed to render PDF.js pages. Extend the private `loadPdfCanvas` adapter in the same module to expose `loadImage` alongside `createCanvas`, `Path2D`, and `DOMMatrix`.

For each cached rendered page, decode the in-memory PNG with `loadImage`, draw the same source rectangle to an opaque destination canvas of the requested `width × height`, apply the CSS-canvas equivalent blur and saturation filter, then draw the tint in normal source-over mode and emit `canvas.toBuffer("image/png")`. Keep the existing panel coordinate arithmetic, cache, output names, `.ready` marker, and second Tectonic pass unchanged. Remove the dynamic import and the direct `sharp` manifest edge afterward.

The in-memory probe reproduced the necessary retained-canvas primitives without writing a file:

```sh
bun -e 'const { createCanvas, loadImage } = require("@napi-rs/canvas"); ...'
# {"outputPngBytes":138,"width":4,"height":4}
```

It verified `loadImage`, `createCanvas`, PNG output, crop-style `drawImage`, `filter = "blur(1.5px) saturate(120%)"`, and tint compositing. It does not establish equivalence with Sharp's blur kernel or alpha-removal semantics; that is why a visual differential is mandatory.

## Required Differential Tests And Runtime Gate

1. Add a deterministic unit fixture for the private panel transform: a small RGBA source containing transparent and opaque color regions, non-zero crop origin, light and dark token styles. Assert output dimensions, fully opaque post-removal-alpha pixels, tint alpha, and decoded RGBA comparison against a pre-removal Sharp baseline with an explicit per-channel tolerance justified by blur implementation differences.
2. Add an integration fixture containing at least one real TeX panel and run the two-pass print build. Compare the retired implementation's and Canvas implementation's decoded panel PNGs and the resulting light/dark PDF rendered pages in the panel bounds. Assert the expected output files and `.ready` marker remain present.
3. Run the existing quick and long print targets through Nx, then dependency verification/list/parity, a frozen Bun install, typecheck/lint/format checks scoped to the changed print files, and a no-ignore literal/dynamic-import/export scan for `sharp`.
4. No browser UI gate is needed: this is a Node/Tectonic print pipeline. The runtime proof is the real two-pass PDF/PNG build on macOS, Linux, and Windows-capable CI because `@napi-rs/canvas` is native and the project explicitly supports those hosts.

## Lock Graph And Count Impact

The direct workspace edge appears at `bun.lock:725-734`. Removing it should change the direct dependency inventory from **138 (75 JS + 63 Rust)** to **137 (74 JS + 63 Rust)**, provided no new dependency is introduced.

Do **not** require the literal `sharp` resolution or all `@img/sharp-*` packages to disappear from `bun.lock`: `next@15.5.22` retains `sharp: ^0.34.3` as an optional dependency at `bun.lock:3393`, and the resolved `sharp@0.34.5` node remains at `bun.lock:3807`. The expected in-scope lock delta is therefore removal of the print workspace's direct edge only; a frozen install must establish the final exact lock graph. `bun.lock` was not modified during this scout.

## Commands Run

| Command                                                                                                                     | Result                                                                                                                                                                                                                          |
| --------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bun ./📜️script.ts verify dependencies`                                                                                     | PASS — baseline 238, current 138, 100 removed, no new third-party identities.                                                                                                                                                   |
| `bun ./📜️script.ts verify dependencies list js --format json                                                                | bun -e '...'`                                                                                                                                                                                                                   | 75 JS identities; exactly one direct `sharp` identity at `^0.34.5`. |
| `bun ./📜️script.ts verify dependencies parity js`                                                                           | PASS — 83 manifests, 260 external rows, 110 evidenced, 150 unowned, 0 undeclared imports, 44 lock workspaces, 0 lock mismatches, 5 lock fixtures.                                                                               |
| `bun install --frozen-lockfile`                                                                                             | PASS — postinstall completed; no lock mutation requested.                                                                                                                                                                       |
| Scoped `rg` for `sharp`, `renderPrintPanelGlass`, and panel-glass consumers, excluding `compose` and generated dependencies | Exactly the manifest and the one runtime chain above; TeX asset consumption confirmed.                                                                                                                                          |
| `bun -e` retained-canvas capability probe                                                                                   | PASS — `loadImage` and `createCanvas` are functions; filter/crop/tint/PNG in-memory probe succeeded.                                                                                                                            |
| `bun x nx show project @semio-tech/print`                                                                                   | Confirms `test-quick`, `test-long`, and `test-exhaustive` run `bun ./📜️script.ts test ...` through Nx.                                                                                                                          |
| `bun x nx run @semio-tech/print:test-quick`                                                                                 | BLOCKED before panel-glass execution: `ENOENT` for `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🔣️tokens.json` from `loadPrintDesignTokens`. This is outside the selected file surface and overlaps active P8 state. |

## Expected Packet Files

The eventual packet should be limited to:

- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️component.ts`
- `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/package.json`
- `bun.lock` (workspace direct edge; resolved transitive Sharp graph may remain because Next owns an optional edge)
- a focused print panel-glass test/fixture in the existing print verification command
- the implementation report and required ticket-local evidence only.

No product source, manifest, lockfile, coordinator report/list, or ticket metadata was changed by this scout; this report is its only write.
