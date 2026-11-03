//! 🧪️ Shortcut dispatch tests; scenarios in `🥒️.feature`. The independent oracle is the handle drag tool's own key vocabulary: every nudge chord must mean what `key_intent` makes of its key.

use super::*;
use crate::accessibility::focus_order::shortcuts;
use crate::direct_manipulation::{key_intent, DragEvent, KeyIntent};

#[test]
fn every_nudge_chord_means_the_same_to_the_handle_drag_tool() {
    let nudges: Vec<_> = shortcuts().into_iter().filter_map(|s| match s.action { ShortcutAction::NudgeHandle { dx, dy, step } => Some((s.chord, dx, dy, step)), _ => None }).collect();
    assert_eq!(nudges.len(), 12);
    for (chord, dx, dy, step) in nudges {
        let Some(KeyCommand::Handle { key, modifiers }) = dispatch_shortcut(Scope::Handle, chord) else { panic!("{chord:?} is a handle key") };
        let KeyIntent::Drag(DragEvent::Nudge { direction, modifiers: seen }) = key_intent(&key, modifiers) else { panic!("{key:?} nudges") };
        assert_eq!((direction, seen), (f64::from(dx + dy).signum(), modifiers), "{chord:?}");
        assert_eq!(modifiers.scale(), step.scale(), "{chord:?}");
    }
}

#[test]
fn the_handle_walk_and_editor_actions_dispatch_by_scope() {
    let tab = |shift| Chord { key: "Tab", ctrl: false, alt: false, shift };
    for (shift, step) in [(false, 1), (true, -1)] {
        let Some(KeyCommand::Handle { key, modifiers }) = dispatch_shortcut(Scope::Handle, tab(shift)) else { panic!("tab") };
        assert_eq!(key_intent(&key, modifiers), KeyIntent::Cycle(step));
    }
    assert_eq!(dispatch_shortcut(Scope::Viewport, tab(false)), None);
    let undo = Chord { key: "z", ctrl: true, alt: false, shift: false };
    for scope in [Scope::Global, Scope::Viewport, Scope::Handle] {
        assert_eq!(dispatch_shortcut(scope, undo), Some(KeyCommand::Host(ShortcutAction::Undo)));
    }
    let left = Chord { key: "ArrowLeft", ctrl: false, alt: false, shift: false };
    assert_eq!(dispatch_shortcut(Scope::Viewport, left), Some(KeyCommand::Host(ShortcutAction::OrbitLeft)));
    assert_eq!(dispatch_shortcut(Scope::Handle, left), Some(KeyCommand::Handle { key: Key::ArrowLeft, modifiers: Modifiers::NONE }));
}

#[test]
fn enter_and_escape_on_a_focused_grip_open_apply_and_cancel_the_value_field_like_the_drag_tool_means_them() {
    let plain = |key| Chord { key, ctrl: false, alt: false, shift: false };
    let Some(KeyCommand::Handle { key, modifiers }) = dispatch_shortcut(Scope::Handle, plain("Enter")) else { panic!("enter") };
    assert_eq!(key_intent(&key, modifiers), KeyIntent::Drag(DragEvent::Commit));
    let Some(KeyCommand::Handle { key, modifiers }) = dispatch_shortcut(Scope::Handle, plain("Escape")) else { panic!("escape") };
    assert_eq!(key_intent(&key, modifiers), KeyIntent::Drag(DragEvent::Abort(crate::direct_manipulation::AbortReason::Escape)));
    assert_eq!(dispatch_shortcut(Scope::Viewport, plain("Enter")), Some(KeyCommand::Host(ShortcutAction::EnterHandles)));
    assert_eq!(dispatch_shortcut(Scope::Viewport, plain("Escape")), Some(KeyCommand::Host(ShortcutAction::ClearSelection)));
}

#[test]
fn ctrl_arrow_dispatches_the_entity_cycle_to_the_host_while_plain_arrows_stay_handle_nudges() {
    let ctrl = |key| Chord { key, ctrl: true, alt: false, shift: false };
    assert_eq!(dispatch_shortcut(Scope::Handle, ctrl("ArrowRight")), Some(KeyCommand::Host(ShortcutAction::CycleGripInEntity { forward: true })));
    assert_eq!(dispatch_shortcut(Scope::Handle, ctrl("ArrowUp")), Some(KeyCommand::Host(ShortcutAction::CycleGripInEntity { forward: false })));
    let plain = Chord { key: "ArrowRight", ctrl: false, alt: false, shift: false };
    assert_eq!(dispatch_shortcut(Scope::Handle, plain), Some(KeyCommand::Handle { key: Key::ArrowRight, modifiers: Modifiers::NONE }));
}
