# 📌️ Status — subset-scoped external-oracle mutation testing

Baseline `a8d1caf41f68204e73ff5e47ce40c5f543ed442d`. Harness **94/94**. TS/Rust/Go/.NET/Python hosts build.

## Measured, not claimed

| | |
| --- | --- |
| Mutation manifests | **22** owners, **432** mutations under the v2 contract |
| External-oracle coverage | **208/658 (31.6%)** — after reclassifying 18 owners whose "third-party oracle" computed its own answers |
| Subset ownership | **658/658 (100%)** — zero wildcard-owned mutations |
| Leaf migration | canonical leaves **28 → ~1520** · descriptors **540 → ~1520** · `semio-s-plugin-stdio` **4620 → 50** errors |
| Fixture provenance | **486/486 (100%)** |
| Fixture reproducibility | **486/486 (100%)** |
| Third-party-generated fixtures | **121** STEP cc6 + **72** BRep kernel (brepjs/OCCT) + **65** mesh (manifold-3d) |
| Leaf descriptors | **1395** (from 540) · payload schemas 88.7% derivable · **67** owners fully derivable |
| Runtime inventory | **30/63 (47.6%)** — `mesh` 17/17 and `brep` 13/13 from production dispatch, 0 differences |

`fixtureReproducibilityCoverage` 1.65% and the three 0.00% rows (`expectedOutcome`, `inverse`,
`metamorphic`, `determinism`) are honest empty denominators: the coordinates are enumerated and
report `missing`, which under this protocol fails rather than reading as 100%.

## What is genuinely externally oracled today

`step@ap214/cc6` is the worked example and it gates for real: **brepjs (OpenCASCADE 8.0 WASM)**
produces the STEP, **ruststep** re-reads it in a different engine family, **manifold-3d** and
**three-mesh-bvh** measure volume/area and symmetric Hausdorff. Meshes are compared in *tessellation
tolerances*, not a fixed constant, so a legitimately different tessellation passes and a wrong solid
does not. Plus `gltf` (120), `png` (15), `jpg` (10), `tiff` (6), `pdf` (5), `bmp` (5), and now
`sequence` (4 of 8), and now **`semio@v1/mesh` — all 17 of 17**, the first owner where every mutation
of the subset is externally oracled. `three` reads the STL/OBJ/PLY/glTF carriers, `manifold-3d`
measures in a different engine family. One mesh through all four readers agrees to 0.000e+00 while a
one-millimetre bore error separates at 1.07e-01. Details and the tolerance assumption it overturned:
`📓️mesh-pilot.md`.

## The finding that reorders the remaining work

**163 serializers do not write the format they declare**: 97 emit `print_dsl` text under a standard
extension, 33 reinterpret their own pack bytes as the target type, 19 coerce through serde into an
empty document, and 14 never read their input at all. The gate shipped finding 80 — four
false-negative classes and one false-POSITIVE class were closed, so 83 exporters had been counted as
real carriers and 2 real ones as stubs. Full breakdown and the reachability map: `📓️reachability.md`. A third-party reader of a standard format
cannot verify a mutation when the artifact writes its own DSL text instead: the parser either fails
outright or, worse, a lenient one accepts garbage. This is now the `stub-serializer` gate.

Eight owners were investigated by reading every `serialize_bytes` body rather than the directory tree,
which lies in both directions. Seven are blocked by the carrier, **not** by any absence of a library —
`tobj 4.0.5`, `stl_io 0.11.0`, `png 0.18.1`, `zip 9.0.0`, `python-pptx 1.0.2` all exist and would work
the day the export writes real bytes. `en1998` is the separate case: verified against npm, PyPI and
crates.io, no Eurocode 8 implementation is published anywhere, and all 49 of its mutations are scalar
field-sets with nothing to recompute.

So the ordering is the reverse of what the goal's phrasing implies. This is not primarily
oracle-research; it is **export correctness**. Details and per-owner verdicts:
`📓️oracle-research-findings.md`.

## Two gate bugs found and fixed this session

* A carrier oracle covered mutations its carrier provably cannot encode, because the gate checked only
  that *some* qualifying oracle declared the capability — capability-level checking standing in for
  per-mutation checking, the exact substitution this protocol exists to forbid. A requirement that
  names an oracle is now discharged only by *that* oracle. Locked by two harness checks.
* `optional: true` previously excused a QUALIFIED probe's hard failure.

## Blocked — narrowed to one identified refactor

