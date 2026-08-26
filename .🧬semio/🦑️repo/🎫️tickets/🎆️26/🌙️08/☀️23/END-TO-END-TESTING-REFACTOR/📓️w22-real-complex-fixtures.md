# Wave 22 — replacing the toy artifacts

> **THE STANDARD, as raised:** *every single mutation exercised on a REAL-WORLD COMPLEX artifact,
> with a SECOND INDEPENDENT IMPLEMENTATION producing the same result.*

Date 2026-08-26. Answers `📓️w14-final-audit.md` §3 point 3 and §9 recommendation 2, which measured
**30 of 42 conversions (1 321 of 1 860 scenarios) running on artifacts under 4 KiB** — `mutate-semio-text`
on a 203-byte demo note, `mutate-en1990-1` on a 215-byte `.dsl.semio` — and **27 cases (538 scenarios)
reading no artifact at all**.

Measurement tooling, derivation scripts and raw JSON: `w22-fixture-upgrade/`. Every `[test]` line is
copied verbatim from the tool's own stdout and read from the tool's own exit status, never through a
pipe.

---

## 0. What could and could not be measured

**Parity could not be measured — not for one scenario, not for one case, and none is reported here.**
Every Rust SUBJECT host links `semio-framework-plugin`, and that crate does not compile in the
working tree:

```
error[E0432]: unresolved import `component::component_persistent_local`
error: could not compile `semio-framework-plugin` (lib) due to 1 previous error; 103 warnings emitted
```

Verified at the start of this wave on `parity exhaustive --case mutate-semio-text` and again on
`subject exhaustive --case mutate-puzzle-3d-1`. The macro belongs to a live peer session refactoring
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor`, whose files were being written throughout;
they were not touched. **Everything below is verified through the ORACLE phase and through the second
implementation's own in-role laws. No claim is made about the Rust subject half of any case.**

No measured ratio was written into any source file this wave touched, and none was already there —
`grep -rn parity --include=component.feature ✏️s` finds only prose and "recorded in the ticket, not
here".

---

## 1. Before — every `mutate-*` case, measured first

`w22-fixture-upgrade/measure.ts` walks `discoverTestCases`, resolves every `asset://`/`shared://`/
`local://` URI through the repository's own `fixtureUrisIn` + `resolveFixtures`, stats the result on
disk, and excludes per-kind specification vectors (URIs under `🧬️mutations/<kind>/🧪️tests/`) from the
artifact size — a handcrafted `(before, mutation, after)` triple is a third statement of the verb's
meaning, not a document. Raw: `measure.json`.

**145 `mutate-*` cases, 4 862 scenarios. 60 cases / 2 158 scenarios ran on an artifact under 4 KiB,
11 of them on no artifact at all.**

## 2. After — eleven cases re-pointed at a real complex artifact, 307 scenarios, all green

Re-measured with the same script into `measure-after.json`.

| case | before | after | factor | sc | oracle-phase result |
|---|---:|---:|---:|---:|---|
| `mutate-xml-1-0-valid` | 631 B | **40 440 B** | 64× | 19 | `executed=19 passed=19 failed=0 errored=0` |
| `mutate-semio-text` | 203 B | **70 816 B** | 349× | 22 | `executed=22 passed=22 failed=0 errored=0` |
| `mutate-semio-audio` | 226 B (document) | **72 341 B** | 320× | 31 | `executed=31 passed=31 failed=0 errored=0` |
| `mutate-semio-kit` | 734 B | **78 066 B** | 106× | 46 | `executed=46 passed=46 failed=0 errored=0` |
| `mutate-semio-brep` | 537 B | **90 063 B** | 168× | 40 | `executed=40 passed=40 failed=0 errored=0` |
| `mutate-xml-1-0` | 747 B | **92 873 B** | 124× | 17 | `executed=17 passed=17 failed=0 errored=0` |
| `mutate-gif-87a` | 2 936 B | **117 704 B** | 40× | 25 | `executed=25 passed=25 failed=0 errored=0` |
| `mutate-semio-graph` | 297 B | **131 964 B** | 444× | 34 | `executed=34 passed=34 failed=0 errored=0` |
| `mutate-svg-1-1-basic` | 1 463 B | **138 219 B** | 94× | 23 | `executed=23 passed=23 failed=0 errored=0` |
| `mutate-semio-video` | 172 B (document) | **220 106 B** | 1 280× | 28 | `executed=28 passed=28 failed=0 errored=0` |
| `mutate-rewrite-1` | 2 454 B | **246 269 B** | 100× | 22 | `executed=22 passed=22 failed=0 errored=0` |

