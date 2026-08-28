# 📓️ `semio@v1/drawing` — carrier verification, per-mutation witnessability, oracle registration

Subset: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing`
Capability: `semio-v1-drawing-mutate` · artifact `s.stdio.semio` · standard `v1` · subset `drawing` · 17 kinds.

Every verdict below was reached by reading the serializer BODY and the mutation diff BODY, never a
directory listing and never a doc comment.

---

## 1. Carriers — all four confirmed REAL, with the svg false-stub corrected

`🚪️io/📤️export/🧵️serializers/🗿️artifacts/<fmt>/**/🦀️component.rs`, read in full.

### 1.1 `svg` — REAL. The earlier STUB flag was a false positive, confirmed here independently.

`…/🎨️svg/🔖️1.1/✳️any/🦀️component.rs` is 228 lines. The ONLY occurrences of `print_dsl`/`parse_dsl`
in the file are at **lines 196–197**, and both are inside `#[cfg(test)] mod tests { … }`, which opens
at **line 164** and closes at line 227:

```rust
163  //#region 🔖️Tests
164  #[cfg(test)]
165  mod tests {
…
196          let text = <SvgSnapshot as store::ArtifactDsl>::print_dsl(&svg);
197          let reparsed = <SvgSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("reparse real svg text");
```

That is the file PROVING its own round trip, not faking an export. The actual export path is
**lines 138–159** and it is a genuine SVG document build:

```rust
138      async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
139          let layer_groups: Vec<SvgElement> = from.layers.iter()
142              .map(|layer| SvgElement::Group {
143                  common: CommonAttrs { id: Some(format!("layer-{}", layer.id)), ..Default::default() },
…
150          let root = SvgElement::Svg {
152              view_box: Some(ViewBox { min_x: 0.0, min_y: 0.0, width: from.canvas.width, height: from.canvas.height }),
155              xmlns: Some("http://www.w3.org/2000/svg".into()),
```

with real per-node lowering at **lines 103–126** (`<path>` from `segments_to_commands`, `<text>` with
a child text node, `<g transform="matrix(...)">` from `semio_transform_to_matrix` at lines 43–47), and
a real base64 data-URI convention for `Image` at **lines 114–124** backed by a hand-rolled encoder at
lines 87–100.

