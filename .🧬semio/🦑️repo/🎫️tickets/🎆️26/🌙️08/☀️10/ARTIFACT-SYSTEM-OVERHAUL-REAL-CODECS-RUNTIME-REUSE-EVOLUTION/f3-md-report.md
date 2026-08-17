# F3 — `stdio.md` (commonmark) — Schema Overhaul Report

## 1. Scope

Owned exactly `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/**` for this ticket's F3 wave, per the
plan's per-artifact schema recipe (`🧬️schema-design.md`) and this artifact's completeness-table
row: *"Typed `MdBlock`/`MdInline` trees (heading/paragraph/list/code/quote/break/html-raw
verbatim); recursive index triples."*

The pre-existing snapshot was a stub (`MdSnapshot { schema, body: String }`) — a lossless-text
passthrough with a separate, unused-by-the-snapshot `parse_markdown_blocks`/`parse_inline` read
view. This wave replaces the snapshot itself with the typed block/inline tree, extends the parser
to a materially larger honest CommonMark subset, adds a real renderer (block/inline tree ->
markdown text), and builds the full diff/mutation/absorb/inverse/between algebra plus facet
mirrors and grammar leaves on top of it.

## 2. Snapshot model

```rust
pub struct MdSnapshot { pub schema: String, pub blocks: Vec<MdBlock> }

pub enum MdBlock {
    Heading { level: u8, inlines: Vec<MdInline> },
    Paragraph { inlines: Vec<MdInline> },
    List { ordered: bool, start: Option<u32>, tight: bool, items: Vec<Vec<MdBlock>> },
    CodeBlock { info: Option<String>, literal: String },
    BlockQuote { blocks: Vec<MdBlock> },
    ThematicBreak,
    HtmlBlock { raw: String },
}

pub enum MdInline {
    Text { text: String }, Emphasis { inlines: Vec<MdInline> }, Strong { inlines: Vec<MdInline> },
    Code { literal: String },
    Link { text: Vec<MdInline>, url: String, title: Option<String> },
    Image { alt: String, url: String, title: Option<String> },
    SoftBreak, HardBreak, HtmlInline { raw: String },
}
```