### 2.1 Case → fixture → bytes

Also as a machine-readable table in `w22-fixture-upgrade/📊️before-after.tsv`.

| case | fixture before | before | fixture after | after |
|---|---|---:|---|---:|
| `mutate-xml-1-0-valid` | `shared://📰️macos-uttype-plist.xml` | 631 | `shared://📰️reuse-marketplaces-plist.xml` | 40 440 |
| `mutate-semio-text` | `asset://…/✳️text/📚️examples/📃️note/🖼️assets/🗣️example.dsl.semio` | 203 | `local://🗣️zukunft-bau-entwerfen-mit-bestand.dsl.semio` | 70 816 |
| `mutate-semio-audio` | `asset://…/✳️any/📚️examples/🎵️tone/🖼️assets/🗣️example.dsl.semio` | 226 | `local://🗣️bauen-mit-bestand-ausschnitt.dsl.semio` | 72 341 |
| `mutate-semio-kit` | `asset://…/✳️kit/📚️examples/🪑️furniture/🖼️assets/🗣️example.dsl.semio` | 734 | `local://🗣️nakagin-capsule-tower.dsl.semio` | 78 066 |
| `mutate-semio-brep` | `asset://…/✳️any/📚️examples/🧊️solid/🖼️assets/🗣️example.dsl.semio` | 443 | `local://🗣️hexagonal-cut-concrete-forest-left.dsl.semio` | 90 063 |
| `mutate-xml-1-0` | `shared://📰️ooxml-word-document.xml` | 747 | `shared://📰️ooxml-readme-document.xml` | 92 873 |
| `mutate-gif-87a` | `shared://🖼️dancing-87a.gif` | 2 936 | `shared://🖼️dancing-87a-large.gif` | 117 704 |
| `mutate-semio-graph` | `asset://…/✳️graph/📚️examples/🕸️wires/🖼️assets/🗣️example.dsl.semio` | 297 | `local://🗣️nakagin-capsule-tower.dsl.semio` | 131 964 |
| `mutate-svg-1-1-basic` | `shared://mouse.svg` | 1 463 | `shared://🎨️semio-brand-and-onboarding.svg` | 138 219 |
| `mutate-semio-video` | `asset://…/✳️any/📚️examples/🎥️clip/🖼️assets/🗣️example.dsl.semio` | 172 | `local://🗣️bauen-mit-bestand-ausschnitt.dsl.semio` | 220 106 |
| `mutate-rewrite-1` | `local://♻️nakagin-ground-floor.snapshot.json` | 2 454 | `local://♻️nakagin-capsule-tower.snapshot.json` | 246 269 |

Every "before" fixture in this table is still read by its case — see §3.2. The `mutate-semio-audio`
and `mutate-semio-video` "before" figures are the DOCUMENT the mutations ran on; the audit's
per-case maxima for those two (1 145 B and 1 815 B) were per-kind specification-vector JSON, not
artifacts.

Whole-artifact re-run over `🧿️semio` after its seven conversions, to prove nothing else there
regressed:

```
[test] level=exhaustive cases=19 executed=692 passed=692 failed=0 errored=0 parity=0/0 not-exercised=1
```

