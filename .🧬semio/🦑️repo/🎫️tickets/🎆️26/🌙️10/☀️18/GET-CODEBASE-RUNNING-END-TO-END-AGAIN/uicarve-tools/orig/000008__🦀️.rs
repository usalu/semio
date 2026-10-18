pub use crate::tui::event::KeypadKey;
use crate::tui::event::{mods, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::Pos;

/// 🐭️ Which pointer events the child asked to receive.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MouseReporting {
    #[default]
    Off,
    Click,
    Press,
    Drag,
    Motion,
}

/// 🧮️ How the child wants pointer reports spelled.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MouseEncoding {
    #[default]
    X10,
    Utf8,
    Sgr,
    Urxvt,
}

/// 🎚️ The child's modes that change which bytes an input event becomes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputModes {
    pub app_cursor: bool,
    pub app_keypad: bool,
    pub newline: bool,
    pub bracketed_paste: bool,
    pub mouse: MouseReporting,
    pub mouse_encoding: MouseEncoding,
    pub focus_reporting: bool,
    pub alt_screen: bool,
    pub alt_scroll: bool,
}

impl Default for InputModes {
    fn default() -> Self {
        Self { app_cursor: false, app_keypad: false, newline: false, bracketed_paste: false, mouse: MouseReporting::Off, mouse_encoding: MouseEncoding::X10, focus_reporting: false, alt_screen: false, alt_scroll: true }
    }
}

const MODIFIERS: u8 = mods::SHIFT | mods::ALT | mods::CTRL;

fn csi(body: &str) -> Vec<u8> {
    let mut out = vec![0x1b, b'['];
    out.extend_from_slice(body.as_bytes());
    out
}

fn ss3(final_byte: u8) -> Vec<u8> {
    vec![0x1b, b'O', final_byte]
}

fn modifier_parameter(modifiers: u8) -> u8 {
    1 + (modifiers & MODIFIERS)
}

fn cursor_key(final_byte: u8, modifiers: u8, application: bool) -> Vec<u8> {
    if modifiers & MODIFIERS != 0 {
        csi(&format!("1;{}{}", modifier_parameter(modifiers), final_byte as char))
    } else if application {
        ss3(final_byte)
    } else {
        csi(&(final_byte as char).to_string())
    }
}

fn tilde_key(code: u8, modifiers: u8) -> Vec<u8> {
    if modifiers & MODIFIERS != 0 {
        csi(&format!("{code};{}~", modifier_parameter(modifiers)))
    } else {
        csi(&format!("{code}~"))
    }
}

fn function_key(n: u8, modifiers: u8) -> Option<Vec<u8>> {
    if !(1..=63).contains(&n) {
        return None;
    }
    let group_modifiers = [0, mods::SHIFT, mods::CTRL, mods::CTRL | mods::SHIFT, mods::ALT, mods::ALT | mods::SHIFT][usize::from((n - 1) / 12)];
    let m = modifiers | group_modifiers;
    match (n - 1) % 12 + 1 {
        base @ 1..=4 => {
            let final_byte = b'P' + (base - 1);
            Some(if m & MODIFIERS != 0 { csi(&format!("1;{}{}", modifier_parameter(m), final_byte as char)) } else { ss3(final_byte) })
        }
        5 => Some(tilde_key(15, m)),
        base @ 6..=10 => Some(tilde_key([17, 18, 19, 20, 21][usize::from(base - 6)], m)),
        base => Some(tilde_key(23 + (base - 11), m)),
    }
}

fn control_byte(c: char) -> Option<u8> {
    match c {
        'a'..='z' => Some(c as u8 & 0x1f),
        'A'..='Z' => Some(c.to_ascii_lowercase() as u8 & 0x1f),
        ' ' | '@' | '2' => Some(0x00),
        '[' | '3' => Some(0x1b),
        '\\' | '4' => Some(0x1c),
        ']' | '5' => Some(0x1d),
        '^' | '6' => Some(0x1e),
        '_' | '/' | '7' => Some(0x1f),
        '?' | '8' => Some(0x7f),
        _ => None,
    }
}

