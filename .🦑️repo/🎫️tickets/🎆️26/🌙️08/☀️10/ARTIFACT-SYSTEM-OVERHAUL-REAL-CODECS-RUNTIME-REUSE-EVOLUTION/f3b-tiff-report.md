# F3B — tiff (standard 6.0) schema overhaul report

## Summary

`TiffSnapshot` was rebuilt from the shared `RasterImage{width,height,rgba}` stub into TIFF's real
generic tag/type/value model: `byte_order: TiffByteOrder`, index-keyed `ifds: Vec<TiffIfd>`, each
holding tag-id-keyed `entries: Vec<TiffTag{tag: u16, kind: TiffFieldType, values: TiffValues}}`
covering all 12 TIFF 6.0 core field types, plus decoded `pixels: Vec<u8>`. `TiffDiff` is a
handcrafted sparse struct (index-keyed triple for `ifds`, tag-id-keyed triple for each IFD's
`entries` — no full-replace slot anywhere). `TiffMutation` gained the full target verb set
(`SetByteOrder`, `InsertIfd`/`RemoveIfd`, `SetTag`/`RemoveTag`, `SetPixels`, plus
`NoMutation`/`SetSnapshot`), every variant's `diff()` handcrafted (no apply-and-capture), every
`inverse()` handcrafted. `impl DiffAlgebra<TiffSnapshot> for TiffDiff` (`inverse`/`between`/
`is_empty`) added standalone. The engine's decode now walks the WHOLE `next IFD offset` chain
generically for every field type (not just the first IFD, not just the tags the codec
specially interprets); encode stays honestly narrower (single-IFD, canonicalized
strip/geometry tags + carried-over extra tags) — documented as `EncodeScopeNote`, matching this
same wave's PNG precedent. `byte_order` now genuinely round-trips through encode (the
pre-migration engine always emitted little-endian).

The `✳️baseline` subset (added mid-plan by the sibling `26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-
VOCABULARIES` ticket while this was scoped as "no IFD, schema-gapped") was updated to match: its
analyzer now implements REAL Adobe TIFF 6.0 Part 1 Baseline conformance checks (Compression/
PhotometricInterpretation/BitsPerSample/strip-vs-tile) against the now-real IFD, closing the gap
its own doc comment had explicitly named as the upgrade path. All findings stay SOFT (warnings)
by policy — `build()`/`compose()` never hard-fail on conformance grounds, unchanged from before.

## Deviations from the literal ticket brief (and why)

- **No top-level `width`/`height` scalar fields on `TiffSnapshot`.** The brief's completeness
  table lists exactly `byte_order`/`ifds`/`pixels` — width/height are genuinely IFD tags (256/257)
  per spec, not separate normative state, so duplicating them as scalars would violate "complete
  per FORMAT SPEC, not per codec capability" (a duplicate source of truth). Added `TiffSnapshot::
  width()`/`height()` convenience accessors instead (read IFD 0's tags).
- **Encode is single-IFD only** (documented `EncodeScopeNote`), even though decode walks the whole
  IFD chain. Mirrors this exact ticket's own PNG precedent (chunk-order/header canonicalization on
  encode) and keeps scope bounded per the brief's own "preserve whatever real strip/tile decode
  logic already exists in the engine — restructure the schema layer around it" instruction. Any
  IFD beyond index 0, and any of IFD 0's own geometry/compression/photometric tags, are recomputed
  fresh on encode; every OTHER tag on IFD 0 (e.g. `Artist`, `ResolutionUnit`) is carried over
  verbatim — round-trip-tested (`carried_ascii_tag_round_trips`, `carried_short_tag_round_trips`).
- **`TiffValues` is adjacently tagged** (`#[serde(tag = "kind", content = "value")]`), not
  internally tagged as originally drafted — internally-tagged enums require struct-shaped variant
  content, and these are newtype variants wrapping arrays/strings. Caught before compiling by
  checking `ply`'s already-real `PlyValue`, which hit the identical shape and already solved it
  this way.