(the one not exercised is `mutate-semio-any`, a recorded `@no-oracle-` case whose evidence is
discharged by the blocked subject phase.)

`contract` after every edit: the same **two** pre-existing `testing/discovery` breaches the w14 audit
recorded and nothing else — no `missing-fixture`, no `orphan-fixture`, no `missing-oracle`, no
`feature-syntax`.

| | before | after |
|---|---:|---:|
| cases on an artifact under 4 KiB | 60 | **49** |
| scenarios in those cases | 2 158 | **1 851** |
| scenarios moved onto a real artifact ≥ 40 KiB | — | **307** |
| of those, onto ≥ 64 KiB | — | **288** |

---

## 3. Provenance of every new fixture — nothing synthesised

Each derivation script is committed in `w22-fixture-upgrade/`; each feature file records the
provenance in its own description.

| fixture | derived ONCE from | reader |
|---|---|---|
| `mutate-semio-brep/🧫️fixtures/🗣️hexagonal-cut-concrete-forest-left.dsl.semio` (90 063 B) + `.pack.semio` (56 627 B) | `♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/📐️hexagonal-cut-concrete-forest-left-bim.stp`, the real Rhino 8.31 / ST-Developer v19.2 BIM export and the richest B-rep committed here: 167 `VERTEX_POINT`, 270 `EDGE_CURVE` each on a real `B_SPLINE_CURVE_WITH_KNOTS`, 127 `EDGE_LOOP`, 127 `ADVANCED_FACE` on real `PLANE`s, 12 `CLOSED_SHELL`/`MANIFOLD_SOLID_BREP` pairs. Every semio id carries the STEP entity number it came from (`v1666` is `#1666=VERTEX_POINT`) | purpose-written ISO 10303-21 Part 21 reader (`🐍️derive-brep-fixture.py`) |
| `mutate-semio-text/🧫️fixtures/🗣️zukunft-bau-entwerfen-mit-bestand.dsl.semio` (70 816 B) + `.pack.semio` (35 241 B) | `🌐️html/🧫️fixtures/🌐️zukunft-bau-entwerfen-mit-bestand.html`, the real 150 KB TYPO3-published German article already committed as this repository's HTML 5 fixture: 384 real text nodes, 344 real `<strong>`/`<a href>` marks with the real URLs the page links to | Python stdlib `html.parser` (`🐍️derive-text-fixture.py`) |
| `mutate-semio-graph/🧫️fixtures/🗣️nakagin-capsule-tower.dsl.semio` (131 964 B) + `.pack.semio` (67 124 B) | `🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc` (2.5 MB, 24 792 entities): 181 nodes, 179 `IfcRelConnectsPorts` edges, 364 `IfcDistributionPort`s, 366 `IfcPropertySingleValue`s | **IfcOpenShell 0.8.4** (`🐍️derive-graph-fixture.py`) |
| `mutate-semio-kit/🧫️fixtures/🗣️nakagin-capsule-tower.dsl.semio` (78 066 B) + `.pack.semio` (50 019 B) | the same IFC 4 file read as a kit of parts: 12 real `IfcBuildingElementProxyType`s, one design of 180 real capsule pieces with real placement transforms (translation in mm, orientation quaternion from the real `Axis`/`RefDirection` pair), 179 real port-to-port connections | **IfcOpenShell 0.8.4** (`🐍️derive-kit-fixture.py`) |
| `mutate-semio-audio/🧫️fixtures/🗣️bauen-mit-bestand-ausschnitt.dsl.semio` (72 341 B) | the first real second of `🔊️wav/🧫️fixtures/🔊️bauen-mit-bestand-ausschnitt.wav` (8 000 real 16-bit PCM frames at its own 8 000 Hz, scaled by 2⁻¹⁵ — exact in binary32, no resampling) plus the four real ID3v2.3 frames (`TSSE`/`TIT2`/`TPE1`/`TLEN`) of the same recording's `🎵️mp3/🧫️fixtures/🎵️bauen-mit-bestand-ausschnitt.mp3` | Python stdlib `wave` + purpose-written ID3v2 reader (`🐍️derive-audio-fixture.py`) |
| `mutate-semio-video/🧫️fixtures/🗣️bauen-mit-bestand-ausschnitt.dsl.semio` (220 106 B) | eight real `00dc` MJPEG frames of `📼️avi/🧫️fixtures/📼️bauen-mit-bestand-mjpeg.avi` (real `MJPG` four-cc, real 480×432, real 15/1) plus twenty-four real MPEG-1 Layer III frames of the same recording's mp3 | purpose-written RIFF/AVI and MPEG frame readers (`🐍️derive-video-fixture.py`) |
| `mutate-rewrite-1/🧫️fixtures/♻️nakagin-capsule-tower.snapshot.json` (246 269 B, of which 218 839 B is the `trinity.graph` before-fixture) | the same IFC 4 file — and it is the SAME data the committed two-piece rule already carried: the committed document's node ids ARE that file's real `ComposePieceAttributes.composeGuid` values, its port ids ARE its real `ComposeConnector.composeConnectorId` values, and the derived graph's first edge reproduces the committed one address for address | **IfcOpenShell 0.8.4** (`🐍️derive-rewrite-fixture.py`) |
| `📰xml/🧫️fixtures/📰️ooxml-readme-document.xml` (92 873 B) | `word/document.xml` of the real committed `📜️docx/🧫️fixtures/📜️example-readme.docx`, extracted by unzip with no other edit — 414 top-level body blocks, a real 37-row/7-column `w:tbl`, a real XML declaration, paragraphs of up to nine sibling runs | `zipfile` extraction only |
| `📰xml/🧫️fixtures/📰️reuse-marketplaces-plist.xml` (40 440 B) | the real 50-row German building-material-reuse survey `📊️csv/🧫️fixtures/📊️reuse-marketplaces.csv` (50 real rows × 12 real columns), serialised in the same Apple PropertyList 1.0 dialect and under the same real Apple DOCTYPE the committed UTType declaration carries | Python stdlib `csv` + Python stdlib `plistlib` (`🐍️derive-xml-valid-fixture.py`) |
| `🎨️svg/🧫️fixtures/🎨️semio-brand-and-onboarding.svg` (138 219 B) | body-for-body composition of `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🔣️logo_dark.svg` (the real animated brand logo: 23 real `<g>`, 23 real `<path>`, 69 real `<animate>` and 69 real `<animateTransform>`, a real `<title>`) and the committed `mouse.svg` (the only committed SVG that declares a `<clipPath>`) | none — verbatim concatenation of the two real bodies (`🐍️derive-svg-basic-fixture.py`) |
| `🎞️gif/🧫️fixtures/🖼️dancing-87a-large.gif` (117 704 B) | three real frames (0, 20, 40) of the real animated 4.4 MB `💃️dancing` GIF, cropped to 400×400 at (200,200), 400×400 at (50,150) and 32×32 at (120,180) rectangles of real already-decoded palette indices, frame 0's real 256-colour local table promoted to the Global Color Table | the `gif` reference crate, through the existing `gif87a-fixture-derive` tool |

