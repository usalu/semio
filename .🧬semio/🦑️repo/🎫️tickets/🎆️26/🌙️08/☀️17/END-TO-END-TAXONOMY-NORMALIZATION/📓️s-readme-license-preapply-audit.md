# README and LICENSE Preapply Audit

## Verdict

**READY with zero observed drift.** The frozen README/LICENSE authority remains an exact match for the current worktree. No production, taxonomy, normalization, source, destination, or Git state was changed by this audit.

## Frozen Authority Integrity

- Golden SHA-256: `051394741822e92d51f3bda15ce64d84c236582c6927335c9c5e0ac3c18a1da4`, exactly matching the frozen authority report.
- Exact current corpus: 40 regular leaves, comprising 32 `README.md` and eight `LICENSE.md` leaves.
- Every source is present with its frozen byte size, `0644` mode, and SHA-256 preimage.
- Dispositions remain exact: four fixed publisher leaves, 28 owner-documentation projections, four attribution projections, one configurable owner-license projection, and three ticket evidence/scratch projections.
- The golden contains no source, destination, evidence, reference-owner, or generator-owner path within any opaque Compose prefix.

## Publisher-Fixed Leaves

All four fixed leaves remain present and byte-current at their recorded package roots: the React package `README.md` and `LICENSE.md`, the repo VS Code package `LICENSE.md`, and the repo coordinator package `LICENSE.md`.

The focused suite revalidated the adjacent non-private manifests and all three independent `bun pm pack --dry-run --ignore-scripts` package roots. It also revalidated the VS Code `.vscodeignore` selection: `README.md` excluded and `LICENSE.md` retained.

## Projection Readiness

- Exact configurable projections: 36.
- Exact destination occupancy: zero; every destination currently returns `ENOENT`.
- Projected destinations are mutually unique and unoccupied under byte-exact, NFC, case-folded NFC, and VS16-folded NFC comparisons against the current admitted path ledger.
- Maximum projected destination length: 144 UTF-8 bytes, below the configured 240-byte limit.
- Collision drift: zero.

## Owner and Consumer Stability

- All 21 recorded owner-evidence classifications remain present in the golden with the frozen per-class counts.
- All 14 concrete owner-evidence paths are present.
- The 62 reference-owner bindings remain partitioned exactly across six owners: Markdown relative-reference adaptation (36), repo CLI development-document discovery (16), Bun publication (4), asset distribution (4), CommonMark scratch reading (1), and VS Code package selection (1).
- Current concrete consumers remain stable: the repo CLI still exposes `EmojiFileDocs = 📃️` and `EmojiFileLicense = ⚖️` and still reads the raw technology/bundle `README.md` locations; the CommonMark scratch reader still reads `README.md`; the VS Code ignore contract remains unchanged.
- The `assets-build` generator owner remains registered at its current raw output and the frozen required output remains `🧰️framework/🔨️modules/🖼️assets/📃️readme/📝️.md`.
- The normalization engine still contains the structured Markdown reference adapter and move-linked reference-edit pipeline; no owner binding has disappeared.

## Focused Verification

```text
bun test --timeout 30000 './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️readme-license-owner-authority.test.ts'
```

Result on 2026-08-27:

```text
4 pass
0 fail
615 expect() calls
Ran 4 tests across 1 file. [12.63s]
```

This includes the language-neutral golden/preimage/path-budget authority and independent parity through `fast-glob`, `ignore`, and Bun's package publisher.

## Exact Readiness

The lane is ready for a single exact signed transaction containing the 36 configured moves plus their registered reference and generator updates. The four publisher-fixed leaves must remain in place. A post-apply empty replan and the publisher/generator/reference gates are still required; this preapply audit did not execute the transaction.

Actual `compose/**`, `temp/compose/**`, and `temp-compose/**` were not traversed, read, or modified.
