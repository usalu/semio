# W3-FOCUS-WINDOW-2 report (2026-11-03)

No cargo/rustc. Rust changes are compile-unverified; the TS law was run through `heavy.sh` (result at the end). "- ready for rebuild: focus window 2" appended to all ten `build-en199x/latest.md`.

## 1. React `focusWindow`
- Pure planner `ShellHost/🎯️focus-window/🟦️.ts`: `planFocusWindowV1` (focused instance of the kind, else first, else open with window id = kind id; refuse undeclared/unavailable), `focusWindowNoticeV1` (EN/DE polite text + codes `shell.window-focus.announced|refused`, same templates as Rust), `focusWindowBodyV1` (moves DOM focus into the stack-active tab panel: first focusable control, else the panel; waits up to 6 frames for a freshly opened window), `focusWindowArgsV1`, `windowKindIdsOfLayoutV1`, `deviceLayoutWindowKindsV1`, `resolveDeviceWindowKindV1`.
- Funnel intercept `ShellHost/🖱️shell-actions/🧭️input-funnel/🎯️focus-window/🟦️.ts` (wired in `input-funnel/🟦️.ts` next to Export/Import Document): opens as root split right via `insertWindowAtDropZone` + `SET_SHELL_LAYOUT`, `SET_ACTIVE_WINDOW_ID`, announces through the existing `showTransientNoticeRef` live region (`role=status aria-live=polite`), refuses with `refuse("not-applied", …, alreadyNotified=true)`.
- i18n: `ui.windowFocus.{raised,opened,refused,unavailable,device.{desktop,mobile,tablet}}` in the typed bundle (`I18n/🟦️.tsx`) and both languages (`🧭️shell-chrome`).
- Test: `engine/🧪️tests/🎯️focus-window-of-kind/🟦️.ts` reads the SAME fixture as the Rust law (`🐚️Shell/🧪️tests/🧫️fixtures/🎯️focus-window-of-kind/🔣️.json`): arguments/ids, open/raise/refuse, every device row, EN/DE announcements and unavailable notices, DOM focus (jsdom).

## 2. Device class (domain-neutral, Rust = TS)
- Rule: device = mobile below the mobile breakpoint, otherwise the user's desktop/tablet layout preference (`selectUiDevice` / Rust `focus_device`). The device layout is the app's named layout whose id equals the device id or ends in `-<device>` (`norm-visual-mobile`); an app without one keeps every declared kind available. A kind that layout lacks maps to the action's stand-in (new optional argument `fallbackWindowKindId`, const `FOCUS_WINDOW_ARG_FALLBACK` in Rust manifest actions + framework re-exports + TS twin) when the layout holds it, else the focus is refused with the localized notice `window.focus.unavailable` ("The {name} window is not part of the {device} layout" / "Das Fenster {name} gehört nicht zum Layout {device}"; device names desktop/mobile/tablet, Desktop/Mobilgerät/Tablet), Warning, code `shell.window-focus.refused`, nothing opened.
- Rust: `FocusDevice`, `device_layout_window_kinds`, `resolve_device_window_kind`, `ShellDockState::resolve_window_kind_for_device` (shell-dock `keyboard-and-palette` staging), `ActionsCtx::focus_device` + resolution in `framework_focus_window`; vocabulary keys `window.focus.unavailable`, `window.focus.device.*`; shell-actions imports `FocusDevice`.
- Norm: Alt+1 now carries `fallbackWindowKindId = norm-<variant>-viewport` (`stand_in_region`: mobile and tablet have no outliner, the scene list of the model window is the outline). Tests updated (`expected_focus_verb`) and extended: no mobile/tablet layout has the outliner window, every one has the viewport, in all ten variants.
- Fixture rows (desktop/mobile/tablet): desktop holds requested; desktop ignores unused fallback; mobile maps to fallback; mobile holds requested; mobile refuses without fallback; tablet maps; tablet refuses when fallback also absent; tablet without device layouts keeps everything; plus 4 EN/DE unavailable notices. Rust tests: `a_window_kind_missing_from_the_device_layout_maps_to_its_stand_in_or_is_refused_like_the_language_agnostic_rows`, `the_focus_window_action_lands_on_the_device_stand_in_and_announces_an_unavailable_kind_in_both_languages` (shell-tests-chrome-3 parity).

## Open / compile-unverified
- Rust: `FocusDevice`/free fns exported via the shell-dock glob; `action_args_json!` with variable values in the new test; `kind_name` closure taking `&Self`; `semio_framework_ui_declaration::layout::{create_named_layout, create_stack_layout}` in the tests crate; `Region: PartialEq` use in norm.
- TS: `selectUiDevice` cast to `FocusWindowDeviceV1`; `core.measuredDevice` and `core.shellStateRef.current.layout.shellLayout` access in the funnel child; no tsc run.
- No third-party layout library exists for this dock; the shared fixture is the cross-implementation oracle.
- Palette rows in React (`window.<kind>` in `search-items`) still dispatch `SET_ACTIVE_WINDOW_ID` directly; switching them to the verb is a one-line follow-up.

## TS law run
Queued behind other heavy commands at report time (flock slot); its output will land in `🗑️generated/focus-window-2/vitest.txt` (delete after reading). Not yet verified.

Update: the vitest run hit its 900 s timeout (exit 124) while queued for the heavy slot and produced no output; the TS law is still unrun. Re-run it through heavy.sh when the slot is free. Generated output removed.
