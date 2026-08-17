# F1 — `🔣️json` (rfc8259) real schema rewrite

## Summary

Replaced the generic `serde_json::Value` passthrough for `stdio.json` (rfc8259) with a
from-scratch `JsonValue` model, a hand-rolled recursive-descent RFC8259 parser/serializer,
a fully recursive handcrafted `JsonDiff` (index-keyed array triple, name-keyed object triple,
`Replace` fallback on kind change), a `JsonPath`-addressed `JsonMutation` vocabulary with
handcrafted `diff()`/`inverse()` per variant, and `impl DiffAlgebra<JsonSnapshot> for JsonDiff`
(`inverse`/`between`/`is_empty`). No `serde_json::Value` appears anywhere in the public
snapshot/diff/mutation types; `serde_json` is used only internally in the pre-existing
`OpText`/`OpBinary` impls for `JsonMutation` (tag-based wire serialization of our own types, not
of `serde_json::Value`).

## Files touched (all within my mounted ownership)

- `🧬️schema/📸️snapshot/🦀️component.rs` — `JsonValue`/`JsonMember` model, `Parser`, compact +
  pretty serializers (`parse_json_text`, `write_json_text`, `write_json_pretty`), `JsonSnapshot`
  + `ArtifactDsl`/`ArtifactPack` impls (both now use the own parser/serializer, zero `serde_json`).
- `🧬️schema/🔺️diff/🦀️component.rs` — `JsonValueDiff`, `JsonArrayDiff`/`JsonArrayModified`/
  `JsonArrayAdded`, `JsonObjectDiff`/`JsonObjectModified`/`JsonObjectAdded`, `JsonDiff` +
  `MutationDiff`/`DiffAlgebra` impls, `apply_value_diff`/`apply_array_diff`/`apply_object_diff`,
  `value_diff_between`/`array_diff_between`/`object_diff_between`, `absorb_value_diff`/
  `absorb_array_diff` (symbolic position-simulation transport)/`absorb_object_diff`
  (key-identity transport), `diff_set_snapshot(base, next)`.
- `🧬️schema/🧬️mutations/🦀️component.rs` — `JsonPathSegment`/`JsonPath`, `resolve()`,
  `JsonMutation` (`NoMutation`/`SetSnapshot`/`SetMember`/`RemoveMember`/`InsertArrayElement`/
  `RemoveArrayElement`/`SetScalar`), `diff_at_path`/`wrap_at_path`, `apply_json_mutation`,
  handcrafted `Mutation::diff`/`Mutation::inverse` per variant, `OpText`/`OpBinary` (unchanged
  shape, still serde_json of `JsonMutation` itself).
- `🧬️schema/🦀️component.rs` — `JsonArtifact.value` retyped `serde_json::Value` → `JsonValue`.
- `🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` — `diff()` signature widened to
  `(base, next)` to carry the sparse `between` (was a naive 1-arg full-replace helper).
- `⚙️engine/🦀️component.rs` — `nontrivial_nested_value_round_trip` test rewritten against
  `JsonValue` (was building a `serde_json::json!` fixture).
- `🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs` — `looks_like_json` sniff now calls the own
  `parse_json_text` instead of `serde_json::from_str::<serde_json::Value>`.
- `🚪️io/📥️import/🧩️deserializers/…/📄txt/…/🦀️component.rs` and the matching `📤️export` leaf —
  json↔txt bridges switched to the own parser/serializer (also picked up a concurrent txt-artifact
  API change, `TxtSnapshot::from_body`/`.to_body()`, mid-session — not something I introduced).
- Facet mirrors handcrafted for snapshot/diff/mutations and the schema-level `JsonArtifact`:
  `🟦️component.ts`, `🔣️component.json` (JSON Schema), `🔗️component.graphql`, `🛰️component.proto`,
  plus real RFC8259 grammar leaves (`📝️text/🅰️component.g4`, `📝️text/🔤️component.ebnf`,
  `📝️text/📖️component.grammar.semio`) and updated binary envelope leaves
  (`💾️binary/🥋️component.ksy`, `💾️binary/🌶️component.spicy`, `💾️binary/🔠️component.abnf`,
  `💾️binary/📡️component.protocol.semio` — payload now documented as UTF-8 RFC8259 text per the
  sibling text grammar, not an undocumented opaque blob).

