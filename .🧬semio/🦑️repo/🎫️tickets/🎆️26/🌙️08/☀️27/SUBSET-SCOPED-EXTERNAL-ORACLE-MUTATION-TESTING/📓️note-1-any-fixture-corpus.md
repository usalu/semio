# 📓️ note-1/any — third-party-generated fixture corpus

Scope: build `fixtureManifests` (there were none) for `s.note.note@1/✳️any`, the subset a prior
investigation this same session already put under a real qualifying oracle (16 witnessable / 17
honestly un-oracled, three `third-party-library` registrations: `dxf` 0.6, `quick-xml` 0.42, `lopdf`
0.44). See `📓️note-layout-carrier-oracle-findings.md` for that investigation; this file covers only
what it left undone — the fixtures.

## Path taken: standalone Rust binary (not the JS fallback)

`cargo --version` → `cargo 1.99.0-nightly (2f0e7011e 2026-07-05)` — a real, working, network-reachable
cargo was available, so the documented fallback (a second JS-ecosystem oracle) was never needed. Built
a standalone `[workspace]`-scoped crate at
`…✳️any/🏭️generator/🦀️note-oracle-codec/` (`Cargo.toml` + `src/{main,recipes,dxf_codec,svg_codec,pdf_codec,cli}.rs`)
pinning exactly `dxf = "0.6"`, `quick-xml = "0.42"`, `lopdf = "0.44"` — the SAME three crates already
registered as this subset's oracle, reused rather than duplicated. It links nothing from
`semio-s-plugin-stdio` or `semio-s-plugin-stdio-test-oracle`; the broken production crate never enters
the picture. Verified standalone BEFORE anything else was wired up:

```
$ cargo build --manifest-path ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🦀️note-oracle-codec/Cargo.toml
   Compiling note-oracle-codec v0.1.0 (…/🏭️generator/🦀️note-oracle-codec)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 16.85s
```

One binary, two roles, mirroring `…✳️cad/🔬️probes/🦀️oracle-probe`'s shape exactly:

- `generate --out <dir> [--only <recipe-id>]...` — writes `<dir>/<recipe>/{before,after}.<ext>`,
  authored by hand in `recipes.rs` (16 recipes, one per witnessable mutation — never derived by
  executing a mutation, exactly the "author both states directly" discipline
  `…💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🏭️generator/📜️script.ts` uses) and encoded with the
  crate's OWN writers: `dxf::Drawing::save` for DXF, `quick_xml::writer::Writer` for SVG,
  `lopdf::Document::save_to` for PDF.
- `{dxf,svg,pdf}-project --input <path>` / `{dxf,svg,pdf}-compare --input <expected> --input <actual>`
  — read bytes back with the crate's OWN readers (`dxf::Drawing::load`, `quick_xml::reader::Reader`,
  `lopdf::Document::load_mem` + `content::Content::decode`) and emit
  `semio.repository-test.probe-report/v2`.

Two thin bun wrappers shell out to it: `…✳️any/🏭️generator/📜️script.ts` (`generate`/`manifests`,
`--only` merges into `🧫️manifests.json` rather than replacing it, `SEMIO_FIXTURE_OUT` as a fixtures
root — the same CLI shape as `…✳️mesh/🏭️generator/📜️script.ts`) and `…✳️any/🔬️probes/📜️script.ts`
(the six probe ids, resolving `--input` paths against the caller's cwd before `cargo run` changes
directory — the same fix `…✳️cad/🔬️probes/📜️script.ts` needed for the same reason).

## What the fixtures reproduce, and what they deliberately do not claim

The three carrier byte shapes are reproduced STRUCTURALLY, per the prior investigation's own reading
of `NoteIntoDxf`/`NoteIntoSvg`/`NoteIntoPdf`'s bodies (DXF: `Ink.points.windows(2)` → `Line` on layer
`"0"`, no transform, no visibility filter, no width; SVG: every VISIBLE block → `<g
transform="matrix(a,b,c,d,e,f)">` wrapping kind-specific content, `font_size` wired to the text
glyph's Y coordinate — reproduced as-is, not "corrected"; PDF: `title` + every Text block's runs,
space-joined, one page, no visibility filter). They are **not** claimed byte-identical to note's own
serializer, because that serializer cannot currently be compiled or run by anyone to check against —
this is the same evidentiary position `…✳️cad`'s STEP fixtures are in relative to a writer that does
not exist at all, made explicit here rather than implied.

## Recipe → carrier map (16 recipes, 50 fixture files)

