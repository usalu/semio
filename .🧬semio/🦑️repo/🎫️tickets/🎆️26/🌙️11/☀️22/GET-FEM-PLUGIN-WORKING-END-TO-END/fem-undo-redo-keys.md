# FEM undo/redo keyboard shortcuts (2D and 3D)

## Verdict
The three failing 3D checks were a harness defect, not a product bug. Run with `--only hist.`, `hist.undo-key` had nothing to undo (no earlier delete in that run), so Ctrl+Z, Ctrl+Shift+Z and Ctrl+Y were no-ops (`47 -> 47`). The palette agent's rewrite in `matrix-v3/checks-core.mjs` (`keyHist`: add a node, blur, press the key) is the one that stands; I removed my duplicate rewrite of the same checks.

## Product path (read from source)
- FEM 2D and 3D declare identical bindings in their manifests (`mod+z` undo, `mod+shift+z` and `mod+y` redo, `delete`/`backspace` removeSelection): 3D `✏️editor/📋️manifest/🦀️.rs`, 2D `✏️editor/🚀️ui-helpers-and-app-assembly/🦀️.rs`. The shortcuts are shared framework behaviour, not duplicated TS.
- Host: `ShellHost/📤️document-transfer-tabs/🔸️document-transfer-ref/🟦️.tsx` `handleAppKeydown` (document keydown inside the shell root). It dispatches `{controllerId, action: "undo"|"redo"}`, the same action the History panel buttons bind (`framework.history.undo.run`, `ui_history_panel`).
- Browser result (6087, 3D): Ctrl+Z, Ctrl+Shift+Z and Ctrl+Y after adding a node: PASS (nodes 16->17->16, redo 17; 2.5 s each), buttons PASS, palette PASS.

## Product defects found and fixed (TS, live after vite restart)
1. User overrides were ignored: Settings > Hotkeys writes overrides under the action id, but `handleAppKeydown` matched the declared `binding.keys`, and the hard-coded `mod+z`/`mod+y` fallback fired regardless. Now `effectiveChordsV1` (override wins, `""` unbinds) drives both the declared loop and the fallback (`historyVerbForChordV1`, defaults `HISTORY_VERB_CHORDS_V1`) in `ShellHost/⌨️window-scope/🟦️.ts`.
2. A `select`, checkbox, slider etc. left focused by an earlier edit swallowed Ctrl+Z (`controlOwnsKeyboardEvent` treated every input/select as text editing; matches the earlier 2D observation "Ctrl+Z with the material select focused emits nothing"). `cleanInterpreterFieldYieldsHistory` in `ShellHelpers/🪟️window-panes/⌨️keyboard-chords/🟦️.ts` now yields history chords for fields that edit no text; text inputs, textareas, contenteditable, textbox role and dirty drafts keep native undo.

## Accessibility (Rust, ships with next wasm publish)
History panel Undo and Redo buttons now carry `aria-keyshortcuts` (`Control+Z Meta+Z`, `Control+Shift+Z Control+Y Meta+Shift+Z`): `lifted/🎮️command/🦀️.rs` (`HISTORY_UNDO_SHORTCUT`, `HISTORY_REDO_SHORTCUT`). Law `history_panel_undo_and_redo_announce_their_chords` in `🧪️tests/🔬️app-panel-kit/🦀️.rs`. NOT run: the `semio-plugin-tests-04` test lib currently fails to compile on peers' in-flight errors (105 errors in other crates, none in the panel crate), log deleted. The announcement shows the default chords, not a user override.

## Laws
Language-neutral fixture `ShellHost/🧪️tests/🧫️fixtures/⌨️history-key-route/🔣️.json` (14 chord rows incl. overrides, 14 focus targets) and suite `🧪️tests/⌨️history-key-route/🟦️.ts` (jsdom; shipped matcher, in-file oracle, third-party `@testing-library/user-event` real events). Registered in the react test config and as target `@semio-tech/framework-renderer-react:history-key-route-check` (`📜️script.ts`, `📋️project.json`). NOT run green: `repo test-file` ran it without DOM (plain bun, `document is not defined`, the older `⌨️history-chord` suite fails identically) and `repo run ...:history-key-route-check` died in the graph reader (`BUILD_SOURCE_UNPARSEABLE`, missing `🎭️actor/🚪️lifetime/🟦️component.js`, a peer's change). Rerun the target when the graph builds.

## 2D (6086) open item, not keyboard-specific
After the wasm of 19:52: Ctrl+Z works (12 nodes, 6-7 s). Redo does nothing within 30-40 s, by key AND by the History Redo button (`hist.buttons` alone: Undo 12 in 7.9 s, Redo still 12), and the Results caption stays "Updating results..." after the undo. So keys and buttons agree (parity holds); 2D redo is blocked behind the pending re-solve or dropped. In the sequence run, after the first undo later undos also failed (`redo-shift`, `redo-y`, `buttons`, `palette-undo-redo`: undo:false). One run crashed the page (`Target crashed`) while the coordinator's chain was loading the host. Needs a follow-up in the 2D history/solve path; 3D is fine.
