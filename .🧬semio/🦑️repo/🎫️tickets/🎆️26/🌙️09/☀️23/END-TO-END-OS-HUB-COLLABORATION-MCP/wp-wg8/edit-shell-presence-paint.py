import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
text = path.read_text()


def replace(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:100])
    text = text.replace(old, new)


replace("""                let Some((pill, rect)) = pills.get(cursor.item) else {
                    close_chrome_overlay_glass_content(cursor, overlay);
                    cursor.item = 0;
                    cursor.scalar = 0;
                    cursor.phase = 8;
                    return false;
                };""", """                let Some((pill, rect)) = pills.get(cursor.item) else {
                    close_chrome_overlay_glass_content(cursor, overlay);
                    cursor.item = 0;
                    cursor.scalar = 0;
                    cursor.phase = 8;
                    return false;
                };""")
replace("""            // 🛑️🖼️ Per-surface overlay controls — the World3d compute cancel. Painted last so they sit
            // above the surface they annotate, and registered last so `InputState::hit_at` (reverse
            // order) resolves them over the surface's own hit.
            // Each grant recomputes its own rect from the live surface bounds, so nothing survives
            // between grants and a surface that stopped painting stops offering its control.
            8 => {
                let controls = self.surface_overlay_controls(theme);
                let Some((control, anchor)) = controls.get(cursor.item) else {
                    cursor.item = 0;
                    cursor.phase = 9;
                    return false;
                };""", """            // 👥️🖼️ Peer presence over every board window — the wgpu twin of React's
            // `CanvasPresenceOverlayV1`: each other actor's viewport rectangle, cursor dot and name chip,
            // then every peer mark as a corner chip of the peer's initials, clipped to the board and in
            // the hub-assigned colour. Non-interactive by construction: nothing here registers a hit.
            8 => {
                let items = self.board_presence_paint_items(theme);
                let Some(item) = items.get(cursor.item).cloned() else {
                    cursor.item = 0;
                    cursor.scalar = 0;
                    cursor.phase = 9;
                    return false;
                };
                let (bounds, color, chip, chip_rect) = match &item {
                    ShellBoardPresencePaint::Cursor { bounds, peer } => {
                        let color = theme.presence_color(peer.color);
                        (*bounds, color, peer.chip.clone(), Rect::new(peer.at[0] + 10.0, peer.at[1] - 4.0, board_presence_chip_width(theme, &peer.chip), theme.font_size_small + 4.0))
                    }
                    ShellBoardPresencePaint::Mark { bounds, rect, mark } => (*bounds, theme.presence_color(mark.color), mark.chip.clone(), *rect),
                };
                if cursor.scalar == 0 {
                    overlay.push_scissor(bounds);
                    if let ShellBoardPresencePaint::Cursor { peer, .. } = &item {
                        let [x, y, w, h] = peer.viewport;
                        let edge = theme.stroke_hairline * 1.5;
                        let frame = color.with_alpha(0.55);
                        overlay.push_solid([x, y, w, edge], frame);
                        overlay.push_solid([x, y + h - edge, w, edge], frame);
                        overlay.push_solid([x, y, edge, h], frame);
                        overlay.push_solid([x + w - edge, y, edge, h], frame);
                        overlay.push_rounded([peer.at[0] - 2.0, peer.at[1] - 2.0, 10.0, 10.0], color, 5.0);
                    }
                    overlay.push_rounded([chip_rect.x, chip_rect.y, chip_rect.w, chip_rect.h], color, 3.0);
                    cursor.scalar = 1;
                    return false;
                }
                match chrome_text_complete_step(overlay, atlas, &chip, chip_rect.x + 4.0, chip_rect.y + chip_rect.h - 3.0, (chip_rect.w - 8.0).max(1.0), theme.font_size_small, theme.active_foreground, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.error = Some("Shell board presence chip exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
                overlay.pop_scissor();
                cursor.item += 1;
                cursor.scalar = 0;
                return false;
            }
            // 🛑️🖼️ Per-surface overlay controls — the World3d compute cancel. Painted last so they sit
            // above the surface they annotate, and registered last so `InputState::hit_at` (reverse
            // order) resolves them over the surface's own hit.
            // Each grant recomputes its own rect from the live surface bounds, so nothing survives
            // between grants and a surface that stopped painting stops offering its control.
            9 => {
                let controls = self.surface_overlay_controls(theme);
                let Some((control, anchor)) = controls.get(cursor.item) else {
                    cursor.item = 0;
                    cursor.phase = 10;
                    return false;
                };""")
replace("""                    RetainedChromeGroupStep::Fault => self.error = Some("Shell surface control exceeded the retained glyph boundary".to_string()),
                }
                cursor.item += 1;
                return false;
            }
            9 => return true,
            _ => return false,
        }
        false
    }
""", """                    RetainedChromeGroupStep::Fault => self.error = Some("Shell surface control exceeded the retained glyph boundary".to_string()),
                }
                cursor.item += 1;
                return false;
            }
            10 => return true,
            _ => return false,
        }
        false
    }

    /// 👥️🖼️ The board presence this frame paints, in paint order: every cursor (viewport, dot, chip),
    /// then every mark chip, stacked down the board's right edge by entity row like React's canvas marks.
    fn board_presence_paint_items(&self, theme: &Theme) -> Vec<ShellBoardPresencePaint> {
        let mut items = Vec::new();
        for (bounds, overlays) in self.board_peer_overlays() {
            items.extend(overlays.cursors.into_iter().map(|peer| ShellBoardPresencePaint::Cursor { bounds, peer }));
            items.extend(overlays.marks.into_iter().map(|mark| {
                let width = board_presence_chip_width(theme, &mark.chip);
                let rect = Rect::new(bounds.x + bounds.w - 8.0 - width, bounds.y + 8.0 + mark.row as f32 * 18.0, width, theme.font_size_small + 4.0);
                ShellBoardPresencePaint::Mark { bounds, rect, mark }
            }));
        }
        items
    }
""")
replace("""/// 🖼️ Whether a surface is big enough to carry an overlay row at all — a collapsed dock pane paints
/// no chrome over its whole body.""", """/// 👥️🖼️ One painted piece of board peer presence: a peer's cursor (with its viewport and name chip) or a
/// peer mark chip, each with the board it is clipped to.
#[derive(Clone, Debug)]
enum ShellBoardPresencePaint {
    Cursor { bounds: Rect, peer: crate::canvas_presence::BoardPeerCursor },
    Mark { bounds: Rect, rect: Rect, mark: crate::canvas_presence::BoardPeerMark },
}

/// 👥️🖼️ A presence chip's width for its text — the same estimate the status pills use.
fn board_presence_chip_width(theme: &Theme, text: &str) -> f32 {
    8.0 + text.chars().count() as f32 * theme.font_size_small * 0.6
}

/// 🖼️ Whether a surface is big enough to carry an overlay row at all — a collapsed dock pane paints
/// no chrome over its whole body.""")
path.write_text(text)
print("ok")
