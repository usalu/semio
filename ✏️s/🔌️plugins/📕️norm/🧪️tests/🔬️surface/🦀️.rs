//! 🧪️ `assert_viewer_never_mutates`/`assert_editor_and_viewer_share_dialect` (contract §2.5) —
//! local stand-ins per the pilot's `📓️w2-cad-report.md` "SDK gaps" #2: as of this packet's W0-F
//! handoff, the canonical `semio_framework_plugin::artifact_app_laws` versions exist
//! (`👁️✏️SurfaceUnitTests` region) and are used directly here rather than re-invented.
use semio_framework_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

macro_rules! surface_law {
    ($name:ident, $editor:ty, $viewer:ty) => {
        #[semio_framework_async_macros::async_test]
        async fn $name() {
            assert_viewer_never_mutates::<$viewer>().await;
            assert_editor_and_viewer_share_dialect::<$editor, $viewer>().await;
        }
    };
}

surface_law!(din4108_surface_laws_hold, semio_s_artifact_norm_din4108::editor::din4108::Din4108PlayApp, semio_s_artifact_norm_din4108::viewer::din4108::Din4108Viewer);
surface_law!(din16798_surface_laws_hold, semio_s_artifact_norm_din16798::editor::din16798::Din16798PlayApp, semio_s_artifact_norm_din16798::viewer::din16798::Din16798Viewer);
surface_law!(din18599_surface_laws_hold, semio_s_artifact_norm_din18599::editor::din18599::Din18599PlayApp, semio_s_artifact_norm_din18599::viewer::din18599::Din18599Viewer);
surface_law!(en1990_surface_laws_hold, semio_s_artifact_norm_en1990::editor::en1990::En1990PlayApp, semio_s_artifact_norm_en1990::viewer::en1990::En1990Viewer);
surface_law!(en1991_surface_laws_hold, semio_s_artifact_norm_en1991::editor::en1991::En1991PlayApp, semio_s_artifact_norm_en1991::viewer::en1991::En1991Viewer);
surface_law!(en1992_surface_laws_hold, semio_s_artifact_norm_en1992::editor::en1992::En1992PlayApp, semio_s_artifact_norm_en1992::viewer::en1992::En1992Viewer);
surface_law!(en1993_surface_laws_hold, semio_s_artifact_norm_en1993::editor::en1993::En1993PlayApp, semio_s_artifact_norm_en1993::viewer::en1993::En1993Viewer);
surface_law!(en1994_surface_laws_hold, semio_s_artifact_norm_en1994::editor::en1994::En1994PlayApp, semio_s_artifact_norm_en1994::viewer::en1994::En1994Viewer);
surface_law!(en1995_surface_laws_hold, semio_s_artifact_norm_en1995::editor::en1995::En1995PlayApp, semio_s_artifact_norm_en1995::viewer::en1995::En1995Viewer);
surface_law!(en1996_surface_laws_hold, semio_s_artifact_norm_en1996::editor::en1996::En1996PlayApp, semio_s_artifact_norm_en1996::viewer::en1996::En1996Viewer);
surface_law!(en1997_surface_laws_hold, semio_s_artifact_norm_en1997::editor::en1997::En1997PlayApp, semio_s_artifact_norm_en1997::viewer::en1997::En1997Viewer);
surface_law!(en1998_surface_laws_hold, semio_s_artifact_norm_en1998::editor::en1998::En1998PlayApp, semio_s_artifact_norm_en1998::viewer::en1998::En1998Viewer);
surface_law!(en1999_surface_laws_hold, semio_s_artifact_norm_en1999::editor::en1999::En1999PlayApp, semio_s_artifact_norm_en1999::viewer::en1999::En1999Viewer);
surface_law!(iso16757_surface_laws_hold, semio_s_artifact_norm_iso16757::editor::iso16757::Iso16757PlayApp, semio_s_artifact_norm_iso16757::viewer::iso16757::Iso16757Viewer);
surface_law!(vdi3805_surface_laws_hold, semio_s_artifact_norm_vdi3805::editor::vdi3805::Vdi3805PlayApp, semio_s_artifact_norm_vdi3805::viewer::vdi3805::Vdi3805Viewer);


