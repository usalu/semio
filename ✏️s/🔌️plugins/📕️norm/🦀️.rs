//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

#![allow(async_fn_in_trait)]

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for all fifteen norm-family editor/viewer pairs.
    pub enum NormApps: PluginApp {
        Din4108Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_din4108::editor::din4108::Din4108PlayApp>>),
        Din4108Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_din4108::viewer::din4108::Din4108Viewer>>),
        Din16798Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_din16798::editor::din16798::Din16798PlayApp>>),
        Din16798Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_din16798::viewer::din16798::Din16798Viewer>>),
        Din18599Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_din18599::editor::din18599::Din18599PlayApp>>),
        Din18599Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_din18599::viewer::din18599::Din18599Viewer>>),
        En1990Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1990::editor::en1990::En1990PlayApp>>),
        En1990Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1990::viewer::en1990::En1990Viewer>>),
        En1991Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1991::editor::en1991::En1991PlayApp>>),
        En1991Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1991::viewer::en1991::En1991Viewer>>),
        En1992Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1992::editor::en1992::En1992PlayApp>>),
        En1992Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1992::viewer::en1992::En1992Viewer>>),
        En1993Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1993::editor::en1993::En1993PlayApp>>),
        En1993Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1993::viewer::en1993::En1993Viewer>>),
        En1994Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1994::editor::en1994::En1994PlayApp>>),
        En1994Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1994::viewer::en1994::En1994Viewer>>),
        En1995Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1995::editor::en1995::En1995PlayApp>>),
        En1995Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1995::viewer::en1995::En1995Viewer>>),
        En1996Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1996::editor::en1996::En1996PlayApp>>),
        En1996Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1996::viewer::en1996::En1996Viewer>>),
        En1997Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1997::editor::en1997::En1997PlayApp>>),
        En1997Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1997::viewer::en1997::En1997Viewer>>),
        En1998Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1998::editor::en1998::En1998PlayApp>>),
        En1998Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1998::viewer::en1998::En1998Viewer>>),
        En1999Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1999::editor::en1999::En1999PlayApp>>),
        En1999Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1999::viewer::en1999::En1999Viewer>>),
        Iso16757Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_iso16757::editor::iso16757::Iso16757PlayApp>>),
        Iso16757Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_iso16757::viewer::iso16757::Iso16757Viewer>>),
        Vdi3805Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_vdi3805::editor::vdi3805::Vdi3805PlayApp>>),
        Vdi3805Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_vdi3805::viewer::vdi3805::Vdi3805Viewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1b) replaces the deleted `register_norm_exports`
