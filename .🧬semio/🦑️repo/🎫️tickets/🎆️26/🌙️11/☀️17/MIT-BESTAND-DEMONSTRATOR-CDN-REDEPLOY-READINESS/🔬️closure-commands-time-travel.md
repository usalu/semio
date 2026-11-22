# Closure: instance-commands and time-travel-turns

## Original errors
- `semio-plugin-instance-commands`: E0609 `original_press_selected` on `ActionIngressSource` (browser-playbook.log).
- `semio-plugin-time-travel-turns`: E0559/E0769/E0599/E0609 on `CloseOriginalSources`, `OriginalProgress`, `PreviewDeferred`, `preview_due` (browser-flow.log).

Both were already consistent with current source when re-read (field exists; `CloseOriginalSources { grant }`, tuple `OriginalProgress`, no preview state remains): another agent finished those refactors after the failed builds. No edit was needed for them.

## Further errors found by widening (-p semio-hub-flow-component -p semio-hub-playbook-component)
1. `semio-plugin` E0432: `history_edit_acceptance` re-exported `assert_latest_history_preview` / `assert_mounted_history_overwrite`, deleted with the history-preview removal. Removed the two re-exports and their only consumers (preview tests).
2. `semio-ui-theme` E0425: `chrome_metrics::TRANSITION_*` / `DIALOG_ENTER_*` missing; `styling/🔣️.json` gained motion tokens but generated Rust tokens were stale. Ran `@semio-tech/ui-styling-tokens:generate`.

## Files changed
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (re-export list, line 108)
- xlsx editor unit tests `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/.../✏️editor/🧪️tests/🔬️unit/🦀️.rs`: removed `latest_preview_history_edits_end_to_end`
- gis terrain editor command tests `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/.../✏️editor/🎮️commands/🧪️tests/🔬️unit/🦀️.rs`: removed `a_committed_press_edited_in_history_replays_deterministically`
- Regenerated `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens/🦀️.rs` (+ other-language token outputs)
- Left: fixture key `historyPreview` in xlsx `📊️sheet-grid/🔣️.json` is now unused.

## Final status (check --lib wasm32-wasip2 release, exit 0)
- semio-plugin-instance-commands: pass
- semio-plugin-time-travel-turns: pass
- semio-hub-flow-component: pass
- semio-hub-playbook-component: pass
