#!/usr/bin/env python3
"""🚪️ W3 — generates the 36 `🚪️io` leaf files and the 3 `🚪️io/🦀️.rs` roots of `✏️s/🔌️plugins/🧱️block`.

One generator instead of 39 hand-copies: the three subsets carry structurally identical io, and
CLAUDE.md's "if code is repeated, it MUST be close to each other" is satisfied by the shared
template here. Re-runnable — every file is written in full, never patched.
"""
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[7]
BLOCK = ROOT / "✏️s/🔌️plugins/🧱️block/🗿️artifacts"

SUBSETS = [
    dict(dir="◻️2d", dim="2d", p="Block2d", mod="block2d", schema="BLOCK_2D_SCHEMA",
         dialect="BLOCK2D_DIALECT", kind="s.block.block2d", ext="block2d",
         envelope="block.block2d", art="a", noun="node kind", anchors="`handles`"),
    dict(dir="🧊️3d", dim="3d", p="Block3d", mod="block3d", schema="BLOCK_3D_SCHEMA",
         dialect="BLOCK3D_DIALECT", kind="s.block.block3d", ext="block3d",
         envelope="block.block3d", art="an", noun="object kind", anchors="`vortices`"),
    dict(dir="🖐️5d", dim="5d", p="Block5d", mod="block5d", schema="BLOCK_5D_SCHEMA",
         dialect="BLOCK5D_DIALECT", kind="s.block.block5d", ext="block5d",
         envelope="block.block5d", art="a", noun="part kind", anchors="`grips`"),
]

IMPORT_DIR = "🚪️io/📥️import/🧩️deserializers/🗿️artifacts"
EXPORT_DIR = "🚪️io/📤️export/🧵️serializers/🗿️artifacts"
LEAF = {
    "json": "🔣️json/🔖️rfc8259/✳️any",
    "txt": "🔤️txt/🔖️utf-8/✳️any",
    "zip": "🎒️zip/🔖️2.0/✳️any",
    "stl": "🔺️stl/🔖️ascii/✳️any",
    "obj": "🧊️obj/🔖️3.0/✳️any",
    "png": "📷️png/🔖️1.2/✳️any",
}

# 🚫️ The three formats the block schema genuinely cannot carry, with the reason each leaf documents.
UNSUPPORTED = {
    "stl": dict(
        Fmt="Stl", DIALECT='Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId::ANY }',
        const="STL_DIALECT", label="stl",
        export_reason="the schema carries no triangle geometry — `representations[].mesh_url` is a URL pointing at an external mesh asset, never vertex data",
        import_reason="an STL solid carries triangles only — it has no kind identity, no handle/vortex/grip catalog and no compatibility rules to build a kind definition from",
        why="Its only geometry-bearing field is `representations[].mesh_url` — a URL pointing at an external mesh asset, never vertex/triangle data — and {anchors} are anchor frames (angle/radius/position/direction), not a surface. Nothing in the schema an STL triangle soup could be built from, and nothing in an STL an identity/catalog/compatibility document could be built from."),
    "obj": dict(
        Fmt="Obj", DIALECT='Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId::ANY }',
        const="OBJ_DIALECT", label="obj",
        export_reason="the schema carries no vertex/face geometry — `representations[].mesh_url` is a URL pointing at an external mesh asset, never mesh data",
        import_reason="an OBJ mesh carries vertices and faces only — it has no kind identity, no handle/vortex/grip catalog and no compatibility rules to build a kind definition from",
        why="Identical to the sibling `🔺️stl` leaf's reasoning: `representations[].mesh_url` is a reference, {anchors} are anchor frames, and an OBJ mesh carries no identity/catalog/compatibility data."),
    "png": dict(
        Fmt="Png", DIALECT='Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY }',
        const="PNG_DIALECT", label="png",
        export_reason="the schema carries no raster data and this plugin ships no rasterizer — emitting a blank or placeholder canvas would silently claim an export that did not happen",
        import_reason="a PNG carries pixels only — it has no kind identity, no handle/vortex/grip catalog and no compatibility rules to build a kind definition from",
        why="A block document has no pixel field at all, and this plugin ships no rasterizer (the `👁️viewer` renders through the framework's window kits, not into a buffer this leaf can reach). Painting a blank canvas — the shape `🗒️note`'s own png leaf settled for — would silently claim an export that did not happen."),
}

