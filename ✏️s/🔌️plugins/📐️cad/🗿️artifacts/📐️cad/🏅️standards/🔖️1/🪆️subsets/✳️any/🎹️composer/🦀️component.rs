//! 🎹️ CadComposer (1/✳️any) — analyzer + builder glued. Reads native `s.cad` sources
//! plus any of: stdio.dwg, stdio.glb, stdio.gltf, stdio.ifc, stdio.json, stdio.obj, stdio.png, stdio.step, stdio.stl. Writes one `s.cad` (1/✳️any) snapshot.

use semio_framework_plugin::{ArtifactComposer, ArtifactBuilder, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
use crate::artifacts::cad::CadSnapshot;
use crate::artifacts::cad::standards::v1::subsets::any::analyzer::CadAnalyzer;
use semio_framework_plugin::ArtifactAnalyzer as _;

const DIALECT: Dialect = Dialect { artifact_kind: "s.cad", standard: StandardId("1"), subset: SubsetId("*") };
const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
const DEP_GLB: Dialect = Dialect { artifact_kind: "s.stdio.glb", standard: StandardId("2.0"), subset: SubsetId("*") };
const DEP_GLTF: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
const DEP_IFC: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId("*") };
const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
const DEP_OBJ: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
const DEP_PNG: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
const DEP_STEP: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };
const DEP_STL: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };


pub struct CadComposer;

impl ArtifactComposer for CadComposer {
    type Snapshot = CadSnapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] {
        &[DIALECT, DEP_DWG, DEP_GLB, DEP_GLTF, DEP_IFC, DEP_JSON, DEP_OBJ, DEP_PNG, DEP_STEP, DEP_STL]
    }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        for source in sources {
            if source.dialect == DIALECT {
                let native = match &source.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                };
                let analysis = CadAnalyzer::analyze(&[native]);
                if let Some(snapshot) = analysis.parts.snapshot {
                    return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                }
            }
            if source.dialect == DEP_DWG {
                let text: Option<String> = match &source.payload {
                    AnalyzeSource::Text(t) => Some(t.to_string()),
                    AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                };
                if let Some(text) = text {
                    if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_text(&text) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            if source.dialect == DEP_GLB {
                let text: Option<String> = match &source.payload {
                    AnalyzeSource::Text(t) => Some(t.to_string()),
                    AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                };
                if let Some(text) = text {
                    if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::glb::v2_0::any::deserialize_text(&text) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            if source.dialect == DEP_GLTF {
                let text: Option<String> = match &source.payload {
                    AnalyzeSource::Text(t) => Some(t.to_string()),
                    AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                };
                if let Some(text) = text {
                    if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::gltf::v2_0::any::deserialize_text(&text) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            if source.dialect == DEP_IFC {
                let text: Option<String> = match &source.payload {
                    AnalyzeSource::Text(t) => Some(t.to_string()),
                    AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                };
                if let Some(text) = text {
                    if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::ifc::v4::any::deserialize_text(&text) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            if source.dialect == DEP_JSON {
                let text: Option<String> = match &source.payload {
                    AnalyzeSource::Text(t) => Some(t.to_string()),
                    AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                };
                if let Some(text) = text {
                    if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_text(&text) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            if source.dialect == DEP_OBJ {
                let text: Option<String> = match &source.payload {
                    AnalyzeSource::Text(t) => Some(t.to_string()),
                    AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                };
                if let Some(text) = text {
                    if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::obj::v3_0::any::deserialize_text(&text) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            if source.dialect == DEP_PNG {
                let text: Option<String> = match &source.payload {
                    AnalyzeSource::Text(t) => Some(t.to_string()),
                    AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                };
                if let Some(text) = text {
                    if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::png::v1_2::any::deserialize_text(&text) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            if source.dialect == DEP_STEP {
                let text: Option<String> = match &source.payload {
                    AnalyzeSource::Text(t) => Some(t.to_string()),
                    AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                };
                if let Some(text) = text {
                    if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::step::v_ap214::any::deserialize_text(&text) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            if source.dialect == DEP_STL {
                let text: Option<String> = match &source.payload {
                    AnalyzeSource::Text(t) => Some(t.to_string()),
                    AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                };
                if let Some(text) = text {
                    if let Ok(snapshot) = crate::artifacts::cad::io::import::deserializers::artifacts::stl::v_ascii::any::deserialize_text(&text) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }

        }
        Err(ComposeError { message: "CadComposer: no source in a known read dialect".into(), diagnostics: Vec::new() })
    }
}
