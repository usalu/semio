# 📓 S2 — Oracle Registry v2 (kind / engine / productionReachable / networkDuringExecution)

Scope: every `**/🧪️oracle/🔣️.json` file except the already-migrated reference
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🧪️oracle/🔣️.json`
and one deliberately-excluded scratch copy (see "Exclusions" below).

Method: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/🧪️oracle-registry-v2/migrate.py`
(kept, along with the analysis scripts `dump.py`, `dump_full.py`, `count.py`,
`dryrun_test.py`, in the same folder). It loads each file with
`object_pairs_hook=OrderedDict`, adds the four v2 fields to every `oracles[]`
entry, bumps `schemaVersion`, appends one sentence to the `rationale` of every
`cross-semio-implementation` entry, and re-serializes with
`json.dump(..., indent=2, ensure_ascii=False)` plus a trailing newline. A
before/after re-serialization test on an untouched file produced a byte-identical
diff (0 lines), confirming the round-trip preserves key order and formatting
everywhere except the one outlier noted below.

**168 files touched, 136 `oracles[]` entries classified.**

## Counts by assigned kind

| kind | count |
| --- | --- |
| `third-party-library` | 79 |
| `cross-semio-implementation` | 57 |
| `third-party-cli` | 0 |
| `standards-reference-tool` | 0 |
| `metamorphic` / `inverse` / `round-trip` / `property` | 0 |

**Qualifying (third-party-library/cli/standards-reference-tool): 79.**
**Supplemental (cross-semio-implementation/metamorphic/inverse/round-trip/property): 57.**

Registry-wide (including the reference file's 2 pre-existing entries, per the
verification command below): 138 total oracles, 81 qualifying, 57 supplemental.

### A corrected first pass

An earlier internal pass through this same census wrongly called 69/138 entries
`standards-reference-tool` on ISO/ECMA/RFC keyword matches alone, and proposed
engine families named after languages (`rust`, `python`, `javascript`). Both
were wrong and were corrected before anything was written:

- **`standards-reference-tool` is reserved for tooling published BY a standards
  body as the conformance reference** (a W3C validator, veraPDF, a NIST CAx
  harness). None of the 79 named-package references in this registry are that —
  every one is an ordinary crate/PyPI/npm distribution (`ruststep`, `lopdf`,
  `image`, `png`, `gif`, `zip`, `csv`, `calamine`, `quick-xml`, `html5ever`,
  `semver`, `clsx`, `class-variance-authority`, etc.), so **all 79 are
  `third-party-library`**, zero are `standards-reference-tool`, and zero are
  `third-party-cli` (nothing in `oracles[]`, as opposed to `probes[]`, is
  invoked as an external process — every entry here is a linked library).
- **`engine.family` names the kernel/parser, never the ecosystem language.**
  Families below are the actual crate/library identity (`lopdf`, `quick-xml`,
  `stepcode-independent` for `ruststep`, `image-rs` for the generic-sounding
  `image` crate, `json-rs` for the equally generic `json` crate), never `rust`/
  `python`/`javascript`.

## Engine family independence ledger

One `engine` object per oracle `id`, matching the convention the reference
`cc6` file already uses (`ruststep` → `stepcode-independent`, `brepjs-occt` →
`opencascade`). Families with **2+ oracles** are the interesting rows — each
one is a single kernel whose defect would be invisible to every oracle sharing
it:

