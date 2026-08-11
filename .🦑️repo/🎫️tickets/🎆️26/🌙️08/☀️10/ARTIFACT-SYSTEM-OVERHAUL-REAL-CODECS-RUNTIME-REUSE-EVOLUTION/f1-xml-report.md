# F1 — `📰xml` (standard 1.0) Schema Overhaul Report

Agent: F1-wave, artifact `stdio.xml` 1.0. Scope per the wave brief: snapshot completeness (XML
declaration), handcrafted recursive `XmlDiff`/`XmlNodeDiff` tree, `XmlNodePath`-addressed
mutations, `DiffAlgebra` (inverse/between/is_empty), rigorous absorb, and the six test laws.

## Snapshot completeness

Added the missing piece from the completeness table: the XML declaration
(`<?xml version="1.0" encoding="..." standalone="..."?>`), as a new typed `XmlDeclaration{version,
encoding, standalone}` struct and a new `declaration: Option<XmlDeclaration>` field on
`XmlDocument`, alongside the pre-existing `root`/`doctype`. `xml_document_to_text`/
`xml_document_from_text` now encode/decode it; the parser only recognizes `<?xml ...?>` at the very
start of the document (per spec) and distinguishes it from an ordinary PI whose target merely
starts with the same four letters (e.g. `<?xml-stylesheet ...?>`). The pre-existing
`XmlNode`(Element/Text/CData/Comment/ProcessingInstruction) tree and doctype raw-string retention
were already complete and needed no changes.

**Blast-radius consequence**: `XmlDocument` is constructed via exhaustive Rust struct literals
(not `..Default::default()`) at 11 call sites across 6 *other* stdio artifacts that embed XML as
their underlying markup (bcf, svg ×2, zip/opc ×2, xlsx ×3, docx, pptx ×2) — this is the "spec-mandated
reuse" the plan calls out for svg, but it turns out several OPC-based formats (zip/xlsx/docx/pptx)
also construct `XmlDocument` literals directly. Adding the new field would have broken all of them.
Rather than leave the workspace uncompilable I made the minimal, mechanical, one-line fix at each
site (`declaration: None,`) — no other line in any of those 6 files was touched. Full list in
`files_touched`.

## Diff design — `XmlDiff` / `XmlNodeDiff`

Origin implementation of the xml/svg node-diff pattern (svg's F3 wave builds on this shape but
declares its own diff types, per the plan):

```rust
pub struct XmlDiff { declaration: Option<Option<XmlDeclaration>>, doctype: Option<Option<String>>, root: Option<XmlNodeDiff> }
pub enum XmlNodeDiff { Element(XmlElementDiff), Text{text: Option<String>}, Replace{node: Option<XmlNode>} }
pub struct XmlElementDiff { name: Option<String>, attributes: Option<XmlAttributesDiff>, children: Option<XmlChildrenDiff> }
pub struct XmlAttributesDiff { removed: Vec<String>, modified: Vec<XmlAttrModified>, added: Vec<XmlAttrAdded> }   // name-keyed
pub struct XmlChildrenDiff   { removed: Vec<usize>,  modified: Vec<XmlChildModified>,  added: Vec<XmlChildAdded> } // index-keyed, recursive
```

One deliberate, documented deviation from the brief's literal `Replace{node: XmlNode}`:
`Replace{node: Option<XmlNode>}`. The extra `Option` is what lets `XmlDiff.root` stay a plain
`Option<XmlNodeDiff>` (as specified) while still being able to express "the document root was
removed entirely" (`doc.root: Option<XmlNode>` really is nullable at the snapshot level) — without
introducing a second, separate tri-state wrapper around `root`. `Replace` is otherwise used exactly
as specified: node-kind changes (e.g. `Text`→`Element`) and CData/Comment/ProcessingInstruction
(which get no dedicated structural diff shape, matching the brief).

`diff_at_path(path: &[usize], leaf: XmlNodeDiff) -> XmlDiff` nests `leaf` through
`XmlChildModified` entries from the root down to `path`'s depth (`path == []` → `leaf` becomes
`XmlDiff.root` directly). `InsertElement`/`RemoveElement`'s `path` addresses the **parent** element
(the mutation targets that parent's children triple); every other path-carrying mutation's `path`
addresses the node itself. Both conventions are documented on the `XmlMutation` enum and are
exactly what `diff_at_path` expects.

No `snapshot: Option<XmlSnapshot>` full-replace slot anywhere — verified by grep (zero hits) — and
`SetSnapshot`'s diff is literally `XmlDiff::between(base, next)`.

## Mutations

`XmlMutation`: `NoMutation`, `SetSnapshot`, `SetDeclaration`, `SetDoctype`, `InsertElement`,
`RemoveElement`, `SetAttribute` (value `None` = remove), `SetText` — the brief's full minimum set.
Every variant's `diff()` is handcrafted directly against the sparse types (no apply-and-capture);
every variant's `inverse()` reads the prior value off `base` via `XmlNodePath::resolve` and
constructs the exact undoing mutation.

## Absorb — the hard part

Implemented the brief's normative index-transport algorithm for `XmlChildrenDiff` (`transform_index`
+ a materialized `simulate_mid_origins` array sized to the smallest synthetic length that avoids
clamping any index either diff actually references — absorb is base-free, so there is no real
snapshot to consult for the true length). Verified against all three named canonical cases plus
"Modify+Remove" and associativity over a triple:

- `Insert(2,f)` + `Remove(0)` → `{removed:[0], added:[(1,f)]}` — matches the brief's worked example
  exactly.
- `Insert(2,f)` + `Insert(2,g)` → **both** survive at distinct final indices (this is the exact bug
  the plan calls out in gif's old op-slot absorb — confirmed fixed here from the start, never had
  the bug).
