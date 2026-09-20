# Driver Editor Runtime Reference

The repaired React server at port 6313 was exercised through the in-app browser after plugin boot. Settings → General → Driver exposed all seven expected axes: Labels Full, Label Tier Normal, Drag Handle, Chrome Reveal Always, Gumball Reveal Always, Tooltips Full, Hotkeys Inline. Driver is Default; Save As is initially disabled and Delete absent.

Changing Drag to Surface immediately removed the Display tree transfer handles and the driver section became `DRIVER (UNSAVED)`. Typing the temporary name `Astra Verification` enabled Save As. Saving selected that named driver, cleared the unsaved marker and name field, disabled Save As, and exposed Delete. Deleting that generated verification preset selected Default, restored Drag Handle and its visible transfer handles, and removed Delete. No verification preset remains.

Observed control ids: `framework.settings.driver.editor`, `.labels`, `.labelTier`, `.drag`, `.chrome`, `.gumball`, `.tooltips`, `.hotkeys`, `.saveLabel`, `.save`, `.delete`; the driver picker is `framework.settings.driver`. React also uses the same id on some containing property rows, so future DOM actions must select the actual control by role/tag rather than count duplicate ids as separate controls.

This is a React-only behavior reference. WGPU implementation is source-stable but native/browser acceptance remains pending the current build. The server warns that puzzle source changed after the preceding activation; final paired acceptance requires the current activation closure. No claim is made that all axes have been runtime-edited yet.

## Driver Drag Axis Reference — Current CUA Session

The actual React settings editor exposes all seven axes. Switching Drag from Handle to Surface through its Select removed the Display window-template handle; DOM evidence on `framework.display.windows.puzzle3d-main.kind` was `draggable="true"` and zero `[data-slot="drag-handle"]` descendants. Switching back to Handle restored exactly one handle and `draggable="false"`. The driver editor showed an unsaved marker after this draft edit; selecting the already-selected Default preset did not clear that marker. No new console error appeared; the latest returned error is the previously recorded theme controlled-input warning at 2026-09-19T23:23:00.240Z, before that fix.

This is React reference evidence only. The physical paired empty-dock probe now requires the semantic WGPU transfer handle and does not fall back to a row drag. Probe syntax bundling passed; paired execution waits for activation7.