### 3.1 Why two fixtures are compositions, stated rather than buried

`🎨️semio-brand-and-onboarding.svg` and `📰️reuse-marketplaces-plist.xml` are the only two fixtures here
that are not a single real file read through a reader.

* **SVG Basic 1.1's distinguishing rule is the clip-path mechanism.** `mouse.svg` is the only
  committed SVG in this repository that declares a `<clipPath>` at all, and it is 1 463 bytes; every
  larger committed SVG (the logos, the QR code, the 24 metabolism icons — checked with
  `git grep -l clipPath -- '*.svg'`, which returns exactly three paths, all copies of the mouse)
  declares none. No single committed file is both real, complex and in-profile, so the two real
  bodies were concatenated under one document element. No element, attribute or character of the
  result is absent from one of the two sources.
* **XML 1.0 §2.8 makes a document valid only if it carries a DOCTYPE whose Name is the document
  element's name.** Exactly one committed document satisfies that — the real macOS UTType
  declaration, 631 bytes. `git grep -l -I -i '<!DOCTYPE'` over the repository returns only HTML files
  besides it, and none of those is well-formed XML. The survey property list is therefore derived
  from real committed CONTENT through two independent standard-library implementations, in Apple's
  own format and under Apple's own DTD.

### 3.2 Nothing was removed to make room

