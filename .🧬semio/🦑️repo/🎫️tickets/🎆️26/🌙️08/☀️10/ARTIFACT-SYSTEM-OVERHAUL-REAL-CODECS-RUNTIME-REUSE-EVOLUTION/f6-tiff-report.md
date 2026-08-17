# F6 — 🖼️tiff (standard 6.0) — OpText/OpBinary + DiffCodec report

**Artifact**: `🖼️tiff`, standard `6.0`, path `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/`
**Ticket**: `26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION`

## Scope confirmation

Followed `f6-recon-report.md` §9's procedure literally. Every standard already had a handcrafted
sparse `TiffDiff` (index-keyed `ifds` triple, tag-id-keyed `entries` triple within each IFD,
`DiffAlgebra` impl with `apply`/`between`/`inverse`/`absorb`) and a named `TiffMutation` enum with
handcrafted per-variant `diff()`/`inverse()`, all landed by the prior F3b sub-wave (per
`STATUS.md`, real `cargo test`-confirmed green, 29/0 at that time). This wave's job was **only**
`OpText`/`OpBinary` (Mutation) + `DiffCodec` (Diff) — no diff/mutation SHAPE was touched.

## STEP 1 — real classification (not trusted from the recon table)

The recon table (row 15) flagged tiff as `DERIVE (probable)` with an explicit caveat:
`CHECK-ENUM-ELSEWHERE`, since the plan's own completeness table says TIFF has a `TiffValues`
typed union for tag entries. Verified for real:

`TiffValues` (`📸️snapshot/🦀️component.rs:106`) is a genuine 12-variant **data-carrying** enum —
`Byte(Vec<u8>)`, `Ascii(String)`, `Short(Vec<u16>)`, `Long(Vec<u32>)`, `Rational(Vec<(u32,u32)>)`,
`SByte(Vec<i8>)`, `Undefined(Vec<u8>)`, `SShort(Vec<i16>)`, `SLong(Vec<i32>)`,
`SRational(Vec<(i32,i32)>)`, `Float(Vec<f32>)`, `Double(Vec<f64>)`. It is reachable from:

- **Diff side**: `TiffDiff.ifds: Option<TiffIfdsDiff>` → `TiffIfdModified.diff: TiffTagsDiff` /
  `TiffIfdAdded.ifd: TiffIfd` → `TiffTagModified.values` / `TiffTagAdded.values` / `TiffTag.values`.
- **Mutation side**: `TiffMutation::SetTag.values: TiffValues` directly, and
  `SetSnapshot.snapshot: TiffSnapshot` / `InsertIfd.ifd: TiffIfd` recursively through
  `ifds`/`entries`.

Confirmed by **actually adding the derives and running `cargo check -p semio-s-plugin-stdio --lib`**
(both temporarily added, both reverted after capturing the errors — no derive attempt left in the
committed code):

```
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslDiff)]
pub struct TiffDiff { ... }
```
→
```
error[E0277]: the trait bound `v6_0::…::TiffValues: DslField` is not satisfied
   --> …/🔺️diff/🦀️component.rs (TiffTagsDiff → TiffIfdsDiff → TiffDiff.ifds)
```
(also independently failed on `TiffByteOrder`/`TiffIfdsDiff`/`TiffSnapshot` since none of those
had ever been given `DslRecord`/`DslScalar` — cascading, but moot: `TiffValues` alone is an
unconditional blocker, no derive exists that can produce `DslField` for a data-carrying enum.)

```
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum TiffMutation { ... }
```
→
```
error[E0277]: the trait bound `v6_0::…::TiffValues: DslField` is not satisfied
   --> …/🧬️mutations/🦀️component.rs:42:17   (SetTag { … values: TiffValues })
error[E0277]: the trait bound `v6_0::…::TiffSnapshot: DslField` is not satisfied
   --> …/🧬️mutations/🦀️component.rs:21:19   (SetSnapshot { snapshot: TiffSnapshot })
error[E0277]: the trait bound `v6_0::…::TiffIfd: DslField` is not satisfied
   --> …/🧬️mutations/🦀️component.rs:30:14   (InsertIfd { … ifd: TiffIfd })
```

**Verdict: HAND-ROLL on both sides (recon §3a — enum-in-tree).** No tri-state (`Option<Option<_>>`,
§3b) anywhere in `TiffDiff` — `byte_order`/`ifds`/`pixels` are plain `Option<T>` ("changed to this
value" semantics, not tri-state removal), so `encode_option`/`decode_option` (present in
`GifDiff`/`SvgDiff`) were deliberately omitted as dead code — the recon table's `Option<Option<`
sweep correctly found 0 for tiff's diff file.

