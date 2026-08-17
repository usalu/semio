# F6 — 🖼️bmp (standard v3): OpText/OpBinary + DiffCodec

## Summary

Both sides — `BmpDiff` (`protocol::DiffCodec`) and `BmpMutation` (`protocol::OpText`/
`protocol::OpBinary`) — landed on the **DERIVE** path, exactly as `f6-recon-report.md` §8 row 14
guessed ("Flat header + palette + rows"). This was verified for real, not assumed: added
`#[derive(dsl::DslDiff)]` to `BmpDiff` and `#[derive(dsl::DslOps)]` to `BmpMutation` directly, ran
`cargo check -p semio-s-plugin-stdio --lib`, and got zero new errors from either derive attempt —
no compiler-error triage/back-out was needed on either side.

## STEP 1 — classification (verified for real)

**Diff side**: `BmpDiff`'s fields are all single-layer `Option<T>` (`header_size`, `width`, …,
`palette: Option<BmpPaletteDiff>`, `pixels: Option<Vec<u8>>`) — never `Option<Option<T>>` anywhere
in the struct or its nested types (`BmpPaletteDiff`, `BmpPaletteModified`, `BmpPaletteAdded`,
`BmpPaletteEntry`). No data-carrying enum is reachable — the only enum in the whole tree,
`BmpRowOrder`, is unit-variant-only (`BottomUp`/`TopDown`). Neither §3a nor §3b's blocker applies.
`#[derive(dsl::DslDiff)]` added directly to `BmpDiff` → clean compile.

**Mutation side**: `BmpMutation`'s only struct-valued payloads are `BmpSnapshot` (via
`SetSnapshot`) and `BmpPaletteEntry` (via `InsertPaletteEntry`/`SetPaletteEntry`) — both flat, no
enum anywhere in `BmpSnapshot`'s own tree beyond the same unit-variant `BmpRowOrder`. No variant
carries a tri-state `Option<Option<T>>` (mutation `Option<T>` args are always "the new value").
`#[derive(dsl::DslOps)]` added directly to `BmpMutation` → clean compile.

Both verdicts match the recon table's "DERIVE (probable)" guess for bmp v3, now confirmed rather
than assumed.

## STEP 2a — derive path work

Cascading `#[derive(dsl::DslRecord)]`/`#[derive(dsl::DslScalar)]` additions needed to make the two
top-level derives compile (all in the same 3 files already owned by this artifact):

