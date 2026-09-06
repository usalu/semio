#!/usr/bin/env python3
"""🚪️ W6 generator — writes remodel's 16 typed io leaves (8 formats x 2 directions).

Each leaf is a `Serializer<RemodelingSnapshot>` / `Deserializer<RemodelingSnapshot>` impl on the
framework's `io_mechanism` channel. Re-run after editing this file, then run `rustfmt` over the
emitted paths (the repo formatting gate).
"""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
IO = ROOT / "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io"

EXPORT = IO / "📤️export/🧵️serializers/🗿️artifacts"
IMPORT = IO / "📥️import/🧩️deserializers/🗿️artifacts"

HEAD_SER = """use crate::artifacts::remodeling::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
"""

HEAD_DE = """use crate::artifacts::remodeling::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
"""

LEAVES: list[tuple[Path, str]] = []


def leaf(path: Path, body: str) -> None:
    LEAVES.append((path, body))


# ──────────────────────────────────────────────────────────── json (Exact, text)
leaf(
    EXPORT / "🔣️json/🔖️rfc8259/✳️any/🦀️.rs",
    HEAD_SER
    + """use semio_framework_os_kernel::ToValue;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_pretty;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf writes.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🧵️ `s.remodel.remodeling@1/*` → `s.stdio.json@rfc8259/*`. The scene is a pure record tree, so its
/// `dsl::ToValue` projection is total: every field survives, and the sibling import leaf reverses it
/// exactly — the one `IoFidelity::Exact` binary-free hop this subset owns.
pub struct RemodelingIntoJson;

impl Serializer<RemodelingSnapshot> for RemodelingIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        let value = pack::json::from_dsl_value(&from.to_value());
        Ok(IoOutcome::clean(IoPayload::Text(write_json_pretty(&JsonSnapshot::from_value(value).value))))
    }
}
""",
)

leaf(
    IMPORT / "🔣️json/🔖️rfc8259/✳️any/🦀️.rs",
    HEAD_DE
    + """use crate::artifacts::remodeling::REMODELING_DOCUMENT_SCHEMA;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf reads.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🔣️ Parses rfc8259 text into this subset's snapshot. An absent/empty `schema` is filled with
/// `REMODELING_DOCUMENT_SCHEMA` so a hand-authored json is still accepted.
pub fn from_json_text(text: &str) -> Result<RemodelingSnapshot, IoError> {
    let value = parse_json_text(text).map_err(|error| IoError { message: format!("json→remodeling: parse failed: {error}"), diagnostics: Vec::new() })?;
    let raw: dsl::DslValue = JsonSnapshot::from_value(value).to_serde_value().into();
    let mut snapshot: RemodelingSnapshot = dsl::FromValue::from_value(raw).map_err(|error| IoError { message: format!("json→remodeling: {error}"), diagnostics: Vec::new() })?;
    if snapshot.schema.is_empty() {
        snapshot.schema = REMODELING_DOCUMENT_SCHEMA.to_string();
    }
    Ok(snapshot)
}

/// 🧩️ `s.stdio.json@rfc8259/*` → `s.remodel.remodeling@1/*`, the exact inverse of the export leaf.
pub struct JsonIntoRemodeling;

impl Deserializer<RemodelingSnapshot> for JsonIntoRemodeling {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Text(text) if text.trim_start().starts_with('{') => Confidence::Low,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "json→remodeling: expected a text json payload".to_string(), diagnostics: Vec::new() });
        };
        Ok(IoOutcome::clean(from_json_text(text)?))
    }
}
""",
)

