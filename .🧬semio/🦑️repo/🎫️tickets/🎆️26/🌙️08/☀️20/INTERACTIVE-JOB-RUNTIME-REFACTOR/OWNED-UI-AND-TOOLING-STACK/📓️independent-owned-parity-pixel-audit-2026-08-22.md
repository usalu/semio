# Independent Owned Parity Pixel Audit

Date: 2026-08-22

## Verdict

**REJECT** — the private comparator does not preserve its input semantics when the supplied output `Uint8Array` partially overlaps either RGBA input. This is an observable, deterministic correctness failure and violates the required stable-diff-buffer behavior.

## Exact Blocker

`compareOwnedParityPixels` writes directly to `diffRgba` while later pixels still read `referenceRgba` and `candidateRgba`; it neither rejects overlapping byte spans nor snapshots the needed input. `Uint8Array` permits distinct views of one `ArrayBuffer`, so identity checks alone would also be insufficient.

The independently executed source-extraction probe used the retained 3-by-3 text-edge fixture with `reference = shared.subarray(0, 36)` and `diff = shared.subarray(4, 40)`. With an independent diff buffer, the comparator returned **2** and emitted the fixed retained markers. With the partially overlapping diff buffer, the identical logical inputs returned **8** and produced mismatch markers after its own writes overwrote future reference pixels. The reference input was mutated as well.

Required remediation: before any output write, reject every overlap between `diffRgba` and either input byte range (same underlying buffer *and* intersecting `[byteOffset, byteOffset + byteLength)` spans), with a permanent partial-overlap fixture. A full snapshot would also be correct but is contrary to the packet's no-per-pixel-allocation/performance intent. Re-run representative differential fixtures after the guard is added.

## Independent Evidence

The asserted packet scope is otherwise accurate: the staged owned diff is 274 additions/2 removals in the dev script, one manifest-row deletion, and three lock-line deletions. It removes the direct dev-manifest `pixelmatch` row and the sole `pixelmatch@7.2.0` lock resolution. No production/runtime/build import or package-manifest occurrence remains outside ticket history and the historical dependency baseline; `bun.lock` has no occurrence. The owned comparator and options type are not exported, so no public external type leaks from this implementation.

The retained `📊️owned-parity-pixel-differential-2026-08-22.json` is internally consistent: `opaque`, `transparent`, `text-edge`, and `scene-gradient` retain legacy counts of 1, 1, 2, and 0, respectively; marker totals agree with the documented policy. Its coverage is representative rather than exhaustive and does not expose the overlap defect.

Focused checks run independently:

- `bun nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache -- -t 'compareOwnedParityPixels'` — 8 passed, 27 skipped.
- `bun nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache` — 35 passed.
- `bun install --lockfile-only --ignore-scripts --no-progress --no-summary --frozen-lockfile` — passed.
- `bun ./📜️script.ts verify dependencies` — baseline 238, current **140**, 98 removed, no additions.
- JavaScript/Rust dependency counts — **77 + 63 = 140**; JS dependency parity reports 0 undeclared imports and 0 lock mismatches.
- Scoped `git diff --cached --check` for the three owned packet files — passed. The whole staged diff separately reports an unrelated pre-existing ticket-file missing-final-newline issue, not an owned-packet source or lock defect.
- Exact scans — no `[DEBUG]` in owned source/manifest/lock; no `pixelmatch` in runtime/build sources, manifests, or `bun.lock`.

## Boundary Review

For non-overlapping typed-array views, the implementation is deterministic row-major code with complete muted/marker writes, exact safe dimension and byte-length validation, finite `[0, 1]` threshold validation, alpha-over-white comparison, bounded 3-by-3 anti-alias classification, and no per-pixel object/array allocation. Adversarial direct probes confirmed zero dimensions return 0 without division, threshold 0/1 behavior is finite, and `Infinity` is rejected. The AA path cannot divide by zero because zero contrast fails before coverage division.

A proxy around the options object is benign if it returns primitive values; a proxy around a typed array throws the engine's typed-array receiver error. Neither changes the blocker: ordinary, valid `Uint8Array` views already reproduce it.

The live React-versus-WGPU browser screenshot sweep remains correctly described as a residual in the supplied packet report. No live-browser PASS is asserted by that report or this audit; it was not re-run because it belongs to the P3 Worker/Wasm browser lane and cannot establish or clear the local overlap failure.
