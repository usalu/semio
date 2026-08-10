//! 👥️ Procedural3dPresence — shareable live ephemeral state + mutations.
//!
//! Shareable live subset of the 3d procedural surface: selection, hover, cameras, utility, show-mode.

use flow::CameraJson;
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use store::ArtifactPack;
use crate::apps::procedural3d::config::Procedural3dPreviewCamera;


//#region 🔖️Presence
/// 👥️ Shareable live subset of procedural 3d view state (selection, hover, cameras, utility).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "procedural3d.presence")]
#[dsl(layout = "lines")]
pub struct Procedural3dPresence {
    /// 👁️ Selected flow-graph widget ids.
    pub selected_node_ids: Vec<String>,
    /// 👁️ Hovered flow-graph widget id.
    pub hovered_node_id: Option<String>,
    /// 📷️ The flow-graph node canvas camera.
    #[dsl(block)]
    pub camera: CameraJson,
    /// 📷️ The 3D preview viewport camera.
    #[dsl(block)]
    pub preview_camera: Procedural3dPreviewCamera,
    /// 🖱️ Marquee selection method.
    pub selection_method: String,
    /// 🧰 Active utility id.
    pub active_utility_id: String,
    /// 👁️ Preview shading mode.
    pub show_mode: String,
}

impl Default for Procedural3dPresence {
    fn default() -> Self {
        Self {
            selected_node_ids: Vec::new(),
            hovered_node_id: None,
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            preview_camera: Procedural3dPreviewCamera::default(),
            selection_method: "rectangle".into(),
            active_utility_id: String::new(),
            show_mode: "shaded".into(),
        }
    }
}

impl protocol::MutationDiff<Procedural3dPresence> for Procedural3dPresence {
    fn apply(&self, _base: &Procedural3dPresence) -> Procedural3dPresence {
        self.clone()
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl store::ArtifactDsl for Procedural3dPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for Procedural3dPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(rename_all = "camelCase")]
pub enum Procedural3dPresenceMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        presence: Procedural3dPresence,
    },
}

impl Mutation<Procedural3dPresence> for Procedural3dPresenceMutation {
    type Diff = Procedural3dPresence;

    fn diff(&self, _base: &Procedural3dPresence) -> Procedural3dPresence {
        match self {
            Self::Snapshot { presence } => presence.clone(),
        }
    }

    fn inverse(&self, base: &Procedural3dPresence) -> Vec<Self> {
        vec![Self::Snapshot { presence: base.clone() }]
    }
}

impl protocol::OpText for Procedural3dPresenceMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() {
                    line[keyword.len()..].trim_start()
                } else {
                    ""
                };
                let record = dsl::parse(
                    body,
                    &spec_fn(),
                    &dsl::ParseOptions {
                        limits: dsl::Limits::default(),
                        mode: dsl::SourceMode::Inline,
                    },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants
            .iter()
            .find(|(k, _)| k == &keyword)
            .map(|(_, s)| *s)
            .expect("variant spec must exist for its own keyword");
        let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for Procedural3dPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation
