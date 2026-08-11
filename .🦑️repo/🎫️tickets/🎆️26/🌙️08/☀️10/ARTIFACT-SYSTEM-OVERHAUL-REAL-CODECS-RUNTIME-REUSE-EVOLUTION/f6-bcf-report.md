# F6 — bcf (2.1) — OpText/OpBinary + DiffCodec Report

Scope: this wave adds ONLY `protocol::OpText`/`protocol::OpBinary` for `BcfMutation` and
`protocol::DiffCodec` for `BcfDiff`. The snapshot/diff/mutation SHAPE (already handcrafted and law-tested
by F5, see `f5-bcf-report.md`) was not touched. Read first: `f6-recon-report.md` (primary spec, §1-3
derive machinery, §4-5 worked examples, §9 procedure).

## 1. Classification — verified for real, both sides HAND-ROLL (recon's guess was wrong on "0 enums")

The recon's §8 classification table guessed bcf as `HAND-ROLL (3b)` — tri-state only, "enum-free by this
file's sweep" — and explicitly flagged this as unverified ("0 enums found in that sweep — verify no enum
exists in `BcfCamera`"). That flag was correct to raise: **`BcfCamera` is a real data-carrying enum**
(`Perspective{..}` / `Orthogonal{..}`, the `visinfo.xsd` `xs:choice` typed as a genuine Rust enum, per
F5's own design), reachable from both the Diff and Mutation sides. So bcf actually hits **both** documented
failure modes (§3a enum-in-tree AND §3b tri-state) on **both** sides, not just tri-state on the diff side.

Verified per the recon's literal STEP 1 procedure — added the derive attempt, ran a real
`cargo check -p semio-s-plugin-stdio --lib`, captured the exact error, then reverted:

**Diff side** (`#[derive(dsl::DslDiff)]` added to `BcfDiff`, then removed):
```
error[E0277]: the trait bound `NamedTripleDiff<String, BcfTopicDiff, BcfTopic>: DslField` is not satisfied
   --> …/🔺️diff/🦀️component.rs:181:24   (pub topics: Option<BcfTopicsDiff>)
error[E0277]: the trait bound `v2_1::…::BcfCamera: DslField` is not satisfied
   --> …/🔺️diff/🦀️component.rs:172:84   (via BcfViewpointDiff.camera: Option<Option<BcfCamera>>)
error[E0277]: the trait bound `v2_1::…::BcfComponents: DslField` is not satisfied
   --> …/🔺️diff/🦀️component.rs:172:84   (via BcfViewpointDiff.components: Option<Option<BcfComponents>>)
```
(Full captured output: `f6-bcf-derive-attempt-check1.txt` in this folder.) Three independent blockers:
`NamedTripleDiff<K,D,T>` itself has no `DslField` impl (no generic bridge exists in the `dsl` crate for
any collection-triple engine — this is now confirmed the SAME root blocker every other collection-triple
artifact in the remaining 28 will hit, not bcf-specific); `BcfCamera` is a genuine enum (§3a); and
`BcfComponents`/several other fields are reached through `Option<Option<T>>` tri-state (§3b, no
`impl<T: DslField> DslField for Option<T>` exists).

**Mutation side** (`#[derive(dsl::DslOps)]` added to `BcfMutation`, then removed):
```
error[E0277]: the trait bound `v2_1::…::BcfSnapshot: DslField` is not satisfied
   --> …/🧬️mutations/🦀️component.rs:28:19   (SetSnapshot { snapshot: BcfSnapshot })
error[E0277]: the trait bound `v2_1::…::BcfTopic: DslField` is not satisfied
   --> …/🧬️mutations/🦀️component.rs:34:16   (InsertTopic { topic: BcfTopic })
error[E0277]: the trait bound `v2_1::…::BcfCamera: DslField` is not satisfied
   --> …/🧬️mutations/🦀️component.rs:87:24   (SetViewpointCamera { camera: Option<BcfCamera> })
```
(Full captured output: same file.) `SetSnapshot` recursively contains `BcfCamera` via
`topics -> viewpoints -> camera`; `InsertTopic`/`InsertComment`/`InsertViewpoint` each carry a whole
snapshot-shaped struct (none `DslRecord`-derived); and `SetViewpointCamera{camera: Option<BcfCamera>}`
carries the enum DIRECTLY as a variant field — the mutation-side twin of `SvgMutation::InsertElement`'s
finding. `SetComment{viewpoint_ref: Option<Option<String>>}` independently fails the tri-state check
too. Both derive attempts were reverted immediately after capturing the error (`git diff` confirmed
only the one derive-attribute line changed and was restored); the citations are preserved as doc
comments on `BcfDiff` and `BcfMutation` in the live files.

## 2. Hand-rolled `DiffCodec` for `BcfDiff` (`🔺️diff/🦀️component.rs`)

Same grammar family established by `GifDiff`/`SvgDiff` (§5 of the recon report): bracket-depth-aware
`split_top_level`/`strip_brackets`, hex `enc_str`/`dec_str` for strings, hex `enc_bytes`/`dec_bytes` for
raw bytes, uniform `encode_option`/`decode_option` for `Option<T>` (`[0]` / `[1,<enc>]`), positional
`[f1,f2,...]` tuples for plain structs, single-letter tag prefix for the one data-carrying enum
(`BcfCamera`: `P[...]` Perspective / `O[...]` Orthogonal), space-separated `name=value` top-level line.

Two additions beyond the recon's literal primitive list, both direct in-spirit extensions (not a
deviation from the grammar's rules, just new reusable pieces this artifact's shape needed):
- `enc_list`/`dec_list` — a bracketed comma-joined list codec for plain (non-keyed) `Vec<T>` fields
  (`labels`, `exceptions`, `selection`, `coloring`); the un-keyed sibling of the named-triple codec
  below.