# ───────────────────────────────────────────────────────────── txt (Exact, text)
leaf(
    EXPORT / "🔤️txt/🔖️utf-8/✳️any/🦀️.rs",
    HEAD_SER
    + """/// 🎯️ The foreign dialect this leaf writes.
pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

/// 🧵️ `s.remodel.remodeling@1/*` → `s.stdio.txt@utf-8/*` — the txt rendition of a remodeling scene IS
/// its own `.remodeling` DSL text, the exact bytes `📚️examples/**/🗣️.dsl.semio` carry, so this hop is
/// `IoFidelity::Exact`. (The pre-W6 leaf here was a stray copy-paste of stdio's internal json↔txt
/// bridge returning `Err("not yet implemented")`.)
pub struct RemodelingIntoTxt;

impl Serializer<RemodelingSnapshot> for RemodelingIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(<RemodelingSnapshot as store::ArtifactDsl>::print_dsl(from))))
    }
}
""",
)

leaf(
    IMPORT / "🔤️txt/🔖️utf-8/✳️any/🦀️.rs",
    HEAD_DE
    + """/// 🎯️ The foreign dialect this leaf reads.
pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.txt@utf-8/*` → `s.remodel.remodeling@1/*` — this subset's own DSL reader, the exact
/// inverse of the export leaf's `print_dsl`.
pub struct TxtIntoRemodeling;

impl Deserializer<RemodelingSnapshot> for TxtIntoRemodeling {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Text(text) if text.contains("remodeling") => Confidence::Low,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "txt→remodeling: expected a text payload".to_string(), diagnostics: Vec::new() });
        };
        let snapshot = <RemodelingSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| IoError { message: format!("txt→remodeling: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
""",
)

# ────────────────────────────────────────────────────────────── ply (Lossy, bin)
leaf(
    EXPORT / "🧱️ply/🔖️1.0/✳️any/🦀️.rs",
    HEAD_SER
    + """use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;

/// 🎯️ The foreign dialect this leaf writes.
pub const PLY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId::ANY };

/// 🧵️ `s.remodel.remodeling@1/*` → `s.stdio.ply@1.0/*`. PLY is the one target that carries BOTH a
/// surface and a bare point set, so it exports `results.mesh` when a bounded durable mesh resolves
/// and falls back to `results.dense`/`results.sparse` otherwise. Real codec throughout: stdio's
/// `SemioMeshToPly` serializer + `ply::engine::encode_ply`. `IoFidelity::Lossy` because everything
/// else in the scene — streams, calibration, GCPs, params, job state, QC, geo products — has no PLY
/// representation at all.
pub struct RemodelingIntoPly;

impl Serializer<RemodelingSnapshot> for RemodelingIntoPly {
    const INTO: Dialect = PLY_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        let semio = io_root::scene_mesh_or_cloud_semio(from).map_err(|reason| IoError { message: format!("remodeling→ply: nothing to export: {reason}"), diagnostics: Vec::new() })?;
        let bytes = io_root::semio_mesh_to_ply_bytes(&semio).map_err(|error| IoError { message: format!("remodeling→ply: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
""",
)

leaf(
    IMPORT / "🧱️ply/🔖️1.0/✳️any/🦀️.rs",
    HEAD_DE
    + """use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use semio_framework_plugin::{resolve_ready, ArtifactDeserializer};
use semio_s_plugin_stdio::artifacts::ply::standards::v1_0::engine::decode_ply;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::ply::v1_0::any::SemioMeshFromPly;

/// 🎯️ The foreign dialect this leaf reads.
pub const PLY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.ply@1.0/*` → `s.remodel.remodeling@1/*` — a fresh scene whose `results.sparse` is the
/// decoded point set (plus `results.mesh` when the file also carried faces; dropping them would be
/// silent loss). `results.dense` is never produced: a dense cloud is distinguished in this schema by
/// per-point confidence, which neither PLY's vertex properties as `SemioMeshSnapshot` models them nor
/// any other foreign mesh dialect carries.
pub struct PlyIntoRemodeling;

impl Deserializer<RemodelingSnapshot> for PlyIntoRemodeling {
    const FROM: Dialect = PLY_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Binary(bytes) if bytes.starts_with(b"ply") => Confidence::High,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "ply→remodeling: expected a binary ply payload".to_string(), diagnostics: Vec::new() });
        };
        let ply = decode_ply(bytes).map_err(|error| IoError { message: format!("ply→remodeling: decode failed: {error}"), diagnostics: Vec::new() })?;
        let semio = resolve_ready(SemioMeshFromPly::deserialize(&ply)).map_err(|error| IoError { message: format!("ply→remodeling: {error}"), diagnostics: Vec::new() })?;
        let scene = io_root::scene_from_semio_cloud(&semio).map_err(|reason| IoError { message: format!("ply→remodeling: {reason}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(scene))
    }
}
""",
)

