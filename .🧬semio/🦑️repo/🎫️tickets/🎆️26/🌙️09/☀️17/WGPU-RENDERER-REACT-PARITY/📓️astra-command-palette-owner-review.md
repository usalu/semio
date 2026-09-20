# Command Palette Producer Review

The earlier blank-palette audit correctly identified the missing view path. Existing item producers still need parity validation while that view is implemented.

React `ShellHost/🟦️.tsx` lines 10474–10568 constructs panel leaves, window kinds, and resolved commands whose `inPalette` flag is true. Only when `hostMode && panel`, it appends available program spawn rows and the host items `studio.undo`, `studio.redo`, and `studio.home`; Home belongs to the navigation category. Argument-carrying commands open their staged command form.

The WGPU `build_search_items` still adds every app-declared keybinding as a separate row, explicitly described in its source as a WGPU-only superset. It also appends `commitCheckpoint` and spells the Home item `studio.goHome`. Those differences can change cold list contents, ranking, grouping, filtered results, and activation behavior once the missing view is painted.

The Sol palette executor is assigned exact content/order/host-condition validation against the actual React producer, alongside view geometry, input, accessibility, and physical activation. Shared fuzzy ranking alone does not prove equivalent palette data.

## Actual React Palette Probe Calibration

The first real React-only query/activation replay opened the focused input, found and physically activated canonical Set Theme, retired the dialog, and opened the Appearance command panel. Its final assertion was invalid: it required nonexistent argument ID command.os.os.setThemeId.arg.value, whereas the real definition uses arg.themeId and the Form disclosure starts closed. The mounted consequence instead exposes the exact form section plus command-os.os.setThemeId-execute and -reset. Terra confirmed the source identities independently.

Root replaced the consequence with those actual panel/form/action controls, preserving focused typing, exact canonical result, physical activation and input/dialog retirement. A second React-only replay is running before applying the oracle to WGPU. This correction is probe calibration, not a production failure or relaxed WGPU acceptance.

The corrected React-only physical activation replay passed all 3 steps with zero failures (palette-react-15b). The final Terra audit also found missing WGPU keyboard forwarding while the accessibility input is focused, modal interior/close/semantics gaps, premature pointer-down activation, and query persistence/Unicode normalization divergence. These are production gaps requiring another execution packet.

Root added a separate keyboard journey: combobox/listbox/active-descendant validation, ArrowDown/ArrowUp selection, localized query, Escape dismissal, reopen with preserved query, and Enter activation with the same staged-form consequence. A React-only replay is running to validate this stronger oracle.