- `enc_named_triple`/`dec_named_triple` — a **generic** codec for bcf's own `NamedTripleDiff<K,D,T>`
  engine (`[removed];[modified];[added]`, `key:diff` for modified entries), written ONCE and
  instantiated per collection (`topics`, `comments`, `viewpoints`, `parts`) rather than copy-pasted per
  collection the way svg's non-generic `SvgChildrenDiff`/`SvgAttributesDiff` needed — a legitimate
  simplification enabled by bcf's collections all sharing the identical generic engine (unlike svg's
  bespoke per-shape triples).

Tri-state fields (`camera`/`components`/`snapshot` on `BcfViewpointDiff`, `viewpoint_ref` on
`BcfCommentDiff`) are handled by nesting `encode_option`/`decode_option` two levels deep — e.g.
`encode_option(&d.camera, |inner: &Option<BcfCamera>| encode_option(inner, enc_camera))` — which falls
straight out of `encode_option`'s existing generic signature with no new primitive needed; `Some(None)`
prints as `[1,[0]]`, `Some(Some(c))` as `[1,[1,<enc_camera(c)>]]`, `None` (unchanged) as the field's
token being omitted entirely from the diff's top-level line.

`encode_diff`/`decode_diff` = `print_diff().into_bytes()` / `parse_diff(utf8(bytes))`, same
simplification `GifDiff`/`SvgDiff`/`WriterDiff` all use — satisfies every `DiffCodec` law without
inventing a second wire format.

