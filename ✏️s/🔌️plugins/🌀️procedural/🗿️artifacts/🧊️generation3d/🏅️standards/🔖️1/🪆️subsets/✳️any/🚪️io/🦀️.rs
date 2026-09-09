//! 🚪️ IO s.generation3d (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl", "stdio.txt"]
}
// 🖼️ No "stdio.png"/"stdio.json" here (export-only list): generation2d owns those EXPORT claims, see
// `🚪️IoRegistry` region below. Import is unaffected — both stay in `import_stdio_kinds` above.
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.gltf", "stdio.las", "stdio.obj", "stdio.ply", "stdio.stl", "stdio.txt"]
}
//#region 🔺️MeshBridge
/// 🔺️ The one place this artifact's IO leaves turn a generation3d document into geometry and back.
///
/// A `s.procedural.generation3d` document is a FLOW GRAPH, not a mesh: its only geometry is what the
/// graph EVALUATES to. Export therefore runs the same preview pipeline the editor's own 3D window
/// runs (`crate::editor::generation3d::export_mesh_from_document`, which merges every previewed
/// widget's tessellated output into one [`semio_framework_plugin::MeshData`]) and hands the result
/// to `s.stdio.semio@v1/mesh`'s own tested per-format bridges
/// (`SemioMeshToStl`/`ToObj`/`ToPly`/`ToGltf`/`ToLas`/`ToDwg`) — never a second, hand-rolled copy of
/// any file grammar. Import is the mirror: the incoming bytes are decoded by the owning
/// `s.stdio.<format>` codec, normalized to `s.stdio.stl@ascii` where the flow evaluator has no
/// native operator for the source format, and planted in a three-widget fixture whose
/// `brep.io.import*` neuron re-evaluates them into real previewable BRep geometry.
///
/// @see ../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🦀️.rs
/// @see ../../../../../../../🌊️flow/🧩️extensions/📐️brep/🦀️.rs — `brep.io.importStl`/`importObj`/`importDwg`.
pub mod mesh_bridge {
    use crate::Generation3dSnapshot;
    use semio_framework_artifact_flow_flow::neural::Dictionary;
    use semio_framework_artifact_flow_flow::{CameraJson, FlowFixture, OrderedMap, OrderedSet, SynapseSpec, Widget, WidgetLayout};
    use semio_framework_plugin::MeshData;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint3;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology, STDIO_SEMIOMESH_DOCUMENT_SCHEMA};

    /// 🏷️ Widget ids the import fixture plants, stable so a re-import overwrites rather than stacks.
    pub const IMPORT_SOURCE_WIDGET: &str = "imported-source";
    pub const IMPORT_GEOMETRY_WIDGET: &str = "imported-geometry";
    pub const IMPORT_PREVIEW_WIDGET: &str = "imported-preview";
    /// 🏷️ `SemioMesh` id every export carries — the merged preview, not a per-widget mesh.
    pub const EXPORT_MESH_ID: &str = "generation3d-preview";

    /// 🚨️ Every failure in this artifact's IO is a typed error carrying WHY — an empty document is
    /// never a legal answer to bytes that did not decode.
    pub fn io_error(message: impl Into<String>) -> store::TextError {
        store::TextError::new(message.into(), dsl::TextSpan::at(1, 1))
    }

    /// 🔤️ Standard base64 (RFC 4648 §4, `=`-padded) — the exact spelling the brep extension's own
    /// `decode_base64` accepts on a `brep.io.import*` node's `data` channel. Reused from the gltf
    /// artifact's data-uri codec rather than re-derived here.
    pub fn base64_encode(bytes: &[u8]) -> String {
        semio_s_artifact_stdio_gltf::engine::b64_encode(bytes)
    }

    /// 🔤️ Inverse of [`base64_encode`], for reading an import fixture's planted payload back.
    pub fn base64_decode(text: &str) -> Result<Vec<u8>, store::TextError> {
        semio_s_artifact_stdio_gltf::engine::b64_decode(text).map_err(io_error)
    }

    /// 🔺️ The renderer's flat `MeshData` as this repo's own typed mesh document.
    ///
    /// `MeshData` is the tessellation wire form: flat `f32` triples plus a `u32` index buffer. A
    /// mesh WITHOUT indices carries no face connectivity at all (a wire/point preview), so it
    /// becomes a `Points` primitive rather than being silently reinterpreted as a triangle soup —
    /// the format bridges that cannot represent points then refuse it by name instead of writing
    /// invented triangles.
    pub fn semio_mesh_from_mesh_data(mesh: &MeshData) -> Result<SemioMeshSnapshot, store::TextError> {
        if mesh.positions.len() < 3 {
            return Err(io_error("generation3d export: the document evaluates to no preview geometry (no positions)"));
        }
        if mesh.positions.len() % 3 != 0 {
            return Err(io_error(format!("generation3d export: preview mesh has {} position floats, not a multiple of 3", mesh.positions.len())));
        }
        let positions: Vec<SemioPoint3> = mesh.positions.chunks_exact(3).map(|p| SemioPoint3 { x: p[0] as f64, y: p[1] as f64, z: p[2] as f64 }).collect();
        let normals: Vec<SemioPoint3> =
            if mesh.normals.len() == mesh.positions.len() { mesh.normals.chunks_exact(3).map(|n| SemioPoint3 { x: n[0] as f64, y: n[1] as f64, z: n[2] as f64 }).collect() } else { Vec::new() };
        let topology = if mesh.indices.is_empty() { SemioTopology::Points } else { SemioTopology::Triangles };
        if topology == SemioTopology::Triangles && mesh.indices.len() % 3 != 0 {
            return Err(io_error(format!("generation3d export: preview mesh has {} indices, not a multiple of 3", mesh.indices.len())));
        }
        if let Some(out_of_range) = mesh.indices.iter().find(|index| **index as usize >= positions.len()) {
            return Err(io_error(format!("generation3d export: preview mesh index {out_of_range} is out of range for {} vertices", positions.len())));
        }
        let primitive = SemioPrimitive { id: format!("{EXPORT_MESH_ID}-prim-0"), topology, positions, normals, uvs: Vec::new(), colors: Vec::new(), indices: mesh.indices.clone(), material_id: None };
        Ok(SemioMeshSnapshot { schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(), meshes: vec![SemioMesh { id: EXPORT_MESH_ID.into(), primitives: vec![primitive] }], materials: Vec::new(), textures: Vec::new() })
    }

    /// 👁️ Evaluates the document's flow graph and returns its merged preview mesh.
    #[cfg(feature = "component-app-assembly")]
    pub fn preview_semio_mesh(snapshot: &Generation3dSnapshot) -> Result<SemioMeshSnapshot, store::TextError> {
        semio_mesh_from_mesh_data(&crate::editor::generation3d::export_mesh_from_document(snapshot))
    }

    /// 👁️ Without the flow evaluator linked there is no geometry to export, and saying so is the
    /// only honest answer — a geometry-free file of the requested format would look like success.
    #[cfg(not(feature = "component-app-assembly"))]
    pub fn preview_semio_mesh(_snapshot: &Generation3dSnapshot) -> Result<SemioMeshSnapshot, store::TextError> {
        Err(io_error("generation3d export: geometry export needs the flow evaluator (build this crate with the `component-app-assembly` feature)"))
    }

    /// 🔺️ Re-encodes any decoded mesh as ASCII STL — the normalization step for the formats whose
    /// geometry the flow evaluator can only re-enter through `brep.io.importStl`.
    pub fn stl_ascii_bytes(mesh: &SemioMeshSnapshot) -> Result<Vec<u8>, store::TextError> {
        let stl = semio_framework_plugin::resolve_ready(<semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::stl::v_ascii::any::SemioMeshToStl as semio_framework_plugin::ArtifactSerializer>::serialize(mesh))
            .map_err(|error| io_error(error.to_string()))?;
        Ok(semio_s_artifact_stdio_stl::engine::encode_stl_ascii(&stl).into_bytes())
    }

    /// 📥️ The three-widget import fixture: a note holding the payload, a `brep.io.import*` neuron
    /// that turns it into real BRep geometry, and a preview sink. `preview: true` on the neuron is
    /// what puts the imported mesh in the 3D window (`widget_previews`).
    pub fn import_document(neuron_kind: &str, data_text: String) -> Generation3dSnapshot {
        let mut layout = OrderedMap::new();
        layout.insert(IMPORT_SOURCE_WIDGET.to_string(), WidgetLayout { x: 0.0, y: 0.0 });
        layout.insert(IMPORT_GEOMETRY_WIDGET.to_string(), WidgetLayout { x: 260.0, y: 0.0 });
        layout.insert(IMPORT_PREVIEW_WIDGET.to_string(), WidgetLayout { x: 520.0, y: 0.0 });
        let fixture = FlowFixture {
            schema: "flow.fixture".into(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            widgets: vec![
                Widget::InputNote { id: IMPORT_SOURCE_WIDGET.into(), text: data_text },
                Widget::Neuron { id: IMPORT_GEOMETRY_WIDGET.into(), neuron_kind: neuron_kind.into(), params: Dictionary::new(), input_ports: Vec::new(), output_ports: Vec::new(), preview: true },
                Widget::OutputPreview { id: IMPORT_PREVIEW_WIDGET.into(), preview: Dictionary::new(), expanded: OrderedSet::new() },
            ],
            synapses: vec![
                SynapseSpec { id: "imported-data".into(), from: IMPORT_SOURCE_WIDGET.into(), to: IMPORT_GEOMETRY_WIDGET.into(), from_port: "text".into(), to_port: "data".into() },
                SynapseSpec { id: "imported-geometry".into(), from: IMPORT_GEOMETRY_WIDGET.into(), to: IMPORT_PREVIEW_WIDGET.into(), from_port: "geometry".into(), to_port: String::new() },
            ],
            layout,
        };
        Generation3dSnapshot { fixture, ..Generation3dSnapshot::default() }
    }

    /// 🔎️ The payload an import fixture planted, and the neuron kind that consumes it.
    pub fn imported_source(snapshot: &Generation3dSnapshot) -> Option<(&str, &str)> {
        let text = snapshot.fixture.widgets.iter().find_map(|widget| match widget {
            Widget::InputNote { id, text } if id == IMPORT_SOURCE_WIDGET => Some(text.as_str()),
            _ => None,
        })?;
        let kind = snapshot.fixture.widgets.iter().find_map(|widget| match widget {
            Widget::Neuron { id, neuron_kind, .. } if id == IMPORT_GEOMETRY_WIDGET => Some(neuron_kind.as_str()),
            _ => None,
        })?;
        Some((kind, text))
    }
}
pub use mesh_bridge::io_error;
//#endregion 🔺️MeshBridge

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::schema::Generation3dAnalyzer;
    use crate::Generation3dSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.generation3d", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_GLTF: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_LAS: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_OBJ: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    const DEP_PLY: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_PNG: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    const DEP_STL: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct Generation3dComposerComposition;

    impl ArtifactComposition for Generation3dComposerComposition {
        type Snapshot = Generation3dSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_DWG, DEP_GLTF, DEP_JSON, DEP_LAS, DEP_OBJ, DEP_PLY, DEP_PNG, DEP_STL, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                    };
                    let analysis = Generation3dAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_DWG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_GLTF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::gltf::v2_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_LAS {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::las::v1_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_OBJ {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::obj::v3_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PLY {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::ply::v1_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PNG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::png::v1_2::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_STL {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::stl::v_ascii::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            Err(ComposeError { message: "Generation3dComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️IoRegistry
/// 🚪️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// the composer/export-entry registry lives with the rest of `🚪️io`, not behind an engine facade.
pub mod io_registry {
    use crate::standards::v1::subsets::any::schema::Generation3dBuilder as Generation3dAnyBuilder;
    use crate::standards::v1::subsets::any::schema::Generation3dComposer as Generation3dAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ArtifactBuilder, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource, IoConfidence, IoPayload, StandardId, SubsetId};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const GENERATION3D_DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.generation3d", standard: StandardId("1"), subset: SubsetId("*") };
    const GENERATION3D_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    async fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::Generation3dSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == GENERATION3D_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => Generation3dAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => Generation3dAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "Generation3dComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == GENERATION3D_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "Generation3dComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_LAS_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };
    fn compose_export_las(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources).await?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::las::v1_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_LAS_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_PLY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    fn compose_export_ply(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources).await?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::ply::v1_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PLY_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    // 🖼️ No `compose_export_png`/`compose_export_json` here: generation2d owns both EXPORT claims
    // (26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME D3, documented tie-break — see
    // `../../🦀️.rs`'s `definition()` docstring). `derived_composition`'s `reads()` above still
    // lists `DEP_PNG`/`DEP_JSON`, so import is unaffected.
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources).await?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    fn compose_export_stl(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources).await?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::stl::v_ascii::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_STL_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    fn compose_export_gltf(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources).await?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::gltf::v2_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_GLTF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    fn compose_export_obj(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources).await?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::obj::v3_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_OBJ_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };
    fn compose_export_txt(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources).await?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_TXT_DIALECT, payload: IoPayload::Text(String::from_utf8(bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?), diagnostics: Vec::new(), confidence: IoConfidence::High })
        })
    }
    //#endregion 🔖️ExportEntries

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<Generation3dAnyComposer>(),
                    ComposerEntry { writes: EXPORT_LAS_DIALECT, reads: &[GENERATION3D_DIALECT], compose: compose_export_las },
                    ComposerEntry { writes: EXPORT_PLY_DIALECT, reads: &[GENERATION3D_DIALECT], compose: compose_export_ply },
                    ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[GENERATION3D_DIALECT], compose: compose_export_dwg },
                    ComposerEntry { writes: EXPORT_STL_DIALECT, reads: &[GENERATION3D_DIALECT], compose: compose_export_stl },
                    ComposerEntry { writes: EXPORT_GLTF_DIALECT, reads: &[GENERATION3D_DIALECT], compose: compose_export_gltf },
                    ComposerEntry { writes: EXPORT_OBJ_DIALECT, reads: &[GENERATION3D_DIALECT], compose: compose_export_obj },
                    ComposerEntry { writes: EXPORT_TXT_DIALECT, reads: &[GENERATION3D_DIALECT], compose: compose_export_txt },
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️IoRegistry