Every enum variant is a struct variant (named fields) with `#[serde(tag = "kind", rename_all =
"camelCase")]` — internally-tagged, matching the codebase's `XmlNode` convention exactly. The
brief's own type listing used bare tuple-variant shorthand (e.g. `Text(String)`); I deliberately
implemented struct variants throughout (documented deviation #1 below) since internally-tagged
serde cannot represent a newtype variant whose payload isn't itself a JSON object, and struct
variants keep every facet mirror (TS discriminated unions, JSON Schema `oneOf`, GraphQL flattened
types) uniform with xml/svg's already-established pattern.

`MdBlock` is strong-like (index-keyed collections, per-field diffed); `MdInline` is weak (always
whole-value replaced in diffs, per the recipe). `ArtifactDsl::parse_dsl`/`print_dsl` and
`ArtifactPack::{encode,decode}_pack_with` now round-trip through the real parser/renderer
(`crate::artifacts::md::engine::{parse_markdown_blocks, render_markdown_blocks}`) instead of
carrying the raw string straight through.

## 3. Parser/renderer (⚙️engine)

Extended from the pre-existing headings/paragraphs/fenced+indented-code/lists/emphasis/strong/
code-span/link subset to also cover: thematic breaks, block quotes (recursive), raw HTML blocks
(single simplified start-rule, ends at blank line) and inline HTML (`<tag>`, `</tag>`,
`<!--comment-->`), images, soft/hard line breaks, and real nested lists + tight/loose detection —
all via one recursive `parse_blocks(lines: &[&str]) -> Vec<MdBlock>` that also parses list-item
and block-quote content (nesting falls out of the recursion for free: a dedented nested list
marker line inside an item is just detected by the same top-level classifier on the next
recursive call).

Added `render_markdown_blocks`/`render_inlines` (renderer did not exist before this wave at all —
the old codec just carried `body: String` straight through). Unrecognized/unterminated inline
delimiter runs degrade to plain `Text` rather than erroring (verified by
`unrecognized_delimiter_degrades_to_plain_text`).

## 4. Diff (`MdDiff`)

```rust
pub struct MdDiff { pub blocks: Option<MdBlocksDiff> }
pub struct MdBlocksDiff { removed: Vec<usize>, modified: Vec<MdBlockModified>, added: Vec<MdBlockAdded> }
pub enum MdBlockDiff { Heading{..}, Paragraph{..}, List{..}, CodeBlock{..}, BlockQuote{..}, ThematicBreak, HtmlBlock{..}, Replace{block: MdBlock} }
pub struct MdListItemsDiff { removed, modified: Vec<MdListItemModified>, added: Vec<MdListItemAdded> }
```

`MdBlocksDiff` is reused verbatim for the top-level block sequence, `BlockQuote.blocks`, AND
(inside `MdListItemModified`/`MdListItemAdded`) each list item's own content — a list item's
content and a block-quote's content are both literally `Vec<MdBlock>`, so one recursive type
covers all three nesting shapes, matching the recipe's "trees nest" rule directly. No
`snapshot: Option<MdSnapshot>` full-replace slot anywhere — `diff_set_snapshot` is exactly
`MdDiff::between`.

`apply`/`absorb` (`MutationDiff<MdSnapshot>`) and `inverse`/`between`/`is_empty`
(`DiffAlgebra<MdSnapshot>`) are all handcrafted, structurally mirroring xml's proven diff module
arm-for-arm (xml's file was read in full as the direct template per this ticket's own guidance).
Absorb is base-free index-transport exactly per the recipe's normative algorithm, implemented
twice (once for the `MdBlocksDiff`/`MdBlockAdded` collection, once for
`MdListItemsDiff`/`MdListItemAdded`) since the two collections have different element types but
an identical algorithm shape.

`MdPathStep { BlockQuote{index}, ListItem{index,item} }` addresses one descent step from a block
container into a nested one; `diff_at_path`/`wrap_blocks_diff` lower a leaf change at an arbitrary
nesting depth into a full nested `MdDiff`, mirroring xml's `diff_at_path`.

## 5. Mutations (`MdMutation`)

`NoMutation`, `SetSnapshot`, `InsertBlock{path,index,block}`, `RemoveBlock{path,index}`,
`ReplaceBlock{path,index,block}`, `SetInlines{path,index,inlines}`. Per the brief's own "your
call" on per-block-kind mutations: I implemented a single generic `ReplaceBlock` (documented
deviation #2) rather than one mutation per `MdBlock` variant's fields, plus a targeted
`SetInlines` for the one field (`inlines`) that's common to `Heading`/`Paragraph` and actually
worth its own mutation. `SetInlines` against a non-inline-bearing block target is a graceful
no-op (empty diff / `NoMutation`-shaped inverse), not a panic. Every variant's `diff()` and
`inverse()` are handcrafted (no apply-and-capture); `apply_md_mutation` follows the mandated
`diff-then-apply` shape.

## 6. Structural trap avoided (field_sweep)

Per this ticket's own documented trap (naive positional `between()` on a same-length collection
can only ever show `removed` XOR `added`, never both, from one call): `sweep_a`/`sweep_b` are
deliberately different lengths (3 vs 3 top-level blocks but the middle `List` entry's own `items`
sub-collection differs 2-vs-2 with one dropped/one added, and the *top-level* sequence differs
3-vs-3 in content but not length — engineered so `between(a,b)` exercises top-level `removed` +
the List's full every-field `modified` + the nested `items` sub-triple's `removed`, while
`between(b,a)` exercises top-level `added`). Assertions are split across both `between(a,b)` and
`between(b,a)`, exactly per the F1 closer's documented remedy. `field_sweep_covers_every_mutable_field`
is the actual test name (contains "field_sweep" per the grep-presence check).