fn with_alt(modifiers: u8, mut bytes: Vec<u8>) -> Vec<u8> {
    if modifiers & mods::ALT != 0 {
        bytes.insert(0, 0x1b);
    }
    bytes
}

/// ⌨️ The bytes the child reads for `event` under `modes`; `None` when the key has no spelling.
pub fn encode_key(event: &KeyEvent, modes: &InputModes) -> Option<Vec<u8>> {
    let m = event.mods & MODIFIERS;
    let ctrl = m & mods::CTRL != 0;
    let shift = m & mods::SHIFT != 0;
    match event.key {
        Key::Char(c) => {
            let bytes = if ctrl { control_byte(c).map_or_else(|| c.to_string().into_bytes(), |b| vec![b]) } else { c.to_string().into_bytes() };
            Some(with_alt(m, bytes))
        }
        Key::Enter => Some(with_alt(m, if modes.newline { b"\r\n".to_vec() } else { vec![b'\r'] })),
        Key::Tab if shift => Some(with_alt(m, csi("Z"))),
        Key::Tab => Some(with_alt(m, vec![b'\t'])),
        Key::BackTab => Some(with_alt(m, csi("Z"))),
        Key::Backspace => Some(with_alt(m, vec![if ctrl { 0x08 } else { 0x7f }])),
        Key::Esc => Some(with_alt(m, vec![0x1b])),
        Key::Up => Some(cursor_key(b'A', m, modes.app_cursor)),
        Key::Down => Some(cursor_key(b'B', m, modes.app_cursor)),
        Key::Right => Some(cursor_key(b'C', m, modes.app_cursor)),
        Key::Left => Some(cursor_key(b'D', m, modes.app_cursor)),
        Key::Home => Some(cursor_key(b'H', m, modes.app_cursor)),
        Key::End => Some(cursor_key(b'F', m, modes.app_cursor)),
        Key::Insert => Some(tilde_key(2, m)),
        Key::Delete => Some(tilde_key(3, m)),
        Key::PageUp => Some(tilde_key(5, m)),
        Key::PageDown => Some(tilde_key(6, m)),
        Key::F(n) => function_key(n, m),
        Key::Keypad(key) => Some(encode_keypad(key, m, modes)),
    }
}

/// 🕹️ The bytes for a numeric keypad key: the plain character, or an `SS3` code in application keypad mode.
pub fn encode_keypad(key: KeypadKey, modifiers: u8, modes: &InputModes) -> Vec<u8> {
    let (plain, application): (u8, u8) = match key {
        KeypadKey::Digit(n) => (b'0' + n.min(9), b'p' + n.min(9)),
        KeypadKey::Decimal => (b'.', b'n'),
        KeypadKey::Plus => (b'+', b'k'),
        KeypadKey::Minus => (b'-', b'm'),
        KeypadKey::Multiply => (b'*', b'j'),
        KeypadKey::Divide => (b'/', b'o'),
        KeypadKey::Enter => (b'\r', b'M'),
        KeypadKey::Equal => (b'=', b'X'),
        KeypadKey::Separator => (b',', b'l'),
    };
    with_alt(modifiers, if modes.app_keypad { ss3(application) } else { vec![plain] })
}

fn paste_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                out.push('\n');
            }
            '\n' | '\t' => out.push(c),
            c if c.is_control() => {}
            c => out.push(c),
        }
    }
    out
}

/// 📋️ The bytes for pasted `text`: wrapped in `200~`/`201~` exactly once when the child asked for bracketed paste, with newlines as carriage returns when it did not.
pub fn encode_paste(text: &str, modes: &InputModes) -> Vec<u8> {
    let clean = paste_text(text);
    if modes.bracketed_paste {
        let mut out = b"\x1b[200~".to_vec();
        out.extend_from_slice(clean.as_bytes());
        out.extend_from_slice(b"\x1b[201~");
        out
    } else {
        clean.replace('\n', "\r").into_bytes()
    }
}

/// 🎯️ The focus report for the child, only when it asked for focus events.
pub fn encode_focus(gained: bool, modes: &InputModes) -> Option<Vec<u8>> {
    modes.focus_reporting.then(|| csi(if gained { "I" } else { "O" }))
}