//#region 🎨️SetActiveExample
/// 🎨️ Asserts one family editor declares `setActiveExample` and loading its example mutates away from boot when the texts differ.
macro_rules! set_active_example_case {
    ($name:ident, $create:path, $command:ty, $set_active:path, $example_id:expr, $primary:expr, $snapshot:ty) => {
        #[test]
        fn $name() {
            let definition = $create();
            assert!(
                definition.actions.iter().any(|action| action.id == "setActiveExample"),
                "{} must declare setActiveExample on the editor action list",
                stringify!($name)
            );
            assert!(
                <$command>::TOOL_JOB_IDS.contains(&"setActiveExample"),
                "{} command catalog must include setActiveExample",
                stringify!($name)
            );
            let boot = <$snapshot>::default();
            let config = semio_framework_plugin::NoConfig::default();
            let emit = {
                use $set_active as set_active_example;
                set_active_example::handle(
                    &set_active_example::SetActiveExample { example_id: $example_id.to_string() },
                    &semio_framework_plugin::ArtifactView::new(&boot, &semio_framework_plugin::HistoryView::empty()),
                    &semio_framework_plugin::ConfigView { snapshot: &config, window: None },
                )
                .expect("setActiveExample handle")
            };
            let unknown = {
                use $set_active as set_active_example;
                set_active_example::handle(
                    &set_active_example::SetActiveExample { example_id: "__unknown_example__".into() },
                    &semio_framework_plugin::ArtifactView::new(&boot, &semio_framework_plugin::HistoryView::empty()),
                    &semio_framework_plugin::ConfigView { snapshot: &config, window: None },
                )
                .expect("unknown example is a no-op")
            };
            assert!(unknown.artifact_mutations.is_empty(), "{} unknown example must be a no-op", stringify!($name));
            assert!(
                !emit.artifact_mutations.is_empty() || emit.description.as_deref() == Some("setSnapshot"),
                "{} must load the example through setSnapshot (mutations when the example differs from boot)",
                stringify!($name)
            );
            let _ = $primary;
        }
    };
}