HEADER_UNSUPPORTED = """//! 🚪️ block{dim} {arrow} {label} — foreign `{trait}<{p}Snapshot>` that HONESTLY REFUSES.
//!
//! A `{kind}` document is {art} {noun} DEFINITION, not geometry or a raster. {why}
//!
//! So this leaf returns a typed `IoError` naming the reason instead of an empty snapshot or an
//! invented solid. It stays REGISTERED on the `io_mechanism` channel at the weakest fidelity
//! (`IoFidelity::Lossy`, rank 0 — the router never prefers it over a real hop) so a caller that does
//! route here gets this reason back rather than a bare "no route" (see `📓️w3-io.md`).
"""

EXPORT_UNSUPPORTED = HEADER_UNSUPPORTED + """
use crate::artifacts::{mod}::{p}Snapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{{Dialect, IoError, IoFidelity, IoPayload, IoResult}};
use semio_framework_plugin::{{StandardId, SubsetId}};

/// 🎯️ The foreign dialect this leaf would write.
pub const {const}: Dialect = {DIALECT};

/// 🧵️ `{kind}@1/*` → `{fkind}` — always `Err`, see this file's module doc.
pub struct {p}Into{Fmt};

impl Serializer<{p}Snapshot> for {p}Into{Fmt} {{
    const INTO: Dialect = {const};
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(_from: &{p}Snapshot) -> IoResult<IoPayload> {{
        Err(IoError {{ message: "{label} export not supported for {art} {noun} definition: {export_reason}".to_string(), diagnostics: Vec::new() }})
    }}
}}
"""

IMPORT_UNSUPPORTED = HEADER_UNSUPPORTED + """
use crate::artifacts::{mod}::{p}Snapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{{Dialect, IoError, IoFidelity, IoPayload, IoResult}};
use semio_framework_plugin::{{StandardId, SubsetId}};

/// 🎯️ The foreign dialect this leaf would read.
pub const {const}: Dialect = {DIALECT};

/// 🧩️ `{fkind}` → `{kind}@1/*` — always `Err`, see this file's module doc.
pub struct {Fmt}Into{p};

impl Deserializer<{p}Snapshot> for {Fmt}Into{p} {{
    const FROM: Dialect = {const};
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(_payload: &IoPayload) -> IoResult<{p}Snapshot> {{
        Err(IoError {{ message: "{label} import not supported for {art} {noun} definition: {import_reason}".to_string(), diagnostics: Vec::new() }})
    }}
}}
"""

EXPORT_JSON = """//! 🚪️ block{dim} → json — foreign `Serializer<{p}Snapshot>` on the framework's `io_mechanism`
//! channel. The snapshot is a pure `dsl::ToValue` record tree, so its rfc8259 rendition carries every
//! field and the sibling `📥️import` leaf reconstructs the snapshot exactly: `IoFidelity::Exact`.

use crate::artifacts::{mod}::{p}Snapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult}};
use semio_framework_plugin::{{StandardId, SubsetId}};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_text;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf writes.
pub const JSON_DIALECT: Dialect = Dialect {{ artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY }};

/// 🔣️ This subset's snapshot as compact rfc8259 text — also the body the `🎒️zip` container leaf
/// embeds, and the exact bytes the TypeScript mirror's parity test compares against.
pub fn json_text(from: &{p}Snapshot) -> String {{
    write_json_text(&JsonSnapshot::from_value(dsl::json::from_dsl_value(&dsl::ToValue::to_value(from))).value)
}}

/// 🧵️ `{kind}@1/*` → `s.stdio.json@rfc8259/*`.
pub struct {p}IntoJson;

impl Serializer<{p}Snapshot> for {p}IntoJson {{
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &{p}Snapshot) -> IoResult<IoPayload> {{
        Ok(IoOutcome::clean(IoPayload::Text(json_text(from))))
    }}
}}
"""

