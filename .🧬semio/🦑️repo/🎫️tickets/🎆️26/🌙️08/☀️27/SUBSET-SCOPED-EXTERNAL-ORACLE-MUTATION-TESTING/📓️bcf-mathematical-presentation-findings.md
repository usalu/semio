# 📓️ bcf / mathematical / presentation — carrier verdicts, witnessability, and a redirect mid-flight

Scope: the three subsets assigned in this pass. Ranked entries are `📓️worklist.md` rows 6, 7, 10.

## Status up front

Research (carrier verdicts, exact mutation vocabularies, exact field shapes) is **complete and
verified by reading source** for all three subsets — see below, every claim has a `file:line`.

Implementation was started via the Rust in-process oracle pattern (the shape `bcf`'s own, already-
committed `🧪️oracle/🦀️component.rs` uses: an independent Rust reimplementation of the mutation's
apply/inverse logic, composed with `zip`+`quick-xml`). Mid-flight, the coordinator redirected away
from this pattern:

* The Rust workspace does not compile right now (`semio-framework-plugin`, unrelated in-flight
  refactor) — the shared `semio-s-plugin-stdio-test-oracle` crate itself DOES compile standalone
  (`cargo build --features oracles` — confirmed, exit 0), because it deliberately carries its own
  `[workspace]` (same isolation `➗️mathematical`'s and `🎬️sequence`'s own oracle-adjacent crates use),
  but repo-wide harness commands are unusable while the peer refactor is in flight.
* A newly-added gate, `reimplementation-registered-as-third-party`, blocks registering an oracle that
  computes the expected mutation result in this repository's OWN Rust and uses the third-party
  library only to confirm the file parses (this is what broke `gltf`/`png`/`jpg`/`bmp`/`tiff`,
  156 mutations, coverage 83.2% → 28.7%). The correct shape: **the library reads; the pipeline
  compares** — exactly the finished `mesh`/`brep` pilots' own `🔬️probes/📜️script.ts` shape (an
  external-process probe that marshals to a vendored library and emits a typed `ProbeReport`,
  computing nothing itself), not the in-process Rust shape.

Everything I had built toward the in-process shape (a `mathematical` oracle `🦀️component.rs`
composing `csv`, and two new `lib.rs` mount points for `mathematical`/`presentation`) has been
**reverted** — deleted the new file, restored `📦️lib.rs` to its prior content, re-verified
`cargo build --features oracles` still succeeds standalone. Nothing of that shape is left in the
tree. `bcf`'s own PRE-EXISTING `🦀️component.rs` was not touched or judged here either way — it
predates this ticket and was not one of the five names the coordinator's audit flagged, but it IS the
same reimplement-and-compare shape the new gate targets in general, and it is not this report's call
to make (see "Open question" at the end).

No repo-wide harness command was run after the redirect landed (a peer's `clean taxonomy plan` job is
at ~95% CPU right now, per the coordinator). The one `contract` run earlier in this pass (well before
the redirect, before the peer job started) is quoted below because it independently confirms the
carrier verdicts against the gate's own machinery.

---

## `bcf@2.1/✳️any` (14 mutations)

**Carrier**: `zip` real, `xml` a stub. Confirmed twice — once by reading the body, once by the gate:

* `…💬️bcf/…/🎒️zip/🔖️2.0/✳️any/🦀️component.rs:13` — `crate::artifacts::bcf::io::encode_bcf(from)`, a
  real bcfzip writer.
* `…💬️bcf/…/📰xml/🔖️1.0/✳️any/🦀️component.rs:8` — returns `XmlSnapshot { doc: XmlDocument::default() }`
  for **any** `_from`, the param unused. Confirmed independently by the live `contract` run (this
  session, before the redirect): `…📰xml/🔖️1.0/✳️any/🦀️component.rs  The xml serializer never reads
  its input`.
* Same `contract` run also reports: `…🧪️oracle/🔣️.json  Catalog bcf-2-1-any declares capability
  bcf-2-1-mutate (14 kind(s)) and no mutation manifest owns it` — i.e. the oracle is REGISTERED
  (`zip-quick-xml-bcf-2-1-mutate`, already in `🔒️dependencies.json` as `zip`/`quick-xml`) but has no
  `mutationManifests` yet. This part of my finding stands regardless of the pattern pivot.

**Oracle already chosen**: `zip-quick-xml-bcf-2-1-mutate` (`zip` 6, `quick-xml` 0.42, both in
`🔒️dependencies.json`). `comparisonProfiles` already declares `semantic-bcf-v1` (topic/comment/
viewpoint/part keyed by guid/name, arrays as sets, a viewpoint's PNG snapshot compared as
size+digest).