Every replaced fixture is still read, and `identity-round-trip` in all eleven cases now reads BOTH
documents, so the property the small one carried is kept and stated:

* the committed `✳️text`/`✳️graph`/`✳️brep`/`✳️kit`/`✳️audio`/`✳️video` examples were written by the
  RUST codec, and the Python implementation reproducing them byte for byte is a cross-language
  agreement the new fixtures — written by the Python implementation — cannot restate;
* the committed `🦠️no-mutation.json` vectors of `✳️audio` and `✳️video` are tied to those examples and
  both sides still assert that tie;
* `📰️ooxml-word-document.xml` is the one committed document on which this repository's writer and
  `quick-xml` are known to CONVERGE character for character, which is the finding the
  serialization-form probe exists for;
* `📰️macos-uttype-plist.xml` is the real production document this repository ships;
* `mouse.svg` is still read on its own;
* `🖼️dancing-87a.gif` is the only GIF87a whose entire index buffer a scenario can name literally;
* `♻️nakagin-ground-floor.snapshot.json` is still read, still held to its own two-node shape, and the
  two rules are additionally required to name the same root piece — so a derivation that had drifted
  off the real model would be red.

`git status` over `*🧫️fixtures*` and `*📚️examples*`: **11 added by this wave, 0 deleted.**
`git diff -- '*🔣️component.json'` filtered to `^[+-]\s*"(ignoreKeys|tolerance|arrays|mode)"`:
**nothing.** No scenario and no assertion was removed; `identity-round-trip` gained assertions in all
eleven cases.

### 3.3 One completion of a second implementation, and one unmeasurable risk it creates

`mutate-semio-brep`'s Python `print_number` refused any magnitude whose shortest round-tripping digit
string needs an exponent. The real Rhino export carries 98 such magnitudes — real floating-point
residues down to 1e-18 in plane normals and B-spline control points — and the committed grammar's
`number = INT | FLOAT` has no exponent lexeme in either alternative, so the only lexeme the grammar
admits is a positional one and that is what the implementation now writes. It was derived from the
grammar, not from reading the Rust. **Whether the Rust codec agrees has not been measured and cannot
be until the subject host builds**; `identity-round-trip` is where a disagreement would surface, in
red. This is the one place this wave added evidence it cannot yet check.

### 3.4 One mistake this wave made, and the law that caught it

The first `mutate-xml-1-0` run after re-pointing:

```
[test] level=exhaustive cases=1 executed=17 passed=16 failed=1 errored=0 parity=0/0
  mutate-set-declaration: "set-declaration" left the semantic projection exactly as it found it
```

The new document carries a real `<?xml version="1.0" encoding="UTF-8" standalone="yes"?>`, and the
row's parameters had been left at exactly that declaration — a genuine no-op against the new
artifact. The row now flips the real `standalone="yes"` to `"no"`, so an implementation that drops
the pseudo-attribute on write, or defaults it on read, fails. The observability law caught it, which
is what it is for. A quick-xml 0.42 probe (`w22-fixture-upgrade/xml-decl-probe/`) confirmed
independently that the reference does round-trip `standalone="no"`, so the row is real evidence and
not a tolerated quirk.

---

## 4. The three cargo-hosted cases