| recipe id | mutation | carriers | files |
| --- | --- | --- | --- |
| retitles-the-document | rename-note | pdf | 2 |
| adds-the-diagram-asset | create-asset | svg | 2 |
| swaps-the-logo-payload | replace-asset-payload | svg | 2 |
| removes-the-logo-asset | delete-asset | svg | 2 |
| creates-an-ink-block | create-block | dxf+svg | 4 |
| deletes-the-intro-text-block | delete-block | svg+pdf | 4 |
| deletes-the-ink-and-text-blocks | delete-blocks | dxf+svg+pdf | 6 |
| duplicates-the-ink-block | duplicate-block | dxf+svg | 4 |
| duplicates-the-ink-and-text-blocks | duplicate-blocks | dxf+svg+pdf | 6 |
| drags-the-callout-group-subtree | drag-blocks | svg | 2 |
| moves-the-math-block | move-block | svg | 2 |
| resizes-the-image-block | resize-block | svg | 2 |
| hides-the-intro-text-block | change-block-visible | svg | 2 |
| edits-the-intro-paragraph | edit-block-text | pdf+svg | 4 |
| thickens-the-sketch-stroke | change-block-ink-width | svg | 2 |
| redraws-the-sketch-polyline | edit-block-ink-stroke | dxf+svg | 4 |

**Total: 10 `.dxf` + 30 `.svg` + 10 `.pdf` = 50 committed fixture files**, plus the auto-merged
`🧫️manifests.json` index (same convention as `…✳️mesh`/`…✳️cad`).

For `create-block`/`delete-block`/`duplicate-block` (single-block verbs), the carrier set is honestly
NARROWER than the mutation kind's full declared list (`dxf+svg+pdf`): a single Ink block only reaches
dxf+svg, a single Text block only reaches svg+pdf — no block kind reaches all three at once, so no
single-block scenario legitimately can either. `delete-blocks`/`duplicate-blocks` (plural verbs) touch
an Ink block AND a Text block in the SAME mutation, so those two recipes do legitimately witness all
three declared carriers in one bundle. No fixture pair was written for a carrier that would have been
byte-identical before/after (e.g. no `.pdf` pair for `change-block-ink-width`, whose only declared
carrier is `svg`) — a pair with nothing to witness would prove nothing and was not manufactured to pad
the count.

## Reproducibility — MEASURED, not assumed

Ran `generate` into two independent output directories from two independent invocations and diffed:

```
$ diff -rq /tmp/note-fixtures-test /tmp/note-fixtures-test2
(no output — byte-identical)
```

No wall-clock/counter leak was found in this generator's own path (confirmed empirically, not
assumed, per the reproducibility warning): `lopdf`'s `Document::save_to` was called deliberately
without ever setting `/Producer`/`/CreationDate`, so nothing there to leak; `dxf`'s `Drawing::save`
DOES stamp four wall-clock header vars (`$TDCREATE`/`$TDUCREATE`/`$TDUPDATE`/`$TDUUPDATE`) exactly as
`…✳️cad/🔬️probes/🦀️oracle-probe`'s own `pin_wall_clock` already found and fixed for the same crate —
the identical fix (pin to a fixed Julian day, through the library's own re-parse, never patched into
opaque bytes) is applied here in `dxf_codec.rs::pin_wall_clock`.

## CLI verification — real, quoted output

```
$ bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts fixture verify --artifact note --standard 1 --subset any
[fixture verify] 16 fixture(s), 0 file problem(s)

$ bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts fixture reproduce --artifact note --standard 1 --subset any
[fixture reproduce] 16 generated fixture(s), 0 problem(s)
```

Lands the same way the reference precedent does (`fixture reproduce --artifact bcf` →
`15 generated fixture(s), 0 problem(s)`).

`matrix --artifact note` (informational; this command reports repository-wide, `--artifact` did not
narrow its per-mutation report rows — confirmed by re-running and grepping for `s.note.note::` in the
"no external oracle" list, which returned nothing despite the 17 un-oracled mutations existing; the
per-mutation carrier-fitness distinction this ticket's earlier investigation drew is finer-grained
than what this command's report currently exposes, a pre-existing gap in the harness, not something
this fixture-authoring task changes or should paper over):

