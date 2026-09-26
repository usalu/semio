#!/usr/bin/env python3
"""WG10 s13 item 4: the native shell's keyboard ring. Measured before: Tab (no content focus) only cycled dock windows —
no chrome control (navbar, panel tabs, footer, pane chips) was keyboard-reachable natively, and nothing painted a keyboard
focus. Now Tab/Shift+Tab walk the chrome's own accessibility publication (every focusable, enabled, visible control in
reading order) and then the dock windows; Enter/Space activate the focused control through the SAME accessibility path the
platform adapter and the browser mirror use; Escape leaves it; the overlay paints the focus ring."""
import pathlib

S = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
text = S.read_text()


def swap(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:120])
    text = text.replace(old, new)


swap("""        // 🪟️ Tab/Shift+Tab cycles the active window across the *whole* dock (cross-window focus order)
        // whenever nothing else claims focus — this renderer has no DOM, so there was no cross-window
        // Tab order at all before this fix (only ever within a single scene, e.g. the text editor's own
        // `KeyAction::Tab` handling). Single-window content-level Tab cycling among widgets is a
        // separate, content-layer concern for `ui_wgpu`'s own engine/EventRouter (owned by the
        // interpreter cutover), not this chrome-level routing.
        if !editing && !palette_open && self.dock_drag.is_none() && action == ui_wgpu::wgpu::KeyAction::Tab {
            self.cycle_active_window(!modifiers.shift);
            return;
        }""", """        // ⌨️ Tab/Shift+Tab walks the keyboard ring ([`Self::keyboard_ring`]) whenever nothing else claims
        // focus: every focusable chrome control, then every dock window. This renderer has no DOM, so
        // before this the chrome had no keyboard order at all and Tab only cycled windows. Content-level
        // Tab among a window's own widgets stays the retained content's (`EventRouter`), routed above.
        if !editing && !palette_open && self.dock_drag.is_none() && action == ui_wgpu::wgpu::KeyAction::Tab {
            self.advance_keyboard_ring(!modifiers.shift, input);
            return;
        }""")
