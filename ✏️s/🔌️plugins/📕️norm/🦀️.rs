//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

//#region 🗃️Apps
/// 🗃️ Closed runtime app fleet for all fifteen norm-family editor/viewer pairs.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum NormApps: PluginApp {
        Din4108Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::din4108::Din4108PlayApp>>),
        Din4108Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::din4108::Din4108Viewer>>),
        Din16798Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::din16798::Din16798PlayApp>>),
        Din16798Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::din16798::Din16798Viewer>>),
        Din18599Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::din18599::Din18599PlayApp>>),
        Din18599Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::din18599::Din18599Viewer>>),
        En1990Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::en1990::En1990PlayApp>>),
        En1990Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::en1990::En1990Viewer>>),
        En1991Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::en1991::En1991PlayApp>>),
        En1991Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::en1991::En1991Viewer>>),
        En1992Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::en1992::En1992PlayApp>>),
        En1992Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::en1992::En1992Viewer>>),
        En1993Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::en1993::En1993PlayApp>>),
        En1993Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::en1993::En1993Viewer>>),
        En1994Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::en1994::En1994PlayApp>>),
        En1994Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::en1994::En1994Viewer>>),
        En1995Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::en1995::En1995PlayApp>>),
        En1995Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::en1995::En1995Viewer>>),
        En1996Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::en1996::En1996PlayApp>>),
        En1996Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::en1996::En1996Viewer>>),
        En1997Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::en1997::En1997PlayApp>>),
        En1997Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::en1997::En1997Viewer>>),
        En1998Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::en1998::En1998PlayApp>>),
        En1998Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::en1998::En1998Viewer>>),
        En1999Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::en1999::En1999PlayApp>>),
        En1999Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::en1999::En1999Viewer>>),
        Iso16757Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::iso16757::Iso16757PlayApp>>),
        Iso16757Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::iso16757::Iso16757Viewer>>),
        Vdi3805Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::vdi3805::Vdi3805PlayApp>>),
        Vdi3805Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::vdi3805::Vdi3805Viewer>>),
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
    let din4108 = crate::artifacts::din4108::declaration(crate::artifacts::din4108::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let din16798 = crate::artifacts::din16798::declaration(crate::artifacts::din16798::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let din18599 = crate::artifacts::din18599::declaration(crate::artifacts::din18599::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1990 = crate::artifacts::en1990::declaration(crate::artifacts::en1990::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1991 = crate::artifacts::en1991::declaration(crate::artifacts::en1991::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1992 = crate::artifacts::en1992::declaration(crate::artifacts::en1992::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1993 = crate::artifacts::en1993::declaration(crate::artifacts::en1993::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1994 = crate::artifacts::en1994::declaration(crate::artifacts::en1994::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1995 = crate::artifacts::en1995::declaration(crate::artifacts::en1995::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1996 = crate::artifacts::en1996::declaration(crate::artifacts::en1996::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1997 = crate::artifacts::en1997::declaration(crate::artifacts::en1997::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1998 = crate::artifacts::en1998::declaration(crate::artifacts::en1998::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1999 = crate::artifacts::en1999::declaration(crate::artifacts::en1999::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let iso16757 = crate::artifacts::iso16757::declaration(crate::artifacts::iso16757::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let vdi3805 = crate::artifacts::vdi3805::declaration(crate::artifacts::vdi3805::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
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
        .editor::<crate::editor::din4108::Din4108PlayApp>(crate::editor::din4108::create_din4108_app())
        .editor_mutation_roster::<crate::editor::din4108::Din4108PlayApp>()
        .viewer::<crate::viewer::din4108::Din4108Viewer>(crate::viewer::din4108::create_din4108_viewer())
        .viewer_mutation_roster::<crate::viewer::din4108::Din4108Viewer>()
        .editor::<crate::editor::din16798::Din16798PlayApp>(crate::editor::din16798::create_din16798_app())
        .editor_mutation_roster::<crate::editor::din16798::Din16798PlayApp>()
        .viewer::<crate::viewer::din16798::Din16798Viewer>(crate::viewer::din16798::create_din16798_viewer())
        .viewer_mutation_roster::<crate::viewer::din16798::Din16798Viewer>()
        .editor::<crate::editor::din18599::Din18599PlayApp>(crate::editor::din18599::create_din18599_app())
        .editor_mutation_roster::<crate::editor::din18599::Din18599PlayApp>()
        .viewer::<crate::viewer::din18599::Din18599Viewer>(crate::viewer::din18599::create_din18599_viewer())
        .viewer_mutation_roster::<crate::viewer::din18599::Din18599Viewer>()
        .editor::<crate::editor::en1990::En1990PlayApp>(crate::editor::en1990::create_en1990_app())
        .editor_mutation_roster::<crate::editor::en1990::En1990PlayApp>()
        .viewer::<crate::viewer::en1990::En1990Viewer>(crate::viewer::en1990::create_en1990_viewer())
        .viewer_mutation_roster::<crate::viewer::en1990::En1990Viewer>()
        .editor::<crate::editor::en1991::En1991PlayApp>(crate::editor::en1991::create_en1991_app())
        .editor_mutation_roster::<crate::editor::en1991::En1991PlayApp>()
        .viewer::<crate::viewer::en1991::En1991Viewer>(crate::viewer::en1991::create_en1991_viewer())
        .viewer_mutation_roster::<crate::viewer::en1991::En1991Viewer>()
        .editor::<crate::editor::en1992::En1992PlayApp>(crate::editor::en1992::create_en1992_app())
        .editor_mutation_roster::<crate::editor::en1992::En1992PlayApp>()
        .viewer::<crate::viewer::en1992::En1992Viewer>(crate::viewer::en1992::create_en1992_viewer())
        .viewer_mutation_roster::<crate::viewer::en1992::En1992Viewer>()
        .editor::<crate::editor::en1993::En1993PlayApp>(crate::editor::en1993::create_en1993_app())
        .editor_mutation_roster::<crate::editor::en1993::En1993PlayApp>()
        .viewer::<crate::viewer::en1993::En1993Viewer>(crate::viewer::en1993::create_en1993_viewer())
        .viewer_mutation_roster::<crate::viewer::en1993::En1993Viewer>()
        .editor::<crate::editor::en1994::En1994PlayApp>(crate::editor::en1994::create_en1994_app())
        .editor_mutation_roster::<crate::editor::en1994::En1994PlayApp>()
        .viewer::<crate::viewer::en1994::En1994Viewer>(crate::viewer::en1994::create_en1994_viewer())
        .viewer_mutation_roster::<crate::viewer::en1994::En1994Viewer>()
        .editor::<crate::editor::en1995::En1995PlayApp>(crate::editor::en1995::create_en1995_app())
        .editor_mutation_roster::<crate::editor::en1995::En1995PlayApp>()
        .viewer::<crate::viewer::en1995::En1995Viewer>(crate::viewer::en1995::create_en1995_viewer())
        .viewer_mutation_roster::<crate::viewer::en1995::En1995Viewer>()
        .editor::<crate::editor::en1996::En1996PlayApp>(crate::editor::en1996::create_en1996_app())
        .editor_mutation_roster::<crate::editor::en1996::En1996PlayApp>()
        .viewer::<crate::viewer::en1996::En1996Viewer>(crate::viewer::en1996::create_en1996_viewer())
        .viewer_mutation_roster::<crate::viewer::en1996::En1996Viewer>()
        .editor::<crate::editor::en1997::En1997PlayApp>(crate::editor::en1997::create_en1997_app())
        .editor_mutation_roster::<crate::editor::en1997::En1997PlayApp>()
        .viewer::<crate::viewer::en1997::En1997Viewer>(crate::viewer::en1997::create_en1997_viewer())
        .viewer_mutation_roster::<crate::viewer::en1997::En1997Viewer>()
        .editor::<crate::editor::en1998::En1998PlayApp>(crate::editor::en1998::create_en1998_app())
        .editor_mutation_roster::<crate::editor::en1998::En1998PlayApp>()
        .viewer::<crate::viewer::en1998::En1998Viewer>(crate::viewer::en1998::create_en1998_viewer())
        .viewer_mutation_roster::<crate::viewer::en1998::En1998Viewer>()
        .editor::<crate::editor::en1999::En1999PlayApp>(crate::editor::en1999::create_en1999_app())
        .editor_mutation_roster::<crate::editor::en1999::En1999PlayApp>()
        .viewer::<crate::viewer::en1999::En1999Viewer>(crate::viewer::en1999::create_en1999_viewer())
        .viewer_mutation_roster::<crate::viewer::en1999::En1999Viewer>()
        .editor::<crate::editor::iso16757::Iso16757PlayApp>(crate::editor::iso16757::create_iso16757_app())
        .editor_mutation_roster::<crate::editor::iso16757::Iso16757PlayApp>()
        .viewer::<crate::viewer::iso16757::Iso16757Viewer>(crate::viewer::iso16757::create_iso16757_viewer())
        .viewer_mutation_roster::<crate::viewer::iso16757::Iso16757Viewer>()
        .editor::<crate::editor::vdi3805::Vdi3805PlayApp>(crate::editor::vdi3805::create_vdi3805_app())
        .editor_mutation_roster::<crate::editor::vdi3805::Vdi3805PlayApp>()
        .viewer::<crate::viewer::vdi3805::Vdi3805Viewer>(crate::viewer::vdi3805::create_vdi3805_viewer())
        .viewer_mutation_roster::<crate::viewer::vdi3805::Vdi3805Viewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::din4108::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::din16798::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::din18599::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::en1990::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::en1991::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::en1992::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::en1993::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::en1994::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::en1995::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::en1996::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::en1997::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::en1998::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::en1999::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::iso16757::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::vdi3805::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist norm family edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 🧪️ `assert_viewer_never_mutates`/`assert_editor_and_viewer_share_dialect` (contract §2.5) —
    //! local stand-ins per the pilot's `📓️w2-cad-report.md` "SDK gaps" #2: as of this packet's W0-F
    //! handoff, the canonical `semio_framework_plugin::testkit` versions exist
    //! (`👁️✏️SurfaceTestkit` region) and are used directly here rather than re-invented.
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    macro_rules! surface_law {
        ($name:ident, $editor:ty, $viewer:ty) => {
            #[test]
            fn $name() {
                assert_viewer_never_mutates::<$viewer>();
                assert_editor_and_viewer_share_dialect::<$editor, $viewer>();
            }
        };
    }

    surface_law!(din4108_surface_laws_hold, crate::editor::din4108::Din4108PlayApp, crate::viewer::din4108::Din4108Viewer);
    surface_law!(din16798_surface_laws_hold, crate::editor::din16798::Din16798PlayApp, crate::viewer::din16798::Din16798Viewer);
    surface_law!(din18599_surface_laws_hold, crate::editor::din18599::Din18599PlayApp, crate::viewer::din18599::Din18599Viewer);
    surface_law!(en1990_surface_laws_hold, crate::editor::en1990::En1990PlayApp, crate::viewer::en1990::En1990Viewer);
    surface_law!(en1991_surface_laws_hold, crate::editor::en1991::En1991PlayApp, crate::viewer::en1991::En1991Viewer);
    surface_law!(en1992_surface_laws_hold, crate::editor::en1992::En1992PlayApp, crate::viewer::en1992::En1992Viewer);
    surface_law!(en1993_surface_laws_hold, crate::editor::en1993::En1993PlayApp, crate::viewer::en1993::En1993Viewer);
    surface_law!(en1994_surface_laws_hold, crate::editor::en1994::En1994PlayApp, crate::viewer::en1994::En1994Viewer);
    surface_law!(en1995_surface_laws_hold, crate::editor::en1995::En1995PlayApp, crate::viewer::en1995::En1995Viewer);
    surface_law!(en1996_surface_laws_hold, crate::editor::en1996::En1996PlayApp, crate::viewer::en1996::En1996Viewer);
    surface_law!(en1997_surface_laws_hold, crate::editor::en1997::En1997PlayApp, crate::viewer::en1997::En1997Viewer);
    surface_law!(en1998_surface_laws_hold, crate::editor::en1998::En1998PlayApp, crate::viewer::en1998::En1998Viewer);
    surface_law!(en1999_surface_laws_hold, crate::editor::en1999::En1999PlayApp, crate::viewer::en1999::En1999Viewer);
    surface_law!(iso16757_surface_laws_hold, crate::editor::iso16757::Iso16757PlayApp, crate::viewer::iso16757::Iso16757Viewer);
    surface_law!(vdi3805_surface_laws_hold, crate::editor::vdi3805::Vdi3805PlayApp, crate::viewer::vdi3805::Vdi3805Viewer);
}
//#endregion 🧪️SurfaceTests
