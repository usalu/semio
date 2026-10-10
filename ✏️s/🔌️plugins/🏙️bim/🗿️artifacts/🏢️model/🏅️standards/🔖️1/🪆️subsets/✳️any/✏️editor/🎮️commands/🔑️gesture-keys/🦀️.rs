//! 🔑️ The keys of the component and MEP route tools: turn the ghost by 15 degrees either way or by a quarter turn, mirror it, change the family (or the kind of section), raise or lower the elevation, change the system.
//! Each key is offered to the gesture of the addressed window; a window whose tool has no use for the key (any other tool) leaves it alone and writes nothing. The keys write no mutation themselves: they change what the
//! tool places with the next click and show it in the window transient.

use crate::editor::bim::gestures::run_key;
use crate::editor::bim::gestures::session::GestureKey;
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};

/// 📐️ The turn of one rotate key, 15 degrees in radians.
pub const TURN: f64 = std::f64::consts::PI / 12.0;
/// 📐️ The turn of the quarter-turn key in radians.
pub const QUARTER: f64 = std::f64::consts::FRAC_PI_2;

/// 🔑️ A command that is one key of a gesture.
pub trait Keyed {
    const KEY: GestureKey;
}

macro_rules! keyed {
    ($($payload:ident => $keyword:literal, $key:expr;)+) => {
        $(
            #[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
            #[dsl(keyword = $keyword)]
            pub struct $payload {}

            impl Keyed for $payload {
                const KEY: GestureKey = $key;
            }
        )+
    };
}

keyed! {
    GestureTurn => "gesture-turn", GestureKey::Turn(TURN);
    GestureTurnBack => "gesture-turn-back", GestureKey::Turn(-TURN);
    GestureQuarter => "gesture-quarter", GestureKey::Turn(QUARTER);
    GestureMirror => "gesture-mirror", GestureKey::Mirror;
    GestureNext => "gesture-next", GestureKey::Next;
    GesturePrevious => "gesture-previous", GestureKey::Previous;
    GestureRaise => "gesture-raise", GestureKey::Raise;
    GestureLower => "gesture-lower", GestureKey::Lower;
    GestureSystem => "gesture-system", GestureKey::System;
}

/// ⌨️ The keys of the commands, for the keybinding table: Alt+R and Alt+Shift+R turn, Ctrl+Alt+R is the quarter turn, Alt+M mirrors, Tab and Shift+Tab change the family or the section, Alt+PageUp and Alt+PageDown change the
/// elevation, Alt+T changes the system.
pub const KEYBINDINGS: &[(&str, &str)] = &[("alt+r", "gestureTurn"), ("alt+shift+r", "gestureTurnBack"), ("mod+alt+r", "gestureQuarter"), ("alt+m", "gestureMirror"), ("tab", "gestureNext"), ("shift+tab", "gesturePrevious"), ("alt+pageup", "gestureRaise"), ("alt+pagedown", "gestureLower"), ("alt+t", "gestureSystem")];

pub fn handle<P: Keyed>(_payload: &P, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    Ok(run_key(ctx, doc, P::KEY)?.unwrap_or_default())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