```
[matrix] runtimeMutationCoverage           44.12%  30/68
[matrix] subsetOwnershipCoverage          100.00%  658/658
[matrix] externalOracleCoverage            64.29%  423/658
[matrix] oracleEvidenceCoverage            46.81%  308/658
[matrix] oracleCapabilityCoverage          70.83%  34/48
[matrix] fixtureClassCoverage             100.00%  3/3
[matrix] fixtureProvenanceCoverage        100.00%  486/486
[matrix] fixtureReproducibilityCoverage   100.00%  486/486
[matrix] dependencyIsolationCoverage      100.00%  209/209
```
(remaining rows — `expectedOutcomeCoverage`, `inverseCoverage`, `metamorphicCoverage`,
`determinismCoverage`, `implementationCoverage` — are 0.00% repository-wide, an existing, honest empty
denominator this ticket's `📌️status.md` already records; not a note-specific regression.)

## Gate validated BOTH WAYS — real numbers, all three carriers

Every probe run against the COMMITTED fixtures via `…✳️any/🔬️probes/📜️script.ts` (which shells to
the same `note-oracle-codec` binary `fixture reproduce` above already exercised independently).

**DXF** (`redraws-the-sketch-polyline`, a real `edit-block-ink-stroke` point edit):
```
ACCEPT (after.dxf vs after.dxf):
  {"agree":true,"differenceCount":0,"differences":[]}
REJECT (before.dxf vs after.dxf):
  {"agree":false,"differenceCount":4,"differences":[
    "expected line start=[10.0, 10.0] end=[40.0, 60.0] not found in actual",
    "expected line start=[40.0, 60.0] end=[80.0, 20.0] not found in actual",
    "actual has extra line start=[10.0, 10.0] end=[50.0, 70.0] not present in expected",
    "actual has extra line start=[50.0, 70.0] end=[90.0, 30.0] not present in expected"]}
```

**SVG** (`thickens-the-sketch-stroke`, stroke-width 2→5):
```
ACCEPT (before.svg vs before.svg):
  {"agree":true,"differenceCount":0,"differences":[]}
REJECT (before.svg vs after.svg):
  {"agree":false,"differenceCount":1,"differences":[
    "block[1].strokeWidth differs: Some(2.0) vs Some(5.0)"]}
```

**PDF** (`retitles-the-document`, title swap):
```
ACCEPT (before.pdf vs before.pdf):
  {"agree":true,"differenceCount":0,"differences":[]}
REJECT (before.pdf vs after.pdf):
  {"agree":false,"differenceCount":1,"differences":[
    "page text differs: expected [\"Untitled Note Welcome to the note.\"] actual [\"Project Kickoff Notes Welcome to the note.\"]"]}
```

All three carriers validated both directions (exceeds the "dxf and svg required, pdf if time"
minimum) — each REJECT names the exact differing field, never a bare boolean.

## Confirmed: no fixtures attempted for the 17 honestly un-oracled mutations

Per `📓️note-layout-carrier-oracle-findings.md`, each of these already carries an `oracleRequirement`
no registered oracle discharges, and none was given a fixture here:

`change-grid-visible`, `change-grid-spacing`, `change-grid-subdivisions`, `change-grid-opacity`,
`change-snap-enabled`, `change-snap-grid-spacing`, `change-pencil-width`, `change-eraser-radius`
(8× editor-only settings — never written to any carrier), `move-block-to-container` (its diff
re-parents without touching x/y), `rename-block` (no carrier renders `name`), `change-block-locked`
(no carrier reads `locked`), `change-block-font-size` (SVG wires `font_size` to the text glyph's Y
COORDINATE, not a size attribute — a real subject bug, not something a fixture could quietly confirm),
`edit-block-math` (Math always renders as a generic outline rect), `insert-table-row`,
`remove-table-row`, `insert-table-column`, `remove-table-column` (Table always renders as a generic
outline rect keyed to width/height only).

## Files touched

- `…✳️any/🏭️generator/🦀️note-oracle-codec/{Cargo.toml,Cargo.lock,src/{main,recipes,dxf_codec,svg_codec,pdf_codec,cli}.rs}` — new, the standalone codec crate.
- `…✳️any/🏭️generator/📜️script.ts` — new, the bun `generate`/`manifests` wrapper.
- `…✳️any/🔬️probes/📜️script.ts` — new, the bun probe wrapper (6 probe ids).
- `…✳️any/🧫️fixtures/<recipe>/{before,after}.{dxf,svg,pdf}` — new, 50 files across 16 recipes, plus `🧫️manifests.json`.
- `…✳️any/🧪️oracle/🔣️.json` — edited: added `probes` (6 entries, `qualified`, evidence quoting the gate proof above) and `fixtureManifests` (16 entries) between the pre-existing `oracles`/`noOracleDecisions`/`mutationCatalogs`/`comparisonProfiles`/`mutationManifests` blocks, which were NOT otherwise touched (confirmed by `git diff` — the only non-additive lines are `1e-9` → `1e-09`, a cosmetic float-repr change from the JSON round-trip, not a value change).
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📜️patch-note-1-any-oracle-json.py` — new, the (kept, per repo rule) script that performed the `fixtureManifests`/`probes` splice.
- `🔬️note-oracle-verify/` (this ticket folder) — untouched; its proof-of-concept was read but not modified, since the permanent crate above supersedes it as the generator/probe implementation rather than reusing its files in place.