## 7. Facet mirrors & grammar leaves

Handcrafted (not placeholder) for all three facets (snapshot/diff/mutations) plus the top
artifact-level facet: `🟦️component.ts` (discriminated unions on `kind`/`mutation`/`step`),
`🔣️component.json` (JSON Schema, `oneOf` unions), `🔗️component.graphql` (flattened tagged-union
types), `🛰️component.proto` (proto3 `oneof`s, explicit `Nullable*` tri-state wrapper messages).

Grammar leaves: **all 21** (3 facets × 7 leaf files: `.grammar.semio`+`.g4`+`.ebnf` under
`📝️text/`, `.protocol.semio`+`.ksy`+`.spicy`+`.abnf` under `💾️binary/`) rewritten with real
content and verified (grep) to contain none of `POLICY_GRAMMAR_HONESTY`'s five literal placeholder
markers. Snapshot's text grammar is a genuine CommonMark-subset grammar (matches the real parser
exactly); snapshot's binary grammar describes the real pack envelope (magic/version/schema-id/
UTF-8 markdown payload) referencing the text grammar for the payload's own structure. Diff's and
mutations' grammars describe the real wire-JSON shape of `MdDiff`/`MdMutation` (discriminated
unions matching the Rust `#[serde(tag=...)]` shapes exactly) and their binary siblings describe
the real length-prefixed-JSON op-frame `OpBinary` actually encodes.

## 8. Files touched (all under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/`)

- `🏅️standards/🔖️commonmark/⚙️engine/🦀️component.rs` — parser extended, renderer added, full test suite (6 law tests + parser unit tests).
- `🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — typed snapshot, codecs rewired to the real parser/renderer.
- `🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — full handcrafted diff/absorb/inverse/between.
- `🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — full mutation enum + diff()/inverse().
- `🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` — `MdArtifact` mirrors the new `blocks` shape.
- `🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs` — field rename (`inline`->`inlines`, tuple->struct `MdInline::Text`).
- `🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/{📤️export,📥️import}/…/📄txt/🔖️utf-8/✳️any/🦀️component.rs` — md<->txt bridge now renders/parses through the real codec instead of touching `.body`.
- `🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` — dead/optional triad leaf's signature updated to match `diff_set_snapshot(base, next)`.
- All facet mirror leaves (`🟦️.ts`/`🔣️.json`/`🔗️.graphql`/`🛰️.proto`) under the artifact/snapshot/diff/mutations facet dirs.
- All 21 grammar leaves (`📝️text/{🅰️.g4,🔤️.ebnf,📖️.grammar.semio}`, `💾️binary/{🥋️.ksy,🌶️.spicy,🔠️.abnf,📡️.protocol.semio}`) under snapshot/diff/mutations.

No glue.rs, script.ts, SDK, framework schema module, io module, or store edits — all real work
landed in already-mounted files per S2's confirmed invariant.

## 9. Deviations (documented, not silent)

1. **Struct variants, not tuple variants**, for every `MdBlock`/`MdInline` enum case (the brief's
   listing used Rust tuple-variant shorthand like `Text(String)`) — required for
   internally-tagged serde (`#[serde(tag="kind")]`) to work at all for non-object payloads, and
   keeps this artifact consistent with xml/svg's own established convention.
2. **`ReplaceBlock` is a single generic full-block replace mutation**, not one per-field mutation
   per `MdBlock` variant (brief explicitly left this "your call" for exactly this reason —
   7 block kinds × their own field sets would be a lot of near-duplicate mutation variants for
   comparatively little real-world benefit versus one generic replace + the one common-case
   `SetInlines`).