**Witnessable (all 14, in principle)** — the zip+xml container carries every field the 14-kind
vocabulary touches: `version`, per-topic `title/description/status/priority/labels/creationDate/
creationAuthor`, per-comment `date/author/text/viewpointRef`, per-viewpoint `camera/components/
snapshot`, and unconsumed raw parts. Nothing in the vocabulary addresses a field the bcfzip shape
lacks (unlike `mathematical`/`presentation` below).

**Remaining work under the read-only/pipeline-compares shape**: a `🔬️probes/📜️script.ts` (vendored
`jszip` 3.10.1 MIT + `fast-xml-parser` 5.11.1 MIT, both present in `node_modules`, versions/licenses
confirmed directly — `node -e "console.log(require('jszip/package.json').version...)"` →
`3.10.1 (MIT OR GPL-3.0-or-later)`; `fast-xml-parser`'s `package.json` read directly →
`5.11.1`/`MIT`) that reads a bcfzip and emits a `ProbeReport` with the semantic projection above —
computing nothing, never applying a mutation. A `🏭️generator/📜️script.ts` that constructs BEFORE/AFTER
bcfzip PAIRS directly via the same two libraries (never by "applying" a mutation in code) for a
representative slice of the 14 kinds, `mutationManifests` (14 entries, all pointing at the registered
oracle), and `fixtureManifests` for the generated pairs. None of this was completed before the
redirect; see "What's next" below.

---

## `mathematical@1/✳️any` (15 mutations)

**Carrier**: `csv` real (two-hop: this subset's own `MathematicalIntoCsv` builds a `CsvSnapshot`
struct, `stdio.csv`'s own `encode_csv` at
`…📊️csv/🔖️rfc4180/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:162` emits genuine RFC 4180 text — confirmed
real by reading both hops in full). `md` sibling is
`…➗️mathematical/…/📝️md/🔖️commonmark/✳️any/🦀️component.rs` = `print_dsl`, confirmed stub by the same
live `contract` run: `…📝️md/…  The md serializer emits the artifact's internal DSL text, not md`.

Exact row shape, read from the serializer body (`…➗️mathematical/…/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs:22-33`):
one row per graph node, **`id, label(quoted), x, y`**, no header. `edges`, `directed`, `algorithm`,
point-cloud geometry and the `equation` AST are never written — documented `IoFidelity::Lossy` in the
serializer's own doc comment.

**Exact mutation field shapes**, read from each kind's own `🔣️payload.schema.json` and its
`🔺️diff/🦀️component.rs`:

| kind | payload fields | outcome classes (from `🔺️diff` body) |
| --- | --- | --- |
| `create-node` | `id,label,x,y` | `fatal(duplicate-id)`→rejected, else applied |
| `delete-node` | `id` | `error(target-missing)`→rejected, else applied (cascades incident edges — invisible to csv) |
| `delete-nodes` | `ids[]` | `error` if ALL missing→rejected, else applied (partial-missing is a warn, still applied) |
| `change-node-label` | `id,newLabel` | `error(target-missing)`→rejected; `empty()` same label→no-op; else applied |
| `move-node` | `id,x,y` | `error(target-missing)`→rejected; `fatal` non-finite→rejected; `empty()` same position→no-op; else applied |

**Witnessable, 5 of 15**: exactly the five above — every field they touch (`id`/`label`/`x`/`y`) is a
csv column. **Not witnessable, 10 of 15**: `change-graph-directed`, `update-graph-algorithm`,
`replace-graph`, `connect-nodes`, `disconnect-nodes`, `replace-points`, `insert-point`, `remove-point`,
`move-point`, `change-coefficient` — every one touches `edges`/`directed`/`algorithm`/point-cloud
geometry/the equation AST, none of which the row shape carries at all. This is the exact shape of the
already-approved `sequence@1/any` precedent the coordinator named: 4-of-8 carried there, 5-of-15 here,
same reason (a flat grid has no edge/graph-metadata concept), same mechanism (the other kinds keep
`mathematical-1-mutate-uncarried` — this subset ALREADY has a `noOracleDecisions` entry
(`mathematical-mutation-semantics`) at `➗️mathematical/…/✳️any/🧪️oracle/🔣️.json:6-17` that would need
its `capabilities` extended with that uncarried tag, mirroring `sequence`'s own entry verbatim
(`✏️s/🔌️plugins/🎬️sequence/…/✳️any/🧪️oracle/🔣️.json`, `noOracleDecisions[0].capabilities: ["sequence-1-mutate-uncarried"]`).

**No vendored JS CSV library exists in this repo.** Checked `node_modules` directly (`ls node_modules
| grep -i csv` → only `csv` as a Rust crate elsewhere) and the root `package.json` — no
`papaparse`/`csv-parse`/`d3-dsv` or similar. The coordinator's redirect assumed "straightforward
vendored JS readers" for all three subsets; that holds for `bcf` and `presentation` (both zip+XML,
`jszip`+`fast-xml-parser` already vendored) but **not** for `mathematical` — a TS probe for this
subset would need a NEW JS-ecosystem dependency added and registered (e.g. `papaparse`, a real,
well-known MIT-licensed package, but not yet vendored here, not yet an approved `test-oracle` entry).
I did not add one; flagging it rather than proposing unilaterally, per "you MUST NOT propose an
unverified library" — `papaparse` needs a version/license pin and an explicit approval, same bar the
already-approved roster went through.