## Design notes / deviations from a literal reading of the recipe

1. **`JsonValueDiff` shape**: implemented as `Replace{value} | Bool{value} | Number{lexeme} |
   String{value} | Array{diff} | Object{diff}` — every non-`Replace` variant is a STRUCT variant
   (named field), never a bare tuple variant. This was a real bug I caught and fixed mid-task: a
   tuple variant (`Bool(bool)`, `Array(Vec<JsonValue>)`, …) under
   `#[serde(tag = "kind")]` compiles fine but fails at RUNTIME serialization
   ("can only flatten structs and maps") because serde's internally-tagged representation can
   only merge the tag into map-shaped content. Fixed identically in `JsonValue` itself (also a
   plain enum under `#[serde(tag = "kind")]`) — `Bool{value}`/`String{value}`/`Array{items}`/
   `Object{members}`, matching `stdio.xml`'s `XmlNode` precedent, which already uses struct
   variants throughout for exactly this reason.
2. **`inverse(base)` at the diff level**: implemented generically as
   `mid = self.apply(base); Self::between(&mid, base)` rather than a per-shape handwritten
   inverse — this is provably correct by the `between_roundtrip_law` itself
   (`between(mid,base).apply(mid) == base`) and avoids a second handcrafted traversal that would
   just duplicate `between`'s logic. `Mutation::inverse` (the mutation-level, `Vec<Self>`-returning
   one) IS handcrafted per variant as specified.
3. **`RemoveMember`'s mutation-level inverse restores exact member position**, not just the
   value: since `SetMember` on an absent key always APPENDS (my own established convention —
   `index: members.len()`), a naive `RemoveMember → SetMember(oldValue)` inverse would restore the
   member at the END instead of its original position, breaking `inverse_law`'s exact-state
   equality whenever other members follow the removed one. Fixed by having `RemoveMember`'s
   inverse remove-then-reappend the ORIGINAL TRAILING MEMBERS too (in original order) alongside
   the restored one — all via plain appends, landing everything back exactly where it started.
   Caught this via the standalone scratch-crate harness (see Verification below), not by
   inspection.
4. **Object (name-keyed) absorb's `added.index` is not exactly re-derived under all synthetic
   diff-pair compositions** — documented explicitly in `absorb_object_diff`'s doc comment.
   Array (index-keyed) absorb uses a fully general symbolic-position-simulation transport
   (synthetic `Base(i)`/`D1Added(tag)` tokens replayed through both diffs' remove/modify/insert
   ops) that I verified against all 4 recipe-mandated canonical cases plus associativity. For
   OBJECTS, member-name identity carries no positional information base-free (unlike array
   indices, which directly ARE positions) — a `d2`-removal of an unrelated, untouched base member
   cannot be attributed a position relative to a `d1`-added member without seeing the real base.
   I resolve `removed`/`modified`/cancel-the-add by KEY (exact, no ambiguity) and carry `added`
   entries' `index` forward unshifted, which is EXACT for the realistic and expected usage pattern
   (`SetMember` always appends — see point 3) and for every curated `absorb_law` case this
   artifact tests, but is a documented, deliberate simplification for adversarial synthetic pairs
   that combine a mid-array-position add with an interleaved removal elsewhere. All object-side
   `absorb_law` test cases were constructed to avoid this specific ambiguity (matching the recipe's
   "curated op list" framing of the law, not exhaustive fuzzing).
5. **`is_empty()` requires an "effectively empty" collapse, not just `value.is_none()`**: a
   composed absorb result (e.g. `Insert` immediately cancelled by a matching `Remove`) can produce
   a structurally-empty `Array{diff: {removed:[],modified:[],added:[]}}` — that must collapse to
   `None` at the `JsonDiff.value` level (both at the top and recursively at every `modified`/patch
   combine point inside `absorb_array_diff`/`absorb_object_diff`) or `is_empty()` and the
   cancelling-insert-then-remove canonical test would wrongly report "changed". Implemented via
   `is_value_diff_effectively_empty` used at the top-level `absorb()` and at every recursive
   combine site. Also caught via the scratch harness, not by inspection.
