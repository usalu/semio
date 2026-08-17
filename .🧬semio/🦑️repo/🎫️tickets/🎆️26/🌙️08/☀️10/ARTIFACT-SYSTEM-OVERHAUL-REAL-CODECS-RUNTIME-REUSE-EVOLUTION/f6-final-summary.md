# F6 Program — Final Summary (for the gate (G) wave)

**Written by**: C6d, the closer of F6d (the last F6 sub-wave). Consolidates the recon pilot + all 4
op-codec fan-out sub-waves (F6a/F6b/F6c/F6d) into one program-wide status for whoever dispatches the
next (gate/G) wave.

## Headline

**31/31 official stdio standards are op-codec-complete.** Every one has a real `protocol::DiffCodec`
impl for its Diff type and real `protocol::OpText`/`protocol::OpBinary` impls for its Mutation type
(no `serde_json`-backed placeholder stub remaining anywhere in the crate). `dsl-migration/
diff-completeness` stdio breach count: **1**, and it is `🏗️ifc/2x3` — the pre-existing 32nd standard
that was explicitly out of scope for this entire program from the recon pilot's very first
classification pass (added by the unrelated sibling ticket
`ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES`, never one of the official 31, never rostered into any
sub-wave). Full crate: **1075 passed, 0 failed** (`cargo test -p semio-s-plugin-stdio --lib`).
`POLICY_DIFF_COMPLETENESS_ALLOWLIST` (`📜️script.ts:2304`): 0 stdio entries — unchanged from before
F6 started. Every breach was resolved by a real implementation; none by allowlisting.

## Scope recap

