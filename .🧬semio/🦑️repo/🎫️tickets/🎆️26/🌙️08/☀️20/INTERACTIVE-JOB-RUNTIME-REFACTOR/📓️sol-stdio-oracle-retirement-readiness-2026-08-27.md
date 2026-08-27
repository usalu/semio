# Sol Stdio Oracle Retirement Readiness — 2026-08-27

## Decision

Do not retire the whole stdio oracle crate yet.

The repository has committed language-neutral scenario definitions, mutation inputs, schemas, fixtures, and domain outcomes, but it does not have committed language-neutral goldens for the live oracle semantic projections. Retiring all third-party implementations now would remove the independent differential evidence instead of freezing and replacing it.

## Evidence

The isolated manifest `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.toml` owns 25 optional third-party identities behind its `oracles` feature:

```text
pdf-writer, lopdf, png, gif, zip, flate2, hound, csv, image, tobj,
stl_io, quick-xml, json, html5ever, markup5ever_rcdom, comrak, dxf,
las, mp4, ruststep, ply-rs, calamine, rust_xlsxwriter, riff, id3
```

The stdio tree contains 88 feature files with 176 `@mode-differential` tag occurrences. Representative features assert live oracle/subject agreement on a semantic projection; the expected oracle projection values are not embedded in those feature files.

A committed-file search excluding build targets found zero files named as projection JSON or oracle goldens:

```text
find ✏️s/🔌️plugins/🗄️stdio … \( -iname '*projection*.json' -o -iname '*oracle*.golden*' -o -iname '*oracle*.json' \)
0 files
```

The test platform confirms that projections are ephemeral runtime artifacts:

- TypeScript host `resultFor` writes `<scenario>.<role>.projection.json` beneath `plan.outputDir`.
- Rust runner writes the same projection filename beneath the plan output directory.
- `planExecution` resolves that output directory under `.semio` metadata's generated `⚡️cache/.../results` tree.

The large committed `🎯️outcome` and `📸️snapshot` fixture populations therefore do not constitute frozen oracle outputs. They preserve owned domain state and mutation contracts, not the complete third-party role result for every differential row.

The crate is also not uniform enough for a single mechanical replacement. Its manifest and feature notes distinguish full second producers from independent readers and composed format checks. Examples include paired XLSX reader/writer dependencies, reader-only STEP treatment, and RIFF/ID3 crates combined with owned format walkers. Retirement must preserve those capability distinctions per format.

## Safe retirement sequence

The whole crate can be retired after parity, but only through a per-capability wave:

1. Run every live differential and independent-reader case under a compiler lease with the current third-party oracles pinned.
2. Canonicalize each oracle projection and relevant byte/round-trip tripwire into committed language-neutral goldens, including declared comparison/lossiness profiles.
3. Add hostile inputs at every owned decoder, encoder, archive traversal, Unicode, numeric-boundary, and malformed-container seam.
4. Replace each live third-party call with an owned implementation or a narrow platform-boundary check that is independent from the subject path.
5. Prove the replacement against the frozen corpus before removing that format's feature and dependency ownership.
6. Remove the isolated oracle crate only after its registry is empty and the full differential corpus still passes without it.

This preserves the already-earned external differential evidence as a durable golden corpus while eliminating ongoing third-party execution. A wholesale deletion before step 2 is not safe.

## Scope

This was a read-only follow-up audit. No stdio oracle source, manifest, or lockfile was changed, and no Cargo, Nx, rustfmt, or modifying Git command was run.
