# Wave 7 — SVG 1.1 `any` exhaustive mutation round-trip

Executor subset: 🎨️svg standard 🔖️1.1 subset ✳️any. Reference: `quick-xml` 0.42.

## What was built

| What | Path |
|---|---|
| Mutation vocabulary (pre-existing, confirmed) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` |
| `pub const KINDS` + `kinds_const_matches_enum_variants_in_declaration_order` test (added) | same file |
| Mutation oracle (written) | `.../🔖️1.1/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs` |
| Catalog + oracle registration + comparison profile (written) | `.../🔖️1.1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json` |
| Real fixture (copied) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🧫️fixtures/qr-code.svg` |
| Test case | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🧪️tests/mutate-svg-1-1/{component.feature,🦀️component.rs}` |

`SvgMutation` declares 11 kinds: `no-mutation`, `set-snapshot`, `set-declaration`, `set-doctype`,
`insert-element`, `remove-element`, `set-element-name`, `set-attribute`, `set-text`, `set-view-box`,
`set-transform`. All 11 got a `mutate-<kind>` and `inverse-<kind>` scenario, plus the
`identity-round-trip` scenario — 23 scenarios total.

## Real input and provenance

`🔣️qr-code.svg`, copied verbatim (`cp`, byte-identical) from the real, committed
`🧰️framework/🔨️modules/🖼️assets/🪧️logos/🔣️qr-code.svg` into this artifact's own `🧫️fixtures/`,
referenced as `shared://qr-code.svg` per the fleet brief's own instruction for this exact situation.
It is a real QR-code logo drawing, not an icon glyph: 664 nested `<g>` groups (many carrying a real
`transform="matrix(...)"`), 329 `<rect>` leaves, 3 `<path>` leaves with real path data, a real
`viewBox="0 0 1015 1015"`, an `<?xml version="1.0" encoding="UTF-8" standalone="no"?>` declaration,
five namespace-declaration attributes on the root (`xmlns`, `xmlns:svg`, `xmlns:xlink`,
`xmlns:inkscape`, `xmlns:sodipodi`), and one `<image>` whose real `xlink:href` carries a ~74 KB
embedded base64 data URI folded across many lines with literal `&#10;` character references.

No real committed `.svg` anywhere in the tree (searched `🧰️framework/🔨️modules/🖼️assets`,
`♻️mit-bestand`, product asset directories) carries a genuine `<style>`/CDATA block, so that specific
subtlety is exercised nowhere in this case — reported here rather than fabricated into the fixture.
Every other subtlety the brief named (namespace declarations, `xlink:href`, attribute ordering,
self-closing elements, numeric precision in path/geometry data) is exercised on real content.

Exact NodePath targets used in the Examples table were computed by replaying this subset's own
`parse_node` whitespace/text-node rules (every non-empty run between tags becomes a `Text` node,
including whitespace-only runs) against the real file with a throwaway Python script
(`/private/tmp/.../scratchpad/svg_index.py`, not committed), so every `path`/`parent` in the feature
file addresses a real, verified node — e.g. `[4,0,0]` is the real `<g id="g3"
transform="matrix(0.35,0,0,0.35,280,0)">` three levels inside the QR module tree, `[3,0]` is the real
`<image>`.

## `set-view-box` / `set-transform` — made substantive per the brief

- `set-view-box` doubles the real root `viewBox` (`0 0 1015 1015` → `0 0 2030 2030` for the plain
  mutation; `set-snapshot` doubles only the width, to `0 0 2030 1015`, to prove the two code paths
  are independent).
- `set-transform` replaces `<g id="g3">`'s real `matrix(0.35,0,0,0.35,280,0)` with
  `translate(50,50) rotate(45)`, and the inverse restores the exact original matrix.

Both are decomposed into typed numeric fields in the projection (not compared as raw attribute
strings) — real writer freedom between two independently-written number formatters, narrowed and
documented in `../🧪️oracle/🔣️component.json`'s `comparisonProfiles` entry, not chased byte-for-byte.

## Oracle module