IMPORT_JSON = """//! 🚪️ block{dim} ← json — foreign `Deserializer<{p}Snapshot>` on the framework's `io_mechanism`
//! channel, the exact inverse of the sibling `📤️export` leaf: `IoFidelity::Exact`.

use crate::artifacts::{mod}::{{{p}Snapshot, {schema}}};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult}};
use semio_framework_plugin::{{StandardId, SubsetId}};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf reads.
pub const JSON_DIALECT: Dialect = Dialect {{ artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY }};

/// 🔣️ Parses rfc8259 text into this subset's snapshot — also used by the `🎒️zip` container leaf.
/// An absent/empty `schema` is filled with `{schema}` so a hand-authored json is still accepted.
pub fn from_json_text(text: &str) -> Result<{p}Snapshot, IoError> {{
    let value = parse_json_text(text).map_err(|error| IoError {{ message: format!("json→block{dim}: parse failed: {{error}}"), diagnostics: Vec::new() }})?;
    let raw: dsl::DslValue = JsonSnapshot::from_value(value).to_serde_value().into();
    let mut snapshot: {p}Snapshot = dsl::FromValue::from_value(raw).map_err(|error| IoError {{ message: format!("json→block{dim}: {{error}}"), diagnostics: Vec::new() }})?;
    if snapshot.schema.is_empty() {{
        snapshot.schema = {schema}.to_string();
    }}
    Ok(snapshot)
}}

/// 🧩️ `s.stdio.json@rfc8259/*` → `{kind}@1/*`.
pub struct JsonInto{p};

impl Deserializer<{p}Snapshot> for JsonInto{p} {{
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn sniff(payload: &IoPayload) -> Confidence {{
        match payload {{
            IoPayload::Text(text) if text.trim_start().starts_with('{{') => Confidence::Low,
            _ => Confidence::None,
        }}
    }}
    async fn deserialize(payload: &IoPayload) -> IoResult<{p}Snapshot> {{
        let IoPayload::Text(text) = payload else {{
            return Err(IoError {{ message: "json→block{dim}: expected a text json payload".to_string(), diagnostics: Vec::new() }});
        }};
        Ok(IoOutcome::clean(from_json_text(text)?))
    }}
}}
"""

EXPORT_TXT = """//! 🚪️ block{dim} → txt — foreign `Serializer<{p}Snapshot>` on the framework's `io_mechanism`
//! channel. `s.stdio.txt@utf-8` for this subset IS its own `.semio` DSL snapshot text
//! (`🧬️schema/📸️snapshot/📝️text`): the very bytes the `📚️examples/**/🖼️assets/**/🗣️.dsl.semio`
//! fixtures carry and `<{p}Snapshot as store::ArtifactDsl>::parse_dsl` reads back, so the hop is
//! `IoFidelity::Exact` and the sibling `📥️import` leaf is its exact inverse.
//!
//! 🐛️ Repaired here (ticket 26/09/05/BLOCK-PLUGIN-END-TO-END, W3): this file used to be an
//! `Err("txt export not yet implemented")` stub that ALSO carried a stray `deserialize_bytes` — an
//! import-direction function inside the export tree, left behind by a copy-paste of stdio's own
//! json↔txt bridge. Both are gone.

use crate::artifacts::{mod}::{p}Snapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult}};
use semio_framework_plugin::{{StandardId, SubsetId}};

/// 🎯️ The foreign dialect this leaf writes.
pub const TXT_DIALECT: Dialect = Dialect {{ artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY }};

/// 🔤️ This subset's snapshot as `.semio` DSL text — also the authoritative member the `🎒️zip`
/// container leaf packs.
pub fn dsl_text(from: &{p}Snapshot) -> String {{
    <{p}Snapshot as store::ArtifactDsl>::print_dsl(from)
}}

/// 🧵️ `{kind}@1/*` → `s.stdio.txt@utf-8/*`.
pub struct {p}IntoTxt;

impl Serializer<{p}Snapshot> for {p}IntoTxt {{
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &{p}Snapshot) -> IoResult<IoPayload> {{
        Ok(IoOutcome::clean(IoPayload::Text(dsl_text(from))))
    }}
}}
"""

