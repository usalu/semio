# Accessible generation form viewport

Generate publishes the framework Form landmark in both generation editors. Its name comes from the localized window label. The form clips and scrolls vertically through the shared viewport layout, with the fields kept in their shared vertical stack. The app waits for a selected generation before building the input spec.

Editable scalar controls use the Field component's stable child key and carry the question label. This connects the visible label to the input in React and gives native accessibility the same ownership. The slider also exposes its own accessible name. The roster rename editor carries an explicit localized name. The roster's other action arguments already come from the row target, which supplies the saved generation ID.

## Verification

- Chromium mounted the actual retained React interpreter, window chrome and stylesheet with the language-neutral form fixture. English and German both exposed the named form, textbox, number input, slider and toggle. Wheel scrolling and Tab reached fields in a 128-pixel pane. Clicking the visible caption label focused the text control.
- The Chromium probe exercised the actual shell intent bridge. Text blur, slider arrow-key preview and release, and the toggle produced the expected values and generation, question and window IDs.
- The pinned Rust compiler built the actual shared UI builder source against cached dependencies. The native witness passed for both localized labels, the Form role, vertical scroll layout and the Field child identity. The compiler reported two existing shadowed budget re-exports in the builder.
- The Generation3d composition test uses the same fixture to verify original Flow widget projection, component roles, labels, keys and bindings. Its full native run and the canonical desktop app journey remain pending. The working volume held 1.07 GiB free at the verification check. The full linked build requires 5.5 GiB free.
- The form still uses the shared 128-node document publication budget. Very large forms need a framework-level control window to retain normal form semantics while mounting only the visible controls.