---

## `semio@v1/✳️presentation` (15 mutations)

**Carrier**: `pptx`, real but through TWO hops, and **narrower than the worklist's own one-line
summary suggested** — I re-read the full serializer body this pass
(`…🧿️semio/…/✳️presentation/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎞️pptx/🔖️ecma-376/✳️any/🦀️component.rs`,
all 140 lines): hop one maps `SemioPresentationSnapshot → PptxSnapshot` (a typed Rust struct, NOT
bytes yet: `PptxSnapshot::from_parts(OpcPackage::default(), Vec::new(), PptxPresentation{slides})` at
line 93 — note `OpcPackage::default()` and an EMPTY asset list); hop two is the already-real,
already-oracled `pptx@ecma-376/any` writer (`zip`, already registered as `pptx-ecma-376-mutate`).

The doc comment (lines 4-19) states the losses PLAINLY: `masters`/`layouts` dropped entirely — no
slot in `PptxPresentation`; `Slide::{id, layout_id, notes}` dropped — no field in `PptxSlide`;
`SlideShape::Table` has no pptx counterpart, dropped (not coerced); `Picture::image.{mime,bytes}`
dropped (only the relationship id survives); non-`Paragraph` `DocBlock`s flatten to plain-text
paragraphs (an honest limitation of pptx's own flat-paragraph text frame, not this mapping's).

