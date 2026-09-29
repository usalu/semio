#!/usr/bin/env python3
"""🧾️ LB2 p11 (window 3, stdio guest content): the snapshot contracts a stdio editor validates its edits against resolve.

- `$ref` integrity: the las `1.0/header` and ifc `4/any` snapshot schemas point at `#/definitions/<Name>` while the named
  subschemas live under `$defs`; the dwg `ac1024/any` snapshot and diff schemas point at camelCase `$defs` of the artifact
  schema whose subschemas are named `Dwg<PascalCase>`. `OwnedJsonSchemaValidator::compile` refuses the contract
  (`snapshot-edit.invalid-schema-contract … unresolved reference`), so every las/dwg (and, once published, ifc 4) edit was refused.
  The fix renames the REFERENCE to the existing subschema (never the subschema), only when exactly one target exists.
- dxf `r12/header` snapshot schema: `DxfTables` REQUIRED `layers`/`styles`/`linetypes`, while the model defaults each to empty and
  omits it when empty (`#[value(default, skip_serializing_if = "Vec::is_empty")]`) — a new document (no tables) failed its own
  contract (`$.tables: missing required property layers`). Absent == empty is the model's contract, so the schema drops `required`.

- Option fields vs their contracts (schema-first: the JSON schema and its TS mirror declare these properties OPTIONAL and
  non-null, the Rust projection emitted `null`): png's 8 optional chunks, jpg's 4 optional segments/hints, xml's `doc.root`,
  ifc 2x3's `edmPreamble` now omit
  `None` (`skip_serializing_if = "Option::is_none"`), so a new document validates against its own contract
  (`schema.fragment.invalid … expected integer at /gama`).
- ifc 2x3's snapshot contract had `schema` and `document` SWAPPED (`schema: Part21Document`, `document: string`; its TS/GraphQL/
  Proto twins have them right) → swapped back. (Its `Part21*` definitions still describe a kind-tagged camelCase Part-21 the shared
  `step::part21` types never project — reported, not fixed here.)
- xml valid: a new document is the minimal VALID document (`<!DOCTYPE root><root/>`, `blank_valid_xml_snapshot`) — the subset
  refuses every edit/undo landing on an invalid one (`inverse-mismatch` on the empty document's undo).
- committed mutation fixtures of those kinds (png 30, jpg 20, dwg 6, ifc 2x3 3 files) spelled the omitted fields as `null` →
  the `null` members are removed as a text edit (every other byte as committed; proven structural), so `committed_json_is_canonical`
  holds; the ifc 2x3 shadow-state law's key list and the epw set-cell law's fixture (now `blank_epw_snapshot()`, a codec fixed
  point — the old `records`-only document had no header records and did not load) follow.
- laws that gave an absent optional field its value with `SetValue` (png `/gama`, jpg `/reEncodeQuality` patch fixtures and the
  jpg large-raster law) insert it (`InsertValue`, RFC 6902 `add`; a present value is still replaced); the png typed-source law
  replaces the demo with the edited source (replacing a document with its own source is correctly a no-op).
- binary's snapshot schema typed `bytes` as a string while its projection (and its TS mirror) is a byte array → array of 0..255.
- dwg `ac1024` artifact schema named the header policy property `dwf_3dPrecision`; the model projects `dwf3dPrecision`
  (camelCase of `dwf_3d_precision`, as every committed fixture shows) → every dwg edit failed `missing required property`.
- avi's snapshot derived `Default` with an EMPTY schema identity (decode defaults it to `stdio.avi`) → explicit `Default`, whose
  main header carries the four zero `dwReserved` DWORDs the writer emits (an empty list saved as four zeros → a different document).
- dwg `DwgHeaderRelations`: the 16 optional handles are optional non-null in the contract and its TS mirror (`paperUcsName?:
  number`) but projected `null` → omitted when `None` (`$.header.relations.paperUcsName: expected integer`, every dwg edit refused).
- wav's default document declared a 16-bit PCM `fmt` but carried `Raw` sample data — not a decode fixed point (a saved new
  document reopened as `pcm16`) → the default data is the empty sample list the default `fmt` describes.
- new documents of kinds whose native file cannot be empty (`EditorApp::default()` panicked `default-options pack encode is
  infallible`, hub genesis could not create them): jpg, tiff, gif 87a/89a are one opaque white pixel, epw is the eight
  header records plus one data record, png one opaque white RGBA pixel (`IHDR` has no zero dimension) — each the fixed point
  of the kind's REAL codec (`decode(encode(seed))`), so a new
  document saves and reopens as itself. The dead `empty_*_snapshot()` (== an invalid `Default`, zero callers) become these
  `blank_*_snapshot()` builders; every editor/viewer `initial_snapshot()` of those kinds returns them.
- pptx's new document was the `Default`: an EMPTY OPC package beside a typed presentation — saving materialized the whole
  package, so a new document never reopened as itself; the six pptx apps now open `blank_pptx_snapshot()`, the minimal package
  the real writer builds (docx's own precedent, `build_minimal_docx`); `Default` stays the empty package builders and the
  subset analyzers' laws start from (the dead async `empty_pptx_snapshot` is replaced).

usage: python3 lb2-p11-editor-documents.py --dry-run | --write | --revert [--root <tree>]
"""
import glob, hashlib, json, os, re, shutil, sys