swap("""    /// 🪟️ Cross-window Tab-focus cycling (see the `KeyAction::Tab` arm in `handle_keyboard`): advances
""", """    /// ⌨️ The native shell's keyboard ring — React's DOM order has no twin without a DOM, so it is the chrome's
    /// accessibility publication (every focusable, enabled, visible control, in reading order) followed by the
    /// dock's windows. The keyboard, a platform assistive technology and the browser mirror therefore reach
    /// exactly the same controls (ticket 26/09/23 slice WG10).
    fn keyboard_ring(&self) -> Vec<ShellKeyboardStop> {
        let mut ring: Vec<ShellKeyboardStop> = self.presented_chrome_accessibility.iter().filter(|node| node.focusable && !node.disabled && !node.hidden).map(|node| ShellKeyboardStop::Chrome(node.key.clone())).collect();
        let mut order = Vec::new();
        Self::dock_window_order(&self.dock.root, &mut Vec::new(), &mut order);
        ring.extend(order.into_iter().map(|(path, window_id)| ShellKeyboardStop::Window(path, window_id)));
        ring
    }

    /// ⌨️ Moves keyboard focus one stop along the ring: onto a chrome control (the accessibility focus the
    /// platform adapter and the mirror set, painted as the focus ring) or onto a dock window (the active
    /// window, whose own content focus takes the keyboard again).
    fn advance_keyboard_ring(&mut self, forward: bool, input: &mut InputState<ActionDescriptor>) {
        let ring = self.keyboard_ring();
        if ring.is_empty() {
            return;
        }
        let current = match self.accessibility_focused_control_id.as_deref() {
            Some(key) => ring.iter().position(|stop| matches!(stop, ShellKeyboardStop::Chrome(control) if control == key)),
            None => self.active_window_id.as_deref().and_then(|active| ring.iter().position(|stop| matches!(stop, ShellKeyboardStop::Window(_, window) if window == active))),
        };
        let next = match current {
            Some(index) if forward => (index + 1) % ring.len(),
            Some(index) => (index + ring.len() - 1) % ring.len(),
            None if forward => 0,
            None => ring.len() - 1,
        };
        match ring[next].clone() {
            ShellKeyboardStop::Chrome(key) => {
                input.blur_input();
                self.chrome_build.focused_retained_surface = None;
                self.accessibility_focused_control_id = Some(key);
            }
            ShellKeyboardStop::Window(path, window_id) => {
                self.accessibility_focused_control_id = None;
                self.dock.set_stack_active(&path, &window_id);
                self.dock.sync_active_window(&window_id);
                self.active_window_id = Some(window_id);
            }
        }
    }

    /// ⌨️ The chrome control the keyboard ring focused, addressed the way an accessibility event addresses it —
    /// `None` when focus is on a window, on a text field, or on a control the presented chrome no longer carries.
    fn keyboard_focused_chrome_target(&self, input: &InputState<ActionDescriptor>) -> Option<ui_render::AccessibilityTarget> {
        let key = self.accessibility_focused_control_id.as_deref().filter(|_| input.focused_id.is_none())?;
        let node = self.presented_chrome_accessibility.iter().find(|node| node.key == key && node.focusable && !node.disabled)?;
        Some(ui_render::AccessibilityTarget { window_id: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(), window_generation: self.presented_chrome_accessibility_generation, node_id: node.node_id, node_key: node.key.clone() })
    }

    /// ⌨️🖼️ Where the keyboard focus ring goes this frame: around the focused chrome control's own hit rect.
    fn keyboard_focus_ring_rect(&self, input: &InputState<ActionDescriptor>) -> Option<Rect> {
        let key = self.accessibility_focused_control_id.as_deref()?;
        input.staged_hits().iter().rev().find(|hit| hit.control_id.as_deref() == Some(key)).map(|hit| hit.rect)
    }

    /// 🪟️ Cross-window Tab-focus cycling (see the `KeyAction::Tab` arm in `handle_keyboard`): advances
""")
swap("""        if matches!(self.overlay_state, OverlayState::Search) && action == ui_wgpu::wgpu::KeyAction::Enter {
            self.activate_search_item(self.search_selected).await?;
            input.blur_input();
            self.accessibility_focused_control_id = None;
            return Ok(());
        }""", """        // ⌨️ Enter/Space press the chrome control the keyboard ring focused — through the same accessibility
        // activation a platform assistive technology and the browser mirror use, so all three press it alike.
        if matches!(action, ui_wgpu::wgpu::KeyAction::Enter | ui_wgpu::wgpu::KeyAction::Space(true)) {
            if let Some(target) = self.keyboard_focused_chrome_target(input) {
                self.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, input).await?;
                return Ok(());
            }
        }
        if matches!(self.overlay_state, OverlayState::Search) && action == ui_wgpu::wgpu::KeyAction::Enter {
            self.activate_search_item(self.search_selected).await?;
            input.blur_input();
            self.accessibility_focused_control_id = None;
            return Ok(());
        }""")
swap("""            10 => return true,
            _ => return false,
        }
        false
    }

    /// 👥️🖼️ The board presence this frame paints, in paint order:""", """            // ⌨️🖼️ The keyboard focus ring around the chrome control the keyboard ring (or an assistive
            // technology) focused — the wgpu twin of the browser's `:focus-visible` outline. Painted last,
            // above every chrome layer; registers no hit.
            10 => {
                if let Some(rect) = self.keyboard_focus_ring_rect(input) {
                    let edge = theme.stroke_hairline * 2.0;
                    let [x, y, w, h] = [rect.x - edge, rect.y - edge, rect.w + edge * 2.0, rect.h + edge * 2.0];
                    overlay.push_solid([x, y, w, edge], theme.focus_ring);
                    overlay.push_solid([x, y + h - edge, w, edge], theme.focus_ring);
                    overlay.push_solid([x, y, edge, h], theme.focus_ring);
                    overlay.push_solid([x + w - edge, y, edge, h], theme.focus_ring);
                }
                cursor.phase = 11;
                return false;
            }
            11 => return true,
            _ => return false,
        }
        false
    }

    /// 👥️🖼️ The board presence this frame paints, in paint order:""")
swap("""/// 👥️🖼️ One peer-presence paint item""", """/// ⌨️ One stop of the native shell's keyboard ring ([`ShellState::keyboard_ring`]): a focusable chrome control by
/// its accessibility key, or a dock window by its stack path and id.
#[derive(Clone, Debug, PartialEq, Eq)]
enum ShellKeyboardStop {
    Chrome(String),
    Window(Vec<usize>, String),
}

/// 👥️🖼️ One peer-presence paint item""")
S.write_text(text)
print("keyboard ring applied")
