# Wave 7 — XML 1.0 `✳️any` exhaustive mutation round-trip

Subset owned: `📰xml` standard `1.0` subset `✳️any`. Reference library: `quick-xml` 0.42 (reads AND
writes — genuinely differential, unlike a read-only reference).

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `pub const KINDS: &[&str]` (8 entries, declaration order) and
  `kinds_const_matches_enum_variants_in_declaration_order` test. No other change to the vocabulary.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`
  — filled in the stub dispatcher: an independent `quick-xml`-backed tree type (`XNode`/`XDoc`, not
  reusing production `XmlNode`/`XmlSnapshot` — this crate never depends on `semio-s-plugin-stdio`),
  parse/serialize, path-addressed apply/inverse for all 8 kinds, and `project_xml_1_0`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`
  — new: registers `quick-xml-1-0-mutate` oracle (capability `xml-1-0-mutate`), the `semantic-xml-v1`
  comparison profile, and the `xml-1-0-any` mutation catalog (8 kinds).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🧫️fixtures/📰️ooxml-word-document.xml` — new, real input (see
  below).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🧪️tests/mutate-xml-1-0/component.feature` — new, 17
  scenarios (8 mutate + 8 inverse + 1 identity-round-trip).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🧪️tests/mutate-xml-1-0/🦀️component.rs` — new, adapter +
  oracle handlers + `sut`-gated subject module (production `XmlSnapshot`/`XmlMutation`).

Nothing outside this subset was touched: no shared family module, no `Cargo.toml`, no `lib.rs`, no
SVG file (SVG 1.1 also builds on `quick-xml` but owns its own subset untouched here).

## Real input

`word/document.xml`, extracted once (`unzip`, no other edit) from the real committed ECMA-376 example
DOCX at `🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/📜️example.docx`
— the brief's first-choice candidate. It is a genuine OOXML WordprocessingML part (747 bytes): prefixed
element names bound through `xmlns:w`, both single- and multi-attribute elements in authored order, and
both self-closing (`<w:b/>`, `<w:pStyle .../>`) and paired-with-content empty/non-empty elements. Copied
verbatim into `🧫️fixtures/📰️ooxml-word-document.xml`, referenced via `shared://`.

The part carries no XML declaration, no DOCTYPE, no CDATA, no comment and no entity reference — real
minified OOXML parts rarely do. Those five subtleties are instead exercised THROUGH the mutation kinds
on the same real document, per the brief's own allowance to prove the vocabulary's own subtleties
rather than requiring them all to pre-exist in one fixture:
- `set-declaration` / `set-doctype` ADD a real declaration/doctype where none existed, and their
  `inverse` removes it again — proving survival (and correct absence) in both directions.
- `insert-element` inserts one real `<w:r>` carrying a comment child, a CDATA child (`<raw> unescaped
  content`), and an entity-escaped text child (`<`, `>`, `&`, `"`) together, with three attributes in a
  deliberately non-alphabetical author order (`w:id`, `w:rev`, `w:note`).
- `set-text` sets literal text containing `<`, `>`, `&` on a real `<w:t>` run, proving write-side
  escaping and read-side unescaping.
- `set-snapshot` replaces the whole document with `word/styles.xml`'s real content (the same DOCX,
  a second real OOXML part), proving full-document replacement round-trips.

## Writer-freedom narrowing (documented, not silently dropped)

- **Attribute order.** `semantic-xml-v1`'s projection represents each element's attributes as an
  unordered name/value map (`Json::Object`), not the ordered list the tree itself stores — the
  comparison mechanism's own `canonicalize()` sorts object keys before comparing, so this needed no
  `ignoreKeys` entry. Verified directly: inserting an element with attrs given as `c,a,b` writes
  `<w:test c="3" a="1" b="2"/>` (author order preserved on the wire) while the projection still
  compares equal regardless of order.
- **Self-closing vs paired empty elements.** Invisible once decoded — both `<a/>` and `<a></a>` decode
  to the same zero-children `Element`, so this needed no special handling at all, on either the oracle
  or the production side (`xml_node_to_text` already always chooses self-closing for zero children).
- **Numeric vs named character/entity references.** Invisible once decoded — `&#38;` and `&amp;` both
  resolve to the same literal `&`. `quick-xml` 0.42 splits every reference out of `Text` into its own
  `Event::GeneralRef`; the oracle accumulates `Text`/`GeneralRef` runs into one logical string before
  building a node, resolving named refs via `quick_xml::escape::resolve_xml_entity` and numeric refs via
  `BytesRef::resolve_char_ref` — the exact same 5-named-entity-plus-numeric scope production's own
  `xml_unescape_text` narrows to.
- **DOCTYPE internal subset.** Both the oracle's hand-rolled DOCTYPE parser and production's
  `parse_doctype` model only `SYSTEM`/`PUBLIC` external ids plus typed `<!ENTITY>` declarations — never
  `<!ELEMENT>`/`<!ATTLIST>`. This is a genuine, shared, intentional narrowing (not something I could
  silently work around): a real DOCTYPE with `<!ELEMENT>`/`<!ATTLIST>` content (e.g. LaTeX's own
  `logreq` build artifacts, found during input search) would fail to parse on both sides. The wave's
  chosen input never exercises this path — `set-doctype`'s own scenario only ever writes a
  SYSTEM/PUBLIC id plus one `<!ENTITY>`, which both sides model completely — so it cost nothing here,
  but it is a real limit of the subset worth flagging for anyone who later points this codec at a DTD
  with element/attribute-list declarations.

No genuine loss was found beyond the above — every subtlety the brief asked for is either invisible by
construction (self-closing, numeric-vs-named refs) or exercised and passing (namespaces, attribute
order, CDATA, comments, entity escaping, declaration/doctype survival).

## Verification

Both run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, real output, exit code 0 both times:

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-xml-1-0
0 high-priority breach(es) across 0 rule(s):

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-xml-1-0
[test] level=exhaustive cases=1 executed=17 passed=17 failed=0 errored=0 parity=0/0
```

17 = 8 kinds × (mutate + inverse) + 1 identity-round-trip. The Rust SUBJECT phase does not compile this
wave (the pre-existing, unrelated os-kernel `semio_framework::` cycle the fleet brief calls out) — the
subject module is written and fully `sut`-gated; only the oracle phase was run/claimed.

Before wiring into the shared crate, the oracle module's logic (parse / mutate / inverse / serialize /
project, all 8 kinds, against this exact real fixture) was additionally validated standalone: a scratch
crate outside the repo, depending only on `quick-xml = "0.42"`, exercised every kind + inverse + the
attribute-order property and passed 9/9. That scratch crate was never part of the repo and left no
trace in it; only the real, reviewed implementation above was committed to the module.

## Findings

None beyond the documented DOCTYPE-subset narrowing above (shared with production, not introduced by
the oracle, and not exercised by this wave's own scenarios). No defect found in `quick-xml` 0.42 itself
for the operations this subset's 8-kind vocabulary exercises.