TREE = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
BACKUP = f"/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-backup/p11/{hashlib.sha256(TREE.encode()).hexdigest()[:12]}"
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
DWG_ARTIFACT = f"{ART}/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🔣️.json"
DWG_ID = "https://json.schemas.assets.semio-tech.com/s/stdio/dwg/ac1024/any/artifact.json"
LOCAL = [f"{ART}/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/📸️snapshot/🔣️.json", f"{ART}/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json"]
DXF = f"{ART}/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/📸️snapshot/🔣️.json"
AVI = f"{ART}/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/📸️snapshot/🦀️.rs"
WAV = f"{ART}/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"
PNG = f"{ART}/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"
PNG_SCHEMA = f"{ART}/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🦀️.rs"
JPG = f"{ART}/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🦀️.rs"
XML = f"{ART}/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs"
IFC2X3 = f"{ART}/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs"
BINARY = f"{ART}/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json"
SKIP = '#[value(default, skip_serializing_if = "Option::is_none")]'
DWG = [f"{ART}/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json", f"{ART}/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json"]

problems = []


def local_defs(text, label):
    document = json.loads(text)
    defs = set(document.get("$defs", {}))
    definitions = set(document.get("definitions", {}))

    def fix(match):
        name = match.group(1)
        if name in definitions:
            return match.group(0)
        if name not in defs:
            problems.append(f"{label}: #/definitions/{name} has no target")
            return match.group(0)
        return f'"$ref": "#/$defs/{name}"'

    return re.sub(r'"\$ref": "#/definitions/([A-Za-z0-9_]+)"', fix, text)


def dwg_defs(text, label, artifact_defs):
    def fix(match):
        name = match.group(1)
        if name in artifact_defs:
            return match.group(0)
        pascal = "Dwg" + name[0].upper() + name[1:]
        if pascal not in artifact_defs:
            problems.append(f"{label}: {name} has no Dwg-named target")
            return match.group(0)
        return f'"$ref": "{DWG_ID}#/$defs/{pascal}"'

    return re.sub(r'"\$ref": "' + re.escape(DWG_ID) + r'#/\$defs/([A-Za-z0-9_]+)"', fix, text)


def dxf_tables(text, label):
    document = json.loads(text)
    tables = document["$defs"]["DxfTables"]
    if tables.get("required") != ["layers", "styles", "linetypes"]:
        problems.append(f"{label}: DxfTables.required is {tables.get('required')}")
        return text
    del tables["required"]
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def once(text, old, new, label):
    if text.count(old) != 1:
        problems.append(f"{label}: anchor x{text.count(old)}")
        return text
    return text.replace(old, new)


def optional_fields(text, fields, label):
    for field in fields:
        text = once(text, f"    #[value(default)]\n    pub {field}: Option<", f"    {SKIP}\n    pub {field}: Option<", f"{label} {field}")
    return text


