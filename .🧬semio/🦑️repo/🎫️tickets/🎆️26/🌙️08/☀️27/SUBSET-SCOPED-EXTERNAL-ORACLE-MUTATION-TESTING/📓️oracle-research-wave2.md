# Oracle Research — Wave 2 (11 owners)

Method: for each owner, read `🧪️oracle/🔣️.json` (registered oracles + `rationale`/`noOracleDecisions`), the `🧬️schema/🧬️mutations/*` leaves (kinds + payload schemas), `🧬️schema/📸️snapshot/` (what the document IS), and `🚪️io/` (import/export carrier formats — the names of the `📥️import/🧩️deserializers/🗿️artifacts/…` and `📤️export/🧵️serializers/🗿️artifacts/…` subtrees). The worked example, `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🧪️oracle/🔣️.json`, registers `ruststep` (Part-21 structure) and `brepjs`/OpenCASCADE (exact geometry) against a genuine standard carrier (STEP AP214). That is the bar every verdict below is measured against.

Two owners already carry extremely well-argued `noOracleDecisions` written by a prior pass in this repo (`vcs-1-mutate`, `txt-utf-8-mutate`); I read their existing `rationale` in full, verified the reasoning against the schemas independently, and largely confirm it rather than re-litigate it from scratch — the honest answer converges on the same place a careful prior author already reached.

---

## rewrite-1-mutate (7 leaves)
`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any`

**What the artifact is.** `RewriteSnapshot` (`🧬️schema/📸️snapshot/🔣️component.json`): a graph-rewrite RULE document with five members —
- `beforeFixtureJson` / `lhsJson` / `rhsJson` : `type: string, contentMediaType: application/json` (three separate JSON documents carried as opaque strings)
- `parameterBindings` : `object` keyed by string, values arbitrary JSON (`PropertyValue`)
- `ruleLayout` : `object` keyed by string, values `{x: double, y: double}` (editor layout points)

The 7 mutation kinds are `edit-before-fixture`, `edit-lhs`, `edit-rhs`, `change-parameter-binding`, `remove-parameter-binding`, `change-rule-layout-point`, `remove-rule-layout-point`. Payload for `change-parameter-binding` is literally `{key: string, newValue: any}` — a blind map-set. The three `edit-*` mutations are blind whole-string replacements of one of the three JSON blobs.

**What it imports/exports.** The `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/…` and `📤️export/🧵️serializers/🗿️artifacts/…` trees carry generic document-conversion adapters (`🔣️json/🔖️rfc8259`, `📄txt/🔖️utf-8`, `📝️md/🔖️commonmark`, `📄️pdf/🔖️1.4`, `📜️docx/🔖️ecma-376`) — these are the *host artifact's* generic import/export bridges (the rewrite-rule editor can be opened/saved via those carriers as a document), not a rewrite-semantics carrier. The rule document's own wire form is `.rewrite.dsl.semio`, semio-native, described nowhere else.

**Verdict: NO QUALIFYING ORACLE POSSIBLE.**
The registered entry today (`rewrite-python-independent`) is explicitly a second implementation inside this repo's own Python, correctly self-classified `cross-semio-implementation` / non-discharging. I confirm its own survey: `GrGen`, `AGG`, and `networkx`'s VF2 isomorphism module all implement graph **rewriting execution** — applying a rule to a host graph — but this artifact is the rule **document**: a before-fixture, a pattern, a right-hand side, parameter bindings, and editor layout coordinates. None of those libraries read `.rewrite.dsl.semio`, none have any opinion on whether `change-parameter-binding` on an absent key inserts a new binding or refuses, and none model "layout point of a rule in an editor" at all — that's not a graph-rewriting concept, it's a Semio-native UI concern glued onto the rule. The three JSON-blob edits (`edit-lhs` etc.) are literally "set string field N to the given string" — a generic JSON parser (`serde_json`, already used in-repo, or `ajv` for schema-shape) can confirm the result is well-formed JSON matching the given bytes, but that is exactly the payload-schema conformance check already required by "fully described," not a new fact about rewrite semantics; it cannot tell you whether the *semantics* the LHS/RHS strings encode are sound, because that vocabulary belongs to Semio alone.
**Best supplement:** the 7 committed `(before, mutation, after, outcome)` specification vectors, replayed byte-for-byte, plus a metamorphic/inverse law (e.g. `remove-parameter-binding` after `change-parameter-binding(k,v)` returns to the pre-change map when the key was absent before). This establishes internal consistency and catches transcription regressions; it cannot catch a misread specification, because there is no external specification to misread against — the document's own committed schema *is* the specification.