`semio-framework` and `semio-framework-plugin` **now compile** (two real bugs fixed — see
`📓️build-unblock.md`); that blocker is gone. Runtime inventory is still 0/9 because
`semio-s-plugin-stdio` does not build: commit `d394744295` (17:14, after this ticket's 11:04 baseline)
ADDED a new `aggregate source is not the taxonomy canonical mutation primary` check to the `Mutations`
derive, and the aggregate files it now demands have not been renamed yet. That is a peer's in-flight
refactor, established by diffing the derive against the baseline rather than inferred, and per repo
rules it is not chased.

## Next

1. **Close the payload-schema tail.** 389 refusals over ~40 domain types (`FemMaterial`, `FemSection`,
   `MapFeature`, …), 2–4 leaves each. Each one unlocks descriptors, which unlock the derive, which
   unlocks that leaf's `MutationLeaf` bound.
2. **Migrate the 1347 nested leaves** to `<leaf>/🦀️.rs` + `dsl::MutationLeaf`. 542 are addressable
   today; the rest wait on step 1. Recipe proven twice (mesh 17, brep 13) and scripted.
3. **Then `brep` executes** with no further work — `test inventory --subset brep` answers from
   production dispatch, exactly as mesh does now.

---

## 2026-08-28 — Reader-oracle retrofit wave complete

Every format subset that *could* be retrofitted with a qualifying reader oracle now has one. The
pattern in each case: the pre-existing `🧪️oracle/🦀️component.rs` (which COMPUTES expected results)
was left untouched and reclassified `cross-semio-implementation`; a separate `third-party-library`
**reader** was registered beside it, judging committed byte-reproducible before/after fixtures
through the third-party crate's own public API. Nothing in this repository predicts what the judge
is judging.

Retrofitted this wave: `obj@3.0`, `dxf@r12`, `gif@89a`, `svg@1.1`, `xml@1.0`, `png@1.2`, `jpg`,
`bmp@v3`, `tiff@6.0`, `gltf@2.0` — alongside the `avi@1.0` reference instance.

### Verified state

| | |
|---|---|
| Harness checks | **116/116** |
| Fixtures | **705** — 100% provenance, 100% byte-reproducible |
| Subset ownership | **658/658 (100%)** |
| External-oracle coverage | 431/658 (65.5%) |
| Oracle **and** evidence | 351/658 (53.3%) |
| Runtime execution | 30/68 — `mesh` 17/17, `brep` 13/13, 0 differences |
| Dependency isolation | 252/252 (100%) |

`brep` now executes from production dispatch; the `📌️Next` item above predicting otherwise is
superseded.

### The 227 uncovered, by cause

Not one uniform gap. Roughly: `pdf` (~120, nine subsets, still on `lopdf-*` cross-semio oracles),
`fem2d`/`fem3d` (~36), `mathematical` (10), `gltf` create/delete (12), `gif@87a` (12), `draw` (3),
`sequence` (4). The rest are **writer-side** limits — kinds where our own encoder discards the field
before any reader could witness it (`tiff::change-byte-order`, `bmp::change-header-fields`, the `jpg`
quantisation/Huffman/restart kinds, `png`'s timestamp and unknown-chunk kinds). Those are export
correctness work, not oracle research — the ordering noted earlier in this file still holds.

### `gif@87a` investigated and documented, not retrofitted

Its twelve kinds stay uncovered deliberately. `gif` 0.13.3 cannot write GIF87a at all — the
signature is hardcoded (`encoder.rs:340`) and a Graphic Control Extension, an 89a-only block, is
emitted for **every** frame (`encoder.rs:178`, unconditional despite its own doc comment saying
"if necessary"). The committed `mosaic-strip.gif` consequently declares `GIF87a` while containing a
GCE. Pillow 11.3.0 writes conformant single-image 87a but sets no interlace bit and reintroduces GCE
for multi-frame.

A partial retrofit of the seven single-image kinds is sound and unblocked; interlace and the three
multi-image kinds are writer-blocked. Three further fixture defects surfaced (a 0-byte example
asset, an 89a file under the 87a subset, a `-large` file whose name contradicts its bytes).
Full evidence: `📓️gif-87a-conformance-and-writer-limits.md`.

### `brep` against the goal's own specification — evidence

The goal names `brep` explicitly: brepjs, STEP files, and meshes that may differ in tessellation but
must agree on Hausdorff distance and volume. All of it is registered **and executing**:

* `brepjs-occt` — `third-party-library`, javascript, **brepjs 18.119.8** (OpenCASCADE 0.15.6).
* **155 STEP fixtures** in the subset; 72 fixture manifests.
* Pipeline `semantic-brep-kernel-v1` — import → validity → `brepjs-reimport-compare`
  (relative volume 1e-8, area 1e-7, centroid and bbox scale-normalized) → topology →
  `manifold-mesh-compare` (**gating**).
* 8 probes, 7 **qualified**; the only provisional one, `step-external-canonicalizer`, is explicitly
  non-gating (`optional: true`, "Reports; gates nothing") and fails only because no stepcode binary
  is installed.

Measured, from `test probe`:

* `brepjs-measure` agrees with closed form at **2.83e-16** (bored box vs 20³ − π·5²·20) and
  **1.71e-16** (tetrahedron vs 10³/6) — the oracle is itself checked against analysis.
* `manifold-mesh-compare` (engine family **manifold** 3.5.1, independent of OpenCASCADE):
  different tessellation **passes** — 398 vs 35 716 triangles on a sphere/box cut → **0.903**
  tolerances against a gate of 3; a different solid **fails** at 2.11e-01; a lost internal cavity
  **fails** at 7.00e+03 tolerances with `genusEqual false` despite an identical outer surface.

The Hausdorff bound is expressed in tessellation tolerances rather than as a fixed constant precisely
because of that measured 0.903-vs-7000 separation. Tessellation freedom and solid identity are
therefore distinguished by measurement, not by assertion.

### `pdf` — the largest remaining block, and why the retrofit pattern does not reach it

The four populated pdf 1.7 subsets (`a`, `e`, `vt`, `x`; 58 fixtures) obtain **both** their mutated
bytes and their expected projection from this repository's own crate
(`oracle_apply_mutation`, `project_conformance`). lopdf lays out the seed and serialises — it is a
codec for our own answer. Their `cross-semio-implementation` classification is **correct** and must
not be "fixed" by registering a reader beside it. Closing pdf means regenerating the `after` bytes
through lopdf's own API first: generator work, per subset, not registration work.

Now pinned by `fixtureWriterProvenanceBreaches` (harness 116 → **120**), which flags exactly those
four and none of the eleven reader-oracle subsets.
Detail: `📓️pdf-fixtures-are-not-admissible-evidence.md`.

### 2026-08-28 (later) — `gltf` resource kinds closed: +24

`gltf@2.0/any`'s 24 `-uncarried` resource kinds (create/delete/move/reorder × accessor, buffer,
bufferView, image, sampler, texture) are now covered by a THIRD reader, `gltf-rs-2-0-mutate-reader`
(`gltf` 1.4.1, rust, standalone `[workspace]` crate).

The `-uncarried` label was honest about `three` — its GLTFLoader builds a scene graph, so an
unreferenced resource is interpreted by nothing — but that was generalised into "unwitnessable", which
does not follow. `@gltf-transform/core` was built and validated first, then **rejected on measurement**:
its `Root` exposes no `listBufferViews`, `listImages` or `listSamplers`, folding three glTF arrays into
one `Texture`, so it would have covered 6 of 24 while appearing to cover more. `gltf` 1.4 exposes all
six as separate typed iterators.

The reader caught three real defects in the recipes feeding it before registration — two invalid
documents (an orphaned animation-sampler `output`, a dangling `textures[].source`) and one fixture that
did not exist under its declared kebab-case kind.

| | before | after |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **455/658 (69.15%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **375/658 (56.99%)** |
| Fixtures | 705 | **729** (100% provenance, 100% reproducible) |
| Harness | 116/116 | **120/120** |

48/48 gate directions correct; all 120 bundles regenerate byte-identically; the 96 pre-existing bundles
verified unmodified against `HEAD`. Detail: `📓️gltf-resource-reader-retrofit.md`.

### 2026-08-28 (later still) — `pdf@1.7/vt` rebuilt on lopdf: +16

The first pdf subset made admissible. Its generator imported `oracle_apply_mutation` and
`project_conformance` from our own crate, so both the mutated bytes and the expected projection were
ours. Replaced by `🏭️generator/🦀️lopdf-engine`, a standalone crate depending on **lopdf alone**, which
performs each mutation through lopdf's public COS API and reads the conformance axes back through it.
The old engine was deleted, not left beside it.

16 of 18 kinds now carry `lopdf-pdf-1-7-vt-mutate-reader`; 32/32 gate directions correct; corpus
byte-reproducible. The two encryption kinds are `-uncarried`: lopdf 0.44's writer requires genuine
encryption state for a `/Encrypt` trailer entry, so a synthetic one can be neither written nor read
back — a writer-side limit, found by the generator's own observability check refusing its own output.

| | before | after |
|---|---|---|
| externalOracleCoverage | 455/658 (69.15%) | **471/658 (71.58%)** |
| Harness | 120/120 | **121/121** |

~86 pdf kinds remain across eight subsets. Detail: `📓️pdf-1-7-vt-rebuilt-on-lopdf.md`.

### 2026-08-28 — every populated pdf 1.7 subset rebuilt on lopdf

`vt` (16), `x` (12), `e` (10), `a` (12), `ua` (11), `h` (10) — **71 kinds** now carry a lopdf READER over
a corpus lopdf itself wrote. Each subset has its own `🏭️generator/🦀️lopdf-engine` (standalone
`[workspace]`, depends on `lopdf` alone); every old `🦀️engine` that imported our mutation engine was
deleted, not left beside it.

`a` added `insert/remove-embedded-file` and `set/remove-af-relationship`; `ua` added the PDF/UA
accessibility axes (MarkInfo, StructTreeRoot, Lang, DisplayDocTitle); `h` added signature fields and
`/Info` author. `ua` and `h` had **no generator and no fixtures at all** before this.

Uncarried: only `insert/remove-encryption-dictionary` in the four subsets that declare them (8 kinds).
lopdf 0.44's writer requires genuine encryption state for a `/Encrypt` trailer entry, so a synthetic
one can be neither written nor read back — a writer-side limit, found by the generator's own
observability check refusing its own output.

`fixtureWriterProvenanceBreaches` now asserts the CLEAN state — 0 generators use our mutation engine —
rather than a count of known offenders.

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **526/658 (79.94%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **446/658 (67.78%)** |
| Fixtures | 705 | **742** (100% provenance, 100% reproducible) |
| Harness | 116/116 | **119/119** |

Remaining pdf: `1.7/base` (16 generic COS kinds), `1.4/a` (2), `1.4/base` (5), `1.4/x` (2) — 25 kinds,
none with fixtures.

### pdf complete

All ten pdf subsets now satisfy `reader + uncarried == kinds`: **96 kinds** carry a lopdf reader over a
corpus lopdf itself wrote, **8** are `-uncarried` (encryption, a writer-side limit). `1.7/ua`, `1.7/h`,
`1.4/a`, `1.4/base` and `1.4/x` had no generator and no fixtures at all before this.

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **546/658 (82.98%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **471/658 (71.58%)** |
| Fixtures | 705 | **767** |
| Harness | 116/116 | **119/119** |

Remaining 112, of which 33 are already recorded `-uncarried`. Detail: `📓️pdf-rebuilt-on-lopdf.md`.

### fem2d + fem3d — 44 kinds closed on the JSON carrier

Both subsets already had two qualifying mesh oracles covering three kinds each; the other 22 each were
`-uncarried`. Correct about the MESH — a material's modulus and a support's DOFs move no triangle —
and wrong as a general claim. Their **json** export is not a stub (unlike csv/md/txt, which wrap the
DSL in a single blob): it is `serde_json::to_value(snapshot)`, the real structured tree, so all nine
collections are carrier-level facts.

`🏭️generator/🦀️json-engine` per subset, `serde_json` as its only dependency, added as a `carrier` mode
on each subset's existing `📜️script.ts`. 88/88 gate directions correct; both corpora byte-reproducible.

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **590/658 (89.67%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **515/658 (78.27%)** |
| Fixtures | 705 | **811** (100% provenance, 100% reproducible) |
| Harness | 116/116 | **119/119** |

### The remaining 68

**34 are already recorded `-uncarried`** with source-verified reasons and are export-correctness work,
not oracle research: obj 10 (tobj is a mesh-only reader), pdf 8 (lopdf cannot write a synthetic
`/Encrypt`), jpg 6, gif@89a 5, png 3, tiff 1, bmp 1.

**34 remain addressable**: `gif@87a` 12 (writer-blocked, see its own report), `mathematical` 10,
`semio` 5, `sequence` 4, `draw` 3. Of these, `mathematical` and `draw` should fall to the fem carrier
pattern; `sequence` will not, because its snapshot is `{schema, content}` with `content` a CHILD
REFERENCE to a separate `s.stdio.semio.flow` artifact — its steps and edges are not in its own carrier.

Detail: `📓️fem-carrier-reader-retrofit.md`.

### Carrier readers extended: draw +3, semio/document +3, semio/cad +2

Same discriminator as fem: is the mutated state INLINE in the subset's own carrier?

* `draw` — `layers: Vec<DrawLayerNode>` is inline, and SVG (its existing reader's carrier) has no
  representation for a layer's `locked`, `blendMode` or authoring `name`. +3.
* `semio@v1/document` — `images: Vec<DocImage>` carries raw bytes inline, which neither the docx nor
  the markdown carrier preserves. +3.
* `semio@v1/cad` — needed no new infrastructure at all: both layer kinds already named a qualifying
  `dxf-crate-cad-r12-read` oracle AND had committed fixtures; the `-uncarried` marker beside them was
  stale. The dxf reader was measured naming each change before the marker was dropped. +2.

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **598/658 (90.88%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **523/658 (79.48%)** |
| Fixtures | 705 | **817** (100% provenance, 100% reproducible) |
| Harness | 116/116 | **119/119** |

The remaining 60 are three different problems, not one backlog — see
`📓️remaining-sixty-anatomy.md`. In short: 34 are export-correctness (our encoder discards the field),
14 are composed-child artifacts whose state never reaches their own carrier (`ArtifactChild`'s
`local_owner` is `#[serde(skip)]` — verified, correcting an earlier estimate here), and 12 are
`gif@87a`, writer-blocked.

### `gif@87a` retrofitted: +10, and the conformance defect fixed

The subset that this ticket documented as writer-blocked is now carried. `gif` 0.13 still cannot write
87a — hardcoded signature, unconditional GCE — so **Pillow 11.3.0 writes** instead, and multi-image 87a
is ASSEMBLED from Pillow's own image blocks (container header from one file, image blocks from several,
trailer), with the source headers asserted byte-identical first so no frame's colours are
re-attributed. Nothing hand-encoded. `gif` 0.13 JUDGES.

The earlier conclusion that rebuilding would "trade one defect for three" was wrong about multi-image
and is corrected in the report. `mosaic-strip.gif` — which declared `GIF87a` while carrying an
89a-only Graphic Control Extension — is gone with the generator that patched it. All 20 committed files
verified `GIF87a` with no GCE, and the reader's projection now leads with the DECLARED VERSION (from
gif's own header parser), so that defect cannot recur unnoticed.

10 carried, 2 `-uncarried` (`set-pixel-aspect-ratio`, `set-image-interlace` — both writer-side).

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **608/658 (92.40%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **538/658 (81.76%)** |
| Fixtures | 705 | **826** (100% provenance, 100% reproducible) |

The remaining **50**: 36 recorded `-uncarried` (export-correctness — our own encoder discards the
field), and 14 composed-child (`mathematical` 10, `sequence` 4) whose mutated state never reaches
their own carrier because `ArtifactChild::local_owner` is `#[serde(skip)]`.

### `mathematical` +1, and the remaining 49 collapse to a single cause

`change-coefficient` closed: `equation` is INLINE in the snapshot, so it reaches the JSON export and a
serde_json reader witnesses it. Chasing that produced a sharper diagnosis of everything left.

**All 49 remaining kinds mutate state that no carrier records.** Two mechanisms, one cause:

* **36** — the encoder writes the carrier but omits the field (tiff endianness, bmp private header
  fields, jpg quant/Huffman, png `tIME`/unknown chunks, pdf synthetic `/Encrypt`, gif aspect-ratio and
  interlace, tobj's document-only blindness).
* **13** — `mathematical` 9 and `sequence` 4: the state sits behind an `ArtifactChild` handle whose
  `local_owner` is `#[serde(skip)]`. `MathematicalIntoCsv` emits only graph nodes and honestly declares
  `IoFidelity::Lossy`, which is exactly why its csv oracle covers exactly the five node kinds.

Found on the way and worth fixing separately: **`SequenceIntoJson` declares `IoFidelity::Exact` and is
not** — the steps and edges behind its content handle are never written. No gate checks fidelity labels.

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **609/658 (92.55%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **539/658 (81.91%)** |
| Fixtures | 705 | **827** (100% provenance, 100% reproducible) |
| Harness | 116/116 | **119/119** |

Reaching 658/658 needs no further oracles or libraries — it needs the carriers to record the fields.

### Export work attempted and BLOCKED — `semio-s-plugin-stdio` does not compile

The remaining 49 need exporter changes, not oracles. Attempted; blocked:

```
$ cargo build -p semio-s-plugin-mathematical --offline
error: could not compile `semio-s-plugin-stdio` (lib) due to 124 previous errors
```

106 `error[...]` lines: 54 `E0277`, 37 `E0433` (`cannot find 'any' in 'subsets'`), 14 `E0432`, 1
`E0308` — concentrated in `🧿️semio` (70) and `📜️docx` (30), the shape of the in-flight aggregate
rename recorded here on 2026-08-27. Established as not this ticket's doing: **no error is in a
`🏭️generator` or `🔬️probes` tree**, the erroring files carry git status `A` (auto-commit staging) not
`M`, and every crate this ticket added still builds clean standalone.

Found while attempting it: **`MathematicalIntoJson` and `SequenceIntoJson` both declare
`IoFidelity::Exact` and are not** — their composed children's scenes are never written.
`MathematicalIntoCsv`, which drops strictly less, honestly declares `Lossy`. No gate checks fidelity
labels.

Detail: `📓️export-correctness-is-blocked-on-a-peer-refactor.md`.

### `obj` +3 — the fourth stale-label correction

`tobj` is a MESH reader and discards `mtllib`, `usemtl` and smoothing statements; `three`'s OBJLoader
parses and keeps them. Measured one kind at a time: those three move its projection and are now
carried; the six vertex/texcoord/normal kinds and `set-unknown-statements` do NOT (unreferenced
elements are dropped by this loader too, unknown lines are skipped with a warning) and stay
`-uncarried`. The measurement set the scope and is recorded in the oracle's qualification criteria.

Fixtures are handcrafted OBJ text — the precedent svg and xml already set here, and named explicitly in
the goal statement.

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **612/658 (93.01%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **542/658 (82.37%)** |
| Fixtures | 705 | **830** (100% provenance, 100% reproducible) |

**The `-uncarried` audit is complete.** Four rounds found stale labels (`cad` 2, `draw` 3,
`semio/document` 3, `obj` 3, plus `gltf` 24 and `fem` 44 earlier); there are no more. All 46 remaining
kinds need the CARRIER to record the field — 26 encoder-side, 7 dropped-by-every-reader, 13 behind
`#[serde(skip)]` child handles — and none is closable while `semio-s-plugin-stdio` does not compile.

### +8 from separating the ENCODER gap from the ORACLE gap

This ticket had filed ~26 kinds as blocked because "our encoder discards the field". That conflated the
FIXTURE (needs a third-party writer — `externalOracleCoverage`) with the SUBJECT (needs our encoder —
`runtimeMutationCoverage`). They are separate dimensions in this repo's own matrix and were being
reported as one.

Separating them closed 8 kinds with no exporter change and no dependency on the blocked crate:

* **png +3** — `png` 0.18 has no `tIME` field and skips unknown ancillary chunks; Pillow writes both
  (`PngInfo.add`) and reads them back through `ChunkStream`, validating every CRC.
* **jpg +2** — `image`-rs decodes to pixels and consumes the quantisation tables and JFIF APP0; Pillow
  keeps both and can write the differences (`quality`, `dpi=`).
* **obj +3** — `tobj` is a mesh reader and discards `mtllib`/`usemtl`/smoothing; three's OBJLoader
  keeps them.

Each claim was measured one kind at a time and the rejected ones left `-uncarried`, recorded in the
oracle's own qualification criteria. `tiff::change-byte-order` was re-checked and stays blocked, but
with a sharper reason: `tiff` 0.11 **does** expose `byte_order()` publicly, so the reader can witness
it — no available writer emits big-endian (encoder rejects non-native, Pillow always writes `II`, and
no big-endian TIFF exists in the repo).

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **617/658 (93.77%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **547/658 (83.13%)** |
| Fixtures | 705 | **835** (100% provenance, 100% reproducible) |

Detail: `📓️encoder-gap-is-not-an-oracle-gap.md`. The remaining 41 should each be re-asked as two
questions — can anything WRITE the pair, can anything READ the difference — rather than one.

### `gif@89a` +4 — two different third-party implementations

The high-level `Decoder` consumes extension blocks on the way to frames; the crate's documented
low-level `StreamingDecoder` surfaces them (`SubBlockFinished` + `last_ext()`). The crate cannot WRITE
them (`ExtensionData` has only `Control`/`Repetitions`), so Pillow writes — `comment=` for the 0xFE
block, `loop=` for the 0xFF NETSCAPE2.0 block. Writer and reader are two DIFFERENT third-party
implementations, stronger than the same-library arrangement used elsewhere here.

`set-pixel-aspect-ratio` stays uncarried: no surveyed writer emits that byte in either GIF version.

**The reframing has now closed 12 kinds** — png 3, jpg 2, obj 3, gif@89a 4 — none of which needed the
blocked crate to compile.

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **621/658 (94.38%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **551/658 (83.74%)** |
| Fixtures | 705 | **839** (100% provenance, 100% reproducible) |
| Harness | 116/116 | **119/119** |

37 remain: `mathematical` 9, `pdf` 8, `obj` 7, `sequence` 4, `jpg` 4, `gif@87a` 2, `gif@89a` 1,
`tiff` 1, `bmp` 1.

### `pdf` COMPLETE — +8 encryption kinds

All 104 pdf kinds now carry a qualifying oracle. The two encryption kinds × four conformance subsets
were the last, and both halves of their `-uncarried` reason turned out to be about **lopdf**:

* writing — its writer demands real encryption state, so a synthetic `/Encrypt` fails on its own output
  (recorded earlier);
* reading — handed a GENUINELY encrypted PDF, lopdf decrypts transparently with the empty password and
  reports `is_encrypted() == false`. **This half had never been tested**; the earlier note assumed the
  writer limitation settled it.

`pypdf` 6.14 encrypts and reports it, byte-deterministically — checked three times, which mattered more
here than anywhere else because a randomised encryption key would have failed the reproducibility gate.
The reader is shared across all four subsets.

**The reframing has now closed 20 kinds** — png 3, jpg 2, obj 3, gif@89a 4, pdf 8 — none needing the
blocked crate.

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **629/658 (95.59%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **559/658 (84.95%)** |
| Fixtures | 705 | **847** (100% provenance, 100% reproducible) |

29 remain: `mathematical` 9, `obj` 7, `sequence` 4, `jpg` 4, `gif@87a` 2, `gif@89a` 1, `tiff` 1,
`bmp` 1.

### `gif@87a` interlace +1, `bmp` header +1 — and a correction to this ticket's own measurement

**`set-image-interlace` was recorded uncarried on a faulty measurement of mine.** The test used 4×3 and
8×8 images; `GifImagePlugin.get_interlace` defaults to 1 and only forces 0 when `min(im.size) < 16`
(its @PIL153 workaround), so both test images sat under the threshold and the keyword genuinely did
nothing *for them*. At 16×16 Pillow writes the bit by default and clears it on `interlace=False`, both
under a GIF87a signature. Now carried, with a 16×16 fixture and the correction recorded in the oracle's
own rationale.

`bmp::change-header-fields`: `image`-rs decodes to pixels and passes over the BITMAPINFOHEADER; Pillow
writes its resolution fields (`dpi=` → 3780/11811) and reads them back. The projection covers all ten
header fields, not only the two this fixture moves.

**The reframing has now closed 22 kinds.**

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **631/658 (95.90%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **561/658 (85.26%)** |
| Fixtures | 705 | **849** (100% provenance, 100% reproducible) |
| Harness | 116/116 | **119/119** |

27 remain: `mathematical` 9, `obj` 7, `sequence` 4, `jpg` 4, `tiff` 1, and the GIF
`set-pixel-aspect-ratio` in both versions (2).

### `obj` +6 — and the failure mode this ticket keeps repeating, named

The six v/vt/vn insert-and-remove kinds were recorded uncarried on a real but too-narrow measurement:
an APPENDED unreferenced element is invisible to a mesh loader. But OBJ face indices are ABSOLUTE — a
FRONT insertion changes what every subsequent index resolves to, so all six move the projection. The
projection was widened to resolved positions/normals/uvs; 9/9 both ways.

The remaining limit is recorded, not hidden: an insertion past the last referenced element still would
not be observable. A document-preserving OBJ parser would cover it; none exists offline (npm, PyPI and
the vendored cargo registry all checked).

**Three negatives in this ticket have now been overturned the same way** — `gif@87a` interlace (tested
below Pillow's 16px threshold), `pdf` encryption (only the write half was tested), `obj` vertices
(only an append was tested). Each was true of the instance tried and false of the kind. The guard is to
vary the instance before recording a negative.

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **637/658 (96.81%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **567/658 (86.17%)** |
| Fixtures | 705 | **855** (100% provenance, 100% reproducible) |
| Harness | 116/116 | **119/119** |

21 remain: `mathematical` 9, `sequence` 4, `jpg` 4, `obj` 1, GIF `set-pixel-aspect-ratio` ×2, `tiff` 1.

### `tiff::change-byte-order` +1 — a negative recorded twice, overturned

Both earlier measurements held: the `tiff` encoder rejects a non-native order, and Pillow's `im.save`
emits `II` regardless of prefix. Neither covered Pillow's **IFD serialiser** —
`ImageFileDirectory_v2(ifh=b'MM...')` encodes every tag in the requested endianness, so the library
does the field layout and the generator lays out only the 8-byte header.

Detail that cost two wrong attempts: PIL's `tobytes()` relocates `StripOffsets` itself, so a computed
value double-counts (122 → 244, file reads truncated); leaving it `0` produces a file both PIL and the
tiff crate accept.

The projection carries the pixel checksum beside the order and the generator **refuses a pair whose
checksums differ** — a `change-byte-order` fixture must change the order and nothing else. Both files
decode to checksum 2464 and load in Pillow as 8×8 mode L.

**Four negatives in this ticket have now been overturned the same way** — gif interlace, pdf
encryption, obj vertices, tiff byte order. Each tested an ENTRY POINT and recorded the result as a
capability. The guard: ask what else in the library could do it, and what other instance of the
mutation would look different, before writing the negative down.

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **638/658 (96.96%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **568/658 (86.32%)** |
| Fixtures | 705 | **856** (100% provenance, 100% reproducible) |
| Harness | 116/116 | **119/119** |

20 remain: `mathematical` 9, `sequence` 4, `jpg` 4, `obj` 1, GIF `set-pixel-aspect-ratio` ×2.

### The last 20 — alternatives enumerated per kind, not assumed

Four negatives here were overturned by asking "what else in this library, what other instance". The
remaining twenty were put through that question deliberately:

* **`mathematical` 9 + `sequence` 4** — their text snapshot codec DOES materialise the graph (it
  handles `MathematicalGraph`/`Node`/`Edge` directly), so a carrying carrier exists. But it is the
  **semio DSL**, our own grammar, which no third party parses — a reader we wrote would be the
  predicting-oracle mistake. Needs the exporters; blocked crate.
* **`jpg` 4** — `zune-jpeg` parses the DRI marker but `restart_interval` is `pub(crate)`; Pillow writes
  it and cannot read it back; Huffman accessors are empty and deprecated for removal in Pillow 12;
  `remove-quant-table` has no writer (a JPEG without one is not decodable).
* **`obj` 1** — needs a document-preserving OBJ parser; npm, PyPI and the vendored cargo registry all
  searched, none available offline.
* **GIF `set-pixel-aspect-ratio` ×2** — definitive: the `gif` crate has NO parse path for byte 12; its
  only mention is `encoder.rs:345` writing a hardcoded `0u8`. Pillow surfaces only `background` and
  `version`. Neither reader nor writer exists.

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **638/658 (96.96%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **568/658 (86.32%)** |
| Fixtures | 705 | **856** (100% provenance, 100% reproducible) |
| Harness | 116/116 | **119/119** |

Detail: `📓️final-twenty-with-alternatives-enumerated.md`.

### Inventory enumerated — 1876 crates, all of node_modules, the Python environment

"What else could do this" is only as good as the inventory it is asked against, so the inventory was
enumerated rather than recalled.

**Found:** `gif` **0.14.2** is vendored alongside 0.13.3 — a version I had not been working with.
Checked: `0.14.2/src/encoder.rs:401` writes the same hardcoded `0u8` aspect ratio and has no parse path
for byte 12. The GIF aspect-ratio negative now holds across **both** vendored versions.

**Not found:** any JPEG marker/structure reader beyond Pillow (DRI handler is `Skip`), `image`-rs and
`zune-jpeg` (`restart_interval` is `pub(crate)` at `decoder.rs:144`); any document-preserving OBJ
parser in cargo, npm or PyPI; anything at all that reads the GIF aspect-ratio byte.

Each of the 20 remaining blockers is now stated at the level of a **specific symbol in a specific
version**. Closing any of them needs a library that is not present, or the blocked crate to build.

Final session state: externalOracleCoverage **638/658 (96.96%)** from 431 (65.50%), oracleEvidence
**568/658 (86.32%)**, fixtures **856** at 100% provenance and reproducibility, subset ownership
658/658, dependency isolation 336/336, harness **119/119**, `mesh` 17/17 and `brep` 13/13 executing
with 0 differences.

### `obj` COMPLETE — `set-unknown-statements` carried, on a definitional point

All ten obj document kinds now carry a qualifying oracle. The recorded reason ("OBJLoader skips an
unrecognised line") was true and had been tested with `zz custom 42`. What it missed:
**`unknown_statements` is defined by what THIS SUBSET'S codec models, not by what three does.** The
codec models v/vt/vn/f/g/o/mtllib/usemtl/s; `l` (polyline) and `p` (point) fall outside it and three
parses both into Line/Points. The fixture uses `l`.

Measured alongside what does NOT work — a `#` comment and a `zz` directive are both invisible — so the
claim is no wider than the evidence, and the rationale records the limit.

**Five negatives overturned now.** Four were "an entry point was tested, not a capability". This fifth
is a new variety: **the mutation's own vocabulary was read as if it were the reader's**. When a kind is
named after a gap in OUR model ("unknown", "unsupported", "other"), its instances are whatever our
model omits — which may be ordinary to the reader judging it.

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **639/658 (97.11%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **569/658 (86.47%)** |
| Fixtures | 705 | **857** (100% provenance, 100% reproducible) |

19 remain: `mathematical` 9, `sequence` 4, `jpg` 4, GIF `set-pixel-aspect-ratio` ×2.

### `jpg::remove-quant-table` +1 — the kind was misread, not the library

Recorded as "no writer: a JPEG without a quantisation table is not decodable". True, and the wrong
question — the kind removes a table from the **table list**, and a JPEG may legally have its components
share ONE table instead of two. Pillow's `qtables=[STD,STD]` vs `[STD]` writes exactly that, at the
same mode and size. Measured `['0','1'] -> ['0']`, mode RGB both sides: only the count moves.

**Six negatives overturned, in two distinct families.** Four were *an entry point was tested, not a
capability* (gif interlace, pdf encryption, obj vertices, tiff byte order). Two were *the KIND was
misread and the negative followed* (obj unknown-statements, jpg remove-quant-table) — the more
dangerous family, because the reasoning looks sound and the library is never at fault. The guard: re-read
what the mutation's own schema says it changes before concluding nothing can observe it.

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **640/658 (97.26%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **570/658 (86.63%)** |
| Fixtures | 705 | **858** (100% provenance, 100% reproducible) |

18 remain: `mathematical` 9, `sequence` 4, `jpg` 3 (restart-interval, Huffman ×2), GIF
`set-pixel-aspect-ratio` ×2.

### The inventory itself was the blind spot — `jpg` +2, `gif` +2, GIF complete

This ticket had enumerated 1876 vendored crates, all of node_modules and the Python environment, and
concluded four kinds had no reader anywhere. Every individual finding was right; two were checked
against two library versions.

**The sweep was scoped to LIBRARIES and never to installed CLIs** — though Protocol v2 lists
`third-party-cli` as qualifying in the same sentence as `third-party-library`. One `command -v` sweep
found `djpeg`/`jpegtran` (libjpeg-turbo 3.2.0) and `giftext`/`gifbuild` (giflib 6.1).

* **jpg +2** — `djpeg -v -v` prints Huffman code-length rows and `Define Restart Interval N`; `jpegtran`
  writes both and is LOSSLESS, so each pair differs in marker structure and not in the image (the
  Start-of-Frame line is asserted identical). The tool banner is filtered so the projection can't vary
  by build date.
* **gif +2, artifact COMPLETE** — `giftext` reports `Aspect = N`, the byte both vendored gif crate
  versions write as a hardcoded zero and neither parses. `gifbuild` round-trips a text description
  carrying it, byte-deterministically, preserving the GIF87a signature. Every other descriptor field is
  asserted identical; giftext's echoed input path is filtered out.

**A third failure mode, above the two already named:** *(c) the INVENTORY was scoped, and the scope was
never stated.* The sweep was exhaustive within libraries; nothing said "libraries", so the conclusion
read as "nothing can do this". Guards (a) and (b) both operate inside a candidate set and cannot
question the set. The guard: state what was searched whenever recording that nothing was found — and
search every oracle KIND the protocol names.

| | turn start | now |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **644/658 (97.87%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **574/658 (87.23%)** |
| Fixtures | 705 | **862** (100% provenance, 100% reproducible) |

14 remain: `mathematical` 9, `sequence` 4, `jpg` 1 (`remove-huffman-table`).

### The stdio blocker, diagnosed properly — and one stray reference of mine fixed

The build blocker was recorded as "a peer's in-flight refactor". Checked rather than assumed:

* Last stdio commit **11:09**, now **18:47** — over 7 hours. No `.rs` file in either erroring tree
  (`🧿️semio`, `📜️docx`) touched in 6 hours. Not in flight; **committed and idle**.
* The docx half is an incomplete rename **already in HEAD**: `📜️docx/🦀️component.rs` is clean and HEAD's
  own copy says `subsets::any`, while the directory holds `✳️base`/`✳️strict`/`✳️transitional`. 52 such
  references.

**Of the 336 erroring files, exactly 2 were modified by this session** — and one of them was genuinely
mine to fix. `✳️document/🧬️schema/🧬️mutations/🦀️.rs` imported the DOCUMENT diff helpers
(`dec_image`, `DocBlockDiff`, `SemioDocumentDiff`) from `subsets::any::schema::diff`; its own sibling
snapshot file imports them from `subsets::document::schema::diff`. HEAD had 29 such references and the
working copy 5, so an earlier migration had converted most and left this one.

Fixed — **surgically, not by blanket rename**: `✳️any/🧬️schema` genuinely contains `🧰️triples`,
`🧮️geometry` AND `🔺️diff`, so the four remaining `subsets::any::schema::triples` references in that file
are legitimately shared and were left alone. Only the `diff` path was wrong.

Result: 124 → **122** errors, 336 → **334** erroring files, and `✳️document` no longer errors. The other
file (`✳️drawing`) fails on an unsatisfied trait bound on `rotate::Rotate`, which belongs to the
mutation-leaf derive work, not to a stray path.

**The remaining 122 are not mine and the docx half should not be guessed at**: `any` was split into
three subsets, and whether a call site becomes `base`, `strict`, `transitional`, or must handle all
three, is a semantic decision belonging to whoever made the split. Guessing would produce
compiling-but-wrong code, which is worse than a clear compile error.

`mathematical` 9 + `sequence` 4 stay blocked on that build.

### The stdio blocker, diagnosis sharpened — and my first version of it corrected

The blocker was first recorded here as "the rename target is a semantic choice I should not guess".
Pushing on it further showed that was **less precise than the evidence supports**, and the earlier
task chip carrying that wording was withdrawn and replaced.

**docx (~30 errors) looks mechanical, not semantic.** `✳️any` is an EMPTY directory husk; every
docx-owned symbol referenced through it resolves UNIQUELY in `✳️base` — `DocxAnalyzer`,
`DocxComposer`, `io_registry`, and the module directories `deserializers`, `serializers`, `mutations`,
`snapshot`, `view`, `edit`.

Two traps, both of which produced false readings while checking, now written into the chip:

* `subsets::any::io::decode_zip` is **not docx's** `any` — `decode_zip` lives in
  `🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io`. A rewrite must be ARTIFACT-scoped; a standard-scoped
  pattern already mis-fired exactly this way twice earlier in this ticket.
* `subsets::any::modes::{edit,view}::windows::main` resolves fine — `main` is nested deeper than a
  shallow directory search reaches. My first check reported it missing and it is not.

**semio (~70 errors) is a different family entirely** — unsatisfied trait bounds on mutation leaves
(`Rotate(rotate::Rotate)` at `✳️drawing/🧬️schema/🧬️mutations/🦀️.rs:51`), belonging to the `MutationLeaf`
derive work rather than to any rename.

Not undertaken here: even a perfect docx fix leaves the semio family, so stdio still would not build and
the 13 kinds would still be blocked — and a large speculative refactor of another ticket's area, which
does not unblock this ticket's goal, is not a trade worth making. It is raised as a chip with the full
diagnosis and both traps instead.

### The semio error family traced: a derive rule that contradicts its own message

The ~70 `E0277` errors are all one cause. `dsl::MutationLeaf` cannot be derived for `Rotate` because
`mutation_leaf_kebab` (`✨️derive/🦀️component.rs:463`) requires `bytes.contains(&b'-')` while its call
site rejects with *"semanticKind must be lowercase kebab-case"*. `rotate` IS lowercase kebab-case — as
are `scale`, `ungroup`, `unflatten`. The clause makes single-word kinds unrepresentable.

Not fixed here, and the reason is specific: **this exact relaxation was applied earlier in this session
and deliberately reverted**, because it caused descriptors to be written that then had to be deleted and
the derive stripped again. That reversal was made with context this session no longer holds, and
re-applying a framework-wide derive change on a partial picture — in another ticket's active area — is
not a call to make from here.

Recorded with evidence and both possible resolutions (drop the clause, or rename the kinds) in
`📓️kebab-rule-blocks-single-word-mutation-kinds.md`. Either is a vocabulary decision.

### The stdio blocker halved: 122 → 50 errors, and a wrong revert of mine corrected

`mutation_leaf_kebab` required `bytes.contains(&b'-')` while its call site rejected with *"must be
lowercase kebab-case"* — and `rotate` IS lowercase kebab-case. Checked before changing it:
`semantic_kind` is validated, stored, and compared for EQUALITY against `SEMANTICS.kind`; nothing
splits it on `-`. The clause had no purpose.

**The earlier revert was the mistake, not the relaxation.** The note said it "caused descriptors to be
written that had to be deleted" — but writing descriptors is what the derive does. That roll-back
deferred to the rule instead of asking whether the rule was right.

Removing the clause alone changed nothing: `Rotate` never had the derive. The six `✳️drawing` leaves
were **unmigrated** — payload schema present, no `🔣️.json`, no `dsl::MutationLeaf` — which is this
ticket's own outstanding "migrate the nested leaves" item. All six have single-word kinds, so the rule
was necessary but not sufficient. Descriptors written and derives added for all six.

**122 → 50 errors.** Framework crate still builds; this ticket's gates unchanged.

A docx-only, artifact-scoped rewrite of the remaining rename was tried and **reverted**: it correctly
spared `🎒️zip`'s own `subsets::any`, but took 50 → 95 by exposing 60 `E0046` missing-trait-item errors
and a second `crate::editor::docx::` namespace. That is another ticket's migration, not a rename.
docx is clean against HEAD again.

### stdio: 122 → 37 errors, and `🧿️semio` is now clean

Compiler-driven, one flagged site at a time, never by blanket rename:

| Fix | Effect |
|---|---|
| `mutation_leaf_kebab`: dropped the spurious `contains(&b'-')` | enabled the six single-word kinds |
| Six `✳️drawing` descriptors + `dsl::MutationLeaf` (`rotate`, `scale`, `group`, `ungroup`, `flatten`, `unflatten`) | 122 → 50 |
| `move_vertex::mutation::` → `move_vertex::` (one compiler-flagged site) | 50 → 49 |
| `delete_node::diff::parent_and_index` → `delete_node::parent_and_index`; spurious `use ::is_contiguous_ascending;` removed | 49 → 47 |
| 67 stale `::mutation::` segments across 6 text codecs — the leaf types moved up out of that submodule, so the imported NAMES are unchanged and no usage site moved | 47 → 41 |
| Two more spurious SELF-imports (`::parent_and_index`, `::collect_flattened_leaves` — both defined in the importing file); `SemioDiff` → `schema::diff::`; `SemioDocumentMutation`/`Snapshot`/`DocBlock` → `subsets::document::` | 41 → 37 |

**All 37 remaining errors are in `📜️docx`.** `🧿️semio` — this ticket's own artifact — compiles clean.

### docx: attempted twice, reverted twice, and the reason is now measured

The rename is genuinely correct (`✳️any` is an empty husk; every docx-owned symbol resolves uniquely in
`✳️base`), and covering all three namespaces (`artifacts::`, `editor::`, `viewer::`) rewrites 40
references while correctly sparing `🎒️zip`'s own `subsets::any`.

It takes the count 37 → **75**, and the reason is **module-fails-to-load masking** — the hazard this
ticket named earlier. The unmasked errors are not docx's: 60 `E0046` of the form
`impl Mutation<WavSnapshot> for WavMutation` missing `DESCRIPTORS`/`descriptor`, spanning **31
artifacts** (las, ply, html, epw, zip, gif, pptx, mp4, svg, mp3, ifc, bcf, csv, step, tsv, xlsx, md,
xml, jpg, avi, wav, json, dwg, dxf, tiff, stl, obj, …). That is the repo-wide `Mutations` derive
migration recorded here on 2026-08-27 against commit `d394744295`.

Reverted because it does not unblock this ticket and perturbs another's files — docx is clean against
`HEAD`. The measurement is the useful output: **whoever finishes docx should expect the count to rise
to ~75 and should plan for the 31-artifact aggregate gap behind it**, not read the rise as a regression.

Gates unchanged throughout: 119/119, 862 fixtures, 644/658, framework crate builds.

### The unmasked gap, measured precisely — it is HAND-WRITTEN impls, not missing descriptors

Sized rather than estimated, because the earlier note guessed at its shape:

* **Leaf descriptors are essentially done.** Repo-wide, **390** mutation leaves carry `🔣️.json`; only
  **12** do not, and all 12 also lack a payload schema, so they are genuinely unauthored rather than
  half-migrated. Leaf-side is not the bottleneck.
* **The bottleneck is the aggregates.** 11 artifacts already use the derive —
  `#[derive(…, dsl::Mutations)]` with `#[mutations(snapshot = …, diff = …, schema = "…")]` (see
  `🔣️json/…/✳️base/🧬️schema/🧬️mutations/🦀️.rs:13`). The ~60 `E0046` sites are **hand-written**
  `impl Mutation<XSnapshot> for XMutation` blocks that predate the trait gaining `DESCRIPTORS` and
  `descriptor`. Each needs either replacing with the derive or extending by hand — a judgment call per
  aggregate, across 31 artifacts.

So the correction to this ticket's own earlier note: the aggregate gap is **not** "leaves need
descriptors" (they have them). It is sixty hand-rolled trait impls left behind by a trait change.

That is a different and larger piece of work than a rename, and squarely another ticket's. This ticket
stops here having taken stdio from 122 to 37, cleaned `🧿️semio` entirely, and measured what remains.

### Feature-gating checked as an escape route — there isn't one

Guard (c) again: before concluding the 13 need the whole crate, the assumption itself was tested.
`semio-s-plugin-stdio` declares exactly one feature (`plugin-root`, default), and
`semio-s-plugin-mathematical` already depends on it with `default-features = false`. If that feature
gated the glue pulling in every artifact, the broken ones would be excluded.

It does not. `cargo build -p semio-s-plugin-stdio --no-default-features` fails with the same **37**
errors, and `cargo build -p semio-s-plugin-mathematical` fails identically. The artifacts are compiled
unconditionally, so the 13 kinds genuinely need the whole crate.

Also worth stating plainly: **stdio has never compiled with docx fixed.** The 60 hand-written
`impl Mutation<…>` gaps are not caused by fixing docx — they are revealed by it, and must be resolved
either way.

### Why the 60 impls are a migration, not an edit — the mechanism

`dsl::Mutations` does not merely fill in `DESCRIPTORS`; it BUILDS it, by collecting every direct leaf
under `mutation_root` and calling `validate_mutation_leaf_descriptor_roster`, which demands "a unique
and complete direct leaf descriptor roster" and checks the aggregate sits at the taxonomy-canonical
path (`✨️derive/🦀️component.rs:1819-1830`). That is the same check commit `d394744295` added.

So a hand-written `impl Mutation<XSnapshot> for XMutation` cannot be converted by adding two items.
`🔊️wav`'s mutations directory has **no leaf subdirectories at all** — its kinds live in one flat file.
Converting it means first SPLITTING the aggregate into leaf directories, each with its own `🦀️.rs`,
`🔣️.json` and payload schema, and only then deriving.

Across 31 artifacts that is the 1347-leaf migration this ticket's own `📌️Next` names — multi-session
work, and the direction the peer's commit already set. It is not something to start and leave half-done
in a shared tree.

**This ticket's contribution to it stands at: 122 → 37 errors, `🧿️semio` clean, and the remaining shape
measured rather than guessed.**

### The migration was ATTEMPTED, and the concrete blocker is a wire-format change

Rather than leave "multi-session work" as an assertion, the smallest candidate was opened up.

`📑️tsv`'s aggregate declares **seven INLINE struct variants** — `NoMutation`, `SetSnapshot { … }`,
`SetTrailingNewline { … }`, `SetLineEnding { … }`, `InsertRow { … }`, `RemoveRow { … }`,
`SetCell { … }` — with only `📄set-snapshot` present as a leaf directory. `dsl::Mutations` requires
newtype variants over leaf TYPES, one directory each with `🦀️.rs`, `🔣️.json` and a payload schema.

**That conversion changes the enum's serde representation**, since the aggregate carries
`#[serde(tag = …, content = "payload")]` and the variants' shape is the wire format. Every fixture,
diff codec and text codec that speaks tsv mutations moves with it.

So it is not a mechanical restructure that can be done artifact-by-artifact in a shared tree without
also migrating each artifact's fixtures and codecs in the same change — across 31 artifacts, with
`🔊️wav`, `🎵️mp3`, `🗜️deflate` and `💾️binary` having no leaf directories at all.

That is a vocabulary-and-wire-format migration, and it is the one commit `d394744295` set in motion. It
belongs to that work, not to this ticket, and starting it here would leave a shared tree mid-format-change.

**Recorded rather than begun.** This ticket's contribution stands: stdio 124 → 37, `🧿️semio` clean,
644/658 external-oracle coverage.

### The blocker's stated reason was wrong; the blocker is now one homogeneous class

Full write-up: `📓️stdio-blocker-reason-corrected-and-halved.md`.

I had recorded that the leaf migration was blocked because newtype variants **change the serde wire
format**. Measured instead of asserted: a newtype variant over a struct serialises **byte-identically**
to the struct variant, internally *and* adjacently tagged. No fixture, diff codec or text codec moves.
That blocker did not exist.

The real one, counted: the 60 failing aggregates hold **0–1 leaves against 5–15 variants each,
≈550 leaves to author**, and each leaf's `🔣️.json` encodes judgments — approved `verb`, `entity`,
past-tense `record`, `invertibility`, `diffParticipation`, `outcomeClasses` — that cannot be derived
from the code. `✳️drawing`, the migrated reference, also **redesigned its vocabulary** (`NoMutation`
and `SetSnapshot` deleted) under an SMO ruling. That is per-artifact design work belonging to
`d394744295`'s ticket.

**124 → 60 errors, 5 causes → 1**, by fixing what was genuinely broken:

* **docx rename completed** (17 artifact files + the plugin root's three namespaces) — removed its
  ~37 errors and unmasked the 60, exactly as predicted.
* **175 leaf descriptors were malformed** — `aggregateVariant` held the *payload type* name where the
  derive requires the *enum variant* name. Mapped by the derive's own `kebab(variant) == semanticKind`
  rule: 382/382 resolved, 0 unmapped. Cleared 8 aggregates.
* **A const-eval waste fixed** (the `/🧬️mutations/` marker was re-tested at every byte after being
  found) and `long_running_const_eval` allowed at the stdio crate root with its reasoning — newly
  *exposed*, not caused: the const previously panicked before exhausting its budget.

**The open question is answered by the kernel, and my earlier fix was the wrong one.**
`mutation_leaf_descriptor_kebab` returns `hyphen` — a hyphen is REQUIRED. So the six single-word kinds
were invalid by contract and my relaxation of the derive's copy only made the two halves disagree.
The derive now requires a hyphen again (verified: **no other single-word kind exists repo-wide**), and
the six were renamed to `rotate-node`, `scale-node`, `group-nodes`, `ungroup-node`, `flatten-node`,
`unflatten-node` — variants, leaf dirs, descriptors, glue modules, text/binary codecs, catalog vectors
and fixtures all moved with them. The verbs stay single words; `APPROVED_VERBS` gates the verb, not
the kind.

Two self-inflicted errors during that rename, both caught and repaired: a blanket `"<kind>"` replace
rewrote **data** (`transform.scale` → `"scale-node"` in 88 places, plus `verb:` and text opcodes), and
a path rewrite over `📦️glue.rs` hit `🔄rotate-object` in another subset. The second is worth
generalising — resolving **every** `#[path]` target in glue against the filesystem found both breaks
immediately.

Harness **119/119** (it caught the rename itself: `registry/no-malformed-contribution` failed until the
catalog's `mutationDirectoryName` vectors moved too). `oracleEvidenceCoverage` 574/658, 862 fixtures,
14 still un-oracled — coverage unchanged.

`mathematical` 9 + `sequence` 4 stay blocked: both crates depend on stdio and their TypeScript
packages are WASM facades (`export {}`) built from the same Rust, so there is no TS bypass.

### Evidence gap 84 → 18, none of it needing the blocked build

Full write-up: `📓️evidence-gap-closed-84-to-18.md`.

`oracleEvidenceCoverage` (a mutation has BOTH a qualifying oracle AND a fixture to run it against) was
574/658. It is now **596/614 (97.07%)**, fixtures 862 → **884**, harness **119/119**.

* **50 were my own defect.** Each `fem` catalog carried TWO `mutationManifests` for one subset,
  differing only in artifact id — the pre-existing `s.fem.2d` and an `s.fem.fem2d` I had added — so 50
  rows were counted twice and my fixtures, targeted at the id no manifest used, were unreachable.
  Merged onto the canonical id, **carrying both `oracleRequirements` forward** so the still-owed
  `fem2d-1-mutate` requirement was not deleted into a fake green. A repo-wide sweep confirmed these
  were the only two such pairs.
* **6 were invisible.** `✳️drawing`'s manifest listed 11 kinds for a 17-kind subset — the six renamed
  leaves were never registered, appearing in neither numerator nor denominator, which is exactly what
  `coverage/untested-appears-as-missing` forbids. Registering them correctly RAISED the gap first.
* **22 fixtures authored by third parties, one bound per mutation.** `✳️drawing` 17 via a standalone
  quick-xml engine (SVG is the one carrier that natively carries both `transform` and `<g>` nesting —
  precisely what those kinds edit); `🔣️json/✳️base` 5 via a standalone serde_json engine with
  `preserve_order` on, without which `set-member`/`remove-member` are not observable. Both are
  `[workspace]` crates with one dependency each, so they build while stdio does not.
* **The harness caught my manifests** — 118/119 until `units` and a string `comparisonProfile` were
  fixed in the GENERATOR rather than in its output.

**Recorded and deliberately not exploited:** `withoutEvidence` keys on the SUBSET, so one fixture
anywhere would have closed all 17 drawing kinds at once. The corpora bind one fixture per mutation
instead. `fem` shows the gap is real — `create-region`/`delete-region`/`replace-region` count as
evidenced on their siblings' fixtures. A per-mutation binding dimension is the next refinement; until
then this number means "the subset has been exercised", not "this mutation has been".

18 remain, all blocked: `mathematical` 9, `sequence` 8, `jpg::remove-huffman-table` 1.

### The last 14 are EXPORT defects, not oracle gaps — one now fixed

Full write-up: `📓️last-14-are-export-defects-not-oracle-gaps.md`.

Both dimensions now read **600/614 (97.72%)**, and their coinciding is the point: no mutation has an
oracle registered with nothing to run it against. Fixtures 862 → **888**. Harness **119/119**.

Splitting the gap BY DIMENSION is what exposed the cause. Four `sequence` kinds were missing evidence
but not an oracle — they already had `csv-rfc4180-reader` and only lacked fixtures; closed with authored
`(before.csv, after.csv)` pairs from a standalone `csv`-crate engine mirroring `SequenceIntoCsv`'s exact
row shape.

The other 13 are blocked one layer further back, on our EXPORT:

* **`SequenceIntoJson` declared `IoFidelity::Exact` and dropped the entire document.** It serialized the
  snapshot, but `SequenceSnapshot` is `{schema, content: ArtifactChild}` and `ArtifactChild` holds its
  scene in a `#[serde(skip)]` `local_owner` — so the "exact" export emitted `{schema, content:{childId,
  target}}`, no steps and no edges. The sibling csv serializer already went through `to_fixture()`.
  **Fixed** to do the same; `SequenceFixture` carries `edges`, `x`/`y` and `collapsed` — every field the
  four remaining kinds touch. **The oracle was deliberately NOT registered**: the fix cannot be executed
  while stdio does not build, and registering against an unrun export is the exact fiction the
  `mathematical` catalog refuses. Those four stay counted missing.
* **`MathematicalIntoJson` has the identical false `Exact`** over three `ArtifactChild` fields — but no
  `to_fixture()` exists, so defining that projection is a schema decision for the artifact, not a
  mechanical substitution. Reported, not guessed. Its nine stay blocked.
* **`jpg::remove-huffman-table`** unchanged: no writer varies the DHT list while staying decodable.

### `jpg::remove-huffman-table` closed, and the fixture it had proved nothing

Full write-up: `📓️jpg-remove-huffman-table-closed-and-a-worthless-fixture-found.md`.

**601/614 (97.88%)** on both dimensions; harness 119 → **121/121**.

My negative ("no writer varies the DHT list while staying decodable") was wrong the same way as the
three before it: it surveyed LIBRARIES (Pillow's empty Huffman accessors, zune-jpeg's private tables)
and never the installed CLI. `djpeg -verbose` prints `Define Huffman Table 0x00/0x10/0x01/0x11` — and
this subset had ALREADY registered `libjpeg-jpg-jfif-1-01-marker-cli` for its sibling kinds.

The writer is `cjpeg -scans`: a JPEG defines a Huffman table BECAUSE a scan references it, so a script
with one fewer AC scan yields one fewer table (4 → 3, AC chroma `0x11`), both halves decoding cleanly.
The generator asserts exactly one table removed, no new table, both halves decodable. **The honest
limit is in the manifest**: the pair does not isolate the table from the scan, because no conforming
writer can separate them.

**A defect found on the way.** The fixture already registered for this kind had `before.jpg` and
`after.jpg` with the IDENTICAL sha256 — the same file twice, declared `outcome: "applied"`. The oracle
could compare those halves forever and never see the mutation. A repo-wide sweep found 23 such
fixtures of 450 pairs; most are correct by construction (`no-mutation`, docx `-no-op-`), and four are
self-documented encoder-gap records whose kinds each carry a genuinely differing pair elsewhere. This
was the only case where the degenerate pair was a kind's ONLY evidence.

Two gates added: `fixture/applied-pair-must-differ` (not "no identical halves" — that would fire on
`no-mutation`, where a DIFFERING pair would be the bug) and its injection companion
`fixture/degenerate-only-pair-is-caught`.

### The mathematical projection EXISTS — my "schema decision" claim was wrong in its premise

Full write-up: `📓️mathematical-projection-exists-my-claim-was-wrong.md`.

I filed the mathematical export as needing a schema decision because *"`mathematical` has NO
`to_fixture()`"*. False. `MathematicalWorkingScene { graph, geometry }` (`🦀️component.rs:301`) IS the
projection, `mathematical_scene_owner` (`:336`, **sync**) is the accessor, and the two structs carry
every field the nine blocked kinds touch — `edges`, `directed`, `algorithm`, and `points`. They are
already serialized together in that same file (`mathematical_scene_id` does
`serde_json::to_string(&(graph, geometry))`). "No carrier can record them" was never a fact about the
data; it was a fact about which function the JSON serializer happened to call.

It is still not `sequence`'s one-token fix, for a reason I had not identified: `mathematical_scene_owner`
returns an **`Option`**, because it reads `ArtifactChild`'s `#[serde(skip)]` `local_owner` — populated
in-process, EMPTY after a round trip. An export built on it would emit a silently empty scene from
decoded bytes, which is worse than today's honest handle because it would look like data. That sourcing
question is the real decision. `sequence`'s `to_fixture()` is total, which is why that fix was safe.

**Second defect, found in passing:** `MathematicalIntoCsv` calls `mathematical_graph(from)` — an `async
fn` — without `.await` and then reads `.nodes` off the future. That cannot compile; it is masked by the
stdio failure upstream. Both `mathematical_graph` and `mathematical_scene` are `async` with fully
synchronous bodies. Source reading, not a compiler result — the crate cannot be built today.

The nine stay counted as missing. Nothing was registered against an unrun export.

### brep audited clause by clause against the goal statement

Full write-up: `📓️brep-audited-against-the-goal-statement.md`.

Checked the built corpus against the goal's own worked example literally, not against the coverage
number. Every clause is satisfied by files on disk: `brepjs-occt` (brepjs 18.119.8) is the named
third-party oracle for **13/13** kernel kinds; **155 STEP files** across 72 fixtures (90 operand files,
plus `expected.step`, `expected.mesh.json`, `expected.metrics.json`); "the same STEP file" is
`canonicalBytesEqual: true` via `step-external-canonicalizer`; "a similar mesh… similar hausdorf
distance" is `manifold-mesh-compare` with **`hausdorffInTessellationTolerancesMax: 3`** — expressed in
units of the fixture's own tessellation tolerance, which is what makes "different tesselation is
allowed" enforceable; "volume, etc" is symmetric-difference volume ≤1% plus relative volume 1e-8, area
1e-7, centroid 1e-8, bbox diagonal 1e-8, components and genus equal.

"Complicated boolean operations" is **18 fixtures** of deliberately hard cases: tangent spheres (point
contact), tangent cylinders (line contact), non-manifold corner-touching boxes, single/double nested
voids at two scales, coincident face stacks, fuse→cut→intersect chains, and a multistep case scaled to
1e4.

Seven fixtures carry no `expected.step`, and all seven should not: two empty boolean results and five
rejected edits, each DECLARING it (`"declaredOutcome": "empty"|"rejected"`, `"hasExpected": false`)
rather than merely lacking the file.

### The thirteen are ONE defect, and I overstated the sequence fix in the source

Full write-up: `📓️one-defect-behind-all-thirteen.md`.

I had filed the remainder as two problems. They are one, in `ArtifactChild`: the materialized scene
lives in a `#[serde(skip)] local_owner` (`🏪️store/🦀️component.rs:2567`), populated in-process and
ABSENT after a decode. `mathematical` surfaces that as an `Option` its exporter never used;
`sequence` swallows it — `local_owner().map(..).unwrap_or_default()` — into an **empty scene**.

**Correction to something I wrote into the repository.** Changing `SequenceIntoJson` to serialize
`from.to_fixture()`, I recorded that the hop is now `IoFidelity::Exact` "for real" and that
*"`sequence` has no such hazard: `to_fixture()` is total and infallible."* It is total by DEFAULTING TO
EMPTY, which is the worse of the two shapes — `mathematical` at least returns an `Option` a caller can
branch on, while `sequence` cannot tell "no steps" from "owner absent". A decoded snapshot exports as
`{schema, steps: [], edges: []}` while still claiming `Exact`. That file's docstring now says so; the
change remains an improvement (a live export carries steps and edges where before it carried neither),
but the declaration is conditional and the source now names the condition.

**Swept, not assumed:** 29 artifacts hold `#[child(...)]` handles, of 335 export serializers repo-wide.
Of the eight exporters across these two subsets only three reach the scene; the rest serialize the
snapshot and emit handles. **The corpora built in this ticket are unaffected** — `SemioDrawingSnapshot`
is inline `#[state(artifact)]` (only `✳️kit`/`✳️object` are child-backed under `🧿️semio`), so the 17
drawing, 5 json and 4 sequence-csv fixtures stand.

**Where it bites:** not on the 601. Both dimensions ask for a registered oracle and a fixture to run it
against, and every fixture here is authored third-party bytes that never pass through our exporters. It
bites on `runtimeMutationCoverage` (0.00%, 0/40 everywhere): when our implementation is finally exported
and compared, any child-backed artifact whose exporter serializes the snapshot will emit handles or an
empty scene and fail for reasons unrelated to the mutation. 13 blocked now, a latent subject-side
blocker across up to 29 artifacts later, one root cause.

### Everything remaining is ONE blocker, now fully sized

Full write-up: `📓️one-blocker-gates-everything-remaining.md`.

Three remainders were being reported separately — 13 un-oracled mutations, `runtimeMutationCoverage`
0.00%, ~550 unwritten leaves. They are one blocker with three symptoms.

**Subject-side 0% is verified, not assumed.** `RuntimeMutationInventory` is JSON the BUILT
implementation emits (`id`/`variant`/`verb`/`entity`/`record`/`outcomes` per subset); the dimension
compares it against the manifest. No such file exists anywhere, because nothing can emit one: of 64
plugin crates, **35 depend on `semio-s-plugin-stdio` — every artifact-owning one** — and the 29 that do
not are all extensions (`flow-extension-*`, `process-*`, `cad-aec-*`, `imperative-*`, `sourcing-*`,
`draw-fsm`, `playbook-procedural`) owning no manifest. The same crate gates `mathematical` and
`sequence`, which is the other 13.

**Why the leaf migration is a vocabulary change — one grep.** The derive asserts
`is_approved_verb(SEMANTICS.verb)` per variant. `set` ✅ and `replace` ✅ are approved;
**`set-snapshot` ❌ and `no-mutation` ❌ are not** — and **all 60** failing aggregates carry
`NoMutation`. So those two variants cannot become leaves; migrating an aggregate means DELETING them,
which is precisely what `✳️drawing` did under its SMO ruling. The wire format of a migrated variant is
unchanged (measured, holds) — but two variants stop existing.

**Sized:** 60 aggregates, ~550 leaves, 60/60 carrying `NoMutation`, **790 `::NoMutation` + 574
`::SetSnapshot` = ~1364 call sites** repo-wide.

Nothing in this ticket's scope is blocked by it — both dimensions read 601/614 and the corpora,
registrations and 121 checks stand independently, because every fixture is authored third-party bytes
that never pass through our exporters. What is blocked is the end-to-end half.

### The leaf migration is smaller than I said — three corrections, all the same direction

Full write-up: `📓️the-migration-is-smaller-than-i-said-three-times.md`.

I overstated this migration's cost three times, each time by asserting a constraint instead of
measuring it:

1. **"The wire format changes."** False — measured: a newtype variant over a struct serialises
   byte-identically to a struct variant, internally and adjacently tagged.
2. **"`SetSnapshot` cannot be a leaf, so all 60 aggregates must delete it."** False, and this was the
   expensive one. I conflated two descriptor fields: the derive asserts `kind == kebab(variant)` AND
   `is_approved_verb(SEMANTICS.verb)` — `verb` is SEPARATE, and only it is checked against the table.
   `✳️drawing`'s own leaf proves it: `{ verb: "rotate", entity: "node", kind: "rotate-node" }`. So
   `SetSnapshot` migrates intact as `{ verb: "set", entity: "snapshot", kind: "set-snapshot" }` — no
   semantics lost, no wire change, and its **574 call sites are a syntax change only**.
3. **"Each artifact forces a semantics decision."** Overstated. With `SetSnapshot` surviving, only
   `NoMutation` genuinely cannot migrate — it is a UNIT variant where the derive requires exactly one
   payload, and none of the 41 approved verbs means "do nothing". That is ONE decision per artifact
   with two honest answers: delete it (drawing's choice — the no-op `inverse()` arms become `vec![]`,
   semantically identical) or rename it to an approved-verb kind. Either way `#[derive(Default)]` goes.

**Corrected size:** variants forced to change semantics drops from `NoMutation`+`SetSnapshot` to
**`NoMutation` alone**; call sites losing behaviour from ~1364 to **790**, and to zero if renamed
rather than deleted.

`📐️step/✳️cc1` is now a tractable worked example — four leaves (`set-snapshot`, `set-file-schema`,
`set-product-identity` all verb `set`; `remove-shape-representation` verb `remove`) and one
`NoMutation` decision. Its documented inversion strategy SURVIVES: the header explains
`remove-shape-representation` degrades to `SetSnapshot`, and under correction 2 that escape hatch is
still available. My earlier reading — that migrating cc1 would strip its undo — was a consequence of
the mistake, not of the migration.

### `step@ap214/✳️cc1` migrated — the blocker is 60 → 59 and the pattern is proven

Full write-up: `📓️cc1-migrated-the-pattern-is-proven.md`.

Stopped analysing the migration and did one. **`cargo build -p semio-s-plugin-stdio` → 59 `E0046`
(was 60), `✳️cc1` produces zero diagnostics, harness 121/121.**

Four leaves authored (`📋set-snapshot`, `🏷set-file-schema`, `🪪set-product-identity`,
`🗑remove-shape-representation`), the aggregate converted to newtype variants under
`#[derive(dsl::Mutations)]`, the hand-written `impl Mutation` deleted, `class_diff`/`class_inverse`
kept `pub(crate)` in the aggregate so the four leaves share one implementation, and the aggregate's
tests, the external harness and the catalog's `kinds` updated.

**Both corrections confirmed in practice.** `SetSnapshot` migrated intact as
`{ verb: "set", entity: "snapshot", kind: "set-snapshot" }` — so CC1's documented escape hatch (undoing
a representation removal by restoring the projection) survives, and my earlier reading that migration
would strip cc1's undo was a consequence of the mistake, not of the work. `NoMutation` was the only
casualty and cost nothing: its one role was the "nothing to undo" arm, now the empty vector.

**No `📦️glue.rs` change was needed** — `mutations` is itself `#[path]`-declared by its schema
component, so `#[path = "📋set-snapshot/🦀️.rs"] pub mod set_snapshot;` inside the aggregate resolves.
That is the common shape; `✳️drawing`'s glue wiring is the alternative.

59 aggregates remain, and the per-artifact cost is now measured rather than estimated: leaves at
~40 lines each, one `NoMutation` decision, and call-site syntax.

### The whole `step@ap214` family migrated — cc1…cc6

Full write-up: `📓️step-ap214-family-migrated.md`.

`✳️cc1` hand-migrated to prove the pattern (**confirmed 60 → 59 `E0046`, zero cc1 diagnostics**), then
`✳️cc2` by a migrator built from it (**confirmed 58**), then cc3…cc6 by the same script. **28 leaf
directories**, each with payload + `MutationKind` impl and descriptor.

All 28 descriptors validated against the derive's own rules BEFORE compiling — owner path, dirname,
hyphenated kind, variant present in the enum, `kind == kebab(variant)`, verb in `APPROVED_VERBS`:
**28 valid, 0 failing.**

**Correction 2 paid off again.** `DemoteShapeRepresentation` looked unmigratable (`demote` is not an
approved verb) and migrated unchanged as `{ verb: "change", kind: "demote-shape-representation" }` —
kind and verb are different fields and only the verb is checked. Under the earlier wrong reading this
variant would have been deleted from four subsets.

The migrator rewrites `Agg::Variant { .. }` → `Agg::Variant(module::Type { .. })` with brace matching,
the SAME transformation for constructions and patterns since Rust spells them identically — which is
why no `diff`/`inverse` body had to be rewritten. Three things it did not cover, each found by sweeping
for stale `NoMutation` references rather than trusting its output: the external harnesses are separate
crates needing fully-qualified leaf paths, `✳️cc6` has a `🏭️bridge/🦀️component.rs` none of its siblings
has, and `✳️cc1`'s declaration-gate test was only partly rewritten by the earlier hand edit. Stale
references now **0**.

**Peer activity, attributed not chased.** Mid-run the build showed 55 `E0433` that vanished next build,
plus 6 `E0425` naming helpers in `✳️drawing/…/🧷group-nodes/🔺️diff/🦀️.rs` and `↩️inverse/🦀️.rs`. Those
paths are not mine and `🔺️diff/🦀️.rs` was created at 22:42, after my last edit to that leaf at 22:39 —
another session is splitting the drawing leaves into per-facet files. Settled by mtime and authorship.

Harness **121/121**; coverage unchanged at 601/614 until the crate builds.

**Confirmed:** `cargo build -p semio-s-plugin-stdio` → **54 `E0046`** (was 60), **0 errors naming `📐️step`**.
The 6 `E0425` + 1 `E0599` that remain are the peer's in-flight `✳️drawing` per-facet split.

### The brep oracle EXECUTED — 72/72 reproduced byte-for-byte

Full write-up: `📓️brep-oracle-executed-not-just-registered.md`.

Everything said about brep here had been established by READING the catalog. So the generator was run:
`bun ✳️brep/🏭️generator/📜️script.ts generate --out <scratch>`. OpenCASCADE's own STEP writer banners
appeared in the console, and the generator reported **72/72 fixtures byte-identical across two
generation passes**.

Compared against the repository: **292 files, 292 byte-identical, 0 differing, 0 missing, 155 STEP
files regenerated.** The committed expectations for all 13 brep kinds are exactly what brepjs produces
today, twice in a row.

This is the load-bearing check `reproducible: true` merely declares — and it matters specifically
because OCCT's STEP export is not byte-deterministic across configurations, which is why it was worth
executing rather than assuming.

The mesh half computes the goal's Hausdorff requirement through **`three-mesh-bvh`** closest-point
queries (`symmetricHausdorff`, normalized against the bounding-box diagonal), consumed by the brep
pipeline's `manifold-mesh-compare` stage at `hausdorffInTessellationTolerancesMax: 3`. That probe's own
header states the constraint this ticket enforces: *"Everything here MARSHALS and INVOKES; nothing here
computes geometry […] which is what keeps the reference external."*

### Generic migration: built, dry-run over all 54, attempted, stopped with cause

Full write-up: `📓️generic-migration-attempted-and-stopped-with-cause.md`.

A generic migrator was written for the remaining 54 — it lifts each aggregate's `diff`/`inverse` bodies
VERBATIM into free functions and has every leaf delegate back via `Agg::Variant(self.clone())`, so
semantics are preserved by construction rather than re-derived. **Dry run: 43 clean, 9 skipped with
reasons** (3 no hand impl, 1 already newtype, 5 needing a verb decision — `stamp`, `strip`, `splice`,
`truncate`, `declare`, `upsert` are not approved verbs).

Two bugs, both caught before compiling: `POOL="🔧🔩…".split()` yields ONE element, so 14 las leaves were
created under directories prefixed with the whole pool (fixed to a list, plus a guard refusing any
dirname that is not `<one-emoji><kind>`); and `set-snapshot` already exists as a leaf in most
aggregates, so the migrator authored a DUPLICATE. las was restored exactly from the git index
(`git show :<path>` — a read, not a checkout) and all 14 generated dirs removed.

**Stopped because of a timestamp, not caution.** Those existing `set-snapshot` leaves carry `🔺️diff/`,
`↩️inverse/` and `🧪️tests/` subdirectories — a PER-FACET layout, not the single-file shape this
migrator emits — and they are moving live: `📄set-snapshot/🦀️.rs`, `🔺️diff/🦀️.rs` and `↩️inverse/🦀️.rs`
all written at **23:16:16**, the same second, by an automated split. The same signature is in
`✳️drawing` and is the source of the 6 `E0425` + 1 `E0599` in the build. Running 43 mechanical
migrations into that would duplicate their leaves, emit a second incompatible shape, and race them file
by file. The step family was safe precisely because `📐️step/🔖️ap214` is not where they are working.

The migrator is preserved. It needs one change (reuse an existing leaf dir per kind) and one
coordination decision (single-file vs per-facet leaves) before it can run at scale.

**Correction to the above.** I wrote that the existing `set-snapshot` leaves "are not stubs". Reading
one settles it: `☁️las/…/📄set-snapshot/🦀️.rs` is, in full, three imports and
`pub fn apply(..) { apply_las_mutation(..) }`. No payload struct, no `dsl::MutationLeaf`, no
`MutationKind` — an APPLY-HELPER directory named after a kind, not a leaf. `dsl::Mutations` would
reject it, and my duplicate `🔧set-snapshot` would have given two directories the same `semanticKind`.

That makes the collision exact rather than probable: migrating any of these aggregates requires that
same `📄set-snapshot/🦀️.rs` to BECOME a leaf, and it is the file the other session's sweep is rewriting
— 19 lines moved out of las's copy into `🔺️diff/` and `↩️inverse/` at 23:16:16. 113 such files exist,
101 in `🧿️semio` where their newest edits are. `📐️step/🔖️ap214` was migratable for a precise reason:
those six subsets have no `📄set-snapshot` helper directory at all, so nothing of theirs was touched.

### The Hausdorff/volume comparator EXECUTED — and shown to discriminate

Full write-up: `📓️hausdorff-and-volume-comparison-executed.md`.

Ran `✳️mesh/🔬️probes/📜️script.ts mesh-compare` (three@0.182.0 + manifold-3d@3.5.1 +
three-mesh-bvh@0.9.14) on real fixtures:

* **Same solid, two carriers** (`expected.stl` vs `expected.obj`) → `symmetricHausdorff: 0`,
  `relativeVolumeError: 0`, `hausdorffSamples: 72`, status ok in 22 ms.
* **Two different solids** → `symmetricHausdorff: 7.80`, `normalizedSymmetricHausdorff: 1.56`,
  `symmetricDifferenceVolume: 1.20`, `relativeVolumeError: 239.0` — failing the brep pipeline's
  `hausdorffInTessellationTolerancesMax: 3` and `normalizedSymmetricDifferenceVolumeMax: 0.01` by
  orders of magnitude, which is what a working gate must do.

A comparator that always returns zero proves nothing, so the second run is the load-bearing one. Worth
noting the asymmetry it exposed — expected→actual 0.08 versus actual→expected 7.80 — which is why the
probe reports both directions and takes the max; a one-directional distance would have called those two
shapes nearly identical.

This settles the MEASUREMENT half of the goal's brep requirement: implemented, executing, real numbers
from third-party engines, discriminating. It does not settle the subject side — both inputs are fixture
meshes, and feeding our kernel's export in as one side needs the 54-aggregate build.

### Eleven more aggregates — the blocker is 60 → 43

Full write-up appended to `📓️step-ap214-family-migrated.md`.

A generic migrator (lift `diff`/`inverse` verbatim into free functions; each leaf reconstructs its
aggregate value and delegates back) was run on eleven more aggregates, **selected by the criterion that
made step safe** — only those with no `📄set-snapshot` helper directory for the other session's sweep to
contend over. `📷️jpg@baseline`, `🖼️tiff@baseline`, `🏗️ifc@cv20/sav/cobie`, and the
`📜️docx`/`📕️xlsx`/`🎞️pptx` strict+transitional pairs. **77 descriptors, validated before compiling,
0 failures.**

It cost ~215 errors across four rounds, and every one was a gap in what "mechanical" reaches — none of
which the step family contained: `Self::` surviving the lift out of an `impl` (181 × `E0433`); unit
variants becoming newtype over an EMPTY payload, needing `(_)` in patterns but `(mod::Ty {})` in values
(16 × `E0532`, then 9 more); construction sites in `✏️editor/` rather than `🧪️tests`; and leaf modules
needing full paths outside their own aggregate. All four are now written into the migrator with the run
order that works.

**Confirmed: 43 `E0046`** (was 60), 0 errors naming the 11 migrated subsets. The remaining 7 non-E0046
errors are all in `✳️drawing` — the other session's work. Harness 121/121.
