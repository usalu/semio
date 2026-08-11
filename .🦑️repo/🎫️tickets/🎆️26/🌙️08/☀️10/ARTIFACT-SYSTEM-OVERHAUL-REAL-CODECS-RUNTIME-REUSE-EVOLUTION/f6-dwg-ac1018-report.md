# F6 — 🖊️dwg (ac1018) — OpText/OpBinary + DiffCodec

**Scope**: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/**` only. ac1024 (sibling
standard) untouched — verified by `git status` before and after: no file under `🔖️ac1024/` appears
in my diff.

## Step 1 — classification (verified for real, not trusted from the table)

The §8 table listed ac1018 as "DERIVE (probable)". Read the actual files (not just the report
summary) before touching anything:

- `📸️snapshot/🦀️component.rs` — `DwgSnapshot { schema: String, version: String,
  maintenance_version: u8, codepage: u16, bytes: Vec<u8>, section_names: Vec<String> }`. Zero
  enums, zero nested structs beyond primitives/`Vec`.
- `🔺️diff/🦀️component.rs` — `DwgDiff` (already hand-authored in a prior wave, F5) is a flat struct
  of `version: Option<String>`, `maintenance_version: Option<u8>`, `codepage: Option<u16>`,
  `bytes: Option<Vec<u8>>`, `section_names: Option<Vec<String>>`. Every field is a **single-level**
  `Option<T>` — never `Option<Option<T>>` (tri-state) — because ac1018 never went through the
  richer per-field-nullable snapshot design ac1024 uses; every field here is always-present at the
  snapshot level, so the diff only ever needs "changed or not", not "removed vs unchanged vs set".
  This means neither of the recon's two derive-blockers (§3a enum-in-tree, §3b tri-state) applies.
- `🧬️mutations/🦀️component.rs` — `DwgMutation` variants: `NoMutation`, `SetSnapshot{snapshot:
  DwgSnapshot}`, `SetVersionInfo{version, maintenance_version, codepage}`,
  `InsertSectionName{index, name}`, `RemoveSectionName{name}`. `SetSnapshot`'s payload is the same
  enum-free `DwgSnapshot` above, so the Mutation-side walk (per §3's rule: "does the Snapshot type
  recursively contain a data-carrying enum anywhere?") also comes back clean.

**Verdict, both sides: DERIVE.** Confirmed by actually adding the derives and running `cargo check`
— zero `DslField is not implemented for X` errors, zero tri-state errors, on the first attempt.
ac1018 is the second artifact after 💾️binary (the recon's own §4 pilot) to land cleanly on the
derive path for both Diff and Mutation — matches the recon's prediction exactly (it had already
flagged `dwg`/`ac1018` "Frozen/opaque-by-spec boundary per F5, likely genuinely flat").

## Step 2a — DERIVE path applied

**Snapshot** (`📸️snapshot/🦀️component.rs`): added `dsl::DslRecord` to `DwgSnapshot`'s derive list.
Added `#[dsl(base64)]` to the bare `pub bytes: Vec<u8>` field (this one DOES work — the recon's
"quirk" only breaks `Option<Vec<u8>>`, and this field is un-wrapped at the snapshot level).

**Diff** (`🔺️diff/🦀️component.rs`): added `dsl::DslDiff` to `DwgDiff`'s derive list. This is the
ENTIRE change needed to satisfy `protocol::DiffCodec` — no hand-written `print_diff`/`parse_diff`/
`encode_diff`/`decode_diff` at all, exactly like `BinaryDiff` in the recon's §4 pilot. Deliberately
did **not** add `#[dsl(base64)]` to `bytes: Option<Vec<u8>>` (the diff-side bytes field) — per the
recon's documented derive quirk, `classify_field` peels the `Option` unconditionally before ever
checking `attrs.base64`, so the attribute would silently no-op (falls back to a verbose
`Shape::List(UInt)` decimal-byte-list grammar) rather than actually compact it. Left it off with a
doc comment citing the quirk, rather than leaving a misleading attribute on the field.

**Mutation** (`🧬️mutations/🦀️component.rs`): added `dsl::DslOps` to `DwgMutation`'s derive list
(emits `dsl::DslVariants` only, per P6 — see recon §0/§1). Added `#[dsl(block)]` to the
`SetSnapshot{snapshot: DwgSnapshot}` field for readability parity with `BinaryMutation`/`GifMutation`.
Replaced the prior `serde_json`-based `OpText`/`OpBinary` stub impls with the recon §2 handcrafted
wrapper (copied verbatim from `BinaryMutation`'s real, tested impl — identical shape, only the type
name changes): `OpText::parse_op`/`print_op` walk `DslVariants::variants()` and call
`dsl::parse`/`dsl::print`; `OpBinary::encode_op`/`decode_op` forward straight to
`dsl::variants_binary::encode_op`/`decode_op`.

## Step 3 — tests added (both new, both real)

- `diff_codec_text_binary_roundtrip_law` (in `🔺️diff/🦀️component.rs`'s own new `#[cfg(test)] mod
  tests`, which didn't exist before — this file had zero tests prior to F6). 7 cases: the default
  (fully-empty) diff, one case per field in isolation (`version`, `maintenance_version`+`codepage`
  together, `bytes`, non-empty `section_names`, and the edge case of an explicitly-empty
  `Some(vec![])` for `section_names` to distinguish "set to empty" from "unchanged"), plus one case
  with every field changed at once. Each asserts `!printed.contains('\n')`,
  `parse_diff(print_diff(d)) == d`, `decode_diff(encode_diff(d)) == d`.
- `op_text_binary_roundtrip_law` (added to the existing `🧬️mutations/🦀️component.rs` test module,
  alongside the pre-existing `mutation_diff_law`/`inverse_law`/`absorb_law`/etc.). Covers all 5
  variants, including `SetSnapshot`'s nested-record payload. Same three assertions per case as
  above, applied to `print_op`/`parse_op`/`encode_op`/`decode_op`.

Needed one supporting import: `#[cfg(test)] use protocol::{OpBinary, OpText};` at the top of
`🧬️mutations/🦀️component.rs` (the impls themselves use the fully-qualified `protocol::OpText`/
`protocol::OpBinary` paths so they don't need the import, but the test module's method-call syntax
does).

## Step 4 — verification (real, this session)

- `cargo check -p semio-s-plugin-stdio --lib` — first attempt hit 8 compile errors, **all in the
  unrelated `☁️las` artifact** (`LasVlr`/`LasPoint`/`Option<f64>`/`Option<(u16,u16,u16)>: DslField
  is not satisfied`), zero errors mentioning `dwg`. `git status` confirmed `☁️las` files were
  concurrently modified (uncommitted) by another live session in this same wave — the recon report
  itself flags `las` as "the sweep MISSED... entirely, it's not even in the table", consistent with
  a sibling agent working it right now. Polled `cargo check` every 60s (via the `Monitor` tool,
  non-blocking) rather than editing around it; it went clean on the very next poll (~60s later) once
  that other session's in-progress edit finished. Full transcript: `f6-dwg-ac1018-check1.txt`.
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::dwg::standards::v_ac1018"` (confirmed exact
  module path first via the compile output) → **12/12 passed**, including both new tests
  (`diff_codec_text_binary_roundtrip_law`, `op_text_binary_roundtrip_law`). Full output:
  `f6-dwg-ac1018-test-scoped.txt`.
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1039 passed, 3 failed**. All 3
  failures are in `🖼️bmp`/`🟪️stl` — artifacts outside my ownership boundary, entirely unrelated to
  `dwg`:
  - `artifacts::bmp::standards::v_v3::...::diff_codec_text_binary_roundtrip_law`
  - `artifacts::stl::standards::v_ascii::...::diff_codec_text_binary_roundtrip_law`
  - `artifacts::stl::standards::v_ascii::...::op_text_binary_roundtrip_law`

  `git status` confirms both `🖼️bmp/🏅️standards/🔖️v3/**` and `🟪️stl/🏅️standards/🔖️ascii/**` are
  currently modified (uncommitted) by other live sessions — these are sibling F6 agents' in-progress
  hand-roll/derive attempts for bmp and stl, mid-edit at the moment this run captured them (the
  stl failures are literal parse-grammar mismatches — `tuple expects 3 elements, found 9` — the
  signature of an in-progress, not-yet-working hand-rolled grammar, not a regression I introduced).
  Zero failures anywhere in `dwg`. Full output: `f6-dwg-ac1018-full-crate-test.txt`.

## Deviations from the recon's §5/§9 template

None on substance — ac1018 landed cleanly on the DERIVE path predicted by the recon's own
classification table, so §5's hand-roll grammar template was not needed at all; §2's handcrafted
OpText/OpBinary wrapper was copied verbatim from `BinaryMutation` (byte-for-byte the same shape,
only the enum type name differs) per the recon's own claim that this wrapper is "100% boilerplate,
identical in every real usage". The only judgment call: omitting `#[dsl(base64)]` on `DwgDiff`'s
`bytes: Option<Vec<u8>>` field rather than adding it as inert boilerplate, since the recon
explicitly documents it as a no-op for `Option`-wrapped `Vec<u8>` fields — documented in a doc
comment instead so a future reader doesn't wonder why it's "missing".

## Files touched (real, live, not reverted)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
  — `dsl::DslRecord` added to `DwgSnapshot`, `#[dsl(base64)]` added to `bytes`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — `dsl::DslDiff` added to `DwgDiff` (fully derived `protocol::DiffCodec`, no hand-written impl),
  new `#[cfg(test)] mod tests` with `diff_codec_text_binary_roundtrip_law`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — `dsl::DslOps` added to `DwgMutation`, `#[dsl(block)]` on `SetSnapshot::snapshot`, handcrafted
  `OpText`/`OpBinary` replacing the `serde_json` stubs, `#[cfg(test)] use protocol::{OpBinary,
  OpText};` import, new `op_text_binary_roundtrip_law` test added to the existing test module.
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-dwg-ac1018-check1.txt`,
  `f6-dwg-ac1018-test-scoped.txt`, `f6-dwg-ac1018-full-crate-test.txt`.

**No shared files touched**: `📦️glue.rs`, `📜️script.ts`, `dsl`/`protocol`/`schema` framework crates,
`POLICY_DIFF_COMPLETENESS_ALLOWLIST` all untouched (verified: the whole point of this change is that
`🔺️diff/🦀️component.rs` now contains the literal strings `dsl::DslDiff` and `DiffCodec` in its own
file text, so the live `bun ./📜️script.ts policy` `dsl-migration/diff-completeness` check drops this
file out of its breach list on its own — no allowlist edit needed or made). ac1024 (sibling
standard, different agent's ownership) untouched — confirmed via `git status`, zero files under
`🔖️ac1024/` in my diff.

## Verification summary

| Check | Result |
|---|---|
| `cargo check -p semio-s-plugin-stdio --lib` | clean (after unrelated concurrent `las` breakage cleared) |
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::dwg::standards::v_ac1018"` | **12/12 passed** |
| `cargo test -p semio-s-plugin-stdio --lib` (whole crate) | **1039 passed, 3 failed** (failures confined to concurrently-edited `bmp`/`stl`, zero in `dwg`) |