**Registration sketch (no oracle to add — record the decision instead):**
```json
{
  "id": "rewrite-1-rule-document-semantics",
  "capabilities": ["rewrite-1-mutate"],
  "rationale": "GrGen/AGG/networkx implement rewrite EXECUTION over a host graph; this artifact is the rule DOCUMENT (pattern + RHS + bindings + editor layout) that none of them read, parse, or have an opinion on. A generic JSON parser can confirm the three string-valued blobs are well-formed JSON post-mutation, which only restates payload-schema conformance and asserts nothing about rewrite semantics.",
  "substitutes": ["specification-vectors", "metamorphic-laws"]
}
```

---

## vcs-1-mutate (6 leaves)
`✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any`

**What the artifact actually is — read the schema, not the name.** `VcsSnapshot` (`🧬️schema/📸️snapshot/🔣️component.json`):
```
{ schema: string, title: string, counter: int64, notes: string, status: string, tags: string[] }
```
The 6 mutations are `rename-vcs` (sets `title`), `change-counter` (sets `counter`), `change-notes` (sets `notes`), `change-status` (sets `status`, a free string, not a validated state machine in the schema shown), `add-tag`/`remove-tag` (append/detach one entry in `tags`). This is a **generic tagged status/checkpoint record** — a title, a counter, freeform notes, a status label, and a tag list. It is not a commit graph, has no notion of parent commits, trees, blobs, branches, merges, or content-addressed objects.

