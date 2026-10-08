//! 📜️ Text representation of the model snapshot: the semio envelope around the derived DSL record, plus the JSON decoders and the
//! comparison projections the language-neutral tests speak.

/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

use crate::ModelSnapshot;

/// 🗄️ The demo model, handcrafted in this facet's DSL.
pub const BIM_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

impl store::ArtifactDsl for ModelSnapshot {
    const EXTENSION: &'static str = "bim";
    fn envelope_id() -> &'static str {
        "bim.model"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let (envelope, body) = store::semio_format::split_text_preamble(text).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        if !envelope.matches_identity(Self::envelope_id(), store::semio_format::Component::Dsl, 1) {
            return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "BIM model native text envelope mismatch", semio_framework_diagnostic::TextSpan::at(1, 1)));
        }
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid BIM model envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📖️ Parses `.bim` DSL text into a [`ModelSnapshot`].
pub fn parse_dsl(text: &str) -> Result<ModelSnapshot, semio_framework_diagnostic::TextError> {
    <ModelSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a [`ModelSnapshot`] back to `.bim` DSL text.
pub fn print_dsl(snapshot: &ModelSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📄️ The demo snapshot, parsed from its committed DSL.
pub fn default_snapshot() -> ModelSnapshot {
    parse_dsl(BIM_EXAMPLE_TEXT).unwrap_or_default()
}

/// 📥️ Decodes a committed snapshot document (JSON) into a [`ModelSnapshot`].
pub fn decode_model_snapshot_json(text: &str) -> Result<ModelSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// ⚖️ The semantic projection this subset is compared through: the canonical JSON of the whole snapshot.
pub fn encode_model_projection_json(snapshot: &ModelSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 💡️ The canonical JSON of one inferred table the inference oracles compare, by slug: `storey-levels`, `wall-layout`, `opening-frames`, `stair-runs`, `spaces`, `quantities` (the closed-form kinds), `plan-metrics`, `diagnostics`, or a derived table such as `wall-solids` (base, top and volume of every straight wall) and `frame-solids`.
pub fn encode_inference_projection_json(snapshot: &ModelSnapshot, slug: &str) -> Option<String> {
    use protocol::Inference;
    let inferred = crate::standards::v1::subsets::any::schema::inferences::ModelInference::infer(snapshot).ok()?;
    match slug {
        "storey-levels" => Some(semio_framework_pack_json::to_json_string(&inferred.storey_levels)),
        "wall-layout" => Some(semio_framework_pack_json::to_json_string(&inferred.wall_layout)),
        "opening-frames" => Some(semio_framework_pack_json::to_json_string(&inferred.opening_frames)),
        "stair-runs" => Some(crate::standards::v1::subsets::any::schema::inferences::stair_runs::table_json(&inferred.stair_runs)),
        "spaces" => Some(crate::standards::v1::subsets::any::schema::inferences::spaces::table_json(&inferred.spaces)),
        "quantities" => Some(crate::standards::v1::subsets::any::schema::inferences::quantities::table_json(snapshot, &inferred.quantities)),
        "frame-solids" => Some(crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::planar_projection_json(snapshot, &inferred.element_solids)),
        "plan-metrics" => Some(crate::standards::v1::subsets::any::schema::inferences::plan_linework::metrics_json(&inferred.plan_linework)),
        "diagnostics" => Some(crate::standards::v1::subsets::any::schema::inferences::diagnostics::table_json(&inferred.diagnostics)),
        "wall-solids" => {
            let rows: Vec<String> = snapshot.walls.iter().filter(|(_, wall)| matches!(wall.axis, crate::Axis::Line { .. })).filter_map(|(id, _)| inferred.wall_layout.get(id).map(|layout| format!("{}:{{\"base_z\":{},\"top_z\":{},\"volume\":{}}}", semio_framework_pack_json::to_json_string(id), layout.base_z, layout.top_z, layout.volume))).collect();
            Some(format!("{{{}}}", rows.join(",")))
        }
        _ => None,
    }
}