Second hop to real bytes: `SvgSnapshot::export_utf8` →
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:62`
`Ok(write_svg_xml(&self.doc).into_bytes())` → `xml_document_to_text`. Real SVG 1.1 XML on disk.

⚠️ NOTE for anyone else touching this: `SvgSnapshot::print_dsl` (same file, line 1334) does NOT emit
XML — it emits a bracketed structural encoding inside a semio envelope. The XML bytes come from
`export_utf8`/`write_svg_xml`. A probe must be handed `export_utf8` output, not `print_dsl` output.

### 1.2 `dxf` — REAL.

`…/🖊️dxf/🔖️r12/✳️any/🦀️component.rs`, 260 lines. Exact-circle recognition at **lines 31–40**
(`as_circle` matches `[MoveTo, ArcTo, ArcTo, Close]` for an EXACT `CIRCLE` round trip), real
32-sample curve flattening at **lines 47–117**, entity build at **lines 138–154**, serializer at
**lines 182–197**:

```rust
141              if let Some((cx, cy, r)) = as_circle(segments) {
142                  return Some(DxfEntity::Circle { center: [cx, cy, 0.0], radius: r, layer: layer.into(), … });
…
149              Some(DxfEntity::Polyline { vertices, closed, layer: layer.into(), … })
…
186              layer_defs.push(DxfLayer { name: layer.id.clone(), color: 7, linetype: "CONTINUOUS".into(),
187                                         flags: if layer.visible { 0 } else { 1 }, … });
```

**Load-bearing limits found by reading, not assumed:**
* `dxf_entity_from_node` returns `None` for `DrawNode::Group` (**line 152**) and `collect_entities`
  (**lines 157–170**) recurses THROUGH a group without applying its transform. **A group transform is
  never applied to child coordinates in DXF.**
* Per-node `style` is bound but unused in `dxf_entity_from_node` (**line 140** `DrawNode::Path { segments, .. }`).
  `DxfLayer.color` is the hard-coded literal `7`. **DXF carries no fill, stroke or stroke-width at all.**
* The DXF layer name is `layer.id`, not `layer.name`.

### 1.3 `pdf` — REAL, but text-only.

`…/📄️pdf/🔖️1.7/✳️any/🦀️component.rs`, 105 lines. One `PdfPage` per `DrawLayer`, `media_box` from the
canvas, page text = every `DrawNode::Text.value` in that layer's tree joined by `\n`
(**lines 17–32, 43–57**). `Path`/`Group`/`Image` are explicitly dropped (**line 30**). Its own test
proves real bytes through `encode_pdf`/`decode_pdf` (lines 99–102). **PDF carries no geometry, no
position, no colour — only text content, text order, and page count.**

### 1.4 `dwg` — REAL, but NOT an oracle leg.

`…/🖊️dwg/🔖️ac1024/✳️any/🦀️component.rs`, 144 lines. Real walk into a `DwgDrawing` via
`paths_to_dwg_drawing` (**lines 46–61, 73–95**), round-tripped through the codec in its own test
(lines 127–142). Its own module doc states the same limit the code shows (**lines 13–15**):
`Group` transforms are NOT applied to child geometry.

**DWG is Autodesk-proprietary and has NO approved third-party reader in this repository's roster**
(`🔒️dependencies.json` has no DWG crate; the standalone `dwg@ac1018`/`ac1024` subsets already decline
for this reason). DWG is therefore recorded as **write-only evidence, never an oracle leg** — matching
the existing, already-correct declines.

---

## 2. Outcome classes — read from `MutationOutcome::{new,empty,error,fatal}` in each `🔺️diff/🦀️component.rs`

Mapping: `new`→`applied`, `empty`→`no-op`, `error`/`fatal`→`rejected`. Doc comments were not consulted.

| kind | constructors found | outcome classes |
| --- | --- | --- |
| `create-layer` | `fatal`(14), `new`(17) | applied, rejected |
| `delete-layer` | `new`(14), `error`(15) | applied, rejected |
| `create-node` | `new`(16), `fatal`(21) | applied, rejected |
| `delete-node` | `error`(25), `new`(29), `error`(31) | applied, rejected |
| `move-node` | `error`(16), `fatal`(19), `empty`(22), `new`(24) | applied, no-op, rejected |
| `drag-nodes` | `fatal`(19), `empty`(22), `error`(34), `new`(36) | applied, no-op, rejected |
| `rotate` | `error`(16), `fatal`(20), `empty`(24), `new`(27) | applied, no-op, rejected |
| `scale` | `error`(15), `fatal`(19), `empty`(23), `new`(26) | applied, no-op, rejected |
| `reorder-nodes` | `empty`(16), `new`(19), `error`(24) | applied, no-op, rejected |
| `group` | `error`(25), `new`(32), `error`(37) | applied, rejected |
| `ungroup` | `error`(14), `new`(19), `error`(21) | applied, rejected |
| `flatten` | `error`(36), `new`(40), `empty`(41,42,44) | applied, no-op, rejected |
| `unflatten` | `error`(13), `empty`(16), `new`(18) | applied, no-op, rejected |
| `replace-path` | `empty`(13), `new`(14), `error`(15) | applied, no-op, rejected |
| `replace-fill` | `error`(15), `fatal`(19), `empty`(23), `new`(25) | applied, no-op, rejected |
| `change-stroke-color` | `error`(15), `fatal`(19), `empty`(23), `new`(25) | applied, no-op, rejected |
| `change-stroke-width` | `error`(15), `fatal`(19), `empty`(23), `new`(25) | applied, no-op, rejected |

11 kinds reach `applied|no-op|rejected`; 6 reach `applied|rejected` only.

---

## 3. Per-mutation witnessability — derived from the committed before/after snapshots

Each of the 17 kinds has exactly one committed scenario under
`🧬️schema/🧬️mutations/<kind>/🧪️tests/<scenario>/📸️snapshot/{⬅️before,➡️after}`. I applied each
carrier's OWN documented lowering (§1) to both snapshots and asked whether the carrier bytes differ.

| kind | svg | dxf | pdf | dwg (non-oracle) | why |
| --- | :-: | :-: | :-: | :-: | --- |
| `create-layer` | ✅ | ✅ | ✅ | ✅ | new `<g id="layer-l2">`; new `DxfLayer{name:"l2",flags:1}`; page count 1→2 |
| `delete-layer` | ✅ | ✅ | ✅ | ✅ | layer group / layer-table row / page removed with all its content |
| `create-node` | ✅ | ✅ | ✅ | ✅ | `<text x=2 y=20>Caption</text>`; new `TEXT` entity; page text gains `\nCaption` |
| `delete-node` | ✅ | ✅ | ✅ | ✅ | `Hello` text element / `TEXT` entity / page text line removed |
| `move-node` | ✅ | ✅ | ❌ | ✅ | text `(5,5)→(12,8)`: `<text x/y>` and `TEXT` position move; PDF drops position |
| `drag-nodes` | ✅ | ⚠️ | ❌ | ⚠️ | text moves (both see it) BUT the sibling group's `translation (0,0)→(2,-1)` is invisible to dxf/dwg |
| `rotate` | ✅ | ❌ | ❌ | ❌ | changes ONLY a nested group's rotation quaternion; dxf/dwg never apply a group transform |
| `scale` | ✅ | ❌ | ❌ | ❌ | changes ONLY a nested group's scale `(1,1)→(2,0.5)`; same reason |
| `reorder-nodes` | ✅ | ✅ | ❌ | ✅ | child order changes → DXF entity SEQUENCE changes; PDF text list is unchanged (`["Hello"]` both sides) |
| `group` | ✅ | ❌ | ❌ | ❌ | wraps two children in an identity `<g>`; DXF/DWG flatten by walk order → **byte-identical** |
| `ungroup` | ✅ | ❌ | ❌ | ❌ | dissolves a group; DFS leaf order unchanged → **byte-identical** |
| `flatten` | ✅ | ❌ | ❌ | ❌ | removes one identity group level; leaf order unchanged → **byte-identical** |
| `unflatten` | ✅ | ❌ | ❌ | ❌ | re-adds one group level; leaf order unchanged → **byte-identical** |
| `replace-path` | ✅ | ✅ | ❌ | ✅ | `M0,0 L10,0 Z` → `M0,0 L4,0 L0,4 Z`: `d` changes; POLYLINE gains a vertex |
| `replace-fill` | ✅ | ❌ | ❌ | ❌ | style `fill` red→blue; **no colour field exists in dxf/pdf/dwg** |
| `change-stroke-color` | ✅ | ❌ | ❌ | ❌ | style `stroke` → `rgba(255,255,255,0.5)`; same reason |
| `change-stroke-width` | ✅ | ❌ | ❌ | ❌ | style `strokeWidth` 1.0→2.5; same reason |

**Totals: svg 17/17 · dxf 8/17 (one of them partial) · pdf 4/17 · dwg 8/17 (non-oracle).**
**Uncarried (witnessed by no carrier at all): 0 of 17.** SVG is the universal carrier for this subset.

### 3.1 Two corrections to the prior research pass's estimate

The prior pass estimated `rotate`, `scale`, `group`, `flatten`, `unflatten` and `ungroup` as
"witnessable via dxf/dwg". **They are not.** Reading `collect_entities`
(`…🖊️dxf/…/🦀️component.rs:157–170`) and `collect_node` (`…🖊️dwg/…/🦀️component.rs:46–61`) shows both
walk THROUGH a `Group` and emit its leaves in DFS order with no matrix applied and no grouping
recorded. For all six of those kinds the before-DXF and after-DXF byte streams are identical, so a
green dxf comparison would prove nothing. Only SVG, which writes `<g transform="matrix(...)">` per
group, can witness them.

`drag-nodes` is a **partial** dxf witness and is marked ⚠️ rather than ✅: dxf sees the text node move
and cannot see the group-transform half of the same mutation.

---

## 4. Oracles — availability finding, and what that means

Approved `test-oracle` readers for these three carriers, from `🔒️dependencies.json` (no new package
proposed anywhere in this document):

| carrier | approved reader | ecosystem | precedent already registered |
| --- | --- | --- | --- |
| svg | `quick-xml 0.42` | rust | `quick-xml-svg-1-1-mutate`, `quick-xml-svg-1-1-tiny-mutate` |
| dxf | `dxf 0.6` (IxMilia) | rust | `dxf-crate-r12-mutate` |
| pdf | `lopdf 0.44` | rust | `lopdf-pdf-1-7-mutate` + 9 siblings |
| dwg | — none — | — | declined, as for `dwg@ac1018`/`ac1024` |

Three genuinely different parser families (`quick-xml`, `ixmilia-dxf`, `lopdf`), none of them ours,
none of them production-reachable from this repository.

**The JS parsers vendored in `node_modules` are NOT an alternative.** `fast-xml-parser`, `sax`,
`saxes`, `@xmldom/xmldom`, `xml2js` and `pdfjs-dist` are all present, but none of them is a
`test-oracle` entry in `🔒️dependencies.json`, and `pdfjs-dist` is production-reachable via
`react-pdf`. Registering one would either propose a new approved package or reproduce the
`oracle-in-production` breach that `three` already carries. The Rust roster above is the only honest
option for this subset.

**No expected-value computation.** Consistent with the new `reimplementation-registered-as-third-party`
gate, these three are registered as READERS only. None of them can compute what a `move-node` or a
`change-stroke-color` should produce — that semantics is this repository's. Their role is to re-derive
the drawing's structure from the exported bytes so the pipeline can compare; nothing in this
registration asks a third-party library to predict a mutation result, and no fixture in this subset
claims a third-party-computed expected state.

---

## 5. What was registered

`✳️drawing/🧪️oracle/🔣️.json` now carries, alongside the untouched `semio-drawing-python-independent`
supplement:

* **3 `third-party-library` oracles** — `quick-xml-drawing-svg-reader`, `ixmilia-dxf-drawing-reader`,
  `lopdf-drawing-pdf-reader`. Three different engine families, all `testOnly`, all
  `productionReachable: false`.
* **7 `external-process` probes** — `drawing-svg-structure`, `drawing-svg-compare`,
  `drawing-style-compare`, `drawing-dxf-entities`, `drawing-dxf-compare`, `drawing-pdf-text`,
  `drawing-pdf-compare`, each with a recorded `qualification`.
* **1 comparison profile** `semantic-drawing-carrier-v1` delegating to a 5-stage pipeline of the same
  name, and **1 tolerance profile** `drawing-exact`.
* **1 mutation manifest** owning all 17 kinds, with outcome classes taken from §2 and one
  `oracleRequirement` per oracle that genuinely witnesses that kind (§3).

### 5.1 The tolerance decision, and why it is not a tessellation tolerance

`drawing-exact` gates near-exact (`absoluteLength 1e-9`, `relativeLength 1e-12`,
`maxOverrideFactor 1`). The BRep pilot's reasoning does not transfer: a solid may legitimately be
tessellated many ways, but there is no legitimate re-encoding of an SVG path — the `d` attribute IS
the curve and this subset lowers one `PathSegment` to one command
(`…🎨️svg/🔖️1.1/✳️any/🦀️component.rs:27–38`). The only producer freedom a 2D vector carrier has is
number FORMATTING and attribute ORDER, and the comparison already normalizes both by comparing parsed
values instead of bytes. Widening beyond that would admit a real geometry error.

### 5.2 Implementation

`✳️drawing/🔬️probes/📜️script.ts` marshals and invokes; it reads no carrier and computes no number.
The readers live in `✳️drawing/🔬️probes/🦀️oracle-probe/` (`Cargo.toml` + `🦀️component.rs`), a crate
declaring its own `[workspace]` that links **no repository crate at all** — only `quick-xml`, `dxf`
and `lopdf`. That isolation is why it builds and runs while `semio-s-plugin-stdio` is mid-refactor,
and it is also what guarantees the reader shares no code with the implementation it adjudicates.

**No expected-value computation anywhere.** Consistent with the `reimplementation-registered-as-third-party`
gate, all three are registered as READERS. None is asked what a `move-node` or a
`change-stroke-color` should produce — that semantics is this repository's. They parse two carriers
and report what each contains; the pipeline compares. Any probe handed a carrier that cannot encode
the property under test returns `unsupported`, never an empty `ok`.

---

## 6. Gate validation — real numbers, both ways

Inputs built BY the third-party writers themselves (`gate-inputs`, committed at
`🧪️drawing-gate-inputs/`). The ACCEPT pair is the same drawing written twice with deliberately
different attribute order and number formatting — byte-different, semantically identical, so a
comparison that merely diffed bytes would fail it:

```
good-a.svg: <path d="M 0 0 L 10 0 Z" stroke="rgba(0,0,0,1)" stroke-width="1" fill="rgba(255,0,0,1)"/><text x="5" y="5">Hello</text>
good-b.svg: <path stroke="rgba(0,0,0,1.0)" stroke-width="1.0" d="M 0.0 0.0 L 10.00 0.0 Z" fill="rgba(255,0,0,1)"/><text x="5.0" y="5.000">Hello</text>
```

| case | verdict | measured |
| --- | --- | --- |
| SVG accept (`good-a` vs `good-b`) | **`equal: true`** | maxPathPointDeviation `0`, maxColorChannelDeviation `0`, maxStrokeWidthDeviation `0`, maxTextOriginDeviation `0`, structureEqual `true` |
| SVG reject — geometry (`10` → `10.05`) | **`equal: false`** | **maxPathPointDeviation `0.05000000000000071`**, `differingElements: [{path:"/0/0/0", tag:"path", reasons:["d differs by 0.05…"]}]` |
| SVG reject — paint (stroke red channel `0` → `5`) | **`equal: false`** | **maxColorChannelDeviation `5`**, `reasons:["stroke channel 0 differs by 5"]` |
| DXF accept (`good-a` vs `good-b`) | **`equal: true`** | maxVertexDeviation `0`, layerNamesEqual `true`, entitySequenceEqual `true`, textValuesEqual `true` |
| DXF reject — geometry | **`equal: false`** | **maxVertexDeviation `0.05000000000000071`** on entity 0 (`POLYLINE`) |
| DXF reject — text (`Hello` → `Hallo`) | **`equal: false`** | **textValuesEqual `false`** on entity 1 (`TEXT`) |

The accept case and the reject case are separated by the full width of the measurement: `0.000e+00`
against `5.000e-02` for geometry and `0.000e+00` against `5.000e+00` for paint. A gate only ever
tested on good input is not a gate; this one was tested both ways.

### 6.1 `unsupported`, proved rather than asserted

```
$ … dxf-compare   --input good-a.svg --input good-b.svg
{"status":"unsupported","measurements":{"reason":".svg is not a DXF carrier"}}
$ … style-compare --input good-a.dxf --input good-b.dxf
{"status":"unsupported","measurements":{"reason":".dxf does not encode SVG structure, paint or stroke width"}}
$ … pdf-text      --input good-a.svg
{"status":"unsupported","measurements":{"reason":".svg is not a PDF carrier"}}
```

The middle one is the load-bearing case: asking DXF about a stroke colour is refused, not answered
with an empty paint set that a pipeline would read as agreement.

---

## 7. Harness results — real output

### `contract --subset drawing`

Repo-wide totals moved 1280 → 1410 high-priority breaches, but **that rise is not this subset's**: 130
of the 131 new breaches are the newly-landed `reimplementation-registered-as-third-party` gate firing
on `png`, `bmp`, `tiff`, `obj`, `avi` and the `dxf` artifact (38 of them are literally
`… is registered as a qualifying third-party oracle, but this owner predicts mutation output in its own
Rust`). Diffing the two breach sets, this subset's own delta is exactly one resolved and one new:

```
RESOLVED:  testing/contract  …/✳️drawing/🧪️oracle/🔣️.json
           Catalog semio-v1-drawing declares capability semio-v1-drawing-mutate (17 kind(s)) and no mutation manifest owns it
NEW:       testing/contract  …/✳️drawing/🧪️oracle/🔣️.json
           No runtime inventory has been produced for s.stdio.semio@v1/drawing
```

No new `testing/oracle` or `testing/fixture` breach names this subset. The new one is honest and is
left standing deliberately: a runtime inventory must come from a `🏭️bridge` that asks PRODUCTION
dispatch, and `semio-s-plugin-stdio` does not currently compile. Hand-writing the inventory would make
the equality gate compare the manifest with itself, which is the one thing the runtime half exists to
prevent. `s.stdio.semio@v1/drawing` now sits in the same "no runtime inventory" queue as `mesh`,
`brep`, `step@ap214/cc6`, `note` and eleven others.

### `matrix --subset drawing`

```
[matrix] externalOracleCoverage            42.54%  151/355
[matrix] fixtureProvenanceCoverage        100.00%  285/285
[matrix] fixtureReproducibilityCoverage   100.00%  285/285
[matrix] dependencyIsolationCoverage      100.00%  174/174
[matrix] runtimeMutationCoverage            0.00%  0/17
```

`[report] Which mutations have no external oracle?` lists `sequence`, `fem2d`, `fem3d`, `jpg`, `png`,
`bmp`, `tiff` and `gltf` kinds — **and no `s.stdio.semio` drawing kind at all. All 17 are discharged.**
`[report] Which oracle and subject share an underlying engine family?` → `none`.
`semio-drawing-python-independent` still appears under "Which tests still use a Semio-derived oracle?",
which is correct: it remains a required supplement and was neither removed nor weakened.

### `fixture reproduce --subset drawing`

```
[fixture reproduce] 0 generated fixture(s), 0 problem(s)
```

**Zero, and deliberately so — this is the honest number, not a gap I failed to fill.** A
`third-party-generated` fixture's `after` state would have to be the expected result of a mutation, and
no third-party library can compute what a `move-node` or a `change-stroke-color` should produce for
this subset; only this repository knows that. Writing those files myself and labelling them
third-party-generated is precisely the `reimplementation-registered-as-third-party` failure. The
corpus is therefore left empty until production can export real `before`/`after` carrier bytes, at
which point the readers and the pipeline registered here consume them unchanged. Note that
`fixtureProvenanceCoverage` and `fixtureReproducibilityCoverage` both stayed at 100% — nothing
unprovenanced was added to make the corpus look fuller.

---

## 8. What remains

1. **A `🏭️bridge/📜️script.ts` for this subset**, blocked on `semio-s-plugin-stdio` compiling. Until
   then `runtimeMutationCoverage` for `s.stdio.semio@v1/drawing` is 0/17 and the contract reports it.
2. **The fixture corpus**, blocked on the same thing — it needs production-exported carrier bytes, not
   hand-written ones (§7).
3. **`drag-nodes` is a partial DXF witness** and is recorded as one. If a future scenario drags only
   group-transformed nodes, DXF would witness nothing at all for it; the SVG requirement still holds.