`mutate-xml-1-0-valid`, `mutate-svg-1-1-basic` and `mutate-gif-87a` share a Rust oracle host that
links `semio-s-plugin-stdio` and rebuilds for tens of minutes under the concurrent repo-wide
`oracle exhaustive` a peer session was running throughout. Run as one batch:

```
### mutate-xml-1-0-valid
[test] level=exhaustive cases=1 executed=19 passed=19 failed=0 errored=0 parity=0/0
### mutate-svg-1-1-basic
error: spawnSync cargo ETIMEDOUT
### mutate-gif-87a
[test] level=exhaustive cases=1 executed=25 passed=25 failed=0 errored=0 parity=0/0
```

The `mutate-svg-1-1-basic` line is the HARNESS timing its own `cargo` child out under the concurrent
load — not a scenario result and not a compile error; it was re-run on its own (§4.2).

### 4.2 `mutate-svg-1-1-basic`, re-run

```
[test] level=exhaustive cases=1 executed=23 passed=23 failed=0 errored=0 parity=0/0
```

### 4.3 Reds that are NOT this wave's, checked rather than assumed

`oracle exhaustive --owner 🔱️trinity` returns `cases=2 executed=47 passed=46 failed=1`. The one red
is `mutate-jack-1 spec-vector-create-node` — "the committed vector declares a refusal, but the
mutation applied" — which is exactly the divergence `📓️w14-final-audit.md` §8 already recorded as one
of its five, in a case this wave did not touch (`git status` shows `🔌️jack`'s four files staged by an
earlier wave). `mutate-rewrite-1`, the trinity case this wave DID change, reports no failure.

`oracle exhaustive --owner 📕️norm` returns `cases=15 executed=799 passed=795 failed=4`. All four are
in cases a peer session converted at **05:59:44**, before this wave's first edit, in a plugin this
wave never touched:

* `mutate-en1990-1 mutate-insert-variable-action` and `inverse-insert-variable-action` — "the
  committed vector declares this mutation applied, yet this implementation refused it:
  insert-variable-action would seed the composed child slot `qK`";
* `mutate-iso16757-1 identity-round-trip` and `mutate-vdi3805-1 identity-round-trip` — "this
  artifact's carrier cannot be read by a second implementation".

They are that session's findings, recorded here only so a reader of this record does not attribute
them to it. The same holds for the owners it was converting while this record was written —
`--owner 🧱️block` reports `cases=3 executed=237 passed=231 failed=6` and `--owner 🧩️puzzle`
`cases=3 executed=181 passed=175 failed=6`, both entirely inside files written after 06:31 today
(§5.5). The owners this wave DID change are clean: `--owner 🧿️semio` 692/692, `--owner 🌀️procedural`
77/77, and `mutate-rewrite-1` 22/22.

The derived GIF87a was additionally checked against the reference reader before any case ran:

```
$ gif87a-fixture-derive inspect 🖼️dancing-87a-large.gif
screen 400x400 bg=0 par=0, 3 frames
frame 0: width=400 height=400
frame 1: width=400 height=400
frame 2: width=32 height=32
```

---

## 5. Cases that could NOT be given a complex real fixture, and why

### 5.1 Bounded by the artifact schema — 12 norm cases, 672 scenarios (+ 84 more, §5.1.1)

`en1990`, `en1991`, `en1992`, `en1993`, `en1994`, `en1995`, `en1996`, `en1997`, `en1998`, `en1999`,
`din16798` and `din18599` are **flat scalar parameter records**. Counted directly off each
`📸️snapshot/🦀️component.rs`:

| artifact | snapshot fields | repeated/keyed fields (`Vec`/`BTreeMap`/`HashMap`) |
|---|---:|---:|
| en1990 | 7 | **0** |
| en1991 | 33 | **0** |
| en1992 | 36 | **0** |
| en1993 | 75 | **0** |
| en1994 | 23 | **0** |
| en1995 | 21 | **0** |
| en1996 | 23 | **0** |
| en1997 | 23 | **0** |
| en1998 | 50 | **0** |
| en1999 | 27 | **0** |
| din16798 | 63 | **0** |
| din18599 | 14 | **0** |