- `📸️snapshot/component.rs`:
  - `BmpRowOrder` → `dsl::DslScalar` (unit-variant enum → `DslField` directly, no `DslVariants`
    needed).
  - `BmpPaletteEntry` → `dsl::DslRecord`.
  - `BmpSnapshot` → `dsl::DslRecord`, plus `#[dsl(block)]` on `palette`/`pixels` (struct/byte
    payload readability, matches `SpaceMutation`/gif89a precedent) and `#[dsl(base64)]` on the
    bare `Vec<u8>` `pixels` field for a compact grammar (works here because `pixels` on the
    *snapshot* is bare `Vec<u8>`, not `Option<Vec<u8>>` — the base64-through-Option quirk from
    §3 doesn't apply to this field).
- `🔺️diff/component.rs`:
  - `BmpPaletteModified`, `BmpPaletteAdded`, `BmpPaletteDiff` → `dsl::DslRecord` each (the
    collection-triple's own container types; bare `Vec<T>` fields bind via the `dsl` crate's
    blanket `Vec<T>: DslField` impl).
  - `BmpDiff` → `dsl::DslDiff` (this is the whole ask for the Diff side — `protocol::DiffCodec` is
    now fully generated, no hand-written impl exists or is needed).
  - `#[dsl(block)]` added to `BmpDiff::palette: Option<BmpPaletteDiff>` for readability. **Did
    NOT** add `#[dsl(base64)]` to `BmpDiff::pixels: Option<Vec<u8>>` — per the recon report's
    documented derive quirk (§3, "Known derive quirk found in passing"), `classify_field` peels
    the `Option` layer before checking the `base64` attribute, so it would compile but silently do
    nothing (fall back to a verbose bracketed decimal-byte-list `Shape::List(UInt)`). Left
    undecorated rather than adding a misleading attribute; documented in a doc comment on
    `BmpDiff` itself. This is a real, accepted token-inefficiency trade-off (not a bug), matching
    the recon report's own guidance not to hand-roll solely to work around this one quirk.
- `🧬️mutations/component.rs`:
  - `BmpMutation` → `dsl::DslOps`.
  - `#[dsl(block)]` on `SetSnapshot::snapshot`, `InsertPaletteEntry::entry`,
    `SetPaletteEntry::entry` (struct payloads).
  - `#[dsl(base64)]` on `SetPixelData::pixels` (bare `Vec<u8>` mutation argument — works cleanly,
    no `Option` wrapper on this one).
  - Replaced the prior `serde_json`-based `OpText`/`OpBinary` stub impls with the §2 handcrafted
    wrapper (mandatory even on full derive success — P6 means `DslOps` emits `DslVariants` only,
    never `OpText`/`OpBinary`). Wrapper body copied verbatim from `f6-recon-report.md` §2 /
    `FlowMutationDsl`/`SpaceMutation`/gif89a precedent — `OpText::parse_op`/`print_op` via
    `dsl::DslVariants::variants()`/`to_named_record`/`from_named_record` + `dsl::parse`/`print`;
    `OpBinary::encode_op`/`decode_op` as a pure forward to `dsl::variants_binary`.

No hand-rolled grammar helpers (`hex_encode`/`split_top_level`/etc.) were needed anywhere in this
artifact — both sides are fully derived.

## STEP 3 — tests added

- `🔺️diff/component.rs`: new `#[cfg(test)] mod tests` (diff/component.rs had none before) with
  `diff_codec_text_binary_roundtrip_law` — exercises every scalar field (`header_size` through
  `colors_important`, `row_order`, `pixels`) plus all three sections of the `palette` collection
  triple via a real `BmpDiff::between()` result over an intentionally asymmetric pair (`a`: 2
  palette entries, `b`: 3 — index 0 stable, index 1 modified, index 2 new), covering `modified`+
  `added` in one direction and `removed`+`modified` in the other (same "asymmetric-length fixture"
  design already used by this file's own `field_sweep_covers_every_mutable_field` mutation test).
  Asserts `!printed.contains('\n')`, `parse_diff(print_diff(x)) == x`,
  `decode_diff(encode_diff(x)) == x` for `BmpDiff::default()` plus both `between()` directions.
- `🧬️mutations/component.rs`: new `op_text_binary_roundtrip_law` in the existing `tests` module,
  covering every one of the 7 `BmpMutation` variants (incl. `SetSnapshot`'s whole-nested-struct
  payload, `InsertPaletteEntry`/`SetPaletteEntry`'s struct payload, `SetPixelData`'s
  base64-compact `Vec<u8>` payload, and `SetHeaderFields`' partial-field-set shape). Same 3
  assertions per variant as the diff-side law test.

## STEP 4 — verification (real, this session)

- `cargo check -p semio-s-plugin-stdio --lib` — clean after the derive additions (no bmp-related
  errors at any point; two rounds of transient errors observed were entirely in `las`/`gif87a`
  files under concurrent edit by other sibling F6 sessions in this same wave, confirmed by
  `git status` showing those files as live-modified/untracked by another session, not touched by
  this one — resolved on its own by the time of the second `cargo check`, consistent with this
  repo's known "concurrent cargo workspace churn" pattern).
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::bmp"` → **16/16 passed** (10 pre-existing +
  6 pre-existing-but-newly-counted... — precisely: the pre-existing `codec_round_trip`,
  `codec_retention_law`, `row_bytes_padding_is_exact`, `bitfields_16bit_555_round_trip`,
  `gradient_checkerboard_24bit_round_trip`, `indexed_4bit_palette_round_trip`,
  `sniff_rejects_non_bmp_bytes`, `empty_snapshot_matches_schema` (engine tests), `demo_source_
  nonempty` (examples), `mutation_diff_law`, `inverse_law`, `absorb_law`, `between_roundtrip_law`,
  `field_sweep_covers_every_mutable_field` (5 pre-existing mutation-module law tests) = 14
  pre-existing, **+ 2 new** (`op_text_binary_roundtrip_law`, `diff_codec_text_binary_roundtrip_law`)
  = 16 total, 0 failed.
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1042 passed, 2 failed** (re-run
  three times over ~10 minutes via a polling monitor to let concurrent sibling sessions settle;
  count held steady at 1042/2 throughout, so this is a stable reading, not a transient glitch). The
  2 failures (`artifacts::stl::standards::v_ascii::…::op_text_binary_roundtrip_law` and
  `…::diff_codec_text_binary_roundtrip_law`) are **not in this artifact's ownership boundary** —
  confirmed via `git status` that `🟪️stl`'s files are currently modified (uncommitted, 35 files) by
  another live F6 sibling session doing the identical OpText/OpBinary+DiffCodec work on the `stl`
  artifact concurrently; the failure is in *their* new grammar (a `parse_op`/`parse_diff` "tuple
  expects 3 elements, found 9" mismatch), not anything this session touched or can fix without
  crossing into another agent's ownership boundary (`✏️s/…/🗿️artifacts/🟪️stl/**`). The baseline
  this session started from (per `f6-recon-report.md` §11) was **1019 passed, 0 failed**; this
  session's own bmp work only ever added passing tests (1019 → 1042 net, +23, of which 2 are this
  artifact's new law tests — the remaining +21 are other sibling F6 sessions' concurrent progress
  landing in the same live tree between the recon snapshot and now) and never removed or broke any
  pre-existing passing test. The 2 failures are `stl`'s own in-flight work-in-progress, unrelated to
  and unaffected by anything in this report.

## Deviations from §5/§9's conventions

None on the grammar side — both sides fully derived, no hand-rolled grammar was written.

One documented, deliberate omission: `BmpDiff::pixels: Option<Vec<u8>>` does NOT carry
`#[dsl(base64)]` (see STEP 2a above) — adding it would compile but do nothing per the recon
report's own documented quirk; a doc comment on `BmpDiff` explains why it's absent rather than
silently missing.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
  — `dsl::DslScalar` on `BmpRowOrder`, `dsl::DslRecord` on `BmpPaletteEntry`/`BmpSnapshot`,
  `#[dsl(block)]`/`#[dsl(base64)]` attributes.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — `dsl::DslRecord` on `BmpPaletteModified`/`BmpPaletteAdded`/`BmpPaletteDiff`, `dsl::DslDiff` on
  `BmpDiff` (fully derived `protocol::DiffCodec`, no hand-written impl), `#[dsl(block)]` attribute,
  new `#[cfg(test)] mod tests` with `diff_codec_text_binary_roundtrip_law`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — `dsl::DslOps` on `BmpMutation`, `#[dsl(block)]`/`#[dsl(base64)]` attributes, handcrafted
  `OpText`/`OpBinary` impls replacing the prior `serde_json` stubs, new
  `op_text_binary_roundtrip_law` test in the existing `tests` module.
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-bmp-check1.txt`, `f6-bmp-check2.txt`,
  `f6-bmp-scoped-test.txt`, `f6-bmp-scoped-test2.txt`, `f6-bmp-full-crate-test.txt`,
  `f6-bmp-full-crate-test-final.txt`.

**No shared files touched**: `glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema` framework
crates were all read-only for this session. `POLICY_DIFF_COMPLETENESS_ALLOWLIST` in `📜️script.ts`
was NOT edited (per instructions — the goal is for `bmp`'s diff file to stop being flagged by
having a real `DiffCodec` impl, which it now does, not to allowlist around it).