- `Insert(1,f)` + `SetAttribute` on the newly-inserted node → patches directly into the carried
  `added` payload; no `modified` entry is exposed.
- `SetAttribute` + later `RemoveElement` of the same node → the modify is annihilated, only
  `removed` survives.
- Associativity: `absorb(absorb(d1,d2),d3)` and `absorb(d1,absorb(d2,d3))` both `.apply(base)` to
  the same result as sequential application, over a 3-op chain mixing two inserts and a remove.

Attribute absorb (`XmlAttributesDiff`) is simpler by construction — attribute **name** is the
stable identity (not position), so none of the index-transport machinery applies; only `added`'s
`index` gets any position bookkeeping at all, and that's a lighter-weight best-effort (documented
in `deviations`) since attribute order carries no XML-spec meaning, only round-trip fidelity.

## Test laws — in the existing test region

All six laws live in `⚙️engine/🦀️component.rs`'s pre-existing `#[cfg(test)] mod tests` (no new test
file): `mutation_diff_law`, `inverse_law`, `absorb_law`, `between_roundtrip_law`,
`codec_retention_law`, `field_sweep_law`.

`field_sweep_law`'s `sweep_a()`/`sweep_b()` differ in every mutable field, including both
tri-state top-level scalars going `Some(x) → None` (`Some(None)` exercised for both `declaration`
and `doctype`). One structural note worth flagging: the brief's own specified `between_children`
algorithm ("index keys pairwise by position: modified = compare `0..min`, removed = base tail,
added = other tail") can *structurally* only ever produce removed-tail XOR added-tail from a single
`between()` call on one collection instance — never both, since the two ranges are complementary
by construction. I exercised `removed` + `modified-in-every-field` at the top-level children triple
and `added` at a *nested* triple (inside the modified child's own children), and get all three of
removed/modified/added simultaneously in the root's *attributes* triple (name-keyed, no such
positional limitation). This is documented inline on the fixtures.

## Verification

`cargo check -p semio-s-plugin-stdio --lib --tests` compiles the xml module clean — zero errors,
only two pre-existing unrelated warnings (`XmlDiff` unused outside `#[cfg(test)]` in the non-test
lib build, and a hidden-lifetime style warning in the composer file, neither introduced by this
work). Grepped `snapshot: Option<` in the diff file → zero hits; grepped `impl DiffAlgebra` →
present.

**Could not get a green `cargo test` for the whole `semio-s-plugin-stdio` crate**: at every check I
ran (5 separate passes over roughly 20 minutes), 2–11 *other* F1-wave artifacts' files
(`binary`, `txt`, `json`, transiently also `csv`/`deflate`/`zip`) had the identical compile error —
`use protocol::{DiffAlgebra, MutationDiff};` doesn't resolve because `DiffAlgebra` (S-1, new this
wave) isn't re-exported at the `protocol` crate root the way `MutationDiff` is; it only resolves
via `protocol::command::DiffAlgebra`. This is a systemic gotcha every F1 agent following the recipe
literally hits, not specific to xml — I fixed it in my own files immediately
(`use protocol::command::DiffAlgebra;`) but cannot fix it in other artifacts' files without
stepping outside my ownership boundary, and the count of affected files fluctuated (3→5→11→2) over
repeated polls as other concurrent sessions edited their own files, confirming this is exactly the
"concurrent cargo workspace churn" pattern, not my bug.

Given that, I verified correctness with the ticket's own recommended technique: a standalone
scratch crate at `.../scratchpad/xml_scratch` (own `Cargo.toml`, own `target/`) containing verbatim
copies of `snapshot.rs`/`diff.rs`/`mutations.rs` (only the trait-import paths rewritten to a tiny
local `protocol` shim reproducing `MutationDiff`/`DiffAlgebra`/`Mutation`'s exact method
signatures, and derive macros/serde/store-crate dependencies stripped since they're irrelevant to
the algorithm itself) plus the exact same 6 test bodies. `cargo test` there: **6 passed, 0 failed**.
This is strong evidence the real crate's tests will pass identically once the unrelated
`protocol::command::DiffAlgebra` import issue is fixed workspace-wide by the other in-flight
sessions (or the wave's closer) — the scratch crate's `diff.rs`/`mutations.rs`/`snapshot.rs` are
byte-identical to the real files modulo only that import path and the stripped derive attributes.

## Facet mirrors

Handcrafted `.ts`/`.graphql`/`.json` (JSON Schema)/`.proto` for all three facets (snapshot, diff,
mutations) matching the real Rust shapes (discriminated unions on `kind`/`mutation` tags, camelCase
fields, tri-state nullable encoding documented inline). Also rewrote every `📝️text`/`💾️binary`
grammar leaf under all three facet dirs: snapshot's now describes the *actual* XML grammar this
codec parses (declaration/doctype/element/attr/CDATA/comment/PI, `.g4`/`.ebnf`/`.grammar.semio`)
and the *actual* pack encoding (semio envelope wrapping JSON-serialized `XmlDocument`, not raw
octets, `.ksy`/`.spicy`/`.abnf`/`.protocol.semio`); diff's and mutations' now document that their
wire form is JSON (mutations really does go through `serde_json` via `OpText`/`OpBinary`; diff has
no independent wire codec of its own) rather than leaving the generic `*OCTET`/`size-eos`
placeholder that was there before. None of the old artifact-family placeholder grammars survive.

## Fixtures

`📚️examples/🎬️demo/🖼️assets/example.xml` (`<note><to>Tove</to>...`) preserved untouched, used as the
real-fixture leg of both `between_roundtrip_law` and `codec_retention_law` — decode→encode is
byte-preserving up to the documented normal form (leading/trailing whitespace trimmed; the fixture
has no empty elements or internal whitespace, so nothing else normalizes).