IMPORT_TXT = """//! 🚪️ block{dim} ← txt — foreign `Deserializer<{p}Snapshot>` on the framework's `io_mechanism`
//! channel: `store::ArtifactDsl::parse_dsl` on this subset's own `.semio` DSL snapshot text, the
//! exact inverse of the sibling `📤️export` leaf (`IoFidelity::Exact`).
//!
//! 🐛️ Repaired here (ticket 26/09/05/BLOCK-PLUGIN-END-TO-END, W3): this file used to be an
//! `Err("txt import not yet implemented")` stub left by a copy-paste of stdio's own json↔txt bridge.

use crate::artifacts::{mod}::{p}Snapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult}};
use semio_framework_plugin::{{StandardId, SubsetId}};

/// 🎯️ The foreign dialect this leaf reads.
pub const TXT_DIALECT: Dialect = Dialect {{ artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY }};

/// 📄️ The `.semio` text preamble every `{envelope}` document opens with — the sniff anchor.
pub const DSL_PREAMBLE: &str = "semio {envelope}.dsl ";

/// 🔤️ Parses `.semio` DSL text into this subset's snapshot — also used by the `🎒️zip` leaf.
pub fn from_dsl_text(text: &str) -> Result<{p}Snapshot, IoError> {{
    <{p}Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| IoError {{ message: format!("txt→block{dim}: {{error}}"), diagnostics: Vec::new() }})
}}

/// 🧩️ `s.stdio.txt@utf-8/*` → `{kind}@1/*`.
pub struct TxtInto{p};

impl Deserializer<{p}Snapshot> for TxtInto{p} {{
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn sniff(payload: &IoPayload) -> Confidence {{
        match payload {{
            IoPayload::Text(text) if text.starts_with(DSL_PREAMBLE) => Confidence::High,
            _ => Confidence::None,
        }}
    }}
    async fn deserialize(payload: &IoPayload) -> IoResult<{p}Snapshot> {{
        let IoPayload::Text(text) = payload else {{
            return Err(IoError {{ message: "txt→block{dim}: expected a text utf-8 payload".to_string(), diagnostics: Vec::new() }});
        }};
        Ok(IoOutcome::clean(from_dsl_text(text)?))
    }}
}}
"""