- **Baseline subset upgraded beyond "keep it compiling."** Its `check_tiff_baseline_conformance`
  used to be permanently soft-and-vacuous ("no IFD, cannot check") by construction. Since this
  wave's own snapshot change is exactly what that function's doc named as the prerequisite, I
  implemented the real checks rather than leave a now-provably-false "cannot be checked" claim in
  place (would have violated the repo's anti-fabrication discipline). Scope stayed conservative:
  real per-field checks, all still SOFT severity (no new hard-gating behavior), so `build()`'s
  "never fails" contract is unchanged — only the *quality* of the diagnostics changed.

## glue_followup (script.ts allowlist entries now stale — I do not touch script.ts)

Three S-8 allowlists (all shrink-only, per the plan) carry `tiff` entries seeded when this
artifact still had generic/stub diffs. All three are now genuinely stale and should be pruned by
the wave closer:

- `POLICY_DIFF_ALGEBRA_ALLOWLIST` (📜️script.ts ~line 8546): remove
  `"stdio/tiff/standards#6.0-subsets-any-schema-diff-component"` — `TiffDiff` now implements
  `DiffAlgebra`.
- `POLICY_FIELD_SWEEP_ALLOWLIST` (📜️script.ts ~line 8604): remove
  `"stdio/tiff/standards#6.0"` — a real `field_sweep_covers_every_mutable_field` test now exists.
- `POLICY_FACET_MIRROR_DRIFT_ALLOWLIST` (📜️script.ts ~line 9207): remove all three —
  `"stdio/tiff/standards#6.0-subsets-any-schema-diff-component"`,
  `"...-mutations-component"`, `"...-snapshot-component"` — every facet mirror (ts/graphql/json/
  proto) was handcrafted field-for-field against the new Rust shape this wave.

`POLICY_GRAMMAR_HONESTY_ALLOWLIST` never had tiff entries and needs no change — verified zero
occurrences of any of the 7 placeholder markers (`size-eos: true`, `payload = *OCTET`, etc.)
remain anywhere under `🗿️artifacts/🖼️tiff/`.

No new top-level directories were needed; all real work landed inside files already mounted in
glue.rs per S2's resolved shape (no glue.rs edit required).

## Verification

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::tiff"` → **29 passed, 0 failed** (includes
  6 engine-level codec tests + all 6 law suites + baseline-subset tests).
- Whole-crate gate: `cargo test -p semio-s-plugin-stdio --lib` → **883 passed, 0 failed** (up from
  the 853/0 pre-wave baseline; the delta includes both my +29 and the concurrently-landing jpg
  sibling agent's own additions). One transient whole-crate compile failure was hit and resolved
  by waiting out concurrent churn: the jpg sibling agent's own in-progress edit (confirmed via
  `git status`/`git diff` — real WIP in `🗿️artifacts/📷️jpg/...`, not caused by anything I touched)
  briefly broke the shared crate; polled `cargo check` every 45s until it cleared rather than
  touching jpg's files.
- Grep gates: `snapshot: Option<` in the diff file → 2 matches, both inside doc-comment prose
  describing what was REMOVED (`` `TiffDiff{snapshot: Option<TiffSnapshot>}` full-replace
  template `` / `` No `snapshot: Option<TiffSnapshot>` full-replace slot `` — identical to PNG's
  own already-verified diff file's phrasing) — zero real struct fields. `impl DiffAlgebra` present.
  Zero apply-and-capture shaped bodies. Zero `RasterImage` type left (only a doc-comment mention
  of the type it replaced).
- `bun ./📜️script.ts policy`: ran; output is dominated by ~22k pre-existing unrelated
  `os-state-authority`/`budget` breaches repo-wide (not this program's scope, confirmed via S2's
  own notes). Verified via direct source inspection (see glue_followup) rather than the CLI's
  truncated/priority-filtered output, since `stdio-artifacts/*` kind breaches are `priority: "low"`
  and didn't surface in the ~100-line human-readable dump; the three now-stale allowlist entries
  are recorded above for the closer.

## Files touched (48)

**Rust logic** (11): `⚙️engine/🦀️component.rs` (full rewrite — generic multi-IFD decode, typed
tag/value codec, single-IFD encode); `🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` (new
`TiffByteOrder`/`TiffFieldType`/`TiffValues`/`TiffTag`/`TiffIfd`/`TiffSnapshot`, kills
`RasterImage`); `.../🔺️diff/🦀️component.rs` (new `TiffDiff` + `TiffTagsDiff`/`TiffIfdsDiff` +
`DiffAlgebra` impl + mutation-diff builders); `.../🧬️mutations/🦀️component.rs` (new
`TiffMutation` enum, 8 variants, handcrafted diff/inverse, full law-suite test module);
`.../🧬️schema/🦀️component.rs` (`TiffArtifact` mirrors the new snapshot shape);
`.../🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` (signature fix for the new 2-arg
`diff_set_snapshot`); `✳️baseline/🧬️schema/🦀️component.rs` (stale doc comment fix);
`✳️baseline/🧐️analyzer/🦀️component.rs` (real conformance checks, full rewrite);
`✳️baseline/🏗️builder/🦀️component.rs` + `✳️baseline/🎹️composer/🦀️component.rs` (doc + test
fixture fixes for the new snapshot shape); `🪆️subsets/🔣️component.json` (stale taxonomy
description fix).

**Facet mirrors** (16): `🟦️component.ts`/`🔗️component.graphql`/`🔣️component.json`/
`🛰️component.proto` at the artifact (`🧬️schema/`), snapshot, diff, and mutations levels — all
four facets rewritten from the stale `TiffEntry{name,data}` stub to the real field-for-field
shape.

**Grammar leaves** (21): `📝️text/{🅰️component.g4, 🔤️component.ebnf, 📖️component.grammar.semio}`
+ `💾️binary/{🥋️component.ksy, 🌶️component.spicy, 🔠️component.abnf,
📡️component.protocol.semio}` under each of snapshot (real byte-order/IFD-chain/generic-tag
binary grammar; hex-dump text grammar), diff, and mutations (both JSON-shape text grammars +
raw-JSON-bytes binary grammars, matching PNG's proven pattern for non-envelope op payloads).

## Report path

This file: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f3b-tiff-report.md`