| family | oracles | notes |
| --- | --- | --- |
| `lopdf` | 11 | All PDF 1.4/1.7 subsets (`a`, `any`, `x`, `e`, `h`, `ua`, `vt`) + plugin-root `lopdf`. Single Rust PDF object-graph library; largest third-party-library concentration in the registry. |
| `quick-xml` | 11 | PPTX strict/transitional, DOCX strict/transitional, SVG any/basic/tiny, XML any/valid, plus the two OOXML `pptx-ecma-376-strict/transitional`/`xlsx-ecma-376-strict/transitional`/`docx-ecma-376-strict/transitional` ids that pair it with `zip`. A `quick-xml` defect ripples across every XML-based format subset simultaneously. |
| `stepcode-independent` | 11 (+1 in the untouched reference file's `ruststep-step-ap214-cc6-mutate` = 12 registry-wide) | All `ruststep` entries: STEP AP214 `any`/`cc1`-`cc5` + IFC `2x3` (`any`/`cobie`/`cv20`/`sav`) + IFC `4`. Deliberately the SAME family as the reference file's own `ruststep` entry, since it is literally the same crate/kernel — this is correct, not a naming drift. |
| `image-rs` | 6 | `image` crate backing three unrelated raster codecs (JPEG/BMP/TIFF) across subset-scoped and plugin-root registrations (`image-jpeg[-jfif-1-01-mutate]`, `image-bmp[-3-mutate]`, `image-tiff[-6-0-mutate]`). One crate is the sole Rust reference for all three formats; a codec-specific `image` bug is invisible to any format that shares it. |
| `zip` | 6 | `zip-2-0-mutate`, `zip-2-0-iso21320-mutate`, `pptx-ecma-376-mutate`, `zip-quick-xml-bcf-2-1-mutate`, `zip-quick-xml-docx-ecma-376-mutate`, plugin-root `zip`. Backs every OPC/container format (PPTX/DOCX/BCF) plus the ZIP subsets themselves. |
| `csv` | 4 | `csv-epw-energyplus-mutate`, `csv-rfc4180-mutate`, `tsv-iana-mutate`, plugin-root `csv`. Same crate reconfigured per delimiter/quoting convention. |
| `gif` | 3 | `gif-87a-mutate`, `gif-89a-any-mutate`, plugin-root `gif`. |
| `flate2` | 2 | `flate2-deflate-rfc1950-mutate`, plugin-root `flate2`. |
| `ifcopenshell` | 2 | IFC2X3 and IFC4 differential entries — same C++/Python engine, two schema versions. |
| `json-rs` | 2 | `json-rust-rfc8259-mutate`, `json-rust-gltf-2-0-mutate` — the `json` crate (not `serde_json`, which is a production dependency and was correctly excluded as an oracle). |
| `png` | 2 | `png-png-1-2-mutate`, plugin-root `png`. |
| `stl-io` | 2 | `stl-io-ascii-mutate`, plugin-root `stl-io`. |
| `tobj` | 2 | `tobj-obj-3-0-mutate`, plugin-root `tobj`. |
| `none` (in-repository second implementation) | 57 | Every `cross-semio-implementation` entry, by definition of the required `{family:"none", implementation:"in-repository second implementation", version:"0"}` triple. Not a genuine independence hazard the way a shared *third-party* kernel is — these are all explicitly supplemental and none of them is claimed to discharge a qualifying-oracle requirement — but flagged here because it is nonetheless the single largest family bucket in the whole registry. |
| all others | 1 each | `calamine`, `clsx`, `comrak`, `cva`, `dxf-rs`, `html5ever`, `las-rs`, `mp4-rust`, `node-semver`, `pdf-writer`, `ply-rs`, `pypdf`, `riff`, `rust-id3`, `simplejson` — singletons, no independence concern. |

`isolationBreaches` (run against the whole post-migration registry, see
verification §3) reports **0** — the framework's own engine-family-vs-mutation
cross-check found nothing wrong with any of the shared-family rows above.

## Full list of `cross-semio-implementation` assignments (57)

All 57 have `"package": ""`. Owner = the plugin/product directory; evidence =
the decisive phrase from that entry's own `rationale`.

| id | owner | evidence |
| --- | --- | --- |
| `writer-python-independent` | `✏️s/🔌️plugins/✒️writer` | "A second implementation of the `s.writer.writer` document and its four typed mutations, in Python" |
| `procedural-2d-python-independent` | `✏️s/🔌️plugins/🌀️procedural` | "A second implementation of … all fourteen typed mutations, in Python" |
| `procedural-3d-python-independent` | `✏️s/🔌️plugins/🌀️procedural` | "A second implementation of … all fourteen typed mutations, in Python" |
| `assembly-python-independent` | `✏️s/🔌️plugins/🌀️procedural` | "A second implementation of … all nine typed mutations, in Python" |
| `gisterrain-python-independent` | `✏️s/🔌️plugins/🌍️gis` | "A second implementation of … both typed mutations, in Python" |
| `gismap-python-independent` | `✏️s/🔌️plugins/🌍️gis` | "A second implementation of … all twelve typed mutations, in Python" |
| `fem2d-python-independent` | `✏️s/🔌️plugins/🏗️fem` | "A second implementation of the `s.fem.fem2d` model and all twenty-five typed mutations, in Python" |
| `fem3d-python-independent` | `✏️s/🔌️plugins/🏗️fem` | "A second implementation of the `s.fem.fem3d` model and all twenty-five typed mutations, in Python" |
| `architect-program-python-independent` | `✏️s/🔌️plugins/🏛️architect` | "A second implementation of the `s.architect.program` … document and all 266 typed mutations, in Python" |
| `lowpoly-python-independent` | `✏️s/🔌️plugins/💠️lowpoly` | "A second implementation of … all seventeen typed mutations, in Python" |
| `forms-python-independent` | `✏️s/🔌️plugins/📋️forms` | "A second implementation of the `s.forms.form` document and all ten typed mutations, in Python" |
| `cad-python-independent` | `✏️s/🔌️plugins/📐️cad` | "A second implementation of the `s.cad.cad` document and all twenty typed mutations, in Python" (the case flagged in this ticket's own `📓️w0-baseline.md` G5 finding) |
| `iso16757-1-python-independent` | `✏️s/🔌️plugins/📕️norm` | "No third-party library in any ecosystem reads or writes `s.norm.iso16757` — checked against PyPI … so this subset's reference is a SECOND IMPLEMENTATION rather than a third-party distribution" |
| `vdi3805-1-python-independent` | `✏️s/🔌️plugins/📕️norm` | same PyPI-survey phrasing as `iso16757` |
| `din4108-1-python-independent` | `✏️s/🔌️plugins/📕️norm` | same PyPI-survey phrasing |
| `din16798-1-python-independent` | `✏️s/🔌️plugins/📕️norm` | same PyPI-survey phrasing |
| `en1990-1-python-independent` | `✏️s/🔌️plugins/📕️norm` | same PyPI-survey phrasing |
| `en1991-1-python-independent` | `✏️s/🔌️plugins/📕️norm` | same PyPI-survey phrasing |
| `en1992-1-python-independent` | `✏️s/🔌️plugins/📕️norm` | same PyPI-survey phrasing |
| `en1993-1-python-independent` | `✏️s/🔌️plugins/📕️norm` | same PyPI-survey phrasing |
| `en1994-1-python-independent` | `✏️s/🔌️plugins/📕️norm` | same PyPI-survey phrasing |
| `en1995-1-python-independent` | `✏️s/🔌️plugins/📕️norm` | same PyPI-survey phrasing |
| `en1996-1-python-independent` | `✏️s/🔌️plugins/📕️norm` | same PyPI-survey phrasing |
| `en1997-1-python-independent` | `✏️s/🔌️plugins/📕️norm` | same PyPI-survey phrasing |
| `en1998-1-python-independent` | `✏️s/🔌️plugins/📕️norm` | same PyPI-survey phrasing |
| `en1999-1-python-independent` | `✏️s/🔌️plugins/📕️norm` | same PyPI-survey phrasing |
| `din18599-1-python-independent` | `✏️s/🔌️plugins/📕️norm` | same PyPI-survey phrasing |
| `playbook-python-independent` | `✏️s/🔌️plugins/📖️playbook` | "A second implementation of the `s.playbook.playbook` document and all nine typed mutations, in Python" |
| `rewrite-python-independent` | `✏️s/🔌️plugins/🔱️trinity` | "A second implementation of the `s.trinity.rewrite` rule document and all seven typed mutations, in Python" |
| `jack-python-independent` | `✏️s/🔌️plugins/🔱️trinity` | "A second implementation of the `s.trinity.jack` scene … and of all eight typed mutations, in Python" |
| `raster-python-independent` | `✏️s/🔌️plugins/🖨️raster` | "A second implementation of the `s.raster.raster` layered document and all twelve typed mutations, in Python" |
| `semio-animation-python-independent` | `✏️s/🔌️plugins/🗄️stdio` | "the second producer THE STANDARD requires is a second IMPLEMENTATION in another language" (no third party reads/writes `.dsl.semio`/`.pack.semio`) |
| `semio-audio-python-independent` | `✏️s/🔌️plugins/🗄️stdio` | same `.dsl.semio`/no-third-party-reader phrasing |
| `semio-brep-python-independent` | `✏️s/🔌️plugins/🗄️stdio` | "the differential comparison's second PRODUCER cannot be a foreign package — it has to be a second implementation" |
| `semio-cad-python-independent` | `✏️s/🔌️plugins/🗄️stdio` | same `.dsl.semio`/no-third-party-reader phrasing |
| `semio-document-python-independent` | `✏️s/🔌️plugins/🗄️stdio` | same `.dsl.semio`/no-third-party-reader phrasing |
| `semio-drawing-python-independent` | `✏️s/🔌️plugins/🗄️stdio` | "No third party reads or writes semio's own `.dsl.semio`/`.pack.semio` envelope" |
| `semio-flow-python-independent` | `✏️s/🔌️plugins/🗄️stdio` | "the second producer a differential comparison needs is a second IMPLEMENTATION" |
| `semio-graph-python-independent` | `✏️s/🔌️plugins/🗄️stdio` | "the differential comparison's second PRODUCER cannot be a foreign package — it has to be a second implementation" |
| `semio-image-python-pillow-independent` | `✏️s/🔌️plugins/🗄️stdio` | **ambiguous — see below**: Pillow is a genuine third-party raster library, but this entry's own `package`/`version` are empty and the carrier/vocabulary half is "a second IMPLEMENTATION … in Python" |
| `semio-kit-python-independent` | `✏️s/🔌️plugins/🗄️stdio` | "the differential comparison's second PRODUCER cannot be a foreign package — it has to be a second implementation" |
| `semio-mesh-typescript-three-independent` | `✏️s/🔌️plugins/🗄️stdio` | **ambiguous — see below**: three.js is a genuine third-party engine, but this entry's own `package`/`version` are empty and the carrier/vocabulary half is "a second IMPLEMENTATION … in TypeScript" |
| `semio-model-python-independent` | `✏️s/🔌️plugins/🗄️stdio` | "The second producer is therefore a second IMPLEMENTATION" |
| `semio-object-python-independent` | `✏️s/🔌️plugins/🗄️stdio` | "the differential comparison's second PRODUCER cannot be a foreign package — it has to be a second implementation" |
| `semio-presentation-python-independent` | `✏️s/🔌️plugins/🗄️stdio` | "No third party reads or writes semio's own `.dsl.semio`/`.pack.semio` envelope" |
| `semio-table-python-independent` | `✏️s/🔌️plugins/🗄️stdio` | "the second producer a differential comparison needs is a second IMPLEMENTATION" |
| `semio-text-python-independent` | `✏️s/🔌️plugins/🗄️stdio` | "this subset's reference is a SECOND IMPLEMENTATION rather than a third-party distribution — which is why `package` is empty" |
| `semio-value-python-independent` | `✏️s/🔌️plugins/🗄️stdio` | "the second producer a differential comparison needs is a second IMPLEMENTATION" |
| `semio-video-python-independent` | `✏️s/🔌️plugins/🗄️stdio` | "the second producer THE STANDARD requires is a second IMPLEMENTATION in another language" |
| `note-python-independent` | `✏️s/🔌️plugins/🗒️note` | "A second implementation of the `s.note.note` document and all thirty-three typed mutations, in Python" |
| `puzzle-2d-python-independent` | `✏️s/🔌️plugins/🧩️puzzle` | "A second implementation of the `s.puzzle.2d` board document and its twenty-six typed mutations, in Python" |
| `puzzle-5d-python-independent` | `✏️s/🔌️plugins/🧩️puzzle` | "A second implementation of the `s.puzzle.5d` assembly document and its twenty-eight typed mutations, in Python" |
| `puzzle-3d-python-independent` | `✏️s/🔌️plugins/🧩️puzzle` | "A second implementation of the `s.puzzle.3d` scene document and its thirty-five typed mutations, in Python" |
| `block-2d-python-independent` | `✏️s/🔌️plugins/🧱️block` | "A second implementation of the `s.block.block2d` node-kind document and all twenty-six typed mutations, in Python" |
| `block-5d-python-independent` | `✏️s/🔌️plugins/🧱️block` | "A second implementation of the `s.block.5d` part-kind document and all forty-one typed mutations, in Python" |
| `block-3d-python-independent` | `✏️s/🔌️plugins/🧱️block` | "A second implementation of the `s.block.3d` object-kind document and all thirty-seven typed mutations, in Python" |
| `curate-python-independent` | `✏️s/🔌️plugins/🪵️sourcing` | "A second implementation of the `sourcing.curate` document and all three typed mutations, in Python" |

This is **57**, matching the coordinator's independently-derived expected count.

## Ambiguous calls and how each was resolved

1. **`semio-image-python-pillow-independent` and `semio-mesh-typescript-three-independent`.**
   Both entries genuinely exercise a real third-party library for HALF their
   evidence (Pillow decodes a real GIF and supplies every raster sample;
   three.js builds a real `BufferGeometry` and supplies every geometry fact),
   which on its face looks like `third-party-library` material. But in both
   cases: (a) the entry's own `package`/`version`/`ecosystem` fields are empty
   — Pillow/three are reached from the host environment rather than pinned as
   this REGISTRATION's own attribution (explicitly not declared via
   `oracleHostPackages`, per the rationale); and (b) the rationale itself
   states the carrier/vocabulary half is answered only by "a second
   IMPLEMENTATION … in Python/TypeScript." Per the task's literal decisive
   signal (empty `package` + "second implementation … from this repository's
   own schemas") and the instruction to prefer the supplemental classification
   under ambiguity, both are classified `cross-semio-implementation`. Each got
   a bespoke (non-templated) appended sentence naming the specific library and
   stating plainly that the unpinned third-party half does not make the
   REGISTRATION itself a qualifying reference.
2. **Scratch copy inside a different, unrelated ticket.** One file matching
   the write-glob,
   `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️energy-support-acceptance/🧾️runs/🧪️energy-support-current-dFiKdD/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`,
   is a duplicated snapshot living inside a *different* ticket's acceptance-test
   "runs" output directory (`END-TO-END-TAXONOMY-NORMALIZATION`, dated
   `☀️17`, i.e. an earlier, unrelated ticket), not a canonical owner
   registration under `✏️s/🔌️plugins/…`. Its `oracles` array is empty (a
   `noOracleDecisions` entry only), so it carries nothing to classify; it was
   deliberately left untouched at `schemaVersion: 1` rather than edited inside
   someone else's in-flight acceptance-test artifact tree.
3. **The `🎞️gif/🔖️87a/✳️any` file had no `schemaVersion` (or `$schema`) key at
   all**, and used compact inline arrays (`"kinds": ["a", "b", …]`) instead of
   the corpus-standard one-value-per-line style every other file already uses.
   `schemaVersion: 2` was inserted as the new first key (matching the position
   every other file uses), and re-serializing through the same `json.dump(...,
   indent=2)` pipeline as everything else necessarily expanded its arrays to
   the standard multi-line form — this is the one file whose diff is not
   "kind/engine-only," but per CLAUDE.md's instruction to fix rather than
   preserve repo inconsistencies in this greenfield codebase, normalizing this
   one outlier to match all 168 other files' formatting was judged correct
   rather than left as a further inconsistency.
4. **Composite oracles naming one package while composing two libraries**
   (`calamine` paired with `rust_xlsxwriter`; `zip` paired with `quick-xml` for
   PPTX/DOCX/BCF). Each such entry declares exactly one `package` (the crate
   that "performs and observes the edit," per its own rationale), so `engine`
   was set to that declared package's family, not the paired container/writer
   — the same one-`id`-one-`engine` convention the reference `cc6` file itself
   uses for `ruststep` vs `brepjs-occt` as two SEPARATE oracle ids rather than
   one merged entry.
5. **`ruststep` sharing `stepcode-independent` with the reference file's own
   `ruststep` entry.** Confirmed deliberate, not an error: it is literally the
   same crate/kernel (`ruststep` 0.4) reused across 12 subset registrations
   (11 touched here + 1 in the untouched reference), so a shared family is the
   factually correct declaration of a real (not accidental) shared-kernel
   relationship.

## Verification (verbatim)

**1. Kind distribution + qualifying count, zero-missing / zero-no-engine check:**

```
{
 "cross-semio-implementation": 57,
 "third-party-library": 81
}
qualifying: 81 of 138
no engine: 0
```

(138 = 136 touched here + 2 in the untouched `cc6` reference file, both
already `third-party-library`: `ruststep-step-ap214-cc6-mutate` and
`brepjs-occt`. 81 = 79 touched here + those same 2.)

**2. Every file still parses as JSON** (checked all 170 files matching the
glob, including the untouched reference and the deliberately-excluded scratch
copy):

```
total files: 170
bad files: 0
```

**3. Isolation breaches:**

```
0
```

Zero breaches — the framework's own engine-family-vs-mutation cross-check
found nothing wrong with any of the shared-family rows in the independence
ledger above (`lopdf`, `quick-xml`, `stepcode-independent`, `image-rs`, `zip`,
etc.), and none of the 57 `cross-semio-implementation` entries were flagged
either.

## Files touched

168 `🧪️oracle/🔣️.json` files under `✏️s/🔌️plugins/…` and `🧰️framework/…`
(exact set = every file matching `**/🧪️oracle/🔣️.json` minus the reference
`✳️cc6` file and the scratch `runs/` copy above — verified by exact-set
comparison against `git diff --name-only -- '*/🧪️oracle/🔣️.json'`, which
returned precisely these 168 plus the reference file, and the reference file's
own diff was independently confirmed to be an unrelated concurrent-agent
fixture addition, not caused by this migration).

Working script and analysis files (kept, per this ticket's file-retention
rule):
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/🧪️oracle-registry-v2/{migrate.py,dump.py,dump_full.py,count.py,dryrun_test.py}`.
Tool-generated intermediates (`entries.json`, `entries_full.json`, `*.log`,
`migration_stats.json`, the `dryrun/` copies) were deleted after use.