EXPORT_ZIP = """//! 🚪️ block{dim} → zip — foreign `Serializer<{p}Snapshot>` on the framework's `io_mechanism`
//! channel: a REAL zip 2.0 container (stdio's own `encode_zip`, not a renamed text blob) holding two
//! members — `{dsl_entry}` with this subset's authoritative `.semio` DSL snapshot text, and
//! `{json_entry}` with its rfc8259 rendition for readers that cannot parse `.semio`. Both members are
//! lossless and the sibling `📥️import` leaf reads the DSL member back first, so the hop is
//! `IoFidelity::Exact`.
//!
//! 🐛️ Repaired here (ticket 26/09/05/BLOCK-PLUGIN-END-TO-END, W3): this file used to hand back
//! `print_dsl(...).into_bytes()` — plain DSL text mislabelled as a zip archive, which no zip reader
//! could open.

use crate::artifacts::{mod}::io::export::serializers::artifacts::json::v_rfc8259::any::json_text;
use crate::artifacts::{mod}::io::export::serializers::artifacts::txt::v_utf_8::any::dsl_text;
use crate::artifacts::{mod}::{p}Snapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult}};
use semio_framework_plugin::{{StandardId, SubsetId}};
use semio_s_plugin_stdio::artifacts::zip::io::encode_zip;
use semio_s_plugin_stdio::artifacts::zip::schema::snapshot::ZipEntry;
use semio_s_plugin_stdio::artifacts::zip::ZipSnapshot;

/// 🎯️ The foreign dialect this leaf writes.
pub const ZIP_DIALECT: Dialect = Dialect {{ artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId::ANY }};

/// 📄️ The container member carrying the authoritative `.semio` DSL snapshot text.
pub const ZIP_DSL_ENTRY: &str = "{dsl_entry}";
/// 📄️ The container member carrying the rfc8259 rendition.
pub const ZIP_JSON_ENTRY: &str = "{json_entry}";

/// 🎒️ Builds the container this leaf writes — shared with the sibling `📥️import` leaf's tests.
pub fn archive_of(from: &{p}Snapshot) -> ZipSnapshot {{
    ZipSnapshot {{
        entries: vec![
            ZipEntry {{ name: ZIP_DSL_ENTRY.to_string(), data: dsl_text(from).into_bytes() }},
            ZipEntry {{ name: ZIP_JSON_ENTRY.to_string(), data: json_text(from).into_bytes() }},
        ],
        ..ZipSnapshot::default()
    }}
}}

/// 🧵️ `{kind}@1/*` → `s.stdio.zip@2.0/*`.
pub struct {p}IntoZip;

impl Serializer<{p}Snapshot> for {p}IntoZip {{
    const INTO: Dialect = ZIP_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &{p}Snapshot) -> IoResult<IoPayload> {{
        let bytes = encode_zip(&archive_of(from)).map_err(|error| IoError {{ message: format!("block{dim}→zip: {{error}}"), diagnostics: Vec::new() }})?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }}
}}
"""

IMPORT_ZIP = """//! 🚪️ block{dim} ← zip — foreign `Deserializer<{p}Snapshot>` on the framework's `io_mechanism`
//! channel: decodes a REAL zip 2.0 container (stdio's own `decode_zip`) and rebuilds the snapshot
//! from its `{dsl_entry}` member, falling back to `{json_entry}`. Exact inverse of the sibling
//! `📤️export` leaf, so `IoFidelity::Exact`.
//!
//! 🐛️ Repaired here (ticket 26/09/05/BLOCK-PLUGIN-END-TO-END, W3): this file used to ignore its
//! `bytes` argument entirely and return `Ok({p}Snapshot::default())` — silent, total data loss on
//! every import.

use crate::artifacts::{mod}::io::export::serializers::artifacts::zip::v2_0::any::{{ZIP_DSL_ENTRY, ZIP_JSON_ENTRY}};
use crate::artifacts::{mod}::io::import::deserializers::artifacts::json::v_rfc8259::any::from_json_text;
use crate::artifacts::{mod}::io::import::deserializers::artifacts::txt::v_utf_8::any::from_dsl_text;
use crate::artifacts::{mod}::{p}Snapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult}};
use semio_framework_plugin::{{StandardId, SubsetId}};
use semio_s_plugin_stdio::artifacts::zip::io::decode_zip;

/// 🎯️ The foreign dialect this leaf reads.
pub const ZIP_DIALECT: Dialect = Dialect {{ artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId::ANY }};

/// 🎒️ Local-file-header magic every zip 2.0 archive opens with (APPNOTE §4.3.7).
pub const ZIP_MAGIC: &[u8] = b"PK\\x03\\x04";

/// 🎒️ Rebuilds this subset's snapshot from real zip 2.0 container bytes.
pub fn from_zip_bytes(bytes: &[u8]) -> Result<{p}Snapshot, IoError> {{
    let archive = decode_zip(bytes).map_err(|error| IoError {{ message: format!("zip→block{dim}: {{error}}"), diagnostics: Vec::new() }})?;
    for (name, parse) in [(ZIP_DSL_ENTRY, from_dsl_text as fn(&str) -> Result<{p}Snapshot, IoError>), (ZIP_JSON_ENTRY, from_json_text as fn(&str) -> Result<{p}Snapshot, IoError>)] {{
        let Some(entry) = archive.entries.iter().find(|entry| entry.name == name) else {{
            continue;
        }};
        let text = std::str::from_utf8(&entry.data).map_err(|error| IoError {{ message: format!("zip→block{dim}: `{{name}}` is not utf-8: {{error}}"), diagnostics: Vec::new() }})?;
        return parse(text);
    }}
    Err(IoError {{ message: format!("zip→block{dim}: archive carries neither `{{ZIP_DSL_ENTRY}}` nor `{{ZIP_JSON_ENTRY}}`"), diagnostics: Vec::new() }})
}}

/// 🧩️ `s.stdio.zip@2.0/*` → `{kind}@1/*`.
pub struct ZipInto{p};

impl Deserializer<{p}Snapshot> for ZipInto{p} {{
    const FROM: Dialect = ZIP_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn sniff(payload: &IoPayload) -> Confidence {{
        match payload {{
            IoPayload::Binary(bytes) if bytes.starts_with(ZIP_MAGIC) => Confidence::Low,
            _ => Confidence::None,
        }}
    }}
    async fn deserialize(payload: &IoPayload) -> IoResult<{p}Snapshot> {{
        let IoPayload::Binary(bytes) = payload else {{
            return Err(IoError {{ message: "zip→block{dim}: expected a binary zip payload".to_string(), diagnostics: Vec::new() }});
        }};
        Ok(IoOutcome::clean(from_zip_bytes(bytes)?))
    }}
}}
"""