**What F6 did**: implement the wire-codec layer (`protocol::DiffCodec` for each artifact's Diff type,
`protocol::OpText`/`protocol::OpBinary` for each artifact's Mutation type) on top of the
snapshot/diff/mutation type triad that F1-F5 already built. This replaced placeholder
`serde_json`-based stubs that technically satisfied the trait *laws* but were not genuine handcrafted
grammars (the same anti-pattern `WriterDiff` used before F6 started).

**What F6 explicitly did NOT do**: touch snapshot/diff/mutation SHAPE (owned by F1-F5), touch
`📦️glue.rs`/`📜️script.ts` except via each sub-wave's single designated closer (and even then, only
when a real edit was needed — most sub-waves needed none), fix any of the 4 real `dsl`-crate gaps
this program surfaced (see §4 below — all out of every F6 agent's ownership boundary), or touch
`🏗️ifc/2x3` (never in scope).

## 1. Every standard's derive-vs-hand-roll outcome (all 31 official + the pilot)

### Recon pilot (3 standards, done before any fan-out sub-wave existed)

| Artifact | Standard | Diff path | Mutation path | Note |
|---|---|---|---|---|
| 💾️binary | raw | derive | derive | Simplest standard in the whole program — both sides derive-clean. |
| 🎞️gif | 89a | hand-roll | derive | Diff hand-rolled (3b tri-state only); Mutation derived clean (no enum in `GifSnapshot`'s tree). |
| 🎨️svg | 1.1 | hand-roll | hand-roll | Both sides hand-rolled (3a `SvgNodeDiff` enum AND 3b tri-state on diff; 3a via `XmlNode` on mutation). Became the template every later hand-roll copied. |

### F6a (7 standards) — ply, ifc/4, txt, pdf/1.4, csv, step, xlsx

| Artifact | Standard | Diff path | Mutation path | Real blocker (if hand-roll) |
|---|---|---|---|---|
| ☁️ply | 1.0 | hand-roll | hand-roll | `PlyProperty`/`PlyValue` enums — recon guessed DERIVE, wrong. |
| 🏗️ifc | 4 | hand-roll | hand-roll | `IfcValue` enum, direct + transitive — recon guessed DERIVE, wrong. |
| 📄txt | utf-8 | derive | derive | Matched recon's guess exactly. |
| 📄️pdf | 1.4 | derive | derive | Matched recon's guess exactly (no `PdfValue` enum reachable here, unlike 1.7). |
| 📊️csv | rfc4180 | hand-roll | hand-roll | Diff: tri-state-adjacent `Vec<Option<CsvFieldDiff>>`. Mutation: **the derive-macro hygiene bug** (see §4.1). |
| 📐️step | ap214 | hand-roll | hand-roll | `StepValue` enum, direct + transitive. |
| 📕️xlsx | ecma-376 | hand-roll | hand-roll | `XlsxCellValue` enum + `NamedTripleDiff<K,D,T>` generic-collection blocker (see §4.4). |

### F6b (7 standards) — dwg/ac1018, dwg/ac1024, bmp, stl, las, gif/87a, zip

| Artifact | Standard | Diff path | Mutation path | Real blocker (if hand-roll) |
|---|---|---|---|---|
| 🖊️dwg | ac1018 | derive | derive+wrapper | None — matched recon's guess, second clean-derive-both-sides landing after `binary`. |
| 🖊️dwg | ac1024 | derive | derive+wrapper | None — matched recon's guess; 145KB real fixture round-trips losslessly. |
| 🖼️bmp | v3 | derive | derive+wrapper | None — matched recon's guess exactly. |
| 🟪️stl | ascii | hand-roll | hand-roll | Recon guessed DERIVE, wrong — **nested fixed-arity array bug** (see §4.2). |
| ☁️las | 1.0 | hand-roll | hand-roll | Missing from the recon's table entirely (gap filled this wave). 3b tri-state PLUS **bare-tuple missing-impl gap** (see §4.3). |
| 🎒️zip | 2.0 | hand-roll | derive | Diff: 3b tri-state (`unix_mtime`). Mutation: derived clean. |
| 🎞️gif | 87a | hand-roll | derive | Diff: 3b tri-state (`gct`/`lct`). Mutation: derived clean, matching gif89a's precedent. |

### F6c (7 standards) — bcf, png, deflate, obj, gltf, pptx, pdf/1.7

| Artifact | Standard | Diff path | Mutation path | Real blocker (if hand-roll) |
|---|---|---|---|---|
| 💬️bcf | 2.1 | hand-roll | hand-roll | 3a (`BcfCamera` enum) AND 3b — recon guessed tri-state-only, corrected. Plus `NamedTripleDiff<K,D,T>` generic blocker (see §4.4). |
| 📷️png | 1.2 | hand-roll | hand-roll | 3a (3 enums) AND 3b (8 tri-states, real count vs recon's guessed 12). |
| 🗜️deflate | rfc1950 | hand-roll | derive | Diff: pure 3b, single tri-state. Mutation: derived clean. |
| 🧊️obj | 3.0 | hand-roll | derive | Diff: pure 3b, 3 tri-states. Mutation: derived clean. |
| 🧊️gltf | 2.0 | hand-roll | hand-roll | Worse than guessed: generic `GltfCollectionDiff<T,D>` blocker (see §4.4) + 2nd enum (`GltfCameraProjection`) + 42 tri-states (largest surface in the program). |
| 🎞️pptx | ecma-376 | hand-roll | hand-roll | 3a+3b matched guess, PLUS the same generic-collection-engine blocker docx independently hits (see §4.4). |
| 📄️pdf | 1.7 | hand-roll | hand-roll | 3a (`PdfObject` object-graph enum) + 3b matched guess, PLUS a 2nd Mutation-only enum (`PdfPathSegment`), invisible to the recon's diff-file-only grep. |

### F6d (7 standards, this wave) — docx, md, xml, jpg, json, dxf, tiff

| Artifact | Standard | Diff path | Mutation path | Real blocker (if hand-roll) |
|---|---|---|---|---|
| 📜️docx | ecma-376 | hand-roll | hand-roll | 3a (`DocxBlockDiff`/`DocxBlock`) AND 3b (`style`/`based_on` tri-states) — matched recon's guess exactly. Reused its own prior-wave generic `IndexedTripleDiff`/`NamedTripleDiff` across 7 collection instantiations with one generic codec pair. |
| 📝️md | commonmark | hand-roll | hand-roll | 3a (3 interacting enum kinds — `MdBlocksDiff`/`MdBlock`/`MdInline`, most of any F6 artifact) AND 3b — matched recon's "LARGEST enum count" guess exactly. New structural device: bare-triple bracket-wrapping for a nested `MdBlocksDiff` inside `MdListItemsDiff`. |
| 📰xml | 1.0 | hand-roll | hand-roll | 3a (`XmlNodeDiff`) AND 3b (`declaration`/`doctype`) — matched recon's guess exactly; built directly off svg's `enc_xml_node`/`dec_xml_node` template per the recon's own explicit instruction, verified byte-identical tag mapping. |
| 📷️jpg | jfif-1.01 | hand-roll | hand-roll | 3a (`JpgFrameChange`) AND 3b (3 tri-states) matched guess, PLUS a **second independent confirmation of the bare-tuple `DslField` gap** (see §4.3) — `(u8,u8)` decisively blocks the Mutation side. |
| 🔣️json | rfc8259 | hand-roll | hand-roll | 3a only (`JsonValueDiff`/`JsonValue`/`JsonPathSegment`), zero tri-state — matched recon's guess exactly. |
| 🖊️dxf | r12 | hand-roll | hand-roll | 3a only (`DxfEntityDiff`/`DxfEntity`/`DxfValue`), zero tri-state — matched recon's guess exactly. Added 2 new generic collection-triple cores mirroring the file's own pre-existing structural-diff cores. |
| 🖼️tiff | 6.0 | hand-roll | hand-roll | 3a only (`TiffValues`, a 12-variant enum) — recon's row 15 flagged `DERIVE (probable)` with an explicit `CHECK-ENUM-ELSEWHERE` caveat; the caveat was correct, this is HAND-ROLL not DERIVE. Zero tri-state. |

### Program-wide tally

- **Derive both sides, clean**: `binary`, `txt`, `pdf/1.4`, `dwg/ac1018`, `dwg/ac1024`, `bmp` — 6
  standards.
- **Hand-roll diff / derive mutation**: `gif/89a`, `zip`, `gif/87a`, `deflate`, `obj` — 5 standards.
- **Hand-roll both sides**: `svg`, `ply`, `ifc/4`, `csv`, `step`, `xlsx`, `stl`, `las`, `bcf`, `png`,
  `gltf`, `pptx`, `pdf/1.7`, `docx`, `md`, `xml`, `jpg`, `json`, `dxf`, `tiff` — 20 standards.
- **Total**: 6 + 5 + 20 = **31/31 official standards**, all real, all `cargo test`-confirmed green.
- Hand-roll density climbed sub-wave over sub-wave as the backlog's easier (more derive-friendly)
  standards were consumed first: F6a 0/7 fully-both-hand-roll-dense (2 pure-derive, 5 hand-roll-both);
  F6b 2/7 hand-roll-both (3 pure-derive, 2 split); F6c 5/7 hand-roll-both (0 pure-derive, 2 split);
  F6d **7/7 hand-roll-both** (0 pure-derive, 0 split) — the highest density of the program, consistent
  with the recon's own sizing note that the tail of the backlog skewed toward the more
  enum/tri-state-heavy standards.

## 2. `ifc/2x3` — the one remaining breach, confirmed genuinely out of scope

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
was directly grepped by this closer: no `impl protocol::DiffCodec`, no `dsl::DslDiff` derive
attribute, no `serde_json` reference of any kind — the breach is real, not a checker quirk or false
positive. It was flagged as out of scope in the recon report's own §8 row 5 from the start ("Not one
of the official 31 (32nd, added by a sibling ticket, per STATUS.md — confirm scope before spending an
agent on it)") and correctly never rostered into F6a/F6b/F6c/F6d. If a future wave wants to bring the
`dsl-migration/diff-completeness` stdio breach count to a literal zero (including this out-of-scope
standard), it would need its own small fan-out — `ifc/2x3` shares its model shape with the
already-hand-rolled `ifc/4` (per the recon's own note), so that would likely be a fast hand-roll
reusing `ifc/4`'s template, not a fresh investigation.

## 3. Test / policy state, program-wide

- **Full crate**: 1075 passed, 0 failed, 0 ignored (baseline at the very start of the 4 fan-out
  sub-waves, per the recon report's own final number: 1019, already including the pilot's 3
  standards' +6 law tests). Growth 1019→1075 = **+56 = exactly 4 sub-waves × 7 standards × 2
  mandatory law tests each** (`diff_codec_text_binary_roundtrip_law` +
  `op_text_binary_roundtrip_law`) — an exact match, confirming no incidental test churn beyond the
  program's own mandated 2-tests-per-standard addition.
- **`dsl-migration/diff-completeness` stdio breach trajectory**: recon baseline 28 (after the pilot's
  3) → F6a closer 22 → F6b closer 15 → F6c closer 8 → **F6d closer 1** (`ifc/2x3` only).
- **`POLICY_DIFF_COMPLETENESS_ALLOWLIST`**: 0 stdio entries throughout the entire program — verified
  by every sub-wave closer, including this one. No shortcut was ever taken.

## 4. Real bugs / gaps found across the whole program

Four are genuine, still-unfixed `dsl`-crate/framework gaps (all out of every F6 agent's ownership
boundary — fixing any of them requires editing shared framework files this ticket forbids touching).
Two are self-caught, self-fixed test/implementation bugs entirely within a single artifact's own
ownership boundary (not framework bugs, listed for completeness).

### 4.1 `dsl::DslOps` derive-macro field-name hygiene bug (csv, F6a) — NOT FIXED, shared file

A `Mutation` variant field literally named `record` collides with the `dsl::DslOps` derive macro's
own internal accumulator variable, also named `record`. Produces a confusing `E0308` (expected
reference, found `RecordValue`) rather than the expected `DslField`-not-satisfied error — easy to
mistake for a real field-type bug. Reproduced for real, documented via doc-comment citation on
`CsvMutation`, csv hand-rolled around it (field was NOT renamed — renaming would change the wire
shape, forbidden). Whoever next works on the `dsl` derive macro's codegen should rename its internal
accumulator to something namespaced/unlikely to collide (e.g. `__dsl_record` or similar).

### 4.2 Nested fixed-arity array print/parse bug (stl, F6b) — NOT FIXED, shared file

`[[f64;3];3]`-shaped fields compile clean under `#[derive(dsl::DslDiff)]` but are not round-trip-safe
at *runtime*: the shared `dsl` crate's `Shape::Tuple` printer flattens every nesting level into one
indistinguishable comma-join, and the parser never bounds a nested tuple's own comma-consumption to
its declared arity — it greedily eats the outer tuple's remaining values too. Real, reproduced runtime
failure (`"tuple expects 3 elements, found 9"`). Traced to
`🧰️framework/…/🗣️dsl/🧬️schema/🦀️component.rs`'s `print_shape`/`parse_shape`. Not fixed — documented via
doc comment on `StlTriangle`/`StlDiff`/`StlMutation`. **Flagging for any future wave**: no repo-wide
grep for other `[[T;N];M]`-shaped fields was run by any F6 agent — any other artifact with this shape
will hit the identical bug.

### 4.3 Bare-tuple missing-`DslField`-impl gap (las F6b, jpg F6d — independently confirmed twice) — NOT FIXED, shared file

`dsl` has no blanket `impl DslField for (A,B,...)` for tuples of any arity — confirmed as a decisive,
independently-fatal blocker for `las`'s `(u16,u16,u16)`/`(f64,f64,f64)` fields (F6b) and, separately,
`jpg`'s `SetJfifHeader.version: (u8,u8)` field (F6d, this wave) — a direct grep of every
`impl DslField for …` in `🧰️framework/…/🗣️dsl/🦀️component.rs` confirms blanket/concrete impls exist
only for `bool`/`f32`/`f64`/`String`/`Wire`/`DslValue`/`Vec<T>`/`BTreeMap<String,T>`/`[T;N]` — no
tuple arm of any arity. Same root-cause family as the tri-state (§3b) gap — a missing blanket impl —
but a different type shape. Not fixed in either case (shared framework file; for jpg specifically,
replacing the tuple with e.g. `[u8;2]` was also considered and rejected since it would change the
Mutation shape, forbidden by this ticket's scope). **Two independent confirmations across two
different sub-waves increases confidence this is a real, worth-prioritizing gap** for whoever next
works on the `dsl` crate's `DslField`/`Shape` machinery.

### 4.4 Generic collection-diff types have no `DslField` bridge (gltf/pptx/docx F6c+F6d, bcf F6c, xlsx F6a) — NOT FIXED, shared file

Every generic collection-diff wrapper type this program encountered
(`GltfCollectionDiff<T,D>`, `IndexedTripleDiff<D,T>`, `NamedTripleDiff<K,D,T>`) has zero generics
support in the `dsl::DslDiff`/`DslOps` derive macros — confirmed by literal **malformed codegen**
(`E0107`, not just a missing-impl error) for gltf's and pptx's generic wrappers. `xlsx` (F6a) and
`bcf` (F6c) each independently hit the same `NamedTripleDiff<K,D,T>: DslField` blocker; `docx` (F6d,
this wave) hit the same family via `IndexedTripleDiff<D,T>`/`NamedTripleDiff<K,D,T>` and, rather than
working around it, wrote ONE generic `enc_indexed_triple`/`enc_named_triple` codec pair reused across
7 collection instantiations — the most direct practical answer to this gap found across the whole
program (a hand-rolled generic bridge, not a derive-macro fix). Not fixed at the framework level
(shared `dsl` crate file, out of scope) — documented via doc-comment citations at each point of use.
**This is the most-hit gap of the 4** (5 independent artifacts across 2 sub-waves), making it the
strongest candidate for a framework-level fix if a future wave revisits `dsl`'s derive machinery.

### 4.5 `xlsx` empty-string-key drop bug (F6a) — self-caught, self-fixed, NOT a framework bug

Every `dec_*` list-splitter chained a defensive `.filter(|s| !s.is_empty())` after
`split_top_level`, silently dropping a legitimate empty-string OPC relationship-owner key (`""`).
Caught by xlsx's own `diff_codec_text_binary_roundtrip_law` test failing for real; self-fixed in
xlsx's own ownership boundary (all 12 occurrences removed) before that sub-wave's closer ran. Listed
here only for program-wide completeness — no framework-level action needed.

### 4.6 `docx` tri-state test-coverage self-catch (F6d, this wave) — self-caught, self-fixed, NOT a framework bug

The first `diff_codec_text_binary_roundtrip_law` fixture draft did not actually exercise the
`based_on: Some(None)` tri-state transition (both fixture snapshots' shared "keep" style started and
ended at `based_on: None`). Caught by the test's own trailing assertion failing for real (not a
passed-but-untested gap); fixed by giving the fixture's "keep" style a non-`None` starting
`based_on`. Entirely within docx's own ownership boundary, self-fixed before this closer ran.

## 5. Handoff notes for the gate (G) wave

- **31/31 official standards are op-codec-complete and `cargo test`-confirmed green.** No further F6
  fan-out work is needed or should be scheduled.
- **`ifc/2x3` remains 1 stdio breach**, genuinely out of the official 31, correctly untouched by
  design throughout the entire program (see §2). Whether to schedule a small follow-up for it is a
  scope decision for whoever owns `ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES` or the gate wave
  itself — not a defect in F6's own completion.
- **4 real, unfixed `dsl`-crate/framework gaps** are now documented across this program (§4.1-4.4),
  each with a doc-comment citation at its point of use in the affected artifact's source. None block
  the F6 program's own completion (every affected artifact was successfully hand-rolled around its
  respective gap), but all 4 are real engineering debt in the shared `dsl` crate that a future
  framework-focused wave should triage, prioritizing §4.4 (hit by 5 independent artifacts) and §4.3
  (hit by 2 independent artifacts, both confirming the same missing-blanket-impl gap).
- **129 non-stdio `dsl-migration/diff-completeness` breaches exist repo-wide**, entirely outside this
  program's `🗄️stdio`-plugin scope (other plugins — `✒️writer`, `➗️mathematical`, and others — already
  flagged in `POLICY_DIFF_COMPLETENESS_ALLOWLIST`'s own comments as deferred to a separate future
  wave, "W6"). Not this program's concern; noted here only so the gate wave doesn't mistake the
  repo-wide 129 for a stdio shortfall — stdio's own count is 1, not 129.
- No `.gitignore` action, no `glue.rs` edit, no `script.ts` edit, and no
  `POLICY_DIFF_COMPLETENESS_ALLOWLIST` edit was needed at any point across the whole 4-sub-wave
  program — every one of the 28 fan-out standards' breaches resolved on the strength of their own
  real implementation.

## 6. Report index (full paper trail)

- Recon (spec for all of F6, includes the 3-standard pilot): `f6-recon-report.md`.
- F6a: `f6a-closer-report.md`, `f6a-verify-report.md`, plus 7 per-artifact reports
  (`f6-ply-report.md`, `f6-ifc-4-report.md`, `f6-txt-report.md`, `f6-pdf-1.4-report.md`,
  `f6-csv-report.md`, `f6-step-report.md`, `f6-xlsx-report.md`).
- F6b: `f6b-closer-report.md`, `f6b-verify-report.md`, plus 7 per-artifact reports
  (`f6-dwg-ac1018-report.md`, `f6-dwg-ac1024-report.md`, `f6-bmp-report.md`, `f6-stl-report.md`,
  `f6-las-report.md`, `f6-gif-87a-report.md`, `f6-zip-report.md`).
- F6c: `f6c-closer-report.md`, `f6c-verify-report.md`, plus 7 per-artifact reports
  (`f6-bcf-report.md`, `f6-png-report.md`, `f6-deflate-report.md`, `f6-obj-report.md`,
  `f6-gltf-report.md`, `f6-pptx-report.md`, `f6-pdf-1.7-report.md`).
- F6d: `f6d-closer-report.md` (this closer's own report), `f6d-verify-report.md`, plus 7 per-artifact
  reports (`f6-docx-ecma-376-report.md`, `f6-md-report.md`, `f6-xml-report.md`, `f6-jpg-report.md`,
  `f6-json-rfc8259-report.md`, `f6-dxf-r12-report.md`, `f6-tiff-report.md`).
- Full ownership ledger, sub-wave-by-sub-wave: `STATUS.md` (F6a/F6b/F6c/F6d sections plus the "F6
  program — CLOSED" capstone section, all appended in order).