def avi_default(text):
    text = once(text, '#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]\n#[value(rename_all = "camelCase")]\n#[artifact_schema(id = "s.stdio.avi")]\n', '#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]\n#[value(rename_all = "camelCase")]\n#[artifact_schema(id = "s.stdio.avi")]\n', "avi derive")
    return once(
        text,
        "fn default_schema() -> String {\n    STDIO_AVI_DOCUMENT_SCHEMA.into()\n}\n",
        "fn default_schema() -> String {\n    STDIO_AVI_DOCUMENT_SCHEMA.into()\n}\n\n/// 🆕️ A new avi document: `stdio.avi`, and a main header whose `dwReserved[4]` are the four zero DWORDs every written\n/// `avih` carries — the empty `reserved` list saved as four zeros and reopened as a different document.\nimpl Default for AviSnapshot {\n    fn default() -> Self {\n        Self { schema: default_schema(), main_header: AviMainHeader { reserved: vec![0; 4], ..AviMainHeader::default() }, streams: Vec::new(), idx1_present: false, unknown_chunks: Vec::new(), hdrl_extra: Vec::new() }\n    }\n}\n",
        "avi default",
    )


def wav_default(text):
    return once(text, "fmt: WavFmt::default(), data: WavData::default(),", "fmt: WavFmt::default(), data: WavData::Pcm16(Vec::new()),", "wav default")


def binary_bytes(text):
    document = json.loads(text)
    if document["properties"]["bytes"] != {"type": "string", "x-semio-state": "artifact"}:
        problems.append(f"binary: bytes is {document['properties']['bytes']}")
        return text
    document["properties"]["bytes"] = {"type": "array", "items": {"type": "integer", "minimum": 0, "maximum": 255}, "x-semio-state": "artifact"}
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def dwg_policy(text):
    if text.count('"dwf_3dPrecision"') != 2:
        problems.append(f"dwg: dwf_3dPrecision x{text.count(chr(34) + 'dwf_3dPrecision' + chr(34))}")
        return text
    return text.replace('"dwf_3dPrecision"', '"dwf3dPrecision"')