def render(template: str, s: dict, **extra) -> str:
    return template.format(
        dim=s["dim"], p=s["p"], mod=s["mod"], schema=s["schema"], kind=s["kind"],
        ext=s["ext"], envelope=s["envelope"], art=s["art"], noun=s["noun"], anchors=s["anchors"],
        dsl_entry=f"snapshot.{s['ext']}.semio", json_entry="snapshot.json", **extra)


def write(path: pathlib.Path, body: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(body, encoding="utf-8")
    print(f"wrote {path.relative_to(ROOT)}")


def main() -> None:
    for s in SUBSETS:
        base = BLOCK / s["dir"] / "🏅️standards/🔖️1/🪆️subsets/✳️any"
        write(base / EXPORT_DIR / LEAF["json"] / "🦀️.rs", render(EXPORT_JSON, s))
        write(base / IMPORT_DIR / LEAF["json"] / "🦀️.rs", render(IMPORT_JSON, s))
        write(base / EXPORT_DIR / LEAF["txt"] / "🦀️.rs", render(EXPORT_TXT, s))
        write(base / IMPORT_DIR / LEAF["txt"] / "🦀️.rs", render(IMPORT_TXT, s))
        write(base / EXPORT_DIR / LEAF["zip"] / "🦀️.rs", render(EXPORT_ZIP, s))
        write(base / IMPORT_DIR / LEAF["zip"] / "🦀️.rs", render(IMPORT_ZIP, s))
        for fmt, u in UNSUPPORTED.items():
            fkind = {"stl": "s.stdio.stl@ascii/*", "obj": "s.stdio.obj@3.0/*", "png": "s.stdio.png@1.2/*"}[fmt]
            common = dict(Fmt=u["Fmt"], DIALECT=u["DIALECT"], const=u["const"], label=u["label"],
                          export_reason=u["export_reason"], import_reason=u["import_reason"],
                          why=u["why"].format(anchors=s["anchors"]), fkind=fkind)
            write(base / EXPORT_DIR / LEAF[fmt] / "🦀️.rs", render(EXPORT_UNSUPPORTED, s, arrow="→", trait="Serializer", **common))
            write(base / IMPORT_DIR / LEAF[fmt] / "🦀️.rs", render(IMPORT_UNSUPPORTED, s, arrow="←", trait="Deserializer", **common))


if __name__ == "__main__":
    main()
