# 📓️ PNG 1.2/any — reader-based external oracle retrofit

Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any` only, mirroring the
`avi` reference instance (`riff-avi-1-0-mutate-reader`) verbatim. jpg/tiff/bmp are owned by parallel
sessions and were not touched.

## What was built (the five artefacts)

1. **`🏭️generator/🦀️png-codec/`** — standalone Cargo crate, own `[workspace]`, `png = "=0.18.1"` (the
   exact patch version already resolvable offline — confirmed present in both
   `~/.cargo/registry/src/*/png-0.18.1/` and the stdio oracle package's own committed
   `Cargo.lock`). `Cargo.lock` is committed. Two subcommands, `build <recipe-id> <out-dir>` and
   `project <path>`, plus `list-recipes`. `cargo test --offline` passes 3/3 unit tests. No mutation
   dispatch logic anywhere in this crate — every "after" state is a literal `png::Info` value this
   binary chose, handed to `png::Encoder`/`Writer`.
2. **`🏭️generator/📜️script.ts`** — `generate [--only <id>]` / `manifests [--only <id>]`, same CLI
   shape as avi's generator. Respects `SEMIO_FIXTURE_OUT`, sha256+byte-length per file, never
   rewrites a file after hashing it.
3. **`🔬️probes/📜️script.ts`** — `png-import` / `png-project` / `png-compare`. Only marshals to
   `png-codec project` and diffs structurally; computes nothing.
4. **`🧫️fixtures/<recipe>/{before,after}.png`** — 15 recipes, one per declared mutation kind, all
   `-applied` (this catalog has no `no-mutation` baseline and no `rejected` outcome, unlike avi).
5. **`🧪️oracle/🔣️.json`** — edited in place. `png-png-1-2-mutate` (the existing
   `cross-semio-implementation` entry backed by this subset's own `🧪️oracle/🦀️component.rs`) was
   **left untouched** per instruction. Added: a new oracle `png-png-1-2-mutate-reader`
   (`kind: third-party-library`), the `semantic-png-1-2-v1` comparisonProfile, the
   `png-1-2-crate-compare-v1` pipeline, three probe registrations, a `noOracleDecisions` entry for
   the uncarried set, and 15 `fixtureManifests` entries.

## Why a new comparisonProfile instead of the repo-wide `semantic-raster-v1`

`semantic-raster-v1` (defined in `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️.json`, shared across
bmp/tiff/jpg/gif/png) explicitly **ignores** `gamma` and `ancillaryChunks` by design — it exists for
byte-fidelity round-tripping, not for this subset's mutation vocabulary, which is made almost
entirely of the fields that profile deliberately canonicalizes away. `semantic-png-1-2-v1` is a new,
locally-defined profile (mirroring how `semantic-avi-v1` is local to avi's own oracle file) scoped to
header fields, palette, trns, gamma (exact scaled integer, never a lossy float), chromaticities,
srgb intent, physical dims, background, and ordered tEXt keyword/text pairs as real typed values, and
the decoded pixel sample buffer as size+digest (the same opaque-payload treatment avi gives a movi
chunk payload).

## Witnessable vs uncarried — 12 / 3, empirically confirmed, not just asserted

A peer session's research (`📓️png-reader-witnessability.md`, independently re-verified against the
vendored crate source while building this retrofit) predicted the split; building the actual codec
and projecting real before/after bytes **confirms it empirically**: all 12 witnessable kinds show a
projection diff between before/after, and all 3 uncarried kinds show byte-identical projections
despite genuinely different bytes on disk.

**Witnessable (12)** — `oracleRequirements` now carries `"oracle": "png-png-1-2-mutate-reader"`:
`change-header` (IHDR width/height/bitDepth/colorType/interlaced), `replace-palette` (`Info::palette`),
`change-transparency` (`Info::trns`), `change-gamma` (`Info::gama_chunk`, exact scaled u32),
`change-chromaticities` (`Info::chrm_chunk`), `change-srgb-intent` (`Info::srgb`),
`change-physical-dims` (`Info::pixel_dims`), `change-background` (`Info::bkgd` — the decoder reads it
even though the encoder has no setter for it at all; written via `Writer::write_chunk`'s raw escape
hatch), `insert-text-chunk`/`remove-text-chunk`/`replace-text-chunk` (`Info::uncompressed_latin1_text`),
`replace-pixels` (decoded pixel sample buffer, size+digest).

**Uncarried (3)** — `oracleRequirements`' capability renamed to `png-1-2-mutate-uncarried`, no
`oracle` field, matching the exact convention already used in the `cad` subset:
`change-timestamp` (`png::Info` 0.18.1 has no `tIME` field at all — confirmed absent from
`src/common.rs`), `insert-unknown-chunk`/`remove-unknown-chunk` (the decoder's own
`SkippedAncillaryChunk` path discards unrecognised ancillary chunks before they ever reach `Info`, no
public accessor exposes what was skipped). All three fixtures are still real, byte-different,
committed material — `png-codec` writes real tIME/unknown-chunk bytes via `Writer::write_chunk` for
both `before` and `after` — what is missing is only a public *read* path in this crate version.

## Reproducibility trap — handled at the write path, not by post-hoc canonicalization

`png::Encoder`/`Writer` never stamps wall-clock or process state into any chunk this crate writes by
default (confirmed by reading `encoder.rs` in full — `encode_header` and `write_image_data` are pure
functions of the `Info`/pixel bytes handed in). The one place PNG time data enters at all is the
`change-timestamp` recipe's raw `tIME` bytes, which are hand-chosen 7-byte payloads
(`0x07E8 01 01 00 00 00` / `0x07E8 06 0F 0C 1E 00`) written via `Writer::write_chunk` — never
`SystemTime::now()`. Verified directly: regenerated `change-timestamp-applied` into a separate output
directory and diffed against the committed fixture — byte-identical.

## Verify, don't assert — real output quoted

- **`fixture verify --artifact s.stdio.png --standard 1.2 --subset any`**:
  `[fixture verify] 15 fixture(s), 0 file problem(s)`
- **`fixture reproduce ... --mutation <id>`**, run individually for all 15 kinds (never a single
  whole-corpus double-run — reproducibility.md's own lesson): every one of the 15 invocations printed
  `[fixture reproduce] 1 generated fixture(s), 0 problem(s)`. 15/15, 0 problems.
- **Gate validated both ways**, real numbers from `png-compare`:
  - Known-good pair (`change-gamma-applied/before.png` vs itself): `equal: true, diffCount: 0`.
  - Known-bad pair (`change-gamma-applied/before.png` vs `after.png`, gAMA deliberately differs):
    `equal: false, diffCount: 1, diffs: ["$.gamma: 45455 ≠ 100000"]`.
- **`png-import`**: `bothImport: true` for every fixture checked.
- **`png-project`** on `replace-palette-applied/after.png`: `width:4, height:2, colorType:"indexed",
  hasPalette:true`, palette `[[255,255,0],[0,255,255],[255,0,255],[32,32,32]]` — the exact replaced
  values, verbatim.

## `contract` and `matrix` — run, not fabricated

`contract --artifact s.stdio.png --standard 1.2 --subset any` in practice validates the whole
repository registry (the `--artifact` flag scopes reporting emphasis, not the validation set). Ran it
twice, before and after fixing one real finding it surfaced:

- **One genuine, actionable breach it found**: `testing/dependency` — *"Oracle
  png-png-1-2-mutate-reader (png) is production-reachable and records no debt"*. Fixed by adding a
  `productionDebt` block (`reachableFrom` naming both the OS host's SVG-rasterization path and
  remodel's image-editing engine, `owner`, `plan`) — the same requirement the pre-existing
  `png-png-1-2-mutate` entry already satisfies. Re-ran `contract`: the breach is gone.
- **Everything else PNG-related in the output is pre-existing and out of this ticket's scope**,
  confirmed by cross-checking the identical pattern already present for `avi` (this session's own
  reference instance) in the same run:
  - `reimplementationOracleBreaches` flags `png-png-1-2-mutate-reader is registered as a qualifying
    third-party oracle, but this owner predicts mutation output in its own Rust` — the SAME audit
    flags `riff-avi-1-0-mutate-reader` for the identical reason (a qualifying reader coexists with a
    computing `component.rs` in the same directory). This is a standing, repo-wide, by-design
    consequence of the two-oracle shape this whole ticket batch is building — not a defect.
  - `"mutation <id> requires a third-party-library for capability png-1-2-mutate-uncarried, and none
    is registered"` for the 3 uncarried kinds — the exact, honest, by-design reporting of an
    unfulfilled requirement (116 occurrences repo-wide across `sequence`/`mathematical`/others using
    the identical `-uncarried` convention).
  - `"No-oracle decision png-tIME-and-unknown-ancillary-chunks-uncarried claims mutation capability..."`
    — an unconditional informational echo of every `noOracleDecisions` entry repo-wide (5 other
    owners get the identical line).
  - `"Mutation <id> is owned by 'any' and s.stdio.png@1.2 declares no narrower subset at all"` (all
    15 kinds) and `"No runtime inventory has been produced for s.stdio.png@1.2/any"` — both fire
    identically for `avi`; structural/subset-architecture and the repo-wide `semio-s-plugin-stdio`
    build blocker (documented in `📓️build-unblock.md`), neither caused by or fixable from this
    ticket.
  - `png` export stub-serializer findings across ~18 OTHER plugins (lowpoly, draw, puzzle, layout,
    shooting, sourcing, gis, procedural, animate, reasoning, dag, block, cad, process, raster,
    remodel, trinity) — pre-existing, tracked under the ticket's own "163 serializers" finding
    (`📓️oracle-research-findings.md`), not this subset's own carrier and not touched here.
  - The pre-existing, untouched `png-png-1-2-mutate` entry's own capability-mismatch report against
    `mutate-png-1-2/🥒️.feature` — present before this session, unrelated to the reader retrofit.

`matrix --artifact s.stdio.png --standard 1.2 --subset any` (repo-wide numbers, since matrix reports
globally): `"Which mutations have no external oracle?"` names exactly
`s.stdio.png::change-timestamp, s.stdio.png::insert-unknown-chunk, s.stdio.png::remove-unknown-chunk`
— confirming the other 12 are now externally oracled and the 3 uncarried ones are honestly reported,
nothing more and nothing less.

## Production reachability — measured, not copied

The sibling `png-png-1-2-mutate` entry already asserted `productionReachable: true`. Verified
independently for the new reader oracle rather than copied:

- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:3008` — `use png::{BitDepth, ColorType,
  Encoder};`, used by `rasterize_svg_to_png_base64`, registered as a real OS media-export handler
  (`register_os_media_export_handler_kind(artifact_kind, "png", ...)`).
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖼️images/🦀️component.rs`
  — real (non-test) `png::Encoder`/`png::Decoder` use in remodel's own image-editing engine (lines
  171–464, 1308–1354; only lines 971–977 are inside `#[cfg(test)]`).
- All three of `os/host`, `remodel` and `lowpoly`'s own `Cargo.toml` declare `png = "0.17.16"` under
  `[dependencies]` — real, non-optional, non-dev.
- `🔒️dependencies.json`'s repo-root `png` entry (`productionReachable: false`, version `0.17.16`) is
  **stale** against this source evidence and against the sibling oracle's own pre-existing
  `productionReachable: true` claim. Not corrected here — it's a shared, generated, repo-root ledger
  and out of this ticket's PNG-subset-only scope — but the discrepancy is recorded in the new oracle
  entry's own rationale so it isn't mistaken for agreement.

## What could and could not be verified

`semio-s-plugin-stdio` does not compile repo-wide (pre-existing, unrelated peer migration, per this
ticket's own `📓️build-unblock.md`) — per instruction, no attempt was made to fix or build it.
Everything above was verified by shelling out to the standalone `png-codec` crate (`cargo
build`/`test`/`run --offline` all pass) and by the repository's own `fixture verify` / `fixture
reproduce` / `contract` / `matrix` commands, which read the JSON registry and fixtures directly and
do not require the plugin crate to build.

## Numeric summary

| | |
|---|---|
| Fixtures | 15/15 generated, verified, reproducible |
| `fixture verify` | 15 fixture(s), 0 problems |
| `fixture reproduce` (per-mutation, 15 runs) | 15/15 passed, 0 problems each |
| Gate accept (known-good pair) | `equal:true, diffCount:0` |
| Gate reject (known-bad pair) | `equal:false, diffCount:1`, exact field named |
| Witnessable kinds | 12/15 — oracle-backed |
| Uncarried kinds | 3/15 — `png-1-2-mutate-uncarried`, honestly unfulfilled |
| New `contract` breach found and fixed | 1 (`productionDebt` missing) |
| Pre-existing/out-of-scope `contract` findings touching PNG | confirmed pattern-identical to `avi`, not new |