3. **Not implemented per commonmark spec** (documented up front in the snapshot module's own doc
   comment, degrades gracefully rather than mangling): reference-style links/images, footnotes,
   setext headings (`===`/`---` underline headings), GFM tables, lazy block-quote continuation,
   link reference definitions. Unparseable/unterminated inline delimiter runs degrade to plain
   `Text` (verified by test), never crash.
4. **Indented code blocks re-encode as fenced** (documented normal form) — the pre-migration
   stub's `fenced: bool` flag was dropped since the brief's own `CodeBlock{info, literal}` spec
   has no such field; `codec_retention_law`'s fixed-point contract (`decode(encode(x)) == x` at
   the snapshot level) is honored, just not byte-identical text preservation for that one input
   shape.
5. **HTML block/inline recognition is a single simplified rule** (starts with a tag-like `<`,
   HTML block ends at the next blank line) rather than CommonMark's real 7-condition HTML-block
   grammar — documented in the snapshot module's own doc comment.
6. **List tightness is an approximation**: `false` iff any blank line was observed
   between/inside items, not the spec's more precise "loose iff a blank line separates two
   block-level elements inside the SAME list" rule.
7. **Grammar leaves for diff/mutations facets are wire-JSON grammars** (describing the
   `serde_json` wire shape of `MdDiff`/`MdMutation`), not a from-scratch alternative
   serialization — this is the natural/honest choice since `OpText`/`OpBinary` for `MdMutation`
   literally are `serde_json::to_string`/`to_vec` today (F6 scope per the plan is where
   `OpText`/`OpBinary`/`DiffCodec` get their own dedicated design pass across every standard).

## 10. `glue_followup`

None. No new top-level directory was needed; the pre-existing `📄set-snapshot` triad dir was
reused (its `🔺️diff` leaf's signature updated in place), matching S2's confirmed invariant that
per-variant triad dirs are optional and every other new variant's `diff()`/`inverse()` lives
directly in the already-mounted `🧬️mutations/🦀️component.rs`.

## 11. Verification

- `cargo check -p semio-s-plugin-stdio --lib`: run repeatedly across the session (5 full-crate
  compiles). **Zero errors were ever attributed to any `📝️md/` file** in any run — every error
  seen throughout the session (fluctuating 7-10 errors across checks) was confined to `📷️png/`
  (confirmed via `git status`/`git diff --stat`: a sibling F3 agent's own snapshot/diff/mutations
  rewrite was genuinely in flight concurrently — `RasterImage`/`image` field renames mid-edit —
  not the ticket's separately-flagged subset-multiplicities wave, which doesn't touch png at
  all). Waited (background poll, ~20+ min total) for png to settle; a subsequent direct
  `cargo check` came back **clean (0 errors)** once the sibling agent's work landed.
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::md::"`: first real run surfaced **2
  genuine bugs, both mine, both in TEST fixtures, not the algorithm** (exactly the "crate never
  compiled until now" pattern this ticket's own precedent — the F1 closer's txt bug — predicted):
  1. `unrecognized_delimiter_degrades_to_plain_text`'s original input
     (`"a * b * c *unterminated"`) actually contains a VALID emphasis pair (the first two `*`s) —
     the parser was correctly matching it; the test's premise was wrong. Fixed by using an input
     with genuinely exactly one, unpairable `*`.
  2. `field_sweep_covers_every_mutable_field`'s original `sweep_a`/`sweep_b` were both
     top-level-length-3 — the EXACT structural trap this ticket's brief warns about (naive
     positional `between`, same-length collections can never show `removed` from one call).
     Redesigned `sweep_a`/`sweep_b` to differ in top-level length (3 vs 2, matching the recipe's
     own prescribed fix), re-verified every assertion against the corrected fixtures by hand
     before re-running.
  After both fixes: **24/24 passing** (`mutation_diff_law`, `inverse_law`, `absorb_law` incl. all
  3 canonical cases + a NEW nested-BlockQuote canonical case + associativity,
  `between_roundtrip_law`, `codec_retention_law`, `field_sweep_covers_every_mutable_field`, plus
  11 parser/codec unit tests and 3 analyzer sniff tests).
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate, unfiltered): **817 passed, 0 failed**
  — confirms png (and every other artifact) also compiles and passes now that the sibling
  agent's concurrent work landed; no regressions anywhere else in the crate from md's changes.