fn button_code(button: MouseButton) -> u8 {
    match button {
        MouseButton::Left => 0,
        MouseButton::Middle => 1,
        MouseButton::Right => 2,
    }
}

fn modifier_bits(modifiers: u8) -> u8 {
    (if modifiers & mods::SHIFT != 0 { 4 } else { 0 }) | (if modifiers & mods::ALT != 0 { 8 } else { 0 }) | (if modifiers & mods::CTRL != 0 { 16 } else { 0 })
}

fn spell_report(code: u8, pos: Pos, release: bool, encoding: MouseEncoding) -> Option<Vec<u8>> {
    let col = u32::from(pos.x) + 1;
    let row = u32::from(pos.y) + 1;
    match encoding {
        MouseEncoding::Sgr => Some(format!("\x1b[<{code};{col};{row}{}", if release { 'm' } else { 'M' }).into_bytes()),
        MouseEncoding::Urxvt => Some(format!("\x1b[{};{col};{row}M", u32::from(code) + 32).into_bytes()),
        MouseEncoding::X10 => {
            if col > 223 || row > 223 {
                return None;
            }
            Some(vec![0x1b, b'[', b'M', code + 32, (col + 32) as u8, (row + 32) as u8])
        }
        MouseEncoding::Utf8 => {
            let mut out = vec![0x1b, b'[', b'M', code + 32];
            for value in [col + 32, row + 32] {
                let c = char::from_u32(value)?;
                let mut buf = [0u8; 4];
                out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
            }
            Some(out)
        }
    }
}

/// 🐁️ The bytes for a pointer event at pane-local cell `local` under `modes`; empty when the child did not ask for this kind of event.
pub fn encode_mouse(event: &MouseEvent, local: Pos, modes: &InputModes) -> Vec<u8> {
    if modes.mouse == MouseReporting::Off {
        return Vec::new();
    }
    let click_only = modes.mouse == MouseReporting::Click;
    let extra = if click_only { 0 } else { modifier_bits(event.mods) };
    let release_code = |button: MouseButton| if modes.mouse_encoding == MouseEncoding::Sgr { button_code(button) } else { 3 };
    match event.kind {
        MouseKind::Down(button) => spell_report(button_code(button) | extra, local, false, modes.mouse_encoding).unwrap_or_default(),
        MouseKind::Up(button) if !click_only => spell_report(release_code(button) | extra, local, true, modes.mouse_encoding).unwrap_or_default(),
        MouseKind::Drag(button) if matches!(modes.mouse, MouseReporting::Drag | MouseReporting::Motion) => spell_report(button_code(button) | extra | 32, local, false, modes.mouse_encoding).unwrap_or_default(),
        MouseKind::Move if modes.mouse == MouseReporting::Motion => spell_report(3 | extra | 32, local, false, modes.mouse_encoding).unwrap_or_default(),
        MouseKind::Scroll { dx, dy } if !click_only => {
            let mut out = Vec::new();
            let steps = |count: i16, negative: u8, positive: u8, out: &mut Vec<u8>| {
                let code = if count < 0 { negative } else { positive };
                for _ in 0..count.unsigned_abs().min(32) {
                    out.extend(spell_report(code | extra, local, false, modes.mouse_encoding).unwrap_or_default());
                }
            };
            steps(dy, 64, 65, &mut out);
            steps(dx, 66, 67, &mut out);
            out
        }
        _ => Vec::new(),
    }
}

/// ↕️ The arrow keys a wheel turns into on the alternate screen when the child does not track the pointer itself.
pub fn encode_wheel_as_arrows(dy: i16, modes: &InputModes) -> Vec<u8> {
    let key = if dy < 0 { Key::Up } else { Key::Down };
    let one = encode_key(&KeyEvent { key, mods: 0 }, modes).unwrap_or_default();
    let mut out = Vec::new();
    for _ in 0..usize::from(dy.unsigned_abs().min(32)) * 3 {
        out.extend_from_slice(&one);
    }
    out
}
