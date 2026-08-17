# F6 — 📰xml 1.0 — OpText/OpBinary/DiffCodec

**Artifact**: `📰xml`, standard `1.0`, path `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/`.

## Classification (verified for real, not trusted from the recon table)

Both the Diff side (`XmlDiff` / `DslDiff`) and the Mutation side (`XmlMutation` / `DslOps`) were
independently verified by actually adding the derive attribute and running
`cargo check -p semio-s-plugin-stdio --lib`, per §9's procedure — not assumed from the recon
report's §8 sweep, which the report itself flags as a grep heuristic.

### Diff side — **HAND-ROLL** (3a + 3b)

Adding `#[derive(dsl::DslDiff)]` to `XmlDiff` fails with:

```
error[E0277]: the trait bound `...::XmlNodeDiff: DslField` is not satisfied
  --> .../📰xml/.../🔺️diff/component.rs:48:22   (root: Option<XmlNodeDiff>)
```

`XmlNodeDiff` is a genuine data-carrying enum (`Element`/`Text`/`Replace`) — no `DslField` impl
exists or can exist for it (only `DslRecord`-derived structs and `DslScalar`-derived unit-only
enums implement `DslField`). A second, independent blocker is also present even without the enum:
`declaration: Option<Option<XmlDeclaration>>` and `doctype: Option<Option<String>>` are tri-state
fields — `classify_field` peels exactly one `Option` layer, and no
`impl<T: DslField> DslField for Option<T>` exists anywhere in the `dsl` crate. This is the
identical structural shape to `SvgDiff` (svg's own `SvgNodeDiff` was literally modeled on
`XmlNodeDiff`, since svg embeds xml's node model), so the failure mode matches exactly.

### Mutation side — **HAND-ROLL** (3a)

Adding `#[derive(dsl::DslOps)]` to `XmlMutation` fails with FOUR distinct field types
simultaneously lacking `DslField`, real captured errors:

```
error[E0277]: the trait bound `XmlSnapshot: DslField` is not satisfied         (SetSnapshot.snapshot)
error[E0277]: the trait bound `XmlDeclaration: DslField` is not satisfied      (SetDeclaration.declaration: Option<XmlDeclaration>)
error[E0277]: the trait bound `XmlNodePath: DslField` is not satisfied         (every path: XmlNodePath field)
error[E0277]: the trait bound `...::XmlNode: DslField` is not satisfied        (InsertElement.node: XmlNode)
```

`SetSnapshot`'s payload (`XmlSnapshot` → `XmlDocument` → `Option<XmlNode>`) reaches the same
enum-shaped blocker as the diff side; `InsertElement.node` carries `XmlNode` directly as a variant
field; and `XmlNodePath` (a plain `Vec<usize>` newtype, not even an enum) simply has no `DslField`
impl either — nobody derived one for it. Both derive attempts were removed after capturing the real
errors (cited verbatim in doc comments on `XmlDiff` and `XmlMutation`); no `E0119` conflicting-impl
residue was left behind.

## What was implemented

Followed §5's hand-roll template directly, using `🎨️svg`'s already-done `SvgDiff`/`SvgMutation`
hand-roll as the literal starting point (per the brief's explicit instruction) — svg embeds this
same `XmlNode` type and its `enc_xml_node`/`dec_xml_node` encoding logic applies verbatim, only the
type names (`Svg*` → `Xml*`) and the artifact-local re-declaration of the primitives (svg's copies
are `pub(crate)` to svg's own crate-visibility scope, not reachable across the artifact boundary)
changed.

**`🔺️diff/component.rs`** (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`):
- New `//#region 🔖️HandcraftedDiffCodec` appended after the existing `SetSnapshot` region (existing
  `XmlDiff`/`XmlNodeDiff`/apply/absorb/`between` code untouched).
- `//#region 🔖️Primitives` (`pub(crate)`, for mutations-side reuse): `hex_encode`/`hex_decode`,
  `enc_str`/`dec_str`, `split_top_level`, `strip_brackets`, `encode_option`/`decode_option`.
- `//#region 🔖️XmlValueCodecs` (`pub(crate)`): `enc_xml_node`/`dec_xml_node` (recursive,
  single-letter tag per `XmlNode` variant: `E`=Element, `T`=Text, `D`=CData, `M`=Comment, `P`=PI),
  `enc_declaration`/`dec_declaration`.
- `//#region 🔖️DiffValueCodecs` (private): `enc_attrs_diff`/`dec_attrs_diff`,
  `enc_node_diff`/`dec_node_diff` (tag `E`/`T`/`R` for the three `XmlNodeDiff` variants),
  `enc_children_diff`/`dec_children_diff` (the `[removed];[modified];[added]` collection-triple
  grammar).
- `//#region 🔖️TopLevel`: `print_xml_diff`/`parse_xml_diff` (space-separated
  `declaration=.../doctype=.../root=...` tokens, absent token = unchanged) and
  `impl protocol::DiffCodec for XmlDiff` — `encode_diff`/`decode_diff` are the text bytes verbatim
  (same simplification `GifDiff`/`SvgDiff`/`WriterDiff` use, satisfies every trait law without a
  second wire format).
- New `#[cfg(test)] mod handcrafted_diff_codec_tests` with `diff_codec_text_binary_roundtrip_law`:
  exercises the recursive `Element`/`Text`/`Replace` `XmlNodeDiff` variants, both top-level
  tri-states (`Some(None)` and `Some(Some(_))`), attribute add/remove/modify, and nested child
  add/remove/modify, over `XmlDiff::default()` plus 4 `between()` combinations (including a
  root-removed case).

**`🧬️mutations/component.rs`** (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`):
- Two new imports from the diff module: the grammar primitives
  (`decode_option`/`dec_declaration`/`dec_str`/`dec_xml_node`/`encode_option`/`enc_declaration`/
  `enc_str`/`enc_xml_node`/`split_top_level`/`strip_brackets`) and `protocol::OpText` added to the
  existing `use protocol::Mutation` line (plus a `#[cfg(test)] use protocol::OpBinary`) — both
  needed because Rust requires a trait in scope to call its methods even from within the same file
  that implements it.
- Replaced the previous `serde_json`-based `OpText`/`OpBinary` impls (which satisfied the trait's
  LAWS but were explicitly flagged by the recon report as not a genuine handcrafted grammar, the
  same anti-pattern `WriterDiff` uses) with a real hand-rolled grammar in the existing `OpCodecs`
  region: `enc_node_path`/`dec_node_path` (bracketed comma-separated index list),
  `enc_xml_snapshot`/`dec_xml_snapshot` (4-field positional tuple: schema, root, doctype,
  declaration), `print_xml_mutation`/`parse_xml_mutation` (`keyword arg=value ...`, one match arm
  per variant), and `impl protocol::OpText`/`impl protocol::OpBinary for XmlMutation` (binary = text
  bytes verbatim, same simplification the diff side uses).
- New `#[cfg(test)] mod op_codec_tests` with `op_text_binary_roundtrip_law`: exercises every
  variant (`NoMutation`, `SetSnapshot` with a full nested document, `SetDeclaration`/`SetDoctype`
  both `Some`/`None`, `InsertElement`'s bare `XmlNode` payload, `RemoveElement`, `SetAttribute` both
  `Some`/`None`, `SetText`).

No existing test files were duplicated — both new test modules extend the existing `component.rs`
files in place, per the region/subregion convention, matching svg's precedent exactly.

## Deviations from svg's template

None of substance. The only differences are mechanical: `XmlNodePath` is a tuple struct
(`XmlNodePath(pub Vec<usize>)`) rather than svg's bare `NodePath = Vec<usize>` type alias, so
`enc_node_path`/`dec_node_path` go through `.0`; xml's `XmlMutation` has no `ViewBox`/`TransformOp`
analog (svg-specific typed-attribute sugar), so the grammar has 8 variants instead of svg's 10 and
no extra enum-tag codecs beyond `enc_xml_node`/`dec_xml_node` were needed.

## Verification (all real, this session)

- `cargo check -p semio-s-plugin-stdio --lib` with `dsl::DslDiff` temporarily added to `XmlDiff` →
  captured the real `E0277` error above, then reverted.
- `cargo check -p semio-s-plugin-stdio --lib` with `dsl::DslOps` temporarily added to `XmlMutation`
  → captured the real `E0277` errors above (4 distinct field types), then reverted.
- `cargo check -p semio-s-plugin-stdio --lib` (final, hand-rolled code in place) → clean, zero
  errors attributable to `📰xml` (confirmed via `-->`-path grep against the full compiler output).
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::xml"` → **24 passed, 0 failed** (includes
  the 2 new law tests: `op_text_binary_roundtrip_law`,
  `diff_codec_text_binary_roundtrip_law`) — `f6-xml-test-scoped.txt`.
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1074 passed, 1 failed** —
  `f6-xml-full-crate-test-final.txt`. All 24 xml tests pass in this run (verified via grep, zero
  xml lines without `... ok`). The single failure,
  `artifacts::docx::...::handcrafted_diff_codec_tests::diff_codec_text_binary_roundtrip_law`
  (`based_on tri-state Some(None) not exercised`), is entirely inside `📜️docx`'s own file, which
  `git status` confirms is currently modified/uncommitted by a live sibling F6 session actively
  working on the docx artifact right now — not touched by this session, not attributable to xml.

**Live-tree churn encountered while verifying** (documented for the record, not fixed, not in
scope): during this session's polling, `cargo check`/`cargo test` transiently failed for reasons
entirely outside `📰xml` as multiple sibling F6 fan-out agents landed and un-landed WIP on other
artifacts in the same shared crate — observed in order: a workspace-manifest churn on an unrelated
`🧊️3d` module (resolved on its own), `🖊️dxf`'s in-progress `dsl::DslDiff` derive attempt, `🔣️json`'s
and `📝️md`'s in-progress hand-rolled `OpText`/`OpBinary` (both missing the same `use
protocol::OpText;` scope fix this session applied to xml — presumably about to self-resolve as
those sessions finish), and `📜️docx`'s in-progress `enc_named_triple`/`dec_named_triple` type
mismatch. None were touched; each was confirmed via `git status` to be another live session's own
uncommitted work before being classified as "not mine" and waited out.

## Ownership boundary respected

Only `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/**` (specifically the `🔺️diff` and `🧬️mutations`
`component.rs` files under `🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/`) and this report were
touched. `📦️glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema` framework crates, `🏪️store`, and
`POLICY_DIFF_COMPLETENESS_ALLOWLIST` were not touched. No git-mutating commands were run. All
temporary `cargo check`/`cargo test` logs kept in this ticket folder as `.txt`
(`f6-xml-diff-derive-check1.txt`, `f6-xml-mutation-derive-check1.txt`, `f6-xml-check-final.txt`,
`f6-xml-check-final2.txt`, `f6-xml-test-scoped.txt`, `f6-xml-full-crate-test1.txt`,
`f6-xml-full-crate-test-final.txt`).