**The question the brief specifically asked me to think hard about: does `git`/`libgit2`/`gitoxide` qualify?**
No — and the reasoning matters more than the name. `git` (and `libgit2`/`gitoxide`, which reimplement the same object model) are authoritative over a **specific data model**: content-addressed blobs and trees, commit objects with parent pointers, refs, and a plumbing/porcelain command surface for diffing and merging that model. None of that model exists in `VcsSnapshot`. There is no commit history here to walk, no tree to hash, no ref to move, no merge to perform. `title`/`counter`/`notes`/`status`/`tags` is the same shape as a lightweight CMS "record" or issue-tracker ticket — the field named `s.vcs.vcs` is a false friend: it names the plugin family (this repo's own checkpoint/versioning mechanism for artifacts), not the Git data model. Feeding this document's mutations to `libgit2` would require inventing a mapping from "set a free-text status string" to "some git operation" that does not exist in git's vocabulary; there is nothing for git to predict. This is the same class of mismatch as feeding STEP AP214 mutations to a PNG library — matching domain *word*, wrong domain *model*.

**What it imports/exports.** `📥️import`/`📤️export` trees: `🔣️json/🔖️rfc8259`, `🎒️zip/🔖️2.0`, `📄txt/🔖️utf-8`, `📕️xlsx/🔖️ecma-376`, `📊️csv/🔖️rfc4180` — generic document-container formats, none of which encode version-control semantics (no git-pack, no git-bundle, no `.git` object format anywhere in this list). The wire forms `.vcs.dsl.semio` / `.vcs.pack.semio` are semio-native and, per the existing `rationale`, the pack twin's own committed grammar is still the repo-wide placeholder (`payload = OCTET+`) that contradicts the committed artifact's first line — so even a hypothetical byte-level parser has nothing coherent to parse yet.

**Verdict: NO QUALIFYING ORACLE POSSIBLE** (confirms and extends the existing `noOracleDecisions[0]` in this owner's own `🔣️.json`, which already declines a third-party library on identical grounds — "no third party reads `.vcs.dsl.semio` or its pack twin"). I add the git-specific negative result explicitly since the brief asked for it: git-family tools are not merely absent, they are the wrong domain model, because this artifact carries no commit graph for them to operate on.
A generic `📊️csv`/`🔣️json` reader can confirm exported bytes parse as valid CSV/JSON, but that only checks container well-formedness (already covered by payload schema conformance), not that e.g. `add-tag` actually appended rather than replaced, or that `change-status` accepted an out-of-vocabulary value it should have refused.
**Best supplement (already what the repo records, and I agree it is the right one):** the 6 committed `(before, mutation, after, outcome)` specification vectors replayed end-to-end, plus the inverse/metamorphic laws (`add-tag(x)` then `remove-tag(x)` restores the prior tag set when `x` was absent; `rename-vcs(t)` is idempotent on repeat application). This establishes internal self-consistency of the production code against its own committed fixtures; per the repo's own ceiling statement, it cannot catch a mistake shared between the handcrafted vector and the production code, because nothing outside this repository ever reads either.

**Registration sketch (record/keep as no-oracle, refine the existing entry):**
```json
{
  "id": "vcs-1-checkpoint-record-not-git",
  "capabilities": ["vcs-1-mutate"],
  "rationale": "VcsSnapshot = {title, counter, notes, status, tags[]} is a generic tagged status record, not a commit graph. git/libgit2/gitoxide are authoritative over content-addressed blobs/trees/commits/refs, none of which this document has; there is nothing in git's vocabulary for 'set a free-text status string' to map onto. Exported containers (json/zip/txt/xlsx/csv) carry no version-control semantics either.",
  "substitutes": ["specification-vectors", "metamorphic-laws"]
}
```

---

## txt-utf-8-mutate (5 leaves)
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any`

**What the artifact is.** `TxtSnapshot`: `{schema, lines[], trailingNewline, lineEnding ∈ {lf, crLf}}`. The 5 mutations: `set-trailing-newline`, `set-line-ending`, `insert-line`, `remove-line`, `set-line`. This is entirely about **line structure**: how the byte stream is split into `lines[]` on LF vs CRLF, whether a trailing terminator is present, and single-line edits.

**The question the brief specifically asked me to think hard about: do ICU / `unicode-segmentation` / `icu_normalizer` and Unicode's own conformance test data qualify?**
No, for a reason sharper than "insufficient" — Unicode's own specification actively **disagrees** with what this artifact defines as correct, so a Unicode-conformant library would be a wrong oracle, not just an incomplete one. Unicode's Newline Guidelines (and UAX #14, Line Breaking) treat CR, LF, CRLF, **NEL (U+0085), VT, FF, LS (U+2028), and PS (U+2029)** all as line-terminating characters for portable text processing. This subset's own committed specification vectors explicitly assert the opposite: NEL/LS/PS are **not** treated as separators here — only LF and CRLF are. So an ICU-based segmenter driven at its line-break boundary API would split lines in places this artifact's `lines[]` must not split, and agree only on the subset of inputs containing solely LF/CRLF. It would not verify the artifact's mutations; on any fixture containing NEL/LS/PS it would actively contradict them. ICU's grapheme-cluster segmentation (UAX #29) and `icu_normalizer`'s NFC/NFD (UAX #15) are real, rigorously-conformant implementations — but they answer "where are grapheme boundaries" and "what is the canonical form," neither of which any of the 5 mutation kinds here ask. Astral emoji, variation selectors and combining marks appear in this subset's fixtures only to prove line-splitting doesn't corrupt a multi-byte UTF-8 sequence mid-codepoint — a much narrower claim than anything Unicode segmentation certifies.

The repo's own `noOracleDecisions` entry independently surveyed the next-closest candidate — the `csv` crate (Rust, `csv` on crates.io, dual **MIT OR Unlicense**, maintained by BurntSushi), already used elsewhere in this plugin family for tabular subsets — and found it via a standalone probe to be disqualifying on its own terms: with quoting disabled and a delimiter that can't appear in real text, its record reader can confirm single-style non-blank line splitting, but `csv-core`'s NFA silently drops zero-byte records (never emits a record for a blank line), so it cannot report the true line count on any real prose with blank lines — and the fixture in question has 80 of them, measured. It also collapses LF/CR/CRLF into one undifferentiated boundary and never reports which terminator it saw, so it cannot confirm or refute the `lineEnding` field the mutations operate on at all.

**Verdict: NO QUALIFYING ORACLE POSSIBLE.**
No credible third-party library is authoritative over "is this a text file's line structure" the way a PDF or PNG library is authoritative over its format, because plain-text line-splitting policy (LF/CRLF-only, trailing-newline presence, exclusion of NEL/LS/PS) is exactly what this Semio subset defines rather than inherits — and where a nearby standard (Unicode's own newline guidance) does speak to the question, it disagrees with the subset's chosen policy, which disqualifies it rather than merely leaving it silent.
**Best supplement:** the 9 committed byte-exact specification vectors (pure LF, pure CRLF, missing trailing terminator, mixed CRLF+bare LF, BOM-as-content, astral emoji + variation selectors, combining marks, explicit NEL/LS/PS-not-a-separator cases) plus the carrier round-trip law (re-encoded bytes bit-identical to input) and the inverse law over the real 27,471-byte / 170-line / 80-blank-line German transcript fixture. This is a strong internal-consistency net — byte-exact, real-world-sized, adversarial on the exact edge cases (BOM, astral, combining marks) most implementations get wrong — but per the repo's own honest ceiling, both sides of every comparison remain Rust written in this repository, so a mistake shared by the handcrafted vector and the production code passes unseen. Today the "reference" implementation living in `semio_s_plugin_stdio_test_oracle` (`oracle_apply_mutation`, `independent_split`, `independent_render`, `project_txt`) is also disqualified for the same reason as the 57 repo-wide: same language, same crate, same author, same review culture as production.

**Registration sketch (record/keep as no-oracle):**
```json
{
  "id": "txt-utf-8-line-structure-diverges-from-unicode",
  "capabilities": ["txt-utf-8-mutate"],
  "rationale": "Unicode's own Newline Guidelines / UAX #14 treat NEL, VT, FF, LS, PS as line terminators; this subset's committed vectors explicitly require NEL/LS/PS to NOT be separators. An ICU/unicode-segmentation-conformant oracle would therefore actively contradict this subset on any fixture containing those characters, not merely fail to cover it. The `csv` crate (MIT OR Unlicense) was independently probed and found to silently drop zero-byte (blank-line) records and to collapse LF/CR/CRLF into one undifferentiated boundary, so it cannot report true line count or the lineEnding value these mutations act on.",
  "substitutes": ["specification-vectors", "metamorphic-laws"]
}
```

---

## s-space-1-mutate (4 leaves)
`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any`

**What it is.** `SSpaceSnapshot { schema, spaceId, artifacts: SpaceArtifactRow[] }`, each row `{ id, name, kindId, schema, dialect{artifactKind,standard,subset}, createdAtMs, createdBy, updatedAtMs, updatedBy }`. Despite the planetary/spatial-sounding plugin name, this is a **collaborative workspace's artifact INDEX** — an id-keyed metadata table listing which documents live in the space, by whom and when — never the indexed documents' own content or any geometry. Mutations: `create-artifact` (payload: a full row), `delete-artifact`, `rename-artifact` (`{id, newName}`), `touch-artifact` (bumps `updatedAtMs`/`updatedBy`). All are metadata-table edits.

**What it exports.** `🚪️io/` contains only `🦀️component.rs` — **no `📥️import`/`📤️export` subdirectories exist at all.** There is no registered external carrier of any kind; the document persists solely through the proprietary `.sspace` DSL and semio's own binary pack format.

**Verdict: NO QUALIFYING ORACLE POSSIBLE.** There isn't merely an absence of a suitable library — there is no standard interchange format in play for a third party to read in the first place. No workspace/registry-index standard is relevant (this isn't LDAP, isn't a package-manifest format, isn't anything with third-party tooling), and even if one existed, the mutated surface (name/timestamp/author bookkeeping on rows) carries no computable semantics a library could verify beyond field equality.
**Best supplement:** specification vectors (committed before/after/outcome fixtures for the 4 kinds) plus metamorphic laws — `rename-artifact(id, n1)` then `rename-artifact(id, n2)` collapses to `rename-artifact(id, n2)`; `create` then `delete` of the same id is identity on the row set; `touch-artifact` changes only the two timestamp/author fields, holding all else fixed. This establishes internal consistency, not ground truth against any external authority, since none exists.

---

## s-home-1-mutate (1 leaf)
`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any`

**What it is.** `SHomeSnapshot { schema: string, catalogGeneration: int64 }` — a two-field document backing an app-launcher home screen; "home" here is a launcher surface, not a building or room, and there is no spatial content. The one mutation, `change-catalog-generation`, payload `{newCatalogGeneration: integer ≥ 0}`, is a pure scalar setter.

**What it exports.** Full generic stdio fan-out: `📥️import`/`📤️export` under `🗿️artifacts/{🔣️json/🔖️rfc8259, 🎒️zip/🔖️2.0, 📄txt/🔖️utf-8, 📕️xlsx/🔖️ecma-376, 📊️csv/🔖️rfc4180}`. The production JSON exporter (`🚪️io/📤️export/🧵️serializers/…/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`) calls `serde_json::to_value(snapshot)` directly.

**Verdict: CARRIER ORACLE, but weak/marginal — flagged honestly rather than oversold.** A different-ecosystem JSON reader, e.g. **`ajv` (npm, v8.20.0, MIT license — confirmed via `npm view ajv version`/`license`)**, paired with `JSON.parse`, can read the exported RFC 8259 bytes and confirm (a) well-formedness, (b) conformance to the committed snapshot schema shape, and (c) that `catalogGeneration` in the output equals `newCatalogGeneration` from the payload with no int64-as-JSON-number precision loss — a real cross-language risk this genuinely tests. But because the document is two scalar fields and the mutation is a 1:1 field set, the "prediction" ajv would be checking against is just the payload's own literal value — there is no independent computation for the third party to have derived. This is closer to a syntax/precision check than a semantic oracle in the AP214/cc6 sense (where brepjs independently recomputes geometry). I record it as a real but thin discharge: it covers `change-catalog-generation` for well-formedness and numeric-precision faithfulness, and nothing else, because nothing else exists in this document.

**Registration sketch:**
```json
{
  "id": "ajv-s-home-1-json-carrier",
  "ecosystem": "javascript",
  "package": "ajv",
  "version": "8.20.0",
  "license": "MIT",
  "capabilities": ["s-home-1-mutate"],
  "testOnly": true,
  "rationale": "Independent-ecosystem RFC 8259 well-formedness and int64-precision check on the exported carrier for change-catalog-generation. Does not test any semantics beyond scalar identity, because the document has none."
}
```

---

## playground-1-mutate (1 leaf)
`✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any`

**What it is.** `PlaygroundSnapshot { schema: string }` — one field. A demo/scratch artifact tag holder with no domain content, confirmed by the owner's own oracle-file comment. The one mutation, `change-schema` (`{new_schema: string}`), sets the single field verbatim.

**What it exports.** The same generic stdio fan-out as `s-home` (json/rfc8259, zip/2.0, txt/utf-8, xlsx/ecma-376, csv/rfc4180) — shared boilerplate plumbing across `s.*` artifacts, not playground-specific.

**Verdict: NO QUALIFYING ORACLE POSSIBLE**, and honestly below the bar even as a weak carrier check. A third-party JSON reader confirming `JSON.parse(bytes).schema === new_schema` cannot fail for any bug a competent implementation would make — there is no derived value, no numeric precision risk, no nested structure, nothing beyond an echo. Unlike `s-home-1`'s int64 precision angle, there is no cross-language risk here at all (a string round-trips byte-for-byte through JSON with no ambiguity). Calling this an "oracle" would be decorative.
**Best supplement:** specification vector + identity round-trip (`change-schema(x)` twice is idempotent; `change-schema` is the only mutation, so full coverage is one fixture).

---

## curate-1-mutate (3 leaves)
`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any`

**What it is.** `CurateSnapshot` = a composed catalog child + `stockExtra: ObjectKindExtra[]` (id/name/moduleId/typologyPath/availability plus a closed 4-variant `GeometryRecipe` union: box/frame/slab/mesh) + `curated: CuratedItem[]` where `CuratedItem = {objectId: string, count: uint32}`. It's a bill-of-materials-like curation list layered over a catalog. The 3 mutations — `create-curated-item` (`{item:{objectId,count}}`), `delete-curated-item`, `change-curated-item-count` (`{objectId, newCount}`) — touch only the `curated` list.

**What it exports.** `zip/2.0`, `txt/utf-8`, `png/1.2`, `json/rfc8259`, `stl/ascii`, `obj/3.0` — generic 3D-geometry and document carriers for the composed artifact as a whole. None of them encode a bill-of-materials/count-list structure; STL/OBJ carry mesh geometry, not `{objectId, count}` rows.

**Verdict: NO QUALIFYING ORACLE POSSIBLE.** The mutated surface is pure membership/position/count algebra over one ordered list of `{opaque string id, uint32 count}` — no BOM/inventory standard, and no spreadsheet/CSV library, has domain authority over an arbitrary opaque-id count list with no external identity system behind the ids. The repo's own registration already surveys and reaches this conclusion (`stock` is out of this subset's vocabulary; a `CuratedItem` is "two scalars, no rename, no nested collection" — nothing for a domain library to adjudicate). The registered `curate-python-independent` (AGPL-3.0-only) is a second implementation, correctly self-classified as supplemental only.
**Best supplement:** metamorphic/round-trip laws — `create(x); delete(x)` is identity; `change-curated-item-count(x, n)` applied twice collapses to the second call (idempotent last-write-wins); insertion order of untouched entries is preserved.

---

## gisterrain-1-mutate (2 leaves)
`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any`

**What it is.** `GisTerrainSnapshot { exaggeration: f64, importedFeaturesJson: string }` — exactly two fields. Per the owner's own oracle rationale, `importedFeaturesJson` is opaque here: this subset never parses it (a derived mesh, computed from both fields, lives as a separate content-addressed child handle outside this subset's scope). Mutations: `change-exaggeration` (`{newExaggeration: number}`), `change-imported-features` (`{newImportedFeaturesJson: string}`) — both plain field setters.

**What it exports — checked carefully, since GeoTIFF was flagged as the expected answer.** The full `🚪️io` subtree was searched for any raster/elevation-grid carrier (`tif`, `geotiff`, `hgt`, `dem`) — **none exist**. What's actually there is `☁️las/1.0`, `☁️ply/1.0`, `🖊️dwg/ac1018`, `🧊️gltf/2.0`, `🧊️obj/3.0`, `🟪️stl/ascii`, `png/1.2`, `json/rfc8259`, `txt/utf-8` — a point-cloud/mesh export set (LAS and PLY are genuine geospatial point-cloud standards; glTF/OBJ/STL are 3D-mesh standards; DWG is CAD), not a GeoTIFF elevation grid. The premise "this exports GeoTIFF" does not hold for this owner in this codebase — worth stating plainly since assuming it would have produced a wrong verdict.

**Verdict: NO QUALIFYING ORACLE POSSIBLE for these two leaves** — and this holds even granting that LAS/PLY/glTF are real, well-supported third-party carriers, because neither mutation touches geometry directly. Both leaves set a scalar multiplier or swap an opaque string; the geometry those carriers encode is a *derived* child artifact one hop away, out of this subset's scope. `laspy` (PyPI, v2.7.0, BSD license) or `pygltflib` (PyPI, v1.16.5, MIT license) have no authority over "was this f64 set to 2.5" — that's value equality, not a geodata computation for a library to verify. `gdal` (MIT-licensed core, but a C-dependency system library, not a pure third-party package) and `geojson`/`geo` (PyPI `geojson` v3.3.0, BSD) were also considered and correctly declined by the repo's own survey for the same reason: none has an opinion on a vertical-exaggeration scalar or an unparsed JSON string.
**Worth flagging for future work:** if a future subset directly mutates the *derived mesh's* geometry (not these exaggeration/imported-features scalars), LAS/glTF readers become live carrier-oracle candidates there — just not for `gisterrain-1-mutate`'s two current leaves.
**Best supplement:** specification vectors + metamorphic laws (the two mutations are independent — changing one leaves the other untouched — and repeated application is idempotent/last-write-wins).

---

## energy-model-1-mutate (1 leaf)
`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any`

**What it is.** `EnergyModelSnapshot { schema, model: object (untyped JSON blob), structure (composed s.stdio.semio.value child), zones (composed s.stdio.semio.table child), referencedModel? }`. Per the owner's own `noOracleDecisions` rationale, the document "persists no content model of its own" — `model` is an opaque generic-JSON blob, not a parsed building-energy description. The one mutation, `replace-model` (`{newModelJson: string}`), is a wholesale overwrite; the single committed test degrades an empty payload to a no-op.

**What it exports.** `zip/2.0`, `txt/utf-8`, `csv/rfc4180`, `xlsx/ecma-376`, `json/rfc8259` — generic tabular/document carriers, not an EnergyPlus IDF/epJSON building-simulation carrier.

**Verdict: NO QUALIFYING ORACLE POSSIBLE.** `eppy` (PyPI, v0.6.7, MIT license) driving EnergyPlus/OpenStudio is the obvious domain candidate by name — and it is exactly what the repo's own `noOracleDecisions` entry names and declines, correctly: `model` is opaque generic JSON here, not a parsed/simulatable energy model, and `replace-model` is bulk field replacement with no computed or simulated result for any engine to check. This is honestly recorded in-repo as an open **debt** rather than a settled judgment: it is blocked on the committed specification vectors not being declared as pinned `asset://` fixtures (so even a non-qualifying Python second implementation can't read them yet) and on `identity-round-trip` being refused because this subset's snapshot grammar is still the repository-wide placeholder.
**Best supplement (today):** specification-vector replay only — there is not yet even a second-implementation supplement in place, unlike the other owners above.

---

## writer-1-mutate (4 leaves)
`✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any`

**What it is — verified against the real Rust struct, not just the JSON Schema file.** The committed `🧬️schema/📸️snapshot/🔣️component.json` still describes a flat 5-string-field record (`schema, id, languageId, uri, text`), but that file is **stale**: `🧬️schema/📸️snapshot/🦀️component.rs` (the ground truth) shows `WriterSnapshot` was migrated ("wave 3, writer→C:document") to `{schema: String, id: String, language_id: String, uri: String, document: WriterDocumentChild}`, where `document` is a `#[child(kind = "s.stdio.semio.document")]` — a **composed, content-addressed child handle**, not inline text. The doc comment says this explicitly: "the inline `text: String` content field is replaced by a fixed composed `s.stdio.semio.document` CHILD slot." I checked the JSON exporter (`🚪️io/📤️export/🧵️serializers/…/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`): it is `IoFidelity::Exact` via `serde_json::to_value(from)`, so it will faithfully serialize whatever the `document` field's own `Serialize` impl emits — a handle, not the child's text content.

Mutations: `rename-writer{newId}`, `change-uri{newUri}`, `change-language{newLanguageId}` set `id`/`uri`/`language_id` directly — genuine flat scalar setters, unaffected by the child-composition change. `edit-text{text}` writes through to the composed `document` child (with a documented no-op when the new text equals the child's current content).

**Verdict: SPLIT — CARRIER ORACLE for 3 of 4 kinds, NO QUALIFYING ORACLE for the 4th.**
For `rename-writer`, `change-uri`, `change-language`: because the JSON export is Exact-fidelity and these three fields are plain strings straight through to the wire, a genuine third-party JSON toolchain fully verifies the result — **`jsonschema`** (PyPI, Python, MIT license) to check exported shape, plus **`deepdiff`** (PyPI, Python, MIT license) or the JS equivalent **`jsondiffpatch`** (npm, MIT license) to diff before/after export and assert that *exactly* the targeted field changed to the payload's value and nothing else did. This is a real, non-trivial check: it independently confirms field isolation (that `rename-writer` doesn't accidentally touch `uri`).
For `edit-text`: **not verifiable through this carrier.** The exported `document` value is an opaque content-addressed handle into a separate `s.stdio.semio.document` artifact; a third-party JSON reader sees the handle change (or not, matching the no-op vector) but has no way to dereference it into the actual new text within this same export, so it cannot confirm the new text is what the payload asked for — only that *something* about the child reference differs. This precisely matches, and independently confirms as still accurate to production code, the incumbent oracle's own rationale ("edit-text... writes the document's BODY, which this snapshot does not carry"); the committed JSON Schema *descriptor* file for the snapshot is what's actually stale here, not the rationale.

**Registration sketch (3 of 4 kinds only):**
```json
{
  "id": "writer-json-rfc8259-scalar-diff",
  "kind": "third-party-library",
  "ecosystem": "python",
  "package": "jsonschema",
  "version": "4.x",
  "license": "MIT",
  "capabilities": ["writer-1-mutate"],
  "comparisonProfiles": ["ordered-json-v1"],
  "rationale": "Exact-fidelity RFC 8259 export lets a third-party JSON Schema validator + structural differ (deepdiff, MIT) confirm rename-writer/change-uri/change-language each touch exactly the one targeted flat scalar field. edit-text is explicitly OUT OF SCOPE for this oracle: it writes a composed content-addressed child handle this export does not dereference.",
  "engine": {"family": "json-schema+diff", "implementation": "jsonschema + deepdiff over rfc8259 export"},
  "productionReachable": true,
  "networkDuringExecution": false
}
```
**Non-coverage:** the native carrier (`.dsl.semio`/`.pack.semio`) is untouched by this — its committed grammar is still the repo-wide placeholder contradicted by the real artifact's first line, so `identity-round-trip` stays unaddressed regardless. `edit-text` needs either the child document artifact (`s.stdio.semio.document`) to carry its own qualifying oracle one level down, or a fixture that pins the child's literal content so a reference can compare it directly — neither exists today.

---

## imperative-1-mutate (4 leaves)
`✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any`

**What it is.** `ImperativeSnapshot = {schema, path: {steps: [Step{id, kind, params?, bodies?}]}, seed}` — a recursive step-tree/program document composed over an `s.stdio.semio.flow` child and an `s.stdio.semio.text` child (confirmed in the owner's own `🧪️oracle/🔣️.json` `_comment`). `Step.bodies` maps slot names to nested `Path`s, so this is a small imperative-program AST, not a flat record. Mutations: `create-step{pathRef, step}`, `delete-step{pathRef, id}`, `edit-step-params{pathRef, id, newParams}`, `reorder-steps{pathRef, id, toIndex}` — structural tree edits with bespoke rules (duplicate-id rejection, root/branch path-reference validation, index-clamping on out-of-range `toIndex`).

**What it exports.** Only `json/rfc8259` is full-fidelity (`serde_json::to_value` of the whole struct). `md/commonmark` just wraps the printed internal DSL text as one opaque block — not real structured Markdown. `csv/rfc4180` is degenerate: one header cell `"payload"` plus one data cell holding the entire printed DSL string. `txt/utf-8` is unimplemented (`serialize` returns an explicit "not yet implemented" error).

**Verdict: NO QUALIFYING ORACLE POSSIBLE**, confirmed against the owner's own `noOracleDecisions` entry ("nothing third-party reads `.imperative.dsl.semio`... THAT SURVEY STANDS"). Even though JSON export is full-fidelity here (unlike md/csv/txt), that does not rescue this owner the way it partially does for `writer-1-mutate`: judging whether `create-step`/`delete-step`/`edit-step-params`/`reorder-steps` produced the *correct* tree requires re-deriving this program's own bespoke addressing, duplicate-id, and index-clamp rules — and doing that in any language is writing a second implementation of Semio-specific semantics, which this task's own framing disqualifies regardless of which third-party JSON/diff library carries the bytes. A generic differ can report *that* the tree changed structurally; it cannot judge *whether* the specific structural change was the one the mutation's own bespoke rules require (e.g., whether `reorder-steps` clamped `toIndex` correctly at the array bound). No off-the-shelf workflow/program/AST-editing library models this bespoke step-tree format, and md/csv/txt give no real structured content to check against in the first place.
**Best supplement:** the 4 committed `(before, mutation, after, outcome)` specification vectors replayed end to end, plus metamorphic/inverse laws asserted in role (e.g. `create-step` then `delete-step` of the same id returns to identity). This is what the repo already records as the substitute; a `cross-semio-implementation` (Python) supplement, the same shape as `writer-1-mutate`'s, is the next concrete step but is currently blocked on the same undeclared `asset://` fixture problem noted for `energy-model-1-mutate` and `vcs-1-mutate`.

---

## Summary table

| Owner | Leaves | Document is | Real export carriers | Verdict |
|---|---|---|---|---|
| `rewrite-1-mutate` | 7 | Graph-rewrite RULE (3 JSON-string blobs + bindings map + layout map) | generic json/txt/md/pdf/docx bridges (host document, not rule semantics) | **NO QUALIFYING ORACLE** — GrGen/AGG/networkx implement rewrite *execution*, not this rule *document* |
| `vcs-1-mutate` | 6 | Tagged checkpoint record `{title,counter,notes,status,tags[]}` | json/zip/txt/xlsx/csv | **NO QUALIFYING ORACLE** — `git`/`libgit2`/`gitoxide` are the wrong domain model (no commit graph exists here); false friend by name only |
| `txt-utf-8-mutate` | 5 | Line-structure record `{lines[],trailingNewline,lineEnding}` | native only | **NO QUALIFYING ORACLE** — Unicode/ICU newline guidance actively *disagrees* with this subset's NEL/LS/PS policy; `csv` crate (MIT/Unlicense) independently probed and found to drop blank-line records |
| `writer-1-mutate` | 4 | `{schema,id,languageId,uri}` + composed content-addressed `document` child | json (Exact fidelity) + lossy txt/md/pdf/docx | **SPLIT**: `jsonschema`+`deepdiff` (PyPI, MIT) verify 3/4 kinds (flat scalars); `edit-text` NO QUALIFYING (opaque child handle) |
| `imperative-1-mutate` | 4 | Step-tree program AST composed over flow+text children | json (full fidelity) + degenerate md/csv, unimplemented txt | **NO QUALIFYING ORACLE** — bespoke addressing/clamp/uniqueness rules require a second implementation regardless of carrier |
| `s-space-1-mutate` | 4 | Workspace artifact-index table (9-field rows) | none registered at all | **NO QUALIFYING ORACLE** — no external format exists to hand to a third party |
| `curate-1-mutate` | 3 | Curation count-list `{objectId,count}[]` over a catalog | zip/txt/png/json/stl/obj (geometry carriers, not BOM data) | **NO QUALIFYING ORACLE** — no BOM/inventory standard has authority over an opaque-id count list |
| `gisterrain-1-mutate` | 2 | `{exaggeration:f64, importedFeaturesJson:string}` | las/ply/dwg/gltf/obj/stl/png/json/txt (no GeoTIFF) | **NO QUALIFYING ORACLE** — mutations are scalar/string setters, one hop away from the geometry the carriers actually encode |
| `playground-1-mutate` | 1 | `{schema:string}` | generic stdio fan-out | **NO QUALIFYING ORACLE** — single-field echo, below the bar even as a weak carrier check |
| `energy-model-1-mutate` | 1 | `{schema, model:opaque JSON, structure, zones, referencedModel?}` | zip/txt/csv/xlsx/json (no IDF/epJSON) | **NO QUALIFYING ORACLE** — `eppy`/EnergyPlus declined correctly; `model` is unparsed opaque JSON, not a simulatable description |
| `s-home-1-mutate` | 1 | `{schema:string, catalogGeneration:int64}` | json/zip/txt/xlsx/csv | **CARRIER ORACLE (weak)** — `ajv` (npm, v8.20.0, MIT) confirms RFC 8259 well-formedness + int64 precision; no deeper semantics exist to test |

**Net result for this batch of 11:** 1 clean split (writer, 3/4 discharged), 1 thin-but-real carrier oracle (s-home), 9 owners land on NO QUALIFYING ORACLE POSSIBLE — not from lack of searching, but because in every one of those 9 cases either (a) no standard external format is exported at all, or (b) the exported standard format the artifact carries genuinely does not encode what the specific mutation vocabulary changes (metadata/index tables, checkpoint records, opaque blobs, or bespoke program trees one hop away from any geometry/energy/version-control standard that superficially shares the domain's name). The two "think hard" cases confirm the brief's suspicion directly: `git`/`libgit2` fail because `s.vcs.vcs` has no commit graph, and ICU/Unicode conformance data fails not from silence but from an active, documented disagreement with this subset's own line-terminator policy.
