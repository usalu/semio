# Next Dependency Scout — 2026-08-22

## Verdict

**Replace the one-call `pixelmatch` image comparator with an owned parity-pixel comparator.** It is the smallest remaining genuine external-dependency packet that is file-disjoint from the active P3 browser worker/frame-job/OS-host sources and P8 plugin component/reactor sources. It removes exactly one JavaScript identity, taking the verified dependency boundary from **141 = 78 JavaScript + 63 Rust** to **140 = 77 JavaScript + 63 Rust**.

This is not a declaration-only deletion: `pixelmatch` has one live executable import and one live call. Its entire consumed surface is finite and private, and it can be replaced behind a local owned function without leaking an external API or touching a product interactive runtime.

`dagre` is expressly excluded. P10bp confirms that Dagre remains present, and the required real browser/Wasm gate is still outstanding. Do not select or co-edit it in this wave.

## Reproduced Boundary

The following read-only commands completed on the current shared tree:

```text
bun ./📜️script.ts verify dependencies list js --format json
bun ./📜️script.ts verify dependencies list rust --format json
bun ./📜️script.ts verify dependencies parity js
```

| Check | Result |
| --- | ---: |
| JavaScript identities | 78 |
| Rust identities | 63 |
| Combined boundary | 141 |
| JS manifests / external rows / evidenced rows | 83 / 263 / 113 |
| JS undeclared imports / lock mismatches | 0 / 0 |

The exact list outputs are retained as `📊️next-dependency-scout-js-list-2026-08-22.json` in this ticket and `PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/📊️next-dependency-scout-rust-list-2026-08-22.json`. The current P10bp independent audit also accepts the owned directed Diagram path, but does **not** retire Dagre.

## Exhaustive Pixelmatch Footprint

| Surface | Evidence | Result |
| --- | --- | --- |
| Direct manifest | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/package.json:28` | One direct row. |
| Executable import | `…/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts:55` | One default import. |
| Executable call | same script, `:3520` | One `pixelmatch` call inside `compareParityRegion`. |
| Public/type surface | `@semio-tech/framework-os-dev` exports `🟦️glue.ts`; the comparator is not exported. No Pixelmatch-derived type appears in source. | Private. |
| Dynamic/config/script references | Exhaustive literal scans across TypeScript, JavaScript, manifests, lock/config/script files found no other executable/config/dynamic import or CLI call. | None. |
| Lock reachability | `bun.lock:718` has the sole workspace edge; `:3556` is the sole `pixelmatch@7.2.0` resolution. | Reconcile through Bun. |
| Shared transitive | `pngjs` remains directly declared and is also required by `@vitest/browser`; it must not be claimed removed. | No collateral removal claim. |

`pixelmatch` is used only to compare two identically sized PNG crops during the existing React-versus-WGPU visual parity harness. It returns a mismatch count and writes a diagnostic diff PNG. It does not handle browser input, scheduling, Worker messages, product state, or plugin component/reactor authority.

## Owned Contract

Add an unexported, explicitly bounded `compareOwnedParityPixels` helper in the `//#region 🔖️PixelCompare` region of the existing mandated `📜️script.ts`. It owns the only consumed contract:

```text
compareOwnedParityPixels(referenceRgba, candidateRgba, diffRgba, width, height, {
  threshold: 0.1,
  ignoreAntialiasing: true,
}) -> mismatchCount
```

Required semantics:

1. Validate `width * height * 4` byte lengths and return a clear owned error for malformed input; retain the all-identical fast path.
2. Compare colors with an owned documented perceptual-distance formula and the existing `0.1` threshold meaning. Handle alpha deterministically before color comparison.
3. Preserve the harness's anti-alias suppression intent (`pixelmatch` currently receives `includeAA: false`): a local 3×3 bounded neighborhood classifier must ignore only edge coverage differences, not genuine local contrast changes.
4. Write an owned diagnostic diff: muted reference pixels, a fixed mismatch colour, and a distinct anti-alias marker. The file remains PNG through the already-retained owned `PNG` port.
5. Traverse once in stable row-major order with no per-pixel allocation, bounded neighbor reads, and no DOM/Worker/browser-global access. This is developer-tooling execution, not an interactive callback, but predictable linear work is still required for large screenshot crops.

Do not recreate a generic Pixelmatch-compatible package or export its types. The formula and bounded classifier must be authored from the desired owned contract and measured against the old comparator; no verbatim vendoring.

## Required Proof Before Retirement

1. Add focused in-source `import.meta.vitest` coverage in the existing `📜️script.ts` suite: identity, below/above threshold, alpha, exact diff pixels, one-pixel line anti-alias suppression, real high-contrast edge mismatch, malformed length, and a bounded large-image allocation/timing smoke check.
2. Add a temporary test-only differential fixture set containing opaque, transparent, text-edge-like, and scene-gradient crops. Run the old comparator and the owned comparator side by side and record mismatch-count/diff-policy results. The permanent suite must encode the approved owned results, then the old import/fixture is removed; do not retain a production dual path.
3. Run `bun nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache`; this is the package's existing in-source Vitest target. Run focused format/lint/typecheck targets if exposed by the current project router.
4. Run the existing browser parity harness only after P3's real Worker/Wasm lane is released and its browser gate is executable. A passing unit differential cannot certify the visual threshold on real renderer screenshots. This browser result is a packet gate, not a prerequisite to start isolated implementation.
5. Reconcile only through `bun install --lockfile-only --ignore-scripts --no-progress --no-summary`, then frozen-lock validation. Run dependency freeze/list/parity and exact scans for `pixelmatch`. Expect JavaScript **77**, total **140**, zero undeclared imports, and zero lock mismatches.

## Exact Changes And Serialization

| File | Change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` | Replace the one import/call with the owned private comparator and its tests. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/package.json` | Remove `pixelmatch`. |
| `bun.lock` | Bun-reconciled removal only. |
| This ticket | Implementation/differential/audit evidence. |

The code file is not in the active P3 source set (`renderer`, `browser_worker`, `frame_job`, `os_host`) or P8's plugin component/reactor set. The comparator region is also separate from this script's plugin-materialization imports. Integration conflicts remain only the globally shared `bun.lock`; serialize its reconciliation with any other manifest packet. Do not modify `pngjs`, renderer packages, P3 sources, P8 sources, Dagre/Diagram, Cargo manifests, or ticket lifecycle state.

## Browser Risk

There is no user-facing browser API change. The residual risk is false pass/fail classification in the visual parity gate: a comparator that does not correctly distinguish genuine geometry drift from font/GPU anti-aliasing would either conceal a regression or create noise. Unit differential fixtures establish local semantics; a real-browser parity sweep after P3's Worker/Wasm readiness establishes the actual screenshot threshold. Until that sweep passes, the comparator may be source-complete but is not an accepted visual-parity gate replacement.

## Rejected Smaller-Looking Rows

- `dagre`: explicitly blocked pending its real browser/Wasm gate and shares the Diagram integration boundary.
- `pngjs`: live PNG read/write port and also reachable from browser test tooling.
- `@tailwindcss/postcss`, `@mdx-js/rollup`, `@bytecodealliance/jco`, `binaryen`, `jsdom`: active config/script/tooling consumers, not stale rows.
- `katex`: direct dynamic renderer import and overlaps the active renderer lane.
- Platform graphics, OS-host, plugin-host, DnD, i18n, React Flow, and database boundaries: active runtime/public-contract redesigns rather than the next smallest isolated packet.

No product source, configuration, manifest, lockfile, Cargo command, or Git state was modified by this scout.
