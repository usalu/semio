# Archive Phase Diagnostic Access

The current `PluginApp::close_diagnostic` implementation in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs:367–388` reports archive load state, phase, terminal target, hydration phase, remaining owner, original close requirement, history and retained presence only if `close_rung()` is `DocumentArchive`. The adjacent `typed_operation_diagnostic` at lines 390–397 does not report archive phase. The close rung selection is in `🔌️plugin/🪜️maintenance-ladder/📥️staging/🔒️close-ladder/🦀️.rs:579–622`; another rung may mask the archive details.

The plugin WIT schema contains no archive phase or close diagnostic endpoint. A search of the plugin Rust sources finds `close_diagnostic` called in native fixture and construction code, not exported through the guest protocol. This source audit does not establish the capabilities of the older published binary.

The native fixture `🔌️plugin/🧪️testing/🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs:380–399` calls real maintenance and archive poll operations and, with `SEMIO_XLSX_ARCHIVE_PROGRESS` set, prints `close_diagnostic` at power-of-two polls starting at 2048. The caller must supply the browser archive to make a comparable witness; the fixture alone does not do so.