R9 = "// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9\n"
BLANKS = {
    f"{ART}/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🦀️.rs": (
        "JpgSnapshot",
        "jpg",
        "/// 🆕️ A new jpg document: one opaque white pixel as the real codec round-trips it — JPEG has no empty image (T.81 §B.2.2:\n"
        "/// a frame is at least 1×1), and a new document must save and reopen as itself.\n",
        "    use crate::standards::v_jfif_1_01::subsets::document::io::{decode_jpg, encode_jpg};\n"
        "    let seed = JpgSnapshot { width: 1, height: 1, pixels: vec![255, 255, 255, 255], ..JpgSnapshot::default() };\n"
        "    encode_jpg(&seed).and_then(|bytes| decode_jpg(&bytes)).expect(\"blank_jpg_snapshot: the 1×1 seed round-trips through the real codec\")\n",
    ),
    f"{ART}/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🦀️.rs": (
        "TiffSnapshot",
        "tiff",
        "/// 🆕️ A new tiff document: one opaque white pixel in one IFD as the real codec round-trips it — baseline TIFF has no\n"
        "/// image without `ImageWidth`/`ImageLength` (TIFF 6.0 §8), and a new document must save and reopen as itself.\n",
        "    use crate::standards::v6_0::subsets::document::io::{decode_tiff, encode_tiff};\n"
        "    use crate::standards::v6_0::subsets::document::schema::snapshot::{TiffFieldType, TiffTag, TiffValues, TAG_IMAGE_LENGTH, TAG_IMAGE_WIDTH};\n"
        "    let geometry = vec![TiffTag { tag: TAG_IMAGE_WIDTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![1]) }, TiffTag { tag: TAG_IMAGE_LENGTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![1]) }];\n"
        "    let seed = TiffSnapshot { ifds: vec![TiffIfd { pixels: Vec::new(), entries: geometry }], pixels: vec![255, 255, 255, 255], ..TiffSnapshot::default() };\n"
        "    encode_tiff(&seed).and_then(|bytes| decode_tiff(&bytes)).expect(\"blank_tiff_snapshot: the 1×1 seed round-trips through the real codec\")\n",
    ),
    f"{ART}/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧬️schema/🦀️.rs": (
        "GifSnapshot",
        "gif",
        "/// 🆕️ A new gif (87a) document: a 1×1 logical screen with one image of one white pixel as the real codec round-trips it —\n"
        "/// GIF87a has no empty screen nor an image-less stream, and a new document must save and reopen as itself.\n",
        "    use crate::standards::v87a::subsets::any::io::{decode_gif, encode_gif};\n"
        "    use crate::standards::v87a::subsets::any::schema::snapshot::{GifColorTable, GifImage, GifRgb};\n"
        "    let white = GifRgb { r: 255, g: 255, b: 255 };\n"
        "    let seed = GifSnapshot { width: 1, height: 1, gct: Some(GifColorTable { sorted: false, colors: vec![white, white] }), images: vec![GifImage { width: 1, height: 1, indices: vec![0], ..GifImage::default() }], ..GifSnapshot::default() };\n"
        "    encode_gif(&seed).and_then(|bytes| decode_gif(&bytes)).expect(\"blank_gif_snapshot: the 1×1 seed round-trips through the real codec\")\n",
    ),
    f"{ART}/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/🦀️.rs": (
        "GifSnapshot",
        "gif",
        "/// 🆕️ A new gif (89a) document: a 1×1 logical screen with one frame of one white pixel as the real codec round-trips it —\n"
        "/// GIF89a has no empty screen nor a frame-less stream, and a new document must save and reopen as itself.\n",
        "    use crate::standards::v89a::subsets::any::io::{decode_gif, encode_gif};\n"
        "    use crate::standards::v89a::subsets::any::schema::snapshot::{GifColorTable, GifFrame, GifRgb};\n"
        "    let white = GifRgb { r: 255, g: 255, b: 255 };\n"
        "    let seed = GifSnapshot { width: 1, height: 1, gct: Some(GifColorTable { sorted: false, colors: vec![white, white] }), frames: vec![GifFrame { width: 1, height: 1, indices: vec![0], ..GifFrame::default() }], ..GifSnapshot::default() };\n"
        "    encode_gif(&seed).and_then(|bytes| decode_gif(&bytes)).expect(\"blank_gif_snapshot: the 1×1 seed round-trips through the real codec\")\n",
    ),
}
BLANKS[PNG_SCHEMA] = (
    "PngSnapshot",
    "png",
    "/// 🆕️ A new png document: one opaque white RGBA pixel as the real codec round-trips it — `IHDR` has no zero dimension\n"
    "/// (PNG §11.2.2), and a new document must save and reopen as itself.\n",
    "    use crate::standards::v1_2::subsets::any::io::{decode_png, encode_png};\n"
    "    let seed = PngSnapshot { width: 1, height: 1, pixels: vec![255, 255, 255, 255], ..PngSnapshot::default() };\n"
    "    encode_png(&seed).and_then(|bytes| decode_png(&bytes)).expect(\"blank_png_snapshot: the 1×1 seed round-trips through the real codec\")\n",
)
PNG_EMPTY_DOC = "/// 🕳️ Relocated verbatim from `⚙️engine` (ticket\n/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, rule 5: pure helpers over document\n/// types live in `🧬️schema/`).\n"
PNG_DOC_MENTIONS = [f"{ART}/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🦀️.rs", f"{ART}/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🦀️.rs", f"{ART}/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
XML_VALID_SCHEMA = f"{ART}/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/🦀️.rs"
IFC2X3_CONTRACT = f"{ART}/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json"
DWG_SNAPSHOT = f"{ART}/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"
INITIAL = {
    "pptx": ("PptxSnapshot::default()", "crate::standards::v_ecma_376::subsets::base::schema::blank_pptx_snapshot()", [f"📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/{subset}" for subset in ("🧱️base", "🔒️strict", "🌉️transitional")]),
    "xml-valid": ("XmlSnapshot::default()", "crate::standards::v1_0::subsets::valid::schema::blank_valid_xml_snapshot()", ["📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid"]),
    "png": ("PngSnapshot::default()", "crate::standards::v1_2::subsets::any::schema::blank_png_snapshot()", ["📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any"]),
    "jpg": ("JpgSnapshot::default()", "crate::standards::v_jfif_1_01::subsets::document::schema::blank_jpg_snapshot()", ["📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document", "📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline"]),
    "tiff": ("TiffSnapshot::default()", "crate::standards::v6_0::subsets::document::schema::blank_tiff_snapshot()", ["🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document", "🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline"]),
    "gif87a": ("GifSnapshot::default()", "crate::standards::v87a::subsets::any::schema::blank_gif_snapshot()", ["🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any"]),
    "gif89a": ("GifSnapshot::default()", "crate::standards::v89a::subsets::any::schema::blank_gif_snapshot()", ["🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base"]),
    "epw": ("EpwSnapshot::default()", "crate::standards::energyplus::subsets::any::schema::blank_epw_snapshot()", ["🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any"]),
}
PPTX_SCHEMA = f"{ART}/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🦀️.rs"
PPTX_ROOT = f"{ART}/📽️pptx/🦀️.rs"
EPW_SCHEMA = f"{ART}/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🦀️.rs"


def blank(text, label, kind, artifact, doc, body):
    old = f"{PNG_EMPTY_DOC if artifact == 'png' else ''}{R9}pub fn empty_{artifact}_snapshot() -> {kind} {{\n    {kind}::default()\n}}\n"
    text = once(text, old, f"{doc}{R9}pub fn blank_{artifact}_snapshot() -> {kind} {{\n{body}}}\n", f"{label} blank")
    return text.replace(f"`empty_{artifact}_snapshot`", f"`blank_{artifact}_snapshot`")


def blank_epw(text):
    anchor = "//#region 🧬️DerivedArtifactFacets\n"
    region = (
        "//#region 🆕️NewDocument\n"
        "/// 🆕️ A new epw document: the eight EnergyPlus header records (empty location, no design conditions / periods / ground\n"
        "/// temperatures / holidays / comments, no data period) and one data record, as the real codec round-trips it — `decode_epw`\n"
        "/// requires all eight header records and at least one record, and a new document must save and reopen as itself.\n"
        + R9
        + "pub fn blank_epw_snapshot() -> EpwSnapshot {\n"
        "    use crate::standards::energyplus::subsets::any::io::{decode_epw, encode_epw};\n"
        "    let seed = EpwSnapshot {\n"
        "        design_conditions: \"DESIGN CONDITIONS,0\".into(),\n"
        "        typical_extreme_periods: \"TYPICAL/EXTREME PERIODS,0\".into(),\n"
        "        ground_temperatures: \"GROUND TEMPERATURES,0\".into(),\n"
        "        holidays_dst: \"HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0\".into(),\n"
        "        comments_1: \"COMMENTS 1,\".into(),\n"
        "        comments_2: \"COMMENTS 2,\".into(),\n"
        "        records: vec![EpwRecord::default()],\n"
        "        ..EpwSnapshot::default()\n"
        "    };\n"
        "    decode_epw(&encode_epw(&seed)).expect(\"blank_epw_snapshot: the seed round-trips through the real codec\")\n"
        "}\n"
        "//#endregion 🆕️NewDocument\n"
        "\n"
    )
    return once(text, anchor, region + anchor, "epw blank")


def xml_valid_blank(text):
    anchor = "pub use valid_mutations::{apply_xml_valid_mutation, inverse_xml_valid_mutation, XmlValidMutation, KINDS as VALID_MUTATION_KINDS};\n"
    blank = (
        "\n"
        "/// 🆕️ A new valid xml document: `<!DOCTYPE root><root/>` as the real parser reads it — XML 1.0 §5.1 validity needs a DOCTYPE\n"
        "/// whose Name is the document element's; the empty document has neither, and this subset refuses every edit (and every\n"
        "/// undo) that would land on an invalid document, so an empty new document could never be edited.\n"
        f"{R9}"
        "pub fn blank_valid_xml_snapshot() -> crate::XmlSnapshot {\n"
        "    let doc = crate::schema::snapshot::xml_document_from_text(\"<!DOCTYPE root><root/>\").expect(\"blank_valid_xml_snapshot: the minimal valid document parses\");\n"
        "    crate::XmlSnapshot { doc, ..crate::XmlSnapshot::default() }\n"
        "}\n"
    )
    return once(text, anchor, anchor + blank, "xml valid blank")


def ifc2x3_contract(text):
    document = json.loads(text)
    properties = document["properties"]
    if properties["schema"].get("$ref", "").endswith("#/$defs/Part21Document") and properties["document"] == {"type": "string", "x-semio-state": "artifact"}:
        properties["schema"], properties["document"] = {"type": "string", "x-semio-state": "artifact"}, {"$ref": properties["schema"]["$ref"], "x-semio-state": "artifact"}
    else:
        problems.append("ifc 2x3 contract: schema/document are not swapped")
        return text
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def pptx_blank(text):
    return once(
        text,
        "pub async fn empty_pptx_snapshot() -> PptxSnapshot {\n    PptxSnapshot::default()\n}\n",
        "/// 🆕️ A new pptx document: the minimal presentation package the real writer builds (`build_minimal_pptx`, docx's\n"
        "/// precedent) — the empty `Default` package saved as a materialized package and reopened as a different document.\n"
        f"{R9}"
        "pub fn blank_pptx_snapshot() -> PptxSnapshot {\n"
        "    crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_pptx(PptxPresentation::default())\n"
        "}\n",
        "pptx blank",
    )


NULL_OMITTED = {
    "📷️png": {"plte", "trns", "gama", "chrm", "srgb", "phys", "time", "bkgd"},
    "📸️jpg": {"reEncodeQuality", "jfifThumbnail", "frame", "restartInterval"},
    "🖊️dwg": {"paperUcsName", "paperUcsOrthographicReference", "paperUcsBase", "modelUcsName", "modelUcsOrthographicReference", "modelUcsBase", "dimensionLeaderBlock", "dimensionBlock", "dimensionBlock1", "dimensionBlock2", "dimensionLinetype", "dimensionExtensionLinetype1", "dimensionExtensionLinetype2", "interfereObjectVisualStyle", "interfereViewportVisualStyle", "dragVisualStyle"},
    "🏗️ifc": {"edmPreamble"},
}


def without_nulls(node, keys):
    if isinstance(node, dict):
        return {key: without_nulls(value, keys) for key, value in node.items() if not (key in keys and value is None)}
    if isinstance(node, list):
        return [without_nulls(value, keys) for value in node]
    return node


def committed_fixture(text, keys, label):
    """🧫️ A committed fixture JSON in the canonical projection: the optional fields p11 omits when `None` lose their `null`
    lines — a text edit (every other byte, number spelling included, stays as committed), proven equal to the re-parsed
    document with those keys dropped."""
    lines = text.split("\n")
    out = []
    for line in lines:
        found = re.match(r'^(\s*)"([A-Za-z0-9]+)": null(,?)$', line)
        if found and found.group(2) in keys:
            if not found.group(3) and out and out[-1].endswith(","):
                out[-1] = out[-1][:-1]
            continue
        out.append(line)
    result = "\n".join(out)
    if json.loads(result) != without_nulls(json.loads(text), keys):
        problems.append(f"{label}: null removal is not structural")
        return text
    return result


def fixtures_with_nulls():
    found = {}
    for kind, keys in NULL_OMITTED.items():
        for path in sorted(glob.glob(os.path.join(TREE, ART, kind, "**", "🔣️.json"), recursive=True)):
            if "🧫️fixtures" not in path:
                continue
            text = open(path, encoding="utf-8").read()
            try:
                document = json.loads(text)
            except ValueError:
                continue
            if document != without_nulls(document, keys):
                found[os.path.relpath(path, TREE)] = keys
    return found


EPW_EDITOR_TESTS = f"{ART}/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
IFC2X3_IO_TESTS = f"{ART}/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🧪️tests/🔬️unit/🦀️.rs"


def epw_editor_tests(text):
    head = "async fn set_cell_reaches_the_document_through_the_registered_native_factory() {\n    use semio_framework_plugin::PluginApp;\n"
    return once(text, head + "    let snapshot = EpwSnapshot { records: vec![crate::standards::energyplus::subsets::any::schema::snapshot::EpwRecord::default()], ..Default::default() };\n", head + "    let snapshot = crate::standards::energyplus::subsets::any::schema::blank_epw_snapshot();\n", "epw set-cell fixture")


def ifc2x3_io_tests(text):
    return once(text, 'vec!["document", "edmPreamble", "schema"]);\n', 'vec!["document", "schema"]);\n', "ifc 2x3 shadow-state keys")


PNG_PATCH = f"{ART}/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs"
JPG_PATCH = f"{ART}/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs"
JPG_EDITOR_TESTS = f"{ART}/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
PNG_EDITOR_TESTS = f"{ART}/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"


def absent_optional_inserts(text, path, count, label):
    """🧫️ An edit that gives an ABSENT optional field (`None`, omitted from the value tree since p11) its value is an insert,
    as RFC 6902 `add` vs `replace`: a `SetValue` on a missing member replaced nothing."""
    old = f'editing::SnapshotEditEvent::SetValue {{ path: "{path}".into(),'
    if text.count(old) != count:
        problems.append(f"{label}: {path} SetValue x{text.count(old)}")
        return text
    return text.replace(old, f'editing::SnapshotEditEvent::InsertValue {{ path: "{path}".into(),')


def jpg_editor_tests(text):
    head = "fn large_raster_quality_edit_uses_compact_native_event() {\n"
    old = '    let event = editing::SnapshotEditEvent::SetValue { path: "/reEncodeQuality".into(), value: dsl::DslValue::Number(dsl::Number::UInt(75)) };\n'
    new = '    let event = editing::SnapshotEditEvent::InsertValue { path: "/reEncodeQuality".into(), value: dsl::DslValue::Number(dsl::Number::UInt(75)) };\n'
    text = once(text, head + "    register_document_schema();\n    let mut snapshot = JpgSnapshot::default();\n    snapshot.pixels = vec![7; 2 * 1_024 * 1_024];\n" + old, head + "    register_document_schema();\n    let mut snapshot = JpgSnapshot::default();\n    snapshot.pixels = vec![7; 2 * 1_024 * 1_024];\n" + new, "jpg editor tests: absent quality insert")
    return once(
        text,
        "    let native_emit = <JpgAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &native_base).expect(\"native quality edit emits\");\n",
        "    let replace = editing::SnapshotEditEvent::SetValue { path: \"/reEncodeQuality\".into(), value: dsl::DslValue::Number(dsl::Number::UInt(75)) };\n"
        "    let native_emit = <JpgAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&replace, &native_base).expect(\"native quality edit emits\");\n",
        "jpg editor tests: present quality replace",
    )


def png_editor_tests(text):
    return once(
        text,
        "    let emit = <PngEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base).expect(\"lossless typed source\");\n",
        "    let emit = <PngEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &crate::schema::demo_png_snapshot()).expect(\"lossless typed source\");\n",
        "png editor tests: typed source replaces the demo",
    )


def png_doc_mention(text):
    return once(text, "mirrors `png`'s own `empty_png_snapshot`/`demo_png_snapshot` placement", "mirrors `png`'s own `blank_png_snapshot`/`demo_png_snapshot` placement", "png doc mention")


DWG_OPTIONAL_HANDLES = ["paper_ucs_name", "paper_ucs_orthographic_reference", "paper_ucs_base", "model_ucs_name", "model_ucs_orthographic_reference", "model_ucs_base", "dimension_leader_block", "dimension_block", "dimension_block1", "dimension_block2", "dimension_linetype", "dimension_extension_linetype1", "dimension_extension_linetype2", "interfere_object_visual_style", "interfere_viewport_visual_style", "drag_visual_style"]


def dwg_handles(text):
    region = re.search(r"pub struct DwgHeaderRelations \{\n(.*?)\n\}\n", text, re.S)
    if not region:
        problems.append("dwg: DwgHeaderRelations")
        return text
    body = region.group(1)
    for field in DWG_OPTIONAL_HANDLES:
        line = f"    pub {field}: Option<u64>,"
        if body.count(line) != 1:
            problems.append(f"dwg: {field} x{body.count(line)}")
            continue
        body = body.replace(line, f"    {SKIP}\n{line}")
    if len(re.findall(r"Option<", body)) != len(DWG_OPTIONAL_HANDLES):
        problems.append("dwg: optional handle roster")
    return text[: region.start(1)] + body + text[region.end(1) :]


def initial_named(text, label, old, new):
    found = re.search(r"    fn initial_snapshot\(\) -> ([A-Za-z:]+) \{\n        " + re.escape(old) + r"\n    \}\n", text)
    if not found:
        problems.append(f"{label}: initial_snapshot anchor")
        return text
    return text[: found.start()] + f"    fn initial_snapshot() -> {found.group(1)} {{\n        {new}\n    }}\n" + text[found.end() :]


def main():
    mode = next((flag for flag in ("--dry-run", "--write", "--revert") if flag in sys.argv), None)
    if mode is None:
        print(__doc__)
        sys.exit(2)
    artifact_defs = set(json.load(open(os.path.join(TREE, DWG_ARTIFACT), encoding="utf-8")).get("$defs", {}))
    edits = {path: (lambda text, path=path: local_defs(text, path.split("/")[4])) for path in LOCAL}
    edits.update({path: (lambda text, path=path: dwg_defs(text, path.split("/")[4], artifact_defs)) for path in DWG})
    edits[DXF] = lambda text: dxf_tables(text, "dxf")
    edits[AVI] = avi_default
    edits[WAV] = wav_default
    edits[PNG] = lambda text: optional_fields(text, ["plte", "trns", "gama", "chrm", "srgb", "phys", "time", "bkgd"], "png")
    edits[XML] = lambda text: optional_fields(text, ["root"], "xml")
    edits[JPG] = lambda text: optional_fields(text, ["re_encode_quality", "jfif_thumbnail", "frame", "restart_interval"], "jpg")
    edits[IFC2X3] = lambda text: optional_fields(text, ["edm_preamble"], "ifc 2x3")
    edits[BINARY] = binary_bytes
    edits[DWG_SNAPSHOT] = dwg_handles
    edits[XML_VALID_SCHEMA] = xml_valid_blank
    edits[EPW_EDITOR_TESTS] = epw_editor_tests
    edits[PNG_PATCH] = lambda text: absent_optional_inserts(text, "/gama", 2, "png patch-snapshot")
    edits[JPG_PATCH] = lambda text: absent_optional_inserts(text, "/reEncodeQuality", 2, "jpg patch-snapshot")
    edits[JPG_EDITOR_TESTS] = jpg_editor_tests
    edits[PNG_EDITOR_TESTS] = png_editor_tests
    edits[IFC2X3_IO_TESTS] = ifc2x3_io_tests
    for path, keys in fixtures_with_nulls().items():
        edits[path] = lambda text, path=path, keys=keys: committed_fixture(text, keys, path.split("/")[4])
    edits[IFC2X3_CONTRACT] = ifc2x3_contract
    for path in PNG_DOC_MENTIONS:
        edits[path] = png_doc_mention
    edits[DWG_ARTIFACT] = dwg_policy
    for path, (kind, artifact, doc, body) in BLANKS.items():
        edits[path] = lambda text, path=path, kind=kind, artifact=artifact, doc=doc, body=body: blank(text, path.split("/")[4], kind, artifact, doc, body)
    edits[EPW_SCHEMA] = blank_epw
    edits[PPTX_SCHEMA] = pptx_blank
    edits[PPTX_ROOT] = lambda text: once(text, "`empty_pptx_snapshot`/`demo_pptx_snapshot`", "`blank_pptx_snapshot`/`demo_pptx_snapshot`", "pptx root doc")
    for name, (old, new, subsets) in INITIAL.items():
        for subset in subsets:
            for role in ("✏️editor", "👁️viewer"):
                path = f"{ART}/{subset}/{role}/🦀️.rs"
                edits[path] = lambda text, path=path, old=old, new=new: initial_named(text, path, old, new)
    if mode == "--revert":
        for path in edits:
            source = os.path.join(BACKUP, path)
            if os.path.isfile(source):
                shutil.copyfile(source, os.path.join(TREE, path))
                print("restored", path)
        return
    staged = {}
    for path, edit in edits.items():
        before = open(os.path.join(TREE, path), encoding="utf-8").read()
        after = edit(before)
        if after == before:
            problems.append(f"{path}: unchanged")
        else:
            if path.endswith(".json"):
                json.loads(after)
            staged[path] = (before, after)
    for problem in problems:
        print("PROBLEM", problem)
    print(f"{len(staged)} files, {len(problems)} problems")
    if mode == "--write" and not problems:
        for path, (before, after) in staged.items():
            backup = os.path.join(BACKUP, path)
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            if not os.path.exists(backup):
                open(backup, "w", encoding="utf-8").write(before)
            open(os.path.join(TREE, path), "w", encoding="utf-8").write(after)
        print("written; backups under", BACKUP)
    sys.exit(1 if problems else 0)


if __name__ == "__main__":
    main()