# ────────────────────────────────────────────────────────────── las (Lossy, bin)
leaf(
    EXPORT / "☁️las/🔖️1.0/✳️any/🦀️.rs",
    HEAD_SER
    + """use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;

/// 🎯️ The foreign dialect this leaf writes.
pub const LAS_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId::ANY };

/// 🧵️ `s.remodel.remodeling@1/*` → `s.stdio.las@1.0/*` — `results.dense` when a dense run produced
/// one, else `results.sparse`, through stdio's real `SemioMeshToLas` serializer +
/// `las::engine::encode_las`. LAS is a point format with no topology, so `results.mesh` is NOT a
/// candidate here (that is what the `🧱️ply`/`🗿️obj`/`🔺️stl`/`🧊️gltf` leaves are for) and per-point
/// classification/confidence have no `SemioMeshSnapshot` channel to travel in: `IoFidelity::Lossy`.
pub struct RemodelingIntoLas;

impl Serializer<RemodelingSnapshot> for RemodelingIntoLas {
    const INTO: Dialect = LAS_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        let semio = io_root::scene_cloud_semio(from).map_err(|reason| IoError { message: format!("remodeling→las: nothing to export: {reason}"), diagnostics: Vec::new() })?;
        let bytes = io_root::semio_mesh_to_las_bytes(&semio).map_err(|error| IoError { message: format!("remodeling→las: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
""",
)

leaf(
    IMPORT / "☁️las/🔖️1.0/✳️any/🦀️.rs",
    HEAD_DE
    + """use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use semio_framework_plugin::{resolve_ready, ArtifactDeserializer};
use semio_s_plugin_stdio::artifacts::las::standards::v1_0::engine::decode_las;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::las::v1_0::any::SemioMeshFromLas;

/// 🎯️ The foreign dialect this leaf reads.
pub const LAS_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.las@1.0/*` → `s.remodel.remodeling@1/*` — the decoded points seed `results.sparse`.
/// LAS classification codes are dropped (`SemioMeshSnapshot` has no per-point class channel), which is
/// exactly why this hop is `IoFidelity::Lossy` rather than semantic.
pub struct LasIntoRemodeling;

impl Deserializer<RemodelingSnapshot> for LasIntoRemodeling {
    const FROM: Dialect = LAS_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Binary(bytes) if bytes.starts_with(b"LASF") => Confidence::High,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "las→remodeling: expected a binary las payload".to_string(), diagnostics: Vec::new() });
        };
        let las = decode_las(bytes).map_err(|error| IoError { message: format!("las→remodeling: decode failed: {error}"), diagnostics: Vec::new() })?;
        let semio = resolve_ready(SemioMeshFromLas::deserialize(&las)).map_err(|error| IoError { message: format!("las→remodeling: {error}"), diagnostics: Vec::new() })?;
        let scene = io_root::scene_from_semio_cloud(&semio).map_err(|reason| IoError { message: format!("las→remodeling: {reason}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(scene))
    }
}
""",
)

