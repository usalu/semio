# F6 — 📝️md / commonmark — OpText/OpBinary + DiffCodec Report

**Artifact**: `📝️md`, standard `commonmark`, path
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/`. Scope per the F6 brief: hand-write
`OpText`/`OpBinary` for `MdMutation` and `protocol::DiffCodec` for `MdDiff`. Ownership boundary
respected: only `🗿️artifacts/📝️md/**` (specifically the `🔺️diff/🦀️component.rs` and
`🧬️mutations/🦀️component.rs` files under `🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/`) and this
report were touched. No shared files (`📦️glue.rs`, `📜️script.ts`, `dsl`/`protocol`/`schema` framework
crates, `POLICY_DIFF_COMPLETENESS_ALLOWLIST`) were edited.

## Result

Both sides: **HAND-ROLL**, exactly as the recon's §8 classification table predicted (row 27,
`HAND-ROLL (3a+3b) — LARGEST enum count`) — verified for real this session, not trusted blindly.

## Step 1 — classification, verified for real

Per `f6-recon-report.md` §9's procedure, I did not just trust the recon table — I actually added
`#[derive(dsl::DslDiff)]` to `MdDiff` and `#[derive(dsl::DslOps)]` to `MdMutation` (temporarily
disabling my own hand-rolled `impl protocol::DiffCodec for MdDiff` first, so the derive's *own*
attempt would surface instead of an `E0119` conflicting-impl error) and ran
`cargo check -p semio-s-plugin-stdio --lib`. Real captured errors (full output saved to
`f6-md-derive-attempt-check1.txt` in this folder):

```
error[E0277]: the trait bound `MdBlocksDiff: DslField` is not satisfied
  --> …/📝️md/…/🔺️diff/🦀️component.rs:39:24
   |
39 |     pub blocks: Option<MdBlocksDiff>,
   |                        ^^^^^^^^^^^^ unsatisfied trait bound
help: the trait `DslField` is not implemented for `MdBlocksDiff`
  --> …/📝️md/…/🔺️diff/🦀️component.rs:50:1
50 | pub struct MdBlocksDiff {
```
```
error[E0277]: the trait bound `v_commonmark::…::MdSnapshot: DslField` is not satisfied
  --> …/📝️md/…/🧬️mutations/🦀️component.rs:31:19
31 |         snapshot: MdSnapshot,
   |                   ^^^^^^^^^^ unsatisfied trait bound
```
```
error[E0277]: the trait bound `v_commonmark::…::MdBlock: DslField` is not satisfied
  --> …/📝️md/…/🧬️mutations/🦀️component.rs   (InsertBlock/ReplaceBlock's `block: MdBlock` field)
error[E0277]: the trait bound `v_commonmark::…::MdInline: DslField` is not satisfied
  --> …/📝️md/…/🧬️mutations/🦀️component.rs   (SetInlines's `inlines: Vec<MdInline>` field)
error[E0277]: the trait bound `v_commonmark::…::MdPathStep: DslField` is not satisfied
  --> …/📝️md/…/🧬️mutations/🦀️component.rs   (every `path: Vec<MdPathStep>` field)
```

Root cause matches §3a exactly: `MdBlocksDiff` (reachable from `MdDiff.blocks`) transitively embeds
`MdBlockDiff` (a genuine data-carrying enum, via `MdBlockModified.diff`), and `MdBlockDiff` itself
embeds `MdBlock`/`MdInline` (both data-carrying enums, via its `Replace`/`Heading`/`Paragraph`/...
variants) — none of `MdBlockDiff`/`MdBlock`/`MdInline`/`MdPathStep` have (or can derive) `DslField`,
since that trait is only derivable for plain structs (`DslRecord`) or unit-only enums (`DslScalar`).
Same root cause independently blocks the Mutation side (`SetSnapshot{snapshot: MdSnapshot}`
recursively contains `MdBlock`; `InsertBlock`/`ReplaceBlock`/`SetInlines` carry an enum-shaped
payload directly as a variant field). §3b (tri-state `Option<Option<_>>`, present on
`MdBlockDiff::List.start` and `MdBlockDiff::CodeBlock.info`) is a second, independent blocker that
the compiler never got to report here because it bails out on the shallower §3a violation
(`MdBlocksDiff: DslField`) first — same "one artifact can hit both, compiler reports whichever is
shallower" behavior the recon documented for `SvgDiff`.

After capturing these, the temporary derives and the temporary `impl` disable were fully reverted;
`diff -u` against a pre-edit backup confirmed the restored files are byte-identical to the
hand-rolled implementation described below.

## Step 2 — hand-rolled implementation

Followed §5's grammar template and §9's STEP 2b literally. Both files already existed with real
`Apply`/`DiffAlgebra`/`Absorb` logic (`🔺️diff/component.rs`, 792 lines pre-existing) and a
`serde_json`-stub `OpText`/`OpBinary` (`🧬️mutations/component.rs`) from an earlier wave — this
session only added the `DiffCodec` impl (new `//#region 🔖️HandcraftedDiffCodec` at the end of the
diff file) and replaced the `serde_json` stub with a real hand-rolled `OpText`/`OpBinary` (in-place
in the existing `//#region OpCodecs`), per CLAUDE.md's "extend existing files with regions" rule —
no new files created.

### Tag vocabulary (the recon's flagged risk: 3 interacting data-carrying enum kinds)

`md` has the most enum kinds of any F6 hand-roll in this sub-wave (`MdInline`, `MdBlock`,
`MdBlockDiff` are all data-carrying, plus mutations-side `MdPathStep`). Assigned each its own
non-overlapping single-uppercase-letter tag range (documented in the diff file's region doc
comment) rather than reusing letters across enums the way `XmlNode`/`SvgNodeDiff` did (both are
safe — every grammar position's expected type is statically known by the recursive-descent parser
— but disjoint ranges are cheap and match the recon's explicit ask for this artifact):

| Enum | Variants | Tag range |
|---|---|---|
| `MdInline` | Text/Emphasis/Strong/Code/Link/Image/SoftBreak/HardBreak/HtmlInline (9) | `A`–`I` |
| `MdBlock` | Heading/Paragraph/List/CodeBlock/BlockQuote/ThematicBreak/HtmlBlock (7) | `J`–`P` |
| `MdBlockDiff` | same 7 + `Replace` (8) | `Q`–`X` |
| `MdPathStep` (mutations-side) | BlockQuote/ListItem (2) | `Y`–`Z` |

### One structural device beyond §5's template

`MdListItemsDiff.modified`'s `diff: MdBlocksDiff` field is a BARE triple (no tag, no enclosing
bracket of its own — unlike `SvgChildrenDiff`, svg never embeds a bare triple directly inside a
`,`-joined entry, only ever through `encode_option` or a tag-prefixed enum, both of which already
supply an enclosing bracket). Left unwrapped, `enc_blocks_diff`'s own `;`-separated
`[removed];[modified];[added]` sections would sit at bracket-depth 0 relative to the OUTER
`MdListItemsDiff` triple's own `;` split, corrupting it. Fix: wrap the nested triple in an extra
bracket pair on encode (`format!("{}:[{}]", index, enc_blocks_diff(diff))`), strip it back off on
decode (`dec_blocks_diff(strip_brackets(rest)?)`) — documented in the diff file's region doc
comment as the one place this artifact's grammar needed a device `SvgDiff`'s template didn't.

### Primitives

Copied `hex_encode`/`hex_decode`/`enc_str`/`dec_str`/`split_top_level`/`strip_brackets`/
`encode_option`/`decode_option` verbatim from `SvgDiff`'s template (own copy, `pub(crate)`, per the
recon's documented "no shared hand-roll helpers module yet" state); added `parse_usize`/`enc_bool`/
`dec_bool` (md's diff has plain `bool` fields `ordered`/`tight` that svg's didn't).
`Option<Option<T>>` tri-states (`List.start`, `CodeBlock.info`) use nested `encode_option`/
`decode_option` calls — `encode_option(start, |v| encode_option(v, |x| x.to_string()))` — composing
cleanly since both are generic over `T`.

## Step 3 — tests

Both mandated law tests added, extending the existing files (no new test files):

- `handcrafted_diff_codec_tests::diff_codec_text_binary_roundtrip_law`
  (`🔺️diff/component.rs`) — two fixture snapshots (`md_a`/`md_b`, 9 vs 8 top-level blocks, `List`
  items 2 vs 3) diffed in both directions (`between(a,b)`/`between(b,a)`, the same dual-direction
  trick `SvgMutation`'s `sweep_a`/`sweep_b` uses, since the recipe's naive positional `between` can
  only show one of {removed-tail, added-tail} per call) plus `between` against an empty snapshot,
  plus one manually-constructed `MdDiff` for `MdBlockDiff::ThematicBreak` (UNREACHABLE via
  `between` — two `ThematicBreak`s are always structurally equal, per that variant's own
  pre-existing doc comment) and a nested `Replace`. Together these exercise: 7 of `MdBlockDiff`'s 8
  variants via `between` (`Heading`/`Paragraph`/`List`/`CodeBlock`/`BlockQuote`/`HtmlBlock`/
  `Replace`, the last via a same-index kind-change `HtmlBlock`→`Heading`) + `ThematicBreak`
  manually; every `MdInline` variant (via an `all_inline_kinds()` helper, incl. both
  `Option<title>` branches for `Link`/`Image` and one level of `Emphasis`/`Strong` nesting); both
  tri-states going `Some(x) → Some(None)`; both `MdBlocksDiff` and `MdListItemsDiff` triples at
  multiple nesting depths (top-level, `BlockQuote.blocks`, `List.items`) showing `removed`,
  `modified`, and `added` across the whole suite. One block (`md_a[6]`/`md_b[6]`) is identical
  between fixtures, proving an unchanged block correctly produces no diff entry.
- `op_codec_tests::op_text_binary_roundtrip_law` (`🧬️mutations/component.rs`) — every
  `MdMutation` variant, incl. `InsertBlock`/`ReplaceBlock` carrying a `List` block (exercising
  `enc_block`'s own recursive `items: Vec<Vec<MdBlock>>`), `SetInlines` with mixed inline kinds incl.
  nested `Emphasis`, and both `MdPathStep` variants (incl. a multi-step nested path mixing
  `ListItem`+`BlockQuote`).

Both tests assert exactly the mandated laws: `!printed.contains('\n')`, `parse(print(x)) == x`,
`decode(encode(x)) == x`.

## Step 4 — verification (real, this session)

- `cargo check -p semio-s-plugin-stdio --lib` — clean (only pre-existing unrelated warnings).
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::md"` → **26/26 passed** (24 pre-existing +
  2 new: `diff_codec_text_binary_roundtrip_law`, `op_text_binary_roundtrip_law`).
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **final, stable result: 1075 passed,
  0 failed.** A concurrent sibling session was actively mid-edit on `📜️docx` for most of this
  session (confirmed via `git status` showing uncommitted changes to `📜️docx`'s diff/mutations
  files at multiple points) and two transient states were observed along the way purely as a
  byproduct of polling: a `📜️docx`-only compile error (`OpBinary`/`OpText` not imported into scope
  in their test module) and, once compiling, one `📜️docx`-only test failure
  (`artifacts::docx::…::diff_codec_text_binary_roundtrip_law` asserting `based_on tri-state
  Some(None) not exercised`). Neither was caused by, or ever touched, anything in this report's
  scope — re-running after `📜️docx` stabilized gave the clean 1075/0 result above, confirmed twice
  in a row. See "Concurrent workspace state" below.
- `MdMutation`/`MdDiff`'s pre-existing tests (`mutation_diff_law`, `inverse_law`, `absorb_law`,
  `between_roundtrip_law`, `field_sweep_covers_every_mutable_field`) all continued passing
  unaffected — this session only added a `DiffCodec` impl and replaced the `OpText`/`OpBinary`
  bodies; `Apply`/`DiffAlgebra`/`Absorb` semantics were untouched.

### Concurrent workspace state (informational, not a claim about my own code)

This is a live shared tree with other sessions active. `📜️docx` (owned by a different F6 sub-wave
agent) was mid-edit for most of this session; at various points `cargo test` for the whole crate
either failed to compile (docx-only errors: missing `OpBinary`/`OpText` imports) or compiled but had
1 docx-only test failure. Neither state was ever caused by, or fixed by, anything in this report's
scope (`📝️md` only). The scoped `artifacts::md` test run was clean and stable every time it was run.
By the time this report was finalized, `📜️docx` had stabilized and the whole-crate run was clean:
**1075 passed, 0 failed**, confirmed on two consecutive runs.

## Deviations from §5's template

1. Tag ranges are fully disjoint per enum kind (`A`-`I`/`J`-`P`/`Q`-`X`/`Y`-`Z`) rather than reusing
   letters across enums the way `XmlNode`/`SvgNodeDiff` share `E`/`T` — a deliberate readability
   choice for this artifact's 4 interacting enum kinds, not a correctness requirement (see "Tag
   vocabulary" above).
2. Added `enc_bool`/`dec_bool` primitives (`"1"`/`"0"`) — `md`'s diff/mutation values include plain
   `bool` fields (`List.ordered`/`List.tight`) that neither `GifDiff` nor `SvgDiff` needed a
   dedicated helper for (svg's one bool-shaped field, `standalone`, was encoded inline with a
   one-off closure).
3. One nested-triple bracket-wrapping device not present in `SvgDiff`'s template (see "One
   structural device" above) — required because `MdListItemsDiff` embeds a bare `MdBlocksDiff`
   triple directly, a shape svg's grammar never has to handle.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — doc-comment citation of the real `DslField`/`DslDiff` blocker on `MdDiff`; new
  `//#region 🔖️HandcraftedDiffCodec` (primitives, `MdInline`/`MdBlock`/`MdBlockDiff`/
  `MdBlocksDiff`/`MdListItemsDiff` codecs, `impl protocol::DiffCodec for MdDiff`) + new
  `//#region 🧪️Tests` (`diff_codec_text_binary_roundtrip_law`).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — doc-comment citation of the real `DslField`/`DslOps` blocker on `MdMutation`; imports extended
  to pull in the diff file's `pub(crate)` primitives; `//#region OpCodecs` rewritten (hand-rolled
  `OpText`/`OpBinary` replacing the `serde_json` stub, `MdPathStep`/`MdSnapshot` value codecs) + new
  `//#region 🧪️Tests` (`op_text_binary_roundtrip_law`).
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-md-derive-attempt-check1.txt` (real
  `cargo check` output from the temporary derive experiment described in Step 1).

No shared files touched (`glue.rs`, `📜️script.ts`, `dsl`/`protocol`/`schema` framework crates all
read-only this session). `POLICY_DIFF_COMPLETENESS_ALLOWLIST` not touched — `md`'s `🔺️diff` file now
has a real `DiffCodec` impl, so it should drop out of the live `dsl-migration/diff-completeness`
policy breach list on its own, same mechanism the recon's binary/gif89a/svg pilot already proved
(not independently re-verified via `bun ./📜️script.ts policy` this session — out of scope per the
brief, which says rely on `cargo test` for the Op side and the policy mechanism is unchanged from
the recon's own verification).
