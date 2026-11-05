//! 🎹️ Canonical shortcut and touch routes into view actions.

use crate::*;
use crate::accessibility::focus_order::{shortcut_action_id, ShortcutAction};

const STEP_ACTIONS: [(ShortcutAction, &str, f64); 6] = [
    (ShortcutAction::NextCheck, VIEW_ACTION_STEP_CHECK, 1.0),
    (ShortcutAction::PreviousCheck, VIEW_ACTION_STEP_CHECK, -1.0),
    (ShortcutAction::NextGoverningCheck, VIEW_ACTION_STEP_GOVERNING, 1.0),
    (ShortcutAction::PreviousGoverningCheck, VIEW_ACTION_STEP_GOVERNING, -1.0),
    (ShortcutAction::NextCombination, VIEW_ACTION_STEP_COMBINATION, 1.0),
    (ShortcutAction::PreviousCombination, VIEW_ACTION_STEP_COMBINATION, -1.0),
];

/// 🎹️ The `viewportAct` action and direction (`1` forward, `-1` backward) of a shortcut action id (`norm.shortcut.next-check`); `None` for every other action.
pub fn shortcut_view_act(action_id: &str) -> Option<(&'static str, f64)> {
    STEP_ACTIONS.iter().find(|(action, _, _)| shortcut_action_id(action) == action_id).map(|(_, act, direction)| (*act, *direction))
}

/// 🖐️ The `viewportAct` action and number of a touch action id (`norm.touch.menu-open` opens the Edit menu, `norm.touch.menu-close` closes it, `norm.touch.frame-selection` toggles the framed camera) and of every stepping shortcut action id; `None` for every other action. The command bridge resolves its `other` ids through this one table.
pub fn view_act_of(action_id: &str) -> Option<(&'static str, f64)> {
    shortcut_view_act(action_id).or(match action_id {
        TOUCH_MENU_OPEN => Some((VIEW_ACTION_MENU, 1.0)),
        TOUCH_MENU_CLOSE => Some((VIEW_ACTION_MENU, 0.0)),
        TOUCH_FRAME_SELECTION => Some((VIEW_ACTION_FRAME, 1.0)),
        _ => None,
    })
}

