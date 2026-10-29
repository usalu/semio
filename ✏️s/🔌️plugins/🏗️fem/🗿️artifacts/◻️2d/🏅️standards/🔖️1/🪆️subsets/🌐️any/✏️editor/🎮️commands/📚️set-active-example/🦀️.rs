//! 📚️ 📚️ Fem2d play app commands command — `set-active-example`.

use semio_framework_plugin::NoConfig;

use semio_framework_plugin::NoConfigMutation;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use pack::snapshot::ArtifactDsl;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️SetActiveExample
//#endregion 🔖️SetActiveExample

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 📚️ The bundled example's own `ExampleSource` id (`📚️examples/🎬️demo`) loads that fixture; any other
/// id resets to an empty document. The id is not a local literal because the shell's navbar switcher
/// dispatches this action straight from `PluginManifest.examples`, whose entries carry exactly that id.
/// Also resets camera and result display to their defaults as two granular rows — never a whole-config
/// `Snapshot`, which would also wipe the session locale.
///
/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (`SetSnapshot` — see
/// `📓️taxonomy.md`'s forbidden vocabulary), so this builds `reset_document_effect` (io)
/// (a `Effect::LoadDocument`, outside undo history) instead of an `artifact_mutations` entry.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let document = if payload.example_id == semio_s_artifact_fem_2d_io::examples::demo::ID {
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot().map_err(|error| Fault::from(format!("fem2d.set-active-example.load: {error}")))?
    } else {
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::empty_fem2d_snapshot()
    };
    Ok(Emit { effects: vec![semio_s_artifact_fem_2d_io::standards::v1::subsets::any::io::reset_document_effect(&document)], ..Default::default() })
}

//#region 🧪️Tests
//#endregion 🧪️Tests
