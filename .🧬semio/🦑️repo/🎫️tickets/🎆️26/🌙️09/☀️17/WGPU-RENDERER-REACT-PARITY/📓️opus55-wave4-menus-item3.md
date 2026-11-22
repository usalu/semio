# Wave 4 item 3: app switcher example-label overlay

## What React does
- `NavbarExampleSelect` (`🖱️ui/🧱️elements/🧪️NavbarExampleSelect`) paints the `options` it is given. `ShellHost/🔎️command-bar/🔶️scope` builds them: `label = resolveAppLabel(appLabelsOverlay, "example", example.id, resolveManifestLabel(example.label, terminology, locale))`.
- `resolveAppLabel` (`🛠️ShellHelpers/🏷️labels`) is `overlay.exampleLabels[id] ?? fallback`. The overlay is `PluginAppLabelsOverlay` (ten id-keyed maps, `🛂️manifest/🗣️contributions-and-examples`), kept in `shellState.windowUi.appLabelsOverlay`.
- Feed: `🏛️ShellHost/🔁️contributions-refresh` reads the `labels` section of each program refresh (`cache.get("labels")`), `normalizeAppLabelsOverlay` fills omitted maps, then dispatches `SET_APP_LABELS_OVERLAY`.
- Finding: the reactor (`🔌️plugin/⚛️reactor/.../🖥️surface-render`) answers the request's `labels` with `let _ = &request.labels;`, so no program publishes labels today and React's overlay is always empty (manifest labels show). The wgpu shell now has the same data path and resolution, so both stay equal the day a program publishes it. `ui.app.setLabelOverride` (module-state `app_labels_overlay`) is a different, app-keyed store React's select never reads; not used.

## Changes
- Shared model (Rust twin of the TS type): `AppLabelsOverlay` (ten `BTreeMap`s, serde camelCase, every map defaulted = `normalizeAppLabelsOverlay`), `AppLabelKind`, `AppLabelsOverlay::resolve`, in `🛂️manifest/🧱️app-surface/📚️breadcrumbs-examples-and-command-grammar/🦀️.rs`; re-exported (both `use` lines) in `📦️packages/🦀️rust/🦀️.rs`.
- Shared fixture `🛂️manifest/🧪️tests/🧫️fixtures/🏷️app-labels-overlay.json` (en/de sample labels, five cases) read by both languages.
- wgpu shell: `DocumentState.app_labels_overlay` (`📄️docs/🧬️component`, constructed in `🐙️state/🧩️session-and-topology`), `DocumentState::set_app_labels_overlay_section(Option<&str>) -> bool` (React's refresh dispatch; `true` = changed, owe a chrome refresh), `shell_overlay_example_rows` + `SHELL_EXAMPLE_CONTROL_PREFIX` (`📄️docs/🧱️chrome-paint-and-navbar`), applied in `session_example_rows`. Its four callers (navbar trigger, dropdown, hit target, overlays) therefore paint the overlay label; the trigger text and the accessible name both come from the row label.

## Laws
- TS (ran, 6 pass): `🛂️manifest/🧪️tests/🏷️app-labels-overlay/🟦️.ts`.
- Rust: `🛂️manifest/🧪️tests/🏷️app-labels-overlay/🦀️.rs` (`the_overlay_resolves_every_shared_case_like_react`, mounted in `🧱️app-surface/🦀️.rs` as `app_labels_overlay_tests`).
- Rust: `the_example_picker_paints_the_overlay_label_and_keeps_the_manifest_label_for_ids_it_omits` appended to `🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🎛️example-picker-and-role-groups/🦀️.rs` (crate `semio-renderer-wgpu-shell-tests-chrome-3` hosts `wgpu-shell-chrome-parity`).

## Not compiled (lane E)
`cargo check --tests` and test: `semio-framework-manifest-app-surface` (tests `app_labels_overlay_tests`, `example_picker_tests`), `semio-framework` (rust package re-export), `semio-renderer-wgpu-shell-docs`, `semio-renderer-wgpu-shell-state`, `semio-renderer-wgpu-shell-view-chrome`, `semio-renderer-wgpu-shell-view-overlays`, `semio-renderer-wgpu-shell-pointer`, test crate `semio-renderer-wgpu-shell-tests-chrome-3` (`example-picker-and-role-groups` laws). Risk: `serde_json` in `shell-docs` (already used there), `SHELL_EXAMPLE_CONTROL_PREFIX` reachable through the docs crate glob re-export.

## Known gaps
- No producer: nothing in the wgpu build calls `set_app_labels_overlay_section` yet, because no program emits a `labels` section (same as React). When the reactor starts answering `labels`, the wgpu refresh must read it (add a `Labels` section beside Engagements/Measures/Tools in `UiRefreshSection` and the shell's `render_refresh_detached`) and call the setter, owing a chrome refresh on `true`.
- Only the example kind is painted; other overlay kinds (action, mode, panel tab, ...) have the model but their wgpu painters still read manifest labels.
- `includeNoExample` row and landing-app hiding stay as audited (no React host passes the option).