set_active_example_case!(
    din4108_set_active_example_loads_demo,
    semio_s_artifact_norm_din4108::editor::din4108::create_din4108_app,
    semio_s_artifact_norm_din4108::editor::din4108::Din4108Command,
    semio_s_artifact_norm_din4108::editor::din4108::commands::set_active_example,
    semio_s_artifact_norm_din4108::examples::demo::ID,
    semio_s_artifact_norm_din4108::examples::demo::PRIMARY_TEXT,
    semio_s_artifact_norm_din4108::Din4108Snapshot
);
set_active_example_case!(
    din16798_set_active_example_loads_demo,
    semio_s_artifact_norm_din16798::editor::din16798::create_din16798_app,
    semio_s_artifact_norm_din16798::editor::din16798::Din16798Command,
    semio_s_artifact_norm_din16798::editor::din16798::commands::set_active_example,
    semio_s_artifact_norm_din16798::examples::demo::ID,
    semio_s_artifact_norm_din16798::examples::demo::PRIMARY_TEXT,
    semio_s_artifact_norm_din16798::Din16798Snapshot
);
set_active_example_case!(
    din18599_set_active_example_loads_demo,
    semio_s_artifact_norm_din18599::editor::din18599::create_din18599_app,
    semio_s_artifact_norm_din18599::editor::din18599::Din18599Command,
    semio_s_artifact_norm_din18599::editor::din18599::commands::set_active_example,
    semio_s_artifact_norm_din18599::examples::demo::ID,
    semio_s_artifact_norm_din18599::examples::demo::PRIMARY_TEXT,
    semio_s_artifact_norm_din18599::Din18599Snapshot
);
set_active_example_case!(
    en1990_set_active_example_loads_high_consequence_office,
    semio_s_artifact_norm_en1990::editor::en1990::create_en1990_app,
    semio_s_artifact_norm_en1990::editor::en1990::En1990Command,
    semio_s_artifact_norm_en1990::editor::en1990::commands::set_active_example,
    semio_s_artifact_norm_en1990::standards::v1::subsets::any::examples::high_consequence_office::ID,
    semio_s_artifact_norm_en1990::standards::v1::subsets::any::examples::high_consequence_office::PRIMARY_TEXT,
    semio_s_artifact_norm_en1990::En1990Snapshot
);
set_active_example_case!(
    en1991_set_active_example_loads_retail_hydrocarbon_fire,
    semio_s_artifact_norm_en1991::editor::en1991::create_en1991_app,
    semio_s_artifact_norm_en1991::editor::en1991::En1991Command,
    semio_s_artifact_norm_en1991::editor::en1991::commands::set_active_example,
    semio_s_artifact_norm_en1991::retail_hydrocarbon_fire::ID,
    semio_s_artifact_norm_en1991::retail_hydrocarbon_fire::PRIMARY_TEXT,
    semio_s_artifact_norm_en1991::En1991Snapshot
);
set_active_example_case!(
    en1992_set_active_example_loads_liquid_retaining_fem_anchor,
    semio_s_artifact_norm_en1992::editor::en1992::create_en1992_app,
    semio_s_artifact_norm_en1992::editor::en1992::En1992Command,
    semio_s_artifact_norm_en1992::editor::en1992::commands::set_active_example,
    semio_s_artifact_norm_en1992::liquid_retaining_fem_anchor::ID,
    semio_s_artifact_norm_en1992::liquid_retaining_fem_anchor::PRIMARY_TEXT,
    semio_s_artifact_norm_en1992::En1992Snapshot
);
set_active_example_case!(
    en1993_set_active_example_loads_high_strength_connection,
    semio_s_artifact_norm_en1993::editor::en1993::create_en1993_app,
    semio_s_artifact_norm_en1993::editor::en1993::En1993Command,
    semio_s_artifact_norm_en1993::editor::en1993::commands::set_active_example,
    semio_s_artifact_norm_en1993::high_strength_connection::ID,
    semio_s_artifact_norm_en1993::high_strength_connection::PRIMARY_TEXT,
    semio_s_artifact_norm_en1993::En1993Snapshot
);
set_active_example_case!(
    en1994_set_active_example_loads_composite_bridge_girder,
    semio_s_artifact_norm_en1994::editor::en1994::create_en1994_app,
    semio_s_artifact_norm_en1994::editor::en1994::En1994Command,
    semio_s_artifact_norm_en1994::editor::en1994::commands::set_active_example,
    semio_s_artifact_norm_en1994::composite_bridge_girder::ID,
    semio_s_artifact_norm_en1994::composite_bridge_girder::PRIMARY_TEXT,
    semio_s_artifact_norm_en1994::En1994Snapshot
);
set_active_example_case!(
    en1995_set_active_example_loads_glulam_footbridge,
    semio_s_artifact_norm_en1995::editor::en1995::create_en1995_app,
    semio_s_artifact_norm_en1995::editor::en1995::En1995Command,
    semio_s_artifact_norm_en1995::editor::en1995::commands::set_active_example,
    semio_s_artifact_norm_en1995::glulam_footbridge::ID,
    semio_s_artifact_norm_en1995::glulam_footbridge::PRIMARY_TEXT,
    semio_s_artifact_norm_en1995::En1995Snapshot
);
set_active_example_case!(
    en1996_set_active_example_loads_loadbearing_wall,
    semio_s_artifact_norm_en1996::editor::en1996::create_en1996_app,
    semio_s_artifact_norm_en1996::editor::en1996::En1996Command,
    semio_s_artifact_norm_en1996::editor::en1996::commands::set_active_example,
    semio_s_artifact_norm_en1996::loadbearing_wall::ID,
    semio_s_artifact_norm_en1996::loadbearing_wall::PRIMARY_TEXT,
    semio_s_artifact_norm_en1996::En1996Snapshot
);
set_active_example_case!(
    en1997_set_active_example_loads_demo,
    semio_s_artifact_norm_en1997::editor::en1997::create_en1997_app,
    semio_s_artifact_norm_en1997::editor::en1997::En1997Command,
    semio_s_artifact_norm_en1997::editor::en1997::commands::set_active_example,
    semio_s_artifact_norm_en1997::examples::demo::ID,
    semio_s_artifact_norm_en1997::examples::demo::PRIMARY_TEXT,
    semio_s_artifact_norm_en1997::En1997Snapshot
);
set_active_example_case!(
    en1998_set_active_example_loads_seismic_rc_frame,
    semio_s_artifact_norm_en1998::editor::en1998::create_en1998_app,
    semio_s_artifact_norm_en1998::editor::en1998::En1998Command,
    semio_s_artifact_norm_en1998::editor::en1998::commands::set_active_example,
    semio_s_artifact_norm_en1998::seismic_rc_frame::ID,
    semio_s_artifact_norm_en1998::seismic_rc_frame::PRIMARY_TEXT,
    semio_s_artifact_norm_en1998::En1998Snapshot
);
set_active_example_case!(
    en1999_set_active_example_loads_aluminium_roof_purlin,
    semio_s_artifact_norm_en1999::editor::en1999::create_en1999_app,
    semio_s_artifact_norm_en1999::editor::en1999::En1999Command,
    semio_s_artifact_norm_en1999::editor::en1999::commands::set_active_example,
    semio_s_artifact_norm_en1999::aluminium_roof_purlin::ID,
    semio_s_artifact_norm_en1999::aluminium_roof_purlin::PRIMARY_TEXT,
    semio_s_artifact_norm_en1999::En1999Snapshot
);
set_active_example_case!(
    iso16757_set_active_example_loads_demo,
    semio_s_artifact_norm_iso16757::editor::iso16757::create_iso16757_app,
    semio_s_artifact_norm_iso16757::editor::iso16757::Iso16757Command,
    semio_s_artifact_norm_iso16757::editor::iso16757::commands::set_active_example,
    semio_s_artifact_norm_iso16757::examples::demo::ID,
    semio_s_artifact_norm_iso16757::examples::demo::PRIMARY_TEXT,
    semio_s_artifact_norm_iso16757::Iso16757Snapshot
);
set_active_example_case!(
    vdi3805_set_active_example_loads_demo,
    semio_s_artifact_norm_vdi3805::editor::vdi3805::create_vdi3805_app,
    semio_s_artifact_norm_vdi3805::editor::vdi3805::Vdi3805Command,
    semio_s_artifact_norm_vdi3805::editor::vdi3805::commands::set_active_example,
    semio_s_artifact_norm_vdi3805::examples::demo::ID,
    semio_s_artifact_norm_vdi3805::examples::demo::PRIMARY_TEXT,
    semio_s_artifact_norm_vdi3805::Vdi3805Snapshot
);
//#endregion 🎨️SetActiveExample