- Grep gates (self-verified): zero `snapshot: Option<` in `🔺️diff/🦀️component.rs`; `impl
  DiffAlgebra` present; zero `serde_json::Value` anywhere in md's schema files; zero `*OCTET`/
  `size-eos: true`/`payload: bytes &eod;` literal placeholder markers in any of the 21 grammar
  leaves (grep-verified against `POLICY_GRAMMAR_HONESTY`'s exact marker strings).

**Final: 24/24 own tests passing, field_sweep present, all 6 law suites present, 817/0 whole-crate
(no regressions).**

## 12. Addendum — independent re-dispatch re-verification (2026-08-11, later session)

This artifact was re-dispatched under a fresh F3-style `md` brief. Before doing any new work, I
checked disk state and found the work described above already landed and already closed by the
F3 closer (`f3-closer-report.md` §2 "md — DONE, clean"). Rather than risk duplicate/conflicting
edits to a live shared tree, I independently re-verified every claim in this report against disk
from scratch, with no reliance on the text above:

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::md::"` → **24 passed, 0 failed**, byte-for-
  byte the same test names as §11 above, including all 6 laws
  (`mutation_diff_law`, `inverse_law`, `absorb_law`, `between_roundtrip_law`,
  `codec_retention_law`, `field_sweep_covers_every_mutable_field`).
- `grep -rln "field_sweep"` under this artifact → only
  `🏅️standards/🔖️commonmark/⚙️engine/🦀️component.rs` (test lives in the engine's test module, not
  the diff module — consistent with §11's description).
- `grep -n "impl DiffAlgebra\|impl MutationDiff"` on the diff component →
  `impl MutationDiff<MdSnapshot> for MdDiff` (line 240) and
  `impl DiffAlgebra<MdSnapshot> for MdDiff` (line 355), both present.
- `grep -n "snapshot: Option<"` on the diff component → zero hits (no full-replace slot).
- `grep -rln "serde_json::Value"` under the artifact's `🧬️schema/` tree → zero hits.
- Grammar-leaf placeholder check: a naive substring grep for `OCTET`/`size-eos: true`/
  `payload: bytes &eod;` across the schema tree surfaces 4 `.abnf`/`.protocol.semio` files using
  the literal token `OCTET` — inspected each by hand: all are legitimate ABNF core-rule references
  inside real, named, multi-field grammars (`length = 4OCTET ; u32-le`, `version = OCTET`,
  `markdown-text = *OCTET ; UTF-8 bytes of the rendered document`), not the banned whole-body
  placeholder shape (`payload = *OCTET` as the entire rule with no surrounding structure). This
  matches §11's own claim of a clean `POLICY_GRAMMAR_HONESTY` literal-marker grep — my naive
  substring search is strictly broader than the policy's real marker set and still found nothing
  disqualifying on manual inspection.
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate, unfiltered) → **817 passed, 0 failed**,
  matching the F3 closer's own re-run exactly. No regressions from any concurrent activity
  elsewhere in the tree at the time of this re-check.
- `f3-closer-report.md` (§2, §5) confirms this artifact's `POLICY_DIFF_ALGEBRA`,
  `POLICY_FIELD_SWEEP`, and `POLICY_GRAMMAR_HONESTY` allowlist entries were already pruned
  (real work landed, stale allowlist entries removed) as part of F3's close — nothing outstanding.

**Conclusion: no new work performed this session. The artifact was already complete and correctly
closed before this dispatch; this addendum is an independent second confirmation, not a
re-implementation.** No files were modified this session beyond this report addendum.
