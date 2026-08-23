# Terra Independent PNG.js Retirement Audit — 2026-08-23

## Verdict

**ACCEPT** — the direct `pngjs` retirement is complete on the current shared tree. Its private OS-dev binding and direct manifest/workspace-lock edge are absent; the one remaining `bun.lock` package node is the legitimate transitive dependency of `@vitest/browser@4.1.10`. No direct `pngjs` source/API edge, public surface, or unapproved product-source change was found.

The accepted prior owned-comparator packet accounts for the concurrently staged `pixelmatch` removal and comparator implementation in the same three paths. This audit accepts only the subsequent direct `pngjs` removal; it does not re-accept Phase 10, P3/P8, Rust, Compose, Dagre, or unrelated dependency work.

## Scope And Current-Tree Evidence

Read in full before auditing:

- repository `AGENTS.md`;
- governing interactivity/zero-dependency plan;
- `📓️terra-next-accepted-dependency-scout-2026-08-22.md`;
- `📓️p10-owned-png-codec-retirement-2026-08-23.md`;
- prior accepted comparator record `📓️owned-parity-pixel-comparator-2026-08-22.md` and its independent re-audit.

The current OS-dev parity script has a private `OwnedParityImage` only. It has no export of that type or of its PNG helpers. The active call graph is:

```text
Playwright screenshot bytes
  -> decodeParityScreenshot(page, bytes)
  -> compareParityRegion(page, decoded React/WGPU images, node, outDir, variant)
  -> cropOwnedParityRgba(...) + compareOwnedParityPixels(...)
  -> encodeParityDiff(page, diff, width, height) only on ratio > per-kind threshold
```

`decodeParityScreenshot` executes in the already-open target page, uses `createImageBitmap` plus Canvas `getImageData`, and copies the transferred numbers into a fresh `Uint8Array`. The focused Chromium fixture proved exact dimensions and RGBA: 4×3, first row `[12,34,56,255,20,40,60,128,0,0,0,0,0,0,0,0]`.

`cropOwnedParityRgba` requires non-negative safe-integer dimensions and coordinates, validates exact `width * height * 4` source length, rejects out-of-bounds crops, and copies complete row-major RGBA rows into fresh storage. The permanent test verifies multi-row byte selection, upper/lower-bound rejection, malformed-input rejection, source preservation, and independent destination storage.

`encodeParityDiff` requires positive safe dimensions and exact RGBA length, uses `ImageData`/Canvas `toBlob("image/png")`, and returns fresh bytes. The in-page round trip preserved exact dimensions and all 16 RGBA bytes, including opaque, alpha, mismatch (`[255,32,64,255]`), and anti-alias (`[255,192,0,255]`) marker pixels.

The changed caller preserves the former rounded rectangle selection, region collection order (now `Promise.all` returns input order), threshold selection (`0.005` default / `0.02` scene), comparator threshold (`0.1`), path sanitization, and diagnostic naming: `diff-<variant>-<sanitized-path>.png`. It awaits page encoding only for a failing region; emitted PNG byte identity remains intentionally non-contractual, while decoded RGBA, dimensions, threshold, ratio, markers, and path remain contractual.

## Direct-Edge And Lock Audit

- Scoped no-ignore scan of the OS-dev script and manifest for `pngjs|createRequire|PNG.sync|PNG.bitblt|new PNG`: no matches.
- Repository live tracked source/manifest scan excluding tickets, generated/cache trees, dependencies, and Compose for `pngjs|PNG.sync|PNG.bitblt|new PNG`: no matches.
- No non-Compose live `package.json` contains `"pngjs"`.
- `bun.lock` contains exactly the expected transitive evidence:
  - `@vitest/browser@4.1.10` declares `pngjs@^7.0.0` at line 2112;
  - resolution `pngjs@7.0.0` remains at line 3560.
- The OS-dev workspace no longer declares `pngjs`; the direct workspace row is absent from the lockfile.

Historical ticket scratch scripts still contain old `pngjs` examples. They are outside the live product/configuration boundary, were not modified, and are not a live binding. A broad literal scan also sees generated/cache material; neither is evidence of a direct dependency edge.

The combined HEAD diff of the three shared paths includes the previously accepted 2026-08-22 owned `pixelmatch` comparator removal as well as the PNG codec change. Its current PNG-specific delta removes `createRequire`, `OwnedPng`, `OwnedPngConstructor`, `PNG.sync.read`, `PNG.bitblt`, and `PNG.sync.write`; it adds only the private Canvas helpers, their focused permanent tests, and asynchronous awaiting at the existing private caller. No production source outside the OS-dev parity harness changed for this retirement.

## Re-executed Gates

| Gate | Command | Result |
| --- | --- | --- |
| Focused permanent crop + Chromium fixture | `bun ./📜️script.ts test quick --testNamePattern='crops complete RGBA rows|preserves fixed CSS color'` from OS-dev package | PASS — 2 passed, 36 skipped; includes the real headless Chromium Canvas decode/crop/encode round trip. |
| Full permanent OS-dev suite | `bun x nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache` | PASS — 1 file, 38/38 tests. The only output besides success was the known non-failing `NO_COLOR`/`FORCE_COLOR` warning. |
| Frozen lock | `bun install --frozen-lockfile` | PASS — 1,946 installs checked across 1,998 packages; no changes. |
| Dependency ratchet | `bun ./📜️script.ts verify dependencies` | PASS — baseline 238, current 137, 101 removed, no new third-party identities. |
| JavaScript dependency list | `bun ./📜️script.ts verify dependencies list js --format json` plus count probe | PASS — `{"javascript":74,"pngjs":0}`. |
| Rust dependency list | `bun ./📜️script.ts verify dependencies list rust --format json` plus count probe | PASS — `{"rust":63}`. |
| Manifest/source/lock parity | `bun ./📜️script.ts verify dependencies parity js` | PASS — 83 manifests, 259 external rows, 110 evidenced, 149 unowned, 0 undeclared imports, 44 lock workspaces, 0 lock mismatches, 5 fixtures. |
| Formatting | `bun x prettier --check <OS-dev script> <OS-dev package>` | PASS. |
| Packet-path whitespace | `git diff --check -- <OS-dev script> <OS-dev package> bun.lock`, `git diff --cached --check -- ...`, and `git diff HEAD --check -- ...` | PASS — no output. |
| Unstaged global whitespace | `git diff --check` | PASS — no output. |

`git diff HEAD --check` over the whole concurrently dirty tree fails only on unrelated P8 ticket JSON/Markdown artifacts (trailing whitespace and EOF blank lines), not on this packet's script, manifest, or lockfile. It is recorded as shared-tree noise and does not block this scoped retirement.

## Provisional Dependency Boundary

**137 third-party identities = 74 JavaScript + 63 Rust.** This is the scout's expected one-identity reduction from 138. The count is provisional for the repository's broader concurrent dependency work; this audit attributes and accepts only removal of the direct `pngjs` identity.

## Residuals

- The retained lock resolution belongs to `@vitest/browser`, not this package; it must remain until that separate dependency is retired.
- A full React/WGPU product parity variant was not part of this audit, as its P3/P8 inputs remain concurrently active. The exact codec replacement is covered by real Chromium screenshot decode/crop/encode tests and the complete OS-dev quick suite.
