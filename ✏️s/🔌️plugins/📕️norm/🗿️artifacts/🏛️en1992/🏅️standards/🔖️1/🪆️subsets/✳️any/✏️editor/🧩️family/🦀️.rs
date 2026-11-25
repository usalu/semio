//! 🧩️ En1992 editor family: the identity constants of the editor, its headless `NormFamily` binding and the incremental check units; the surfaces and the app wiring depend on it.

use semio_s_plugin_norm_compliance::NormHost;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;
use semio_s_artifact_norm_en1992_mutations::En1992Mutation;

//#region 🔖️Constants
/// 🏷️ This standard's display name â the app label, its artifact-kind name and the catalogue headline.
pub const LABEL: &str = "EN 1992";
/// 🆔️ The playground/registry variant key â every body key, window id and schema is derived from it.
pub const VARIANT: &str = "en1992";
/// 🆔️ Retained-command / UI action controller id.
pub const CONTROLLER_ID: &str = "s.norm-eurocode-structural.en1992@1/*#editor";
pub const DOCUMENT_SCHEMA: &str = "semio.norm.en1992/v1";
/// 🏷️ The scene entity kinds EN 1992 projects: one interaction granularity each.
pub const ENTITY_KIND_SCENE_KINDS: &[semio_s_plugin_norm_structural_scene::EntityKind] = &[
    semio_s_plugin_norm_structural_scene::EntityKind::Member,
    semio_s_plugin_norm_structural_scene::EntityKind::Plate,
    semio_s_plugin_norm_structural_scene::EntityKind::Solid,
    semio_s_plugin_norm_structural_scene::EntityKind::Fastener,
    semio_s_plugin_norm_structural_scene::EntityKind::Support,
    semio_s_plugin_norm_structural_scene::EntityKind::Load,
    semio_s_plugin_norm_structural_scene::EntityKind::Surface,
];
//#endregion 🔖️Constants

//#region 🧩️ComplianceFamily
/// 🧩️ Headless `NormFamily` binding (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. This is stateful/host-facing behaviour, so it
/// belongs to the app that edits the artifact, not the artifact's own `🧬️schema`.
pub struct En1992Family;

impl semio_s_plugin_norm_compliance::NormFamily for En1992Family {
    type Document = En1992Snapshot;
    type Mutation = En1992Mutation;

    fn family_id() -> semio_s_plugin_norm_compliance::NormFamilyId {
        semio_s_plugin_norm_compliance::NormFamilyId::En1992
    }

    fn evaluate(document: &En1992Snapshot) -> semio_s_plugin_norm_compliance::CheckReport {
        <En1992Family as semio_s_plugin_norm_compliance::IncrementalNormFamily>::evaluate_incrementally(document)
    }

    fn live_report(document: &En1992Snapshot) -> std::sync::Arc<semio_s_plugin_norm_compliance::CheckReport> {
        semio_s_artifact_norm_en1992_inferences::live::report(document)
    }

    fn live_progress() -> semio_s_plugin_norm_compliance::incremental_evaluation::EvalProgress {
        semio_s_artifact_norm_en1992_inferences::live::progress()
    }

    fn live_responsive(document: &En1992Snapshot, budget: std::time::Duration) -> (std::sync::Arc<semio_s_plugin_norm_compliance::CheckReport>, bool) {
        semio_s_artifact_norm_en1992_inferences::live::responsive(document, budget)
    }

    fn cancel_live() {
        semio_s_artifact_norm_en1992_inferences::live::cancel()
    }
}

pub type Host = NormHost<En1992Family>;
//#endregion 🧩️ComplianceFamily

//#region 🪢️Mounts
#[path = "../🧮️incremental/🦀️.rs"]
pub mod incremental;
//#endregion 🪢️Mounts
