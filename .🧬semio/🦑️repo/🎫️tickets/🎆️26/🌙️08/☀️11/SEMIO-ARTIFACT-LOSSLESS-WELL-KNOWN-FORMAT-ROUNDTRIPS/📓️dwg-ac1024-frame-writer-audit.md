# DWG AC1024 Frame and Writer Audit

## Scope and evidence

This is a read-only audit of `/Users/ueli/Documents/semio/temp/architectural_example.dwg`.
No DWG production file was edited. The fixture is AC1024, 148,638 bytes, SHA-256
`52d14a7bdb946099d3cf16fd276d19bd8924348fd02b2ddd0003cd4f6b34cce7`. Its decoded
inventory has 652 framed objects. The exact type and class counts are in
`🧪️dwg-object-type-inventory.log` and `🧪️dwg-custom-class-inventory.log`.

Primary format authority: [Open Design Specification for DWG Files](https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf),
especially sections 2.12, 20.1, 20.2, 20.4, 23 and 28. Named implementation-layout
research: LibreDWG [`dwg.spec`](https://raw.githubusercontent.com/LibreDWG/libredwg/master/src/dwg.spec),
[`dwg2.spec`](https://raw.githubusercontent.com/LibreDWG/libredwg/master/src/dwg2.spec), and
[`dwg_spec_shared.h`](https://raw.githubusercontent.com/LibreDWG/libredwg/master/src/dwg_spec_shared.h).
LibreDWG layouts are candidate field prescriptions, not proof; any `HANDLE_UNKNOWN_BITS` path is
forbidden by this ticket and must remain a typed rejection.

## Exact R2010 frame offsets and stream contract

All offsets below are relative to the object address obtained from `AcDb:Handles`. That address is
an offset into the decompressed logical `AcDb:AcDbObjects` section, not a physical file address.

| Relative location | R2010 field | Writer requirement |
| --- | --- | --- |
| `+0` | `MS object_size_bytes` | Modular-short size excludes the trailing `RS` CRC. Retain the exact encoded MS bytes for CRC input while materializing, but never persist them in the schema. |
| `+len(MS)` | `MC handle_stream_bits` | Unsigned modular-char; `0x40` is data, not a sign bit. Count includes handle-stream terminal padding. This field replaces the R2000-R2007 pre-handle `RL bitsize`. |
| next byte boundary | payload bit 0: `BOT object_type` | R2010 BOT is a two-bit selector then one/two raw bytes: selector 0 byte; 1 byte + `0x1f0`; 2 raw short; 3 decode as 2. Dynamic type `500 + n` selects class-list index `n`; class list order is semantic. |
| immediately after BOT | `H object_handle` | Handle must equal the handle-map key. Handle code/counter and relative resolution are validated; imported handles are not reallocated. |
| next | typed EED sequence | Repeated `BS size`, application handle, and typed EED items, terminated by zero size. R2007+ EED strings are length plus UTF-16 code units. |
| remaining pre-handle region | common prefix then class body | Separate bounded data reader. For entities, graphic presence and optional `BLL + graphic` precede entity common data. |
| backward from pre-handle end | optional string stream | Last pre-handle bit is presence. If set, the preceding `RS` gives string bit size; high bit adds a second `RS` and the total is `low15 | high<<15`. Unicode fields are read from this independent stream. |
| `payload_bits - MC` | handle stream | Separate bounded handle reader: common references first, then class-specific references, then zero padding to a byte. |
| after payload | `RS CRC` | CRC seed `0xC0C1`; ODA explicitly says CRC includes the object-size bytes. Exact acceptance must test the fixture's CRC span, including the MS prefix and R2010 MC/frame bytes, rather than CRC over payload alone. |

The `AcDb:Handles` writer must rebuild entries from emitted object offsets. Blocks start with a
big-endian `RS` block size, contain unsigned handle deltas and signed location deltas as modular
chars, end with their CRC, are capped at 2032 bytes, and terminate with the size-2 empty block.

## Critical live-writer corrections before any type body

The current `dwg_write_object` is not an AC1024 frame writer. It emits `MS`, a pre-R2010 `BS type`,
an `RL bitsize`, handle, body, handles, and CRC over payload only. It omits R2010 `MC`, BOT, EED,
string-stream size/presence, and the specified CRC span. `dwg_encode_entity_common` also omits the
graphic flag, full R2010 color payload, material/shadow/visual-style presence state,
invisibility/lineweight ordering, and conditional handles. No class-body implementation can be
accepted until this foundation is replaced by a version-gated AC1024 writer.

For R2010 entity main data, the required order after graphic/EED is: `entmode BB`, reactor count
`BL`, xdictionary-missing `B`, `ENC` color (alpha first, then either a color reference or RGB, then
color-name/book semantics), linetype scale `BD`, linetype flags `BB`, plotstyle flags `BB`, material
flags `BB`, shadow `RC`, three visual-style-presence bits (full/face/edge), invisible `BS`, lineweight
`RC`, then the class body. LibreDWG version-gates `nolinks` through R2002; the ODA prose saying it is
always one for R2004+ describes the semantic condition, not an emitted AC1024 bit. The corresponding
common handle stream is optional color reference first, owner when `entmode==0`, reactors, optional
xdictionary, layer, conditional linetype, material, shadow, plotstyle, and the visual-style references
selected by the three R2010 bits. The three
visual-style main bits and matching handles are currently absent and are a concrete desynchronizer.

For non-entities, common main data is reactor count then xdictionary-missing; common handle order is
owner, reactors, optional xdictionary. Class-specific data/strings/handles follow. Counts must be
derived from logical collections and every bounded stream must be consumed exactly.

## Prioritized type-layout and writer matrix

| Priority | Fixture family/count | Typed field layout | Symmetric writer gate |
| --- | ---: | --- | --- |
| P0 | All 652 frames | AC1024 frame/common object/common entity/EED/string/handle/CRC contract above | Exact per-frame main/string/handle consumption; exact CRC; rebuilt handle map; no identity-only fallback |
| P1 | XRECORD 145; DICTIONARY 83; WDFLT 1; DICTIONARYVAR 8 | XRECORD: `xdata_size BL`, typed resbuf sequence by group code, `cloning BS`, object-id handles. DICTIONARY: `count BL`, `cloning BS`, `hard_owner RC`, names in string stream, matching refs in handle stream. WDFLT adds default ref. DICTIONARYVAR: schema byte plus Unicode value. | Derive byte/count fields; preserve logical pair/value order; forbid raw `databytes`; reject unknown resbuf code atomically. This unlocks the ownership spine. |
| P2 | Block graph 43; table graph 48 | BLOCK/ENDBLK markers; INSERT point/scale/rotation/extrusion and block/attribute/seqend refs; BLOCK_CONTROL count plus header refs and model/paper refs; BLOCK_HEADER strings, flags, base/xref/description/preview/units and ownership graph. Controls derive record counts; table records use Unicode name/xref state then typed LAYER/STYLE/LTYPE/VPORT/APPID/DIMSTYLE fields. | Emit owner/reactor/xdic before role handles; preserve logical record/owned-entity order; exact reference resolution; preview only as semantic image data. Empty VIEW/UCS controls remain valid typed empty lists. |
| P3 | LINE 40; ARC 12; DIMENSION_LINEAR 12; LWPOLYLINE 16 | LINE: `z_zero B`, X/Y `RD+DD`, conditional Z `RD+DD`, `BT`, `BE`. ARC: center `3BD`, radius `BD`, thickness `BT`, extrusion `BE`, start/end `BD`. LWPOLYLINE: flags then conditional width/elevation/thickness/normal, counts, ordered vertices/bulges/ids/widths. DIMENSION R2010 version `RC`, common dimension fields, linear extension points, oblique/rotation, dimstyle/block refs. | Derive all presence/count flags. Preserve parallel LWPOLYLINE arrays and dimension refs. Entity common visual-style correction must land first. |
| P4 | VIEWPORT 2; MLINESTYLE 1; PLACEHOLDER 1; LAYOUT 2 | VIEWPORT and MLINESTYLE complete ODA fields; LAYOUT page setup/plot strings and numeric policy, layout/UCS/extents/axes/viewport collections; PLACEHOLDER has common object state only. | String fields go only to string stream; plot-view, visual-style, block, UCS and viewport refs go only to handle stream; conditional version fields obey R2010 gates. |
| P5 | VISUALSTYLE 19; SCALE 17; SORTENTSTABLE 7; MATERIAL 3; EVALUATION_GRAPH 2; TABLESTYLE/MLEADERSTYLE 1 each | Use ODA fields for style/context records. VISUALSTYLE R2010 is description plus 28 typed property/modifier pairs and no class-local handles. SCALE is name plus paper/drawing units and scale flag. SORTENTSTABLE is ordered entity/sort-handle pairs. Evaluation graph is typed nodes/edges. | Derive counts; require exact handle-stream exhaustion. No packed style record or unknown graph bytes. |
| P6 | Dynamic-block custom 71 | Highest count first: BLOCKGRIPLOCATIONCOMPONENT 23 uses EvalExpr parent/major/minor/value-code/value/node then grip type/string; BLOCKREPRESENTATION_DATA 12; STRETCHACTION 6; then parameter/grip/action bodies from `dwg2.spec`. | Base-before-derived field order, typed EvalVariant, strings in string stream, code-91 value handle in logical position. Enable each class only after all same-class frames consume exactly. |
| P7 | Associative custom 117 | GEOMDEPENDENCY 31 is dependency core plus geom version/enabled/persistent-subentity name/compound flag. VALUEDEPENDENCY 23 is the same dependency core with its inherited name interpreted as value name. VARIABLE 18 is the R2010 action core plus named variable/expression/evaluator/description/value/merge semantics. LibreDWG's `HANDLE_UNKNOWN_BITS` is non-consuming debug capture, not an unknown prefix. | Remove debug raw capture, implement the typed candidates, and require class-wide bounded-consumption and graph-role proof before writing. Never preserve opaque bits. |

The most valuable concrete type order from the fixture is therefore 79, 42, 67, 19, 544, 520,
506, then the typed-candidate 541/545 families. This order maximizes reference-graph coverage while keeping
writer proof ahead of body breadth.

## Acceptance requirements

1. Every one of 652 frames has one typed body or a typed import rejection; no raw/unknown frame.
2. Main, string and handle streams have exact bounded-consumption assertions per frame.
3. Writer emits AC1024 MC/BOT framing and CRC span, then regenerates `AcDb:Handles` from offsets.
4. Decode-write equality is checked against the original 148,638-byte fixture, not a canonical intermediate.
5. DSL/pack, diff/apply/inverse/absorb, mutation/inverse, analyzer and composer retain every logical object.

## Related completed lanes and exact commands

- PPTX/ZIP report: `🛠️pptx-implementation.md`; exact log: `🧪️pptx-exact-logical-lifecycle.log`.
- MP4 report: `🛠️mp4-pptx-implementation.md`.
- PPTX lifecycle: `CARGO_TARGET_DIR="$TICKET_DIR/🎯️mp4-pptx-logical-target" bun nx run @semio-tech/stdio-plugin:test-long -- fixture_survives_logical_io_persistence_diff_and_mutation_pipelines --nocapture`.
- ZIP/OPC lifecycle: `CARGO_TARGET_DIR="$TICKET_DIR/🎯️mp4-pptx-logical-target" bun nx run @semio-tech/stdio-plugin:test-long -- zip_logical_lifecycle_reconstructs_original_pptx_fixture --nocapture`.
- MP4 lifecycle: `CARGO_TARGET_DIR="$TICKET_DIR/🎯️mp4-pptx-logical-target" bun nx run @semio-tech/stdio-plugin:test-long -- exact_bauen_mit_bestand_fixture_round_trips_byte_for_byte --nocapture`.
- Anti-shadow gate: `CARGO_TARGET_DIR="$TICKET_DIR/🎯️mp4-pptx-logical-target" bun nx run @semio-tech/stdio-plugin:test-long -- facets_exclude_shadow_or_raw_state --nocapture`.

Recorded results: PPTX 1/1 in 17.53s, ZIP/OPC 1/1 in 9.56s, MP4 1/1 in 11.24s,
and anti-shadow MP4/PPTX/ZIP 3/3.

## P2a/P2b AC1024 table and block checklist

This checklist specializes the R2010 branch of LibreDWG
[`COMMON_TABLE_FLAGS`](https://github.com/LibreDWG/libredwg/blob/master/src/spec.h#L754-L800),
[`BLOCK` through `MINSERT`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec#L610-L1054),
and [`BLOCK_CONTROL` through `DIMSTYLE`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec#L3125-L4453).
`R_2010b` gates below apply to the AC1024 fixture. Types are logical schema concepts; encoded count,
presence, compact-number, string-footer and relative-handle forms are derived by the writer.

### Shared R2010 table contract and controls

Every record below starts, after the non-entity common fields, with the R2007+ table scalar prefix:
Unicode `name T` in the object string stream, then `is_xref_resolved BS` in main data. The table
`xref H` is the first table-specific handle after common owner/reactor/extension-dictionary handles.
LibreDWG's R13+ [`VALUE_H`](https://github.com/LibreDWG/libredwg/blob/master/src/enc_macros.h#L708-L717)
always routes `FIELD_HANDLE` and `SUB_FIELD_HANDLE` to `hdl_dat`, even when a handle declaration is
textually interleaved with scalar declarations in `dwg.spec`; declaration order still determines its
relative position among handles.
The older `is_xref_ref B` and `is_xref_dependent B` fields end with the R2004 layout and must not be
read or written for AC1024. Every listed table record has a string-stream footer; fixed controls do
not. Control main data and role handles are:

| Type | AC1024 main data | Handle-stream order after common owner/reactors/xdic |
| --- | --- | --- |
| `BLOCK_CONTROL` 48 | `entry_count BL` | exactly `entry_count` entries, model-space, paper-space |
| `LAYER_CONTROL` 50 | `entry_count BL` | exactly `entry_count` entries |
| `STYLE_CONTROL` 52 | `entry_count BL` | exactly `entry_count` entries |
| `LTYPE_CONTROL` 56 | `entry_count BS` | entries, by-block, by-layer |
| `VPORT_CONTROL` 64 | `entry_count BS` | exactly `entry_count` entries |
| `APPID_CONTROL` 66 | `entry_count BS` | exactly `entry_count` entries |
| `DIMSTYLE_CONTROL` 68 | `entry_count BS`, `more_count RC` | entries, then exactly `more_count` hard references |

Schema: use distinct tagged control bodies or validate the `type_code` against its allowed extras;
`entry_count` and `more_count` are derived from semantic vectors. Decoder: do not run the backward
string-footer parser on controls, and reject a null/missing/count-mismatched role handle atomically.
Writer: always emit the control owner handle field even when null, then reactors/xdic, then the roles
above; never use one universal BS count.

Version-gate checklist: before R13 these control counts are `RS`; in the R13+ object layout
BLOCK/LAYER/STYLE use `BL`, while LTYPE/VPORT/APPID/DIMSTYLE use `BS`. BLOCK_CONTROL adds
paper-space after R13 and always places model-space before it. DIMSTYLE_CONTROL adds `more_count RC`
and its matching hard-reference vector from R2000. For AC1024 all of those later branches apply.

### P2b typed table records

| Record | R2010 class-main order after the shared table prefix | Class-specific handle order / writer conditions |
| --- | --- | --- |
| `LAYER` 51 | `flag0 BS` (frozen, off, frozen-in-new, locked, plot, lineweight), `color CMC` | after common + table-xref handles: plotstyle, material, linetype. No visualstyle before R2013. Model each flag and color logically; derive packed `flag0`. |
| `STYLE` 53 | `is_shape B`, `is_vertical B`, `text_size BD`, `width_factor BD`, `oblique_angle BD`, `generation RC`, `last_height BD`, `font_file T`, `bigfont_file T` | common + table-xref handles only. Font strings belong only to the string stream. |
| `LTYPE` 57 | `description T`, `pattern_length BD`, `alignment RC`, `dash_count RC`; for each dash: `length BD`, `shape_code BS`, style `H`, X/Y offsets `RD`, scale `BD`, rotation `BD`, `shape_flags BS`; when any dash has text, a 512-byte R2007+ Unicode strings area follows | after common + table-xref handles: each dash's style ref in dash order. Persist each dash's semantic text, not the 512-byte area; materialize the area deterministically and reject overflow. Although style `H` is declared within the dash loop, it is emitted in the handle stream. |
| `VPORT` 65 | `view_height BD`, `view_width BD`, center `2RD`, target `3BD`, direction `3BD`, twist/lens/front/back `BD`, `view_mode 4BITS`, `render_mode RC`; R2007+: default-lights `B`, lighting-type `RC`, brightness/contrast `BD`, ambient `CMC`; then lower-left/upper-right `2RD`, UCS-follow `B`, circle-zoom `BS`, fast-zoom `B`, UCS-icon `BB`, grid-mode `B`, grid-unit `2RD`, snap-mode/style `B`, snap-isopair `BS`, snap-angle `BD`, snap-base `2RD`, snap-unit `2RD`, UCS-at-origin/UCSVP `B`, UCS origin/X/Y `3BD`, elevation `BD`, ortho-view `BS`, grid-flags/grid-major `BS` | after common + table-xref handles: background, visual-style, sun, named-UCS, base-UCS. AC1024 includes snap angle/base; the special omission is only `dwg_version == 0x1a` (AC1020/R2006). Background/visual-style/sun are declared before later main scalars but encoded in the handle stream. |
| `APPID` 67 | registered-application group-71 value `RC` | common + table-xref handles only. Retain the named semantic group-71 value; it is an undocumented logical APPID field, not container bytes. |
| `DIMSTYLE` 69 | ordered groups listed below | ordered trailing references listed below; do not flatten them into an unordered bag. |

The complete AC1024 `DIMSTYLE` main sequence after the shared prefix is:

1. `DIMPOST T`, `DIMAPOST T`; `DIMSCALE BD(1 default)`; `DIMASZ`, `DIMEXO`, `DIMDLI`,
   `DIMEXE`, `DIMRND`, `DIMDLE`, `DIMTP`, `DIMTM` as `BD(0 default)`.
2. R2007+ `DIMFXL BD`, `DIMJOGANG BD`, `DIMTFILL BS`, `DIMTFILLCLR CMC`.
3. `DIMTOL B`, `DIMLIM B`, `DIMTIH B`, `DIMTOH B`, `DIMSE1 B`, `DIMSE2 B`, `DIMTAD BS`,
   `DIMZIN BS`, `DIMAZIN BS`; R2007+ `DIMARCSYM BS`.
4. `DIMTXT`, `DIMCEN`, `DIMTSZ`, `DIMALTF`, `DIMLFAC`, `DIMTVP`, `DIMTFAC`, `DIMGAP`,
   `DIMALTRND` as `BD`; `DIMALT B`, `DIMALTD BS`, `DIMTOFL B`, `DIMSAH B`, `DIMTIX B`,
   `DIMSOXD B`; colors `DIMCLRD`, `DIMCLRE`, `DIMCLRT` as `CMC`.
5. `DIMADEC`, `DIMDEC`, `DIMTDEC`, `DIMALTU`, `DIMALTTD`, `DIMAUNIT`, `DIMFRAC`,
   `DIMLUNIT`, `DIMDSEP`, `DIMTMOVE`, `DIMJUST` as `BS`; `DIMSD1 B`, `DIMSD2 B`,
   `DIMTOLJ BS`, `DIMTZIN BS`, `DIMALTZ BS`, `DIMALTTZ BS`, `DIMUPT B`, `DIMATFIT BS`.
6. R2007+ `DIMFXLON B`; R2010+ `DIMTXTDIRECTION B`, `DIMALTMZF BD`, `DIMALTMZS T`,
   `DIMMZF BD`, `DIMMZS T`; then `DIMLWD BS`, `DIMLWE BS`, and `flag0 B`.

The DIMSTYLE handle stream, after common + table-xref handles, is `DIMTXSTY`, `DIMLDRBLK`,
`DIMBLK`, `DIMBLK1`, `DIMBLK2`, then R2007+ `DIMLTYPE`, `DIMLTEX1`, `DIMLTEX2`. `DIMTXSTY`
is declared between scalar groups but is routed to the same handle stream and retains that first
class-specific handle position. Schema should group these named values without changing order;
decoder and writer must enforce each scalar width/default and the exact main/string/handle boundaries.

Table-record gate checklist: LAYER gains packed `flag0` and plotstyle at R2000, material at R2007,
and visualstyle only at R2013; AC1024 therefore stops at material. STYLE's typed body is stable from
R13. LTYPE uses a conditional 512-byte Unicode strings area from R2007 (256 bytes through R2004),
derived solely from text-bearing dashes. VPORT gains render/UCS state at R2000 and background,
visual-style, sun, lighting and grid state at R2007; AC1020/R2006 alone omits snap-angle/base, so
AC1024 includes them. APPID's group-71 `RC` is present from R13. DIMSTYLE's main later layout begins
at R2000, adds `DIMFXL`/jog/fill/arc/fixed-extension and linetype handles at R2007, and adds text
direction plus alternate/main measurement zero-suppression factor/string pairs at `R_2010b`; every
one of those gates is active for the fixture.

### P2a block ownership graph

| Body | AC1024 typed class-main order | Handle and graph order |
| --- | --- | --- |
| `BLOCK` 4 | Unicode block name only | common entity handles; owner resolves to its `BLOCK_HEADER`. Header flags/base/xref/description are not duplicated into this marker. |
| `ENDBLK` 5 | no class fields | common entity handles; owner is the same header. |
| `SEQEND` 6 | no AC1024 class fields | common entity handles; owner is the owning INSERT/MINSERT or polyline. |
| `INSERT` 7 | insertion point `3DPOINT`; `scale_flag BB` followed by its exact scale variant (3=no values, 2=one `RD`, 1=Y/Z `DD` from 1, 0=X `RD` plus Y/Z `DD` from X); rotation `BD`; extrusion `3DPOINT`; `has_attributes B`; if true `owned_count BL` | common entity handles, block-header ref, exactly `owned_count` attributes, then seqend when attributes exist. The R13-R2000 first/last pair is absent in AC1024. |
| `MINSERT` 8 | INSERT fields through optional `owned_count`, then columns `BS`, rows `BS`, column spacing `BD`, row spacing `BD` | same AC1024 handle order as INSERT. |
| `ATTRIB` 2 | compact text `dataflags RC`; conditional elevation `RD`, insertion `2RD`, conditional alignment `2DD`, extrusion `BE`, thickness `BT`, conditional oblique/rotation `RD`, height `RD`, conditional width `RD`, value `T`, conditional generation/horizontal/vertical `BS`; R2010 locked-in-block `RC`; tag `T`, field-length `BS`, attribute flags `RC`, lock-position `B`, keep-duplicate `RC` | common entity handles, then text-style ref. Reject invalid tag/boolean/range values instead of silently normalizing imported state. |
| `ATTDEF` 3 | same compact text sequence using default-value `T`; R2010 locked-in-block `RC`; tag `T`, field-length `BS`, flags `RC`, lock-position `B`, keep-duplicate `RC`, prompt `T` | common entity handles, then text-style ref. R2018 embedded-MTEXT fields are absent in AC1024. |
| `BLOCK_HEADER` 49 | shared table prefix; anonymous/has-attributes/is-xref/is-overlaid/xref-loaded `B`; if not xref/overlay `owned_count BL`; base point `3DPOINT`; xref path `T`; insert count encoded as the terminated RC sequence; description `T`; preview size `BL` plus semantic preview; R2007+ insert-units `BS`, explodable `B`, block-scaling `RC` | common object handles, table-xref, block-entity, exactly `owned_count` entities for non-xref blocks, endblk, exactly the derived insert backrefs, layout. R13-R2000 first/last entity refs are absent in AC1024. |

Block-graph gate checklist: BLOCK/ENDBLK/SEQEND use the R13+ Unicode/common-handle forms. INSERT
and MINSERT use the R2000 compact two-bit scale form and gain `num_owned BL` plus the ordered
attribute vector at R2004; their R13-R2000 first/last attribute pair is not encoded in AC1024.
ATTRIB/ATTDEF use compact text `dataflags` from R2000, lock-position from R2007, and
locked-in-block/keep-duplicate bytes from `R_2010b`; R2018 embedded-MTEXT is absent. BLOCK_HEADER
uses separate Boolean flags from R13, xref-loaded plus terminated insert-count/description/preview
from R2000, `num_owned BL` and its entity vector from R2004, and insert-units/explodable/scaling from
R2007. The terminated insert count is writer-derived as one `RC(1)` per backref followed by `RC(0)`.

Schema invariants: one handle-keyed entity authority must join geometry, common state and the tagged body;
do not associate the current independent `entities` and `objects` vectors by position. A block header owns
an ordered entity list whose first/last semantic markers are BLOCK/ENDBLK, INSERT attributes are owned in
order and terminate at one SEQEND, and every forward reference has the required reverse ownership/backref.
Preview bytes are permitted only as decoded semantic image content.

Decoder checklist: select the AC1024 gates above; bound main/string/handle readers separately; preserve
the declared relative order of all handles in the dedicated handle stream; validate derived counts, owner types, duplicate handles,
BLOCK/ENDBLK pairing and exact terminal fill; reject unsupported table variants atomically instead of
emitting a name-only record. Writer checklist: derive all compact flags/counts from logical fields, build
the string stream once, emit the handle stream in prescribed declaration order, calculate frame
and handle-map offsets in two passes, and require decode-write equality for every same-type fixture frame
before enabling that tagged body.

## P3/P4 AC1024 geometry and fixed-support checklist

This checklist specializes the R2010 native-DWG branches of LibreDWG
[`ARC` and `LINE`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec#L1461-L1574),
[`DIMENSION_LINEAR`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec#L1756-L1799),
[`COMMON_ENTITY_DIMENSION`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg_spec_shared.h#L27-L143),
[`VIEWPORT`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec#L2412-L2528),
[`MLINESTYLE`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec#L4513-L4570),
[`LAYOUT`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec#L5316-L5443),
[`LWPOLYLINE`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec#L5446-L5541), and
[`PLACEHOLDER`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L1185-L1189).
The ODA sections are 20.4.4/20.4.6, 20.4.8, 20.4.50, 20.4.85 and the named object records in
section 20.2. As elsewhere, `T` values are logical strings in the independent string stream and every
`H` is routed to the handle stream regardless of where its declaration appears among scalar fields.

### P3 geometry bodies

| Body | Exact AC1024 class-main order | Class handle order and writer gates |
| --- | --- | --- |
| `ARC` | center `3BD`, radius `BD`, thickness `BT0`, extrusion `BE`, start-angle `BD`, end-angle `BD` | common entity handles only. Store angles/radius/geometry, not compact selectors; derive default-zero thickness and default-Z extrusion encodings. Reject non-finite values and negative radius. |
| `LINE` | derived `z_is_zero B`; start-X `RD`, end-X `DD(start-X)`, start-Y `RD`, end-Y `DD(start-Y)`; when Z is nonzero, start-Z `RD`, end-Z `DD(start-Z)`; thickness `BT0`, extrusion `BE` | common entity handles only. `z_is_zero` is never schema state: derive it from both endpoint Z values. Preserve exact endpoint semantics while selecting each `DD` representation deterministically. |
| `DIMENSION_LINEAR` | sequence listed below | common entity handles, then dimstyle, then anonymous dimension block. Derive native version/flag bytes and require both references to resolve to the right object kinds. |
| `LWPOLYLINE` | derived `flag BS`; conditional constant-width `BD`, elevation `BD`, thickness `BD`, extrusion `3BD`; point count `BL`; conditional bulge count `BL`, R2010b vertex-id count `BL`, width count `BL`; first point `2RD`, later points `2DD(previous)`; all bulges `BD`; all vertex IDs `BL`; all start/end-width `BD` pairs | common entity handles only. Native arrays are grouped, not DXF-interleaved. Derive all flag presence bits and counts from logical values/vectors; maximum 20,000 points; vertex IDs, when present, must match point count. |

The complete native AC1024 `DIMENSION_LINEAR` main sequence is:

1. Derived `class_version RC = 0` for the `R_2010b` representation.
2. Common dimension values: extrusion `3BD`, text midpoint `2RD`, elevation `BD`, derived
   `flag1 RC`, user text `T`, text rotation `BD0`, horizontal direction `BD0`, insertion scale
   `3BD_1`, insertion rotation `BD0`.
3. R2000+ attachment `BS`, line-space style `BS1`, line-space factor `BD1`, actual measurement `BD`.
4. R2007+ reserved bit `B = 0`, flip-arrow-1 `B`, flip-arrow-2 `B`; clone insertion point `2RD0`.
5. Linear body: extension-line point 1 `3BD`, extension-line point 2 `3BD`, definition point `3BD`,
   oblique angle `BD`, dimension rotation `BD0`.

For a linear dimension, the low dimension-type nibble is fixed to linear and is not an independent
native field. Derive `flag1` from the logical group-70 status bits rather than persisting it:
`(flag & 0xe0) | (flag bit 7 is clear ? 1 : 0) | (flag bit 5 is set ? 2 : 0)`; decode performs
the inverse and must reject impossible reserved combinations. The class version and reserved R2007
bit are serializer policy, not logical properties. Only `user_text` enters the string stream.

`ARC` is stable in this form from R13. `LINE` switches to the derived zero-Z plus `RD`/`DD` form at
R2000. `DIMENSION_LINEAR` gains attachment/line-spacing/measurement at R2000, flip-arrow bits at
R2007 and the class-version byte at `R_2010b`. `LWPOLYLINE` uses the predecessor-delta point vector
from R2000 and gains the conditional vertex-ID count/vector at `R_2010b`; all apply to AC1024.

### P4 fixed-support bodies

`VIEWPORT` native AC1024 main data, after its common entity prefix, is exactly:

1. center `3BD`, width `BD`, height `BD`;
2. view target `3BD`, view direction `3BD`, twist `BD`, view height `BD`, lens length `BD`, front
   clip `BD`, back clip `BD`, snap angle `BD`, view center `2RD`, snap base `2RD`, snap unit `2RD`,
   grid unit `2RD`, circle zoom `BS`, R2007+ grid major `BS`;
3. derived frozen-layer count `BL`, status flags `BL`, style-sheet `T`, render mode `RC`;
4. UCS-at-origin `B`, UCS-per-viewport `B`, UCS origin/X/Y axes `3BD`, UCS elevation `BD`,
   orthographic-view `BS`, R2004+ shade-plot mode `BS`;
5. R2007+ default-lights `B`, lighting type `RC`, brightness `BD`, contrast `BD`, ambient color `CMC`.

Its handle stream is common entity handles, frozen-layer soft references in logical order, clip
boundary, named UCS, base UCS, background, visual style, shade plot, sun. The old viewport-entity-header
reference exists only through R2002 and is absent. AC1020/R2006 alone omits snap angle/base; AC1024
includes them. `on_off` and numeric viewport `id` in LibreDWG's DXF branch are not native AC1024 body
fields. Derive the frozen-layer count and validate every optional role handle against its target type.

`MLINESTYLE` native AC1024 main/string order is name `T`, description `T`, flags `BS`, fill color
`CMC`, start angle `BD`, end angle `BD`, derived line count `RC`, then for each ordered line: offset
`BD`, color `CMC`, linetype index `BS`. Native angles are radians; only DXF converts to degrees.
R2010 uses the signed linetype index (`32767` by-layer, `32766` by-block, `0` continuous); per-line
linetype handles begin only at R2018. Its handle stream therefore contains common object handles only.
Derive the count, validate angle/color/index domains, and never materialize an R2018 handle in AC1024.

`PLACEHOLDER` has no class scalar, string, or role-handle fields. It immediately starts the object
handle stream and carries only common object owner/reactor/extension-dictionary semantics. A decoder
must still prove zero class-main bits and exact common handle consumption; a writer must not invent a
payload byte to represent the empty tagged body.

`LAYOUT` native AC1024 main/string declaration order is:

1. Plot settings: printer-config `T`, paper-size `T`, plot flags `BSx`, left/bottom/right/top margins
   `BD`, paper width/height `BD`, canonical-media-name `T`, plot origin `2BD_1`, paper unit `BS`,
   rotation mode `BS`, plot type `BS`, window lower-left `2BD_1`, window upper-right `2BD_1`.
2. Plot-view `H`; paper units `BD`, drawing units `BD`, stylesheet `T`, standard-scale type `BS`,
   standard-scale factor `BD`, paper-image origin `2BD_1`; R2004+ shade-plot type `BS`, resolution
   level `BS`, custom DPI `BS`; R2007+ shade-plot `H`.
3. Layout: name `T`, tab order `BS`, layout flags `BSx`, insertion base `3DPOINT`, limits min/max
   `2RD`, UCS origin/X/Y axes `3DPOINT`, UCS elevation `BD`, orthographic view `BS`, extents min/max
   `3DPOINT`, R2004+ derived viewport count `BL` (maximum 10,000).

For R2010 the class handle order, after common object handles, is plot-view, shade-plot, block-header,
active viewport, base UCS, named UCS, then exactly the derived viewport vector. A plot-view name is
serialized only through R2002; AC1024 serializes the reference, so a derived display name must not be
duplicated into the native string stream. The exact AC1024 string stream order is printer-config,
paper-size, canonical-media-name, stylesheet, layout-name. Derive viewport count and all compact/default
numeric encodings; require the block-header ownership and active/UCS/viewport target types to resolve.

Decoder checklist: run the common entity/object decoder first, then the exact bounded class-main reader;
read `T` values only from the independent string stream and all references only from the handle stream;
derive and cross-check every flag/count/default; reject leftover bits, invalid targets, parallel-vector
mismatches and unsupported version branches atomically. Writer checklist: materialize strings and handles
in the declaration orders above, derive compact selectors/counts from the logical body, compute the
R2010 string/footer and handle boundary once, and require same-type decode/write equality across every
fixture instance before enabling its tagged variant.

## P5/P6/P7 AC1024 high-count custom-class checklist

This checklist follows LibreDWG's R2010 branches for
[`VISUALSTYLE`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L2092-L2276),
[`AcDbEvalExpr` and block grip`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L2860-L2898),
[`AcDbAssocDependency`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L1863-L1882),
[`ASSOCVALUEDEPENDENCY`/`ASSOCGEOMDEPENDENCY`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3168-L3182),
[`AcDbAssocAction`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3089-L3125), and
[`ASSOCVARIABLE`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L5720-L5734).
Semantic names for variable fields are corroborated by Autodesk's
[`AcDbAssocVariable`](https://help.autodesk.com/cloudhelp/2018/ENU/OARX-RefGuide/files/OREF-AcDbAssocVariable.html)
API and its name/expression/evaluator/description/value/merge methods.

### `HANDLE_UNKNOWN_BITS` is not a field

LibreDWG's decoder implementation copies all remaining object bits into a debug buffer and then
resets the reader to its original position; it does not consume a prefix or describe any standard
concept. Its encoder writes that buffer and returns before the subsequent typed declarations. It is
therefore a whole-body replay bypass, not storage and not evidence of an additional field. For
`ASSOCVALUEDEPENDENCY` and `ASSOCVARIABLE`, remove `HANDLE_UNKNOWN_BITS` and every corresponding
unknown-bit schema/facet path while keeping the following typed decoder. Accept the class only when
that decoder consumes main, string and handle streams exactly for every fixture instance; otherwise
return a typed unsupported-class error. This reconciles the IFC audit: there is no offset to skip and
no opaque value to retain.

### VISUALSTYLE, dynamic type 506, 19 fixture objects

The R2010 main/string prefix is description `T`, style type `BL` (named internal style enum 0..27),
extended-lighting model `BS` (canonical R2010 default 2), and internal-only `B`. It is followed by
these 28 ordered `(typed value, BS property modifier)` pairs:

| # | Logical property | Native value type |
| ---: | --- | --- |
| 1 | face lighting model | `BL` |
| 2 | face lighting quality | `BL` |
| 3 | face color mode | `BL` |
| 4 | face modifier flags | `BS` |
| 5 | face opacity | `BD` |
| 6 | face specular amount | `BD` |
| 7 | face monochrome color | `CMC` |
| 8 | edge model | `BL` |
| 9 | edge style | `BL` |
| 10 | edge intersection color | `CMC` |
| 11 | edge obscured color | `CMC` |
| 12 | edge obscured linetype | `BL` |
| 13 | edge intersection linetype | `BL` |
| 14 | edge crease angle | `BD` |
| 15 | edge modifier flags | `BL` |
| 16 | edge color | `CMC` |
| 17 | edge opacity | `BD` |
| 18 | edge width | `BL` |
| 19 | edge overhang | `BL` |
| 20 | edge jitter | `BL` |
| 21 | silhouette color | `CMC` |
| 22 | silhouette width | `BL` |
| 23 | edge halo gap | `BL` |
| 24 | edge isoline count | `BL`, maximum 5000 |
| 25 | hidden-edge precision | `B` |
| 26 | display settings | `BL` |
| 27 | display brightness | `BD` |
| 28 | display shadow type | `BL` |

Each modifier is a named property-override/modifier value paired with its property, not a parallel
untyped integer array. Only description enters the string stream. There are no class-specific handles;
the stream contains common object owner/reactors/xdic only. Do not emit the pre-R2010 layout or the
R2013 58-property extension/`num_props`: AC1024 selects exactly the 28-pair branch. Writer derives the
R2010 extended-lighting default, validates all enums/colors/opacities and the crease/isoline ranges,
and emits every modifier immediately after its value.

### BLOCKGRIPLOCATIONCOMPONENT, dynamic type 520, 23 fixture objects

Model this as a tagged `EvalExpression` followed by a `BlockGripExpression`:

1. signed parent node ID `BLd`, evaluator major `BL`, evaluator minor `BL`, value code `BSd`;
2. exactly one tagged value selected by that code: `40 -> BD scalar`, `10 -> 2RD point`,
   `11 -> 2RD point` (LibreDWG's native prescription despite its `pt3d` member name), `1 -> T text`,
   `90 -> BL integer`, `91 -> H object reference`, `70 -> BS integer`, `-9999 -> null`;
3. node ID `BL`, grip type `BL`, grip expression `T`.

The string stream contains the value text only for code 1, then the grip expression. The handle stream
contains common object handles and, only for code 91, the evaluator value reference at that declaration
position. The code is derived from the tagged value on write, not stored beside it. Reject every other
value code; never reinterpret it as bytes. The class is a R2007+ dynamic-
block concept and has no additional R2010 gate. Parent/node IDs, major/minor, typed value, grip type and
expression are semantic; no evaluator blob is permitted.

### Shared R2010 association dependency body

Both dependency classes begin with this exact typed sequence after common object main data:

1. derived base class version `BS = 2`; status `BL`; read-dependency `B`; write-dependency `B`;
   attached-to-object `B`; delegate-to-owning-action `B`; signed evaluation order `BLd`;
2. dependent-on-object `H`; derived name-presence `B`; optional dependency name `T`;
3. dependency-chain link A `H` (`readdep` in LibreDWG), dependency-chain link B `H` (`node`),
   dependency-body reference `H`, signed dependency-body ID `BLd`.

The common object owner is the owning action. The four class references retain distinct roles and must
be type/resolution checked; they are not an unordered handle bag. Handle order after common owner/reactors/
xdic is dependent-on object, chain link A, chain link B, dependency body. Autodesk exposes previous- and
next-dependency-on-object links, but the provisional `readdep`/`node` source names do not prove which is
which. Resolve that naming by reciprocal graph checks across all instances before freezing the public
schema. The optional name is the first dependency string. Derive class version and name-presence; preserve
the logical status/flags/order/body ID.

`ACDBASSOCVALUEDEPENDENCY`, dynamic type 541, 23 fixture objects, ends immediately after that shared
body and has no additional main/string/handle fields. Its inherited optional dependency name is the
standard value name exposed by `AssocValueDependency`, not a generic/raw string. Its current LibreDWG
raw-copy macro must simply be removed as described above; the known base body is the complete candidate
layout.

`ACDBASSOCGEOMDEPENDENCY`, dynamic type 544, 31 fixture objects, then adds derived geom class version
`BS = 0`, enabled `B` (normally true), persistent-subentity class name `T`, and
dependent-on-compound-object `B`. Its string order is optional dependency name then persistent-subentity
class name; its handle order is unchanged from the base dependency. Treat the persistent-subentity name
as a named standard class discriminator, not raw source text. Reject unsupported discriminator values
until their corresponding typed subentity semantics are implemented.

### ACDBASSOCVARIABLE, dynamic type 545, 18 fixture objects

The inherited R2010 `AcDbAssocAction` prefix is:

1. derived action class version `BS = 1`; geometry/evaluation status `BL` using the named 0..6 action
   status enum; owning-network `H`; action-body `H`; action index `BL`; maximum dependency index `BL`;
2. derived dependency count `BL`; for each dependency in logical order, ownership `B` followed by its
   reference `H` (hard-owner when owned, soft-pointer otherwise).

The `class_version > 1` owned-parameter/value-parameter extension is R2013+ and must not be emitted for
AC1024. The variable body then contains derived variable class version `BL = 2`, name `T`, expression
`T`, evaluator ID `T`, description `T`, one tagged `EvalVariant`, mergeable `B`, mergeable variable
name `T`, and must-merge `B`. These names replace LibreDWG's provisional `t58`, `has_t78`, `t78` and
`b290` identifiers with the Autodesk API concepts.

`EvalVariant` is a schema tagged union keyed by its semantic result/group code: real `BD`, signed-32
`BL`, signed-16 `BS`, signed-8 `RC`, text `T`, or object reference `H`. Reuse the XRECORD/resbuf code-to-
type classifier and reject binary, object-ID, 3D-point, int64, bool, invalid and unrecognized cases
until each has a named standard encoding; never retain a payload buffer. The exact string order is
name, expression, evaluator ID, description, optional text variant, mergeable variable name. The exact
class handle order after common object handles is owning network, action body, dependency refs in order,
then an optional handle variant. Derive both counts, ownership handle codes and class-version constants;
emit the mergeable Boolean followed by the mergeable-name string exactly as prescribed, including an
empty name when the semantic value is absent.

Custom-class decoder gate: remove non-consuming raw-copy instrumentation first; run common-object decode,
then these base-before-derived bodies with separately bounded string and handle readers; validate constant
versions, count/vector equality, value discriminators, target kinds, and exact terminal fill for all same-
class fixture frames. Writer gate: emit no unknown/raw state, route every interleaved `H` to the handle
stream in declaration order, derive constants/counts/presence/handle ownership from logical fields, and
enable a class only after every instance decodes and deterministically re-encodes through the typed path.

## R2010 EED And Common Object/Entity Contract

Primary sources for this section are LibreDWG's
[`common_entity_data.spec`](https://github.com/LibreDWG/libredwg/blob/master/src/common_entity_data.spec),
[`common_entity_handle_data.spec`](https://github.com/LibreDWG/libredwg/blob/master/src/common_entity_handle_data.spec),
[`common_object_handle_data.spec`](https://github.com/LibreDWG/libredwg/blob/master/src/common_object_handle_data.spec),
and the typed EED paths in
[`decode.c`](https://github.com/LibreDWG/libredwg/blob/master/src/decode.c#L3589-L3977) and
[`encode.c`](https://github.com/LibreDWG/libredwg/blob/master/src/encode.c#L6765-L6896). The model below
retains named DWG concepts only. Lengths, flags, counts, raw color packing, stream placement and handle
encoding are all derived by the R2010 codec.

### Typed EED application records

Do not reuse `DwgXRecordValue`. EED uses a one-byte discriminator and a different, closed wire taxonomy;
XRECORD uses a two-byte DXF group code and a broader resbuf taxonomy. Define a dedicated tagged
`DwgEedValue` and change `DwgExtendedEntityData.values` to that type:

| EED code | R2010 storage after `RC code` | Logical tagged value |
| --- | --- | --- |
| 0 | `RS` UTF-16 code-unit count, then exactly that many little-endian `RS` units | `Text(String)` |
| 1 | `RS` | `ApplicationIndex(u16)` |
| 2 | `RC`, zero open/nonzero close | `GroupControl(Open | Close)` |
| 3 | `RLL` little-endian | `LayerReference(u64)` |
| 4 | `RC` length plus exactly that many octets | `BinaryChunk(Vec<u8>)`; this is named semantic EED binary content |
| 5 | `RLL_BE` | `EntityReference(u64)` |
| 10 | `3RD` | `Point([f64; 3])` |
| 11 | `3RD` | `WorldPosition([f64; 3])` |
| 12 | `3RD` | `WorldDisplacement([f64; 3])` |
| 13 | `3RD` | `WorldDirection([f64; 3])` |
| 14 | `3RD` | `ReservedPoint1014([f64; 3])` |
| 15 | `3RD` | `ReservedPoint1015([f64; 3])` |
| 40 | `RD` | `Real(f64)` |
| 41 | `RD` | `Distance(f64)` |
| 42 | `RD` | `ScaleFactor(f64)` |
| 70 | `RS` | `Integer16(i16)` |
| 71 | `RL` | `Integer32(i32)` |

Codes 14 and 15 are accepted by the native LibreDWG decoder but do not have public Autodesk XDATA
semantics; explicit reserved variants preserve their standard discriminator without a generic code/raw
escape hatch. Reject every other discriminator. A stricter public API may reject 14/15 until a primary
semantic name is found, but it must never collapse them to bytes.

The record loop is `BS data_size`; zero terminates EED; otherwise `H APPID`, followed by exactly
`data_size` bytes of ordered values. The size excludes both the `BS` and application handle. Decode in a
bounded subreader, require exact exhaustion, then continue to the next record. Resolve the application
handle to an APPID table record, layer references to LAYER records, and entity references to valid drawing
objects. Validate balanced group-control nesting per application record. On write, encode values into a
temporary byte-aligned writer first, derive `data_size`, write `BS`, APPID `H`, the value bytes, and finally
the zero `BS` terminator. R13--R2007 strings use the older `RC length + RS_BE codepage + bytes` form;
AC1024/R2010 uses only the UTF-16 form above.

Live blockers: `decode_r2010_eed` at IO lines 2292--2300 errors on the first nonzero application record;
snapshot lines 343--349 incorrectly type EED values as `DwgXRecordValue`. Geometry import lines
2981--2982 silently skip any frame with nonempty EED, and the XRECORD writer at lines 2594--2596 rejects
semantic EED instead of invoking a common writer.

### R2010 common object data and handles

Common object main-data order, immediately after object handle and EED, is:

1. reactor count `BL`;
2. extension-dictionary-missing `B`;
3. no data-store bit in AC1024 (`has_ds_data B` starts at R2013);
4. class body main/string data.

The common object handle roles are owner `H`, then exactly reactor-count reactor `H` values, then an
extension-dictionary `H` iff the missing bit is false. Control objects defer this group until their class
count fields, but retain the same role order. Logical schema remains
`owner: Option<Handle>, reactors: Vec<Handle>, extension_dictionary: Option<Handle>`; the count and missing
bit are derived. A null reactor is invalid rather than filtered. The live decoder at lines 2643--2649
currently converts null to zero and then drops it, destroying declared cardinality and positional evidence.

### R2010 common entity main data

After object handle and EED, the exact AC1024 main-data order is:

1. entity-graphic-present `B`; if true, `BLL byte_count` and exactly that many graphic bytes;
2. entity mode `BB`;
3. reactor count `BL`;
4. extension-dictionary-missing `B`;
5. encoded entity color `ENC` described below;
6. linetype scale `BD`;
7. linetype selector `BB`: ByLayer, ByBlock, Continuous, explicit reference;
8. plot-style selector `BB`: ByLayer, ByBlock, Continuous/default, explicit reference;
9. material selector `BB`;
10. shadow selector `RC`;
11. full visual-style-present `B`;
12. face visual-style-present `B`;
13. edge visual-style-present `B`;
14. invisibility `BS`;
15. lineweight `RC`;
16. class body main/string data.

There is **no** `nolinks` bit in AC1024. It exists only through R2002. The live decoder at lines
1496--1517 consumes this nonexistent bit and omits all three visual-style bits, so every affected entity's
class boundary is shifted. The live writer at lines 1462--1475 is likewise a pre-R2010-shaped subset: it
does not write the graphic gate, ENC, material/shadow/visual-style gates, or complete visibility state.

Model entity placement as `Owned(handle) | PaperSpace | ModelSpace` and derive entity mode 0, 1 or 2;
reject the unused value 3. Model graphic data with closed named variants such as a recognized WMF preview
asset or parsed proxy-graphic commands. Derive presence and size. If a present payload is not a supported
standard graphic encoding, reject the entity instead of retaining `Unknown(Vec<u8>)` or skipping it. The
live `dwg_skip_r2010_graphic` at lines 1488--1493 is therefore data loss.

### ENC color, transparency, color names and color-book semantics

Decode `BS raw` into a nine-bit ACI index and high-byte flags; do not persist either packed value. In the
main stream, flag `0x20` means a `BL alpha_raw` follows; flag `0x40` means a DBCOLOR/AcDbColor reference
will occur in the handle stream; otherwise flag `0x80` means a `BL RGB` follows. Conditions
`(flags & 0x41) == 0x41` and `(flags & 0x42) == 0x42` introduce the semantic color name and color-book name
text values. In ODA's unshifted ENC notation these correspond to the high raw masks `0x2000`, `0x4000`
and `0x8000` respectively. `alpha_raw` has an alpha-kind top byte (0 ByLayer, 1 ByBlock, 3 explicit) and
the explicit alpha in its low byte.

Use a closed `DwgEntityColor` union (`ByLayer`, `ByBlock`, `Aci(index)`, `TrueColor { rgb, name,
book_name }`, `ColorReference { handle, name, book_name }`) plus `DwgTransparency` (`ByLayer`, `ByBlock`,
`Explicit(u8)`). The book name is text, not a handle. The DBCOLOR reference is a distinct role. LibreDWG's
encoder explicitly defers this reference before all common entity handles, so its handle is first in the
handle stream. The live decoder at lines 1502--1509 discards RGB/transparency and lines 2657--2659 call
the DBCOLOR reference `color_book`; both must be replaced. Version-aware TV/TU string decoding must be
routed through the R2010 string-stream abstraction at the declaration point rather than treated as raw
text bytes.

### R2010 common entity handle order

The exact handle sequence is:

1. DBCOLOR/AcDbColor reference iff ENC requests one;
2. owner iff entity mode is zero;
3. exactly reactor-count reactor handles;
4. extension dictionary iff present;
5. layer, always;
6. linetype iff its selector is explicit;
7. material iff its selector is explicit;
8. shadow iff its selector is explicit/value 3;
9. plot style iff its selector is explicit;
10. full visual style iff present;
11. face visual style iff present;
12. edge visual style iff present;
13. class-specific handles.

There are no previous/next-entity handles in AC1024. Preserve these roles as typed fields rather than the
live generic `referenced_handles` bag. The source comment labels shadow values 0/1/2 as casts-and-receives,
receives, and casts, while value 3 gates an explicit shadow handle; represent value 3 as `Reference(handle)`
and validate the referenced object's class before assigning a stronger public name. The live relations
decoder at lines 2652--2679 currently puts only layer/ltype/material/plot-style into a flat vector and
omits DBCOLOR, shadow and all visual styles. It also drops null reactor slots. Lines 2711--2716 silently
ignore an entity-common decode error and still publish the partial object; lines 2721--2728 do the same for
non-XRECORD object-common failures. Both paths must reject the frame atomically.

### Version gates and fixture invariants

- Graphic byte count is `RL` through R2007 and `BLL` from R2010b.
- Extension-dictionary-missing is R2004a+; entity/object data-store is R2013+ and absent from AC1024.
- `nolinks` and previous/next entity handles end at R2002; they are absent from AC1024.
- ENC is R2004a+; linetype/plot-style selectors and lineweight are R2000b+; material/shadow are R2007a+;
  all three visual-style gates/handles are R2010b+.
- EED uses the repeated `BS size + H APPID` form from R13; strings are Unicode from R2007a.

For every one of the fixture's 652 framed objects, require object handle equality with the handle map,
exact EED record exhaustion and terminal zero, resolvable APPID/layer/entity references, and no residual
EED bytes. For every entity, log graphic presence/type/derived size, common-main end bit, entity placement,
reactor count, xdic presence, color source/transparency, selectors, visual-style gates, and the ordered
typed handle roles. Assert reactor-vector cardinality, owner/entity-mode equivalence, xdic flag/handle
equivalence, RGB versus DBCOLOR exclusivity, color-name/book conditions, selector/handle equivalence,
visual-style gate/handle equivalence, and exact transition to class body. Reject any unsupported graphic,
EED discriminator, invalid handle target, leftover common bits, leftover common handles or partial decode.
The writer must derive all gates/counts/sizes/packed selectors and reproduce the same typed field and
handle order; no preview skip, raw EED, generic handle bag or error-swallowing path is eligible for exact
roundtrip acceptance.

## AC1024 DICTIONARY And ACDBDICTIONARYWDFLT Checklist

Primary references are ODA sections 20.4.44--20.4.45 in the
[`Open Design Specification for .dwg files`](https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf),
LibreDWG's [`DICTIONARY` and `DICTIONARYWDFLT` prescriptions](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec#L2609-L2748),
Autodesk's [`DICTIONARY` DXF contract](https://help.autodesk.com/cloudhelp/2015/ENU/AutoCAD-DXF/files/GUID-40B92C63-26F0-485B-A9C2-B349099B26D0.htm),
and Autodesk's
[`ACDBDICTIONARYWDFLT` contract](https://help.autodesk.com/cloudhelp/2020/ENU/AutoCAD-DXF/files/GUID-A6605C05-1CF4-42A4-95EC-42190B2424EE.htm).
The fixture inventory proves 83 fixed type-42 `DICTIONARY` frames and one dynamic type-500
`ACDBDICTIONARYWDFLT` frame. Existing logs do not yet contain trustworthy per-frame item counts, so those
counts must be emitted by the first strict decoder run rather than inferred.

### Logical schema

Use one dictionary base value with ordered, unique `DwgDictionaryEntry { name: String, object: Handle }`,
`hard_owner: bool`, and a tagged duplicate-record-cloning policy. Replace the live raw `u16 cloning_flag`
with the closed standard enum: `NotApplicable = 0`, `KeepExisting = 1`, `UseClone = 2`,
`XrefMangleName = 3`, `MangleName = 4`, `UnmangleName = 5`; reject every other `BS`. The count is derived.
`ACDBDICTIONARYWDFLT` extends the base with `default_object: Option<Handle>` and must be a distinct tagged
body or validated from its class kind. A plain type-42 dictionary must never carry a default.

The hard-owner flag is semantic: when true, dictionary elements are treated as hard-owned. Native DWG
still prescribes the dictionary item handle vector generically as `H`/soft-owner references; do not try to
persist or infer ownership from a compact wire handle code. The first handle nibble encodes absolute or
relative handle compression, not soft/hard ownership. Choose that nibble deterministically from
`current_handle` and target (`+1`, `-1`, positive delta, negative delta, otherwise absolute/null), while
the dictionary field role supplies ownership semantics.

Autodesk identifies the WDFLT default as a hard pointer, DXF group 340, and says the class is currently
used for the plot-style dictionary's default entry named `Normal`. ODA prints group 7 for this field,
where LibreDWG and Autodesk use 340; that is a DXF/interchange-label disagreement, not a native DWG wire
layout difference. The native field is one `H` hard-pointer role after the item vector. For this fixture,
verify rather than assume that the sole WDFLT default target equals the handle paired with `Normal`.

### Exact AC1024 decode order

Frame prefix and main stream:

1. outer object `MS payload_byte_count`, `UMC handle_stream_bit_count`, then byte alignment;
2. class `BOT`: fixed 42 for `DICTIONARY`, dynamic 500 for this fixture's first class
   `ACDBDICTIONARYWDFLT`;
3. object handle `H`, then the complete EED record sequence and its zero terminator;
4. common object main data: reactor count `BL`, extension-dictionary-missing `B`; no R2013 data-store bit;
5. item count `BL`;
6. duplicate-record-cloning policy `BS`;
7. hard-owner flag `RC`, constrained to zero or one;
8. end of class main data.

R2010 string stream, in dictionary entry order:

9. exactly item-count `T` values, each materialized as a `BS` UTF-16-unit count followed by those
   little-endian `RS` units;
10. string-stream bit count (`RS`, or high `RS` plus flagged low `RS` when over 0x7fff bits), then the
    string-present `B = 1` marker. An empty dictionary has no semantic names; follow the format-wide
    R2010 policy for an absent (`B = 0`) rather than empty-present string stream.

Handle stream:

11. common owner `H`;
12. exactly common reactor-count reactor `H` values;
13. extension-dictionary `H` iff the missing bit was false;
14. exactly item-count entry-object `H` values, zipped positionally with the names;
15. for WDFLT only, exactly one default hard-pointer-role `H` (the handle value may be null only if the
    logical default is absent; the fixture must prove which case it contains);
16. serializer terminal fill only; no further semantic handles.

The name vector and item-handle vector are parallel. Preserve their paired order; never sort one without
the other. Resolve every entry target and validate hard-owner graph semantics. Reject null entry targets,
duplicate dictionary keys under the project's AutoCAD key comparison, invalid UTF-16, count overflow,
unknown cloning policy, a non-Boolean hard-owner byte, or a WDFLT default that does not resolve. When a
default exists, require it to equal one entry target; for the standard plot-style use, also require that
entry's name to be `Normal`.

### Version gates

- `numitems` and paired text/handle vectors are common dictionary fields.
- ODA describes the R14-only byte at this position as an unknown zero. LibreDWG treats the post-R13c3
  field as `is_hardowner`, except for the earliest R13c3 maintenance revision. This disagreement is
  irrelevant to AC1024: R2000+ unambiguously carries cloning `BS` followed by hard-owner `RC`.
- R2007+ `T` values use the independent Unicode string stream; AC1024 must not decode names inline in the
  main stream.
- R2004+ common objects carry extension-dictionary-missing `B`; R2013+ alone adds data-store `B`.
- WDFLT is a dynamic `AcDbDictionaryWithDefault` class in the classes section. Its base layout is identical
  to DICTIONARY, followed by the one default handle; its class number is derived from class-list position,
  not hard-coded generally. It happens to be dynamic 500 in this fixture.

### Deterministic writer and derived padding

Construct separate main, string and handle bit writers. Emit BOT/handle/EED/common main/base dictionary
fields; encode names into the string writer; append its bits and the canonical short/extended size footer
plus present bit; then append common handles, item handles, and WDFLT default in the order above. Derive
item count, reactor count, xdic-missing, string size, handle-stream size, payload byte count, compact
relative handle forms and CRC. Persist none of them.

The handle stream begins immediately after the string-present marker, without byte alignment between
streams. After the last semantic handle, add exactly `0..=7` one-bits required to make the payload
byte-aligned, using the same format-wide R2010 terminal-fill policy already proved for fixture XRECORDs.
Include those fill bits in the outer `UMC handle_stream_bit_count`. Then emit the outer prefix, payload,
and CRC16 using the same coverage/seed as the accepted R2010 frame writer. Empty names, zero entries and a
null optional default do not authorize arbitrary extra bytes or handles; every pad bit is derived solely
from final bit position.

### Live implementation comparison and concrete corrections

The live decoder at IO lines 2858--2869 has the correct AC1024 base field order, reads names from the
R2010 string stream, reads item handles after common object handles, and places the WDFLT default after
the item vector. Keep that spine. Correct these remaining issues:

1. `DwgDictionaryBody` snapshot lines 305--314 stores cloning as raw `u16` and permits a default on an
   ordinary dictionary; introduce the typed policy and enforce the class-tagged default extension.
2. `r2010_string_stream(...).ok()` at IO line 2854 discards the original boundary error. Dictionary decode
   must propagate it with handle/class context.
3. After reading hard-owner, require `data.bit_position() == main_end_bit`. After names, require exact
   string-stream exhaustion. After item/default handles, accept only the derived `0..=7` one-bit terminal
   fill and require exact payload exhaustion. The current dictionary branch performs none of these checks.
4. Validate cloning range, hard-owner byte, unique names, non-null/resolved item targets, count/vector
   equality, default membership and referenced-object ownership. Current code checks only null item
   handles.
5. Do not flatten dictionary/common decode errors into partial objects. The common-object fallback at IO
   lines 2842--2849 can still publish an untyped dictionary after a failed common decode.
6. There is no DICTIONARY/WDFLT frame encoder or exact verifier; only XRECORD has
   `encode_r2010_xrecord_frame`/`verify_r2004_xrecord_frames`. Factor the accepted common R2010 frame
   assembler, add typed dictionary main/string/handle emission, and compare each reconstructed frame
   directly to its fixture frame before integrating it into the object-section writer.

### Fixture gate: 84 dictionary frames

The strict diagnostic must output one tuple per frame:
`(handle, class, owner, reactors, xdic, item_count, cloning, hard_owner, names, item_targets,
default_target, main_end_bit, string_end_bit, handle_end_bit, terminal_fill_bits)`. Acceptance requires:

- exactly 83 fixed type-42 tuples and exactly one dynamic type-500 WDFLT tuple;
- every declared count equals its decoded vector cardinality and every stream boundary is exact;
- all 84 owner/entry/default targets resolve and the ownership graph is consistent;
- every name is represented once and zipped with exactly one target;
- the WDFLT frame has exactly one default-handle field after its item vector and no plain dictionary does;
- terminal fill is the unique derived all-ones suffix of at most seven bits;
- decode then typed encode reproduces all 84 original framed byte slices, including outer sizes and CRCs;
- mutation plus inverse restores the same 84 logical bodies and exact native frame bytes without retaining
  imported frame bytes, offsets, counts, string footer, compact handle codes, padding or CRC state.

## P8 — AC1024 Layout, Block Graph, Placeholder, Group, Mline Style and Custom-Object Handoff

This section is the code-ready handoff for the remaining fixed support objects and their dependency on the
high-count custom-object common prefix. It reconciles LibreDWG `dwg.spec`/`dwg2.spec` with the ODA R2010
object layouts and the fixture inventories. Physical BOT, compact handle encodings, stream sizes, terminal
fill, frame sizes and CRCs remain serializer-derived and must never enter the snapshot.

Primary references:

- LibreDWG [`dwg.spec`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec): `BLOCK_HEADER`,
  `GROUP`, `MLINESTYLE` and `LAYOUT` declarations.
- LibreDWG [`dwg2.spec`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec):
  `ACDBPLACEHOLDER` and dynamic class declarations.
- ODA [Open Design Specification for .dwg files](https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf):
  common object data/handles, block header and layout object sections.

### Fixture census and acceptance boundary

| Object | Wire type/class | Fixture count | Byte gate |
| --- | ---: | ---: | --- |
| `BLOCK_CONTROL` | fixed 48 | 1 | required with the block-header set |
| `BLOCK_HEADER` | fixed 49 | 10 | required, including graph handles |
| `BLOCK` / `ENDBLK` | fixed 4 / 5 | 10 / 10 | required as reciprocal graph markers |
| `INSERT` | fixed 7 | 12 | required as block-header backlinks |
| `GROUP` | fixed 72 | 0 | specification/schema law only; not fixture-byte evidence |
| `MLINESTYLE` | fixed 73 | 1 | required |
| `ACDBPLACEHOLDER` | fixed 80 | 1 | required |
| `LAYOUT` | fixed 82 | 2 | required with reciprocal block-layout links |
| `VISUALSTYLE` | dynamic 506 | 19 | common prefix plus typed class body |
| `BLOCKGRIPLOCATIONCOMPONENT` | dynamic 520 | 23 | common prefix plus typed class body |
| `ACDBASSOCVALUEDEPENDENCY` | dynamic 541 | 23 | common prefix plus typed class body |
| `ACDBASSOCGEOMDEPENDENCY` | dynamic 544 | 31 | common prefix plus typed class body |
| `ACDBASSOCVARIABLE` | dynamic 545 | 18 | common prefix plus typed class body |

The five custom classes contribute 114 frames. Counts are fixture observations, not schema fields.

### Shared AC1024 stream contract

Every object below starts with the R2010 framed-object envelope, object handle and typed EED sequence. For
an ordinary object, class main data then starts with reactor count `BL` and extension-dictionary-missing
`B`; AC1024 has no R2013 data-store bit. The handle stream always begins with owner, reactor vector, and
optional extension dictionary in that order. Table records additionally use the already-audited common
table-record prefix and its xref handle role. Text `T` fields are written to the independent R2007+
UTF-16 string stream in declaration order, never inline in the main stream.

For every decoder branch require exact exhaustion of main data, string data and semantic handles. The
only permitted residue is the uniquely derived all-one terminal fill of at most seven bits. For every
writer derive string footer, stream bit counts, relative handle form, frame length and CRC from typed
values. A named known object with zero class fields is still a tagged body, not `None`.

### `LAYOUT` (fixed type 82, two fixture frames)

Use a typed `DwgLayoutBody { plot_settings, name, tab_order, flags, insbase, limmin, limmax, ucs_origin,
ucs_x_axis, ucs_y_axis, ucs_elevation, ucs_orthoview, extmin, extmax, plot_view, visual_style,
block_header, active_viewport, base_ucs, named_ucs, viewports }`. `plot_settings` must itself be typed; raw
flag words and unnamed strings are insufficient.

Main-data order for AC1024:

1. page-setup name `T`;
2. printer/configuration name `T`;
3. plot flags `BS` bitset;
4. left, bottom, right and top margins, four `BD`;
5. paper width and height, two `BD`;
6. canonical-media name `T`;
7. plot origin `2BD_1`;
8. paper-units `BS`, rotation `BS`, plot-type `BS`;
9. plot-window lower-left and upper-right, two `2BD_1`;
10. R2000+ plot-view handle presence is represented in the handle stream; the pre-R2002 plot-view-name
    text variant is absent in AC1024;
11. custom-scale numerator/paper units `BD`, denominator/drawing units `BD`;
12. style-sheet name `T`;
13. standard-scale type `BS`, standard-scale factor `BD`;
14. paper-image origin `2BD_1`;
15. R2004+ shade-plot mode `BS`, shade-plot resolution `BS`, custom DPI `BS`;
16. layout name `T`;
17. tab order `BS`; LibreDWG's AC1024 declaration is authoritative here despite the ODA prose/table
    showing `BL` in some editions;
18. layout flags `BS` bitset;
19. insertion base `3BD`, limits minimum/maximum `2RD`, UCS origin/x/y `3BD`, UCS elevation `BD`, and
    orthographic-UCS view `BS`;
20. extents minimum/maximum `3BD`;
21. R2004+ viewport count `BL`, derived from the viewport handle vector.

The string-stream order is therefore exactly page setup, printer/configuration, canonical media, style
sheet, layout name. Do not preserve imported XML-like spelling or string-stream placement state.

Handle order after common object handles:

1. R2004+ plot-view hard pointer;
2. R2007+ visual-style/shade-plot soft pointer;
3. associated block-header soft pointer;
4. active-viewport soft pointer;
5. base-UCS hard pointer;
6. named-UCS hard pointer;
7. R2004+ ordered viewport soft-pointer vector.

Version gates: LAYOUT exists from R2000; use the handle rather than pre-R2002 plot-view text; shade
settings and the viewport vector are R2004+; visual style is R2007+; no new layout-local R2010 field is
inserted. Model paper unit, rotation, plot type, standard scale, shade plot, resolution and orthographic
view as closed enums, flags as typed bitsets, and every geometric value as finite. Counts are derived.

Graph invariants: each of the two fixture layouts resolves to exactly one block header; that header's
layout handle points back. A non-null active viewport resolves and is consistent with the layout's
viewport set. Base/named UCS and visual-style targets must match their declared roles. Reject duplicate
viewport handles and count mismatch.

### `BLOCK_HEADER` graph (fixed type 49, ten fixture frames)

Reuse `DwgTableRecordCommon` for name/table flags/xref semantics, then model a dedicated
`DwgBlockHeaderBody { anonymous, has_attributes, is_xref, xref_overlaid, xref_loaded, base_point,
xref_path, description, preview, insert_units, explodable, scaling, block_entity, owned_entities,
end_block, inserts, layout }`. Do not duplicate the table-record name in this body.

Main/string-data order after the common table-record prefix:

1. anonymous `B`;
2. has-attributes `B`;
3. is-xref `B`;
4. xref-overlaid `B`;
5. R2000+ xref-loaded `B`;
6. R2004+ owned-entity count `BL` only when neither xref nor overlay is set;
7. base point `3BD`;
8. xref path `T`;
9. R2000+ insert-count sentinel sequence: one nonzero `RC` per insert reference, followed by zero `RC`;
   the nonzero byte values carry no semantic state and the canonical writer emits one for each item;
10. description `T`;
11. preview byte count `BL` and typed preview content;
12. R2007+ insertion-units `BS`, explodable `B`, block-scaling `RC`.

The two block-local strings are xref path then description; the common table-record name precedes them in
the shared string declaration. Preview bytes are semantic only when decoded as the specified preview
image concept; an unknown opaque preview bag is not acceptable.

Handle order after common object/table handles, including the common table xref role when present:

1. `BLOCK` begin-marker hard owner;
2. R2004+ ordered owned-entity hard-owner vector when not xref/overlay;
3. `ENDBLK` end-marker hard owner;
4. ordered INSERT soft-pointer backlink vector, cardinality derived from the sentinel sequence;
5. layout hard pointer.

AC1024 has no pre-R2004 first/last-entity pair. The owned-vector field is absent, not empty-present, for
xref/overlay headers. Insertion units and scaling are enums; all five Boolean fields are true Booleans.
Counts and sentinel bytes are writer-derived.

Required fixture graph checks:

- the 10 headers biject with 10 `BLOCK` and 10 `ENDBLK` markers;
- each marker's owner points back to the same header;
- the one `BLOCK_CONTROL` references the complete header set according to the control's ordinary and
  model/paper-space roles, with no duplicate or missing header;
- all 12 `INSERT.block_header` targets resolve and every insert appears exactly once in its target
  header's backlink vector;
- non-xref owned-entity vectors partition their block-owned entities without duplicates or cycles;
- each of the two layout/header pairs is reciprocal;
- xref/overlay headers do not carry an owned count/vector;
- preview length is derived and every handle role resolves to its expected object category.

### `ACDBPLACEHOLDER` (fixed type 80, one fixture frame)

This class has no class-local main fields, strings or handles. After EED/common object main data, switch to
the handle stream and emit only common owner/reactors/extension dictionary. Represent it as an explicit
zero-sized `DwgLogicalObjectBody::Placeholder`, because `body = None` means unsupported or undecoded and
cannot distinguish a successfully decoded empty standard object. Exact acceptance requires zero residue
in every semantic stream and no invented marker byte, count, handle or raw payload.

### `GROUP` (fixed type 72, absent from this fixture)

Use `DwgGroupBody { description, unnamed, selectable, members }`. The first `T` is the description, not
the dictionary key under which the group is named. The key remains a DICTIONARY entry concept.

Field order after common object main data:

1. description `T`;
2. unnamed `BS` constrained to Boolean semantics;
3. selectable `BS` constrained to Boolean semantics;
4. member count `BL`, derived from the ordered member vector;
5. after common handles, ordered member entity handles (`340`/hard-pointer role in the object grammar).

Reject more than 10,000 members, null/unresolved members and non-Boolean flag values. Preserve semantic
member order. Because the fixture count is zero, add specification-vector and generated lifecycle tests,
but do not claim native fixture byte equality for GROUP.

### `MLINESTYLE` (fixed type 73, one fixture frame)

Use `DwgMLineStyleBody { name, description, flags, fill_color, start_angle, end_angle, lines }`; each line
is `DwgMLineStyleLine { offset, color, linetype }`. AC1024 field order after common object main data is:

1. name `T`;
2. description `T`;
3. style flags `BS`;
4. fill color `CMC`;
5. start angle `BD`;
6. end angle `BD`;
7. line count `RC`, derived from the line vector;
8. for each line: offset `BD`, color `CMC`, linetype index `BS`.

There are no class-local handles in AC1024; only common object handles follow. R2018 replaces each line's
linetype index with a handle, but that gate must not activate for AC1024. Model special indices as named
variants (`ByLayer = 32767`, `ByBlock = 32766`, `Continuous = 0`) and any permitted table index as a
validated signed value. Model CMC as the typed DWG color concept and flags as named bits: fill, display
miters, and square/inner-arc/round start/end caps. Reject reserved flag bits, excessive line count and
non-finite angles/offsets. Preserve line order and angles in radians without normalization.

### High-count custom-object common-prefix bridge

The existing P4/P5-P7 class-specific layouts remain authoritative for dynamic types 506, 520, 541, 544
and 545. Their implementation must use the same typed common object prefix and frame assembler as the
fixed objects above:

1. object handle, then complete typed EED and terminator;
2. reactor count `BL`, xdic-missing `B`, no R2013 data-store bit;
3. class-specific main and string fields in the declared `dwg2.spec` order;
4. common owner, reactors and optional xdic in the handle stream;
5. class-specific handles in their declared semantic roles;
6. uniquely derived terminal fill, outer sizes and CRC.

Do not expose `HANDLE_UNKNOWN_BITS`, a generic referenced-handle bag, unknown class bytes, or skipped
tail bits as storage. Add tagged variants for `VisualStyle`, `BlockGripLocationComponent`,
`AssocValueDependency`, `AssocGeomDependency` and `AssocVariable`; represent presence flags, enum/version
discriminants, values and every handle role explicitly. The 19/23/23/31/18 fixture populations must each
have class-wide main/string/handle exhaustion and target-resolution gates before exact writer claims.

### Live-code corrections and implementation order

The live snapshot's `DwgLogicalObjectBody` at `schema/snapshot/component.rs:353` still has only
Dictionary, TableControl, TableRecord and XRecord. The IO mapping at `io/component.rs:2430-2459` names
the fixed types but does not decode their typed bodies, and the only typed frame writer remains
`encode_r2010_xrecord_frame` at `io/component.rs:2740`. Consequently these known objects are still
identity-only/body-less and cannot participate in a logical exact roundtrip.

Implement in this dependency order:

1. factor the accepted XRECORD framing into one R2010 common object/table frame assembler;
2. add explicit `Placeholder` and `MLineStyle` bodies to prove empty and no-class-handle branches;
3. add `BlockHeader` together with BLOCK/ENDBLK/INSERT and BLOCK_CONTROL graph validation;
4. add `Layout` and enforce reciprocal block-header linkage;
5. add `Group` with specification-vector coverage despite zero fixture instances;
6. route the five high-count custom variants through the same common prefix/boundary verifier;
7. propagate every tagged body through snapshot DSL/binary, diff, mutation, artifact and language facets;
8. for each populated class, require decode/encode frame equality, mutation plus inverse equality, exact
   stream exhaustion, resolved handles and absence of physical/raw shadow state.

## P9 — DICTIONARY `0x0c` Handle-Stream Mismatch Diagnosis

The failing frame had an exact main/string prefix, original length 541 versus encoded length 524, and
original/encoded handle-stream widths 278/142 bits. This was not an undiscovered dictionary field. The
original raw handle sequence is:

| Position | Raw token | Absolute value | Semantic role |
| ---: | --- | ---: | --- |
| 0 | code `4`, zero payload bytes | null | owner |
| 1 | code `2`, two payload bytes | `0x1bca` | dictionary entry 0 |
| 2 | code `2`, one payload byte | `0x73` | dictionary entry 1 |
| 3 | code `2`, one payload byte | `0x0d` | dictionary entry 2 |
| 4 | code `2`, one payload byte | `0x1a` | dictionary entry 3 |
| 5 | code `2`, one payload byte | `0x72` | dictionary entry 4 |
| 6 | code `2`, one payload byte | `0xd7` | dictionary entry 5 |
| 7 | code `2`, one payload byte | `0x17` | dictionary entry 6 |
| 8 | code `2`, one payload byte | `0x19` | dictionary entry 7 |
| 9 | code `2`, one payload byte | `0x0e` | dictionary entry 8 |
| 10 | code `2`, one payload byte | `0xb6` | dictionary entry 9 |
| 11 | code `2`, one payload byte | `0x86` | dictionary entry 10 |
| 12 | code `2`, one payload byte | `0x99` | dictionary entry 11 |
| 13 | code `2`, one payload byte | `0x66` | dictionary entry 12 |
| 14 | code `2`, two payload bytes | `0x0adb` | dictionary entry 13 |
| 15 | code `2`, two payload bytes | `0x1084` | dictionary entry 14 |

There are no reactors, the extension dictionary is absent, and six terminal one-bits follow. The exact
width is `8 + 3 * 24 + 12 * 16 + 6 = 278` bits.

The broken encoded sequence was owner `(code 12, one byte, value 0x0c)`, fifteen `(code 2, zero bytes,
value 0)` tokens, and the same six fill bits: `16 + 15 * 8 + 6 = 142`. Thus all 16 semantic roles were
present, but the owner and all 15 item values were corrupted. Codes `2` and `4` are absolute handle forms;
only codes `6`, `8`, `10` and `12` are relative to the containing object's handle. Code `4` value zero is
the null owner. The minimal correction is to apply that resolver split and make the dictionary writer emit
the role-prescribed absolute code `2` for item references and code `4`/zero for its null owner, rather than
generic relative compaction. Terminal fill remains the uniquely derived six one-bits. After the resolver
correction the `0x0c` frame passed and verification advanced to later dictionary frames.

## P10 — Explicit 364/652 Identity-Only Inventory and Research Partition

The red baseline is reproducible from the current typed branches. Exactly 288 frames receive a body:
84 DICTIONARY/WDFLT, 145 XRECORD, nine table controls and 50 table records. The remaining
`652 - 288 = 364` frames are identity-only. The 50 current table-record bodies are name-only and are not
therefore complete logical acceptance, but they are intentionally outside this identity-only census.

### Partition summary

| Research state | Frames | Meaning |
| --- | ---: | --- |
| Code-ready field/stream/handle research already recorded | 250 | Dedicated AC1024 checklist exists; implementation and fixture proof remain |
| Cohort-level prescription only | 39 | Family is identified, but exhaustive class-specific field order/gates are still required |
| Missing complete class-specific schema research | 75 | Only inventory/base-family knowledge exists |
| **Identity-only total** | **364** | Exact current red baseline |

### Code-ready researched identity-only frames — 250

| Type/class | Count | Existing report authority |
| --- | ---: | --- |
| 4 `BLOCK` | 10 | P2a block ownership graph; P8 block-header graph |
| 5 `ENDBLK` | 10 | P2a block ownership graph; P8 block-header graph |
| 7 `INSERT` | 12 | P2a block ownership graph; P8 backlink invariants |
| 17 `ARC` | 12 | P3 geometry bodies |
| 19 `LINE` | 40 | P3 geometry bodies |
| 21 `DIMENSION_LINEAR` | 12 | P3 geometry bodies |
| 34 `VIEWPORT` | 2 | P4 fixed-support bodies |
| 73 `MLINESTYLE` | 1 | P4 fixed-support bodies; P8 dedicated checklist |
| 77 `LWPOLYLINE` | 16 | P3 geometry bodies |
| 80 `ACDBPLACEHOLDER` | 1 | P4 fixed-support bodies; P8 dedicated checklist |
| 82 `LAYOUT` | 2 | P4 fixed-support bodies; P8 dedicated checklist |
| 506 `VISUALSTYLE` | 19 | P5/P6/P7 dedicated 28-property checklist |
| 520 `BLOCKGRIPLOCATIONCOMPONENT` | 23 | P5/P6/P7 dedicated EvalExpression checklist |
| 541 `ACDBASSOCVALUEDEPENDENCY` | 23 | P5/P6/P7 shared dependency body |
| 542 `ACDBASSOCDEPENDENCY` | 18 | P5/P6/P7 shared dependency body itself |
| 544 `ACDBASSOCGEOMDEPENDENCY` | 31 | P5/P6/P7 dependency plus geometry suffix |
| 545 `ACDBASSOCVARIABLE` | 18 | P5/P6/P7 dedicated action/variable checklist |
| **Subtotal** | **250** | |

### Cohort-level research, exhaustive class checklist still missing — 39

| Type/class | Count | Existing evidence and missing work |
| --- | ---: | --- |
| 503 `DICTIONARYVAR` | 8 | P1 identifies schema byte plus Unicode value; still needs dedicated main/string/handle/version/exhaustion checklist |
| 504 `TABLESTYLE` | 1 | P5 style/context cohort only; extract every R2010 style/cell/handle field |
| 505 `MATERIAL` | 3 | P5 style/context cohort only; extract complete mapper/procedural/color/transparency handle gates |
| 507 `SCALE` | 17 | P5 identifies name, paper/drawing units and scale flag; needs exhaustive boundary/version proof |
| 508 `MLEADERSTYLE` | 1 | P5 style/context cohort only; needs complete R2010 field and handle order |
| 516 `SORTENTSTABLE` | 7 | P5 identifies ordered entity/sort pairs; needs complete owner/block/count/version order |
| 517 `ACAD_EVALUATION_GRAPH` | 2 | P5 identifies typed nodes/edges; needs exhaustive node/edge variant and graph-handle layout |
| **Subtotal** | **39** | |

### Missing complete class-specific schema research — 75

Dynamic-block remainder, 48 frames:

| Type/class | Count |
| --- | ---: |
| 521 `BLOCKMOVEACTION` | 2 |
| 522 `ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION` | 2 |
| 527 `BLOCKLINEARPARAMETER` | 2 |
| 528 `BLOCKLINEARGRIP` | 4 |
| 529 `BLOCKFLIPPARAMETER` | 3 |
| 530 `BLOCKFLIPGRIP` | 3 |
| 531 `BLOCKVISIBILITYPARAMETER` | 1 |
| 532 `BLOCKVISIBILITYGRIP` | 1 |
| 533 `BLOCKALIGNMENTPARAMETER` | 2 |
| 534 `BLOCKALIGNMENTGRIP` | 2 |
| 535 `BLOCKSTRETCHACTION` | 6 |
| 536 `BLOCKSCALEACTION` | 1 |
| 537 `BLOCKFLIPACTION` | 3 |
| 538 `BLOCKBASEPOINTPARAMETER` | 1 |
| 546 `BLOCKVERTICALCONSTRAINTPARAMETER` | 1 |
| 547 `ACDB_DYNAMICBLOCKPROXYNODE` | 1 |
| 548 `BLOCKHORIZONTALCONSTRAINTPARAMETER` | 1 |
| 559 `ACDB_BLOCKREPRESENTATION_DATA` | 12 |
| **Dynamic-block subtotal** | **48** |

Associative remainder, 27 frames:

| Type/class | Count |
| --- | ---: |
| 539 `ACDBASSOCNETWORK` | 5 |
| 540 `ACDBASSOC2DCONSTRAINTGROUP` | 4 |
| 543 `BLOCKPARAMDEPENDENCYBODY` | 6 |
| 549 `ASSOCDIMDEPENDENCYBODY` | 12 |
| **Associative subtotal** | **27** |

The two missing-research subtotals are `48 + 27 = 75`. Every dynamic type number above is this
fixture's class-list assignment; the schema discriminator is the standard class name, not the numeric
slot.

### Immediate implementation/research queue requested by the red baseline

1. `ACDBPLACEHOLDER` 1 — explicit zero-sized tagged body; P8 is code-ready.
2. `MLINESTYLE` 1 — typed style/line vector and no class-local AC1024 handles; P8 is code-ready.
3. `BLOCK` 10 and `ENDBLK` 10 — marker bodies plus reciprocal header owners; P2a/P8 are code-ready.
4. `LAYOUT` 2 — typed plot settings and reciprocal block-header/UCS/viewport graph; P8 is code-ready.
5. `DICTIONARYVAR` 8 — first remaining P1 spine class; promote its cohort note into an exhaustive
   checklist before implementation.
6. `TABLESTYLE` 1 — extract a complete R2010 class prescription; current P5 entry is not sufficient.
7. `MATERIAL` 3 — extract the complete material mapper/procedural/color prescription; current P5 entry
   is not sufficient.
8. `VISUALSTYLE` 19 — the dedicated 28-property P5/P6/P7 checklist is code-ready and offers the largest
   immediate custom-style reduction.

This queue covers 55 of the 364 identity-only frames. Implementing it leaves 309 identity-only frames,
before considering any failures caused by incomplete name-only table-record bodies.

## P11 — Complete Typed Research for the Former 75-Frame Gap

This section supersedes P10's “missing complete class-specific schema research” label. All 75 frames now
have closed typed candidate layouts. The implementation must still prove every candidate against the
bounded fixture streams before enabling serialization; LibreDWG marks several classes unstable and ODA's
public DWG specification defines framing/common objects but not these private dynamic-class payloads.
Failure of a candidate is a typed unsupported-class error, never permission to retain raw bits.

Primary sources are LibreDWG [`dwg2.spec`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec)
for the R2010 declarations and [`dwg.h`](https://github.com/LibreDWG/libredwg/blob/master/include/dwg.h)
for the corresponding typed records, checked against the ODA
[Open Design Specification for .dwg files](https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf)
for the shared AC1024 object/string/handle framing contract. The fixture probes establish the exact class
populations listed below. `HANDLE_UNKNOWN_BITS` remains excluded: it is LibreDWG debug replay, not a field.

### Shared typed bases and stream routing

Every body follows common AC1024 object main data and common owner/reactor/xdic handles. All `T` values
below enter the independent UTF-16 string stream in declaration order. Every `H` enters the handle stream
in declaration order even when interleaved with main scalars. Counts, presence flags, Eval discriminators,
string footer, handle codes, terminal fill, outer sizes and CRC are serializer-derived.

`DwgEvalExpr` main order:

1. signed parent node ID `BLd`;
2. evaluator major `BL`, evaluator minor `BL`;
3. signed tagged-value code `BSd`;
4. one value selected by code: `40 => BD`, `10 => 2RD`, `11 => 2RD`, `1 => T`, `90 => BL`,
   `91 => H`, `70 => BS`, `-9999 => None`;
5. node ID `BL`.

Use a tagged union and derive the code. Code 1 contributes the next string; code 91 contributes the next
handle. No other code is accepted.

`DwgBlockElement` extends EvalExpr with element name `T`, element major `BL`, element minor `BL`, and
semantic application value `BL` (DXF 1071). For R2007+ the expected major/minor family is 33/29, but
decode the declared values and validate them class-wide rather than persisting a version flag.

`DwgBlockGrip` extends BlockElement with two typed grip-state integers `BL`, location `3BD`, insert-cycling
`B`, and signed cycling weight `BLd`. Give the state fields named enum/newtype roles after correlating their
fixture values; they are scalar standard fields, not raw bytes.

`DwgBlockParameter` extends BlockElement with `show_properties B` and `chain_actions B`.

`DwgBlockOnePointParameter` then contains default point `3BD`, property-info 1 and 2, then the declared
property-info count `BL`. Each property info is derived connection count `BL` followed by that many
connection codes `BL`; its parallel connection names are `T` values in the string stream.

`DwgBlockTwoPointParameter` contains default base/end points `3BD`, four property-info records, exactly
four property-state `BL` values, and parameter-base-location `BS`. Connection names retain declaration
order. The four-state cardinality is format-defined, not stored as a snapshot count.

`DwgBlockParameterValueSet` contains flags `BL`, minimum/maximum/increment `BD`, value-list count `BS`,
and that many `BD` values. Its LibreDWG `desc` is DXF/JSON-only and is not a native AC1024 string field.

`DwgBlockAction` extends BlockElement with display location `3BD`, dependency count `BL`, dependency
handle vector, action-code count `BL`, and action-code `BL` vector. Preserve both vectors in order and
derive counts. `DwgBlockActionWithBasePoint` adds offset `3BD`, two `(code BL, name T)` connections,
dependent `B`, and base point `3BD`.

`DwgAssocAction` main order is class version `BS` (AC1024 value 1), evaluation status `BL`, action index
`BL`, maximum dependency index `BL`, dependency count `BL`, then one ownership `B` per dependency. Handle
order is owning-network, action-body, then dependency references in the same order; ownership selects
hard-owner versus soft-pointer role. The class-version-greater-than-one parameter/value extension is
R2013+ and absent.

### Thin constant/reference bodies — 33 frames, first implementation cohort

| Type/class/count | Exact class main and string order | Class handle order and typed invariants |
| --- | --- | --- |
| 522 `ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION`, 2 | purge-preventer version/flag `BS` | one block hard pointer after common handles; target must be a block header; reject replay macro |
| 559 `ACDB_BLOCKREPRESENTATION_DATA`, 12 | representation version/flag `BS` | one represented-block soft pointer after common handles; target must resolve to a block header |
| 547 `ACDB_DYNAMICBLOCKPROXYNODE`, 1 | exactly `DwgEvalExpr`; no suffix strings/scalars | only optional EvalExpr code-91 handle after common handles; exact exhaustion proves the nominally empty derived class |
| 543 `BLOCKPARAMDEPENDENCYBODY`, 6 | dependency-body version `BS = 1`, dimension-base version `BS = 1`, name `T`, class version `BS = 0` | no class-specific handles |
| 549 `ASSOCDIMDEPENDENCYBODY`, 12 | dependency-body version `BS = 1`, dimension-base version `BS = 1`, name `T`, class version `BS = 1` | no class-specific handles |

The two dependency-body classes are distinct tagged variants despite their one-bit logical difference.
The version constants are derived on write. Their names are semantic parameter/dimension dependency names.

### Grip bodies — 10 frames

All four start with `DwgBlockGrip`; their only potential inherited class handle is an EvalExpr code-91
value.

| Type/class/count | Exact suffix | Class-local handles |
| --- | --- | --- |
| 528 `BLOCKLINEARGRIP`, 4 | orientation `3BD_1` | none |
| 530 `BLOCKFLIPGRIP`, 3 | combined state `BL`, orientation `3BD_1` | none; commented update/state fields are not AC1024 storage |
| 532 `BLOCKVISIBILITYGRIP`, 1 | empty tagged suffix | none |
| 534 `BLOCKALIGNMENTGRIP`, 2 | orientation `3BD_1` | none |

Orientations must be finite vectors. Combined state is a typed dynamic-block state identifier, not an
unvalidated integer bag. Exact class-wide exhaustion decides the candidate; no commented LibreDWG field
may be emitted.

### Action bodies — 12 frames

Every connection is a typed `{ code: u32, name: String }`; code enters main data and name enters the string
stream at the same declaration position. Action dependency handles follow inherited EvalExpr handle (if
present), then common/action roles as described by the base.

| Type/class/count | Exact suffix after `DwgBlockAction` | Additional handle order |
| --- | --- | --- |
| 521 `BLOCKMOVEACTION`, 2 | two connections; X offset `BD`, Y offset `BD`, angle offset `BD` | none |
| 537 `BLOCKFLIPACTION`, 3 | four connections | none; current R2010 `dwg2.spec` emits no trailing doubles, so any residue rejects the candidate |
| 536 `BLOCKSCALEACTION`, 1 | base-point action suffix (offset, two connections, dependent, base point), then three scale connections | none |
| 535 `BLOCKSTRETCHACTION`, 6 | two connections; point count `BL` and `2RD` points; selected-object count `BL`; per selected object index-count `BS` and `BL` indices; selector-code count `BL`; per selector code `BL`, index-count `BS`, `BL` indices; X/Y/angle offsets `BD` | selected-object handle vector, in selected-object order, after inherited action dependency handles |

For stretch actions, model `DwgStretchSelection { object, vertex_indices }` and
`DwgStretchSelector { code, indices }`. Derive all counts, require every index in range of the target's
logical subgeometry, reject duplicate selection entries, and preserve point/selection/selector order.

### Parameter bodies — 11 frames

| Type/class/count | Exact suffix after its typed base | Additional handle order |
| --- | --- | --- |
| 527 `BLOCKLINEARPARAMETER`, 2 | two-point base; distance name `T`, description `T`, distance `BD`, one parameter-value set | inherited EvalExpr handle only |
| 529 `BLOCKFLIPPARAMETER`, 3 | two-point base; flip label `T`, label description `T`, base-state label `T`, flipped-state label `T`, default-label point `3BD`, state/label identifier `BL`, tooltip `T` | inherited EvalExpr handle only |
| 531 `BLOCKVISIBILITYPARAMETER`, 1 | one-point base; initialized `B`, visibility name `T`, description `T`, secondary visibility-policy `B`, block count `BL`, state count `BL`; each state has name `T`, block count `BL`, parameter count `BL` | inherited EvalExpr handle, top-level block refs, then each state's block refs and parameter refs in state order |
| 533 `BLOCKALIGNMENTPARAMETER`, 2 | two-point base; align-perpendicular `B` | inherited EvalExpr handle only |
| 538 `BLOCKBASEPOINTPARAMETER`, 1 | one-point base; parameter point `3BD`, base point `3BD` | inherited EvalExpr handle only |
| 546 `BLOCKVERTICALCONSTRAINTPARAMETER`, 1 | two-point base; constraint dependency; expression name `T`, description `T`, value `BD`, value set | inherited EvalExpr handle, then dependency hard pointer |
| 548 `BLOCKHORIZONTALCONSTRAINTPARAMETER`, 1 | same linear-constraint body as vertical | inherited EvalExpr handle, then dependency hard pointer |

For visibility parameter state use `DwgVisibilityState { name, visible_blocks, controlled_parameters }`.
The secondary Boolean must be a named `visibility_policy_enabled` concept until API correlation gives a
narrower public label; it is still a Boolean logical field and never opaque storage. Top-level and state
block/parameter counts are derived, targets resolve to block headers/dynamic parameters, and state names
are unique. Constraint dependency handles resolve to `ACDBASSOCDEPENDENCY`-family objects. All points and
distances are finite and value-set ranges are coherent.

### Associative graph bodies — 9 frames

`ACDBASSOCNETWORK`, type 539, five frames, extends `DwgAssocAction` with network version `BS`, network
action index `BL`, action count `BL`, one ownership `B` per action, then owned-action count `BL`. Its handle
order after the inherited action handles is the action vector followed by the owned-action vector.
Represent the first as ordered `{ owned, action }` edges and the second as ordered soft references; derive
counts, cap each vector at 100 as prescribed, require every target to be an action, and require every
hard-owned edge to agree with the target owner. The network version is a closed AC1024 constant validated
across all five frames.

`ACDBASSOC2DCONSTRAINTGROUP`, type 540, four frames, extends `DwgAssocAction` with:

1. constraint-group version `BL` (R2010 candidate 2);
2. constraint-context-active `B` (fixture candidate false);
3. work plane origin, X axis and Y axis, three `3BD` values;
4. group dependency/body handle;
5. action count `BL` and action handles;
6. node count `BL`; each R2010 node is signed node ID `BLd`, status `RC`, connection count `BL`, and
   ordered connected-node IDs `BL`.

Handle order after inherited action handles is group dependency/body then action vector. Model a typed
orthonormal `DwgConstraintWorkPlane`, a role-named dependency/body reference, and
`DwgConstraintNode { id, status, connections }`. Derive counts; require unique node IDs, reciprocal or
otherwise specification-valid edges, no dangling connection IDs, finite/nondegenerate axes, action target
resolution and exact four-frame stream exhaustion. R2013 moves node status after connections; AC1024 uses
the pre-R2013 order above. The richer geometry/constraint-node variants in LibreDWG are not selected by
this candidate declaration and must not be guessed from remaining bits.

### Fixture invariants and zero-raw acceptance

- Populations must remain exactly thin 33, grips 10, actions 12, parameters 11, associative graphs 9,
  totaling 75.
- Every EvalExpr discriminator matches exactly one typed value and every node/parent reference is valid in
  the evaluation graph when non-null.
- Every derived count equals its logical collection; connection, state, selection, dependency and action
  order is stable.
- All common and class handles resolve to their named role and use ownership-compatible wire codes.
- Every class consumes main, string and handle streams exactly, leaving only the derived all-one suffix of
  at most seven bits.
- Decode then typed write reproduces every same-class original frame before the class is admitted to the
  full object-section writer.
- Snapshot, diff, mutation, DSL/binary facets name every union/field above. No unknown bits, raw tails,
  original frame bytes, imported offsets, compact handle tokens, string footers, counts, padding or CRCs.

### Implementation ordering

1. Thin bodies: types 559/549/543/522/547, 33 frames. They prove empty suffixes, constants, strings and
   one-reference bodies with minimal derived complexity.
2. Grip bodies: types 528/530/532/534, 10 frames, sharing the already researched BlockGrip base.
3. Action bodies: types 521/537/536/535, 12 frames; implement stretch last within the cohort.
4. Parameter bodies: types 527/529/531/533/538/546/548, 11 frames; visibility last because of nested
   state/handle vectors.
5. Associative graph bodies: types 539/540, nine frames; network before 2D constraint group.

P10's 75-frame missing-research bucket is therefore zero. The revised identity-only research partition is
325 code-ready frames and 39 cohort-only frames, still totaling 364.

## P12 — Complete Typed Research for the Final 39 Frames

This section eliminates P10/P11's last cohort-only bucket. All 364 currently identity-only frames now
have a class-specific typed candidate. The source hierarchy is ODA's normative R2010/R24 prescriptions
where published, then LibreDWG's `dwg.spec`/`dwg2.spec` declaration order, with the fixture census as the
admission boundary. A candidate is enabled only after all same-class fixture frames consume and reproduce
exactly; source comments such as `unknown`, `unstable`, and `HANDLE_UNKNOWN_BITS` never become schema.

References:

- ODA [Open Design Specification for .dwg files](https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf),
  especially 20.4.87 MLEADERSTYLE, 20.4.92 SCALE, 20.4.93 SORTENTSTABLE and 20.4.101 R24 TABLESTYLE.
- LibreDWG [`dwg.spec`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec), DICTIONARYVAR.
- LibreDWG [`dwg2.spec`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec), MATERIAL,
  EVALUATION_GRAPH and the embedded content/cell-style declarations.

### `DICTIONARYVAR`, dynamic type 503, eight frames

Typed schema: `DwgDictionaryVariable { schema: DwgDictionaryVariableSchema, value: String }`.

- Main: signed schema/version `RCd`.
- String: value `T`.
- Handles: common owner/reactors/xdic only.

The schema byte is a closed standard revision/newtype validated across the eight fixture instances, not a
generic integer. There are no class handles. Require valid UTF-16, exact stream exhaustion and dictionary
ownership from the parent key. Writer derives no auxiliary count.

### `SCALE`, dynamic type 507, 17 frames

Typed schema: `DwgScale { name, paper_units, drawing_units, unit_scale }`.

- Main: class flag/version `BS = 0`; paper units `BD`; drawing units `BD`; unit-scale `B`.
- String: name `T`.
- Handles: common handles only.

Derive the zero class flag. Units must be finite and positive. Names are unique under the owning scale-list
dictionary. `unit_scale` is a Boolean semantic property, not inferred merely from floating-point equality.
All 17 names/ratios must survive mutation/inverse without storing compact-number choices.

### `SORTENTSTABLE`, dynamic type 516, seven frames

Typed schema: `DwgSortentsTable { block_owner, entries: Vec<DwgDrawOrderEntry> }`, where each entry is
`{ entity, sort_handle }`. Both values are semantic references; neither wire token is persisted.

Native AC1024 order is unusual and must remain explicit:

1. main data count `BL`;
2. exactly count sort handles, encoded as code-0 handle values in the main data stream;
3. switch to the dedicated handle stream;
4. after common owner/reactors/xdic, block-owner handle;
5. exactly count entity handles.

There is no string stream. Zip entity/sort handles positionally into `DwgDrawOrderEntry`, derive the count,
cap it at 50,000, require unique entity references and resolution into the stated block, and reject a null
pair member. Preserve draw-order entry order. The main-stream placement of sort handles is writer policy,
not physical snapshot state.

### `MLEADERSTYLE`, dynamic type 508, one frame

Typed schema groups named enums/colors/references rather than mirroring integer words:
`DwgMLeaderStyle { content_type, draw_multileader_order, draw_leader_order, max_points,
first_segment_angle, second_segment_angle, leader_type, leader_line, landing, dogleg, description,
arrowhead, text, block_content, scale, annotative, break_size, attachments }`.

Exact AC1024 main order after common object data:

1. class version `BS = 2`;
2. content type `BS`, multi-leader draw order `BS`, leader draw order `BS`, maximum points `BL`;
3. first/second segment angles `BD`, leader type `BS`, line color `CMC`, line-type `H`, lineweight `BLd`;
4. landing enabled `B`, landing gap `BD`, dogleg enabled `B`, landing distance `BD`;
5. description `T`, arrowhead block `H`, arrowhead size `BD`, default text `T`, text style `H`;
6. left/right attachment `BS`, text-angle type `BS`, text-alignment type `BS`, text color `CMC`, text
   height `BD`, text-frame `B`, always-align-left `B`, alignment space `BD`;
7. content block `H`, block color `CMC`, block scale X/Y/Z as three `BD`, use-block-scale `B`, block
   rotation `BD`, use-block-rotation `B`, block-connection `BS`;
8. overall scale `BD`, changed `B`, annotative `B`, break size `BD`;
9. R2010 attachment direction `BS`, top attachment `BS`, bottom attachment `BS`.

String order is description then default text. Class-handle order after common handles is line type,
arrowhead block, text style, content block. The R2013 text-extended Boolean is absent. All enums are closed,
angles/scales finite, colors typed, non-null references role-checked, and maximum-points bounded. `changed`
is a semantic dirty/update property documented by ODA, not a source-replay marker.

### `MATERIAL`, dynamic type 505, three frames

Typed schema:

- `DwgMaterialColor { factor, source: Current | Override { rgb } }`;
- `DwgMaterialMapper { projection, tiling, auto_transform, transform: [f64; 16] }`;
- `DwgMaterialMap { blend_factor, mapper, source: Scene | File { filename } | Procedural { texture } }`;
- `DwgProceduralTexture = Wood { color1, color2 } | Marble { color1, color2 } |
  Generic(DwgProceduralValue)`;
- `DwgProceduralValue = Boolean | Integer | Real | Color | Text | Table(Vec<{ name, texture }>)`;
- `DwgMaterial { name, description, ambient, diffuse, diffuse_map, specular, specular_map,
  specular_gloss, reflection_map, opacity, opacity_map, bump_map, refraction_index, refraction_map,
  translucence, self_illumination, reflectivity, illumination_model, channel_flags, mode }`.

Color encoding is flag `RC`, factor `BD`, and RGB `BLx` only for Override. Each map encodes blend `BD`,
projection `RC`, tiling `RC`, auto-transform `RC`, sixteen matrix `BD`, then source `RC`; File adds `T`,
Procedural adds the typed texture. Projection is Inherit/Planar/Box/Cylinder/Sphere; tiling is
Inherit/Tile/Crop/Clamp/Mirror. Auto-transform is a typed bitset for no-transform, scale-to-entity and
current-block-transform behavior.

Procedural texture order:

- texture mode `BS` 0 Wood or 1 Marble, followed by two material colors;
- mode 2 Generic, followed by value discriminator `BS` and exactly one Boolean `B`, integer `BS`, real
  `BD`, color `CMC`, text `T`, or table;
- a table has count `BS`, then depth-first `(name T, nested texture)` entries, followed by derived table-end
  `B = true`.

Complete material main/string traversal order:

1. name `T`, description `T`, ambient color, diffuse color;
2. diffuse map;
3. specular color, specular map, then specular-gloss `BD`;
4. reflection map;
5. opacity percent `BD`, opacity map;
6. bump map;
7. refraction index `BD`, refraction map;
8. R2007+ translucence `BD0`, self-illumination `BD0`, reflectivity `BD0`, illumination model `BL0`,
   channel flags `BL0`, mode `BL0`.

Strings appear in that traversal: name, description, conditional filenames/text/table names and recursively
nested procedural strings. There are no class-specific handles. The disabled LibreDWG advanced-material
block is stored, when present, through the semantic ADVMATERIAL extension-dictionary object and must not be
invented inside this body. Bound recursive depth/count, validate enum ranges, factors and finite matrices,
and require all three fixture frames to exhaust exactly.

### `TABLESTYLE`, dynamic type 504, one frame

ODA's R24 prescription corrects LibreDWG's misleading `numoverrides`/one-record interpretation. The
logical body is `DwgTableStyle { name, format_version, flags, template_style, table_style,
builtin_styles }`. The primary style is `Table`; the following count is the three repeated built-ins
`_Data`, `_Title`, `_Header` in that native order. Each is
`DwgNamedCellStyle { id, class, name, format }`.

R24 outer order:

1. R24 format discriminator `RC`;
2. table-style name/description `T`;
3. table-style format version `BL`, table-style flags `BL`;
4. template/base-cell-style hard-owner `H`;
5. Table cell-style fields, identity ID `BL = 4`, class `BL`, name `T`;
6. built-in-style count `BL = 3`;
7. three repetitions of cell-style fields, identity ID `BL`, class `BL`, name `T`.

The two words ODA labels unknown are modeled as the named R24 format-version and table-style-flags fields,
each constrained by the single fixture and future specification vectors; they are standard typed scalars,
not opaque data. The hard owner is a role-named template/base style reference.

`DwgCellStyle` exact order:

1. style type `BL`, data-present flags `BSx`;
2. when data is present: property-override flags `BLx`, merge flags `BLx`, background color `CMTC`,
   content-layout flags `BL`;
3. content format: override flags `BLx`, property flags `BLx`, value data type `BLx`, value unit type
   `BLx`, format string `T`, rotation `BD`, block scale `BD`, alignment `BL`, content color `CMTC`, text
   style `H`, text height `BD`;
4. margin-override flags `BSx`; if present, vertical, horizontal, bottom, right, horizontal-spacing and
   vertical-spacing margins as six `BD`;
5. border count `BL`, maximum six;
6. per border: edge mask `BLx`; if nonzero, override flags `BL`, border type `BL`, color `CMTC`,
   lineweight `BLd`, linetype `H`, invisibility `BL`, double-line spacing `BD`.

Use closed cell-style, class, data/unit, alignment, layout, edge, override and border-type enums/bitsets.
String order is outer name, Table content-format string, Table identity name, then each built-in's format
string and identity name. Handle order after common handles is template style, then each style's content
text-style and border-linetype handles in Table/Data/Title/Header traversal order. Require IDs/names
`4/Table`, `3/_Data`, `1/_Title`, `2/_Header`, unique IDs/names, exactly three built-ins, finite dimensions
and exact one-frame reproduction.

### `EVALUATION_GRAPH`, dynamic type 517, two frames

Typed schema: `DwgEvaluationGraph { first_node, nodes: Vec<DwgEvaluationNode>,
edges: Vec<DwgEvaluationEdge>, cycle_state: Option<DwgEvaluationCycleState> }`.

Main order:

1. signed first-node ID `BLd`, followed by the required identical copy `BLd`;
2. node count `BL`;
3. per node: ID `BL`, edge flags `BL = 32`, signed next ID `BLd`, EvalExpr object `H`, four signed node
   relation IDs `BLd`, and an active-cycle `B` when cycle-state fields are present;
4. edge count `BL`;
5. per edge: ID `BL`, signed next ID `BLd`, signed relation IDs e1/e2/e3 `BLd`, then five signed
   outgoing-edge IDs `BLd`.

There is no string stream. Node EvalExpr references form the class-specific handle vector after common
handles, in node order. Store first-node once and derive its duplicate. Derive edge flag 32. Model active
cycle presence as one graph-wide semantic option and require every node to have the same presence shape;
do not store LibreDWG's unstated `has_graph` implementation flag. Counts are derived.

Across both graphs require unique node/edge IDs, valid first/next/relation/outgoing IDs or the standard -1
sentinel, EvalExpr target resolution, reciprocal graph consistency, and exact main/handle exhaustion. If
the fixture contradicts the optional cycle-state candidate, reject atomically and refine the named graph
concept; never capture the remaining frame.

### Final zero-unknown admission matrix

| Class | Count | Primary acceptance invariant |
| --- | ---: | --- |
| DICTIONARYVAR | 8 | schema/value and dictionary ownership exact |
| SCALE | 17 | unique names and finite positive ratios |
| SORTENTSTABLE | 7 | paired sort/entity refs, block membership, split-stream order |
| MLEADERSTYLE | 1 | version 2, four class handles in role order |
| MATERIAL | 3 | tagged conditional map traversal, no disabled advanced tail |
| TABLESTYLE | 1 | Table plus exactly Data/Title/Header cell styles |
| EVALUATION_GRAPH | 2 | root-copy equality, flag 32, closed node/edge graph |
| **Total** | **39** | exact same-class frame reconstruction |

The full 364-frame identity-only research partition is now 364 code-ready, zero cohort-only, and zero
missing. Implementation order for this final group is DICTIONARYVAR/SCALE (25), SORTENTSTABLE (7),
MLEADERSTYLE (1), MATERIAL (3), TABLESTYLE (1), then EVALUATION_GRAPH (2). TABLESTYLE and graph candidates
remain strict all-frame gates because the secondary source labels them unstable; that uncertainty is
represented as rejection risk, not unknown schema state.

## P13 — AC1024 structured persistence and language-facet gap audit (read-only, 2026-08-14)

This audit compared the live Rust logical schema and its derived DSL/pack/diff/mutation codecs with every
committed TypeScript, GraphQL, Proto and JSON schema leaf. No production file was edited and no Nx/Cargo
command was run. The Rust persistence path is genuinely structured: snapshot text uses `DslRecord`, pack
uses `pack_rt::encode_document`, diff uses `DslDiff`, and operations use `DslOps` plus
`variants_binary`. The fixture lifecycle test also routes the real drawing through DSL, pack, diff and
`SetSnapshot`. The blocking risk is schema drift that Rust compilation cannot see because most language
facets are passive `include_str!` resources.

### P0 — loss in every non-Rust object-body facet

The live Rust source at `schema/📸️snapshot/🦀️component.rs:316-544` has nine tagged table-control
variants and two tagged table-record variants (`RegisteredApplication`, `TextStyle`). All four external
facets still describe the earlier flat control and name-only record:

- artifact Proto `schema/🛰️component.proto:11-15`, TS `schema/🟦️component.ts:8-12`, GraphQL
  `schema/🔗️component.graphql:16-20`, JSON `schema/🔣️component.json:38-47`;
- snapshot Proto `schema/📸️snapshot/🛰️component.proto:11-15` and GraphQL
  `schema/📸️snapshot/🔗️component.graphql:16-20` duplicate those stale definitions (snapshot TS
  reexports the stale artifact TS; snapshot JSON references the stale artifact JSON);
- diff Proto `schema/🔺️diff/🛰️component.proto:11-16`, TS `schema/🔺️diff/🟦️component.ts:9-14`
  and GraphQL `schema/🔺️diff/🔗️component.graphql:16-21` copy the same stale graph; diff JSON
  reaches the stale artifact definition by `$ref`.

Required atomic schema-first replacement: declare role types `DwgTableControlEntry`,
`DwgTableControlEntries`, `DwgBlockTableControl`, `DwgLinetypeTableControl` and
`DwgDimensionStyleTableControl`, then a closed tagged/oneof control with all nine variants. Declare
`DwgTableRecordCommon`, `DwgRegisteredApplicationTableRecord`, `DwgTextStyleTableRecord`, then a closed
tagged/oneof record. Add future record variants only to the Rust tagged union and all language facets in
the same change. Do not flatten variant-specific handles into a common vector or generic object.

### P0 — EED is silently absent outside Rust

Rust persists `DwgExtendedEntityData` and `DwgLogicalObject.extended_data` at
`schema/📸️snapshot/🦀️component.rs:556-562,599-615`. Artifact, snapshot and diff TS/GraphQL/Proto/JSON
all omit the field. A consumer following any committed external schema therefore drops all EED on a
round trip even though the native decoder has recovered it.

Do not expose EED as unrestricted `DwgXRecordValue`. The Rust type currently permits the full XRECORD
union and relies on the native writer to reject non-EED group codes. Introduce a closed `DwgEedValue`
union for the standard 1000/1002/1003/1004/1005/1010–1015/1040–1042/1070/1071 concepts and use a
role-named application/APPID handle. Propagate that same union and `extendedData` field to every facet.
This moves validity into schema rather than serialization-time rejection.

### P0 — diff facets omit six live replacement fields

Rust `DwgDiff` has optional `header`, `classes`, `dependencies`, `summary`, `application` and `template`
at `schema/🔺️diff/🦀️component.rs:28-45`. Diff Proto `:17-22`, TS `:15-20`, GraphQL `:22`, and
JSON `:5-10` expose only version/maintenance/codepage/drawing. The Rust diff codecs retain the fields,
but all four published schemas deny that wire shape. Add all six fields with the same ordinals and
optionality as the Rust `DslDiff`; prefer references to one canonical shared logical type graph rather
than copying the entire stale object schema into the diff facet.

### P0 — committed binary/text protocol leaves remain opaque

Snapshot/diff/mutation `📡️component.protocol.semio` end in `chain body bytes` (snapshot line 12,
diff/mutation line 11). Their ABNF accepts `*OCTET`, Kaitai uses `size-eos`, and Spicy uses `bytes &eod`.
Text GraphQL leaves define `Document { schema, payload: String }`; text Proto leaves define
`Artifact { schema, bytes payload }`; text JSON leaves are unconstrained `{ type: object }`. These do not
describe the structured `DslRecord`/`DslDiff`/`DslOps` records the Rust runtime actually emits.

The existing conformance test at `🚪️io/🦀️component.rs:4398-4428` only proves the generic
protocol walker consumed all bytes; an opaque tail necessarily passes it. Replace the protocol leaves
with the actual field-tag/ordinal/value-container grammar and test decoded field traces against the
logical record, including each body variant, EED, and every diff/op variant. Replace opaque text facets
with references to the corresponding structured snapshot/diff/op schema. Binary representation TS may
remain `Uint8Array` as a transport API, but the normative protocol must define its structure.

### P1 — cross-language integer and union fidelity

- TS uses `number` for every `u64` handle and `i64` XRECORD integer (`schema/🟦️component.ts:5-12`),
  losing values outside 53-bit safe integer range. Use schema-owned `DwgHandle`/`Int64` decimal-string or
  bigint-safe concepts consistently.
- GraphQL uses `Float` for handles and i64, and collapses integer8/16/32/64 into one
  `DwgXRecordIntegerValue` (`schema/🔗️component.graphql:5-20`). That loses both exact value and integer
  width. Use exact-width tagged object types and lossless custom scalars; define corresponding input
  tagged types for mutations because GraphQL unions are output-only.
- Proto's separate oneof integer members and `uint64`/`sint64` are fidelity-safe, but it still needs the
  missing EED and new control/record message graph.
- JSON's integer-width discriminator is retained, but JSON/JS interoperability still needs a lossless
  decimal representation for i64/u64. All tagged alternatives and mutation alternatives should set
  `additionalProperties: false` so an unmodeled payload cannot hide beside a recognized variant.

### P1 — ambiguous or role-less Rust shapes to close before facet generation

- `DwgTableControlEntry.handle: Option<u64>` (`snapshot/🦀️component.rs:316-321`) lets a persisted table
  control contain a null entry; the writer currently serializes it as handle zero. Make a table member a
  required role-named reference, reserving optionality only for fields the standard actually makes
  optional.
- `DwgDimensionStyleTableControl.additional_handles` (`:350-357`) is an order-dependent, role-less bag.
  Replace it with the named DIMSTYLE-control roles established by the native layout before publishing it.
- `DwgLogicalObject.referenced_handles` (`:609-613`) duplicates or obscures references that now belong in
  typed bodies. Retire it as each body becomes typed; do not carry it into new body facets as an escape
  hatch.
- `DwgXRecordBody.values` plus separate `object_id_handles` (`:546-554`) need a schema invariant tying
  object-ID value order/cardinality to resolved handle order, or a single typed value carrying both
  semantic pieces. Parallel uncorrelated lists can be mutated into an unwritable state.
- `DwgTableRecordCommon.xref_resolution` and registered-application `group_71` (`:458-472`) are raw
  implementation labels. Replace them with named enum/bitset concepts before the forthcoming record
  variants are propagated.
- `DwgTableControlBody::from_value` (`:441-454`) and `DwgTableRecordBody::from_value` (`:534-542`) select
  the active payload but do not reject extra inactive payload fields. Match the strict
  `DwgLogicalObjectBody`/XRECORD behavior: require exactly discriminator plus one payload and reject every
  inactive payload. This prevents ambiguous structured DSL/pack documents.

### P1 — mutation facet is not a closed tagged schema in GraphQL

Rust mutation is a closed `DslOps` enum (`schema/🧬️mutations/🦀️component.rs:11-24`) and preserves
new bodies through `SetSnapshot`. Proto/TS/JSON express the three variants, but GraphQL
`schema/🧬️mutations/🔗️component.graphql:1` is a nullable field bag with a string discriminator, so
invalid combinations are representable. Publish a closed mutation-kind enum plus variant-specific input
objects/one-of convention. JSON mutation alternatives also need `additionalProperties: false` and exact
u8/u16 bounds.

### P1 — current tests do not detect facet drift

`schema_facets_contain_no_container_shadow_state` at `🚪️io/🦀️component.rs:4303-4351` searches only
for old forbidden words. `schema_facets_reject_imported_byte_shadow_state` at `:4469-4492` even reads the
snapshot **text-representation** GraphQL/Proto payload wrappers rather than the logical snapshot
GraphQL/Proto files, and likewise does not assert any required tagged symbol/field. Add a schema-parity
law that requires, in artifact/snapshot/diff and mutation-reachable facets, every Rust body variant,
every EED variant, `extendedData`, and every diff field. Add negative checks for `payload`, `body bytes`,
`size-eos`, `*OCTET`, untyped `type: object`, Float handles, and role-less fallback fields. Add rich demo
cases containing every live tagged control/record/XRECORD/EED variant; current demo diff's default drawing
does not independently prove these nested branches.

### Compile-ready implementation order

1. Close the Rust semantic gaps (EED union, required/role-named refs, exact-one payload checks); add all
   forthcoming table records to the one Rust tagged union.
2. Regenerate/hand-update the artifact logical TS/GraphQL/Proto/JSON graph once; make snapshot, diff and
   mutation facets reference it rather than copy it.
3. Add the six missing diff fields and closed mutation inputs.
4. Replace opaque grammar/protocol leaves with structural record schemas.
5. Add facet-parity and rich nested codec laws, then compile. Rust compiler pressure will catch missing
   `match` arms in `DwgTableRecordBody::to_value/from_value` and native encode/decode; it cannot catch any
   passive facet omission without these tests.

## P13 Addendum — Post-Type 542/541 Facet and Codec Parity (Read-Only, 2026-08-14)

This addendum supersedes the earlier P5 statement that type 541 ends after the common dependency body.
The live native decoder proves that `ACDBASSOCVALUEDEPENDENCY` adds value-dependency version `BS = 0`,
cached-value group code `BS = 90`, cached `BL` integer32 and a required value-name `TU` after the shared
type-542 dependency fields. The ticket inventory remains 18 type-542 frames and 23 type-541 frames.

### Rust authority and stable structured tags

The persisted authority is `schema/📸️snapshot/🦀️component.rs:1015-1084,1134-1204`:

| Logical concept | Closed Rust shape | DSL record/tag contract |
|---|---|---|
| dependency status | `DwgAssociativeDependencyStatus::UpToDate` | closed scalar `upToDate`; reject every other value |
| type 542 | `DwgAssociativeDependency` | field IDs 0..11 in declaration order: `status`, four Boolean flags, `order`, `dependentOnObjectHandle`, optional `name`, optional `readDependencyHandle`, optional `dependencyNodeHandle`, optional `dependencyBodyHandle`, `dependencyBodyId` |
| cached evaluation | `DwgEvaluationVariant::Integer32(i32)` | record field 0 `kind = integer32` (ordinal 0), field 1 `integer32`; exactly one discriminator and payload |
| type 541 | `DwgAssociativeValueDependency` | field 0 `dependency`, field 1 `cachedValue`, field 2 `valueName` |
| object-body union | `AssociativeDependency` / `AssociativeValueDependency` | kind ordinals 5/6 and payload field IDs 6/7 respectively; the live `from_value` enforces exactly discriminator plus one payload |

The Rust snapshot DSL/pack path is therefore structurally complete for both variants. `DwgDiff` stores a
typed `Option<DwgLogicalDrawing>` and derives `DslDiff`; `SetSnapshot` stores a typed `DwgSnapshot` and
derives `DslOps`. A nested type-541 body is retained by those Rust codecs without JSON, source bytes,
unknown bytes, or raw/tail fields. `DwgEvaluationVariant::from_value` rejects any unrecognized cached
value tag. No Rust compile blocker was found in this narrow lane.

The native implementation at `🚪️io/🦀️component.rs:3900-4004` is also bounded: it rejects unsupported
class/status/value versions, demands exact main and string-stream exhaustion, reads four role-named class
handles, and validates terminal fill. The role handles belong in `DwgAssociativeDependency`; they must not
be copied into `DwgLogicalObject.referenced_handles` as a generic fallback. Likewise, no future evaluation
code may be retained as an opaque payload: unsupported codes must fail import until a named
`DwgEvaluationVariant` is implemented end to end.

### Exact missing-symbol matrix

| Surface | Live evidence | Required code-ready correction |
|---|---|---|
| artifact TypeScript | `schema/🟦️component.ts:18-20` has type 542 but its object-body union ends there | add `DwgEvaluationVariant = { kind: 'integer32'; value: number }`, `DwgAssociativeValueDependency { dependency; cachedValue; valueName }`, and the `associativeValueDependency` union arm |
| artifact GraphQL | `schema/🔗️component.graphql:23-24` has only dependency and uses `status: String!` | add closed `DwgAssociativeDependencyStatus`, `DwgEvaluationInteger32`, union `DwgEvaluationVariant`, `DwgAssociativeValueDependency`, and the object-body union member; do not model the cache as `String`, `Bytes`, or a nullable field bag |
| artifact Proto | `schema/🛰️component.proto:18-19` has only dependency; status is a string | add status enum; `DwgEvaluationVariant { oneof value { sint32 integer32 = 1; } }`; value-dependency fields 1/2/3; append object-body oneof arm `associative_value_dependency = 9` without renumbering 1..8 |
| artifact JSON | `schema/🔣️component.json:43-50` defines only dependency and its union arm | add a strict `evaluationVariant` oneOf tagged `integer32`, strict `associativeValueDependency`, and strict object-body arm; every branch must use `additionalProperties: false` |
| snapshot TypeScript/JSON | `snapshot/🟦️component.ts` reexports artifact types; `snapshot/🔣️component.json` references artifact drawing | become correct transitively only after the artifact graph is fixed; add a reference-resolution parity test |
| snapshot GraphQL/Proto | `snapshot/🔗️component.graphql:23-24` and `snapshot/🛰️component.proto:18-19` duplicate the stale graph | apply the same status/evaluation/value-dependency additions and tags, or replace duplication with the canonical artifact graph |
| diff TypeScript | `diff/🟦️component.ts:3-14` is older still: it omits ARC/LWPOLYLINE entity-body arms and type 541 | mirror the complete artifact `DwgLogicalObjectBody`, including the nested entity tagged union and both associative bodies; preferably import the canonical graph rather than hand-copy it |
| diff GraphQL/Proto | `diff/🔗️component.graphql:23-24` and `diff/🛰️component.proto:18-19` include entities and type 542 only | add the closed status/evaluation/value-dependency graph and type-541 union/oneof arm with the artifact tags |
| diff JSON | `diff/🔣️component.json` references artifact drawing | becomes correct transitively only after artifact JSON is fixed; still lacks the six non-drawing replacement fields already identified above |
| mutation TS/Proto/JSON | `mutations/🟦️component.ts:1-5`, `🛰️component.proto:3-6`, `🔣️component.json:4-8` reach bodies through `SetSnapshot` | no separate type-541 operation is required, but the referenced snapshot graph must resolve to both tagged variants; JSON alternatives also need `additionalProperties: false` |
| mutation GraphQL | `mutations/🔗️component.graphql:1` is a nullable field bag and does not define/import `DwgSnapshot` in the leaf | publish closed variant-specific mutation inputs and reference the complete snapshot input graph; this is a schema-validation blocker even apart from type 541 |

The artifact/snapshot/diff facets also omit live `DwgLogicalObject.extended_data` at Rust lines 1223-1224.
That pre-existing P13 blocker remains relevant: a type-541 or type-542 object carrying EED is lossy in
every external facet even after the body union is repaired.

### Exact opaque-leaf checklist

All three structured persistence families currently publish opaque normative leaves despite the Rust
codecs being structured:

| Family | ABNF | Kaitai | Spicy | protocol-semio | text-schema fallbacks |
|---|---|---|---|---|---|
| snapshot | `💾️binary/🔠️component.abnf:2-3` → `payload = *OCTET` | `🥋️component.ksy:4-6` → `payload size-eos` | `🌶️component.spicy:2-4` → `bytes &eod` | `📡️component.protocol.semio:8-12` ends `chain body bytes` | text GraphQL `payload: String`, Proto `bytes payload`, JSON unconstrained object, and token-star grammar |
| diff | same constructs at ABNF 2-3, Kaitai 4-6, Spicy 2-4 | same | same | `📡️component.protocol.semio:7-11` ends `chain body bytes` | same payload wrappers/token-star grammar |
| mutation | same constructs at ABNF 2-3, Kaitai 4-6, Spicy 2-4 | same | same | `📡️component.protocol.semio:7-11` ends `chain body bytes` | same payload wrappers; mutation grammar nests `snapshot-token*` |

Replace each EOF payload with the actual record framing: bounded field count, field ordinal, value-kind
tag, length for variable-width values, recursively structured record/list/optional values, and exact end
of record. The snapshot grammar must name all logical snapshot fields and object-body alternatives. The
diff grammar must name all ten optional replacement fields. The mutation grammar must recursively use
the snapshot grammar for `set-snapshot`, not `snapshot-token*`. ABNF, Kaitai and Spicy must expose the
same field/tag structure rather than merely proving that arbitrary bytes reach EOF. Binary TypeScript
may remain `Uint8Array` as an API carrier; it is not a normative grammar.

### Required assertions before accepting type 542/541 parity

Extend the existing tests in `🚪️io/🦀️component.rs`; do not add a new test file:

1. Decode the architectural fixture and assert exactly 18 `AssociativeDependency` bodies and 23
   `AssociativeValueDependency` bodies. For every value dependency assert `cachedValue` is the tagged
   `Integer32`, `valueName` is retained, the four role handles remain in `dependency`, and no class body
   is represented by a raw/unknown/tail payload.
2. Build one rich snapshot containing both bodies with all optional dependency fields populated. Require
   `print_dsl/parse_dsl` and `encode_pack/decode_pack` equality. Require unknown status, evaluation kind,
   inactive union payload, trailing record field, and truncated nested record to fail.
3. Put that rich drawing through diff text/binary, `between/apply/inverse/absorb`, mutation text/binary,
   `SetSnapshot/apply/inverse`, and direct native export. Assert both bodies and all nested fields after
   every route; the current default demo drawing does not exercise either branch.
4. Keep the real-fixture lifecycle baseline against the original 148,638 bytes after DSL, pack, diff and
   mutation routes. Add explicit type-541 and type-542 body equality before each export so exact native
   equality cannot mask a dropped facet branch.
5. For artifact/snapshot/diff TS, GraphQL, Proto and JSON sources, assert the symbols
   `DwgAssociativeDependency`, `DwgAssociativeValueDependency`, `DwgEvaluationVariant` (or the exact
   language spelling), `cachedValue`, `valueName`, and all four role-handle names. Assert Proto oneof tags
   are unique and stable and JSON branches are closed.
6. Expand the anti-shadow source set to the logical artifact/snapshot/diff facets and every
   ABNF/Kaitai/Spicy/protocol/text leaf. Reject `payload`, `body bytes`, `*OCTET`, `size-eos`, `bytes &eod`,
   `snapshot-token*`, `raw`, `opaque`, `unknown`, and `tail`. Legitimate XRECORD binary octets must be
   checked by their narrowly typed `DwgXRecordValue::Binary` path rather than exempting generic bytes.
7. Replace the present protocol-walk assertion `consumed == bytes.len()` with field-trace assertions:
   traces must include body kind 5 or 6, nested dependency field IDs 0..11, evaluation kind/value fields
   0/1, and value-dependency fields 0..2. An EOF byte chain can satisfy consumption while proving no
   schema parity.

Implementation priority is: canonical artifact graph first; snapshot/diff references second; closed
mutation inputs third; structural text/binary facets fourth; rich nested codec/parity laws last. This
avoids repairing three copied schemas independently and prevents another Rust-only body addition.

## P14 — Type 520 `BLOCKGRIPLOCATIONCOMPONENT` Readiness Validation (Read-Only, 2026-08-14)

The live post-type-544 tree is **not type-520 ready**: `DwgLogicalObjectBody` ends with
`AssociativeGeometryDependency` at `schema/📸️snapshot/🦀️component.rs:1143-1154`, and IO contains no
type-520 decode, encoder or exact-frame verifier. Type 520 therefore remains a body-less custom object
despite the ticket inventory proving 23 fixture frames. This section reduces the prior P6 research to the
minimal implementable contract.

Primary layout authority is LibreDWG's
[`AcDbEvalExpr_fields`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L2723-L2772),
[`AcDbBlockGripExpr_fields`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3100-L3103)
and the
[`BLOCKGRIPLOCATIONCOMPONENT` declaration](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3200-L3210).
The `DXF`-only `nodeid` declaration near the start is not AC1024 binary order. The non-DXF stream writes
`nodeid` after the tagged evaluation value.

### Minimal logical schema

Reuse and extend the live `DwgEvaluationVariant`; do not create a second overlapping value union and do
not persist the native `value_code`. Its existing `Integer32(i32)` remains the first/stable variant for
type 541. Append the standard alternatives required by `AcDbEvalExpr`:

```text
DwgEvaluationVariant =
  Integer32(i32)                         // existing; native code 90
  | Real(f64)                            // code 40
  | PointGroup10([f64; 2])              // code 10, exactly 2RD
  | PointGroup11([f64; 2])              // code 11, exactly 2RD despite LibreDWG's pt3d member name
  | Text(String)                         // code 1
  | ObjectReference(u64)                 // code 91, class-local hard pointer
  | Integer16(i16)                       // code 70
  | Null                                 // code -9999

DwgEvaluationExpression {
  parent_node_id: i32,
  evaluator_major: u32,
  evaluator_minor: u32,
  value: DwgEvaluationVariant,
  node_id: u32,
}

DwgBlockGripLocationComponent {
  evaluation: DwgEvaluationExpression,
  grip_type: u32,
  expression: String,
}
```

`PointGroup10` and `PointGroup11` deliberately retain distinct named standard alternatives because both
encode as two reals but their group discriminators are different. Do not invent unproven position/vector
semantics and do not collapse them into one point variant plus a raw integer. `Null` has no payload. Every
numeric and point value must be finite where applicable. Object-reference zero is invalid. The writer
derives native codes 90/40/10/11/1/91/70/-9999 from the variant and the decoder rejects every other code.

This record contains no class version, raw evaluator bytes, unknown bits, source tail, value-code field,
handle-code field, string-size field, frame-size field, fill field or CRC field. Those are serializer
policy. AC1024 selects the R2007+ dynamic-block layout directly; unsupported older-version export must
fail rather than add a compatibility branch.

### Exact bounded AC1024 streams

After the standard object type/self-handle/EED/common prefix, the class main stream is exactly:

1. signed `parent_node_id BLd`;
2. `evaluator_major BL`, then `evaluator_minor BL`;
3. derived signed `value_code BSd`;
4. one conditional value: `BD`, `2RD`, `2RD`, no main bits for text, `BL`, no main bits for object
   reference, `BS`, or no bits for null, according to the variant order above;
5. `node_id BL`;
6. `grip_type BL`;
7. no further class-main fields.

R2010 `T` values are read from the separate string stream in declaration order. For a text evaluation the
order is evaluation text then grip expression; for every other variant the only string is grip expression.
The decoder must compute the R2010 main/string boundary first, read strings from the bounded string
reader, and require exact exhaustion of both. The serializer appends the string bits, derived compact
`RS` bit length and presence bit using the same `append_r2010_string_stream` policy already used by types
541/542/544.

The handle stream is:

1. common owner, exactly reactor-count reactors, and extension dictionary iff present;
2. only for `ObjectReference`, one `evaluation_value_object` hard pointer with native role code 5;
3. the unique all-one terminal fill of zero through seven bits.

The evaluation reference must live only in `DwgEvaluationVariant::ObjectReference`, never in
`DwgLogicalObject.referenced_handles`. Common owner encoding may use the standard resolved absolute or
relative H forms; reactors and xdic follow the accepted common-object policy. The class-local code-91
role is specifically code 5 and must resolve to a nonzero drawing object.

Outer framing must use the accepted `finish_r2010_object_frame` contract at IO lines 2749-2769: derive
payload bytes and handle-stream bit count, emit `MS(payload_size)`, `UMC(handle_bits)`, byte-aligned
main/string/handle payload, then little-endian `CRC16(seed 0xC0C1)` over the complete outer header and
payload. No frame byte participates in the logical schema.

### Stable DSL and facet tags after type 544

Preserve every live ordinal/tag. The current Rust body kinds 0..7 and payload fields 1..8 end at geometry
dependency. Append `blockGripLocationComponent` as body kind ordinal **8** and payload field ID **9**.
The component record uses fields `evaluation = 0`, `grip_type = 1`, `expression = 2`; the evaluation
record uses `parent_node_id = 0`, `evaluator_major = 1`, `evaluator_minor = 2`, `value = 3`, `node_id = 4`.

The live evaluation DSL already assigns `Integer32` kind ordinal 0/value field 1. Append, without
renumbering it: `Real` ordinal 1/field 2, `PointGroup10` 2/3, `PointGroup11` 3/4, `Text` 4/5,
`ObjectReference` 5/6, `Integer16` 6/7 and `Null` 7 with no payload field. Enforce exactly discriminator
plus one payload, except exactly the discriminator for null.

Facet mapping follows the now-live type-544 pattern:

- TypeScript adds the extended discriminated `DwgEvaluationVariant`, `DwgEvaluationExpression`,
  `DwgBlockGripLocationComponent`, then the body arm;
- Proto preserves `integer32_value = 1`, appends value oneof tags 2..8 using point/null wrapper messages,
  defines evaluation fields 1..5 and component fields 1..3, then appends
  `DwgBlockGripLocationComponent block_grip_location_component = 11` to the body oneof;
- JSON adds strict closed evaluation alternatives and records, then kind
  `blockGripLocationComponent`; all branches use `additionalProperties: false` and point arrays require
  exactly two items;
- GraphQL uses exact wrapper types for every evaluation alternative plus a closed union; it must not use
  `Bytes`, `String`, a generic scalar bag or an untyped handle.

Apply the same graph to artifact, snapshot and diff facets; mutation carries it through `SetSnapshot`.
The opaque ABNF/Kaitai/Spicy/protocol blockers identified in P13 remain blockers for accepting type 520
through non-Rust persistence.

### Fixture invariants and enablement gate

The established fixture invariant is exactly **23** dynamic type-520
`BLOCKGRIPLOCATIONCOMPONENT` frames. The R2007+ evaluator lineage suggests major/minor 33/29, but that is
not a schema default: the first decoder run must inventory and assert the actual `(major, minor)`
distribution across all 23 before the writer fixes any canonical policy. The same probe must record, by
semantic value kind, counts and values for parent/node IDs, grip types, expression lengths, common
owner/reactor/xdic roles, class-local handle references, handle wire-code sequences and terminal-fill
lengths. Do not claim an unobserved per-field constant.

Acceptance requires one existing-test extension, not a new file:

1. decode exactly 23 typed bodies and zero body-less type-520 objects;
2. assert every main/string/handle reader exactly reaches its independently computed bound;
3. verify every class-local handle is present iff the value is `ObjectReference`, uses code 5 and resolves;
4. encode each frame independently and compare the complete original frame including `MS`, `UMC`, fill
   and CRC; report first byte/bit and both decoded logical records on mismatch;
5. mutate every evaluation alternative plus parent/node/version/grip/expression fields, require
   DSL/pack/diff/op round trips, then apply inverse and recover the original 23 frames exactly;
6. reject unsupported value codes, null object handles, non-finite numbers, wrong point cardinality,
   missing/extra strings, extra handles, zero terminal fill bits, CRC mismatch and any inactive DSL union
   payload;
7. extend facet parity assertions with every new symbol/field/kind and anti-shadow assertions forbidding
   evaluator payload/raw/unknown/tail storage.

Implementation order is schema/value-union first, bounded decoder second, independent exact-frame
verifier third, writer fourth, export dispatcher fifth, then all facets and lifecycle laws. Do not enable
the writer from the 23-count alone: exact per-frame bounded consumption and comparison are the gate.

## P15 — Type 547 `ACDB_DYNAMICBLOCKPROXYNODE` Live-Readiness Oracle (Read-Only, 2026-08-14)

LibreDWG declares this class as exactly
[`AcDbEvalExpr_fields`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L4989-L4996).
Its preceding `HANDLE_UNKNOWN_BITS` is the already-rejected debug replay macro, not a prefix or suffix.
There is no derived class version, subclass scalar, string, or handle after the evaluation expression.

The live type-520 implementation has now supplied reusable `DwgEvaluationExpressionValue` and
`DwgEvaluationExpression` at `schema/📸️snapshot/🦀️component.rs:1095-1173`, plus bounded type-520 IO at
`🚪️io/🦀️component.rs:3611-3673,4011-4046`. Type 547 is not yet live: no proxy-node body variant,
decoder, encoder, exact verifier or facet exists. Implement it by sharing the evaluation-expression
codec, never by copying type 520's switch.

### Minimal typed record and exact native order

The class-specific logical body is one field:

```text
DwgDynamicBlockProxyNode {
  evaluation_expression: DwgEvaluationExpression,
}
```

The wrapper remains necessary as a distinct object-body member and GraphQL/JSON/Proto concept; it carries
no marker/version/unknown field. Its evaluation expression uses the live fields and variants unchanged:
`parent_id`, `major_version`, `minor_version`, tagged `value`, `node_id`.

After BOT, self handle, typed EED, reactor-count `BL` and xdic-missing `B`, exact main order is:

1. signed parent node ID `BLd`;
2. evaluator major `BL`;
3. evaluator minor `BL`;
4. derived signed value discriminator `BSd` and its conditional value, using the complete type-520
   evaluation union;
5. node ID `BL`;
6. absent-string marker `B = 0` when the value is not text;
7. no suffix data.

If a future valid proxy node uses the text alternative, that text is the sole R2010 string-stream value.
If it uses object-reference code 91, that reference is the sole class-local handle, a nonnull hard pointer
with role code 5 after common handles. All other value alternatives add no class-local handle. The writer
derives both discriminator and string/footer/handle presence from the union; it never stores them.

The handle stream is common owner, exactly reactor-count reactors, optional xdic, optional evaluation
object reference, then the unique zero-to-seven-bit all-one fill. The evaluation reference belongs inside
`DwgEvaluationExpressionValue::ObjectReference`, not `DwgLogicalObject.referenced_handles`.

Outer assembly is the existing `finish_r2010_object_frame` contract: `MS` payload byte size, `UMC`
handle-stream bit size, aligned payload, and little-endian `CRC16(seed 0xC0C1)` over the outer header and
payload. Sizes, native value-code, handle code, absent-string bit, fill and CRC are serializer outputs,
never schema fields.

### Exact singleton fixture oracle

The existing ticket probe was rerun read-only against `architectural_example.dwg` and establishes this
complete signature:

| Property | Exact fixture value |
|---|---|
| class population | 1 |
| object handle / self-handle code | `0x1164` / code 0 |
| BOT selector | 1 |
| EED | empty, terminated normally |
| reactor count / compact branch | 0 / BL branch 2 |
| xdic | missing |
| expression | parent `-1` BL branch 0; major `29` BL branch 1; minor `2` BL branch 1; `Empty` / native `-9999` BS branch 0; node `226` BL branch 1 |
| strings | absent, zero string bits |
| handles | owner only: resolved `0x1155`, native code 12 |
| handle-stream bits / fill | 22 / exactly `111111` |
| data boundary / class end | bit 122 / bit 121; the one intervening bit is the absent-string marker |
| outer prefix / payload / total frame | 3 / 18 / 23 bytes |
| CRC16 | `0x10f2` |

This is a byte-exact fixture oracle, not a reason to make `-1/29/2/Empty/226` schema constants. Decode
them into typed fields and allow valid mutations. The writer must reproduce the compact branches above
for the original values and select branches deterministically for mutated values.

### Append-only tags after live type 520

Rust body ordinals 0..8 and payload fields 1..9 now end at
`BlockGripLocationComponent` (`snapshot/🦀️component.rs:1230-1313`). Append
`dynamicBlockProxyNode` as kind ordinal **9**, payload field ID **10**. The wrapper's sole
`evaluation_expression` is field ID 0. Do not renumber type 520 or any earlier body.

The published artifact facets are currently one implementation behind Rust: TS/GraphQL/Proto/JSON still
end at geometry dependency and omit type 520. Their atomic append must therefore reserve the already-live
type-520 facet position before type 547:

- Proto: add type-520 body arm 11, then `DwgDynamicBlockProxyNode dynamic_block_proxy_node = 12`;
- TypeScript: add the complete shared evaluation value/expression, type-520 component and proxy-node
  wrapper, then both discriminated body arms in Rust order;
- JSON: add strict shared evaluation definitions and both closed body alternatives with
  `additionalProperties: false`;
- GraphQL: add exact evaluation alternative wrappers/union, type-520 component, proxy-node wrapper, then
  both union members. Use a lossless handle scalar for object reference, not `Float`.

Snapshot and diff duplicated facets require the same append or canonical artifact references. Mutation
reaches the proxy node through `SetSnapshot`. The opaque grammar/protocol leaves documented in P13 still
prevent full cross-language persistence acceptance.

### Corrections required before sharing the type-520 codec

1. `DwgEvaluationExpressionValue::from_value` at snapshot lines 1149-1161 does not reject inactive extra
   payload fields. An `Empty` value can currently hide arbitrary sibling values. Enforce exactly one
   discriminator and its one active payload, or discriminator alone for `Empty`, before type 547 uses it.
2. Factor `encode_evaluation_expression` and `decode_evaluation_expression` helpers. They must accept an
   optional bounded string reader and class-handle reader: type 520 always adds grip-expression text,
   whereas the fixture type 547 has no string stream at all.
3. Require exact absence/presence: the singleton must have absent-string marker zero, no class handle and
   exactly six terminal ones. Do not silently accept a zero-length present string stream.
4. Keep `PointGroup10` and `PointGroup11` distinct and validate two finite coordinates at the shared
   schema/codec boundary, not only in the type-520 writer.
5. Reject zero `ObjectReference`, unknown value codes, non-finite numerics, extra strings/handles and any
   residue before terminal fill. The singleton's empty branch does not waive the other union laws.

### Existing-test extension

Add the type-547 assertions to the existing AC1024 IO/lifecycle test family:

1. decode exactly one typed proxy-node body at `0x1164` and assert the complete logical expression above;
2. independently encode the frame and compare all 23 bytes, including prefix, six fill bits and CRC;
3. require shared evaluation DSL/pack round trips and rejection of `Empty` plus an inactive payload;
4. mutate parent, versions, node and each evaluation alternative, run diff/op/apply/inverse, then recover
   the exact original frame;
5. assert the proxy node never populates generic `referenced_handles`, raw, tail, unknown or source bytes;
6. require every artifact/snapshot/diff/mutation-reachable facet to contain both the type-520 and type-547
   symbols before declaring parity.

The implementation sequence is shared evaluation strictness/helper extraction, proxy-node schema/body
tag, bounded singleton decoder, exact-frame verifier, writer/export route, facets, then lifecycle laws.

## P16 — Fixed `BLOCK` / `ENDBLK` Post-Authority Readiness (Read-Only, 2026-08-14)

This pass reconciles the earlier exact-frame probe with the live handle-keyed object authority after the
independent geometry vector was removed. Primary layout evidence remains LibreDWG `dwg.spec`: AC1024
`BLOCK` declares one `BLOCK_NAME (name, 2)` and common entity handles; `ENDBLK` declares no class field
and only common entity handles. The ticket's `🧪️dwg-block-endblk-frame-probe.py` was rerun read-only and
still proves 10/10 type-4 and 10/10 type-5 frames with bounded main/string/handle streams and valid native
CRC. No production file and no Nx/Cargo target was touched by this pass.

### Sole-authority logical shape

The native BLOCK name and BLOCK_HEADER entry name are two encodings of one logical block name. They are
equal for ordinary blocks, but AC1024 anonymous blocks may store only the anonymous family prefix in the
header and the full generated name in the BLOCK marker. The post-authority shape should therefore be:

```text
DwgBlockBeginEntity { common: DwgEntityCommon }
DwgBlockEndEntity   { common: DwgEntityCommon }
DwgEntityBody::{BlockBegin(DwgBlockBeginEntity), BlockEnd(DwgBlockEndEntity)}
```

On import, decode both names transiently and reconcile them with the anonymous-name rule in P18; persist
the resulting full logical name only in the block header. On export, derive both native spellings from
that logical name and the header's anonymous flag. For explicit-owner markers, additionally require
`marker.owner_handle == header.handle`. For model-space/paper-space markers, the native owner slot is
absent and the header is selected by the block control's model/paper role plus
`block_entity_handle/end_block_entity_handle`. This keeps handle-keyed object bodies authoritative and
prevents a header-name mutation from leaving a stale BLOCK name.

`BlockBegin` and `BlockEnd` are semantic entity bodies even though they do not project geometry.
`DwgLogicalDrawing::entities()` must return `None` for both marker variants, while retaining them in
`drawing.objects`; do not synthesize empty geometry and do not fall back to an identity-only object.

### Complete AC1024 stream and handle order

Both frames use BOT, self handle, typed EED, then the live common entity main order:

1. graphic-present `B` and typed graphic payload when present;
2. entity mode `BB`, reactor count `BL`, xdictionary-missing `B`;
3. ENC color, linetype scale `BD`, linetype/plotstyle/material `BB` selectors;
4. shadow `RC`, full/face/edge visual-style presence `B`, invisibility `BS`, lineweight `RC`;
5. no class-main scalar for either marker;
6. BLOCK only: one full logical-name TU in the independent R2010 string stream, followed by
   derived `RS(string_bits)` and present `B = 1`; ENDBLK: absent-string `B = 0`;
7. handle stream: optional color handle, owner only for explicit-owner mode, reactors, optional xdic,
   layer, then conditional linetype/material/shadow/plotstyle/full/face/edge visual-style handles;
8. derived all-one terminal fill to byte alignment, outer `MS(payload bytes)`, `UMC(handle bits)`, and
   little-endian CRC-16 with seed `0xC0C1` over `MS + UMC + payload`.

The live common writer at IO lines 3247-3311 already has the correct fixture role order and relative
owner policy. In this cohort, BLOCK owner codes are `(8,5)` on five frames and `(12,5)` on three;
ENDBLK uses `(12,5)` on all eight explicit frames; both implicit-space pairs use only layer code 5.
Code 8 means the owner is the immediately preceding handle, and code 12 carries the positive backward
delta. These native codes are serializer decisions, not schema fields.

Fixture invariants shared by both cohorts are exact and exhaustive: BOT selector 0, self-handle code 0,
empty EED, no graphics, zero reactors, no xdic, ByLayer color 256, linetype scale 1, ByLayer selectors,
shadow/visual/invisibility zero, lineweight 29 and layer `0x10` on all 20. Modes are eight explicit, one
paper-space and one model-space in each cohort. The eight explicit header identities are `0x238`,
`0x110d`, `0x1145`, `0x195a`, `0x1f57`, `0x1fa4`, `0x201e`, `0x2077`; the two implicit identities are
the model/paper block-control roles. Each identity has exactly one begin and one end marker.

| Variant | Exact frame signatures | String / terminal fill |
| --- | --- | --- |
| BLOCK | payload 22/35/38/40/47/51; total 27/40/43/45/52/56; handle bits 20/28/36 | ten names; string bits 58/154/202/250/282; exactly `1111` fill |
| ENDBLK | payload 11 or 14; total 16 or 19; handle bits 22 or 38 | no string stream; exactly `111111` fill |

The ten derived BLOCK names are `*Model_Space`, `*Paper_Space`, `_ArchTick`, `Door - Imperial`,
`Window - Imperial`, `_ClosedBlank`, `*U4`, `*U5`, `*U6`, `*U7`. Exact CRC oracles remain those in
the ticket implementation report: BLOCK `bb32 fe36 cdca 6f49 e4cf 2ae5 1de9 1ac5 3505 9503` and
ENDBLK `d3ad 39a4 9d86 15e8 51cc de08 0b5e 2394 e0d4 7c9c`, in the probe's frame-row order.

### Live implementation delta

1. Snapshot Rust lines 958-1013 define only LINE/ARC/LWPOLYLINE. Add the two common-only structs and
   append entity-body ordinals 3/4 with DSL payload fields 4/5. The outer
   `DwgLogicalObjectBody::Entity` stays ordinal 4 / field 5; BLOCK/ENDBLK must not consume new outer-body
   tags.
2. IO lines 3999-4124 only decode those three geometry entities, so type 4/5 currently leave an entity
   with `body = None`. Add bounded branches before that fallthrough: common main, BLOCK string split or
   ENDBLK false marker, common handles, exact all-one terminal fill, then the tagged body.
3. Add sibling `encode_r2010_block_frame` and `encode_r2010_endblk_frame`. Reuse the common main/handle
   helpers and `finish_r2010_object_frame`; only BLOCK needs a local string writer. Resolve the header
   before encoding so the native TU is derived, and reject missing/ambiguous/mismatched graph edges.
4. `DwgLogicalDrawing::entities()` at snapshot lines 1475-1496 is exhaustive over geometry bodies.
   Add marker arms that return `None` from the projection without deleting the underlying object.
5. Add exact verifiers through the existing generic fixed-entity verifier, but make BLOCK encoding accept
   the drawing/header lookup context. Count assertions are exactly 10 and 10, never merely nonzero.

### Required schema-first facet arms

- Canonical TypeScript: add `DwgBlockBeginEntity` and `DwgBlockEndEntity`, then append
  `{ kind: 'blockBegin' ... }` and `{ kind: 'blockEnd' ... }` to `DwgEntityBody`.
- Rust structured DSL/pack: append discriminants 3/4 and payload fields 4/5; both payload records contain
  only `common`. Diff/apply/inverse/absorb and SetSnapshot mutation then retain markers structurally.
- GraphQL's currently flattened object-body union must append the two concrete entity types. Proto's
  flattened body oneof already ends at associative-variable tag 13, so append `block_begin = 14` and
  `block_end = 15` (or introduce a nested `DwgEntityBody` atomically without renumbering existing arms).
- Canonical JSON's entity alternative must add two closed, discriminated variants. A common-only BLOCK
  and common-only ENDBLK are structurally identical without the discriminator; `oneOf` over bare record
  shapes is therefore invalid.
- Snapshot GraphQL/Proto duplicate the canonical definitions and need the same arms; snapshot TS/JSON
  inherit/reference canonical artifacts. Diff GraphQL/Proto duplicate them too, while diff TypeScript
  currently omits the entire entity alternative and must be reconciled atomically. Mutation remains
  SetSnapshot-based but every referenced snapshot facet must expose the two tags.
- ABNF/Kaitai/Spicy/protocol and text grammar must name the structured variants rather than claiming an
  opaque/native body. Add anti-shadow assertions forbidding raw frame, string-stream, handle-code,
  terminal-fill, prefix/size and CRC fields.

### Exact acceptance extension

Extend the existing AC1024 lifecycle tests, not a new test file: decode all 20 typed markers; prove the
10 header/begin/end bijections and two implicit-space roles; compare every independently encoded frame
including `MS/UMC`, fill and CRC; mutate a header name and prove both native spellings change coherently;
inverse it to exact original bytes; reject orphan, duplicate, invalid anonymous-family/index and
cross-header pairs. Carry the exact
fixture bytes through structured DSL, pack, diff/apply/inverse/absorb, mutation/inverse, analyzer and
composer. The gate is not complete until the derived geometry projection excludes all 20 markers while
the handle-keyed object authority retains them.

## P17 — `DIMENSION_LINEAR` x14 Post-Authority Live Oracle (Read-Only, 2026-08-14)

This pass reconciles the fixed type-21 cohort with the block-local Handles correction and the live
handle-keyed entity authority. The historical probe saw 12 frames because it retained handle/address
bases across the second Handles block. The corrected inventory adds genuine DIMENSION_LINEAR objects
`0x2255` and `0x2266`, yielding **14**, not 12. The original bounded probe plus
`🧪️dwg-recovered-handle-block-frames-probe.py` validate all 14 main/string/handle boundaries, all-one
terminal fill and CRC-16. The two recovered frames use the same standard typed layout; no new variant,
raw tail or source state is required. No production source and no Nx/Cargo target was touched.

Primary field order is LibreDWG `COMMON_ENTITY_DIMENSION` in `dwg_spec_shared.h` followed by
`DIMENSION_LINEAR` in `dwg.spec`. For AC1024 the native declaration and the measured fixture agree.

### Sole-authority typed schema

Use a shared dimension aggregate inside the linear body, with the outer handle-keyed object retaining
the ordinary common entity relations:

```text
DwgDimensionStatus {
  block_reference_is_exclusive: bool, // standard group-70 bit 5
  user_positioned_text: bool,          // standard group-70 bit 7
}

DwgDimensionEntityCommon {
  common: DwgEntityCommon,
  extrusion: Point3,
  text_midpoint: Point2,
  elevation: f64,
  status: DwgDimensionStatus,
  user_text: String,
  text_rotation: f64,
  horizontal_direction: f64,
  insertion_scale: Point3,
  insertion_rotation: f64,
  attachment: DwgDimensionTextAttachment,
  line_spacing_style: DwgDimensionLineSpacingStyle,
  line_spacing_factor: f64,
  actual_measurement: f64,
  flip_arrow_1: bool,
  flip_arrow_2: bool,
  clone_insertion_point: Point2,
  dimension_style_handle: Handle<DimensionStyle>,
  dimension_block_handle: Option<Handle<BlockHeader>>,
}

DwgLinearDimensionEntity {
  dimension: DwgDimensionEntityCommon,
  extension_line_1: Point3,
  extension_line_2: Point3,
  definition_point: Point3,
  oblique_angle: f64,
  dimension_rotation: f64,
}
```

`DwgDimensionTextAttachment` and `DwgDimensionLineSpacingStyle` are named standard enums, not integers
with fixture-only validation. The fixed type code derives the linear subtype. Do not persist class
version, native `flag1`, low type nibble, reserved bit, compact selectors, string size/presence, native
handle codes, fill, frame sizes or CRC. `dimension_block_handle = None` is semantic even though the
native code-5 null slot is physically emitted. Do not mirror either class reference in the generic
`referenced_handles` vector.

The generic `DwgLogicalGeometry` vector must not become a second authority. Until there is a dedicated
typed dimension projection, `DwgLogicalDrawing::entities()` may omit DIMENSION_LINEAR exactly as it omits
non-geometric marker bodies, but `drawing.objects` must retain all 14 complete bodies. If a downstream
projection is added, derive it from `DwgLinearDimensionEntity` and never flatten it back into persisted
coordinates/text that can diverge.

### Exact AC1024 main and string order

After BOT, self handle, typed EED and the complete R2010 entity-common main prefix, class data is:

1. serializer-derived class version `RC = 0`;
2. extrusion `3BD`, text midpoint `2RD`, elevation `BD`;
3. serializer-derived `flag1 RC`;
4. user text `T` in the independent string stream;
5. text rotation `BD0`, horizontal direction `BD0`, insertion scale `3BD_1`, insertion rotation `BD0`;
6. attachment `BS`, line-spacing style `BS1`, line-spacing factor `BD1`, actual measurement `BD`;
7. serializer-derived reserved `B = 0`, flip-arrow-1 `B`, flip-arrow-2 `B`;
8. clone insertion point `2RD0`;
9. extension-line point 1 `3BD`, extension-line point 2 `3BD`, definition point `3BD`;
10. oblique angle `BD`, dimension rotation `BD0`;
11. the accumulated user-text TU, derived `RS(string_bits)` and present `B = 1`.

For this fixed linear subtype, derive:

```text
status = (exclusive ? 0x20 : 0) | (user_positioned ? 0x80 : 0)
flag1  = 0x08 | (status & 0xe0)
       | (user_positioned ? 0 : 1)
       | (exclusive ? 2 : 0)
```

Decode the inverse and reject disagreement between the high semantic bits and their low mirror bits,
the linear-invalid ordinate bit 6, a non-linear low subtype, or reserved combinations. All 14 fixture
statuses are default and therefore encode `flag1 = 0x09`.

All 14 take these measured branches: class version 0; extrusion selectors `(2,2,1)` for `(0,0,1)`;
elevation/text rotation/horizontal direction/insertion rotation/oblique angle zero branches; insertion
scale `(1,1,1)` via `3BD_1`; attachment 5; line-spacing style 1; line-spacing factor 1; measurement as
full RD; reserved/flip bits false; clone point as two full RDs; and all three class points via selectors
`(0,0,2)`. Dimension rotation is full RD on nine and default zero on five. Validate all coordinates and
angles as finite, standard attachment/style enum domains, and the standard positive line-spacing-factor
domain before writing.

The 14 semantic user texts are the original twelve plus the two recovered bindings:

```text
Room1=12'-0"  Room2=12'-0"  Room3=12'-0"  hall=12'-0"
Wall1=bldWALL Wall2=bldWALL Wall3=bldWALL bldDEPTH=50'-0"
bldWALL=6" iWALL1=6" iWALL2=iWALL1 iWALL3=iWALL1 iWALL4=iWALL1
bldWIDTH=40'-0"
```

Their string-bit histogram is `{154:1, 170:1, 186:1, 202:3, 218:6, 250:2}`. The UTF-16 units,
string-bit count and presence bit are deterministic serialization results, never logical fields.

### Handles, outer framing and complete fixture oracle

Every frame is model-space mode 2 and therefore has no owner slot. Common/class handle order is exactly:

1. one reactor, native code 4;
2. layer, native code 5, always `0x1b81`;
3. dimension style, native code 5, always `0x242` and required to resolve to fixed type 69;
4. anonymous dimension block, native code 5, null on all 14; a non-null mutation must resolve to a
   block header and satisfy its graph invariants;
5. exactly seven terminal one-fill bits.

The reactor is distinct per dimension. Recovered dimensions `0x2255` and `0x2266` point to dependency
objects `0x2258` and `0x2269`; those dependencies reciprocally target the dimensions and own typed
ASSOCDIMDEPENDENCYBODY objects. Preserve this graph as relations, not a generic reference bag.

All frames have BOT selector 0, self-handle code 0, empty EED, no graphic, one reactor via BL branch 1,
missing xdic, ByLayer color 256, linetype scale 1, ByLayer linetype/plot/material, shadow/visual/
invisibility zero, lineweight 29, `UMC = 87`, one present user-text stream and fill `1111111`.

| Payload / total | Data / class-end / string bits | Fixture handles |
| --- | --- | --- |
| 145 / 150 | 1073 / 870 / 186 | `2250` |
| 149 / 154 | 1105 / 870 / 218 | `2128 21ea 2255` |
| 149 / 154 | 1105 / 934 / 154 | `2151` |
| 151 / 156 | 1121 / 934 / 170 | `211d` |
| 153 / 158 | 1137 / 870 / 250 | `2266` |
| 155 / 160 | 1153 / 934 / 202 | `2156 2161 2177` |
| 157 / 162 | 1169 / 934 / 218 | `2122 215b 2166` |
| 161 / 166 | 1201 / 934 / 250 | `2107` |

The exact CRC oracle in handle order is:
`2107:1efc 211d:4db9 2122:fbf5 2128:b911 2151:e3e3 2156:a3b0 215b:f394
2161:9ad9 2166:7980 2177:26c6 21ea:36b9 2250:114f 2255:b1c7 2266:da31`.
CRC is little-endian on wire and is calculated with seed `0xC0C1` over `MS + UMC + payload`.

### Live implementation and facet delta

1. Live snapshot Rust lines 958-1013 still define only LINE/ARC/LWPOLYLINE. After P16's planned
   BlockBegin/BlockEnd arms, append `DimensionLinear` as entity-body ordinal **5**, payload field **6**;
   if implemented concurrently, reserve ordinals 3/4 for those marker arms rather than renumbering later.
   The outer `DwgLogicalObjectBody::Entity` ordinal/field remains unchanged.
2. IO's entity branch around lines 3999-4124 has no type-21 decoder, so every dimension currently exits
   with `body = None`. Add a bounded shared-dimension decoder plus the five linear suffix fields, exact TU
   exhaustion, common handles plus the two mandatory class slots, and seven-one terminal validation.
3. Add `encode_r2010_dimension_linear_frame`, reusing common main/handle helpers and the ordinary
   string-writer/`finish_r2010_object_frame` path. A null dimension block still calls
   `write_handle(5, 0)`. Add a 14-count exact verifier; any 12-count assertion is now an anti-acceptance
   bug because it silently misses the second Handles block.
4. Geometry projection code around snapshot lines 1475-1496 must handle the new enum arm explicitly;
   omit it from the generic projection or derive a dedicated typed view, but never clone persisted state.
5. Graph validation must cover style type, optional block type, unique reactor relationship and the two
   recovered dimension/dependency/dependency-body triples.

Required schema-first propagation is atomic:

- Rust structured DSL/pack: the shared-dimension record, named status/attachment/spacing types and the
  entity-body discriminant/payload above; diff/apply/inverse/absorb and SetSnapshot must retain all fields.
- Canonical TypeScript: append `dimensionLinear` to nested `DwgEntityBody`. Canonical JSON requires a
  closed discriminated alternative and fixed-length point arrays.
- GraphQL's flattened object-body union appends `DwgLinearDimensionEntity`. Proto currently ends at outer
  tag 13; reserve P16 marker tags 14/15 and append `dimension_linear = 16`, or introduce a nested entity
  oneof atomically without renumbering prior arms. Use lossless handle scalars/types, not `Float`.
- Snapshot GraphQL/Proto and diff GraphQL/Proto duplicate canonical bodies and need the same arm. Diff
  TypeScript already omits the complete entity alternative and must be reconciled, not patched with only
  DIMENSION_LINEAR. Mutation remains SetSnapshot-based but all referenced snapshot facets must carry it.
- Text grammar and ABNF/Kaitai/Spicy/protocol facets must name the structured dimension fields. Add
  anti-shadow assertions forbidding class-version/flag1/type-nibble/reserved/selectors/string-size/native
  handle codes/fill/frame-size/CRC/raw-tail/source fields.

Extend the existing AC1024 lifecycle test family: require 14 typed imports and 14 independent exact-frame
encodes; assert every stream boundary/CRC/role and the complete dependency graph; mutate/inverse user text,
measurement, a zero/full dimension-rotation branch, attachment/spacing, status mirrors and nullable block;
then recover original fixture bytes through DSL, pack, diff/apply/inverse/absorb, mutation/inverse,
analyzer and composer. Native exact equality, not a canonical intermediate, is the acceptance baseline.

## P18 — Anonymous BLOCK Name Reconciliation Correction (Read-Only, 2026-08-14)

The strict equality rule originally stated in P16 is wrong for this fixture's four anonymous blocks.
Independent bounded reads prove that the BLOCK_HEADER decoder is **not** truncating, splitting or
mis-ordering a string. Each affected header string stream contains exactly three exhausted TU values:
entry name `*U`, empty xref path and empty description. The linked BLOCK marker separately contains the
full name `*U4`, `*U5`, `*U6` or `*U7` and exhausts its own string stream. Both spellings are native.

The exact ten-name relation is:

| BLOCK_CONTROL role/index | Header / BLOCK handles | Anonymous | Native header entry | Native BLOCK name | Logical name |
| --- | --- | ---: | --- | --- | --- |
| model-space role | `1f / 20` | 0 | `*Model_Space` | `*Model_Space` | `*Model_Space` |
| paper-space role | `58 / 5a` | 0 | `*Paper_Space` | `*Paper_Space` | `*Paper_Space` |
| ordinary 0 | `238 / 23a` | 0 | `_ArchTick` | `_ArchTick` | `_ArchTick` |
| ordinary 1 | `110d / 1138` | 0 | `Door - Imperial` | `Door - Imperial` | `Door - Imperial` |
| ordinary 2 | `1145 / 116b` | 0 | `Window - Imperial` | `Window - Imperial` | `Window - Imperial` |
| ordinary 3 | `195a / 195b` | 0 | `_ClosedBlank` | `_ClosedBlank` | `_ClosedBlank` |
| ordinary 4 | `1f57 / 1f58` | 1 | `*U` | `*U4` | `*U4` |
| ordinary 5 | `1fa4 / 1fa5` | 1 | `*U` | `*U5` | `*U5` |
| ordinary 6 | `201e / 201f` | 1 | `*U` | `*U6` | `*U6` |
| ordinary 7 | `2077 / 2078` | 1 | `*U` | `*U7` | `*U7` |

The one BLOCK_CONTROL stores eight ordinary headers in exactly the index order shown, followed by the
separate model/paper roles. Thus the decimal suffix equals the ordinary entry's zero-based control index
for all four anonymous blocks. Their marker handle also happens to equal header handle plus one, but that
relationship carries no suffix information and must not be used to invent the name. ODA standard fields
establish the ordered BLOCK_CONTROL entries, the BLOCK_HEADER anonymous flag and the separate BLOCK
name; the fixture establishes the `*U{entry_index}` AutoCAD generation convention. Treat the index match
as an import invariant/canonical new-name policy, not as permission to discard an imported full name.

### Correct sole-authority normalization

1. Decode header entry name and BLOCK name into deserializer-local temporaries, then join by the explicit
   `block_entity_handle`; also validate the marker's explicit owner or model/paper role.
2. For `anonymous == false`, require exact header/BLOCK name equality.
3. For `anonymous == true`, require a full marker name matching `*<family><decimal-index>`, require the
   header entry to equal the two-character `*<family>` prefix, and require the decimal suffix to equal
   that header's ordinary BLOCK_CONTROL index. This fixture's family is `U`.
4. Persist only the full logical name (`*U4`…`*U7`) in `DwgBlockHeaderTableRecord.common.name`; the marker
   body remains common-only. The short native header prefix is not snapshot state.
5. On write, ordinary/model/paper headers emit the full logical name in both native locations. Anonymous
   headers derive the two-character family prefix for the BLOCK_HEADER entry and emit the full logical
   name in the BLOCK TU. For a newly created anonymous block without a supplied suffix, allocate the full
   name from its ordinary control index before snapshot acceptance; never assign a name from handle math.

The live table-record decoder around IO lines 4849-4897 correctly reads `*U`; its error is only that it
immediately commits that native prefix to `common.name` before graph reconciliation. The live table-record
writer around lines 2949-3023 likewise writes `common.name` verbatim and would emit `*U4` into the header,
breaking exact bytes after normalization. Required correction is a post-decode block-graph normalization
pass plus a block-aware native header-name projection in the writer. Do not weaken string exhaustion and
do not store `native_header_name`, anonymous suffix, control index copy or source bytes.

Acceptance must assert this complete 10-row mapping, not universal string equality. Mutating logical
`*U4` to another valid family/index must update the derived header prefix and marker TU together; inverse
must restore both exact frames. Reordering BLOCK_CONTROL ordinary entries must either deterministically
rename affected generated anonymous blocks as one validated graph mutation or reject atomically—never
leave suffix/index disagreement.

## P19 — VISUALSTYLE Type 506 Live Readiness Oracle (Read-Only, 2026-08-14)

The live AC1024 tree has no type-506 body yet. `DwgLogicalObjectBody` ends at
`AssociativeDimensionDependencyBody`; IO has no `type_code == 506` decoder branch, writer or exact-frame
verifier; and no canonical/snapshot/diff facet names `VisualStyle`. This is a clean append-only addition,
not an extension of `DwgEntityBody`: `VISUALSTYLE` is a common object stored in `ACAD_VISUALSTYLE`.

The field taxonomy below follows LibreDWG's stable R2007+ `VISUALSTYLE` prescription and Autodesk's
[VISUALSTYLE DXF table](https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-DXF/files/GUID-8A8BF2C4-FC56-44EC-A8C4-A60CE33A530C.htm),
[visual-style types](https://help.autodesk.com/cloudhelp/2024/ENU/OARX-ManagedRefGuide/files/OARX-ManagedRefGuide-Autodesk_AutoCAD_GraphicsInterface_VisualStyleType.html),
and [property operations](https://help.autodesk.com/cloudhelp/2024/ENU/OARX-ManagedRefGuide/files/OARX-ManagedRefGuide-Autodesk_AutoCAD_GraphicsInterface_VisualStyleOperation.html).
For AC1024, accept only the R2010b 28-property branch. Do not read/write the pre-R2010 record and do not
emit the R2013 `num_props` plus 58-property extension.

### Code-ready logical body

Add a `DwgVisualStyle` record containing `description`, typed `style_type`, typed
`extension_lighting_model`, `internal_only`, and one fixed `DwgVisualStyleProperties` record. Model each
of the following as `DwgVisualStyleProperty<T> { value, operation }`; there is no property map, property
count, parallel operation array or native selector state.

| # | Named property | Logical value type | Native AC1024 value |
| ---: | --- | --- | --- |
| 1 | face lighting model | enum `Invisible=0, Constant=1, Phong=2, Gooch=3` | `BL` |
| 2 | face lighting quality | enum `NoLighting=0, PerFace=1, PerVertex=2, PerPixel=3` | `BL` |
| 3 | face color mode | enum `NoColor=0, Object=1, Background=2, Custom=3, Mono=4, Tinted=5, Desaturated=6` | `BL` |
| 4 | face modifiers | flags `Opacity=1, Specular=2` | `BS` |
| 5 | face opacity | finite normalized scalar | `BD` |
| 6 | face specular amount | finite scalar | `BD` |
| 7 | face monochrome color | typed `DwgComplexColor` | `CMC` |
| 8 | edge model | enum `NoEdges=0, Isolines=1, FacetEdges=2` | `BL` |
| 9 | edge styles | flags `Visible=1, Silhouette=2, Obscured=4, Intersection=8` | `BL` |
| 10 | edge intersection color | typed `DwgComplexColor` | `CMC` |
| 11 | edge obscured color | typed `DwgComplexColor` | `CMC` |
| 12 | edge obscured line pattern | enum `Solid=1` through `SparseDot=11` | `BL` |
| 13 | edge intersection line pattern | same typed line-pattern enum | `BL` |
| 14 | edge crease angle | finite degrees, `-360..=360` | `BD` |
| 15 | edge modifiers | flags `Overhang=1, Jitter=2, Width=4, Color=8, HaloGap=16, AlwaysOnTop=64, Opacity=128` | `BL` |
| 16 | edge color | typed `DwgComplexColor` | `CMC` |
| 17 | edge opacity | finite normalized scalar | `BD` |
| 18 | edge width | typed standard width amount | `BL` |
| 19 | edge overhang | typed standard overhang amount | `BL` |
| 20 | edge jitter | typed jitter amount | `BL` |
| 21 | edge silhouette color | typed `DwgComplexColor` | `CMC` |
| 22 | edge silhouette width | typed standard width amount | `BL` |
| 23 | edge halo gap | typed percentage/amount | `BL` |
| 24 | edge isolines | count, `0..=5000` | `BL` |
| 25 | hidden-edge precision | boolean | `B` |
| 26 | display settings | flags `Backgrounds=1, Lighting=2, Materials=4, Textures=8` | `BL` |
| 27 | display brightness | finite scalar | `BD` |
| 28 | display shadow type | enum `None=0, GroundPlane=1, Full=2, FullAndGround=3` | `BL` |

`DwgVisualStylePropertyOperation` is the standard operation enum `Inherit=0`, `Set=1`, `Disable=2`,
`Enable=3`; reject `Invalid=-1` and any other value. The fixture uses only 0, 1 and 2. Keep the operation
attached to its named property because Autodesk's API sets/gets it per property. The three override
styles specifically prove that a body-wide operation is incorrect.

The current `DwgComplexColor { index, rgb, name, book_name }` is not sufficient for strict semantic
authority because `rgb` still contains the packed method byte. Refactor or wrap it so method and value
are named: at minimum `ByColor { red, green, blue }`, `ByAci { index }`, and `None`, plus optional color
and book names. The writer derives the native BS index, BL method/value word and flags. All five color
slots in these frames have native BS index zero, flag zero and no name/book strings. Observed method/value
words are `c2ffffff`, `c2808080`, `c3000007`, `c8000000`, `c2787878`; never persist those packed words.

### Exact AC1024 stream order and fixture invariants

The frame begins with BOT selector 1, self handle wire code 0 and empty EED. Common-object main data is
`reactor_count BL = 1`, then `xdic_missing B = true`. Class main data is, in order:

1. `style_type BL`, `extension_lighting_model BS`, `internal_only B`;
2. the 28 table rows above, with each native `operation BS` emitted immediately after its value;
3. no further R2010 class fields.

The separate string stream contains exactly one TU, `description`, even though the native declaration
places it first. The handle stream contains exactly common `owner` then `reactor[0]`; both resolve to
`0x99`. There is no extension-dictionary handle because the missing bit is true and there are no
class-local handles. Native role-code pairs are owner/reactor `(12,4)` on 15 frames, `(4,4)` on three
override frames and `(8,4)` on handle `0x9a`. Relative/absolute codes are serializer decisions derived
from semantic handles, not persisted state. Every frame has zero terminal fill bits.

All 19 objects have extension lighting model 2. `internal_only` is true on 14 and false on five. Names
are `2dWireframe`, `3D Hidden`, `3dWireframe`, `Basic`, `Brighten`, `ColorChange`, `Conceptual`, `Dim`,
`EdgeColorOff`, `Facepattern`, `Flat`, `FlatWithEdges`, `Gouraud`, `GouraudWithEdges`, `JitterOff`,
`Linepattern`, `OverhangOff`, `Realistic`, `Thicken`; style types are `0..9`, `11..16`, `20..22`, once
each. Sixteen built-ins use `Set=1` on every property. `JitterOff` (`0x323`), `OverhangOff` (`0x324`) and
`EdgeColorOff` (`0x325`) use `Inherit=0` everywhere except edge modifiers, which use `Disable=2`.

The complete semantic value histogram is:

- face model `2:14,1:2,0:2,3:1`; quality `2:16,1:2,0:1`; color mode `1:14,0:3,2:1,3:1`;
  face flags `0:15,2:4`; opacity `0.6:19`; specular `30:19`;
- edge model `1:14,0:3,2:2`; style flags `4:16,2:2,0:1`; obscured pattern `1:17,2:1,7:1`;
  intersection pattern `1:18,7:1`; crease `1:17,40:2`; edge flags `8:10,0:6,12:1,10:1,9:1`;
- edge opacity/width/overhang/jitter are `1/1/6/2` on all 19; silhouette width `5:17,3:2`; halo,
  isolines and hidden precision are `0/0/false` on all 19;
- display flags `1:18,13:1`; brightness `0:17,-50:1,50:1`; shadow type `0:19`.

Exact signatures (`payload/total : handle-bits,data-bits,class-end,string-bits : handles`) are:

```text
103/108:32,792,621,154:323       107/112:32,824,621,186:324
109/114:32,840,621,202:325       118/123:24,920,829,74:9a
120/125:32,928,821,90:9e         125/130:32,968,829,122:9c
126/131:32,976,837,122:a6        126/131:32,976,901,58:a4
129/134:32,1000,829,154:a3       132/137:32,1024,821,186:9f a0
134/139:32,1040,837,186:a7 a8 a9 136/141:32,1056,901,138:a5
137/142:32,1064,893,154:a1       138/143:32,1072,837,218:9b
140/145:32,1088,901,170:a2       144/149:32,1120,837,266:9d
```

Stored CRC oracle:
`9a:767e 9b:805c 9c:5e7e 9d:95cb 9e:8fd4 9f:abe5 a0:752d a1:9981 a2:d4e1 a3:336e
a4:11c1 a5:a320 a6:29a4 a7:8d81 a8:3b92 a9:6afd 323:6f8f 324:54da 325:6edf`.

### Append-only propagation and strict gates

Reserve the current next outer-body slot for the earlier-scheduled type-559 thin body described in P20;
then append `VisualStyle(DwgVisualStyle)` as outer body ordinal 13 and payload field 14, retaining every
existing ordinal/field ID. The flattened Proto oneof now uses tags 15/16/17/18 for
BLOCK/ENDBLK/INSERT/DIMENSION_LINEAR and P20 reserves 19, so append `visual_style = 20`. Append the named record and arm to
canonical and snapshot TypeScript/GraphQL/Proto/JSON, and to every duplicated diff facet. Mutation is
SetSnapshot-based, but its referenced snapshot facets and structured DSL/pack must carry the new arm.
ABNF/Kaitai/Spicy/protocol/grammar facets must describe the same named fields and never a property blob,
packed CMC, native body, raw tail or source bytes. Diff TypeScript currently also omits the complete
`Entity` arm; reconcile that existing drift atomically rather than copying another incomplete union.

Decoder acceptance requires exact exhaustion of main, string and handle streams, all range/enum/flag
checks above, and exactly 19 typed bodies in this fixture. Writer acceptance requires 19 independently
byte-identical frames and the exact signature/CRC oracle; derive compact BL/BS/BD selectors, handle wire
codes, string-size metadata, fill and CRC from logical values. Add anti-shadow assertions forbidding
property count/map, parallel operations, packed color words, selectors, stream boundaries, handle codes,
fill, CRC, class/body raw data and source bytes in Rust plus every facet.

Extend the existing AC1024 lifecycle test rather than adding a file: assert all names/types/value and
operation histograms, owner/reactor graph and 19 frame identities; mutate/inverse one enum, one scalar,
one color method/value, one flag set and one operation; then recover the original fixture through native
encode, structured DSL, binary pack, diff/apply/inverse/absorb, mutation/inverse, analyzer and composer.
The baseline is the original DWG bytes, never a canonical intermediate.

## P20 — ACDB_BLOCKREPRESENTATION_DATA Type 559 Live Oracle (Read-Only, 2026-08-14)

The live type assignment is dynamic class 559, `ACDB_BLOCKREPRESENTATION_DATA`, with twelve fixture
objects. LibreDWG's
[`BLOCKREPRESENTATION` prescription](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L2392-L2400)
declares only common `AcDbObject`, one `flag BS` and one block reference at DXF group 340. It does not
inherit BlockElement, EvalExpression, AssocAction, AssocDependency or any other dynamic-block core.
`HANDLE_UNKNOWN_BITS` in that source is the already-rejected non-consuming replay macro, not an inherited
field. Therefore the exact logical body is only:

```text
DwgBlockRepresentationData {
  represented_block_header_handle: u64
}
```

The native `flag BS` is 1 in all twelve frames and is a closed AC1024 class marker/version. Validate it
on import and derive 1 on export; do not persist or expose `flag`, `version`, its compact selector or a
generic scalar bag. The target is a required hard pointer (DXF 340) to a `BLOCK_HEADER`, not the soft
pointer claimed by P11's earlier matrix. The fixture's actual wire nibble is 5 on all twelve frames.

### Inherited common core and exact stream order

Every frame starts with BOT selector 1, self handle wire code 0 and empty EED. The inherited common-object
main stream is `reactor_count BL = 1`, then `xdic_missing B = true`; AC1024 has no data-store bit. The
class main stream is exactly derived `flag BS = 1`. There are no class strings and the R2007 string-stream
presence bit is false. Handle order is:

1. common owner, wire code 8 with zero payload, resolving to `self - 1`;
2. common reactor 0, wire code 4 absolute, equal to the owner;
3. represented block-header hard pointer, wire code 5 absolute.

There is no extension-dictionary handle and no other inherited or class-local handle. All twelve owners
and reactors resolve to the immediately preceding type-42 `DICTIONARY`; the representation relationship
itself must remain in the typed body rather than `referenced_handles`. Represented targets are block
header `0x110d` on four frames and block header `0x1145` on eight. Exact graph rows are:

| representation | owner = reactor dictionary | represented block header |
| ---: | ---: | ---: |
| `1f40` | `1f3f` | `110d` |
| `1f8d` | `1f8c` | `110d` |
| `1fe0` | `1fdf` | `110d` |
| `1ffa` | `1ff9` | `110d` |
| `2014` | `2013` | `1145` |
| `206d` | `206c` | `1145` |
| `20aa` | `20a9` | `1145` |
| `20b7` | `20b6` | `1145` |
| `20c4` | `20c3` | `1145` |
| `20d1` | `20d0` | `1145` |
| `20de` | `20dd` | `1145` |
| `20eb` | `20ea` | `1145` |

All frames share one native signature: prefix 3 bytes, payload 15 bytes, total frame 20 bytes; handle
stream 62 bits; handle start/data boundary bit 58; class main ends at bit 57; no string payload; handle
roles/codes `(owner:8, reactor:4, represented-block:5)`; six derived terminal one-fill bits `111111`.
That one-bit gap between class end 57 and handle start 58 is the false string-stream-presence footer,
not body padding.

CRC oracle:
`1f40:a900 1f8d:eabc 1fe0:0cc1 1ffa:0795 2014:9b73 206d:b0a7
20aa:348e 20b7:fea7 20c4:d073 20d1:dde7 20de:1a4e 20eb:c9e7`.

### Live append-only and lifecycle checklist

The live Rust outer `DwgLogicalObjectBody` still ends at ordinal 11 / payload field 12. Append
`BlockRepresentationData(DwgBlockRepresentationData)` at ordinal 12 / payload field 13 without changing
the nested entity discriminator. The flattened canonical/snapshot/diff Proto oneof now ends with
`dimension_linear = 18`; append `block_representation_data = 19`. This reservation supersedes P19's
earlier pre-DIMENSION tag suggestion and leaves VISUALSTYLE the following outer/tag slots. Add the same
named record and union arm to canonical and snapshot Rust/TypeScript/GraphQL/Proto/JSON, every duplicated
diff facet, and every structured DSL/binary/grammar/ABNF/Kaitai/Spicy/protocol description. Mutation is
SetSnapshot-based but must transit the body through its referenced snapshot codecs.

Decoder acceptance requires all twelve bodies, flag 1, exact three-stream exhaustion, exact graph rows,
required non-null type-49 target and no body fallback. Writer acceptance requires twelve independently
byte-identical frames, including the common relation roles, false string footer, six one-fill bits and
per-handle CRCs. Derive the native class marker, selectors, wire codes, boundaries, fill and CRC.

Extend the existing AC1024 lifecycle test: assert 12/12 typed imports and exact frames; mutate one
represented block from `0x110d` to the other valid `0x1145` target and inverse it; verify graph validation
rejects null/non-BLOCK_HEADER targets; and recover original fixture bytes through native encode,
structured DSL, binary pack, diff/apply/inverse/absorb, SetSnapshot mutation/inverse, analyzer and
composer. Anti-shadow facets must forbid `flag`, class version, raw/unknown body, source frame, native
handle code, footer, boundary, fill and CRC fields.

## P21 — Post-Entity/Dependency/VISUALSTYLE Facet and Codec Parity Refresh (Read-Only, 2026-08-14)

This is a static audit of the live tree after `Entity`, dependency-core and `VisualStyle` landed. No
production file was edited and no Cargo/Nx command was run. The Rust snapshot plus its explicit
`DslField` record specifications are the current authority. No immediate Rust syntax blocker was found
by inspection; the release blockers below are semantic facet drift and normative codec descriptions
which accept opaque tails instead of the committed structured representation.

### Authority and append-only correction

The live outer Rust `DwgLogicalObjectBody` order is now:

```text
ordinal 0..4: Dictionary, TableControl, TableRecord, XRecord, Entity
ordinal 5..11: AssociativeDependency, AssociativeValueDependency,
               AssociativeGeometryDependency, BlockGripLocationComponent,
               DynamicBlockProxyNode, AssociativeVariable,
               AssociativeDimensionDependencyBody
ordinal 12 / payload field 13: VisualStyle
```

The nested `DwgEntityBody` remains `Line=0/field1`, `Arc=1/field2`, `LwPolyline=2/field3`,
`BlockBegin=3/field4`, `BlockEnd=4/field5`, `Insert=5/field6`, and
`DimensionLinear=6/field7`. This live assignment supersedes the prospective slot statements in P19 and
P20: the flattened Proto body currently ends at `dimension_linear = 18`, so `visual_style` must append
as tag 19. The not-yet-live type-559 body must append after it as Rust outer ordinal 13 / field 14 and
flattened Proto tag 20. Never renumber the live Rust discriminants or existing Proto tags.

### Exact remediation matrix

| Priority | Surface | Live evidence | Exact cleanup |
| --- | --- | --- | --- |
| P0 | Rust semantic authority, `snapshot/🦀️component.rs:1402-1475` | `VisualStyle` is structurally tagged and its 28 properties are named, but `style_type`, `extension_lighting_model`, lighting/color/edge models, line patterns, modifiers, display settings and shadow type remain generic `u32/u16` values | Close these standard concepts as enums/bitflags before treating any duplicated numeric facet as final. Retain the generic `DwgVisualStyleProperty<T>` wrapper and closed `Inherit/Set/Disable/Enable` operation. Do not introduce a property map, packed CMC value or raw selector. |
| P0 | Canonical/snapshot/diff Proto, `🛰️component.proto:5,21-22,37-38` in each duplicated graph | No `DwgComplexColor*`, `DwgVisualStyle*`, `visual_style`, EED, or seven optional entity role handles; table bodies are still coarse | Add the closed complex-color variants and named visual property record, append `visual_style = 19`, append `DwgEntityCommon` tags 11..17 in Rust field order, add `DwgExtendedEntityData` and append `extended_data = 10` to `DwgLogicalObject`. Replace coarse table bodies with tagged control/record messages. Preserve tags 1..18. |
| P0 | Canonical JSON, `🔣️component.json:23-85` | The body union ends at `associativeDimensionDependencyBody`; visual style and EED are absent; entity common ends at `layerHandle`; table control/record are collapsed; header/classes/dependencies/summary/application/template are unconstrained generic objects | Add strict `$defs` for all Rust semantic records, every complex-color arm and the visual arm; add all role handles and `extendedData`; replace generic top-level object definitions and coarse table definitions; use closed `oneOf` records with `additionalProperties: false`. Snapshot JSON inherits drawing through this file, so repair authority here first. |
| P0 | Diff TS/GraphQL/Proto/JSON, `diff/🟦️component.ts:37-44`, `diff/🔗️component.graphql:54-56`, `diff/🛰️component.proto:38-44`, `diff/🔣️component.json:5-10` | Rust `DwgDiff` has ten optional replacements, but every language facet exposes only `version`, `maintenanceVersion`, `codepage`, `drawing`. Diff TS also omits the entire `{kind:'entity'; value:DwgEntityBody}` arm and therefore cannot carry Line/Arc/LWPOLYLINE/BLOCK/ENDBLK/INSERT/DIMENSION bodies | Add `header`, `classes`, `dependencies`, `summary`, `application`, `template` without changing the existing four field/tag identities. Add the TS entity arm and reuse/import the canonical graph instead of maintaining a second partial copy. A diff route is not accepted until a non-empty value in each of all ten fields survives both text and binary diff codecs. |
| P1 | Canonical TS, `🟦️component.ts:23-46` | Closest duplicated facet: all current dependency and visual body arms plus seven entity role handles exist. It still reduces `DwgTableControlBody` to three handle collections and `DwgTableRecordBody` to `{name}`, and omits `DwgExtendedEntityData`/`extendedData` | Replace the two coarse table records with tagged unions matching Rust control and record variants; add typed EED and `extendedData`. After Rust closes visual enums/flags, replace the affected `number` types one-for-one. |
| P1 | Canonical/snapshot/diff GraphQL, `🔗️component.graphql:3,26-27,44-54` in each duplicated graph | Visual style is present, but operations/status/reference modes/attachments/evaluation kinds remain strings; `DwgComplexColorValue` has only `None/ByColor/ByIndex`, losing `ByLayer`, `ByBlock`, `ByAci` versus `ByPen`, `Foreground`, `LayerOff`, and `LayerFrozen`; XRECORD integer widths are merged; seven role handles and EED are absent; table bodies remain coarse | Use closed GraphQL enums and distinct object arms for every Rust tagged variant. Add all role handles and EED. Replace table bodies with their tagged variants. Keep entity flattening only as a GraphQL union necessity; it must still represent every nested Rust entity arm exactly once. Prefer one generated canonical graph over the three copies. |
| P1 | Snapshot TS/JSON | `snapshot/🟦️component.ts:1-3` reexports canonical TS and snapshot JSON references canonical drawing | No separate body copy is needed. Fix the canonical authority, then assert that references resolve and validate a rich imported snapshot rather than adding parallel definitions. Snapshot GraphQL/Proto do duplicate the graph and require the P0/P1 repairs above. |
| P1 | Mutation facets, `mutations/🦀️component.rs:13-25`, `🟦️component.ts:1-5`, `🛰️component.proto:3-6`, `🔣️component.json:4-8`, `🔗️component.graphql:1` | Rust/TS/Proto/JSON correctly model `NoMutation`, `SetSnapshot`, and `SetVersionInfo`; GraphQL is one nullable field bag. JSON alternatives do not close additional properties | Replace GraphQL with three distinct union members and JSON with three closed alternatives. No body-specific mutation arm is required because `SetSnapshot` must transit the complete referenced snapshot graph. |
| P2 | Handles in GraphQL | Persisted `u64` handles are exposed as `Float` throughout | Introduce/use a lossless unsigned-64 scalar. A JavaScript numeric transport cannot exactly represent all DWG handles. |

The live Rust `DwgComplexColorValue` is already the correct closed nine-arm authority (`None`, `ByLayer`,
`ByBlock`, `ByColor`, `ByAci`, `ByPen`, `Foreground`, `LayerOff`, `LayerFrozen`) and must not be collapsed
again. The live `DwgExtendedEntityData { application_handle, values }` is also semantic state, not an
opaque native tail; every external schema must carry it as `DwgLogicalObject.extended_data`.

### Normative text/binary codec debt

The runtime Rust codecs are structurally derived (`DslRecord`, `DslDiff`, `DslOps` and the corresponding
record/variant binary codecs), but the committed normative files do not describe that structure:

| Family | ABNF | Kaitai | Spicy | protocol.semio | text grammar/facets |
| --- | --- | --- | --- | --- | --- |
| snapshot | `payload = *OCTET` | `payload size-eos` | `payload: bytes &eod` | `chain body bytes` | `snapshot-token*`; G4/EBNF describe only the schema header; text GraphQL/Proto expose `payload` |
| diff | `payload = *OCTET` | `payload size-eos` | `payload: bytes &eod` | `chain body bytes` | `diff-token*`; G4/EBNF describe only the header; text GraphQL/Proto expose `payload` |
| mutations | `payload = *OCTET` | `payload size-eos` | `payload: bytes &eod` | `chain body bytes` | `set-snapshot` falls back to `snapshot-token*`; G4/EBNF describe only the header; text GraphQL/Proto expose `payload` |
| inferences | same opaque ABNF/Kaitai/Spicy forms | same | same | `segment payload varint bytes` | grammar uses `payload = OCTET+`; text GraphQL/Proto expose `payload` |

Replace each wildcard/end-of-stream body with the actual recursive record encoding: record header,
field ordinal, field format, bounded scalar/list/record value and closed variant discriminator/payload.
The protocol documents must name that recursion instead of terminating at `bytes`; ABNF, Kaitai and
Spicy must be equivalent descriptions generated from or checked against the same field/discriminator
table. Text grammar must enumerate record fields/values and the three mutation productions rather than
accepting arbitrary tokens. `bytes` remains valid only for a named semantic byte field such as an
XRECORD binary group value, never for a complete snapshot/diff/op or an unknown body tail.

### Acceptance assertions for final cleanup

Extend the existing AC1024 tests rather than adding a file:

1. Add a positive symbol-parity table over Rust/TS/GraphQL/Proto/JSON for every current outer body arm,
   every seven entity role handles, `DwgExtendedEntityData`, all nine complex-color variants, all four
   property operations, all seven nested entity arms, every tagged table control/record arm, and all ten
   diff fields. Negative forbidden-term scans cannot detect an omitted semantic field.
2. Add normative-source rejection for `payload = *OCTET`, `size-eos`, `bytes &eod`,
   `chain body bytes`, `segment payload varint bytes`, `snapshot-token*`, `diff-token*`, and generic
   `payload` text fields. The current anti-shadow tests at IO lines 7070-7115 and 7236-7258 do not reject
   these constructs and inspect only a subset of the committed normative files.
3. Build one rich logical snapshot containing EED, all seven entity references, each nested entity arm,
   the table variants, dependency/value/geometry bodies, block grip, proxy node, variable,
   dimension-dependency body, every complex-color method and one `VisualStyle` with all 28 properties.
   Assert snapshot DSL/pack, diff text/binary/apply/inverse/absorb, SetSnapshot text/binary/inverse,
   analyzer and composer preserve equality and the exact original fixture export.
4. Validate the actual facet schemas against that same rich instance. Parseability alone is
   insufficient: Proto tag 19 must resolve to VisualStyle, JSON must reject a second body payload or an
   unknown property, GraphQL must distinguish all tagged color/XRECORD/evaluation variants, and a u64
   handle above `2^53` must survive every lossless facet.

Implementation order is: close remaining Rust visual enums/flags; repair canonical TS/GraphQL/Proto/JSON
and generate/reuse them for snapshot/diff; restore all ten diff fields and close mutations; replace the
normative opaque grammars/protocols; then add positive parity and rich lifecycle gates. This avoids
copying today's coarse numeric/table/color shapes into another supposedly authoritative facet.

## P22 — VIEWPORT Entity Type 34 Exact-Ready Live Oracle (Read-Only, 2026-08-14)

The existing bounded fixture probe was rerun read-only against `architectural_example.dwg`; it verifies
both stored CRCs with seed `0xC0C1` over the complete `MS + MC + payload` frame. No production source was
edited and no Cargo/Nx command was run. LibreDWG's current `VIEWPORT` prescription confirms the AC1024
main/string/handle order already recorded in P4. The live production audit finds no
`DwgViewportEntity`, no `DwgEntityBody::Viewport`, and no type-34 decoder/writer path: type 34 currently
appears only in the fixed-type name mapping. Therefore this cohort is exact-ready research, not live
typed support.

### Sole logical authority and code-ready body

Persist one `DwgLogicalObject` for each viewport with `category=Entity`, `type_code=34`,
`class_name="VIEWPORT"`, and `body=Entity(Viewport(...))`. The entity body is the sole persisted
authority; do not add a second viewport/entity collection to `DwgLogicalDrawing`. The existing derived
geometry projection must deliberately return no geometry for VIEWPORT rather than duplicating its
center/bounds. Common object state stays in `DwgLogicalObject`; common entity state stays in
`DwgEntityCommon`; only class-local concepts belong in the new body:

```text
DwgViewportEntity {
  common: DwgEntityCommon,
  center: Vec3, width: f64, height: f64,
  view_target: Vec3, view_direction: Vec3,
  twist_angle: f64, view_height: f64, lens_length: f64,
  front_clip: f64, back_clip: f64, snap_angle: f64,
  view_center: Vec2, snap_base: Vec2, snap_unit: Vec2, grid_unit: Vec2,
  circle_zoom_percent: u16, grid_major: u16,
  frozen_layer_handles: Vec<u64>,
  status: Set<DwgViewportStatusFlag>, style_sheet: String,
  render_mode: DwgViewportRenderMode,
  ucs_at_origin: bool, ucs_per_viewport: bool,
  ucs_origin: Vec3, ucs_x_axis: Vec3, ucs_y_axis: Vec3,
  ucs_elevation: f64, orthographic_view: DwgOrthographicView,
  shade_plot_mode: DwgShadePlotMode,
  use_default_lights: bool, default_lighting_type: DwgDefaultLightingType,
  brightness: f64, contrast: f64, ambient_color: DwgComplexColor,
  clip_boundary_handle: Option<u64>, named_ucs_handle: Option<u64>,
  base_ucs_handle: Option<u64>, background_handle: Option<u64>,
  visual_style_handle: Option<u64>, shade_plot_handle: Option<u64>,
  sun_handle: Option<u64>
}
```

Use a closed named status-flag set for DXF group 90, not a generic integer. The standard flags through
AC1024 are perspective, front clipping, back clipping, UCS follow, front clip not at eye, UCS icon
visible, UCS icon at origin, fast zoom, snap, grid, isometric snap, hide plot, iso-pair top,
iso-pair right, zoom lock, always-enabled, non-rectangular clipping, viewport off, grid beyond drawing
limits, adaptive grid, adaptive subdivision and grid-follows-workplane. Likewise type render,
orthographic, shade-plot and lighting values as closed standard enums. Native frozen count, compact
selectors, packed status integer, string size/presence, packed CMC, handle wire codes, stream boundaries,
fill and CRC are serializer state and must never appear in the snapshot.

### Exact main and string stream

After the accepted common-entity prefix, write the AC1024 class main stream in this exact order:

1. center `3BD`, width `BD`, height `BD`;
2. view target `3BD`, view direction `3BD`, twist `BD`, view height `BD`, lens length `BD`, front clip
   `BD`, back clip `BD`, snap angle `BD`;
3. view center, snap base, snap unit and grid unit as four `2RD`; circle zoom `BS`; R2007+ grid major
   `BS`;
4. derived frozen-layer count `BL`, status mask `BL`, style sheet `T`, render mode `RC`;
5. UCS-at-origin `B`, UCS-per-viewport `B`, UCS origin/X/Y `3BD`, UCS elevation `BD`, orthographic view
   `BS`, R2004+ shade-plot mode `BS`;
6. R2007+ default-lights `B`, lighting type `RC`, brightness `BD`, contrast `BD`, ambient `CMC`.

AC1024 includes snap angle and snap base; their omission is the AC1020/R2006-only branch. It has no
native `on_off`, viewport numeric ID, obsolete R13/R14 viewport-entity-header handle, or R2010-local
extension field. Both fixture frames take the same compact/default branches:

| Concept | Exact fixture value / branch on both frames |
| --- | --- |
| center | X/Y full `BD`, Z zero selector: `(0,0,2)` |
| width, height | full-RD `BD` selector 0 |
| target / direction | zero `(2,2,2)` / world Z `(2,2,1)` |
| twist, front clip, back clip, snap angle | zero selector 2 |
| view height / lens | full-RD selector 0; lens exactly 50 |
| view center / snap base | two full RDs; snap base `(0,0)` |
| snap unit / grid unit | `(0.5,0.5)` via full RDs |
| circle zoom / grid major | 1000 via BS selector 0 / 5 via BS selector 1 |
| frozen layers | empty, count BL selector 2 |
| render / UCS | render 0; at-origin false; per-viewport true |
| UCS frame | origin `(0,0,0)`, X `(1,0,0)`, Y `(0,1,0)` with selectors `(2,2,2)`, `(1,2,2)`, `(2,1,2)` |
| UCS elevation / ortho / shade plot | zero selectors 2 |
| lighting | default lights true, type 1, brightness and contrast zero selectors 2 |
| ambient color | `ByColor #333333`, no name/book; derive packed CMC `0xc2333333`, index selector 2, RGB BL selector 0, flag 0 |
| style sheet string | logical empty string, but an explicitly present 2-bit empty `T` stream |

For both frames, class main ends at bit 1158. The independent empty string body is 2 bits, followed by
the 16-bit string-size footer and true presence bit, so handle data starts at bit
`1158 + 2 + 17 = 1177`. An empty logical string does not permit omission of this stream.

The prior implementation-note parentheticals were incorrect: `0x28b` and `0x290` are object handles,
not hexadecimal spellings of the status values. The exact semantic status sets are:

| handle | packed status oracle | named set bits |
| --- | ---: | --- |
| `0x28b` | `819232 = 0x0c8020` | UCS icon visible, always-enabled, grid beyond drawing limits, adaptive grid |
| `0x290` | `557152 = 0x088060` | UCS icon visible, UCS icon at origin, always-enabled, adaptive grid |

Decode the mask into the named set and reject bits outside the closed AC1024 domain; encode derives the
integer from that set.

### Common state, handles, fill and CRC

Both frames use BOT selector 0 and a direct self handle wire code 0; have no EED or graphics; use
paper-space entity mode 1, zero reactors via BL selector 2, a present extension dictionary, ByLayer
color/linetype/plot-style/material, linetype scale 1, shadow 0, no common visual-style overrides,
invisibility 0 and lineweight 29. Paper-space mode supplies no common owner-handle slot.

After common extension dictionary and layer handles, emit zero frozen-layer handles, then exactly seven
class slots in declaration order: clip boundary code 5, named UCS 5, base UCS 5, background 4, visual
style 5, shade plot 4 and sun 3. Null optionals still emit their declared null handle slot. Both frames
have only visual style `0x9f` non-null. Exact rows are:

| self / address | xdictionary / layer | payload / total | handle bits | data / class / string bits | stored CRC |
| --- | --- | ---: | ---: | --- | ---: |
| `0x28b` / `0x3b8a` | `0x28c` / `0x10` | 161 / 166 | 111 | 1177 / 1158 / 2 | `0xa3dc` |
| `0x290` / `0x3c30` | `0x291` / `0x1b4` | 162 / 167 | 119 | 1177 / 1158 / 2 | `0x651a` |

Each handle stream terminates in exactly seven derived one-fill bits `1111111`. The main/string state is
the same length; the second frame's one-byte growth comes only from deterministic handle encoding.
Validate each xdictionary's reciprocal owner, both layer targets, visual style type 506, and all future
non-null role target kinds; frozen-layer targets must be layer table records and unique.

### Append-only facets and acceptance

Append `Viewport(DwgViewportEntity)` to the nested Rust `DwgEntityBody` as ordinal 7 / payload field 8;
the outer Rust object-body remains `Entity` ordinal 4 / field 5. Add `viewport` to canonical TypeScript's
entity union, GraphQL's flattened body union, JSON `entityBody`, and every duplicated snapshot/diff
facet. The live flattened canonical/snapshot/diff Proto oneof now includes `visual_style = 19`; append
`DwgViewportEntity viewport = 20`. Consequently the future type-559 flattened Proto reservation moves
to tag 21, while its Rust outer ordinal 13 / field 14 is unchanged. Mutation remains SetSnapshot-based
and requires no special viewport operation.

Acceptance requires 2/2 imports as the typed sole-authority body, exact exhaustion of main/string/handle
streams, both exact full frames and CRCs, and original-fixture equality through native export, snapshot
DSL/pack, diff/apply/inverse/absorb, mutation/inverse, analyzer and composer. Extend existing tests to
mutate/inverse center/size, one named status flag, present-empty versus nonempty style sheet, frozen
layer vector, ambient color and one nullable role handle. Anti-shadow/parity assertions must require the
new body and every named field in Rust/TS/GraphQL/Proto/JSON while forbidding raw status integers,
selectors, packed color, count, string footer, handle codes, stream boundaries, fill, CRC and frame
bytes.

## P23: AC1024 type 529 `BLOCKFLIPPARAMETER` exact-ready oracle (2026-08-14)

This is a read-only production audit. No production file was edited and no Cargo/Nx command was run.
The existing R2 probe was rerun in memory against the exact fixture and its parsed graph relations were
joined by absolute handle. LibreDWG's current
[`dwg2.spec`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3051-L3169) confirms the
inheritance order `AcDbEvalExpr -> AcDbBlockElement -> AcDbBlockParameter ->
AcDbBlock2PtParameter`; its
[`BLOCKFLIPPARAMETER`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3240-L3251)
declaration confirms the four strings, label point, group-96 scalar and final group-309 string. The
fixture resolves the two fields which that source still calls `bl96` and `tooltip`.

### Semantic correction: graph target, not current-state index or tooltip

The group-96 scalar is the evaluation-node identifier of the associated updated-flip expression. The
group-309 string is that expression's name. All three frames use `"UpdatedFlip"`, and every scalar
resolves through the owning type-517 evaluation graph to a node whose expression handle is a type-530
`BLOCKFLIPGRIP`:

| flip parameter | owner graph | source graph slot `(id,next_id,expression)` | updated graph slot `(id,next_id,expression)` |
| --- | --- | --- | --- |
| `0x1118` | `0x110f` | slot 8 `(8,26,0x1118/type529)` | slot 9 `(9,27,0x1119/type530)` |
| `0x111d` | `0x110f` | slot 13 `(13,31,0x111d/type529)` | slot 18 `(18,50,0x1122/type530)` |
| `0x1156` | `0x1155` | slot 0 `(0,120,0x1156/type529)` | slot 1 `(1,128,0x1157/type530)` |

Thus the parameter's inherited `DwgEvaluationExpression.node_id` is the source graph node's `next_id`,
not its storage `id`; the type-local group-96 value is the target graph node's `next_id`. Do not persist
the target expression handle: it is derived from the typed graph membership. Do not append either value
to generic `referenced_handles`; `owner_handle` and the named graph-node IDs are the sole authorities.

The first inherited two-point `prop_states` value is also exactly the updated node ID in all three
frames: `(27,0,0,0)`, `(50,0,0,0)`, `(128,0,0,0)`. The writer must derive that native vector and the
later group-96 scalar from one logical `updated_flip.node_id`; persisting both would duplicate the same
logical relation. Reject a native frame when slots 1-3 are nonzero or slot 0 differs from group 96 until
those alternative standard semantics have a named model.

There is no generic numeric `AcDbBlockParamValueSet` in the flip-parameter declaration. The logical
flip value set is exactly its two named states. Do not add range flags, minimum, maximum, increment or a
numeric value list. A code-ready sole-authority shape is:

```text
DwgBlockParameterConnection { code: u32, name: String }
DwgBlockParameterProperty { connections: Vec<DwgBlockParameterConnection> }
DwgBlockParameterBaseLocation { StartPoint, Midpoint }
DwgBlockFlipValueSet { base_label: String, flipped_label: String }
DwgNamedEvaluationNodeReference { node_id: u32, expression_name: String }
DwgBlockFlipParameter {
  evaluation_expression: DwgEvaluationExpression,
  name: String,
  show_properties: bool,
  chain_actions: bool,
  definition_base: Vec3,
  definition_end: Vec3,
  properties: [DwgBlockParameterProperty; 4],
  base_location: DwgBlockParameterBaseLocation,
  label: String,
  description: String,
  value_set: DwgBlockFlipValueSet,
  label_point: Vec3,
  updated_flip: DwgNamedEvaluationNodeReference
}
```

Reuse the live tagged `DwgEvaluationExpression` and its `Empty` value. The native block-element format
major/minor repeat the expression's logical version in this fixture, and the final block-element marker
is a class/version constant. Do not persist a second version pair or marker. For these AC1024 frames the
expression and repeated element versions are `29/2`, the value discriminator is `-9999` (`Empty`), and
the derived marker is zero. LibreDWG's generic encoder currently proposes `33/29` for later versions;
that policy does not match this fixture and must not override the observed AC1024 writer oracle.

### Exact main and string order

After BOT, self handle, typed EED and accepted common-object data, consume/write the main stream in this
order:

1. evaluation expression: parent ID `BL`, major `BL`, minor `BL`, value discriminator `BS`, conditional
   value (absent for `Empty`), node ID `BL`;
2. block-element name `T`, repeated major `BL`, repeated minor `BL`, derived zero marker `BL`;
3. show-properties `B`, chain-actions `B`, definition base `3BD`, definition end `3BD`;
4. four property groups in order, each derived connection count `BL` followed by each connection code
   `BL` and name `T`;
5. four native property-state `BL`s derived as `[updated_flip.node_id,0,0,0]`, then base location `BS`;
6. label `T`, description `T`, base-state label `T`, flipped-state label `T`, label point `3BD`, updated
   node ID `BL`, updated expression name `T`;
7. start the common object handle stream; this class adds no local handle slot.

Because AC1024 diverts `T` values, the independent string stream is declaration-ordered: conditional
evaluation string (none here), element name, every connection name, label, description, base-state
label, flipped-state label and updated expression name. It is not legal to read all flip strings before
the inherited property strings. Native counts, compact selectors, repeated node scalar, string
size/presence/footer, stream boundary and handle encodings are serializer state.

All three frames have `show_properties=true`, `chain_actions=false`, four empty connection sets,
`base_location=StartPoint`, empty evaluation value, parent `-1`, expression version `29/2`, repeated
element version `29/2`, and marker zero. Their exact logical differences and compact branches are:

| handle | expression/source node | name; definition base -> end | flip label / description | value set | label point; updated node/name |
| --- | --- | --- | --- | --- | --- |
| `0x1118` | `26` | `hinge`; `(15,-12.5,0)` `(0,0,2)` -> `(15.000000000000002,0,0)` `(0,2,2)` | `Hinge` / `Sets the side the door is hung on` | `Left` / `Right` | `(-9.999999999999998,0,0)` `(0,2,2)`; `27` / `UpdatedFlip` |
| `0x111d` | `31` | `swing`; `(15,-2.5,0)` `(0,0,2)` -> `(0,-2.5,0)` `(2,0,2)` | `Swing` / `Sets the direction of the swing` | `Inside` / `Outside` | `(15,-5.000000000000001,0)` `(0,0,2)`; `50` / `UpdatedFlip` |
| `0x1156` | `120` | `Flip`; `(0,3,0)` `(2,0,2)` -> `(0,6,0)` `(2,0,2)` | `Flip Window` / `Flip window along the frame axis` | `Right` / `Left` | `(0,6.5,0)` `(2,0,2)`; `128` / `UpdatedFlip` |

For each row, the nonzero derived property-state and later updated-node `BL` use selector 1; the other
three property states and StartPoint use selector 2. Every empty property-count uses selector 2.

### Common state, graph ownership, fill and CRC

All use BOT selector 1 and a direct self handle code 0. They have zero reactors (BL selector 2), no
extension dictionary, and exactly one common owner role. The handle writer derives code C with delta 9
for `0x1118 -> 0x110f`, code C with delta 14 for `0x111d -> 0x110f`, and code 8 for
`0x1156 -> 0x1155`. No class-local handle follows.

The first two frames have no EED. `0x1156` has one genuine typed EED record: application handle
`0x10d4`, group 1010 point `(1.1316127095806177,1.8683872904193823,0)`. This belongs only in the live
`DwgLogicalObject.extended_data`; the native EED byte size 25 and application-handle code 5 are derived.

| self / address | payload / total | handle bits | main end / data end / string bits | terminal fill | stored CRC |
| --- | ---: | ---: | --- | --- | ---: |
| `0x1118` / `0x6574` | 195 / 200 | 20 | 455 / 1540 / 1068 | `1111` | `0x461e` |
| `0x111d` / `0x6721` | 207 / 212 | 20 | 519 / 1636 / 1100 | `1111` | `0x4d63` |
| `0x1156` / `0x9436` | 223 / 228 | 10 | 625 / 1774 / 1132 | `11` | `0x236d` |

Each uses the three-byte modular payload-size prefix and two-byte CRC. Derive payload size, handle-bit
count, string footer, one-fill and CRC after both streams are complete; none belongs in the logical
body.

### Live gap, append-only tags and exact gate

The live tree advanced during this read-only audit: Rust now has nested entity `Viewport`, outer
`DynamicBlockPurgePreventer` ordinal 15 / payload field 16, and outer `EvaluationGraph` ordinal 16 /
field 17. Append `BlockFlipParameter(DwgBlockFlipParameter)` as ordinal 17 / field 18.
Canonical/snapshot/diff Proto already use tags 23 and 24 for VIEWPORT and EVALUATION_GRAPH, so append
`DwgBlockFlipParameter block_flip_parameter = 25`; never renumber it afterward. Mirror the
named records and tagged arm through
canonical, snapshot and diff TypeScript/GraphQL/Proto/JSON; the structured DSL/pack codecs follow the
Rust `DslField` shapes. Mutation remains SetSnapshot-based and gains no native-tail operation.

The inspected IO has no type-529 decode/write branch; the live typed evaluation-graph body supplies the
authority needed for the validations above. Import must require: owner type 517; one graph node whose expression handle
is self and whose `next_id` equals `evaluation_expression.node_id`; one graph node whose `next_id`
equals `updated_flip.node_id`, whose expression target is type 530; distinct nonempty state labels; four
property sets; finite points; exact main/string/handle exhaustion; and valid fill/CRC. Export repeats
the same checks before materialization.

Acceptance is 3/3 typed imports and exact full-frame equality for all three rows, then exact original
fixture equality through native export, snapshot DSL/pack, diff/apply/inverse/absorb,
mutation/inverse, analyzer and composer. Extend the existing lifecycle test to mutate/inverse the two
state labels, label point, one property connection, and an updated graph-node relation as one coherent
snapshot edit. Anti-shadow assertions must require the body and graph references across every facet and
forbid `bl96`, `tooltip`, `current_state_index`, duplicated `prop_states`, native version-marker copies,
counts, selectors, string/footer bits, stream offsets, handle wire codes, fill, CRC and frame bytes.

## P24: AC1024 type 531 `BLOCKVISIBILITYPARAMETER` exact-ready oracle (2026-08-14)

This is a read-only production audit. No production file was edited and no Cargo/Nx command was run.
The existing R2 probe was rerun in memory and all 136 handle roles were resolved to absolute objects and
joined to the owning evaluation graph. LibreDWG's current
[`BLOCKVISIBILITYPARAMETER`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3337-L3360)
declaration confirms the one-point base, initialization flag, two descriptive strings, secondary
policy bit, eligible member vector and ordered state records. The fixture supplies the narrower logical
names and proves that the source's `blocks` and `params` labels are broader than their actual targets.

### Logical state and evaluation-graph authority

The object is handle `0x111e`, owned by evaluation graph `0x110f`. In that graph, slot 14 is
`(id=14,next_id=46,expression=0x111e/type531)`. The inherited evaluation expression therefore has
`node_id=46`. The trailing `BL` after the two one-point property groups is `47`, not a count of 47:
graph slot 15 is `(id=15,next_id=47,expression=0x111f/type532 BLOCKVISIBILITYGRIP)`. Model that scalar as
`updated_visibility_node_id`; do not persist the target expression handle, graph slot, or a misleading
`num_propinfos`/`property_count` field.

The source calls both entity references and graph-expression references “blocks/params,” but the fixture
resolves them precisely:

- the 11 eligible members are five type-77 LWPOLYLINE entities, four type-17 ARC entities and two
  type-19 LINE entities;
- every state's visible-member vector is a unique ordered subset of those 11 eligible entity handles;
- every state's 21 controlled references is an expression handle present in owner graph `0x110f`, with
  types `{527x2, 528x2, 529x2, 530x2, 533x1, 534x1, 535x6, 536x1, 521x2, 537x2}`.

Use `eligible_entity_handles`, `visible_entity_handles` and `controlled_expression_handles`, not
`blocks` or `controlled_parameters`. Preserve each vector's logical evaluation order: state 3 proves it
is not an incidental numeric sort. The repeated controlled vector is genuinely state membership and
must remain in each state even though all five happen to be equal in this fixture.

The secondary group-91 bit is the evaluation-history policy already identified in the R2 research.
Model a closed `DwgVisibilityEvaluationHistory::{Stateless,Required}` enum. This fixture is `Stateless`.
It contains no current-state field and no history/predecessor record; do not invent or persist either.
`Required` is admissible only when a future typed graph relation supplies its predecessor semantics,
never as an unexplained Boolean.

A code-ready sole-authority body is:

```text
DwgVisibilityEvaluationHistory { Stateless, Required }
DwgVisibilityState {
  name: String,
  visible_entity_handles: Vec<u64>,
  controlled_expression_handles: Vec<u64>
}
DwgBlockVisibilityParameter {
  evaluation_expression: DwgEvaluationExpression,
  element_name: String,
  show_properties: bool,
  chain_actions: bool,
  definition_point: Vec3,
  properties: [DwgBlockParameterProperty; 2],
  updated_visibility_node_id: u32,
  initialized: bool,
  name: String,
  description: String,
  evaluation_history: DwgVisibilityEvaluationHistory,
  eligible_entity_handles: Vec<u64>,
  states: Vec<DwgVisibilityState>
}
```

Reuse P23's `DwgBlockParameterProperty` and the live tagged `DwgEvaluationExpression`. The body does not
duplicate native counts, block-element versions/marker, target expression handle, string boundaries or
handle wire encodings.

### Exact fixture memberships

The eligible vector is ordered exactly as follows:

```text
0x1139/LWPOLYLINE, 0x113a/LWPOLYLINE, 0x113b/LWPOLYLINE,
0x113c/LWPOLYLINE, 0x113d/LWPOLYLINE,
0x113e/ARC, 0x113f/ARC, 0x1140/ARC, 0x1141/ARC,
0x1142/LINE, 0x1143/LINE
```

The five ordered states are:

| State | Visible entity handles in logical evaluation order | Eligible indexes |
| --- | --- | --- |
| `Open 30º` | `0x113a, 0x1141, 0x1142, 0x1143` | `1,8,9,10` |
| `Open 45º` | `0x113b, 0x1140, 0x1142, 0x1143` | `2,7,9,10` |
| `Open 60º` | `0x113c, 0x113f, 0x1142, 0x1143` | `3,6,9,10` |
| `Open 90º` | `0x113d, 0x1142, 0x1143, 0x113e` | `4,9,10,5` |
| `Closed` | `0x1139, 0x1142, 0x1143` | `0,9,10` |

Every state has the same ordered controlled-expression vector:

```text
0x1110/type527, 0x1111/type528, 0x1114/type527, 0x1115/type528,
0x1118/type529, 0x1119/type530, 0x111d/type529, 0x1122/type530,
0x1126/type533, 0x1127/type534,
0x112a/type535, 0x112b/type535, 0x112c/type535, 0x112d/type535,
0x112e/type535, 0x112f/type536, 0x1130/type535,
0x1131/type521, 0x1132/type521, 0x1133/type537, 0x1134/type537
```

Import/export validation requires unique nonempty state names, unique eligible entities, unique visible
entities per state, every visible entity in the eligible vector, every controlled expression in the
owner graph, source/updated node membership as above and no duplicate controlled handle within a state.
Do not require every state to share the same controlled vector; that equality is fixture data, not a
format rule.

### Exact evaluation/parameter main and string streams

After BOT, self, EED and accepted common-object data, consume/write:

1. evaluation expression: parent ID `BL`, major `BL`, minor `BL`, value discriminator `BS`, conditional
   value, node ID `BL`;
2. element name `T`, repeated element major `BL`, repeated minor `BL`, derived class marker `BL`;
3. show-properties `B`, chain-actions `B`, definition point `3BD`;
4. two property groups, each derived connection count `BL`, then ordered connection code `BL` / name
   `T` pairs;
5. `updated_visibility_node_id BL`;
6. initialized `B`, visibility name `T`, description `T`, evaluation-history policy `B`;
7. derived eligible count `BL`, derived state count `BL`; for each state in order, name `T`, derived
   visible count `BL`, derived controlled-expression count `BL`;
8. common handle stream: owner, eligible entities, then for each state its visible entities followed by
   controlled expressions. The class adds no other handle.

The independent string stream is declaration-ordered: optional evaluation string (none here), element
name, property-connection names (none), visibility name, description and the five state names. The exact
logical scalar values are:

| Concept | Fixture value / compact branch |
| --- | --- |
| evaluation | parent `-1` BL selector 0; version `29/2` selectors 1/1; `Empty` discriminator `-9999` BS selector 0; source node `46` selector 1 |
| element | `Visibility State`; repeated version `29/2` selectors 1/1; derived marker zero selector 2 |
| parameter | show true; chain false; point `(-5,15,0)` with BD selectors `(0,0,2)` |
| properties | two empty connection vectors, both counts selector 2; updated node `47` selector 1 |
| visibility | initialized true; name `Opening Angle`; description `Sets the angle of the door opening`; history `Stateless` (false bit) |
| collection counts | eligible `11`, states `5`, every visible count `4/4/4/4/3`, every controlled count `21`; all nonzero counts selector 1 |

As in P23, derive the repeated block-element version from the logical evaluation-expression version and
the marker from the AC1024 class policy. Do not persist a second `29/2/0` triplet.

### Common roles, frame mechanics and append-only facets

The exact frame is at object-section address `0x67f5`. It has BOT selector 1, direct self handle code 0,
no EED, zero reactors via BL selector 2, no extension dictionary and owner graph `0x110f` via wire code C
with delta 15. Every one of the remaining 135 entity/expression roles uses declared absolute wire code
4. The modular payload prefix is four bytes; payload is 673 bytes, total frame 679 bytes, handle-bit
count 3256, main/string boundary bit 415, data/handle boundary bit 2128, string content 1696 bits, no
terminal fill bits, and stored CRC `0x7e73`. Counts, the 17-bit string footer/presence encoding, boundaries,
handle codes, size prefix and CRC are writer-derived.

At audit time live Rust includes outer `EvaluationGraph` ordinal 16 / field 17 and flattened Proto
already uses VIEWPORT tag 23 and EVALUATION_GRAPH tag 24. Implement in dependency order: type529 as
Rust outer ordinal 17 / field 18 and Proto tag 25; then
`BlockVisibilityParameter(DwgBlockVisibilityParameter)` as Rust ordinal 18 / field 19 and flattened
Proto tag 26. If these land in one patch, reserve both tags explicitly. Mirror every record/enum/arm in
canonical, snapshot and diff TypeScript/GraphQL/Proto/JSON and the structured DSL/pack shapes. Mutation
remains SetSnapshot-based.

Acceptance is 1/1 typed import with exact graph/entity membership, exact main/string/handle exhaustion,
exact frame/CRC and original-fixture equality through native export, snapshot DSL/pack,
diff/apply/inverse/absorb, mutation/inverse, analyzer and composer. Extend the existing lifecycle test to
mutate/inverse a state name, reorder one logical visible vector, move one eligible entity between states,
change the history enum only through a valid graph relation, and reject a foreign controlled expression.
Anti-shadow/parity assertions must require the named enum/body/state fields while forbidding
`unknown_bool`, `history_required` Boolean, `blocks`, `params`, `num_propinfos`, `property_count`, native
counts, repeated version/marker copies, stream/footer fields, handle codes, fill, CRC and frame bytes.

## P25: AC1024 PLACEHOLDER and type 503 `DICTIONARYVAR` exact-ready oracles (2026-08-15)

This is a read-only production audit. No production file was edited and no Cargo/Nx command was run.
An in-memory bounded probe reused the accepted object-page, handle-map, EED, R2010 string-stream,
common-object, absolute-handle and CRC readers. It admitted exactly one fixed type-80 frame and eight
dynamic type-503 frames, and asserted main/string/handle exhaustion plus stored CRC for every frame.
LibreDWG's current
[`PLACEHOLDER`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L1137-L1143)
declaration confirms an empty class body, while its
[`DICTIONARYVAR`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec#L4414-L4421)
declaration confirms one signed schema byte followed by one `T` value and no class-local handles.

### PLACEHOLDER: explicit empty semantic body and reciprocal default role

Persist an explicit `DwgLogicalObjectBody::Placeholder(DwgPlaceholder {})`. `body=None` remains reserved
for unsupported or undecoded objects and cannot represent successful recognition of a standard empty
body. The body has no scalar, string, byte, handle, marker or count field.

The sole fixture object is fixed type 80, handle `0x0f`, object-section address `0x4463`. It is owned by
and has one reactor `0x0e`, the type-500 `ACDBDICTIONARYWDFLT`. That parent dictionary has exactly the
entry `("Normal",0x0f)` and the same `0x0f` as its default-entry handle. Exact semantic validation is:

- type 80/class `ACDBPLACEHOLDER`, object category, explicit `Placeholder` body;
- no EED and no extension dictionary;
- `owner_handle=0x0e`, `reactor_handles=[0x0e]`;
- parent `0x0e` resolves to WDFLT, contains this object exactly once under the nonempty key `Normal`,
  and has the same object as its default entry;
- no other dictionary claims the placeholder as an owned/default entry.

Native stream order is BOT, direct self handle, zero EED terminator, common reactor count/xdic-missing
bit, absent-string-stream bit, then common owner and reactor handles. There is no class main bit between
common data and the absent-string bit and no class handle after the common roles. The exact frame oracle
is:

| BOT / self | prefix / payload / total | main end / data end / handle bits | string | roles | fill / CRC |
| --- | --- | --- | --- | --- | --- |
| selector 0 / code 0 | 3 / 8 / 13 bytes | bit 39 / bit 40 / 24 | absent, 0 bits | owner code 8 -> `0x0e`; reactor code 4 -> `0x0e` | empty / `0x4db3` |

Reactor count one uses BL selector 1 and the xdictionary-missing bit is true. The absent string marker,
payload-size prefix, handle-bit count, compact handle codes and CRC are serializer state and never body
fields.

### DICTIONARYVAR: logical value and dictionary-key identity

Use the sole-authority record `DwgDictionaryVariable { value: String }`. The preliminary P12 proposal to
persist `DwgDictionaryVariableSchema` is too broad: all eight native schema bytes are revision zero, the
field is a class serialization revision rather than document content, and the dictionary entry key is
the variable's logical identity. Import must accept schema zero only; export derives zero. Do not copy
the parent key into the body or parse string values into guessed Booleans/integers: the standard class
defines its semantic value as a string, and keys such as `LAYERNOTIFY` and `LIGHTINGUNITS` have distinct
domains despite sharing numeric-looking text.

All eight objects are owned by and have one reactor `0x66`, a fixed type-42 DICTIONARY. That dictionary's
complete reciprocal key/value map is:

| Key | object | logical string value |
| --- | ---: | --- |
| `CANNOSCALE` | `0x00f0` | `1'-0" = 1'-0"` |
| `CMLEADERSTYLE` | `0x00ef` | `STANDARD` |
| `CTABLESTYLE` | `0x0089` | `STANDARD` |
| `DIMASSOC` | `0x0067` | `2` |
| `HIDETEXT` | `0x006b` | `1` |
| `LAYEREVAL` | `0x014d` | `0` |
| `LAYERNOTIFY` | `0x014e` | `0` |
| `LIGHTINGUNITS` | `0x02d7` | `1` |

Require unique keys in the parent, exactly one reciprocal entry for each variable, owner equal to the
entry dictionary, reactor vector exactly `[owner]`, no EED, no extension dictionary and valid Unicode.
The dictionary entry order remains the dictionary's logical ordered map; the DICTIONARYVAR body has no
independent name/order field.

### Exact DICTIONARYVAR streams, roles, fill and CRC

For every frame, consume/write BOT selector 1, direct self handle code 0, zero EED terminator, common
reactor count (one, BL selector 1), xdictionary-missing true, derived schema revision `RC=0`, then the
value in the independent `T` string stream. After the 16-bit string-size footer and presence bit, write
only common owner and reactor roles. There is no class handle.

The resolved frame matrix is:

| handle / address | value | prefix / payload / total bytes | main end / data end / string bits | handle bits; owner wire | fill / CRC |
| --- | --- | --- | --- | --- | --- |
| `0x0067` / `0x4b44` | `2` | 3 / 15 / 20 | 47 / 90 / 26 | 30; code 8 -> `0x66` | `111111` / `0x6bbb` |
| `0x006b` / `0x2acc` | `1` | 3 / 16 / 21 | 47 / 90 / 26 | 38; code C delta 5 -> `0x66` | `111111` / `0x7627` |
| `0x0089` / `0x4b21` | `STANDARD` | 3 / 30 / 35 | 47 / 202 / 138 | 38; code C delta 35 -> `0x66` | `111111` / `0x731f` |
| `0x00ef` / `0x4afe` | `STANDARD` | 3 / 30 / 35 | 47 / 202 / 138 | 38; code 4 -> `0x66` | `111111` / `0x635c` |
| `0x00f0` / `0x4ad1` | `1'-0" = 1'-0"` | 3 / 40 / 45 | 47 / 282 / 218 | 38; code 4 -> `0x66` | `111111` / `0x97fe` |
| `0x014d` / `0x2ae1` | `0` | 3 / 17 / 22 | 55 / 98 / 26 | 38; code 4 -> `0x66` | `111111` / `0x68d9` |
| `0x014e` / `0x2af7` | `0` | 3 / 17 / 22 | 55 / 98 / 26 | 38; code 4 -> `0x66` | `111111` / `0x69d8` |
| `0x02d7` / `0x2b0d` | `1` | 3 / 17 / 22 | 55 / 98 / 26 | 38; code 4 -> `0x66` | `111111` / `0xefc9` |

Every reactor is an absolute code-4 handle to `0x66`. The later main-end bit for handles above `0xff`
comes only from the longer self-handle representation; it is not a body version branch. Every string
stream is present, including the one-character values. Derive the schema byte, TU length, string footer,
stream boundaries, handle encoding, six one-fill bits, payload size and CRC.

### Append-only facets and strict lifecycle gate

The live Rust outer union ends at `EvaluationGraph` ordinal 16 / payload field 17, and flattened Proto
ends at EVALUATION_GRAPH tag 24. Preserve the P23/P24 reservations: type529 takes Rust 17/field18 and
Proto 25; type531 takes Rust 18/field19 and Proto 26. Then append `Placeholder` as Rust ordinal 19 /
field 20 and Proto tag 27, followed by `DictionaryVariable` as Rust ordinal 20 / field 21 and Proto tag
28. Mirror both through canonical, snapshot and diff TypeScript/GraphQL/Proto/JSON and structured
DSL/pack. Proto uses an empty `DwgPlaceholder` message; JSON uses an object with zero properties and
`additionalProperties:false`. Because GraphQL unions cannot contain an empty object or scalar, expose a
resolver-derived singleton placeholder-kind field solely in the GraphQL projection; it must never enter
Rust/JSON/DSL/pack/diff/mutation persistence.

Mutation remains SetSnapshot-based. Acceptance requires 1/1 PLACEHOLDER and 8/8 DICTIONARYVAR typed
imports, exact semantic stream exhaustion, all reciprocal dictionary relations, nine exact full frames
and original-fixture equality through native export, snapshot DSL/pack, diff/apply/inverse/absorb,
mutation/inverse, analyzer and composer. Extend the existing lifecycle test to mutate/inverse ASCII and
non-ASCII dictionary-variable values; atomically rename one parent key without duplicating it; remove
and restore the placeholder together with both parent entry/default relations; and reject schema != 0,
orphaned/multiply-owned variables, a non-WDFLT placeholder parent and malformed Unicode.

Anti-shadow/parity assertions must require the explicit empty Placeholder arm and DictionaryVariable
value across all facets while forbidding `schema`, `schemaByte`, native string lengths/text bytes,
string-presence/footer bits, counts, stream offsets, handle codes, fill, CRC, frame/payload bytes, a
duplicated variable name/key, or a persisted GraphQL singleton marker.

## P26: AC1024 type 516 `SORTENTSTABLE` exact-ready oracle (2026-08-15)

This is a read-only production audit. No production file was edited and no Cargo/Nx command was run.
An in-memory bounded probe reused the accepted AC1024 object-page, handle-map, EED, R2010
string-stream, common-object, absolute-handle and CRC readers. It admitted exactly the seven dynamic
type-516 frames and asserted the main/string/handle endpoints and stored CRC for every frame.
LibreDWG's primary [`SORTENTSTABLE`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L143-L167)
declaration confirms the asymmetric native encoding: a derived `BL` count, `count` sort handles encoded
with wire code 0 in the main stream, then the block owner and `count` entity references encoded with
wire code 4 in the handle stream.

### Sole-authority logical model and critical handle distinction

Persist one ordered pairing rather than two native vectors:

```text
DwgDrawOrderEntry { entity_handle: u64, sort_handle: u64 }
DwgSortEntitiesTable { block_header_handle: u64, entries: Vec<DwgDrawOrderEntry> }
```

`entity_handle` is an object reference. `sort_handle` is a semantic numeric draw-order key represented
by the DWG handle primitive, but it is **not** necessarily an object reference. The populated fixture
table has 45 unique entity handles and 45 unique sort handles, yet their sets intersect at only 37.
These eight valid sort keys have no target in the object map: `0x1f80`, `0x2100`, `0x21a2`, `0x1f83`,
`0x1f85`, `0x21ac`, `0x2133`, `0x1f7f`. Conversely, these eight entity references do not occur as sort
keys: `0x1f7d`, `0x21b1`, `0x1f36`, `0x2156`, `0x1f38`, `0x211d`, `0x1f7c`, `0x1f3d`.
Therefore the decoder must not resolve or reject a sort key as an object reference, and the schema must
not type both halves as generic references. The order of `entries` is semantic; the native count and
split vectors are writer-derived.

The exact populated ordered sequence at table `0x10c5`, with the resolved entity type shown solely as
probe evidence, is:

| # | entity handle / type | sort handle |
| ---: | --- | ---: |
| 1 | `0x20e8` / INSERT 7 | `0x2161` |
| 2 | `0x1f86` / LINE 19 | `0x20a7` |
| 3 | `0x1f35` / LINE 19 | `0x1f35` |
| 4 | `0x222c` / LINE 19 | `0x1f80` |
| 5 | `0x1f7b` / LINE 19 | `0x2011` |
| 6 | `0x20b4` / INSERT 7 | `0x2128` |
| 7 | `0x20a7` / INSERT 7 | `0x2122` |
| 8 | `0x1f81` / LINE 19 | `0x1f83` |
| 9 | `0x1fdd` / INSERT 7 | `0x20db` |
| 10 | `0x2166` / DIMENSION_LINEAR 21 | `0x222f` |
| 11 | `0x1f3d` / INSERT 7 | `0x20c1` |
| 12 | `0x2011` / INSERT 7 | `0x2100` |
| 13 | `0x215b` / DIMENSION_LINEAR 21 | `0x222d` |
| 14 | `0x1f87` / LINE 19 | `0x1f85` |
| 15 | `0x2122` / DIMENSION_LINEAR 21 | `0x21a2` |
| 16 | `0x1f36` / LINE 19 | `0x1f81` |
| 17 | `0x2230` / LINE 19 | `0x1f89` |
| 18 | `0x1f38` / LINE 19 | `0x1ff7` |
| 19 | `0x2161` / DIMENSION_LINEAR 21 | `0x222e` |
| 20 | `0x222d` / LINE 19 | `0x1f7f` |
| 21 | `0x1f7c` / LINE 19 | `0x1f88` |
| 22 | `0x2128` / DIMENSION_LINEAR 21 | `0x21ac` |
| 23 | `0x2156` / DIMENSION_LINEAR 21 | `0x222c` |
| 24 | `0x1f82` / LINE 19 | `0x1f84` |
| 25 | `0x211d` / DIMENSION_LINEAR 21 | `0x2177` |
| 26 | `0x1f88` / LINE 19 | `0x20b4` |
| 27 | `0x2151` / DIMENSION_LINEAR 21 | `0x21ea` |
| 28 | `0x20c1` / INSERT 7 | `0x2133` |
| 29 | `0x1f37` / LINE 19 | `0x1f87` |
| 30 | `0x2231` / LINE 19 | `0x206a` |
| 31 | `0x222e` / LINE 19 | `0x1fdd` |
| 32 | `0x1f7d` / LINE 19 | `0x1f37` |
| 33 | `0x206a` / INSERT 7 | `0x2107` |
| 34 | `0x1f89` / LINE 19 | `0x1f86` |
| 35 | `0x222f` / LINE 19 | `0x1f8a` |
| 36 | `0x1f7e` / LINE 19 | `0x1f82` |
| 37 | `0x21b1` / LINE 19 | `0x1f7e` |
| 38 | `0x1f84` / LINE 19 | `0x1f7b` |
| 39 | `0x20db` / INSERT 7 | `0x215b` |
| 40 | `0x20ce` / INSERT 7 | `0x2151` |
| 41 | `0x1f8a` / INSERT 7 | `0x20ce` |
| 42 | `0x2107` / DIMENSION_LINEAR 21 | `0x2166` |
| 43 | `0x2177` / DIMENSION_LINEAR 21 | `0x2230` |
| 44 | `0x1ff7` / INSERT 7 | `0x20e8` |
| 45 | `0x21ea` / DIMENSION_LINEAR 21 | `0x2231` |

The populated entity cohort is exactly 12 INSERT, 22 LINE and 11 DIMENSION_LINEAR objects. The table
must retain this pair order; sorting either side independently destroys the mapping.

### Exact streams, reciprocal graph and frame matrix

After BOT, self, zero EED terminator and common-object data, consume/write:

1. derive `count = entries.len()` and write it as `BL`;
2. in the **main data stream**, write each `entry.sort_handle` in entry order as declared handle code 0;
3. consume/write the absent R2010 string-stream marker; this body has no strings;
4. in the handle stream, write common owner/reactor/extension-dictionary roles;
5. write `block_header_handle` with declared wire code 4;
6. write each `entry.entity_handle` in the same entry order with declared wire code 4.

The importer must bound `count` before allocation, read exactly that many values in each half, zip the
halves without reordering, and reject an endpoint mismatch. The exporter derives the count and both
vectors from the paired entries. It never persists count, stream selection, wire codes or boundaries.

All seven tables have no EED, one reactor equal to their common owner DICTIONARY, no extension
dictionary and no strings. Each owner dictionary contains exactly one reciprocal `ACAD_SORTENTS`
entry pointing to the table; that dictionary's owner is the same BLOCK_HEADER referenced by the table
body. The exact reciprocal graph is:

| table | owner/reactor DICTIONARY | body BLOCK_HEADER | dictionary relation |
| ---: | ---: | ---: | --- |
| `0x10c5` | `0x015d` | `0x001f` | owner `0x001f`; `ACAD_SORTENTS -> 0x10c5` |
| `0x1bb9` | `0x110e` | `0x110d` | owner `0x110d`; `ACAD_SORTENTS -> 0x1bb9` |
| `0x1bba` | `0x1146` | `0x1145` | owner `0x1145`; `ACAD_SORTENTS -> 0x1bba` |
| `0x1f66` | `0x1f65` | `0x1f57` | owner `0x1f57`; `ACAD_SORTENTS -> 0x1f66` |
| `0x1fb3` | `0x1fb2` | `0x1fa4` | owner `0x1fa4`; `ACAD_SORTENTS -> 0x1fb3` |
| `0x2037` | `0x2032` | `0x201e` | owner `0x201e`; `ACAD_SORTENTS -> 0x2037` |
| `0x2090` | `0x208b` | `0x2077` | owner `0x2077`; `ACAD_SORTENTS -> 0x2090` |

The six non-model-space tables are legal empty tables. The complete physical oracle is:

| handle / address | count | prefix / payload / total bytes | handle bits / data end / main end | common roles; block role | fill / CRC |
| --- | ---: | --- | --- | --- | --- |
| `0x10c5` / `0x5349` | 45, BL selector 1 | 4 / 286 / 292 | 1150 / 1138 / 1137 | owner code 4 -> `0x015d`; reactor code 4 -> `0x015d`; block code 4 -> `0x001f` | `111111` / `0xb5a3` |
| `0x1bb9` / `0x748a` | 0, BL selector 2 | 3 / 16 / 21 | 78 / 50 / 49 | owner code C -> `0x110e`; reactor code 4 -> `0x110e`; block code 4 -> `0x110d` | `111111` / `0x75bf` |
| `0x1bba` / `0x7c29` | 0, BL selector 2 | 3 / 16 / 21 | 78 / 50 / 49 | owner code C -> `0x1146`; reactor code 4 -> `0x1146`; block code 4 -> `0x1145` | `111111` / `0x698a` |
| `0x1f66` / `0x7c53` | 0, BL selector 2 | 3 / 14 / 19 | 62 / 50 / 49 | owner code 8 -> `0x1f65`; reactor code 4 -> `0x1f65`; block code 4 -> `0x1f57` | `111111` / `0x0370` |
| `0x1fb3` / `0x525b` | 0, BL selector 2 | 3 / 14 / 19 | 62 / 50 / 49 | owner code 8 -> `0x1fb2`; reactor code 4 -> `0x1fb2`; block code 4 -> `0x1fa4` | `111111` / `0x4f3d` |
| `0x2037` / `0x52a9` | 0, BL selector 2 | 3 / 15 / 20 | 70 / 50 / 49 | owner code C -> `0x2032`; reactor code 4 -> `0x2032`; block code 4 -> `0x201e` | `111111` / `0x22e6` |
| `0x2090` / `0x52f8` | 0, BL selector 2 | 3 / 15 / 20 | 70 / 50 / 49 | owner code C -> `0x208b`; reactor code 4 -> `0x208b`; block code 4 -> `0x2077` | `111111` / `0xd470` |

Every frame uses BOT selector 1 and a direct self handle with wire code 0. All string streams are absent
and have zero content bits. Counts, compact BL branch, absent-string bit, stream boundaries, native
split vectors, relative common-owner encodings, size prefix, six terminal one-fill bits and CRC are
serializer state.

### Strict validation, append-only facets and lifecycle gate

Import validation must require `count <= 50_000`, exact paired exhaustion, nonzero entity references,
unique entity handles and a unique total order of sort keys. Each entity must resolve and occur in the
referenced BLOCK_HEADER's logical `owned_entity_handles`; a sort key need not resolve. Require the
reciprocal dictionary chain above generically: common owner is a DICTIONARY with exactly one
`ACAD_SORTENTS -> self` entry, the table's sole reactor is that dictionary, and the dictionary owner
equals `block_header_handle`. Empty entry vectors are valid. Reject duplicate entity membership,
duplicate sort keys, a foreign-block entity, orphaned/multiply-owned tables, mismatched split counts,
extra class handles and trailing main/string/handle bits beyond writer-derived fill.

At audit time live Rust has subsequently appended type529, type531, Placeholder and DictionaryVariable,
ending with `DictionaryVariable` at outer ordinal 20 / payload field 21. Append
`SortEntitiesTable(DwgSortEntitiesTable)` as outer ordinal 21 / payload field 22. The flattened live
Proto still ends at EVALUATION_GRAPH tag 24 and has not yet received the already-reserved 25–28 arms;
preserve those reservations and use Proto tag 29 for SORTENTSTABLE. Mirror `DwgDrawOrderEntry`,
`DwgSortEntitiesTable` and the body arm through canonical, snapshot and diff
TypeScript/GraphQL/Proto/JSON plus structured DSL/pack. GraphQL should expose the ordered `entries`
list; it must not expose independent `sortHandles` and `entityHandles` lists. Mutation remains
SetSnapshot-based.

Acceptance requires 7/7 typed import, exact semantic stream exhaustion, all seven reciprocal graph
relations, seven exact full frames and original-fixture equality through native export, snapshot
DSL/pack, diff/apply/inverse/absorb, mutation/inverse, analyzer and composer. Extend the existing
lifecycle test to mutate/inverse one pair's sort key, atomically reorder two pairs, add/remove a valid
block-owned entity pair, and roundtrip both the populated and empty forms. Negative tests must reject
unpaired native counts, duplicate entities/keys, a foreign-block entity and a missing or wrong
`ACAD_SORTENTS` dictionary relation.

Anti-shadow/parity assertions must require the paired ordered record and named block-header relation in
every facet while forbidding persisted `count`/`num_ents`, separate `sort_handles`/`entities` native
vectors, stream/footer offsets, handle codes, fill, CRC, frame/payload bytes or resolution of
`sort_handle` as an object body/reference.

## P27: AC1024 type 505 `MATERIAL` exact-ready oracle (2026-08-15)

This is a read-only production audit. No production file was edited and no Cargo/Nx command was run.
An in-memory bounded probe reused the accepted object-page, handle-map, EED, R2010 string-stream,
common-object and CRC readers and implemented the declared MATERIAL color/map/procedural traversal. It
admitted exactly three dynamic type-505 frames. The complete documented R2007+ body lands exactly on the
main/string boundary in all three; strings and common handles also exhaust exactly, and all stored CRCs
validate. This proves that the disabled advanced-material block is absent from these fixture frames.
LibreDWG's primary [`MATERIAL`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L2677-L2820)
declaration supplies the field order and the conditional color/map/procedural branches.

### Sole-authority typed material model

Use named channel records and tagged source concepts, never a packed property record:

```text
DwgMaterialColorSource = Current | Override { rgb: DwgRgbColor }
DwgMaterialColor { factor: f64, source: DwgMaterialColorSource }

DwgMaterialProjection = Inherit | Planar | Box | Cylinder | Sphere
DwgMaterialTiling = Inherit | Tile | Crop | Clamp | Mirror
DwgMaterialAutoTransform { scale_to_entity: bool, use_current_block_transform: bool }
DwgMaterialMapper {
  projection: DwgMaterialProjection,
  tiling: DwgMaterialTiling,
  auto_transform: DwgMaterialAutoTransform,
  transform: [f64; 16]
}

DwgMaterialMapSource = None | CurrentScene | File { filename: String } |
                       Procedural { texture: DwgProceduralTexture }
DwgMaterialMap { blend_factor: f64, mapper: DwgMaterialMapper,
                 source: DwgMaterialMapSource }

DwgMaterialChannels {
  diffuse: bool, specular: bool, reflection: bool,
  opacity: bool, bump: bool, refraction: bool
}

DwgMaterial {
  name, description,
  ambient, diffuse, diffuse_map,
  specular, specular_map, specular_gloss,
  reflection_map,
  opacity, opacity_map,
  bump_map,
  refraction_index, refraction_map,
  translucence, self_illumination, reflectivity,
  illumination_model, enabled_channels, mode
}
```

The native color flag is derived from the tagged color source. RGB is a validated 24-bit semantic
color, not a `BLx` container integer. The native auto-transform byte is derived as value 1 when both
logical transform flags are false and from named scale/current-block flags otherwise; it must not be a
raw bitmask. Likewise native channel flags are derived as diffuse 1, specular 2, reflection 4, opacity
8, bump 16 and refraction 32. The fixture value 63 means all six named channels are enabled. Model
illumination and mode as closed enums; this fixture proves only their documented default value zero, so
the R1 decoder must reject unproved values rather than retain an integer escape arm.

Native map source 1 plus an empty filename means “no map” in the primary declaration. Canonicalize that
combination to logical `None`; the deterministic AC1024 writer materializes `None` as source 1 plus the
empty `T`. Require a nonempty valid filename for logical `File`. Native source 0 maps to
`CurrentScene`; source 2 recursively materializes the typed procedural union. This removes an encoding
artifact from the logical schema without losing fixture equality.

The procedural union remains fully named even though no fixture frame takes it:
`Wood { color1, color2 }`, `Marble { color1, color2 }`, or
`Generic(Boolean | Integer | Real | Color | Text | Table)`. A table is an ordered list of
`{ name, texture }`; derive its count and terminal true marker, bound recursion and reject a false
terminator. There is no generic byte/string property bag.

### Exact fixture channel and map values

All three materials have empty descriptions. Ambient, diffuse and specular are each `Current` with
factor 1.0; no frame contains an override RGB. All map slots have blend 1.0, projection `Box`, tiling
`Tile` and no automatic transform. Diffuse, specular, reflection, opacity and bump have logical source
`None`; refraction has `CurrentScene`. The remaining common scalars are specular gloss 0.5, opacity 1.0,
refraction index 1.0, translucence 0.0, self illumination 0.0, reflectivity 0.0, default illumination,
all six channels enabled and default mode.

Let `I` be the row-major 4x4 identity and
`S = diag(0.020800000056624413, 0.020800000056624413, 1.0, 1.0)`. The complete logical differences are:

| handle | name | diffuse | specular | reflection | opacity | bump | refraction |
| ---: | --- | --- | --- | --- | --- | --- | --- |
| `0x0096` | `ByLayer` | `None`, `I` | `None`, `I` | `None`, `I` | `None`, `I` | `None`, `I` | `CurrentScene`, `I` |
| `0x0097` | `ByBlock` | `None`, `I` | `None`, `I` | `None`, `I` | `None`, `I` | `None`, `I` | `CurrentScene`, `I` |
| `0x0098` | `Global` | `None`, `S` | `None`, `I` | `None`, `S` | `None`, `S` | `None`, `S` | `CurrentScene`, `I` |

All finite numeric fields must be validated. Color/map factors, opacity, translucence, self illumination
and reflectivity are constrained to `[0,1]`; refraction index is positive; every transform has exactly
16 finite elements. Validate the closed projection/tiling/source/illumination/mode domains and the
procedural depth/count limits before allocating or recursing.

### Main/string traversal, semantic graph and class handles

After BOT, self, EED and common-object data, consume/write in this exact declaration order:

1. name and description `T`; ambient and diffuse colors (`RC`, `BD`, conditional RGB `BLx`);
2. diffuse map: blend `BD`, projection/tiling/auto-transform `RC`, 16 `BD`, source `RC`, conditional
   filename `T` or typed procedural texture;
3. specular color and specular map, then specular gloss `BD`;
4. reflection map; opacity `BD` and opacity map; bump map;
5. refraction index `BD` and refraction map;
6. AC1021+ translucence, self illumination and reflectivity `BD0`, illumination model, derived channel
   flags and material mode `BL0`.

The independent string traversal is name, description, then conditional map filenames/procedural text
in the same declaration order. In this fixture it is the name followed by six empty strings: description
and the five logical-None file encodings. Refraction `CurrentScene` adds no string. There are **no
class-local handles**. After the string footer, only the common owner, reactor and conditional extension
dictionary roles are legal.

All three common owners and sole reactors are DICTIONARY `0x0072`. Root DICTIONARY `0x000c` has the
reciprocal `ACAD_MATERIAL -> 0x0072` entry, and `0x0072` contains exactly the ordered entries
`ByBlock -> 0x0097`, `ByLayer -> 0x0096`, `Global -> 0x0098`. Require each material's name to equal its
unique reciprocal dictionary key, its owner and sole reactor to equal that dictionary, and no material
to be multiply claimed.

`Global` alone carries typed EED under APPID `ACAD` (`0x0012`): integer16 values `-1, 3, 0`, empty text,
integer32 zero and integer16 zero in that order. It also has extension DICTIONARY `0x0110`, owned by the
material, with four semantic entries:

| key | XRECORD | typed values |
| --- | ---: | --- |
| `BUMPTILE` | `0x0112` | integer16 groups 270/271 = `1/1` |
| `DIFFUSETILE` | `0x0111` | integer16 groups 270/271 = `1/1` |
| `OPACITYTILE` | `0x0113` | integer16 groups 270/271 = `1/1` |
| `REFLECTIONTILE` | `0x0114` | integer16 groups 270/271 = `1/1` |

Each XRECORD is owned by and has sole reactor `0x0110`. Preserve this through the already-typed common
EED/extension-dictionary/XRECORD graph; do not duplicate tile references or XRECORD values in the
MATERIAL body. `ByLayer` and `ByBlock` have neither EED nor an extension dictionary. This graph is not
the disabled ADVMATERIAL scalar tail, and no such tail may be invented.

### Exact frame oracle

| handle / address | prefix / payload / total bytes | handle bits / data end / main end | string bits | common roles | fill / CRC |
| --- | --- | --- | ---: | --- | --- |
| `0x0096` / `0x5cd2` | 3 / 93 / 98 | 38 / 706 / 555 | 134 | owner code C -> `0x0072`; reactor code 4 -> `0x0072` | `111111` / `0x6f73` |
| `0x0097` / `0x5c70` | 3 / 93 / 98 | 38 / 706 / 555 | 134 | owner code C -> `0x0072`; reactor code 4 -> `0x0072` | `111111` / `0x6859` |
| `0x0098` / `0x5d34` | 3 / 181 / 186 | 60 / 1388 / 1253 | 118 | owner code C -> `0x0072`; reactor code 4 -> `0x0072`; xdictionary code 3 -> `0x0110` | `1111` / `0x0b82` |

All three use BOT selector 1 and direct self wire code 0. Their string streams are present and consume
exactly the reported content bits plus the 17-bit footer/presence encoding. The branch-level default
compression is deterministic writer state: unit/zero values mostly use compact `BD`/`BL` selectors,
while 0.5 and the four `Global` scale factors use full doubles. The EED byte count, string lengths/footer,
stream boundaries, compact branches, size prefix, handle codes, terminal fill and CRC are never schema
fields.

### Append-only facets and strict lifecycle gate

Preserve the P26 reservation for SORTENTSTABLE at Rust outer ordinal 21 / payload field 22 and Proto
tag 29. Append `Material(DwgMaterial)` next at Rust ordinal 22 / payload field 23 and Proto tag 30.
Mirror every color/map/mapper/procedural/channel enum and body arm through canonical, snapshot and diff
TypeScript/GraphQL/Proto/JSON and structured DSL/pack. GraphQL must use tagged object arms for map and
procedural sources; Proto must use `oneof`; JSON must reject additional properties. Mutation remains
SetSnapshot-based.

Acceptance requires 3/3 typed MATERIAL import, exact main/string/handle exhaustion, the complete
material-dictionary/EED/tile-XRECORD graph, three exact frames and original-fixture equality through
native export, snapshot DSL/pack, diff/apply/inverse/absorb, mutation/inverse, analyzer and composer.
Extend the existing lifecycle test to mutate/inverse one channel factor, one mapper transform, map source
None to a nonempty File and one enabled-channel Boolean; atomically rename a material with its owner
dictionary key; and preserve/remove/restore `Global` with its typed extension graph. Add a synthetic
structured procedural map to exercise every tagged codec without weakening exact fixture acceptance.

Anti-shadow/parity assertions must forbid packed `channel_flags`, raw `auto_transform`, numeric
projection/tiling/source/illumination/mode escapes, packed property records, generic procedural bytes or
strings, copied dictionary keys, duplicated tile references, advanced-tail bytes, native string/RGB
lengths, selector branches, stream/footer offsets, handle codes, fill, CRC and frame/payload bytes.

## P28: AC1024 type 521 `BLOCKMOVEACTION` exact-ready oracle (2026-08-15)

This is a read-only production audit. No production file was edited and no Cargo/Nx command was run.
An in-memory bounded probe reused the accepted EED, R2010 string-stream, common-object,
`DwgEvaluationExpression`, block-element, action-core, graph and CRC readers. It admitted exactly the
two dynamic type-521 frames and joined their dependencies and action-node identifiers to the already
decoded type-517 evaluation graph. LibreDWG's primary
[`AcDbBlockAction` and connection declarations](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3208-L3285)
and [`BLOCKMOVEACTION`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3471-L3478)
declaration establish the inheritance and field order, but the bounded fixture corrects that source's
final XY/angle macro as described below.

### Sole-authority typed action model

Reuse the accepted expression and action cores, with named connection and dependency concepts:

```text
DwgBlockActionConnection { code: u32, name: String }
DwgBlockActionDependency { expression_handle: u64 }
DwgBlockAction {
  evaluation_expression: DwgEvaluationExpression,
  name: String,
  display_location: [f64; 3],
  dependencies: Vec<DwgBlockActionDependency>,
  action_node_ids: Vec<u32>
}
DwgBlockMoveCoordinateMode = CartesianXY
DwgBlockMoveAction {
  action: DwgBlockAction,
  x_connection: DwgBlockActionConnection,
  y_connection: DwgBlockActionConnection,
  distance_multiplier: f64,
  angle_offset: f64,
  coordinate_mode: DwgBlockMoveCoordinateMode
}
```

The two connection positions are standard semantic X/Y roles, not a length-two anonymous native vector.
Their integer codes are standard connection identifiers validated against the owning graph and paired
with their names; they are not handle codes. `dependencies` and `action_node_ids` retain semantic order,
while their native counts are derived. A dependency is a resolvable expression object in the same
evaluation graph, not a generic handle bag. The fixture has exactly a controlling parameter followed by
its updated grip. The sole action-node identifier equals the controlling parameter's evaluation node ID.

### Native-tail correction: two doubles plus typed XY mode

The preliminary P10/P12 prescription of X offset `BD`, Y offset `BD`, angle offset `BD` is disproved by
both bounded frames. After the two connections, each frame contains exactly:

1. distance multiplier `BD = 0.5` using the full-double selector;
2. angle offset `BD = 0.0` using the compact-zero selector;
3. one native `RC = 0` Cartesian-XY mode marker.

Reading a third `BD` starts with that zero byte and overlaps the independent string stream by 58 bits.
Reading two `BD` plus one `RC` lands exactly on the main/string boundary in both frames. This matches the
standard interchange concepts group 140 distance multiplier, group 141 angle offset and group 280 action
XY type. Treat `CartesianXY` as a closed semantic enum; the AC1024 writer maps it to native zero (the DXF
projection uses its documented external value) and rejects unproved native modes. Do not retain the byte
as an unknown marker or a third scalar.

### Exact fixture values and evaluation-graph joins

Both inherited evaluation expressions have parent `-1`, version `29/2`, empty value discriminator
`-9999`, and no value handle. Their block-element version repeats `29/2`; the native application marker
is zero. Persist the version once in the accepted expression/core authority and derive the repeated
block-element values and marker.

| action | name / display location | expression node | dependencies in semantic order | action node IDs | X / Y connections | multiplier / angle / mode |
| ---: | --- | ---: | --- | --- | --- | --- |
| `0x1131` | `MoveHinge` / `(45,-15,0)` | 74 | `0x1118` BLOCKFLIPPARAMETER node 26; `0x1119` BLOCKFLIPGRIP node 27 | `[26]` | `(1,EndXDelta)` / `(1,EndYDelta)` | `0.5 / 0 / CartesianXY` |
| `0x1132` | `MoveSwing` / `(45,-20,0)` | 75 | `0x111d` BLOCKFLIPPARAMETER node 31; `0x1122` BLOCKFLIPGRIP node 50 | `[31]` | `(8,EndXDelta)` / `(8,EndYDelta)` | `0.5 / 0 / CartesianXY` |

The common owner for both is evaluation graph `0x110f`. That graph has storage slot 33
`(id=33,next_id=74,expression=0x1131)` and slot 34
`(id=34,next_id=75,expression=0x1132)`. The parameter/grip dependency pairs are the already-proved graph
slots 8/9 and 13/18. Require the action's expression node, both dependency expressions and every
`action_node_id` to exist in the owner graph; require the first dependency's node ID to equal the sole
action node ID and the second to be its typed updated-grip relation. Do not persist graph storage slots,
next-ID copies or dependency target node IDs beside these sole authorities.

All points and scalars are finite. Distance multiplier must be positive. Names are valid nonempty
Unicode, X/Y connection names are nonempty, dependency handles are unique, and action-node IDs are
unique. The R1 fixture contract requires two dependencies, one action-node ID and exactly the two named
X/Y connection fields; reject cardinality or graph mismatches atomically.

### Exact main/string/handle traversal and frame oracle

After BOT, self, zero EED terminator and common-object data, consume/write:

1. evaluation parent ID `BLd`, major/minor `BL`, value discriminator `BSd`, conditional value, node ID
   `BL`;
2. block-element name `T`, derived repeated major/minor `BL`, derived application marker `BL`;
3. display location `3BD`;
4. derived dependency count `BL` (dependency values are deferred to the handle stream);
5. derived action-node count `BL` and ordered action-node IDs `BL`;
6. X connection code `BL` / name `T`, then Y connection code `BL` / name `T`;
7. distance multiplier `BD`, angle offset `BD`, Cartesian-XY mode `RC`;
8. after the string footer, common owner followed by dependency expression handles in action order.

The independent string stream is exactly action name, X connection name, Y connection name. Each has
nine UTF-16 code units, so both frames contain 462 semantic string bits plus the 17-bit footer/presence
encoding. There is no EED, reactor, extension dictionary, expression-value handle or other class-local
handle.

| handle / address | prefix / payload / total bytes | handle bits / data end / main end | string bits | common/class roles | fill / CRC |
| --- | --- | --- | ---: | --- | --- |
| `0x1131` / `0x7176` | 3 / 119 / 124 | 70 / 882 / 403 | 462 | owner code C -> `0x110f`; dependencies code 4 -> `0x1118,0x1119` | `111111` / `0xddd5` |
| `0x1132` / `0x71f2` | 3 / 119 / 124 | 70 / 882 / 403 | 462 | owner code C -> `0x110f`; dependencies code 4 -> `0x111d,0x1122` | `111111` / `0x7936` |

Both use BOT selector 1, direct self wire code 0, zero reactors via compact BL selector 2 and a missing
extension dictionary. Counts, repeated version/marker, compact number branches, string lengths/footer,
stream boundaries, handle codes, size prefix, terminal fill and CRC are deterministic writer state.

### Append-only facets and strict lifecycle gate

Preserve P26 SORTENTSTABLE at Rust outer ordinal 21 / payload field 22 and Proto tag 29, and P27
MATERIAL at Rust ordinal 22 / field 23 and Proto tag 30. Append
`BlockMoveAction(DwgBlockMoveAction)` at Rust ordinal 23 / payload field 24 and Proto tag 31. Mirror the
connection, dependency, action core, coordinate-mode enum and body arm through canonical, snapshot and
diff TypeScript/GraphQL/Proto/JSON plus structured DSL/pack. Proto uses repeated typed dependencies and
action-node IDs but singular named X/Y connections. Mutation remains SetSnapshot-based.

Acceptance requires 2/2 typed imports, exact graph joins, exact main/string/handle exhaustion, both full
frames/CRCs and original-fixture equality through native export, snapshot DSL/pack,
diff/apply/inverse/absorb, mutation/inverse, analyzer and composer. Extend the lifecycle test to
mutate/inverse display location, multiplier, angle, one connection code/name and an action-node relation;
atomically swap a valid controlling parameter/grip pair within the same graph; and reject foreign-graph,
reversed, duplicate or cardinality-invalid dependencies and an unsupported coordinate mode.

Anti-shadow/parity assertions must require named X/Y connections, typed dependencies, graph node IDs,
distance multiplier, angle and coordinate mode while forbidding `num_deps`, `num_actions`, a third
`angle_offset`/raw-double slot, `action_offset_x/y`, raw XY marker, repeated version/marker copies,
graph storage slots, generic handle bags, string/footer offsets, handle codes, fill, CRC and frame bytes.

## P29: AC1024 alignment/grip/action batch exact-ready oracle (2026-08-15)

This is a read-only production audit. No production file was edited and no Cargo/Nx command was run.
One bounded in-memory probe reused the accepted common-object, EED, R2010 string-stream,
evaluation-expression, block-element, two-point-parameter, grip, action-core, graph, absolute-handle and
CRC readers. It admitted exactly 14 frames: type 533 `BLOCKALIGNMENTPARAMETER` x2, type 534
`BLOCKALIGNMENTGRIP` x2, type 535 `BLOCKSTRETCHACTION` x6, type 536 `BLOCKSCALEACTION` x1 and type 537
`BLOCKFLIPACTION` x3. Every frame exhausts main/string/handle streams and validates its stored CRC after
the two fixture-proven native-tail corrections below.

Primary declarations are LibreDWG's
[`AcDbBlockElement`, parameter, grip and action cores](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3208-L3360),
[`BLOCKALIGNMENTGRIP` and `BLOCKALIGNMENTPARAMETER`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3385-L3398),
[`BLOCKSCALEACTION` and `BLOCKFLIPACTION`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3480-L3520),
and [`BLOCKSTRETCHACTION`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L6165-L6192).

### Shared semantic corrections and sole-authority records

The integer carried by every block-action connection is an owning-graph evaluation **node ID**, not an
opaque connection code. Exact joins prove values 1, 8, 26, 31 and 120 all identify graph nodes whose
expression objects are the corresponding linear/flip parameters. Refine the P28 preliminary record to:

```text
DwgBlockActionConnection { node_id: u32, name: String }
DwgBlockActionDependency =
    EvaluationExpression { object_handle: u64 }
  | ControlledEntity { object_handle: u64 }
  | NestedAction { object_handle: u64 }
DwgBlockAction {
  evaluation_expression: DwgEvaluationExpression,
  name: String,
  display_location: [f64; 3],
  dependencies: Vec<DwgBlockActionDependency>,
  action_node_ids: Vec<u32>
}
```

The dependency tag is determined and validated from the resolved target's logical body; it is not a
copied type code. Preserve dependency order. Every connection/action/selector node ID must exist in the
same owner evaluation graph. Counts are derived.

The alignment records are:

```text
DwgBlockAlignmentParameter {
  parameter: DwgBlockTwoPointParameter,
  updated_grip_node_id: u32,
  align_perpendicular: bool
}
DwgBlockAlignmentGrip {
  grip: DwgBlockGrip,
  first_location_node_id: u32,
  second_location_node_id: u32,
  orientation: [f64; 3]
}
```

`updated_grip_node_id` is the sole logical authority for the first native property-state slot; derive
the native four-slot vector `(updated,0,0,0)`. The grip's two formerly unnamed `bg_bl91/bg_bl92` values
are graph node IDs for its two type-520 `BLOCKGRIPLOCATIONCOMPONENT` expressions, not generic state
integers. Derive all four empty property-group counts and the repeated block-element version/marker.

The action-specific records are:

```text
DwgStretchSelection { object_handle: u64, vertex_indices: Vec<u32> }
DwgStretchSelector { node_id: u32, point_indices: Vec<u32> }
DwgBlockStretchAction {
  action: DwgBlockAction,
  x_connection, y_connection,
  points: Vec<[f64; 2]>, selections: Vec<DwgStretchSelection>,
  selectors: Vec<DwgStretchSelector>,
  distance_multiplier: f64, angle_offset: f64,
  coordinate_mode: CartesianXY
}
DwgBlockActionWithBasePoint {
  action: DwgBlockAction,
  offset: [f64; 3], x_base_connection, y_base_connection,
  dependent: bool, base_point: [f64; 3]
}
DwgBlockScaleAction {
  base: DwgBlockActionWithBasePoint,
  uniform_scale_connection, x_scale_connection, y_scale_connection,
  mode: XY
}
DwgBlockFlipAction {
  action: DwgBlockAction,
  flip_connection, updated_flip_connection,
  updated_base_connection, updated_end_connection
}
```

As P28 proved for MOVEACTION, every STRETCHACTION suffix is distance multiplier `BD`, angle offset `BD`,
then Cartesian-XY `RC=0`; the preliminary three-double names are wrong. SCALEACTION likewise has one
fixture-proven trailing `RC=0` XY scale-mode byte after its three scale connections. LibreDWG's current
binary declaration omits this byte: without it main data stops eight bits before the string boundary.
Use a closed `XY` semantic mode and reject unproved values; never retain an unknown byte.

### Alignment parameter/grip exact graph and values

All four objects use empty evaluation values, parent `-1`, evaluator/block-element version `29/2`, class
marker zero, no property connections, show-properties true, chain-actions false and parameter-base
location zero. The exact logical graph is:

| parameter | owner graph / expression | definition base -> end | updated grip node | grip / expression | location; orientation | location-node IDs | cycling / weight |
| ---: | --- | --- | ---: | --- | --- | --- | --- |
| `0x1126` | `0x110f` / node 54 | `(0,-1.7763568394002505e-15,0)` -> `(15,-1.7763568394002505e-15,0)` | 55 | `0x1127` / node 55 | `(0,-1.7763568394002505e-15,0)`; `(15,0,0)` | 56 -> `0x1128/type520`; 57 -> `0x1129/type520` | true / -1 |
| `0x115c` | `0x1155` / node 192 | `(0,0,0)` -> `(6,7.347638122934264e-16,0)` | 200 | `0x115d` / node 200 | `(0,0,0)`; `(6,7.347638122934264e-16,0)` | 208 -> `0x115e/type520`; 216 -> `0x115f/type520` | false / -1 |

Both alignment parameters are named `Alignment`, have `align_perpendicular=true`, and native property
states `(55,0,0,0)` / `(200,0,0,0)`. Both grips are named `Grip`. Require graph membership for parameter,
grip and both location components; parameter updated-grip relation must resolve to the paired grip; grip
location must equal parameter definition base, and orientation must equal the definition direction
within exact finite-float semantics. Do not persist the three graph targets twice in generic reference
vectors.

Their exact frames are:

| handle / address / type | prefix / payload / total | handle bits / data end / main end | string bits | roles | fill / CRC |
| --- | --- | --- | ---: | --- | --- |
| `0x1126` / `0x6c0b` / 533 | 3 / 71 / 76 | 21 / 547 / 376 | 154 | owner code C -> `0x110f` | `11111` / `0x33e4` |
| `0x1127` / `0x6c57` / 534 | 3 / 56 / 61 | 19 / 429 / 338 | 74 | owner code C -> `0x110f` | `111` / `0x6ec3` |
| `0x115c` / `0x7851` / 533 | 3 / 63 / 68 | 21 / 483 / 312 | 154 | owner code C -> `0x1155` | `11111` / `0x8e99` |
| `0x115d` / `0x7895` / 534 | 3 / 56 / 61 | 19 / 429 / 338 | 74 | owner code C -> `0x1155` | `111` / `0xc16c` |

### STRETCHACTION exact logical selections

Every stretch action has two points, Cartesian-XY mode, distance multiplier 1.0 and X/Y connections
named `EndXDelta`/`EndYDelta`. The four angular state actions use connection node 1; the final general
stretch uses node 8. All selected indices are semantic subgeometry indices, not native offsets.

| action / node / name | display | ordered dependencies | action node IDs | points | ordered selections | selectors | angle |
| --- | --- | --- | --- | --- | --- | --- | ---: |
| `0x112a` / 65 / `Stretch0` | `(45,-1.7763568394002505e-15,0)` | `0x1126` alignment parameter, `0x111d` flip parameter, `0x1111` linear grip, `0x1143` LINE, `0x1139` LWPOLYLINE | `[31,54]` | `(34.5,4.5)`, `(26,-6.5)` | `0x1139:[1,2]`, `0x1143:[0,1]` | node 31:`[1]`, node 54:`[1]` | 0 |
| `0x112b` / 66 / `Stretch30` | `(45,13.458912962596134,0)` | `0x113a` LWPOLYLINE | `[]` | `(29,16)`, `(24,10.5)` | `0x113a:[1,2]` | none | `π/6` |
| `0x112c` / 67 / `Stretch45` | `(45,19.975766568519965,0)` | `0x113b` LWPOLYLINE | `[]` | `(24.5,23)`, `(19,17.5)` | `0x113b:[1,2]` | none | `π/4` |
| `0x112d` / 68 / `Stretch60` | `(45.00000000000001,25.06152106586386,0)` | `0x113c` LWPOLYLINE | `[]` | `(18.5,27.5)`, `(12.5,22.5)` | `0x113c:[1,2]` | none | `π/3` |
| `0x112e` / 69 / `Stretch90` | `(45.00000000000001,29.948914838437805,0)` | `0x113d` LWPOLYLINE | `[]` | `(3,32.5)`, `(-2,27)` | `0x113d:[1,2]` | none | `π/2` |
| `0x1130` / 72 / `Stretch` | `(45,-10,0)` | `0x1143` LINE, `0x1142` LINE | `[]` | `(31,-4)`, `(-1,-6)` | `0x1142:[1]`, `0x1143:[1]` | none | 0 |

`0x112a` connection node is 1; `0x112b` through `0x112e` also use 1; `0x1130` uses 8. Require
selected objects to resolve to typed geometry, every vertex index to be valid for that object's logical
subgeometry, selector point indices to be in `points`, selector node IDs to be in `action_node_ids` and
the owner graph, and selected objects to appear in the action's typed dependency set. Preserve point,
selection, index and selector order; reject duplicates and count multiplication overflow.

The exact STRETCHACTION frames are:

| handle / address | prefix / payload / total | handle bits / data end / main end | string bits | dependency roles then selection roles | fill / CRC |
| --- | --- | --- | ---: | --- | --- |
| `0x112a` / `0x6cfa` | 4 / 175 / 181 | 190 / 1210 / 747 | 446 | deps `1126,111d,1111,1143,1139`; selections `1139,1143`, all code 4 | `111111` / `0xfa7b` |
| `0x112b` / `0x6daf` | 3 / 155 / 160 | 68 / 1172 / 693 | 462 | dep `113a`; selection `113a`, code 4 | `1111` / `0xcc06` |
| `0x112c` / `0x6e4f` | 3 / 155 / 160 | 68 / 1172 / 693 | 462 | dep `113b`; selection `113b`, code 4 | `1111` / `0xea70` |
| `0x112d` / `0x6eef` | 3 / 155 / 160 | 68 / 1172 / 693 | 462 | dep `113c`; selection `113c`, code 4 | `1111` / `0x1680` |
| `0x112e` / `0x6f8f` | 3 / 155 / 160 | 68 / 1172 / 693 | 462 | dep `113d`; selection `113d`, code 4 | `1111` / `0x8a45` |
| `0x1130` / `0x70db` | 3 / 150 / 155 | 114 / 1086 / 639 | 430 | deps `1143,1142`; selections `1142,1143`, code 4 | `11` / `0x26ab` |

Every row also begins its handle stream with owner code C -> `0x110f`. Selection-object handles occur
after all inherited dependencies, even though their index records are in main data.

### SCALEACTION exact base-point action

The sole action `0x112f` is graph `0x110f` node 70, named `ScaleArc`, display `(45,-5,0)`, with no action
node IDs. Its ordered controlled-entity dependencies are ARC handles
`0x1141,0x1140,0x113f,0x113e`. Offset and base point are both `(0,0,0)`, `dependent=true`; base
connections are node 1 `UpdatedBaseX`/`UpdatedBaseY`, and scale connections are node 1
`Scale`/`XScale`/`YScale`. The typed mode is `XY` and materializes the proven final `RC=0`.

The frame at address `0x702f` is prefix/payload/total `3/167/172`, handle bits 115, data/handle boundary
1221, main/string boundary 360, string content 844 bits, owner code C -> `0x110f`, four dependency
handles code 4 in the order above, fill `111`, CRC `0x3fab`. Without the mode byte main ends at bit 352;
with it both streams exhaust exactly.

### FLIPACTION exact graph control sets

All four connections are named `Flip`, `UpdatedFlip`, `UpdatedBase`, `UpdatedEnd`, and each row uses its
single controlling node ID for all four. Exact values are:

| action / owner / node | name / display | connection node | action node IDs | ordered dependencies |
| --- | --- | ---: | --- | --- |
| `0x1133` / `0x110f` / 76 | `FlipHinge` / `(35,-14.999999999999996,0)` | 26 | `[79,65,66,67,68,69,1,8,31,46,54]` | `1127,1126,1122,111f,111e,111d,1115,1114,1111,1110,1143,1142,1141,1140,113f,113e,113d,113c,113b,113a,1139,112e,112d,112c,112b,112a,1135` |
| `0x1134` / `0x110f` / 77 | `FlipSwing` / `(35,-20.000000000000004,0)` | 31 | `[79,65,66,67,68,69,1,8,26,46,54]` | same list, except dependency position 5 is `1118` instead of `111d` |
| `0x115b` / `0x1155` / 184 | `Flip Window` / `(0,8.5,0)` | 120 | `[]` | `116e,116d` LINE entities |

The two 27-member vectors intentionally mix parameters/grips, controlled LINE/ARC/LWPOLYLINE entities,
nested stretch actions and a base-point parameter. Materialize the typed dependency union per target and
preserve this order. Every action/connection node ID must exist in the owner graph; connection node 26,
31 or 120 resolves to that row's controlling flip parameter. Reject a foreign node, a dependency from a
different graph/drawing, an untyped target or an action self-cycle not represented by the graph.

Exact FLIPACTION frames are:

| handle / address | prefix / payload / total | handle bits / data end / main end | string bits | roles | fill / CRC |
| --- | --- | --- | ---: | --- | --- |
| `0x1133` / `0x726e` | 4 / 238 / 244 | 670 / 1234 / 447 | 770 | owner C -> `110f`, 27 dependencies code 4 | `111111` / `0x5094` |
| `0x1134` / `0x7362` | 4 / 238 / 244 | 670 / 1234 / 447 | 770 | owner C -> `110f`, 27 dependencies code 4 | `111111` / `0x5c31` |
| `0x115b` / `0x77bc` | 3 / 144 / 149 | 68 / 1084 / 265 | 802 | owner C -> `1155`, dependencies `116e,116d` code 4 | `1111` / `0x90ba` |

### Common writer, append-only facets and lifecycle gate

All 14 frames use BOT selector 1, direct self wire code 0, parent `-1`, evaluator/block-element version
`29/2`, empty evaluation value, no EED, zero reactors via BL selector 2 and no extension dictionary. No
frame has an evaluation-value class handle. Native counts, repeated versions/marker, string unit counts
and footer, compact numeric branches, stream boundaries, handle codes, size prefixes, fill and CRC are
writer-derived.

Preserve P26–P28 reservations through BLOCKMOVEACTION Rust ordinal 23 / field 24 and Proto tag 31.
Append in dependency/type order: `BlockAlignmentParameter` Rust 24/field25 Proto32;
`BlockAlignmentGrip` 25/field26 Proto33; `BlockStretchAction` 26/field27 Proto34;
`BlockScaleAction` 27/field28 Proto35; `BlockFlipAction` 28/field29 Proto36. Mirror every shared core,
dependency union, named connection, selection/selector, mode and body arm through canonical, snapshot and
diff TypeScript/GraphQL/Proto/JSON plus structured DSL/pack. Mutation remains SetSnapshot-based.

Acceptance requires 14/14 typed imports, every graph/dependency/geometry join, exact stream exhaustion,
all exact frames/CRCs and original-fixture equality through native export, snapshot DSL/pack,
diff/apply/inverse/absorb, mutation/inverse, analyzer and composer. Extend the lifecycle test to
mutate/inverse alignment points/nodes, grip orientation/cycling, stretch points/selections/selectors,
scale base/mode and flip connection/dependency order; reject out-of-range geometry indices, foreign
graph nodes, split dependency/index counts, duplicate selections, unsupported modes and graph cycles.

Anti-shadow/parity assertions must forbid unnamed `bg_bl91/bg_bl92`, property-state arrays, connection
`code` bags, packed dependency handles, native counts, `HANDLE_UNKNOWN_BITS`, third-double action tails,
raw mode bytes, repeated version/marker copies, graph storage slots/next-ID duplicates, stream/footer
offsets, handle codes, fill, CRC and frame/payload bytes.

## P30: AC1024 singleton constraint/base-point and LAYOUT exact-ready oracle (2026-08-15)

This is a read-only production audit. No production file was edited and no Cargo/Nx command was run.
One bounded in-memory probe reused the accepted object-page, handle-map, common-object, typed EED,
R2010 string-stream, block-parameter, absolute-handle and CRC readers. It admitted exactly five frames:
type 538 `BLOCKBASEPOINTPARAMETER` x1, type 546 `BLOCKVERTICALCONSTRAINTPARAMETER` x1, type 548
`BLOCKHORIZONTALCONSTRAINTPARAMETER` x1 and fixed type 82 `LAYOUT` x2. Every row exhausts its main,
string and handle streams and validates the stored CRC.

Primary declarations are LibreDWG's
[`AcDbBlock1PtParameter` and `BLOCKBASEPOINTPARAMETER`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3308-L3315),
[`AcDbBlockLinearConstraintParameter` and horizontal/vertical derived classes`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg2.spec#L3357-L3368), and
[`LAYOUT`](https://github.com/LibreDWG/libredwg/blob/master/src/dwg.spec#L5316-L5443). The fixture is the
writer oracle where source labels or prescribed handle codes disagree with actual AC1024 bytes.

### Sole-authority logical records

Reuse P29's typed block element/parameter/property records and introduce only named standard concepts:

```text
DwgBlockBasePointParameter {
  parameter: DwgBlockOnePointParameter,
  point: [f64; 3],
  base_point: [f64; 3]
}
DwgBlockParameterAllowedValues { values: Vec<f64> }
DwgBlockLinearConstraintParameter {
  parameter: DwgBlockTwoPointParameter,
  displacement_grip_node_id: u32,
  dependency_handle: u64,
  expression_name: String,
  expression_description: String,
  value: f64,
  allowed_values: DwgBlockParameterAllowedValues
}
DwgBlockConstraintParameter =
    Vertical(DwgBlockLinearConstraintParameter)
  | Horizontal(DwgBlockLinearConstraintParameter)
```

The two derived constraint classes are separate tagged variants; do not persist a class/type integer or
an axis byte beside the tag. Their two named property connections share the same evaluation-node ID and
mean `DisplacementX` and `DisplacementY`. `displacement_grip_node_id` is the sole authority for that ID;
the writer derives both native connection IDs and the four-slot native property-state layout
`(0,id,0,0)`. The target graph expression resolves to a type-528 `BLOCKLINEARGRIP`.

Both constraint value sets have native flag word 8, meaning the allowed-value list is active. Persist
only the semantic allowed values. For a uniformly stepped nonempty list the deterministic writer derives
the native minimum, maximum and increment from the list; otherwise it emits zero helpers. This exactly
reconstructs the vertical native helpers `(4,6,1)` from `[4,5,6]` and the horizontal helpers `(0,0,0)`
from its nonuniform list, without retaining inactive duplicate range state. Counts and flag words are
derived.

LAYOUT is one tagged body with typed page/plot settings rather than parallel raw words:

```text
DwgLayout {
  page_setup_name, printer_configuration, canonical_media_name, stylesheet, name,
  plot_options: Set<DwgPlotOption>, margins, paper_size, plot_origin,
  paper_unit, rotation, plot_area, plot_window, custom_scale,
  standard_scale, paper_image_origin, shade_plot, tab_order,
  options: Set<DwgLayoutOption>, insertion_base, limits,
  ucs_origin, ucs_x_axis, ucs_y_axis, ucs_elevation, orthographic_view,
  extents, plot_view_handle, visual_style_handle, block_header_handle,
  active_viewport_handle, base_ucs_handle, named_ucs_handle,
  viewport_handles
}
```

The native first two `T` declarations are semantically `page_setup_name` then
`printer_configuration`: both fixture page-setup names are empty and the second strings are
`DWFx ePlot (XPS Compatible).pc3`. LibreDWG's `printer_cfg_file` / `paper_size` member labels are not a
reason to expose a semantically false API. In native string-stream order follow canonical media,
stylesheet and layout name. Use closed enums for inches, 90-degree rotation, display/layout plot area,
custom/1:1 standard scale, as-displayed shade plot, normal resolution and orthographic view. The fixture
plot-option words decompose to named options: both use standard scale, plot viewport borders, plot with
lineweights and draw viewports first; Model additionally uses model type, update paper and initializing.
The layout-option word 1 is the named paper-space-linetype-scaling option. No raw flag word is persisted.

Margins, paper dimensions/origins/windows/scales, limits/UCS/extents and custom DPI are genuine logical
values. Plot-view, visual-style, active viewport and UCS handles are optional semantic references; null
native handles become `None`, never retained `(code,0)` values. The ordered viewport vector is semantic
layout membership; its count is derived.

### BLOCKBASEPOINTPARAMETER exact graph and frame

Object `0x1135` belongs to evaluation graph `0x110f`. Its expression has parent -1, version 29/2, empty
value, node ID 79 and name `Base Point`; graph-local node 37 resolves to this object and has next/evaluation
ID 79. Show-properties is true, chain-actions false. Definition point, class point and base point are all
`(0,0,0)`. Both property connection lists are empty and the declared native property-info count is zero.
P29's two flip actions reference this parameter as a typed evaluation-expression dependency.

The frame at address `0x7456` uses prefix/payload/total `3/47/52`, handle bits 20, data/handle boundary
356, main/string boundary 169 and 170 string-content bits. BOT selector is 1, self handle code is 0,
EED is empty, reactors are zero via BL selector 2, extension dictionary is missing, and the sole handle
role is owner code C -> graph `0x110f`. Terminal fill is `1111`; CRC is `0xe590`.

### Vertical and horizontal constraint exact graph

Both constraints belong to evaluation graph `0x1155`, use parent -1, version 29/2, empty evaluation
value, show-properties true, chain-actions false, parameter base location start and empty first/second
property groups. Exact logical values are:

| object / expression | axis name; definition base -> end | displacement grip | expression value | allowed values | dependency chain |
| --- | --- | --- | ---: | --- | --- |
| `0x1160` / node 222 | `H`; `(0,-4,0)` -> `(0,0,0)` | node 223 -> `0x1161` type 528 | `-15.198233639336081` | `[4,5,6]` | `0x114b -> 0x2028 -> 0x2081` |
| `0x1165` / node 227 | `W`; `(0,-2,0)` -> `(36,-2,0)` | node 228 -> `0x1166` type 528 | `-19.6528394930898` | `[12,24,30,36,42,48,54,60]` | `0x1151 -> 0x202e -> 0x2087` |

Expression descriptions are empty. Third/fourth properties are exactly named `DisplacementX` and
`DisplacementY` and use the row's grip node ID. Graph-local nodes 10/11 resolve vertical parameter/grip
with evaluation IDs 222/223; nodes 15/16 resolve horizontal parameter/grip with IDs 227/228.

The `dependency_handle` is the first type-542 object in the corresponding three-link parameter chain.
All three chain objects also occur as common reactors in chain order. Their type-543 bodies and
previous/next joins are already proved in the parameter-dependency-body oracle. Import must validate
that all three dependencies target this parameter, that the first dependency has no previous link, the
last no next link, bodies point back, and the parameter's class handle equals the first chain member.
The fixture encodes that class role with wire code 4 despite LibreDWG's prescribed code 5; the writer
must derive the fixture-proven role encoding and the schema/facets must never retain either code.

Exact frames are:

| handle / address / type | prefix / payload / total | handle bits / data end / main end | string bits | ordered roles | fill / CRC |
| --- | --- | --- | ---: | --- | --- |
| `0x1160` / `0x7938` / 546 | 3 / 166 / 171 | 112 / 1216 / 709 | 490 | owner C -> `1155`; reactors code 4 -> `114b,2028,2081`; dependency code 4 -> `114b` | empty / `0xc0a6` |
| `0x1165` / `0x7a9f` / 548 | 3 / 208 / 213 | 118 / 1546 / 1039 | 490 | owner C -> `1155`; reactors code 4 -> `1151,202e,2087`; dependency code 4 -> `1151` | `111111` / `0x7396` |

Both use BOT selector 1, self code 0, empty EED, reactor count 3 via BL selector 1 and a missing extension
dictionary. Native property counts/states, flag/range helpers, compact selectors, stream boundaries,
handle codes, size prefix, fill and CRC are writer-derived.

### LAYOUT exact values, references and frames

Both layouts have one typed EED record owned by APPID `0x28e` (type 67): the semantic values are the
printer configuration, its display name without `.pc3`, `File`, an empty string and integer 0. Preserve
these through the existing typed EED union and application reference; do not copy their native size,
group-code bytes or UTF-16 representation into the layout body.

| field | Model `0x22` | Layout1 `0x59` |
| --- | --- | --- |
| plot options | native 11952; named common four plus model-type/update-paper/initializing | native 688; named common four |
| margins L/B/R/T | `5.793749809265137,17.793750762939453,5.7937469482421875,17.79376220703125` | same |
| paper W/H; media | `215.89999389648438,279.3999938964844`; `ANSI_A_(8.50_x_11.00_Inches)` | same |
| origin/window; units | zero; inches, drawing custom `14.618923835299519` | zero; inches, drawing 1 |
| stylesheet; standard scale | empty; custom factor `0.06840448799557697` | `acad.ctb`; 1:1 factor 1 |
| paper-image origin | `(138.5828241861512,90.29440047654174)` | `(0,0)` |
| name/tab/options | `Model`, 0, paper-space-linetype-scaling | `Layout1`, 1, paper-space-linetype-scaling |
| limits min/max | `(0,0)` / `(12,9)` | `(-0.7005418191744587,-0.22810038619154083)` / `(10.299457940529651,8.27189937351257)` |
| extents min/max | zero / zero | `(0.6288321226941207,0.7996673279882671,-1.1249790867928804e-11)` / `(9.029821079143858,7.2001602728074445,2.955929880312366e-13)` |

Both use printer configuration `DWFx ePlot (XPS Compatible).pc3`, plot origin/window zero, 90-degree
rotation, shade as displayed at normal resolution/300 DPI, zero insertion base and UCS origin/elevation,
UCS axes X `(1,0,0)` and Y `(0,1,0)`, and no base/named UCS or plot-view/visual-style reference.

The exact logical reference graphs are:

| layout | owner / reactor / xdictionary | block header | active viewport | viewport membership |
| --- | --- | --- | --- | --- |
| `0x22` Model | dictionary `0x1a` / `0x1a` / dictionary `0x108d` | `0x1f` | `0x94` fixed viewport | empty |
| `0x59` Layout1 | dictionary `0x1a` / `0x1a` / dictionary `0x2a7` | `0x58` | `0x28b` VIEWPORT entity | `[0x28b,0x290]` VIEWPORT entities |

Require each block header's typed layout reference to point back. The active viewport need not be a
member of Model's empty R2004+ viewport vector; Layout1's active viewport is its first member. Reject a
duplicate viewport, a target of the wrong type, a missing reciprocal block link, nonfinite geometry or
an enum outside the R2010 domain.

Exact frames are:

| handle / address | prefix / payload / total | handle bits / data end / main end | string bits | role codes | fill / CRC |
| --- | --- | --- | ---: | --- | --- |
| `0x22` / `0x5abe` | 3 / 429 / 434 | 124 / 3308 / 2233 | 1058 | owner C -> `1a`; reactor 4 -> `1a`; xdic 3 -> `108d`; null plot-view 5; null visual style 4; block 4 -> `1f`; active 4 -> `94`; null UCS 5/5 | `1111` / `0x6b5c` |
| `0x59` / `0x58db` | 4 / 477 / 483 | 180 / 3636 / 2393 | 1226 | owner 4 -> `1a`; reactor 4 -> `1a`; xdic 3 -> `2a7`; null plot-view 5; null visual style 4; block 4 -> `58`; active 4 -> `28b`; null UCS 5/5; viewports 4 -> `28b,290` | `1111` / `0x1bca` |

LAYOUT uses BOT selector 0, self code 0, reactor count 1 via BL selector 1 and a present extension
dictionary. Null role encodings, EED framing, native option words, viewport count, compact numeric
branches, stream footer/boundaries, handle codes, size prefixes, fill and CRC are all derived.

### Append-only facets and lifecycle gate

Preserve P29 through BLOCKFLIPACTION Rust ordinal 28 / artifact field 29 / Proto oneof tag 36. Append
`BlockBasePointParameter` at Rust 29 / field 30 / Proto 37, `BlockVerticalConstraintParameter` at
30/31/38, `BlockHorizontalConstraintParameter` at 31/32/39 and `Layout` at 32/33/40. Mirror each shared
parameter/value-set/layout enum/option/reference record and body arm through canonical, snapshot and diff
TypeScript/GraphQL/Proto/JSON plus structured DSL/pack. Mutation remains SetSnapshot-based.

Acceptance requires 5/5 typed imports, exact stream exhaustion and CRC, every evaluation/dependency/
block/layout/viewport/EED join and original-fixture equality through native export, snapshot DSL/pack,
diff/apply/inverse/absorb, mutation/inverse, analyzer and composer. Mutate/inverse the three base-point
coordinates, constraint definitions/value/allowed-values/grip/dependency, layout plot enums/options/
geometry and viewport membership. Reject a nonuniform list encoded as an active numeric range, a foreign
graph grip, an incomplete dependency chain, invalid plot enum/option, broken reciprocal block link and
duplicate/wrong-type viewport.

Anti-shadow/parity assertions must forbid raw value-set/plot/layout flag words, native range helpers,
property-state arrays, duplicated connection IDs, native counts, class/type discriminators beside body
tags, EED group/storage bytes, null-handle encodings, handle codes, graph storage IDs, stream/footer
offsets, compact-selector branches, fill, CRC and frame/payload bytes.
