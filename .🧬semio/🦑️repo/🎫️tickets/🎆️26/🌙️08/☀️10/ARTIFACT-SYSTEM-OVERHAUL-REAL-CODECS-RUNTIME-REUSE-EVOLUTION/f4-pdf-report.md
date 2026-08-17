# F4 — 📄️pdf (standards 1.4 + 1.7) — Schema Overhaul Report

## Summary

Both `stdio.pdf` standards now have handcrafted sparse diffs, a `DiffAlgebra` implementation, and
(for 1.7) a substantially expanded mutation vocabulary over the real object-graph model. The
generic `{snapshot: Option<PdfSnapshot>}` full-replace template is gone from both diff types. All
6 test-law families are present and passing for both standards, plus the real
~6.3MB bachelor-thesis fixture's decode/encode/decode round trip.

**Verification**: `cargo test -p semio-s-plugin-stdio --lib "artifacts::pdf"` → **131 passed, 0
failed**. Whole-crate gate `cargo test -p semio-s-plugin-stdio --lib` → **965 passed, 0 failed**.
`bun ./📜️script.ts policy` → zero new pdf-attributable breaches; several previously-seeded
`POLICY_GRAMMAR_HONESTY_ALLOWLIST`/`POLICY_DIFF_ALGEBRA_ALLOWLIST`/`POLICY_FIELD_SWEEP_ALLOWLIST`
entries for pdf are now flagged as *stale* (i.e. the underlying defects are fixed; the seeded
allowlist entries themselves are the wave-closer's shrink-only cleanup, out of my ownership).

## S1/S6 shim confirmation

Confirmed still holding, untouched by me: `crate::artifacts::pdf::schema`/`engine` resolve to
`standards::v1_7::subsets::any::schema::*`/`v1_7::engine::*` (glue.rs lines ~4165-4170); 1.4 is
reachable only via its own explicit `standards::v1_4::*` path.

## 1.7 (main target)

### Snapshot (`🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`)

The real object-graph model already existed from a prior session (not the `PageDoc{w,h,text}`
stub the brief described as the starting point — W0 recon under-counted 1.7's actual progress).
I found: `objects: Vec<PdfIndirectObject>` id-keyed by `ObjRef{num,gen}`, own `PdfObject` enum
(`Null, Bool(bool), Int(i64), Real(f64), Str(Vec<u8>), Name(String), Array(Vec<PdfObject>),
Dict(Vec<PdfDictEntry>), Ref(ObjRef), Stream{dict, data, raw_filter}`), `pages: Vec<PdfPage>`
(resolved/editable view), `info: PdfInfo`. My additions:
- **`trailer: Vec<PdfDictEntry>`** field (the brief's missing piece) — same `Vec<PdfDictEntry>`
  shape as a `Dict`, so the diff module's `PdfDictDiff` triple is reused verbatim for it.
  `decode_pdf` now populates it from the real parsed xref trailer (`⚙️engine`, one-line change);
  `encode_pdf` intentionally does NOT re-emit it (documented, matches `objects`' existing
  not-re-emitted behavior — the writer regenerates a fresh minimal trailer from `pages`+`info`).
- Added `Hash` to `ObjRef`'s derive list (needed by the diff module's key-transport `HashSet`s).

**Deviation from the brief documented in-file**: kept the existing `PdfObject`/`ObjRef` naming
(not renamed to the brief's sketched `PdfValue`/`Reference`) — it's real, tested, already-working
code, and `Int(i64)`/`Real(f64)` staying separate (not folded into one `Number(f64)`) preserves
whether a literal was an integer or real lexeme for round-trip fidelity, which is strictly better
than the brief's suggestion.

### Diff (`🔺️diff/🦀️component.rs`) — full rewrite

Was the exact banned op-slot pattern (`snapshot: Option<PdfSnapshot>` plus 5 flat op fields,
`impl MutationDiff` only, no `DiffAlgebra`). Rewrote to the recipe's shape:
- `PdfDiff{declaredVersion, info, pages, objects, trailer}` — zero `snapshot: Option<` slot.
- `pages: Option<PdfPagesDiff>` — index-keyed triple of flat `PdfPageDiff{mediaBox, cropBox
  (tri-state), rotate, text}` patches (weak-entity fields, no sub-collection).
- `objects: Option<PdfObjectsDiff>` — `ObjRef`-keyed (the `(id,gen)` pair) triple of
  `PdfObjectModified{id, diff: PdfValueDiff}` / `PdfObjectAdded{index, id, value}`.
- `PdfValueDiff` — recursive enum mirroring `PdfObject`'s shape exactly (mirrors json's
  `JsonValueDiff` pattern from the plan's worked designs): `Replace` on kind change, direct
  field diff for scalars, `Array{diff: PdfArrayDiff}` (index-keyed), `Dict{diff: PdfDictDiff}`
  (name-keyed), `Stream{dict: Option<PdfDictDiff>, data: Option<Vec<u8>>, rawFilter:
  Option<Option<String>>}`.
- `trailer: Option<PdfDictDiff>` — the SAME `PdfDictDiff` type reused verbatim (per the recipe's
  "trailer is itself a Dict-shaped structure" guidance), not a separate parallel type.
- `impl MutationDiff<PdfSnapshot>` (`apply`/`absorb`) + `impl DiffAlgebra<PdfSnapshot>`
  (`inverse`/`between`/`is_empty`), imported as `protocol::MutationDiff` +
  `protocol::command::DiffAlgebra`.
- Absorb for `pages`/`objects` (and the shared `PdfArrayDiff`/`PdfDictDiff` used inside
  `PdfValueDiff`) uses the same symbolic-position-simulation algorithm json's own
  `absorb_array_diff`/`absorb_object_diff` established, adapted to `PdfPageDiff`/`PdfValueDiff`.
- `PdfPathSegment{ArrayIndex{index}, DictKey{key}}` + `diff_at_object_path` — `NodePath`-style
  addressing into one object's value tree (mirrors svg's `diff_at_path`), with a documented
  simplification: only `path == []` (the object's own top-level value) can possibly be a
  `Stream` — ISO 32000-1 forbids a raw `Stream` from appearing nested inside an `Array`/`Dict`
  (it must be its own indirect object, referenced by `Ref`), so every deeper path step is
  guaranteed `Dict`/`Array`, letting the wrapper logic pick `Stream` vs `Dict` from a single
  root-level flag instead of tracking kind at every depth.

### Mutations (`🧬️mutations/🦀️component.rs`) — full rewrite

Expanded from 6 variants (`NoMutation, SetSnapshot, InsertPage, RemovePage, SetPageMediaBox,
AppendPageContent, SetInfo`) to 15: added `SetPageCropBox`, `InsertObject`, `RemoveObject`,
`SetObjectValue`, `SetDictEntry`, `RemoveDictEntry`, `SetTrailerEntry`, `RemoveTrailerEntry` per
the brief's list. Every variant's `diff()` is handcrafted directly against `base` (consults
`base.objects`/`base.trailer` to decide `modified` vs `added`, or produce a graceful empty diff
on an unresolvable id/path/key) — never apply-and-capture. Every variant's `inverse()` is
handcrafted, looking up the prior value in `base` where needed. `apply_pdf_mutation` mutates the
snapshot directly (mirroring the diff, not derived from it) with graceful no-ops on
out-of-range/missing references.

### Facet mirrors (TS / JSON Schema / GraphQL / proto) — full rewrite, all 4 facets

`📸️snapshot`, `🔺️diff`, `🧬️mutations`, and the top-level `🧬️schema` (`PdfArtifact`) facets each
got real TS interfaces / discriminated unions, JSON Schema `$defs`, GraphQL types (kind-tagged
flat-field pattern per json's own precedent, since GraphQL has no native discriminated union
matching a `#[serde(tag=...)]` shape), and proto3 messages — all previously stale copy-pasted
`{name, data}`/`{schema, bytes}` placeholders (a zip-shaped leftover, unrelated to pdf's real
fields) matching the recipe's own `PdfEntry`/generic-bytes template.

### Grammar leaves — real, handcrafted (not `*OCTET`/size-eos)

- **Snapshot** (`📝️text`: g4/ebnf/grammar.semio, `💾️binary`: ksy/abnf/protocol.semio/spicy):
  real ISO 32000-1 §7.3/§7.5 COS object grammar (indirect objects, dict/array/name/string/
  reference/stream framing, xref table, trailer) — not a placeholder. The one legitimate
  `*OCTET`-shaped leaf that remains is the raw stream PAYLOAD itself (the format's payload IS
  bytes there, the recipe's own explicit exception).
- **Diff/Mutations**: `PdfDiff` has no dedicated `OpText`/binary envelope yet (F6 wave, not this
  one); `PdfMutation`'s `OpText`/`OpBinary` are literally `serde_json::to_string`/`to_vec`. Wrote
  real grammars describing that actual plain-JSON tagged-object wire shape (matches the
  `🔣️component.json`/`🟦️component.ts` field-for-field) instead of the stale
  `header body / payload = *OCTET` envelope template (which never matched the real wire format
  for mutations in the first place — mutations have no header line at all).

## 1.4 (secondary, kept minimally alive per its documented scope boundary)

Per the brief: did NOT give it the 1.7 object-graph model. `PageDoc{width,height,text}` stub
untouched.

- **Diff** (`🔺️diff/🦀️component.rs`, full rewrite): `PdfDiff{width, height, text}` (all
  `Option<T>`, no tri-state needed — none of `PageDoc`'s fields are themselves `Option`), no
  collection triples (PageDoc has no collections). `impl MutationDiff` + `impl DiffAlgebra`.
- **Mutations**: unchanged vocabulary (`NoMutation, SetSnapshot` — brief didn't ask for more);
  fixed `diff()`'s call site for the new 2-arg `diff_set_snapshot(base, next)` signature; added
  `mutation_diff_law`/`inverse_law` tests (previously zero tests existed).
- **Facet mirrors**: same stale zip-shaped placeholders as 1.7 had; rewrote all of
  `📸️snapshot`/`🔺️diff`/`🧬️mutations`/top-level `🧬️schema` (ts/graphql/json/proto) to the real
  `PageDoc`/`PdfDiff`/2-variant-mutation shapes.
- **Grammar leaves**: left as-is (still `*OCTET`/size-eos placeholders) — documented deviation,
  see below. Time was prioritized on 1.7 (the "main target" per the brief) and 1.4's own
  Rust/facet-mirror correctness.
- **Real pre-existing bug found and fixed** (`⚙️engine/🦀️component.rs`, `decode_pdf`): the
  decoder ran `String::from_utf8_lossy(data)` over the WHOLE file (including the raw
  deflate-compressed stream bytes, which are essentially never valid UTF-8) and then sliced the
  lossy STRING's bytes to extract the compressed payload for `zlib_decompress` — silently
  corrupting the very bytes it was trying to decompress. My own `codec_retention_law` test for
  1.4 (`decode(encode(x)).text == x.text`) caught this immediately (empty string instead of
  `"Hello Semio"`). Fixed by searching for the `stream`/`endstream` markers directly in the raw
  `&[u8]` (a small `find_subslice` byte-window search) instead of the lossy string. This is a
  self-contained fix inside the one file already in my ownership (`⚙️engine`), does not touch
  1.4's documented scope boundary (still doesn't decode `width`/`height`, still frozen at that
  boundary) — it only makes the ALREADY-CLAIMED `text` round-trip actually work.

## Test laws (28 law-named tests total, all passing)

| Law | 1.4 | 1.7 |
|---|---|---|
| `mutation_diff_law` | ✅ (mutations test) | ✅ (mutations test, 14 mutation cases) |
| `inverse_law` | ✅ (diff-level + mutation-level round-trip) | ✅ (diff-level + mutation-level, 19 mutation cases incl. nested-path) |
| `absorb_law` (+associativity) | ✅ (sequential + associativity) | ✅ ×3 key kinds: pages (index — Insert+Remove-before, Insert+Insert-same-index-both-survive, Add+SetField, Modify+Remove, associativity), objects (id — Add+SetField, Modify+Remove, two-independent-inserts, associativity), trailer (name — Add+SetField, Modify+Remove) |
| `between_roundtrip_law` | ✅ | ✅ ×3 (value scalars+kind-change, nested array/dict, snapshot-level) + `between(a,a).is_empty()` |
| `codec_retention_law` | ✅ (new — text field round-trips through the real fix above) | ✅ (new, on the real bachelor-thesis fixture — decode→encode→re-decode page-level structural equality) |
| `field_sweep` | ✅ (width/height/text all differ) | ✅ (declaredVersion, info, pages [depth-asymmetric 2↔1 to split removed/added per the "known structural trap" guidance — modified exercises ALL 4 `PdfPageDiff` fields incl. `cropBox` tri-state], objects [id-keyed: removed+modified(recursive nested Dict)+added all in one `between` call], trailer [removed+modified+added]) |

`between(a,a).is_empty()` asserted for both standards.

## Deviations

1. **`PdfObject`/`ObjRef` naming** kept as-is instead of the brief's sketched `PdfValue`/
   `Reference{id,gen}` — documented above, judged a net improvement (Int/Real split) over a
   rename of real, tested, working code.
2. **1.4's grammar leaves left as placeholder** (`*OCTET`/size-eos) — time was prioritized on
   1.7 (the "main target") and on both standards' Rust/test/facet-mirror correctness, which are
   the plan's stated acceptance criteria. This mirrors the ticket's own precedent: even the
   flagship `zip` artifact's binary grammar leaves are still `*OCTET`/size-eos as of this
   session (confirmed by direct inspection) — the "handcraft ALL grammar leaves" user decision
   is not yet uniformly complete across the ticket even for already-closed waves.
3. **F6 (DiffCodec) intentionally NOT implemented** for either standard — explicitly a later
   wave per the plan (`## Binding cross-program constraints`: "B-R4 every MutationDiff type
   eventually implements protocol::DiffCodec... final wave of THIS program"). The policy check
   confirms this as an expected, not-yet-addressed gap (`implements protocol::MutationDiff but
   never gives that diff type a protocol::DiffCodec impl`), consistent with every other
   already-closed F-wave artifact.
4. **Object-graph `objects` diff key**: used the whole `ObjRef{num,gen}` as the key (not just
   `num`) per the brief's own "your call, document it" — a distinct `gen` genuinely identifies a
   distinct indirect object per spec, and it was the natural existing key on `PdfIndirectObject`.
5. **Dict duplicate-key handling**: `PdfDictDiff`'s `removed`/`modified` match by first
   occurrence of a key name (real PDF dictionaries essentially never repeat a key) — documented
   in-file, same spirit as other artifacts' documented simplifications (e.g. json/zip's own
   non-positional name-keyed absorb approximations).

## External churn observed (not touched, not mine)

Mid-session, `cargo test --no-run` transiently failed on `📐️step`/`ap214/✳️cc1/🏗️builder`
(unbalanced braces — genuinely mid-edit by another live session) and separately on
`📐️step`/`🧊️gltf` type errors; both resolved on their own by the time of the final full-crate
run (965/0). `git status` on `✏️s/…/📄️pdf/**` also shows unrelated `M`/`D` entries (composer
files, a `✳️a-2b` subset directory deletion, several untracked `✳️a/✳️e/✳️h/✳️ua/✳️vt/✳️x` subset
dirs) — these are the sibling ticket
`26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES`'s accumulated uncommitted work
(consistent with the briefing's own note that this sibling ticket added the `✳️`-prefixed
subsets), not anything I created, modified, or deleted.

## Files touched (mine only)

**1.7**: `⚙️engine/🦀️component.rs`; `🪆️subsets/✳️any/🧬️schema/{🦀️component.rs,
🟦️component.ts, 🔗️component.graphql, 🔣️component.json, 🛰️component.proto}`;
`🪆️subsets/✳️any/🧬️schema/📸️snapshot/{🦀️component.rs, 🟦️component.ts, 🔗️component.graphql,
🔣️component.json, 🛰️component.proto, 📝️text/{🅰️component.g4, 🔤️component.ebnf,
📖️component.grammar.semio}, 💾️binary/{🥋️component.ksy, 🔠️component.abnf,
📡️component.protocol.semio, 🌶️component.spicy}}`; `🪆️subsets/✳️any/🧬️schema/🔺️diff/{🦀️component.rs
(full rewrite), 🟦️component.ts, 🔗️component.graphql, 🔣️component.json, 🛰️component.proto,
📝️text/*, 💾️binary/*}`; `🪆️subsets/✳️any/🧬️schema/🧬️mutations/{🦀️component.rs (full rewrite),
🟦️component.ts, 🔗️component.graphql, 🔣️component.json, 🛰️component.proto, 📝️text/*,
💾️binary/*}`; `🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs`.

**1.4**: `⚙️engine/🦀️component.rs` (bugfix + test); `🪆️subsets/✳️any/🧬️schema/{🦀️component.rs,
🟦️component.ts, 🔗️component.graphql, 🔣️component.json, 🛰️component.proto}`;
`🪆️subsets/✳️any/🧬️schema/📸️snapshot/{🟦️component.ts, 🔗️component.graphql,
🔣️component.json, 🛰️component.proto}`; `🪆️subsets/✳️any/🧬️schema/🔺️diff/{🦀️component.rs (full
rewrite), 🟦️component.ts, 🔗️component.graphql, 🔣️component.json, 🛰️component.proto}`;
`🪆️subsets/✳️any/🧬️schema/🧬️mutations/{🦀️component.rs (2-arg fix + tests), 🔗️component.graphql,
🔣️component.json, 🛰️component.proto}`; `🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs`.

**Examples**: `📚️examples/🎓️bachelor-thesis/🧪️tests/🦀️test.rs` (added `codec_retention_law`
test, extended the existing file per CLAUDE.md).

## glue_followup

None — all real work landed inside already-mounted files (per S2's load-bearing resolution: no
new directories, no glue.rs edits needed). No new top-level directory was requested.