Not one has a single repeated or keyed collection, so the committed `.dsl.semio` already carries
**every field the schema has**: `mutate-en1990-1`'s 215-byte artifact is a complete `s.norm.en1990`
document, not a truncated one. Growing the file would mean inventing fields the standard's own
parameter set does not have. This is a property of the format, not a fixture choice.

#### 5.1.1 The three norm schemas that are NOT flat — `din4108` (45 sc) and `vdi3805` (39 sc)

`din4108` has 19 fields of which one repeats: `layers: Vec<LayerDocument>`, and a `LayerDocument` is
a `(thickness_m, lambda_w_mk)` pair. It is bounded in practice for the same reason as the flat ones —
a real wall build-up has single-digit layers, and a construction with hundreds is not a real
construction.

`vdi3805` (3 keyed catalogues) and `iso16757` (a product catalogue, already 4 128 B and therefore
outside the under-4-KiB set) are genuinely unbounded schemas: a real VDI 3805 manufacturer dataset or
a real ISO 16757 product catalogue would make either of them a complex artifact. Neither is committed
anywhere in this repository, and neither can be derived from what is — `git ls-files` finds no
manufacturer product data of any kind. These two are the only norm cases where a real source, if one
were ever committed, would change the answer.

### 5.2 Bounded by the artifact schema — `mutate-semio-object`, 28 scenarios

`document = artifact-mark schema-line transform-line brep-line mesh-line properties-line`, where
`transform` is exactly ten numbers and the other three lines are child handles (`[]` or
`[hex, hex]`). The maximum size of an `s.stdio.semio.object` document is fixed by its own grammar at
a few hundred bytes.

### 5.3 Bounded by the repository's real content — 4 cases, 133 scenarios

* `mutate-block-2d-1` (79) — a `block2d` document is a node-KIND definition, not an instance. Its
  fixture is already a documented derivation from the artifact's own committed real example (the
  *Hexagonal Cut Concrete Forest Left* kind: its real camera, six real handle kinds, eleven real
  handles at their real radian angles). The `🧱️block` plugin commits exactly two real kinds and both
  are of this size.
* `mutate-gismap-1` (37) and `mutate-gisterrain-1` (7) — the committed Liège fragment (two positions
  with true WGS84 coordinates, two named routes with real polylines) is the only real GIS vector data
  in the repository. `git ls-files` finds no `.geojson`, no `.gpkg`, no OSM extract and no raster DEM
  anywhere.
* `mutate-curate-1` (10) — a stock entry needs an `availability` and a dimensioned `GeometryRecipe`
  per component. The committed demo's ten real entries are already all carried across. The one
  plausible alternative source, the real 50-row `📊️reuse-marketplaces.csv`, is a survey of
  MARKETPLACES (platform, country, access, data fields, procurement steps) and carries neither
  availabilities nor geometry; using it would mean inventing both columns.

### 5.4 No real source exists — `mutate-raster-1`, 37 scenarios

A `raster.document` is a layer tree over an asset pool, and a `pixel` layer needs a `width`, a
`height` and an `imageKey`. The repository holds real images but no real LAYERED document. The one
candidate surveyed was the real committed `🎞️pptx/🧫️fixtures/🎞️semio-talk.pptx` (7 real slides, real
embedded PNG media): its shapes carry no `a:off`/`a:ext` of their own — they inherit their extents
from the slide layout — so every derived pixel layer's width and height would have to be invented.
Declined for that reason.

### 5.5 Converted by a live peer session DURING this wave — 12 cases, 568 scenarios

A peer session spent this wave converting the `@no-oracle-` backlog to Python oracles, writing files
continuously (latest observed write 07:07 on `mutate-writer-1`, and 07:11 on `mutate-forms-1`/
`mutate-playbook-1` while this record was being written). These cases gained an oracle between this
wave's first and second measurement and were **not touched**, because editing a file another session
is mid-write on destroys its work:

| case | sc | artifact | converted at |
|---|---:|---:|---|
| `mutate-block-3d-1` | 75 | **none** | 06:31 |
| `mutate-block-5d-1` | 83 | **none** | 06:34 |
| `mutate-puzzle-2d-1` | 53 | **none** | 06:45 |
| `mutate-puzzle-3d-1` | 71 | **none** | 06:45 |
| `mutate-puzzle-5d-1` | 57 | **none** | 06:46 |
| `mutate-cad-1` | 41 | **none** | 06:49 |
| `mutate-lowpoly-1` | 35 | **none** | 06:51 |
| `mutate-procedural-2d-1` | 29 | **none** | 06:55 |
| `mutate-procedural-3d-1` | 29 | **none** | 06:55 |
| `mutate-assembly-1` | 19 | **none** | 06:57 |
| `mutate-note-1` | 67 | 545 B | 07:05 |
| `mutate-writer-1` | 9 | 269 B | 07:07 |

**This is the largest remaining opportunity and it is a concrete one.** The three `🧩️puzzle` cases can
be pointed at real committed documents that already exist under their own owner roots and are
reachable through `asset://` with no derivation at all:

```
3 035 069 B  🖐️5d …/📚️examples/🌙️capsule-dream/🖼️assets/🗣️dream.dsl.semio
  168 232 B  🖐️5d …/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio
  128 755 B  🧊️3d …/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio
   93 682 B  ◻2d …/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio
```

What that costs, measured rather than guessed: the peer's `🐍️component.py` for `mutate-puzzle-3d-1`
states in its own header that it does not read `.dsl.semio` and works on the committed JSON
snapshots, and the puzzle examples commit no JSON form. So the work is a Python implementation of the
`s.puzzle.3d` DSL carrier plus 35 parameter sets against the real tower — not a re-point. It is worth
doing; it is not a five-minute change.

### 5.6 Blocked on the subject phase — 17 `@no-oracle-` cases, 329 scenarios

`mutate-os-config-opening`, `mutate-s-home-1`, `mutate-playground-1`, `mutate-vcs-1`,
`mutate-s-space-1`, `mutate-semio-any`, `mutate-energy-model-1`, `mutate-playbook-1`,
`mutate-imperative-1`, `mutate-mathematical-1`, `mutate-wires-1`, `mutate-shooting-1`,
`mutate-present-1`, `mutate-sequence-1`, `mutate-forms-1`, `mutate-flow-1`, `mutate-dag-1`. A
recorded no-oracle case runs **no oracle role at all** — every law it asserts runs in the SUBJECT
role, and the subject host does not build (§0). Re-pointing these today would produce evidence that
cannot be compiled, let alone executed. (Two of them, `mutate-forms-1` and `mutate-playbook-1`, were
being converted by the peer session as this record was written and may no longer be in this list.)


---

## 6. The arithmetic closes

672 (flat norm records) + 45 (`din4108`) + 39 (`vdi3805`) + 28 (`mutate-semio-object`) + 133 (bounded
by the repository's real content) + 37 (`mutate-raster-1`) + 568 (peer-owned, §5.5) + 329 (blocked on
the subject phase, §5.6) = **1 851**, which is exactly the number of scenarios still on an artifact
under 4 KiB after this wave. Every remaining scenario is accounted for by a named reason.

Of those 1 851, **897 are genuinely reachable**: 568 in the twelve cases a peer session owned this
hour (§5.5, of which 415 in seven cases have no artifact at all and four of those can be re-pointed
at documents already committed under their own owner roots), 329 behind the subject-host build (§5.6),
and 84 behind product data nobody has committed (§5.1.1). The other 954 are bounded by the artifact
schemas themselves and no fixture choice can move them.