## STEP 2b — hand-rolled implementation

Copied §5's primitive set (`hex_encode`/`hex_decode`/`split_top_level`/`strip_brackets`) into
`🔺️diff/🦀️component.rs`, generalized two of them for tiff's shape (`parse_num<T>` — generic over
every scalar type this format carries, since tiff has far more scalar kinds than svg/gif's
mostly-string/usize grammars; `enc_list`/`dec_list` — generic bracketed-list helper subsuming
`enc_num_list`/pair-lists/entity-lists in one function). All made `pub(crate)` so
`🧬️mutations/🦀️component.rs` reuses them directly (same intra-artifact pattern svg's
diff↔mutations reuse established).

**Grammar**:
- `TiffValues`: single-uppercase-letter tag prefix + bracketed positional payload — `B`=Byte,
  `A`=Ascii, `S`=Short, `L`=Long, `R`=Rational, `E`=SByte, `U`=Undefined, `H`=SShort, `G`=SLong,
  `Q`=SRational, `F`=Float, `D`=Double. `Byte`/`Undefined` (raw octets) and `Ascii` (text) are hex;
  every numeric list is decimal comma-separated; `Rational`/`SRational` pairs nest as `[n,d]`.
- `TiffTag`: positional `[tag,kind,values]` (kind as its `TiffFieldType::to_u16()` decimal code).
- `TiffIfd`: bracketed list of `TiffTag` entries.
- `entries`/`ifds` triples: `[removed];[modified];[added]`, `modified`/`added` entries are
  `tag:kind:values` (tags triple) / `index:<tags-triple>` or `index:<ifd>` (ifds triple, recursive)
  — colon-separated, safe since no value encoding ever emits a literal `:`.