Real captured `print_diff` output (`between(sweep_b, sweep_a)`, from `diff_codec_text_binary_roundtrip_law`
via a temporary `eprintln!` + `cargo test -- --nocapture`, then removed — full capture in
`f6-bcf-diff-print-sample.txt`), exercising all 4 collection triples + every tri-state `Some(None)`
transition + the `BcfCamera` `Perspective`->`Orthogonal` transition:
```
version=322e31 topics=[746f7069632d616464];[6b656570:[[1,4b6565702d746f706963206265666f7265],[1,6265666f72652064657363],[1,4f70656e],[1,4c6f77],[1,[6265666f7265]],[1,323032342d30312d30315430303a30303a30302b30303a3030],[1,61406578616d706c652e636f6d],[1,[632d616464];[632d6b656570:[[1,323032342d30312d30315430303a30303a30302b30303a3030],[1,61406578616d706c652e636f6d],[1,6265666f72652074657874],[1,[1,76702d72656d6f7665]]]];[[632d72656d6f7665,323032342d30312d30315430303a30303a30302b30303a3030,61406578616d706c652e636f6d,77696c6c2062652072656d6f766564,[1,76702d6b656570]]]],[1,[76702d616464];[76702d6b656570:[[1,[1,P[[1,2,3],[0,0,-1],[0,1,0],60]]],[1,[1,[[324f32467224743458375a66384e4f657733464c4f48],[0,[317951426f6f3764354545424c69794d784767544c63]],[[4646464630303030,[304254424677366639304e666839725031646c5f336e]]]]]],[1,[1,02]]]];[[76702d72656d6f7665,[1,P[[1,2,3],[0,0,-1],[0,1,0],60]],[1,[[324f32467224743458375a66384e4f657733464c4f48],[0,[317951426f6f3764354545424c69794d784767544c63]],[[4646464630303030,[304254424677366639304e666839725031646c5f336e]]]]],[1,01]]]]]];[[746f7069632d72656d6f7665,57696c6c2062652072656d6f766564,,4f70656e,,[],,,[],[]]] parts=[706172742d6164642e747874];[706172742d6b6565702e747874:[[1,6265666f7265]]];[[706172742d72656d6f76652e747874,676f6e65]]
```
Decoded shape: `version=<hex(2.1)> topics=[<removed guid>];[<key>:<9-field topic-diff>];[<added
topic>]` where the modified topic's own diff nests `[...];[<comment-key>:<comment-diff>];[<added
comment>]` and `[1,<viewpoints-triple>]` (the `viewpoints` field being `Option<...>`, wrapped once
more) whose modified viewpoint entry shows `camera: Some(Some(Perspective{...}))` as
`[1,[1,P[[1,2,3],[0,0,-1],[0,1,0],60]]]`.

## 3. Hand-rolled `OpText`/`OpBinary` for `BcfMutation` (`🧬️mutations/🦀️component.rs`)

Replaced the pre-existing `serde_json`-based stub impls (verbose, not a real grammar) with a hand-rolled
`keyword arg=value ...` grammar, same shape the derive's own handcrafted-wrapper convention and
`SvgMutation`'s hand-roll both use. Reuses the diff module's `pub(crate)` primitives
(`enc_str`/`enc_camera`/`enc_topic`/`encode_option`/...) via import rather than duplicating them a
second time in this file — same reuse pattern `SvgMutation` established against `SvgDiff`. One new
function local to this file: `enc_bcf_snapshot`/`dec_bcf_snapshot` (the `SetSnapshot` variant's payload
codec, `[schema,version,[topics...],[parts...]]`).

`encode_op`/`decode_op` = `print_op().into_bytes()` / `parse_op(utf8(bytes))`, same simplification as
the diff side.

Real captured `print_op` output (from `op_text_binary_roundtrip_law`, same capture-and-remove
methodology as above):
```
set-comment topic-guid=7431 guid=6331 date=[0] author=[0] text=[1,55706461746564] viewpoint-ref=[1,[0]]
set-comment topic-guid=7431 guid=6331 date=[1,323032352d30312d30315430303a30303a30302b30303a3030] author=[1,61406578616d706c652e636f6d] text=[0] viewpoint-ref=[1,[1,767032]]
set-viewpoint-camera topic-guid=7431 guid=767031 camera=[1,P[[1,2,3],[0,0,-1],[0,1,0],60]]
set-viewpoint-camera topic-guid=7431 guid=767031 camera=[1,O[[4,5,6],[1,0,0],[0,0,1],2.5]]
set-viewpoint-camera topic-guid=7431 guid=767031 camera=[0]
```
Confirms: `viewpoint-ref=[1,[0]]` is the `SetComment{viewpoint_ref: Some(None)}` tri-state-cleared case
(outer `[1,...]` = "field touched", inner `[0]` = "new value is None"); `viewpoint-ref=[1,[1,767032]]`
is `Some(Some("vp2"))`; `camera=[1,P[...]]`/`[1,O[...]]`/`[0]` are `Some(Perspective)`/`Some(Orthogonal)`/
`None` respectively, exercising both `BcfCamera` tags plus the outer `Option`.

## 4. Tests added (extended the existing engine test module, no new test files)

Per this artifact's own established convention (all 6 F5 law tests already live in
`⚙️engine/🦀️component.rs`'s `mod tests`, reusing shared fixtures `sample_snapshot`/`sweep_a`/`sweep_b`/
`perspective_camera`/`orthogonal_camera`/...), the two new laws were added to the SAME module rather
than creating new test files or duplicating fixtures elsewhere:

- **`op_text_binary_roundtrip_law`** (Law 7): every `BcfMutation` variant, including
  `SetViewpointCamera` with `Some(Perspective)`, `Some(Orthogonal)`, and `None` (exercising the enum
  codec's both tags plus the outer `Option`), and `SetComment` with `viewpoint_ref: Some(None)` and
  `Some(Some(_))` (both tri-state transitions). Asserts `!printed.contains('\n')`,
  `parse_op(print_op(m)) == m`, `decode_op(encode_op(m)) == m`.
- **`diff_codec_text_binary_roundtrip_law`** (Law 8): `BcfDiff::default()`, `between(sweep_a, sweep_b)`,
  `between(sweep_b, sweep_a)`, `between(sweep_a, sweep_a)` — the same fixtures `field_sweep` already
  uses, so every collection triple (topics/comments/viewpoints/parts) and every tri-state field's
  `Some(None)` transition is exercised for free, plus the `BcfCamera` `Perspective`->`Orthogonal`
  transition inside `vp-keep`'s diff. Same three assertions as above, diff-flavored.

No existing test was modified; no new test files were created (both laws extend
`⚙️engine/🦀️component.rs`'s existing `#[cfg(test)] mod tests`, matching CLAUDE.md's
extend-don't-create rule).