/// `.setup()` fan-out with fifteen data declarations, one per norm family. `.editor(…)`/`.viewer(…)`
/// (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) replace the retired `.document_app(…)`
/// call per family with the role-split pair — the shared `NormConfig` schema every one of the fifteen
/// `PlayApp`s uses is still registered idempotently by whichever
/// editor binds first (`ArtifactEditor::app_schema()` override), mirroring the `🗒️note` exemplar.
/// `.activation(…)`/`.execution(…)`/`.requests(…)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining, `📓️design-abi.md` §3/§6) are this
/// crate's migration proof: one `OnArtifactKind` event per owned norm family, read live from each
/// family's own `artifact_kind().id` (never hardcoded, same standard `🗄️stdio`'s 36-kind migration
/// set), `Isolated` execution, one `documents.write` ask covering all fifteen editors.
pub fn plugin() -> Result<Plugin<NormApps>, PluginAssemblyError> {
    let din4108 = semio_s_artifact_norm_din4108::declaration(semio_s_artifact_norm_din4108::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let din16798 = semio_s_artifact_norm_din16798::declaration(semio_s_artifact_norm_din16798::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let din18599 = semio_s_artifact_norm_din18599::declaration(semio_s_artifact_norm_din18599::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1990 = semio_s_artifact_norm_en1990::declaration(semio_s_artifact_norm_en1990::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1991 = semio_s_artifact_norm_en1991::declaration(semio_s_artifact_norm_en1991::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1992 = semio_s_artifact_norm_en1992::declaration(semio_s_artifact_norm_en1992::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1993 = semio_s_artifact_norm_en1993::declaration(semio_s_artifact_norm_en1993::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1994 = semio_s_artifact_norm_en1994::declaration(semio_s_artifact_norm_en1994::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1995 = semio_s_artifact_norm_en1995::declaration(semio_s_artifact_norm_en1995::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1996 = semio_s_artifact_norm_en1996::declaration(semio_s_artifact_norm_en1996::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1997 = semio_s_artifact_norm_en1997::declaration(semio_s_artifact_norm_en1997::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1998 = semio_s_artifact_norm_en1998::declaration(semio_s_artifact_norm_en1998::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1999 = semio_s_artifact_norm_en1999::declaration(semio_s_artifact_norm_en1999::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let iso16757 = semio_s_artifact_norm_iso16757::declaration(semio_s_artifact_norm_iso16757::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let vdi3805 = semio_s_artifact_norm_vdi3805::declaration(semio_s_artifact_norm_vdi3805::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    Plugin::<NormApps>::builder("norm")
        .label("Norm")
        .version("0.1.0")
        .package_id("semio:norm")
        .artifact(din4108)
        .artifact(din16798)
        .artifact(din18599)
        .artifact(en1990)
        .artifact(en1991)
        .artifact(en1992)
        .artifact(en1993)
        .artifact(en1994)
        .artifact(en1995)
        .artifact(en1996)
        .artifact(en1997)
        .artifact(en1998)
        .artifact(en1999)
        .artifact(iso16757)
        .artifact(vdi3805)
        .editor::<semio_s_artifact_norm_din4108::editor::din4108::Din4108PlayApp>(semio_s_artifact_norm_din4108::editor::din4108::create_din4108_app())
        .editor_mutation_roster::<semio_s_artifact_norm_din4108::editor::din4108::Din4108PlayApp>()
        .viewer::<semio_s_artifact_norm_din4108::viewer::din4108::Din4108Viewer>(semio_s_artifact_norm_din4108::viewer::din4108::create_din4108_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_din4108::viewer::din4108::Din4108Viewer>()
        .editor::<semio_s_artifact_norm_din16798::editor::din16798::Din16798PlayApp>(semio_s_artifact_norm_din16798::editor::din16798::create_din16798_app())
        .editor_mutation_roster::<semio_s_artifact_norm_din16798::editor::din16798::Din16798PlayApp>()
        .viewer::<semio_s_artifact_norm_din16798::viewer::din16798::Din16798Viewer>(semio_s_artifact_norm_din16798::viewer::din16798::create_din16798_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_din16798::viewer::din16798::Din16798Viewer>()
        .editor::<semio_s_artifact_norm_din18599::editor::din18599::Din18599PlayApp>(semio_s_artifact_norm_din18599::editor::din18599::create_din18599_app())
        .editor_mutation_roster::<semio_s_artifact_norm_din18599::editor::din18599::Din18599PlayApp>()
        .viewer::<semio_s_artifact_norm_din18599::viewer::din18599::Din18599Viewer>(semio_s_artifact_norm_din18599::viewer::din18599::create_din18599_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_din18599::viewer::din18599::Din18599Viewer>()
        .editor::<semio_s_artifact_norm_en1990::editor::en1990::En1990PlayApp>(semio_s_artifact_norm_en1990::editor::en1990::create_en1990_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1990::editor::en1990::En1990PlayApp>()
        .viewer::<semio_s_artifact_norm_en1990::viewer::en1990::En1990Viewer>(semio_s_artifact_norm_en1990::viewer::en1990::create_en1990_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1990::viewer::en1990::En1990Viewer>()
        .editor::<semio_s_artifact_norm_en1991::editor::en1991::En1991PlayApp>(semio_s_artifact_norm_en1991::editor::en1991::create_en1991_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1991::editor::en1991::En1991PlayApp>()
        .viewer::<semio_s_artifact_norm_en1991::viewer::en1991::En1991Viewer>(semio_s_artifact_norm_en1991::viewer::en1991::create_en1991_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1991::viewer::en1991::En1991Viewer>()
        .editor::<semio_s_artifact_norm_en1992::editor::en1992::En1992PlayApp>(semio_s_artifact_norm_en1992::editor::en1992::create_en1992_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1992::editor::en1992::En1992PlayApp>()
        .viewer::<semio_s_artifact_norm_en1992::viewer::en1992::En1992Viewer>(semio_s_artifact_norm_en1992::viewer::en1992::create_en1992_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1992::viewer::en1992::En1992Viewer>()
        .editor::<semio_s_artifact_norm_en1993::editor::en1993::En1993PlayApp>(semio_s_artifact_norm_en1993::editor::en1993::create_en1993_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1993::editor::en1993::En1993PlayApp>()
        .viewer::<semio_s_artifact_norm_en1993::viewer::en1993::En1993Viewer>(semio_s_artifact_norm_en1993::viewer::en1993::create_en1993_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1993::viewer::en1993::En1993Viewer>()
        .editor::<semio_s_artifact_norm_en1994::editor::en1994::En1994PlayApp>(semio_s_artifact_norm_en1994::editor::en1994::create_en1994_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1994::editor::en1994::En1994PlayApp>()
        .viewer::<semio_s_artifact_norm_en1994::viewer::en1994::En1994Viewer>(semio_s_artifact_norm_en1994::viewer::en1994::create_en1994_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1994::viewer::en1994::En1994Viewer>()
        .editor::<semio_s_artifact_norm_en1995::editor::en1995::En1995PlayApp>(semio_s_artifact_norm_en1995::editor::en1995::create_en1995_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1995::editor::en1995::En1995PlayApp>()
        .viewer::<semio_s_artifact_norm_en1995::viewer::en1995::En1995Viewer>(semio_s_artifact_norm_en1995::viewer::en1995::create_en1995_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1995::viewer::en1995::En1995Viewer>()
        .editor::<semio_s_artifact_norm_en1996::editor::en1996::En1996PlayApp>(semio_s_artifact_norm_en1996::editor::en1996::create_en1996_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1996::editor::en1996::En1996PlayApp>()
        .viewer::<semio_s_artifact_norm_en1996::viewer::en1996::En1996Viewer>(semio_s_artifact_norm_en1996::viewer::en1996::create_en1996_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1996::viewer::en1996::En1996Viewer>()
        .editor::<semio_s_artifact_norm_en1997::editor::en1997::En1997PlayApp>(semio_s_artifact_norm_en1997::editor::en1997::create_en1997_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1997::editor::en1997::En1997PlayApp>()
        .viewer::<semio_s_artifact_norm_en1997::viewer::en1997::En1997Viewer>(semio_s_artifact_norm_en1997::viewer::en1997::create_en1997_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1997::viewer::en1997::En1997Viewer>()
        .editor::<semio_s_artifact_norm_en1998::editor::en1998::En1998PlayApp>(semio_s_artifact_norm_en1998::editor::en1998::create_en1998_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1998::editor::en1998::En1998PlayApp>()
        .viewer::<semio_s_artifact_norm_en1998::viewer::en1998::En1998Viewer>(semio_s_artifact_norm_en1998::viewer::en1998::create_en1998_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1998::viewer::en1998::En1998Viewer>()
        .editor::<semio_s_artifact_norm_en1999::editor::en1999::En1999PlayApp>(semio_s_artifact_norm_en1999::editor::en1999::create_en1999_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1999::editor::en1999::En1999PlayApp>()
        .viewer::<semio_s_artifact_norm_en1999::viewer::en1999::En1999Viewer>(semio_s_artifact_norm_en1999::viewer::en1999::create_en1999_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1999::viewer::en1999::En1999Viewer>()
        .editor::<semio_s_artifact_norm_iso16757::editor::iso16757::Iso16757PlayApp>(semio_s_artifact_norm_iso16757::editor::iso16757::create_iso16757_app())
        .editor_mutation_roster::<semio_s_artifact_norm_iso16757::editor::iso16757::Iso16757PlayApp>()
        .viewer::<semio_s_artifact_norm_iso16757::viewer::iso16757::Iso16757Viewer>(semio_s_artifact_norm_iso16757::viewer::iso16757::create_iso16757_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_iso16757::viewer::iso16757::Iso16757Viewer>()
        .editor::<semio_s_artifact_norm_vdi3805::editor::vdi3805::Vdi3805PlayApp>(semio_s_artifact_norm_vdi3805::editor::vdi3805::create_vdi3805_app())
        .editor_mutation_roster::<semio_s_artifact_norm_vdi3805::editor::vdi3805::Vdi3805PlayApp>()
        .viewer::<semio_s_artifact_norm_vdi3805::viewer::vdi3805::Vdi3805Viewer>(semio_s_artifact_norm_vdi3805::viewer::vdi3805::create_vdi3805_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_vdi3805::viewer::vdi3805::Vdi3805Viewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_din4108::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_din16798::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_din18599::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1990::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1991::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1992::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1993::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1994::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1995::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1996::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1997::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1998::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1999::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_iso16757::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_vdi3805::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist norm family edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests

semio_framework_plugin::plugin_exports!(plugin, NormApps);
