# Artifact Schema Design — Recipe, Spine Changes, Completeness Table, Worked Designs

Placed by **W0 recon** (this ticket, 2026-08-11) as the reference every future wave's fan-out agents should be pointed at instead of re-reading the whole plan file. Copied **verbatim** from the full plan at `~/.claude/plans/the-current-schemas-are-scalable-journal.md` — that file remains the source of truth for execution/wave structure, risk register, verification gates, and anything not reproduced below. See `w0-recon-report.md` in this same ticket folder for the W0 ownership ledger, live-artifact findings, and the recommended F1-F5 roster (which differs in places from the plan's draft roster).

---

## The per-artifact schema grammar (the recipe every standard follows)

Derived from the compose Gen-1 implemented pattern (`compose/client/lib/rs/lib.rs:9100-9320` `CanonicalKitDiff`/`apply_diff`/`Operation::to_diff`; C# `Compose.cs` `ApplyDiff`/`GetXDiff`/`XsDiff{removed,modified,added}`), with a corrected absorb.

**Snapshot** — complete per FORMAT SPEC, not per codec capability:
- Strong-like entities = the format's keyed repeating structures (GifFrame, ZipEntry, XmlNode children, OpcPart, StepEntity, GltfNode…) in ordered collections, each with its own per-field diff.
- Weak entities = value structs (GifRgb, ViewBox, ZipExtraField, IHDR groups…) — whole-value replaced in diffs, never sub-diffed.
- No `serde_json::Value` anywhere (json gets own `JsonValue` enum preserving key order + number lexemes; gltf gets fully typed 2.0 model + own local `GltfJson` for extras/extensions). No bare `Vec<u8>` except where the format's payload IS bytes (binary/deflate payloads, zip entry data, pixel/index buffers, media parts) or typed raw-retention entities (`PngChunk{kind,data}`, `GifAppExtension{identifier,auth_code,data}`) — nothing real on disk silently dropped; decode→encode of untouched docs is byte-preserving up to documented normalizations.
- Identity fields (`schema`) never appear in diffs. Copy-pasted shared types (RasterImage ×4, MeshVertex ×4, BrepMesh ×2, entries ×5) die → per-artifact named types. Spec-mandated reuse only: glb embeds gltf's document model; svg embeds xml's node model (but declares its OWN diff types).

**Diff** — handcrafted sparse structs per entity:
```rust
pub struct XDiff {
    pub field_a: Option<A>,              // present = changed to value
    pub field_b: Option<Option<B>>,      // nullable field tri-state: Some(None) = removed
    pub children: Option<XChildrenDiff>, // one per owned strong collection
}
pub struct CsDiff  { pub removed: Vec<K>, pub modified: Vec<CModified>, pub added: Vec<CAdded> }
pub struct CModified { pub key: K, pub diff: CDiff }   // recursive
pub struct CAdded    { pub index: usize, pub item: C }  // full payload + final position
```
All derive `Clone, Debug, Default, PartialEq, Serialize, Deserialize`, camelCase, `skip_serializing_if` on options, `#[artifact_schema(id = "s.stdio.<art>.diff")]`. **No `snapshot: Option<XSnapshot>` full-replace slot** — even SetSnapshot's diff is the sparse field-by-field `between(base, next)`.

Apply semantics (normative): `removed`/`modified` keys refer to BASE state (index removals processed descending; modified-of-removed illegal, ignored on apply); `added` indices refer to FINAL state (insert ascending at `min(index, len)`). Out-of-range keys = graceful no-ops.

**Key kinds per collection**: index `usize` (gif frames, txt lines, csv records, xml/svg children per level, png chunks, tiff IFDs, geometry rows, gltf top-level arrays, pdf page order) · name `String` (zip entries, OPC parts, dxf tables, json object members) · numeric id `u64` (step/ifc `#id`, pdf `(id,gen)`) · guid (bcf) · rel-id (OPC relationships). **Trees nest** — no path addressing inside diffs; `NodePath` stays mutation-level (svg precedent) and each mutation's `diff()` lowers it to a nested `CModified` chain via a per-artifact `diff_at_path` helper.

**Absorb** — structural, total, base-free (`absorb(&mut self, other)` composes base→mid with mid→after; sequential-coalesce per B-R7). Scalars: LWW. Collection triples: key/index transport φ (base→mid, from d1's removed/added; renames tracked for name keys) —
1. removed: `r1 ∪ φ⁻¹(r2 ∩ Base)`; a d2-removal of a d1-added item annihilates the add; drop m1 entries of merged-removed keys.
2. modified: d2 patch on surviving base item → recursive per-field absorb into m1 entry; d2 patch on d1-added item → patch INTO the carried added payload.
3. added: surviving a1 remapped mid→after (ψ from r2/a2) ∪ a2 verbatim; sorted ascending.
Canonical correctness cases (become unit tests everywhere): `Insert(2)`+`Remove(0)` → `{removed:[0], added:[(1,f)]}`; `Insert(2,f)`+`Insert(2,g)` → both survive (fixes gif's LWW-slot bug); `Insert(1,f)`+`SetField(1,v)` → patch-into-added. Laws: `absorb(d1,d2).apply(base) == d2.apply(d1.apply(base))`; associativity over the artifact's own vocabulary.

**Mutations** — named per-artifact enums, imperative verbs (gif/svg precedent): `#[serde(tag="mutation", camelCase)] enum XMutation { NoMutation, SetSnapshot{snapshot}, Set*/Insert*/Remove*/Rename*/Move*/Add*… }`. **Every variant's `diff()` handcrafted** — constructs the sparse XDiff directly; apply-and-capture is banned (svg's `other =>` arm deleted). `inverse(&self, base)` handcrafted per variant, key/index-aware. Each variant gets its triad dir `🧬️mutations/📄<variant-kebab>/{🦠️mutation,🔺️diff,↩️inverse}/` (existing `📄set-snapshot` + `POLICY_MUTATION_TRIAD_DIRS` pattern), mounted in glue.rs.

**Verb set per artifact**:
```rust
// 🔺️diff: impl protocol::MutationDiff<XSnapshot> for XDiff { apply, absorb }
//          + impl protocol::DiffAlgebra<XSnapshot> for XDiff (NEW trait, see spine):
fn inverse(&self, base: &XSnapshot) -> Self;          // diff-level undo
fn between(base: &XSnapshot, other: &XSnapshot) -> Self; // state delta (compose GetXDiff)
fn is_empty(&self) -> bool;
// 🧬️mutations:
pub fn apply_x_mutation(snapshot: &mut XSnapshot, mutation: &XMutation) -> XDiff
// body: let d = mutation.diff(&*snapshot); *snapshot = d.apply(snapshot); d  — diff is the single semantics source
```
`between` matching: index keys pairwise by position (modified = compare `0..min`, removed = base tail, added = other tail); name/id keys by key (renames = removed+added, documented); trees recursive with `Replace` fallback on node-kind change.

---

## Spine changes (shared files, serial waves only)

| # | Change | File |
|---|---|---|
| S-1 | NEW `pub trait DiffAlgebra<P> { inverse, between, is_empty }` next to `MutationDiff` (NOT methods on `MutationDiff` — 51 repo-wide impls would break; follow the DiffCodec W1 precedent: separate trait + seeded shrink-only policy rule requiring it for stdio artifact diffs). Normative absorb contract added to `MutationDiff::absorb` docs. | 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs |
| S-2 | `ArtifactBuilder::mutate(self, m) -> (Self, Self::Diff)` + `type Mutation: Mutation<Snapshot, Diff = Diff>` bound. Sweep ALL implementors incl. non-stdio (grep `impl ArtifactBuilder`). | 🧰framework/…/🔌️plugin/🦀️component.rs:428 |
| S-3 | DELETE dead `ArtifactEngine` trait + all ~30 impl blocks (no construction site anywhere; per-artifact ⚙️engine files STAY as codec homes — only the trait impl block is removed). | 🧰framework/…/⚙️engine/🦀️component.rs:81 + 30 artifacts |
| S-4 | `ArtifactSchemaDescriptor` gains `mutations: FacetLeaves`; all 30 descriptor constructors gain the 4th include_str! block. | 🧰framework/🔨️modules/🧬️schema/🦀️component.rs:96 |
| S-5 | `register_document_codec`: silent last-write-wins → debug_assert-panic on duplicate schema id (full dialect-keyed registry stays D4 scope). | 🧰framework/…/🏪️store/🦀️component.rs:480-605 |
| S-6 | gif 87a/89a: root `register()` calls BOTH standards' engines; gif root re-exports 89a as primary, 87a under explicit path. | 🗿️artifacts/🎞️gif/🦀️component.rs + stdio root |
| S-7 | vcs `CollectionDiff`/`CollectionMutation`: KEEP (real users: 🌊️flow/🌿️vcs `FlowMutation`, 🏪️store re-export — verified) but policy-ban from stdio artifact schema dirs. | 📜️script.ts |
| S-8 | New script.ts policy rules, all seeded shrink-only: `POLICY_FACET_MIRROR_DRIFT` (every Rust field appears camelCased in sibling .ts/.graphql/.json/.proto and vice versa; seed empty — mirrors rewritten same-wave), `POLICY_GRAMMAR_HONESTY` (no `*OCTET`/size-eos placeholder grammar leaves; seeded with current census), `POLICY_DIFF_ALGEBRA` (stdio diff types implement DiffAlgebra), stdio-ban on vcs collection machinery, field-sweep-test presence check (grep per standard). | 📜️script.ts |
| S-9 | `CommandOutcome`: untouched (app-layer, out of scope). `pack_schema_hash=[0;32]`: out of scope (hub pinning wave). | — |

> **W0 note**: S-5's target file (`register_document_codec`, store `🦀️component.rs:629-632`) is confirmed STILL the plain flat-HashMap `.insert()` (silent last-write-wins) as of this recon — V4 (scoped-down) deliberately did NOT touch it. S-1 is clear to implement here. See `w0-recon-report.md` for the S-6-equivalent finding on pdf (1.4 vs 1.7 wired backwards, same shape as gif 87a/89a) which the plan doesn't currently call out as its own spine row but should be handled alongside S-6.

---

## Snapshot completeness spec (per-agent contract, 1-3 lines each)

| Artifact | Complete model |
|---|---|
| 💾️binary raw | `bytes` (format IS bytes). Diff = splice list `Vec<ByteSplice{offset, remove_len, insert}>`, index-transported absorb. |
| 📄txt utf-8 | `lines: Vec<String>` + `trailing_newline` + `line_ending{Lf,CrLf}`; index-keyed lines. |
| 📝️md commonmark | Typed `MdBlock`/`MdInline` trees (heading/paragraph/list/code/quote/break/html-raw verbatim); recursive index triples. |
| 🔣️json rfc8259 | Own `JsonValue{Null,Bool,Number{lexeme},String,Array,Object(Vec<JsonMember>)}` — key order + number lexemes preserved. |
| 📰xml 1.0 | Existing XmlNode tree + XML decl (version/encoding/standalone), doctype raw, PI/comment/CDATA distinction. |
| 📊️csv rfc4180 | records×fields + has_header + per-field `quoted` retention. |
| 🎨️svg 1.1 | Already rich; adds decl/doctype via xml model; OWN recursive diff types (see worked design). |
| 🎒️zip 2.0 | Already complete; diff = name-keyed entry triples + rename-aware absorb transport. |
| 🗜️deflate rfc1950 | Typed zlib container: cmf/flg fields, dict_id, decompressed payload; adler32 recomputed. |
| 📷️png 1.2 | Typed IHDR/PLTE/tRNS + typed ancillary set (gAMA,cHRM,sRGB,pHYs,tIME,bKGD,tEXt/zTXt/iTXt) + decoded pixels + chunk order + unknown chunks verbatim (`PngChunk`). |
| 📷️jpg jfif-1.01 | Typed JFIF APP0, SOF, DQT/DHT id-keyed, restart interval, other APPn/COM verbatim, decoded pixels + re-encode quality. |
| 🖼️bmp v3 | Full BITMAPINFOHEADER (11 fields), palette, height-sign/bottom-up, decoded rows. |
| 🖼️tiff 6.0 | byte_order + IFD list w/ typed tag entries (`TiffValues` union); unknown tags typed-raw. |
| 🎞️gif 87a | Screen descriptor + GCT + `images` (left/top/w/h/interlace/lct/indices) — no GCE (spec has none). |
| 🎞️gif 89a | GCT/bg-index/aspect + per-frame LCT/interlace/`indices` (palette-lossless; rgba becomes derived accessor) + GCE (transparent_index: Option<u8>) + loop + comments + plain-text + unknown app extensions verbatim. |
| 📄️pdf 1.4(+1.7) | Object graph `(id,gen)`-keyed, own `PdfValue` (incl. `Stream{dict, raw, decoded: Option}`), trailer; page-tree accessors; replaces `PageDoc`. |
| 🖊️dwg ac1018/24 | Honest boundary: version/maintenance/codepage header + name-keyed sections (proprietary; opaque-by-spec allowed) + decoded header variables where codec reads them. |
| 🖊️dxf r12 | `$VAR`-keyed typed header, name-keyed tables (layers/styles/linetypes), blocks, typed r12 entity list (LINE/CIRCLE/ARC/POLYLINE/TEXT/SOLID/INSERT…), unknown group codes retained `(code, raw)`. |
| 🟪️stl ascii | solid_name + index-keyed `StlTriangle{normal, vertices}` — complete. |
| 🧊️obj 3.0 | v/vt/vn rows, faces w/ index triples, name-keyed groups/objects, smoothing, mtllib/usemtl, unknown statements retained in position. |
| ☁️ply 1.0 | format(+endian), comments, name-keyed elements w/ typed properties (scalar/list) + typed rows. |
| ☁️las 1.0 | Full public header typed, VLRs verbatim, typed point records per format id (0/1). |
| 📐️step ap214 | ISO 10303-21: typed HEADER triple, id-keyed `StepEntity{id, name, args: Vec<StepValue>}` w/ own value enum ($,*,refs,enums,aggregates). BrepMesh scrape → analyzer. |
| 🏗️ifc 4 | Same exchange grammar with OWN `IfcEntity`/`IfcValue` types. |
| 🧊️gltf 2.0 | Fully typed 2.0 schema (asset/scenes/nodes/meshes+primitives/accessors+sparse/bufferViews/buffers/materials/textures/images/samplers/skins/animations/cameras/extensionsUsed) + own `GltfJson` for extras/extensions. Kills serde_json::Value. |
| 🧊️glb 2.0 | (merged into gltf by V2a — verified real in W0, not a stub: real `.glb` 12-byte header + chunk walker + BIN chunk embed, `GltfSourceForm::Glb` tracked) container: version + embedded gltf document + BIN + unknown chunks verbatim. |
| 📕️xlsx ecma-376 | OPC model (content_types defaults/overrides, name-keyed parts Xml-or-Bytes, rId-keyed relationships) + typed workbook layer (sheets, shared strings, `(row,col)` cells). NOT Vec<ZipEntry>. |
| 📜️docx ecma-376 | OWN OPC copy + typed document layer (body block tree: paragraphs/runs/props/tables; styles part). Pattern-setter for the OPC trio. |
| 🎞️pptx ecma-376 | OWN OPC copy + slide list + per-slide shape tree + layouts/masters as parts. |
| 💬️bcf 2.1 | bcfzip typed: version + guid-keyed topics (markup, guid-keyed comments/viewpoints w/ typed camera/components, PNG snapshot bytes); unknown files retained as parts. |

Rule everywhere: codec fills what it decodes; spec-real-but-undecoded regions carry typed raw retention; nothing fabricated (bachelor-thesis-fixture honesty pattern).

---

## Worked designs (normative examples for the fan-out briefs)

- **gif 89a**: snapshot per table above; `GifDiff{width, height, gct: Option<Option<GifColorTable>>, background_color_index, pixel_aspect_ratio, loop_count: Option<Option<u16>>, frames: Option<GifFramesDiff>, comments, app_extensions}` with `GifFramesDiff{removed: Vec<usize>, modified: Vec<GifFrameModified{index, diff: GifFrameDiff}>, added: Vec<GifFrameAdded{index, frame}>}`; ~20 mutations (SetScreenSize, SetGlobalColorTable, SetLoopCount, InsertFrame, RemoveFrame, MoveFrame, SetFrameGeometry, SetFramePixels, SetFrameDelay, SetFrameDisposal, SetFrameTransparency, SetFrameInterlace, SetFrameUserInput, Insert/RemoveComment, Add/RemoveAppExtension…). Old op-slot GifDiff deleted outright.
  - **W0 note**: current gif 89a diff (`GifDiff`, 140 lines) only covers `FrameInsert`/`FrameDelay`/`LoopCountChange`/`FrameDisposalChange` — no GCT, background_color_index, pixel_aspect_ratio, comments, or app_extensions coverage yet. Mutation enum has only 6 of the ~20 target variants (NoMutation, SetSnapshot, InsertFrame, RemoveFrame, SetFrameDelay, SetLoopCount, SetFrameDisposal). `apply_gif_mutation` still returns `()`, not a `Diff`. `absorb` exists but per the plan's own intro carries the known op-slot LWW bug that loses coalesced inserts. This artifact is the FURTHEST ALONG of the 31 standards but is still "partial", not "rich" — F3's gif agent has real remaining scope, not just polish.
- **zip**: `ZipDiff{comment, entries: Option<ZipEntriesDiff>}` name-keyed; `ZipEntryDiff` covers all 13 entry fields incl. `name: Option<String>` rename (absorb transport tracks renames) and tri-state `unix_mtime: Option<Option<i64>>`; ~14 mutations (AddEntry/RemoveEntry/RenameEntry/SetEntryData/Method/Timestamps/Flags/Versions/Attributes/Extra/Comment/SetArchiveComment).
- **svg**: `SvgDiff{root: Option<SvgNodeDiff>}`; `SvgNodeDiff = Element(SvgElementDiff) | Text{text} | Replace{node}`; `SvgElementDiff{name, attributes: Option<SvgAttributesDiff> (name-keyed triple), children: Option<SvgChildrenDiff> (index-keyed recursive triple)}`; `diff_at_path(NodePath, leaf) -> SvgDiff` lowers mutation paths to nested chains. Existing 8 mutations get handcrafted diffs; xml gets the same shape with own `Xml*` names.

## Facet mirrors & grammar leaves (user decision: ALL handcrafted)

Per schema facet dir (snapshot/diff/mutations): handcraft `🟦️component.ts` (real interfaces matching serde shapes, discriminated unions on the `mutation` tag), `🔗️component.graphql`, `🔣️component.json` (JSON Schema), `🛰️component.proto`, AND all grammar leaves honestly — `📝️text/`: .g4, .ebnf, 📖️.grammar.semio (repo-native normative); `💾️binary/`: .ksy, .spicy, 🔠️.abnf, 📡️.protocol.semio. No `*OCTET`/size-eos placeholders survive; each agent handcrafts its own artifact's full leaf set in its wave (they know the model best). `POLICY_FACET_MIRROR_DRIFT` + `POLICY_GRAMMAR_HONESTY` (S-8) keep them honest forever. The io serializer/deserializer leaves become honest `ArtifactSerializer`/`ArtifactDeserializer` trait impls (per-leaf, replacing 3-line free functions). Note `POLICY_TS_FACADE_CONSTITUTIONAL_FACETS` already accepts TS stubs under triad leaf dirs — triad `📄<variant>` leaves stay thin delegates; the real TS mirrors live at the facet level.

## Test laws (per artifact, in existing test regions, same names everywhere for greppability)

1. `mutation_diff_law`: ∀ variant: `m.diff(base).apply(base) == { apply_x_mutation(&mut s, m); s }` and returned diff == `m.diff(base)`.
2. `inverse_law`: mutation-level (every variant round-trips) + diff-level `d.inverse(base).apply(&d.apply(base)) == base`.
3. `absorb_law`: cartesian product over curated op list (MUST include Insert+Remove-before, Insert+Insert same index, Add+Rename, Add+SetField, Modify+Remove per key kind): `absorb(d1,d2).apply(base) == sequential`; associativity.
4. `between_roundtrip_law`: `between(a,b).apply(a) == b` on fixtures (dancing.gif, bachelor-thesis, architectural, metabolism) + synthetic.
5. `codec_retention_law`: decode→encode byte-preserving (or documented normal form) on fixtures.
6. **`field_sweep` (THE acceptance criterion for "diff can change every field")**: per artifact, `sweep_a()`/`sweep_b()` snapshots differing in EVERY mutable field (incl. per collection: one removed, one modified-in-every-field, one added; every tri-state exercising Some(None)). Assert `between(a,b).apply(a)==b`, `between(b,a).apply(b)==a`, structurally every patch field `is_some()` (hand-written per-field assertion — fails when a snapshot field is added until diff+sweep are extended), `between(a,a).is_empty()`.

---

*For execution wave structure (W0/S1/S2/F1-F6/G), glue.rs ownership policy, per-fan-out-agent briefing contract, sizing, and the risk register, see the full plan file: `~/.claude/plans/the-current-schemas-are-scalable-journal.md`.*