## 5. Verification (all real, this session)

- `cargo check -p semio-s-plugin-stdio --lib`: hit transient concurrent-churn compile breaks from
  sibling F6 sessions mid-fixing their own `OpText` import (`no method named print_op ... trait OpText
  ... implemented but not in scope`) across three different artifacts in sequence (`pptx`, `pdf 1.7`,
  `gltf`) while this session's own bcf code was already clean — confirmed via `git status`/repeated
  `cargo check` that these were NOT bcf files; polled (25s interval) rather than chased, per this
  ticket's documented convention, until the whole crate compiled clean with zero errors.
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::bcf"` -> **18 passed, 0 failed** (the pre-existing
  16 from F5 + the 2 new law tests above), confirmed on the FINAL file state (after the temporary
  `eprintln!` sample-capture lines used for §2/§3 were added and then removed again). Full output:
  `f6-bcf-scoped-test-final.txt` (earlier confirming run: `f6-bcf-scoped-test1.txt`).
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) -> **1061 passed, 0 failed, 0 filtered**
  (final run; an earlier run mid-session read 1059/0 — the +2 delta is other concurrent sessions'
  work landing, never this artifact's). Full output: `f6-bcf-full-crate-test-final.txt` (earlier:
  `f6-bcf-full-crate-test1.txt`).
- `git status --porcelain` scoped to `🗿️artifacts/💬️bcf/` confirms this session's edits are exactly
  three files: `⚙️engine/🦀️component.rs` (2 new tests), `🧬️schema/🔺️diff/🦀️component.rs` (hand-rolled
  `DiffCodec` + doc-comment citation), `🧬️schema/🧬️mutations/🦀️component.rs` (hand-rolled
  `OpText`/`OpBinary` + doc-comment citation, replacing the old `serde_json` stub). No other bcf file,
  no shared file (`glue.rs`/`📜️script.ts`/framework crates), and no sibling artifact touched.
