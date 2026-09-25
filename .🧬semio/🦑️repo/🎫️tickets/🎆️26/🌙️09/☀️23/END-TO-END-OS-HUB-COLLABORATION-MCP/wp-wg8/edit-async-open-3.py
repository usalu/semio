import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
text = path.read_text()


def replace(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:100])
    text = text.replace(old, new)


replace(
    """    PluginInstall,
    TreeDrag,
    TutorialGesture,""",
    """    PluginInstall,
    /// 🚪️ The frame-pumped document open's band — phase, step, elapsed time and its cancel control —
    /// painted under the plugin-install band while the guest instantiates and loads the document.
    DocumentOpen,
    TreeDrag,
    TutorialGesture,""",
)
replace(
    """            ShellChromeFramePhase::PluginInstall => {
                if !self.render_plugin_install_step(&mut cursor.child, overlay, atlas, input, theme, w) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::TreeDrag);
            }""",
    """            ShellChromeFramePhase::PluginInstall => {
                if !self.render_plugin_install_step(&mut cursor.child, overlay, atlas, input, theme, w) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::DocumentOpen);
            }
            ShellChromeFramePhase::DocumentOpen => {
                if !self.render_document_opening_step(&mut cursor.child, overlay, atlas, input, theme, w) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::TreeDrag);
            }""",
)
replace(
    """//#endregion 🎬️PluginInstallBand
""",
    """//#endregion 🎬️PluginInstallBand

//#region 🚪️DocumentOpenBand
/// 🆔️ The open band's own control: a real cancel while a step is out
/// ([`ShellState::cancel_document_opening`]), a close once the open settled.
const DOCUMENT_OPEN_CANCEL_CONTROL_ID: &str = "shell.document-open.cancel";

/// 🔢️ The steps an open takes (the app instance, then the document), for the band's `step/total`.
const DOCUMENT_OPEN_STEPS: u8 = 2;

/// 🗣️ The band's message: phase, which app, step and elapsed seconds while it runs; a failure appends
/// its own reason. English first, German through [`shell_chrome_string`].
fn document_opening_banner_text(opening: &ShellDocumentOpening, now_ms: f64, is_de: bool) -> String {
    let (key, step) = match opening.phase {
        ShellDocumentOpenPhase::Instantiating => ("document.open.instantiating", Some(1)),
        ShellDocumentOpenPhase::Seeding => ("document.open.seeding", Some(DOCUMENT_OPEN_STEPS)),
        ShellDocumentOpenPhase::Cancelled => ("document.open.cancelled", None),
        ShellDocumentOpenPhase::Failed(_) => ("document.open.failed", None),
    };
    let head = format!("{} {}", shell_chrome_string(key, is_de), opening.label);
    match (&opening.phase, step) {
        (ShellDocumentOpenPhase::Failed(reason), _) => format!("{head}: {reason}"),
        (_, Some(step)) => format!("{head} · {step}/{DOCUMENT_OPEN_STEPS} · {} s", ((now_ms - opening.started_at_ms).max(0.0) / 1000.0).floor() as u64),
        _ => head,
    }
}

/// 🎨️ `(border, fill, text)` for one phase, the same severity map every shell banner uses.
fn document_opening_tone(phase: &ShellDocumentOpenPhase, theme: &Theme) -> (Rgba, Rgba, Rgba) {
    match phase {
        ShellDocumentOpenPhase::Instantiating | ShellDocumentOpenPhase::Seeding => transient_notice_tone(semio_framework::Severity::Info, theme),
        ShellDocumentOpenPhase::Cancelled => transient_notice_tone(semio_framework::Severity::Warning, theme),
        ShellDocumentOpenPhase::Failed(_) => transient_notice_tone(semio_framework::Severity::Error, theme),
    }
}

/// 📐️ The band's rect: the plugin-install band's geometry, one band height and gap further down, so
/// an open that installs its plugin shows both.
fn document_opening_rect(message: &str, action_label: &str, width: f32, theme: &Theme) -> Rect {
    let band = plugin_install_rect(message, action_label, width, theme);
    Rect::new(band.x, band.y + band.h + PLUGIN_INSTALL_TOP_GAP, band.w, band.h)
}
//#endregion 🚪️DocumentOpenBand
""",
)
replace(
    """    /// 🛂️ Resolves this frame's administration sheet from the retained operation alone — packet
    /// W15e.""",
    """    /// 🚪️ Paints the frame-pumped document open's band: which app, which phase, the step and the time
    /// it has taken, and the control that stops it. A settled `Cancelled`/`Failed` record stays until the
    /// user closes it, exactly like the plugin-install band above it.
    fn render_document_opening_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) -> bool {
        let Some(opening) = self.document_opening.as_ref() else { return true };
        let is_de = self.locale_id == "de";
        let action_label = shell_chrome_string(if opening.running() { "document.open.cancel" } else { "common.close" }, is_de);
        let message = document_opening_banner_text(opening, chrome_now_ms(), is_de);
        let (border, fill, text_color) = document_opening_tone(&opening.phase, theme);
        let band = document_opening_rect(&message, action_label, width, theme);
        let action = plugin_install_action_rect(band, action_label, theme);
        let hair = theme.stroke_hairline;
        match cursor.scalar {
            0 => overlay.push_rounded([band.x, band.y, band.w, band.h], fill, theme.border_radius),
            1..=4 => {
                let edge = match cursor.scalar {
                    1 => [band.x, band.y, band.w, hair],
                    2 => [band.x, band.y + band.h - hair, band.w, hair],
                    3 => [band.x, band.y, hair, band.h],
                    _ => [band.x + band.w - hair, band.y, hair, band.h],
                };
                overlay.push_solid(edge, border);
            }
            5 | 6 => {
                let (value, x, max_w) = if cursor.scalar == 5 { (message.as_str(), band.x + theme.padding_standard, (action.x - band.x - theme.padding_standard * 2.0).max(1.0)) } else { (action_label, action.x, action.w.max(1.0)) };
                let baseline = band.y + (band.h + theme.font_size_small) * 0.5 - 1.0;
                match chrome_text_complete_step(overlay, atlas, value, x, baseline, max_w, theme.font_size_small, text_color, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.error = Some("Shell document open band text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
            }
            7 => input.register_hit(HitTarget { rect: action, event: None, control_id: Some(DOCUMENT_OPEN_CANCEL_CONTROL_ID.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None }),
            8 => {
                if self.chrome_build.clicked_this_frame && action.contains(input.pointer_x, input.pointer_y) {
                    self.cancel_document_opening();
                }
            }
            _ => return true,
        }
        cursor.scalar += 1;
        false
    }

    /// 🛂️ Resolves this frame's administration sheet from the retained operation alone — packet
    /// W15e.""",
)
replace(
    """        ("plugin.install.cancel", false) => "Cancel",
        ("plugin.install.cancel", true) => "Abbrechen",
""",
    """        ("plugin.install.cancel", false) => "Cancel",
        ("plugin.install.cancel", true) => "Abbrechen",
        ("document.open.instantiating", false) => "Starting",
        ("document.open.instantiating", true) => "Wird gestartet:",
        ("document.open.seeding", false) => "Loading the document in",
        ("document.open.seeding", true) => "Dokument wird geladen in",
        ("document.open.cancelled", false) => "Opening cancelled:",
        ("document.open.cancelled", true) => "Öffnen abgebrochen:",
        ("document.open.failed", false) => "Could not open",
        ("document.open.failed", true) => "Konnte nicht geöffnet werden:",
        ("document.open.cancel", false) => "Cancel",
        ("document.open.cancel", true) => "Abbrechen",
        ("document.open.busy", false) => "Another document is still opening",
        ("document.open.busy", true) => "Ein anderes Dokument wird noch geöffnet",
""",
)
path.write_text(text)
print("ok")
