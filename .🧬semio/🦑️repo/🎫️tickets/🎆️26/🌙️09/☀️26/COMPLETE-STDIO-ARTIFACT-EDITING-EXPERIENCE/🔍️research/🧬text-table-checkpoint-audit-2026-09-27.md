# Text and Table Checkpoint Audit — 2026-09-27

## Scope and proof limit

This is a read-only audit of the current shared checkout. It inspects the stdio
JSON/I-JSON, XML/Valid XML, and TXT editor sources, and the CSV, TSV, BCF, EPW,
and XLSX primary table windows. No product source was changed and no native build
or test was started. Every conclusion here is static-source evidence, not a runtime
green result.

The ticket checkpoint records that concurrent text edits overlapped the prior
component compilation. That result is not proof of the current source state. The
checkpoint’s earlier XLSX test repair and CSV/BCF/EPW run state are likewise
historical until current targets run.

## Verified source evidence

### Text editors

- JSON Base and I-JSON generate ordinal m=member and i=item node segments, so
  duplicate names, literal #2 text, and separator-containing names are labels
  instead of identity:
  [JSON Base main](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:32)
  and
  [I-JSON main](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:32).
  Both compare a submitted revision with the canonical host revision before
  parsing and mutating:
  [Base](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🦀️.rs:223)
  and
  [I-JSON](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🦀️.rs:225).
  Their retained generic snapshot route creates compact PatchSnapshot mutations:
  [Base](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🦀️.rs:507)
  and
  [I-JSON](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🦀️.rs:509).

- XML Base and Valid XML check canonical revision, parse root-source edits, reject
  invalid source, require a non-root target to resolve to a text node, and return
  no mutation for unchanged root source or node text:
  [XML Base](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs:242)
  and
  [Valid XML](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/✏️editor/🦀️.rs:248).

- TXT supplies the exact natural document body to its primary text control in
  [the main window](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:27).
  Its reducer compares canonical revision, reconstructs the body into the native
  snapshot, returns no mutation for an identical result, and emits line/ending
  mutations for changes:
  [TXT editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/✏️editor/🦀️.rs:199).
  Authored, unrun tests cover empty source, CRLF/extra-carriage preservation,
  Unicode/malformed operation text, no-op, and stale revision:
  [TXT tests](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:4).

### Primary table bindings

The shared contract requires row, column, revision, and value, and localizes action
labels in English and German:
[revision-addressed table contract](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs:1126).
It creates an editable action binding for every presented cell:
[cell binding factory](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs:1175).

The cell paths are real artifact edit routes, not only generic Details editing:

- CSV and TSV publish revision-addressed bindings:
  [CSV](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:35)
  and
  [TSV](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:33).
- BCF uses the same binding route and checks the submitted revision:
  [Markup main](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:36)
  and
  [BCF reducer](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/✏️editor/🦀️.rs:115).
- EPW binds a per-row revision and validates the target field:
  [EPW main](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:71)
  and
  [EPW editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🦀️.rs:129).
- XLSX binds sheet, row, column, and a cell revision, then validates that address
  before emitting a native cell mutation:
  [XLSX Base main](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:29)
  and
  [XLSX Base editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs:226).

Generic snapshot Details is also registered, for example in
[BCF](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/✏️editor/🦀️.rs:369),
[EPW](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🦀️.rs:169),
and
[XLSX](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs:211).
This fallback is registered; it is not described as unreachable.

## Actionable findings

### P1 — Node-id decoding accepts noncanonical aliases

JSON Base and I-JSON special-case the root but then split all other submitted ids
and discard empty segments and later root segments:

- [JSON Base decoder](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🦀️.rs:50)
- [I-JSON decoder](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🦀️.rs:50)

XML Base and Valid XML use the same discard pattern:

- [XML Base decoder](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs:68)
- [Valid XML decoder](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/✏️editor/🦀️.rs:72)

Thus a rooted descendant and its rootless form have the same retained segments;
doubled separators and repeated roots are normalized away. Empty JSON input also
folds to the root value path. Exact integer parsing still rejects m=01 and 01, but
only after the alias normalization. Existing JSON unit tests expect empty input to
fail:
[Base test](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs:32)
and
[I-JSON test](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🧪️tests/🔬️unit/🦀️.rs:32).

Require one root at segment zero for every non-root address; reject empty segments
and later roots; parse only canonical ordinal segments; then round-trip through the
canonical encoder. Add emit-level tests for rootless, empty, repeated-root, and
doubled-separator inputs in all four profiles.

### P1 — Primary tables cannot create or structurally shape a blank document

The primary windows construct cells only from existing data:

- CSV derives headers from an existing first record or computes width from existing
  records, then renders only data rows:
  [CSV main](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:35).
  The header is a label and has no header-cell edit binding.
- TSV derives width and rows only from existing records:
  [TSV main](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:33).
- BCF projects the existing topics collection:
  [BCF main](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:23).
- EPW projects existing weather records and explicitly excludes eight header lines
  from this window:
  [EPW main](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:1).
- XLSX flattens existing cells and binds only their value column:
  [XLSX Base main](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:29).

All of these primary windows finish by calling the editable-cells table renderer.
Its public output contains only per-cell editable text actions:
[framework renderer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:33878).
An empty shape has no primary editable cell to create its first value, and this
surface publishes no add/remove row/column or header action.

Add revision-addressed native actions for first-row/column creation and structural
add/remove. Add CSV header editing and a header-presence operation. Add a
header-focused EPW primary window. The blank state must have a localized creation
control and runtime coverage through the primary window; Details snapshot editing
does not fulfil this interaction.

### P1 — Editable tables materialize every row and bypass the viewport path

The table view owns complete vectors of rows and columns:
[framework table view](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:33740).
The editable renderer visits every cell, searches the editable list for each one,
serializes all rows, and creates one complete table scene:
[editable table renderer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:33879).
CSV clones every displayed field and TSV clones the complete records before that
call:
[CSV](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:42)
and
[TSV](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:36).

The separate row renderer uses TreeWindows, an accessible table label, and a
requested slice:
[windowed renderer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:33944).
It does not accept per-cell editable bindings, and the primary artifact windows
above invoke the complete editable renderer instead.

Add a public windowed editable-table API with per-cell revisions, viewport request,
accessibility label, progress, and cancellation. Migrate all table artifacts and
prove at runtime that offscreen cells are not serialized before their slice is
requested.

### P2 — Primary table localization and accessibility are incomplete

The action contract is localized, but primary headers are not consistently
localized. CSV and TSV synthesize English-only Column N labels:
[CSV](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:39)
and
[TSV](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:34).
EPW emits raw canonical field names:
[EPW](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:20).
XLSX uses raw sheet, row, col, and value headers:
[XLSX Base](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:29).

The complete editable renderer lacks the label accepted by the windowed renderer.
Static review cannot prove keyboard navigation, conflict-focus restoration,
screen-reader announcements, or viewport usability. Localize generated/artifact
headers, add accessibility semantics to the editable windowed API, and verify
keyboard and screen-reader behavior in English and German at runtime.

### P2 — BCF replaces a complete snapshot for one table-cell edit

After BCF validates the revision and row/column, it clones the whole snapshot and
emits a SetSnapshot mutation for one cell:
[BCF reducer](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/✏️editor/🦀️.rs:115).
This differs from EPW’s field-level and XLSX’s cell-level native mutations. Define
a BCF field-level event with the same revision rule and prove in a retained-event
test that a one-cell edit does not carry the full snapshot.