- `POLICY_DIFF_COMPLETENESS_ALLOWLIST` (`📜️script.ts:2304`): **not touched**, per instructions —
  `policyDiffCompletenessBreaches` (`📜️script.ts:3185`) detects coverage via a literal
  `content.includes("DiffCodec for")` check on the same file as the `MutationDiff` impl; the diff file
  now contains `impl protocol::DiffCodec for BcfDiff` in the exact same file as
  `impl MutationDiff<BcfSnapshot> for BcfDiff`, so bcf naturally drops out of that breach list the same
  way binary/gif89a/svg did in the recon pilot — verified by direct inspection of the check's logic and
  the file's own content (did not re-run the full ~21k-line `bun ./📜️script.ts policy` output given its
  cost and that the check's exact string-match condition is now visibly satisfied; the recon pilot
  already proved the mechanism live for 3 other artifacts using the identical pattern).

## 6. Deviations from the recon's §9 template (both additive)

- `enc_list`/`dec_list` and generic `enc_named_triple`/`dec_named_triple` added beyond the recon's
  literal helper-function list (§5/§9), because bcf's collections are all instances of one generic
  `NamedTripleDiff<K,D,T>` engine rather than bespoke per-collection triple types like svg's — writing
  the codec generically once (instantiated 4 times: topics/comments/viewpoints/parts) is more in the
  spirit of "reusable, not project specific" than copy-pasting 4 near-identical non-generic codecs would
  have been. No existing primitive was renamed or removed.
- Recon's §8 classification guessed bcf as tri-state-only HAND-ROLL; verified (both sides, real compiler
  errors) as enum-AND-tri-state HAND-ROLL, since `BcfCamera` is reachable from both the diff and
  mutation sides. Documented as a correction in §1 above and in doc comments on both `BcfDiff` and
  `BcfMutation` in the live files, matching the citation style `GifFrameDiff`/`SvgDiff` established.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/⚙️engine/🦀️component.rs` — added
  `op_text_binary_roundtrip_law` and `diff_codec_text_binary_roundtrip_law` to the existing `mod tests`;
  added `DiffCodec`/`OpBinary`/`OpText` to the test module's `use protocol::{...}` line.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — hand-rolled `impl protocol::DiffCodec for BcfDiff` (full grammar + primitives, `pub(crate)` for
  mutations-side reuse), doc-comment citing the real `#[derive(dsl::DslDiff)]` compile error, extended
  the snapshot-module import list (`BcfColoring`/`BcfPoint3`/`BcfVisibility`).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — replaced the `serde_json`-stub `OpText`/`OpBinary` with hand-rolled grammar impls (reusing the diff
  module's primitives), doc-comment citing the real `#[derive(dsl::DslOps)]` compile error, added
  `OpText` to the `protocol::{Mutation, OpText}` import.
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-bcf-baseline-check.txt`,
  `f6-bcf-derive-attempt-check1.txt`, `f6-bcf-check1.txt` through `check4.txt`,
  `f6-bcf-check-poll.txt`, `f6-bcf-check-poll2.txt`, `f6-bcf-check-poll3.txt`,
  `f6-bcf-scoped-test1.txt`, `f6-bcf-scoped-test-final.txt`, `f6-bcf-full-crate-test1.txt`,
  `f6-bcf-full-crate-test-final.txt`, `f6-bcf-diff-print-sample.txt`.

## Ownership boundary respected

Touched only `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/**` (exactly the 3 files above) + this report.
Did NOT touch `glue.rs`, `📜️script.ts`, `POLICY_DIFF_COMPLETENESS_ALLOWLIST`, the `dsl`/`protocol`/
`schema` framework crates, `🏪️store`, or any sibling artifact — `pptx`/`pdf 1.7`/`gltf`'s transient
concurrent compile breaks encountered mid-session were never touched or fixed by this agent, only
waited out via polling. No git-mutating command was run.