# ───────────────────────────────────────────────────────────── obj (Lossy, text)
leaf(
    EXPORT / "🗿️obj/🔖️3.0/✳️any/🦀️.rs",
    HEAD_SER
    + """use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use semio_framework_plugin::{resolve_ready, ArtifactSerializer};
use semio_s_plugin_stdio::artifacts::obj::standards::v3_0::engine::encode_obj;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::obj::v3_0::any::SemioMeshToObj;

/// 🎯️ The foreign dialect this leaf writes.
pub const OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId::ANY };

/// 🧵️ `s.remodel.remodeling@1/*` → `s.stdio.obj@3.0/*` — the reconstructed surface `results.mesh`
/// only, through stdio's real `SemioMeshToObj` serializer + `obj::engine::encode_obj`. A scene with
/// only a point cloud gets a typed `Err` naming that, never an empty solid.
pub struct RemodelingIntoObj;

impl Serializer<RemodelingSnapshot> for RemodelingIntoObj {
    const INTO: Dialect = OBJ_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        let semio = io_root::scene_mesh_semio(from).map_err(|reason| IoError { message: format!("remodeling→obj: nothing to export: {reason}"), diagnostics: Vec::new() })?;
        let obj = resolve_ready(SemioMeshToObj::serialize(&semio)).map_err(|error| IoError { message: format!("remodeling→obj: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Text(encode_obj(&obj))))
    }
}
""",
)

leaf(
    IMPORT / "🗿️obj/🔖️3.0/✳️any/🦀️.rs",
    HEAD_DE
    + """use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use semio_framework_plugin::{resolve_ready, ArtifactDeserializer};
use semio_s_plugin_stdio::artifacts::obj::standards::v3_0::engine::decode_obj;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::obj::v3_0::any::SemioMeshFromObj;

/// 🎯️ The foreign dialect this leaf reads.
pub const OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.obj@3.0/*` → `s.remodel.remodeling@1/*` — the decoded surface becomes a real durable
/// mesh asset (`durable_artifacts` chunks + a replayable `results.mesh.mesh` handle, the same shape a
/// committed reconstruction writes), with `MeshSource::Imported`. Materials, groups and object names
/// are dropped: `IoFidelity::Lossy`.
pub struct ObjIntoRemodeling;

impl Deserializer<RemodelingSnapshot> for ObjIntoRemodeling {
    const FROM: Dialect = OBJ_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Text(text) if text.lines().any(|line| line.starts_with("v ") || line.starts_with("f ")) => Confidence::Medium,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "obj→remodeling: expected a text obj payload".to_string(), diagnostics: Vec::new() });
        };
        let obj = decode_obj(text).map_err(|error| IoError { message: format!("obj→remodeling: decode failed: {error}"), diagnostics: Vec::new() })?;
        let semio = resolve_ready(SemioMeshFromObj::deserialize(&obj)).map_err(|error| IoError { message: format!("obj→remodeling: {error}"), diagnostics: Vec::new() })?;
        let scene = io_root::scene_from_semio_mesh(&semio).map_err(|reason| IoError { message: format!("obj→remodeling: {reason}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(scene))
    }
}
""",
)

# ───────────────────────────────────────────────────────────── stl (Lossy, text)
leaf(
    EXPORT / "🔺️stl/🔖️ascii/✳️any/🦀️.rs",
    HEAD_SER
    + """use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use semio_framework_plugin::{resolve_ready, ArtifactSerializer};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::stl::v_ascii::any::SemioMeshToStl;
use semio_s_plugin_stdio::artifacts::stl::standards::v_ascii::engine::encode_stl_ascii;

/// 🎯️ The foreign dialect this leaf writes.
pub const STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId::ANY };

/// 🧵️ `s.remodel.remodeling@1/*` → `s.stdio.stl@ascii/*` — `results.mesh` as an ASCII triangle soup
/// through stdio's real `SemioMeshToStl` serializer + `stl::engine::encode_stl_ascii`. STL carries no
/// vertex sharing, colors, uvs or names, so this is `IoFidelity::Lossy` even for the mesh alone.
pub struct RemodelingIntoStl;

impl Serializer<RemodelingSnapshot> for RemodelingIntoStl {
    const INTO: Dialect = STL_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        let semio = io_root::scene_mesh_semio(from).map_err(|reason| IoError { message: format!("remodeling→stl: nothing to export: {reason}"), diagnostics: Vec::new() })?;
        let stl = resolve_ready(SemioMeshToStl::serialize(&semio)).map_err(|error| IoError { message: format!("remodeling→stl: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Text(encode_stl_ascii(&stl))))
    }
}
""",
)

