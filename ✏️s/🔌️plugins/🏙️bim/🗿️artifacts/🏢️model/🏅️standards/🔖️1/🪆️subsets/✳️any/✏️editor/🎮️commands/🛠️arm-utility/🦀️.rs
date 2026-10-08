//! 🛠️ The utility hotkeys: one command per tool, each arming its utility in the addressed window (`Effect::SetActiveUtility`). A utility is host-owned window state, never a document
//! operation, so these commands write nothing; the keybinding table binds a letter to each (W wall, C column, B beam, S slab, R roof, N window, D door, T stair, L railing,
//! P space, G grid, M measure, V select). Arming a utility drops the gesture another utility had in progress.

use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, NoConfig, NoConfigMutation};

/// 🛠️ A command that arms one utility.
pub trait Armed {
    const UTILITY: &'static str;
}

macro_rules! armed {
    ($($payload:ident => $keyword:literal, $utility:literal;)+) => {
        $(
            #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
            #[dsl(keyword = $keyword)]
            pub struct $payload {}

            impl Armed for $payload {
                const UTILITY: &'static str = $utility;
            }
        )+

        /// 🛠️ The utility every arming command arms, in table order.
        pub const ARMED_UTILITIES: &[&str] = &[$($utility),+];
    };
}

armed! {
    ArmSelect => "arm-select", "select";
    ArmWall => "arm-wall", "wall";
    ArmWallArc => "arm-wall-arc", "wall-arc";
    ArmCurtainWall => "arm-curtain-wall", "curtain-wall";
    ArmColumn => "arm-column", "column";
    ArmBeam => "arm-beam", "beam";
    ArmSlab => "arm-slab", "slab";
    ArmRoof => "arm-roof", "roof";
    ArmWindow => "arm-window", "window";
    ArmDoor => "arm-door", "door";
    ArmOpening => "arm-opening", "opening";
    ArmStair => "arm-stair", "stair";
    ArmRailing => "arm-railing", "railing";
    ArmSpace => "arm-space", "space";
    ArmGrid => "arm-grid", "grid";
    ArmMeasure => "arm-measure", "measure";
}

pub fn handle<P: Armed>(_payload: &P, _doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let mut emit = Emit::default();
    if let Some(window) = ctx.view.as_ref().and_then(|view| view.window_id.clone()) {
        emit.effects.push(Effect::SetActiveUtility { window_id: window, utility_id: P::UTILITY.to_string() });
    }
    Ok(emit)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