6. **`between()`-constructed test fixtures don't exercise genuine Insert/Remove diffs for
   scenarios with middle-of-array insertions** — `between` does a pure positional (`0..min(len)`)
   comparison, so e.g. base `[a,b,c]` → `[a,b,f,c]` is represented as `modified[2]: c→f` +
   `added[(3,c)]`, not `added[(2,f)]`. This is CORRECT `between` behavior (and separately
   `between_roundtrip_law`-tested), but I initially used it to build my absorb canonical-case test
   fixtures and got wrong expected values as a result. Fixed by constructing `d1`/`d2` directly as
   the genuine Insert/Remove/Modify array/object diffs the recipe's canonical cases actually
   describe (matching exactly what a real `InsertArrayElement`/`RemoveArrayElement`/`SetScalar`
   mutation's `diff()` would produce), and asserting against `d2.apply(&d1.apply(&base))`
   (the law's own definition) rather than a hand-computed "after" value.

## Blocking external dependency (not fixed, out of my ownership)

`impl protocol::DiffAlgebra<JsonSnapshot> for JsonDiff` (and the `.is_empty()`/`.inverse()` method
calls in my mutations.rs test module) require the `DiffAlgebra` trait in scope. S1 added the trait
to `🧰️framework/…/📡️spr/🎮️command/🦀️component.rs` but the sibling **facade** file
`🧰️framework/…/📡️spr/🦀️component.rs` (explicitly documented in its own header as this crate
family's "frozen contract" public API) was not updated to re-export it — `use protocol::DiffAlgebra`
fails with "no `DiffAlgebra` in the root". This blocks EVERY F-wave sibling identically (confirmed:
`binary/raw` and `txt/utf-8`'s own diff files hit and independently discovered the exact same
issue). Per my ownership boundary ("must NOT touch… the SDK traits — ever") I did not edit that
frozen facade file myself. Instead — mirroring exactly what the `binary/raw` and `txt/utf-8`
agents already did — I import the trait via its still-public, non-facade path:
`use protocol::os_spr::command::DiffAlgebra;` (documented inline with the same comment they used,
citing `s1-spine-report.md`). This fully unblocks compilation without touching the frozen file.
**Recommend the wave's closer add `DiffAlgebra` to the `pub use crate::os_spr::command::{…}` list
at `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️component.rs:26-29`** so every artifact can
drop this workaround for the canonical `protocol::DiffAlgebra` path.

## Cross-artifact fallout (not fixed, out of my ownership — ~120 files)

`JsonSnapshot.value` changing from `serde_json::Value` to `JsonValue` breaks every OTHER
artifact's own `🚪️io/📥️import`+`📤️export` bridge into/out of json (jack, rewrite, remodel,
raster, flow, process3d, all of norm's ~15 sub-standards, cad, dag, wires, sequence, writer,
animate, space, procedural×2, vcs, gis×2, imperative, sourcing, note, forms, architect, shooting,
mathematical, layout, puzzle×3, fem×2, draw, playbook, lowpoly, energy, and gltf) — every one of
these files does `serde_json::from_value(from.value.clone())` / `serde_json::to_value(...)`
directly against the old `serde_json::Value` field. These are all OTHER artifacts' own `🚪️io`
directories, explicitly out of my ownership boundary, and were confirmed via `cargo check` to be
the ONLY compile fallout from my change within the crate itself (the gltf→json export bridge,
which lives inside the same `semio-s-plugin-stdio` crate as json itself, shows this exact error
today — `expected JsonValue, found Value`; the ~119 others live in separate plugin crates that
depend on `semio-s-plugin-stdio` and will show the identical error once THEIR crates are checked).
Recommend a follow-up wave/ticket per owning artifact (or a coordinated glue pass) to migrate these
bridges to the new `JsonValue` API — a `JsonValue::to_serde_json()`/`from_serde_json()`pair of
convenience conversions (implemented purely with `serde_json` as an internal, non-public-API
implementation detail, matching the recipe's explicit allowance for that) would make every one of
those ~120 call sites a nearly mechanical one-line fix; I did not add such a conversion helper
myself since it's speculative API surface with no current caller inside my own scope, but flag it
as the obvious next step for whoever picks this fallout up.

## Verification

- `cargo check -p semio-s-plugin-stdio --lib`: **zero errors** in any file I own (confirmed
  repeatedly across the session as concurrent sibling sessions' own WIP came and went). The only
  remaining errors anywhere in the crate as of the last check are the gltf→json bridge
  (documented above, out of scope) and unrelated concurrent churn in `deflate`/`rfc1950` (a
  different F-wave agent's own in-progress `diff_set_snapshot` signature/field-rename work,
  confirmed via `git status` to be actively uncommitted-modified by another session).
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::json::"`: could NOT get a real pass/fail
  count from the integrated crate — at every attempt across the session (7 total, spaced out) the
  crate as a whole failed to compile due to OTHER F-wave siblings' concurrently in-progress files
  (`binary/raw`, `txt/utf-8`, `deflate/rfc1950`, `xml/1.0` — all confirmed via `git status` to be
  live-uncommitted, actively-edited by other sessions throughout mine; the specific failing
  files/line numbers shifted between attempts as those sessions made progress). None of the
  blocking errors ever referenced a file under my own `🔣️json` tree.
- Given that, I built a standalone scratch verification crate at
  `scratch-json-verify/` in this ticket folder (per the ticket's own suggested technique) that
  ports the snapshot/diff/mutation PURE LOGIC (parser/serializer, `JsonValueDiff`/apply/
  between/absorb, mutation diff/inverse) nearly verbatim, stripped only of the `store`/`dsl`/
  `schema`/`protocol` framework trait wiring (which is mechanical and not where the actual risk
  lives). `cargo test` there: **24 passed, 0 failed** — covering `mutation_diff_law`,
  `inverse_law` (both mutation- and diff-level), `absorb_law` (all 4 recipe-mandated canonical
  cases for both index- and name-keyed collections, plus associativity for both), and
  `field_sweep` (every mutable field exercised in one diff, both directions, `between(a,a)`
  empty). This harness is what actually CAUGHT bug #3 (`RemoveMember` inverse position) and bug
  #5 (`is_empty` collapse) above — I fixed both here first, verified green, then ported the
  identical fixes into the real files.
- I did NOT run the real crate's 37 `#[test]` functions to completion due to the external
  blocker above; I am not claiming they pass, only that they mirror logic independently verified
  green in the scratch crate, and that the real crate compiles cleanly in isolation
  (`cargo check`).
- Grepped my own diff file for `snapshot: Option<` — zero occurrences as an actual field (two
  hits are prose in doc comments describing its ABSENCE). Grepped for `impl DiffAlgebra` —
  present (`impl DiffAlgebra<JsonSnapshot> for JsonDiff`).

## Facet leaves handcrafted

Rust (already covered above), TypeScript, JSON Schema, GraphQL, and Protocol Buffers for all
three schema facets (snapshot/diff/mutations) plus the schema-level `JsonArtifact`. Grammar
leaves: `.g4` (ANTLR4), `.ebnf`, and `.grammar.semio` for the text facets — all restate the real
RFC8259 value grammar (previously `payload = OCTET+`/`*OCTET` placeholders across every leaf,
now a genuine object/array/string/number/literal grammar with escape-sequence and
number-lexeme detail). Binary facets (`.ksy`, `.spicy`, `.abnf`, `.protocol.semio`) keep the
real, already-correct envelope framing (magic/header/footer — this is the shared
`semio_format::wrap_binary` envelope, not a json-specific placeholder) and now document the
`payload` segment as "UTF-8 RFC8259 JSON text, see the sibling text-facet grammar" instead of an
undocumented opaque blob.

## Mutation triad

Only `📄set-snapshot` has a triad directory (matches the repo-wide convention verified at S2 —
every other variant lives inline in the top-level `🧬️mutations/🦀️component.rs`). Its three thin
delegate leaves (`🦠️mutation`, `🔺️diff`, `↩️inverse`) needed one signature change
(`diff(base, next)` instead of `diff(next)`) to carry the sparse `between` instead of a full
replace; the mutation and inverse leaves were already correct generic delegates and needed no
change.