leaf(
    IMPORT / "🔺️stl/🔖️ascii/✳️any/🦀️.rs",
    HEAD_DE
    + """use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use semio_framework_plugin::{resolve_ready, ArtifactDeserializer};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::stl::v_ascii::any::SemioMeshFromStl;
use semio_s_plugin_stdio::artifacts::stl::standards::v_ascii::engine::decode_stl_ascii;

/// 🎯️ The foreign dialect this leaf reads.
pub const STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.stl@ascii/*` → `s.remodel.remodeling@1/*` — the decoded triangle soup becomes a real
/// durable mesh asset. Vertices stay unshared (STL has no index list to recover): `IoFidelity::Lossy`.
pub struct StlIntoRemodeling;

impl Deserializer<RemodelingSnapshot> for StlIntoRemodeling {
    const FROM: Dialect = STL_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Text(text) if text.trim_start().starts_with("solid") => Confidence::Medium,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "stl→remodeling: expected a text ascii-stl payload".to_string(), diagnostics: Vec::new() });
        };
        let stl = decode_stl_ascii(text).map_err(|error| IoError { message: format!("stl→remodeling: decode failed: {error}"), diagnostics: Vec::new() })?;
        let semio = resolve_ready(SemioMeshFromStl::deserialize(&stl)).map_err(|error| IoError { message: format!("stl→remodeling: {error}"), diagnostics: Vec::new() })?;
        let scene = io_root::scene_from_semio_mesh(&semio).map_err(|reason| IoError { message: format!("stl→remodeling: {reason}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(scene))
    }
}
""",
)

# ───────────────────────────────────────────────────────────── gltf (Lossy, bin)
leaf(
    EXPORT / "🧊️gltf/🔖️2.0/✳️any/🦀️.rs",
    HEAD_SER
    + """use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use semio_framework_plugin::{resolve_ready, ArtifactSerializer};
use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::engine::encode_glb;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::gltf::v2_0::any::SemioMeshToGltf;

/// 🎯️ The foreign dialect this leaf writes.
pub const GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId::ANY };

/// 🧵️ `s.remodel.remodeling@1/*` → `s.stdio.gltf@2.0/*` — `results.mesh` as a self-contained GLB
/// container through stdio's real `SemioMeshToGltf` serializer + `gltf::engine::encode_glb`. The mesh's
/// baked texture (`results.mesh.texture_asset_id`) is NOT embedded: it lives in `scene.assets` as a
/// separate composed image child, and inventing a glTF material/texture pair for it would claim a
/// binding the scene does not record. `IoFidelity::Lossy`.
pub struct RemodelingIntoGltf;

impl Serializer<RemodelingSnapshot> for RemodelingIntoGltf {
    const INTO: Dialect = GLTF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        let semio = io_root::scene_mesh_semio(from).map_err(|reason| IoError { message: format!("remodeling→gltf: nothing to export: {reason}"), diagnostics: Vec::new() })?;
        let gltf = resolve_ready(SemioMeshToGltf::serialize(&semio)).map_err(|error| IoError { message: format!("remodeling→gltf: {error}"), diagnostics: Vec::new() })?;
        let bytes = encode_glb(&gltf).map_err(|error| IoError { message: format!("remodeling→gltf: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
""",
)