- Top-level `TiffDiff`: space-separated `byte-order=…`/`ifds=…`/`pixels=…` tokens, absent token =
  unchanged (exactly matches `TiffDiff`'s existing plain-`Option<T>` semantics, no tri-state tag).
- Top-level `TiffMutation`: `keyword arg=value …`, one arm per variant (`set-snapshot`,
  `set-byte-order`, `insert-ifd`, `remove-ifd`, `set-tag`, `remove-tag`, `set-pixels`,
  `no-mutation`).
- `encode_diff`/`encode_op` = `print_diff()/print_op().into_bytes()` — same simplification
  `GifDiff`/`SvgDiff`/`WriterDiff` use; satisfies every `DiffCodec`/`OpBinary` law without
  inventing a second wire format.

Both `TiffDiff`'s and `TiffMutation`'s struct/enum doc comments now cite the real compiler errors
(F6-confirmed citation style matching `GifFrameDiff`/`SvgDiff`/`SvgMutation`'s precedent).

## STEP 3 — tests added

- `artifacts::tiff::…::diff::component::handcrafted_diff_codec_tests::diff_codec_text_binary_roundtrip_law`
  — exercises all 12 `TiffValues` variants (incl. `Rational`/`SRational` pair lists, `Ascii`/`Byte`/
  `Undefined` hex, signed/unsigned numeric lists, `Float`/`Double`), IFD-level (index-keyed)
  removed/modified/added, tag-level (id-keyed) removed/modified/added in one `between()` call, and
  the scalar `byte_order`/`pixels` tokens, over 5 cases (`default`, `between(a,b)`, `between(b,a)`,
  `between(a,c)` with `c` empty, `between(c,a)`).
- `artifacts::tiff::…::mutations::component::tests::op_text_binary_roundtrip_law` — exercises every
  `TiffMutation` variant incl. `SetTag`'s bare `TiffValues` payload across all 12 field-type
  variants and `SetSnapshot`'s nested `TiffSnapshot`/`TiffIfd`/`TiffTag` payload, plus out-of-range
  targets (still valid grammar, no special-casing needed for round-trip).

Both assert `!printed.contains('\n')`, `parse(print(x)) == x`, `decode(encode(x)) == x`, per the
trait LAWS.

## STEP 4 — real verification

Cargo workspace was mid-flight during this session (a large, unrelated concurrent relocation of
the `◻2d`/`🧊3d` modules from `✏️s/🔨️modules` to `🧰️framework/🔨️modules`, plus several other F6
sibling sub-wave agents actively editing `dxf`/`xml`/`jpg`/`docx` in the same crate) — matches the
documented "Concurrent Cargo Workspace Churn" pattern; polled `cargo check -p
semio-s-plugin-stdio --lib` until it compiled cleanly (0 errors) rather than assuming any error
seen mid-poll was caused by this work. Confirmed at every intermediate poll that **zero** errors
ever appeared inside any `🖼️tiff` file — only pre-existing warnings (`hidden lifetime parameters`,
unrelated `unused import`s in `glue.rs`'s generated wrapper code).

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::tiff"` → **31 passed, 0 failed** (29
  pre-existing from F3b + 2 new law tests).
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate, no filter) → **1074 passed, 1 failed**.
  The 1 failure — `artifacts::docx::…::diff_codec_text_binary_roundtrip_law`,
  `"based_on tri-state Some(None) not exercised"` — is entirely inside `📜️docx`, a **different**
  F6 sub-wave agent's own in-progress work (a tri-state-coverage assertion in *its* test, not
  touched by this session, outside this ticket's ownership boundary
  `🗿️artifacts/🖼️tiff/**` + this report). Not chased, per the recipe's own "classify, don't chase"
  precedent (F1/F2/F3 closers' reports document the identical situation repeatedly for other
  artifacts' concurrent in-flight work).

## Deviations from §5's grammar conventions

- **No `encode_option`/`decode_option`**: unlike `GifDiff`/`SvgDiff`, `TiffDiff` has zero
  `Option<Option<_>>` tri-state fields (confirmed by the recon table's own sweep: 0 occurrences)
  and zero bare `Option<T>` fields inside nested value types — every `Option<T>` in this codec's
  reach is `TiffDiff`'s own top-level "changed to this value" scalars, encoded directly as a
  present/absent top-level token, never as an inline `[0]`/`[1,x]` marker. Including the unused
  helper would have been dead code.
- **Two primitives generalized rather than duplicated per-type**: `parse_num<T: FromStr>` (one
  generic parser for `u8`/`u16`/`u32`/`i8`/`i16`/`i32`/`f32`/`f64`/`usize`, vs. svg's/gif's
  per-type `parse_usize`-style helpers) and `enc_list`/`dec_list` (one generic bracketed-list
  codec, vs. svg's separate list-handling inlined per call site) — tiff's grammar has
  substantially more distinct scalar/list shapes (12 `TiffValues` variants) than svg's or gif's,
  so the generic form avoids ~10 near-duplicate one-off functions. Both are drop-in equivalents of
  the recipe's own primitive set, not a different design.
- **`kind` is a redundant-but-explicit field**: `TiffTag`/`TiffTagModified`/`TiffTagAdded` all
  carry `kind: TiffFieldType` alongside `values: TiffValues` even though `values.kind()` already
  derives it — the grammar encodes both explicitly (`[tag,kind,values]` / `tag:kind:values`)
  rather than reconstructing `kind` from `values` on decode, matching the existing (untouched)
  struct shape exactly rather than second-guessing it.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — F6-confirmed doc comment on `TiffDiff` (real compiler-error citation), hand-rolled
  `impl protocol::DiffCodec for TiffDiff` + grammar primitives + value codecs (all `pub(crate)`
  where reused by mutations) + `handcrafted_diff_codec_tests::diff_codec_text_binary_roundtrip_law`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — F6-confirmed doc comment on the module (real compiler-error citation), replaced the
  `serde_json`-based `OpText`/`OpBinary` stubs with hand-rolled implementations reusing the diff
  file's primitives, + `op_text_binary_roundtrip_law` test.
- Ticket-folder scratch (`.txt`, kept per repo rules):
  `f6-tiff-check1.txt` through `f6-tiff-check4.txt`, `f6-tiff-poll-1.txt`/`f6-tiff-poll-2.txt`,
  `f6-tiff-pollbg-1.txt`/`f6-tiff-pollbg-2.txt`, `f6-tiff-test1.txt`,
  `f6-tiff-fullcrate-test.txt`, `tiff-baseline-check.txt`/`tiff-baseline-check2.txt`,
  `poll_check.sh`, `poll_check.log`.

**No shared files touched**: `glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema` framework
crates, and `POLICY_DIFF_COMPLETENESS_ALLOWLIST` were all read-only for this session. Did not
touch `dxf`/`xml`/`jpg`/`docx` (other agents' concurrent in-flight work observed but not edited),
did not touch the `2d`/`3d` module relocation (unrelated concurrent wave observed but not edited).
