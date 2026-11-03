//! ⌨️ Keyboard equivalents of every pointer gesture: Tab cycles handles, arrows nudge, digits type a value, Enter commits, Escape aborts. See [`super`].

use super::{DragEvent, Modifiers, AbortReason};

/// ⌨️ A key press reduced to the vocabulary the editor cares about.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Key {
    Tab,
    Enter,
    Escape,
    Backspace,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Char(char),
}

/// 🧭️ What a key means for handle manipulation.
#[derive(Clone, Debug, PartialEq)]
pub enum KeyIntent {
    Cycle(i32),
    Drag(DragEvent),
    Ignored,
}

/// ⌨️ Maps one key press to an intent; Shift+Tab cycles backwards, Shift multiplies and Alt divides a nudge, Up/Right increase and Down/Left decrease.
pub fn key_intent(key: &Key, modifiers: Modifiers) -> KeyIntent {
    let nudge = |direction: f64| KeyIntent::Drag(DragEvent::Nudge { direction, modifiers });
    match key {
        Key::Tab => KeyIntent::Cycle(if modifiers.shift { -1 } else { 1 }),
        Key::ArrowRight | Key::ArrowUp => nudge(1.0),
        Key::ArrowLeft | Key::ArrowDown => nudge(-1.0),
        Key::Enter => KeyIntent::Drag(DragEvent::Commit),
        Key::Escape => KeyIntent::Drag(DragEvent::Abort(AbortReason::Escape)),
        Key::Backspace => KeyIntent::Drag(DragEvent::EntryBackspace),
        Key::Char(c) if c.is_ascii_digit() || matches!(c, '-' | '.' | ',') => KeyIntent::Drag(DragEvent::Entry(*c)),
        Key::Char(_) => KeyIntent::Ignored,
    }
}

/// 🔄️ The next focus in the cyclic Tab order; `None` focus starts at the first (forward) or last (backward) handle.
pub fn cycle_focus(ids: &[String], current: Option<&str>, step: i32) -> Option<String> {
    let len = ids.len() as i32;
    (len > 0).then(|| {
        let position = current.and_then(|id| ids.iter().position(|candidate| candidate == id)).map(|p| p as i32);
        let next = match position {
            Some(p) => (p + step).rem_euclid(len),
            None if step < 0 => len - 1,
            None => 0,
        };
        ids[next as usize].clone()
    })
}

/// 📣️ The announcement for a focused handle: its accessible name plus the current displayed value, spoken by the live region.
pub fn focus_announcement(name: &str, value: &str, unit: &str) -> String {
    format!("{name}: {value} {unit}")
}