leaf(
    IMPORT / "🧊️gltf/🔖️2.0/✳️any/🦀️.rs",
    HEAD_DE
    + """use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;
use semio_framework_plugin::{resolve_ready, ArtifactDeserializer};
use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::engine::decode_glb;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::gltf::v2_0::any::SemioMeshFromGltf;

/// 🎯️ The foreign dialect this leaf reads.
pub const GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.gltf@2.0/*` → `s.remodel.remodeling@1/*` — the first mesh primitive of the decoded GLB
/// becomes a real durable mesh asset. Scene graph, nodes, animations, cameras and materials are
/// dropped (this document has no slot for any of them): `IoFidelity::Lossy`.
pub struct GltfIntoRemodeling;

impl Deserializer<RemodelingSnapshot> for GltfIntoRemodeling {
    const FROM: Dialect = GLTF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Binary(bytes) if bytes.starts_with(b"glTF") => Confidence::High,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "gltf→remodeling: expected a binary glb payload".to_string(), diagnostics: Vec::new() });
        };
        let gltf = decode_glb(bytes).map_err(|error| IoError { message: format!("gltf→remodeling: decode failed: {error}"), diagnostics: Vec::new() })?;
        let semio = resolve_ready(SemioMeshFromGltf::deserialize(&gltf)).map_err(|error| IoError { message: format!("gltf→remodeling: {error}"), diagnostics: Vec::new() })?;
        let scene = io_root::scene_from_semio_mesh(&semio).map_err(|reason| IoError { message: format!("gltf→remodeling: {reason}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(scene))
    }
}
""",
)

# ────────────────────────────────────────────────────────────── png (Lossy, bin)
leaf(
    EXPORT / "📷️png/🔖️1.2/✳️any/🦀️.rs",
    HEAD_SER
    + """use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;

/// 🎯️ The foreign dialect this leaf writes.
pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

/// 🧵️ `s.remodel.remodeling@1/*` → `s.stdio.png@1.2/*` — the reconstruction's raster preview: the DSM,
/// else the orthophoto, else the DTM, else the mesh's baked texture, read back as real composed
/// `s.stdio.semio/v1/image` content through `io_root::remodeling_png_asset`. A scene with no raster
/// and no texture returns a typed `Err` rather than a blank canvas. `IoFidelity::Lossy`: one raster is
/// not the scene.
pub struct RemodelingIntoPng;

impl Serializer<RemodelingSnapshot> for RemodelingIntoPng {
    const INTO: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        let asset = io_root::remodeling_png_asset(from).map_err(|reason| IoError { message: format!("remodeling→png: {reason}"), diagnostics: Vec::new() })?;
        if asset.mime != "image/png" {
            return Err(IoError { message: format!("remodeling→png: the selected asset is {:?}, not image/png", asset.mime), diagnostics: Vec::new() });
        }
        let bytes = base64_codec::base64_standard_decode(asset.data.as_bytes()).map_err(|error| IoError { message: format!("remodeling→png: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
""",
)

leaf(
    IMPORT / "📷️png/🔖️1.2/✳️any/🦀️.rs",
    HEAD_DE
    + """use crate::artifacts::remodeling::standards::v1::subsets::any::io as io_root;

/// 🎯️ The foreign dialect this leaf reads.
pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.png@1.2/*` → `s.remodel.remodeling@1/*` — one PNG becomes a one-frame
/// `MediaKind::ImageSequence` stream over a real durable image asset, the same document shape
/// `📥️import-frames` builds for a photo set, so a single dropped photo is a legal (if degenerate)
/// reconstruction input rather than an unusable blob. `IoFidelity::Lossy`: nothing else in the scene
/// exists yet.
pub struct PngIntoRemodeling;

impl Deserializer<RemodelingSnapshot> for PngIntoRemodeling {
    const FROM: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Binary(bytes) if bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]) => Confidence::High,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "png→remodeling: expected a binary png payload".to_string(), diagnostics: Vec::new() });
        };
        let scene = io_root::scene_from_png_bytes(bytes).map_err(|reason| IoError { message: format!("png→remodeling: {reason}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(scene))
    }
}
""",
)


def main() -> None:
    for path, body in LEAVES:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(body, encoding="utf-8")
        print(f"wrote {path.relative_to(ROOT)}")
    print(f"{len(LEAVES)} leaves")


if __name__ == "__main__":
    main()