**Exact mutation vocabulary**, read from `…✳️presentation/🧬️schema/🧬️mutations/🦀️component.rs:26-90` —
this is INDEX-addressed (`slide_index`/`shape_index`, `usize`), not id-keyed, and camelCase on the
wire (`#[serde(rename_all = "camelCase")]` cascades through `SlideFrame`/`SlideShape`'s
`#[serde(tag="shapeKind")]`/`PlaceholderKind`'s `#[serde(tag="kind")]`):

| kind | touches | witnessable via pptx? |
| --- | --- | --- |
| `no-mutation` | nothing | yes (identity) |
| `set-snapshot` | whole document | yes, over the carried fields only (same partial-projection shape as `bcf`'s own `set-snapshot`) |
| `insert-slide` / `remove-slide` | `Slide` at `index` | yes — a `PptxSlide` is added/removed from the presentation's slide list |
| `set-slide-layout` | `Slide.layout_id` | **no** — no field in `PptxSlide` |
| `set-slide-notes` | `Slide.notes` | **no** — no field in `PptxSlide` |
| `insert-shape` / `remove-shape` | a `SlideShape` at `(slide_index, shape_index)` | yes, UNLESS the shape is a `Table` (maps to `None`, so a Table insert/remove is invisible — pick TextBox/Picture/Placeholder recipes) |
| `set-shape-frame` | position/size | yes — `Transform{x,y,cx,cy}` |
| `set-textbox-blocks` | a TextBox's `blocks` | yes, over FLATTENED paragraph text only (matches the exporter's own `block_to_pptx_paragraphs`) |
| `insert-master` / `remove-master` / `insert-layout` / `remove-layout` / `set-layout-master` | masters/layouts | **no** — the whole array is dropped, `OpcPackage::default()` |

**Witnessable, 8 of 15**: `no-mutation`, `set-snapshot` (partial), `insert-slide`, `remove-slide`,
`insert-shape`, `remove-shape`, `set-shape-frame`, `set-textbox-blocks`. **Not witnessable, 7 of 15**:
`set-slide-layout`, `set-slide-notes`, `insert-master`, `remove-master`, `insert-layout`,
`remove-layout`, `set-layout-master` — same "field does not exist in the carrier at all" shape as
`mathematical`'s 10, not a partial/approximate loss.

**Existing supplemental oracle, already registered, DOES NOT discharge the requirement**: this
subset's own `🧪️oracle/🔣️.json` already carries `semio-presentation-python-independent`, explicitly
typed `"kind": "cross-semio-implementation"` — a required SUPPLEMENTAL that the schema's own
`OracleKind` `$defs` structurally cannot use to discharge a mutation's `oracleRequirement`
(`qualifyingKind` must be one of `third-party-library`/`third-party-cli`/`standards-reference-tool`).
Its own rationale (already committed, read in full) already surveyed and DECLINED `python-pptx` for
exactly the reason this pass's own reading confirms independently: "it cannot create slide masters or
slide layouts at all, which removes a third of the vocabulary." That survey stands; a qualifying
`zip`+`quick-xml`(Rust)/`jszip`+`fast-xml-parser`(JS) reference is still owed for the 8 carried kinds.

**Remaining work under the read-only/pipeline-compares shape**: same shape as `bcf` — a `🔬️probes/
📜️script.ts` (vendored `jszip`+`fast-xml-parser`, confirmed present) that reads a real pptx zip and
projects slide/shape structure, a `🏭️generator/📜️script.ts` that constructs BEFORE/AFTER pptx zip pairs
directly (never by applying a mutation), `mutationManifests` split 8 carried / 7
`semio-v1-presentation-mutate-uncarried`, `fixtureManifests`. Not completed before the redirect.

---

## UPDATE — `bcf` probe+generator+fixtures built, and the gate answered its own open question

Built the full read-only/pipeline-compares suite for `bcf` this session:

* `🔬️probes/📜️script.ts` — `bcf-import` (independent-open check), `bcf-project` (guid-keyed semantic
  projection), `bcf-compare` (set/map-keyed deep-equal over two projections). Computes no mutation
  semantics; reads and structurally compares only.
* `🏭️generator/📜️script.ts` — 15 recipes, each a BEFORE (and, where legal, AFTER) bcfzip built
  DIRECTLY by `jszip`+`fast-xml-parser`'s `XMLBuilder`, never by "applying" a mutation. Covers 12 of
  the 14 mutation kinds with a real fixture (`set-snapshot` and `set-viewpoint-snapshot` do not have
  one yet — honestly left out rather than faked).
* **Reproducibility bug found and fixed while building this**: `jszip` auto-creates an implicit
  parent-folder ZIP entry for any nested path (`topic-guid/markup.bcf` auto-creates a `topic-guid/`
  directory entry) and stamps THAT entry with `new Date()`, ignoring the child file's own `date`
  option — confirmed empirically (a nested-path-only zip was non-reproducible across a 2-second gap;
  giving the folder its own explicit `{dir:true, date: FIXED}` entry fixed it). Re-verified
  byte-identical across a real 3-second gap after the fix.
* **Fixtures**: 15 fixture bundles, 27 files, committed at
  `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧫️fixtures/` (the correct
  location — one level up from `🧪️oracle/`, matching `../🧫️fixtures/<id>/<file>` resolved against the
  oracle directory, the same convention `mesh`'s own committed fixtures use; NOT the artifact-root
  `💬️bcf/🧫️fixtures/` where one older, unrelated example file already lived and was left untouched).
* **`fixture verify`** (`bun 🧰️framework/…/🧪️test/📜️script.ts fixture verify --artifact bcf --subset
  any`): `15 fixture(s), 0 file problem(s)`.
* **`fixture reproduce` via the real harness command timed out on an UNRELATED repo-wide crash** —
  `TypeError: undefined is not an object (evaluating 'fixture.target.artifact')` inside
  `matchesFixture`, coming from some OTHER contributor's fixture manifest with a malformed `target`
  (not mine — the aggregation iterates every `🧪️oracle/🔣️.json` in the repo, and this crashed before
  reaching mine). Given the coordinator's own warning about a peer's `clean taxonomy plan` job running
  concurrently, this reads as more of the same repo-wide instability, not a bcf-specific defect — but
  I did not chase down which file it is. **Verified reproducibility directly instead**, replicating
  exactly what `fixture reproduce` does (re-run each fixture's own recorded `generator.command`,
  compare byte digests) without going through the crashing aggregation:
  `15 fixture(s), 27 file(s) checked, 0 problem(s)`.
* **Gate validated BOTH ways, real numbers** (via `bcf-compare`): a byte-identical pair →
  `{equal:true, diffCount:0}`; a pair differing in exactly one field (a viewpoint camera's
  `fieldOfView`, `60` vs `999`) → `{equal:false, diffCount:1, diffs:["$.topics.t1.viewpoints.vp1.camera.fieldOfView: 60 ≠ 999"]}`
  — the gate both accepts a known-good pair and names the exact defect in a known-bad one.
* **`contract` run against the real, current registration** (this answers my own "open question"
  from earlier in this file): it fires the EXACT gate the coordinator warned about, on `bcf` itself —
  `testing/oracle …/💬️bcf/…/🧪️oracle/🦀️component.rs  zip-quick-xml-bcf-2-1-mutate,
  jszip-fast-xml-parser-bcf-2-1-mutate is registered as a qualifying third-party oracle, but this
  owner predicts mutation output in its own Rust`. The gate is keyed on the OWNER, not the individual
  oracle entry — it fires because `bcf`'s PRE-EXISTING `🧪️oracle/🦀️component.rs` (1299 lines, not
  written this session, reimplements every mutation's apply+inverse logic independently in Rust)
  still predicts output, regardless of my new, compliant, read-only `jszip`+`fast-xml-parser` oracle
  sitting alongside it. **This is now a confirmed, not merely suspected, finding**: `bcf` cannot pass
  `contract` with ANY oracle registered until that pre-existing Rust file's mutation-predicting shape
  is resolved (removed, or the gate taught to distinguish a genuinely-independent-but-still-predicting
  reimplementation from the five flagged ones — not this report's call). I did not touch that file.
  Every OTHER contract line for `bcf` is clean: no stub-serializer complaint on the zip carrier
  (only the already-known xml stub), no mutation-manifest-ownership complaint beyond a repo-wide
  "any"-only-subset convention note that predates this work.

Net: the probe, generator, fixtures and gate mechanics are real, tested, and reproducible — the ONE
thing standing between this and a green `contract` is a pre-existing file this ticket did not create
and whose disposition is the coordinator's call.

## What's next, in order

1. **`bcf` and `presentation` are both immediately actionable** with what's already vendored
   (`jszip` 3.10.1 MIT, `fast-xml-parser` 5.11.1 MIT) — no new dependency, no new approval needed,
   `node_modules` already carries both. `mathematical` needs a CSV library decision first (propose
   `papaparse` for approval, or accept the 5-kind surface stays un-oracled).
2. Per the coordinator's own priority ("one finished subset beats three half-done"), the next unit of
   work should be ONE complete `🔬️probes/📜️script.ts` + `🏭️generator/📜️script.ts` + fixtures +
   `mutationManifests`/`fixtureManifests` pass, most naturally `bcf` (14/14 fields witnessable, no
   library gap, existing `comparisonProfiles` entry already in its `🔣️.json`).
3. **Open question for the coordinator**: `bcf`'s own pre-existing `🧪️oracle/🦀️component.rs`
   (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`,
   1299 lines, not written this session) independently reimplements every mutation's apply+inverse
   logic in Rust, composed with `zip`+`quick-xml` for I/O only — the same general shape
   (reimplement-and-compare) the new gate targets, though NOT one of the five names the audit flagged.
   Worth an explicit ruling on whether that file needs to be judged against
   `reimplementation-registered-as-third-party` too, or whether "independent reimplementation of a
   simple, well-specified structural format" is meaningfully different from what broke
   `gltf`/`png`/`jpg`/`bmp`/`tiff`. This report takes no position — flagging it rather than silently
   building the new TS probe on top of an oracle registration that might itself need to change.

## Files touched this session

**Reverted, net zero change:**
* `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/📦️lib.rs` — edited then fully reverted (verified
  969 lines, no `mathematical`/`presentation` mod entries remain, `cargo build --features oracles`
  re-confirmed green).
* `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`
  — created, then deleted.

**New, kept:**
* This markdown file.
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🔬️probes/📜️script.ts` —
  new, `bcf-import`/`bcf-project`/`bcf-compare`.
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🏭️generator/📜️script.ts` —
  new, 15 recipes.
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧫️fixtures/` — new, 15
  directories, 27 files (`before.bcf`/`after.bcf` pairs, 3 rejected-outcome recipes have only
  `before.bcf`).
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` —
  updated: +1 oracle entry (`jszip-fast-xml-parser-bcf-2-1-mutate`, additive — the pre-existing
  `zip-quick-xml-bcf-2-1-mutate` Rust entry is untouched), +3 probes, +1 comparisonPipeline, +1
  comparisonProfile, +`mutationManifests` (14 mutations), +`fixtureManifests` (15 fixtures).

**Untouched (deliberately, per the "open question" above):**
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`
  — the pre-existing Rust reimplementation the `contract` run now confirms trips
  `reimplementation-registered-as-third-party` for this owner.