`.../🧪️oracle/🦀️component.rs` builds its own small `quick-xml`-backed element tree (`QNode`/`QDoc`),
completely independent of this subset's own `XmlNode`/`XmlDocument`/`xml_document_from_text`/
`xml_document_to_text` codec — no function or type is imported from the production crate into the
oracle crate. It hand-rolls its own `viewBox`/`transform` grammars (independent of
`parse_view_box`/`parse_transform_list`), its own JSON mutation-spec codec, and its own
`inverse_spec` (reads the CURRENT pre-mutation tree to compute the undo spec, mirroring
`SvgMutation::inverse`'s semantics without importing it). `project_svg_1_1` is the single independent
reader both the oracle's and the (future) subject's re-serialized bytes are read back through.

One real quick-xml 0.42 subtlety worth recording: `Reader::read_event()` splits entity/character
references out of body text into separate `Event::GeneralRef` events (not opt-in) — the parser
coalesces runs of `Text`+`GeneralRef` between two structural events back into one `QNode::Text`, to
match this subset's own single-Text-node-per-run model.

**No shared family module was created.** The brief allows one only when two subsets genuinely share
an implementation; the sibling 📰xml 1.0 oracle (which also uses quick-xml) is still an unfilled stub
as of this wave, so there is no second subset to share with yet, and the xml subset was not touched.

## Finding — not narrowed, not worked around

The production `xml` codec's `xml_escape_attr`
(`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`,
a shared module this subset does not own) escapes only `&`, `<` and `"` when re-emitting an
attribute value. It does not re-escape a literal tab/newline/CR — produced when it *decodes* a
`&#10;`-style numeric character reference, which the real `xlink:href` above carries dozens of times
— back into a character reference on write. `quick-xml`'s own attribute writer escapes all three,
symmetrically with its own reader's attribute-value normalization rule (XML spec: literal whitespace
bytes in an attribute value normalize to a space on the NEXT parse; character-reference-produced
ones do not). So once the SUBJECT phase compiles, any scenario that leaves this specific attribute
untouched (`identity-round-trip` foremost) is expected to show the subject's re-serialized value
differing from the oracle's under exact-string comparison. This is real loss in a shared module out
of this subset's ownership; documented in the Feature description and the oracle module's own
`_comment`/rationale rather than hidden by loosening the projection.

## Verification (verbatim, both from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`)

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-svg-1-1
0 high-priority breach(es) across 0 rule(s):

full breach set (including non-blocking priorities): .../⚡️cache/breaches/testing.json
EXIT_CODE=0

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-svg-1-1
[test] level=exhaustive cases=1 executed=23 passed=23 failed=0 errored=0 parity=0/0
EXIT_CODE=0
```

23/23 (11 mutate + 11 inverse + 1 identity-round-trip). Spot-checked the cached raw/projection
results (`.🧬semio/🦑️repo/⚡️cache/tests/results/test-s-plugins-stdio-artifacts-svg-*-mutate-svg-1-1-oracle-rust/`)
to confirm the mutations are genuinely substantive, not vacuously passing:
- `set-view-box`: real bytes change `viewBox="0 0 1015 1015"` → `"0 0 2030 2030"`; inverse restores
  `[0, 0, 1015, 1015]` exactly.
- `set-transform`: `<g id="g3">`'s transform becomes `[translate(50,50), rotate(45)]`; inverse
  restores the exact original `matrix(0.35,0,0,0.35,280,0)`.
- `set-attribute`: the real `xlink:href` (7301 chars) is replaced with an 82-char data URI; inverse
  restores the full original 7301-char value byte-for-byte (proving the oracle's own `&#10;`
  round trip is correct — see the Finding above for where the SUBJECT differs).
- `set-snapshot`: root gets `id="wave7-snapshot-marker"` and `viewBox` width-only doubled to
  `[0, 0, 2030, 1015]`.
- `insert-element`/`remove-element`: the target's child count moves 3 → 4 → 1 (remove coalesces two
  now-adjacent whitespace-only text siblings into one `Text` node on re-parse — expected, universal
  XML-serializer behaviour on both the oracle and the future subject side alike, not a defect).
- `set-doctype`/`set-declaration`: the real SVG 1.1 public DOCTYPE and the flipped 1.1/UTF-8/
  standalone=true declaration both land exactly as specified.

**Rust SUBJECT phase**: written and `sut`-gated per the brief (expected not to compile this wave —
a concurrent os-kernel refactor cycle, not this subset's bug). Not run; no subject/parity claim made
(`parity=0/0` above confirms it did not run). The added `kinds_const_matches_enum_variants_in_
declaration_order` unit test lives inside the production crate's own `#[cfg(test)]` and is likewise
unverified this wave for the same reason.

## Hard-rule compliance

- Stayed entirely inside `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/`; the sibling 📰xml 1.0 subset was
  read (for its `XmlNode`/`XmlDoctype`/`XmlDeclaration`/`XmlAttr` types, which `SvgMutation` already
  depends on) but never edited.
- Did not touch `🧪️oracle/📦️packages/🦀️rust/Cargo.toml` or `📦️lib.rs` — `quick-xml` was already
  linked and the `svg` module already wired by a prior session.
- Did not touch `.gitignore`, `project.json`, or `launch.json`.
- No `deferredKinds`; no migration/compat code; no `git commit`/`stash`/`checkout` run.
