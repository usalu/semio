//! ⌨️ Shortcut dispatch: turns a scoped chord of the accessibility shortcut map into what the editor does, so handle nudging by keyboard runs through the same key vocabulary and drag statechart as the pointer.
//!
//! 🔗️ Map: [`shortcuts`](crate::accessibility::focus_order::shortcuts); handle keys: [`key_intent`](crate::direct_manipulation::key_intent); see `visual-editor-architecture.md` §6.3 and §8.2.

use crate::accessibility::focus_order::{lookup, Chord, NudgeStep, Scope, ShortcutAction};
use crate::direct_manipulation::{Key, Modifiers};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

/// 🧭️ What a chord asks for: a key for the focused handle's drag statechart, or an editor-level action the host performs.
#[derive(Clone, Debug, PartialEq)]
pub enum KeyCommand {
    Handle { key: Key, modifiers: Modifiers },
    Host(ShortcutAction),
}

/// ✖️ The modifiers that give a nudge its step size: Alt fine, none normal, Shift large.
pub fn step_modifiers(step: NudgeStep) -> Modifiers {
    match step {
        NudgeStep::Fine => Modifiers::ALT,
        NudgeStep::Normal => Modifiers::NONE,
        NudgeStep::Large => Modifiers::SHIFT,
    }
}

/// 🎯️ The command of a chord in a scope; `None` when the chord is unbound there. Arrow nudges and the handle walk become handle keys, everything else is left to the host.
pub fn dispatch_shortcut(scope: Scope, chord: Chord) -> Option<KeyCommand> {
    Some(match lookup(scope, chord)? {
        ShortcutAction::NudgeHandle { dx, dy, step } => {
            let key = if dx > 0 { Key::ArrowRight } else if dx < 0 { Key::ArrowLeft } else if dy > 0 { Key::ArrowUp } else { Key::ArrowDown };
            KeyCommand::Handle { key, modifiers: step_modifiers(step) }
        }
        ShortcutAction::CycleHandle { forward } => KeyCommand::Handle { key: Key::Tab, modifiers: Modifiers { shift: !forward, ..Modifiers::NONE } },
        ShortcutAction::ConfirmHandle => KeyCommand::Handle { key: Key::Enter, modifiers: Modifiers::NONE },
        ShortcutAction::CancelHandle => KeyCommand::Handle { key: Key::Escape, modifiers: Modifiers::NONE },
        action => KeyCommand::Host(action),
    })
}
