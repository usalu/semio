# TUI Window Silhouette

The dashboard window outline was not one closed stroke.

The title chip steps up from the body, and the cells beside that chip are outside the window. The command list was laid out on the hairline where the chip bends into the body, so that stroke was overwritten. The whole window rectangle was also filled, so the notch read as window surface.

The outline is now the union of the body and the raised chips. Notches keep the shell floor. Child content is inset to the row under that hairline, including when a bottom chip is present, so the stroke stays closed while the body paints.

The body hairline runs the full width of the window. It stays closed under every inactive tab. Only the active tab leaves that edge open, so the chip and the body are one continuous outline. A focused window paints that whole stroke in the active color (`--active-base`). An unfocused window keeps the normal border.

Checked with `cargo test --features tui --lib window_` in `semio-framework-ui`. The active-tab opening, active-color stroke, notch, and composed-wizard cases passed, and the existing window-chrome cases passed. `shell_window_wizard_body_paints_options_after_remount` still reads the wizard filter row and fails the same way it does against the current wizard painter.
