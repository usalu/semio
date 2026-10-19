//! 🧭️ The keyboard cursor: the arrow keys move a cursor of the armed utility in the plan and the section by one fine step (`mod+arrow`, 0.1 m) or one coarse step (`mod+shift+arrow`, 1 m), and `mod+enter` clicks at the
//! cursor. The cursor starts where the pointer last was, else at the point the gesture hangs on, else at the centre of the view; every placement tool takes the click like a pointer click at the exact point, so a
//! drawing can be made without a pointer. Moving the cursor writes nothing: the tool shows its marks at the cursor in the window transient, as it does for a pointer move.

use crate::editor::bim::gestures::run_cursor;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};

/// 📏️ The metres of one fine step of the keyboard cursor.
pub const FINE_STEP: f64 = 0.1;
/// 📏️ The metres of one coarse step of the keyboard cursor.
pub const COARSE_STEP: f64 = 1.0;

/// 🧭️ A command of the keyboard cursor: a step of the cursor, or the click at it.
pub trait Cursor {
    const DELTA: Option<[f64; 2]>;
}

macro_rules! cursor {
    ($($payload:ident => $keyword:literal, $delta:expr;)+) => {
        $(
            #[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
            #[dsl(keyword = $keyword)]
            pub struct $payload {}

            impl Cursor for $payload {
                const DELTA: Option<[f64; 2]> = $delta;
            }
        )+
    };
}

cursor! {
    CursorLeft => "cursor-left", Some([-FINE_STEP, 0.0]);
    CursorRight => "cursor-right", Some([FINE_STEP, 0.0]);
    CursorUp => "cursor-up", Some([0.0, FINE_STEP]);
    CursorDown => "cursor-down", Some([0.0, -FINE_STEP]);
    CursorLeftFar => "cursor-left-far", Some([-COARSE_STEP, 0.0]);
    CursorRightFar => "cursor-right-far", Some([COARSE_STEP, 0.0]);
    CursorUpFar => "cursor-up-far", Some([0.0, COARSE_STEP]);
    CursorDownFar => "cursor-down-far", Some([0.0, -COARSE_STEP]);
    CursorPlace => "cursor-place", None;
}

pub fn handle<P: Cursor>(_payload: &P, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    run_cursor(ctx, doc, P::DELTA)
}


