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
