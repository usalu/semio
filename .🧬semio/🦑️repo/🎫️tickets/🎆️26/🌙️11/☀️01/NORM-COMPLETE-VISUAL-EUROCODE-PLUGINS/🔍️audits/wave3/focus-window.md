# W3-FOCUS-WINDOW report (2026-11-03)

Static work only (no cargo, no repo run). freeze.txt absent at every edit batch. All compile-unverified; first rebuild must cover the framework crates `shell-dock`, `shell-actions`, `shell-vocab`, `shell-tests-chrome-3`, `manifest-actions` and the norm `inspection` crate.

## Finding
The framework had `ShellCommand::FocusWindow { window_id }` (schema-defined, only sets `active_window_id` in the headless shell state: it neither opens, raises, focuses nor announces) and a palette row `window:<kind>` that only wrote `windows.active_window_id`. `norm.shortcut.focus-properties` (Alt+3) was a *declared app action of the viewport window*: the shell activated the viewport owner and the app accepted it as a no-op `ViewportAct`, so Properties was never focused.

## Framework (domain-neutral)
- Verb `focusWindow` with text argument `windowKindId`: consts `FOCUS_WINDOW_ACTION_ID` / `FOCUS_WINDOW_ARG_KIND` (Rust `manifest/actions/utility-tool-and-document-actions`, re-exported in the framework package; TS twin in `definitions-action-ids-and-shell-brand` + manifest index). Shell-intercepted like Export/Import Document, not a guest reserved job (no job table, binary tag or ABI change; the action schema family `ShellCommand` is the headless state and stays untouched).
- One implementation: `ShellDockState::focus_window_of_kind` (`shell-dock`, new `WindowKindFocus::{Raised, Opened}`): focused instance of the kind, else first, else opens it as a root split (first instance id = kind id, topology queue credit checked, layout persisted); sets stack-active + dock-active + `windows.active_window_id`; refuses with a reason for no session, undeclared kind, full queue.
- `ActionsCtx::framework_focus_window` (`shell-actions` dispatch funnel, intercepted right after the ledger tap): clears `accessibility_focused_control_id` so the keyboard ring continues inside the window (same end state as a Tab stop on a window), announces through the transient notice, which the accessibility projection already publishes as a polite `status` node (ARIA live). Codes `shell.window-focus.announced` (Info) and `shell.window-focus.refused` (Warning, also returned as the error).
- Chrome vocabulary (EN+DE, no default language): `window.focus.raised|opened|refused` templates with `{name}` = the window kind label in the active locale.
- Palette rows `window:<kind>` now dispatch the verb (the ad-hoc `active_window_id = ...` write is gone).
- Chord seam: `ShellKeyOutcome::Verb` already dispatches framework verbs; no new outcome variant (proto/TS twin unchanged).

## Norm
- `HostRoute::FocusWindow(Region)`: Alt+1..Alt+5 (outliner, 3D model, properties, checks, diagrams; they are the only Alt+N window shortcuts; F6 cycling is unchanged and still app-local) now answer `ShellKeyOutcome::verb(focusWindow, windowKindId = norm-<variant>-<role>)` via `focus_window_outcome`. Variant: parsed from the window scope (`variant_of_window`), else the variant of the last layout choice (`CURRENT_VARIANT`), else `Ignored`.
- Context menu bulk-edit row raises the same verb for the Properties kind (disabled with an EN/DE reason when no variant is known).
- Removed ad-hoc handling: `is_focus_shortcut`, its macro arm in `norm_command_from_action!` (viewportAct no-op), and the focus rows from `shortcut_action_definitions()` (they are no app actions any more; the palette offers every window kind through the verb).

## Tests
- Framework (`wgpu-shell-chrome-parity/🎯️focus-window-of-kind`, mounted in `shell-tests-chrome-3`), language-agnostic fixture `Shell/🧪️tests/🧫️fixtures/🎯️focus-window-of-kind/🔣️.json` (action, argument, window kinds, EN/DE announcements): open-when-closed / raise-when-open / refuse-undeclared at the dock level; dispatch of the action clears the chrome focus, activates the window, announces EN and DE with codes and severities, refusal announced and errored, vocabulary templates in both languages.
- Norm editor level (`shell-hooks` tests): every Alt+1..5 chord in every one of the ten variants and from several window scopes yields the verb with the right kind, and that kind is a window of the variant's desktop layout; no focus action remains an app action; updated law tests use variant-bearing scopes. Host routes: `Focus(region)` routes to `FocusWindow`, never to an entity call. Context menu: bulk row carries the verb and kind (`norm-en1993-properties`).
- Third-party check: none available for a native dock; the fixture is shared by the Rust law and can be read by a TS shell twin.

## Open
- The React `ShellHost` has no handler for `focusWindow` yet (TS twin only declares the ids); it needs the same intercept when that shell consumes `ShellKeyOutcome::Verb`.
- Mobile/tablet have no outliner window in their layouts; Alt+1 there opens an outliner window as a root split (the app declares the kind) rather than being ignored; device-aware filtering would need the device class in `handle_key`.
- Compile-unverified symbols: glob re-export of `WindowKindFocus` through `shell-dock`, `UiDirtyScope` in scope of the dock keyboard file, `DslValue::String(kind)`, `ShellTransientNotice` field access in the test (`message`, `code`, `severity`).

"- ready for rebuild: focus window" appended to all ten `build-en19xx/latest.md`.
