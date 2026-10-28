# Interoperability Protocol Review

Focused source audit of the current native STEP/IDML session, IDML text/assets/parent helpers and format-local export job integration. No concrete new borrow, visibility, error-conversion or cleanup defect was established from the inspected source, so this review made no source edits. This is not a compilation or native runtime pass.

## Checked Against Primary Definitions

- Canonical XML `xml_document_to_text_checked` returns Result<String,String>; the fragment helpers preserve that error type correctly. XmlNode, XmlDocument and XmlAttr constructors match their first-party definitions.
- Canonical ZIP `StoredZipStream` exists at the imported public stream path and derives Debug. Its begin_entry, write_chunk, end_entry, known_entry, finish_one, entry_is_open, next_release_bytes, close_one and terminal_is_empty signatures match the session calls. Its public 4096-byte chunk limit matches session emission.
- IDML helper and module visibility reaches sibling export_session through pub(super), with text/asset helpers exposed at crate scope where needed. No private-child access was found.
- STEP project_edges returns the inspected EdgeProjection with curves, unit_to_points and issues. B-rep fields and selected-page export calls match their primary owners. Error conversions use String or explicit to_string conversions.
- LayoutExportKind tool IDs, MIME types, extensions, binary/text classification, begin/encode dispatch and checkpoint tags cover Png, Svg, Pdf, Package, Step and Idml. STEP remains raw UTF8; IDML follows base64 output. Selected STEP and whole-package IDML dispatch select the intended session.
- Export-to-IO dependency direction does not introduce a new regular dependency cycle in the inspected manifests; all explicit manifest paths exist.
- Layout IO cold reference folds call apply_diff_cold and import ColdMutation; no superseded apply_diff calls were found in this integration. The previously reported aggregate derive blocker is outside this session audit.

## Ownership And Close Review

InterchangeExportSession owns pending bytes, IDML layer IDs, ZIP metadata and an optional StoryFragmentPlan. Part and STEP state own only scalar cursors. StoryFragmentPlan itself contains only usize/Option<usize> fields, so its zero-byte one-item close demand is justified.

next_release_bytes matches the next allocation actually retired: pending Vec capacity; one last layer String capacity; the empty layer Vec backing allocation; then the ZIP writer next allocation. ZIP similarly closes an active name, one stored name, then the metadata Vec backing allocation. close_one refuses an undersized byte budget before those allocations are released. terminal_is_empty covers every dynamic owner in the session.

The engine quotes the session next_release_bytes for the Interchange stage and consumes the matching release receipt. It removes the session only after terminal_is_empty, then reaches Fidelity; Fidelity now dispatches to its original retained close route. The captured request Arc remains live until session cleanup precedes snapshot retirement. Existing native laws cover bounded chunks, partial session close, typed format round trips and the ordinary driver cleanup. Those laws have not been executed in this environment.

Source-level NLL analysis found no retained field borrow used after a self.publish call or Part replacement; the fragment bytes and resume scalars are evaluated before the enclosing borrow ends. This is inspection evidence, not a substitute for the borrow checker.

## Verification Performed

A fresh strict Rust grammar parse passed for all 39 current interchange/export Rust source files. Both IO/export manifests had zero missing explicit path dependencies. Primary XML/ZIP/STEP definitions and current dispatch source were read directly. Native compilation, borrow checking, actual ZIP/XML/STEP output execution and GUI job completion were not run; the repo-native test execution limitation remains.

## Limits Retained

- The engine supplies one stable captured Arc snapshot. Standalone InterchangeExportSession callers must likewise keep the document unchanged across advance calls; the public session accepts a fresh borrow on each call and does not itself capture or enforce source identity.
- 4096 bytes bounds emitted chunks, not every temporary XML fragment allocation or every semantic operation. Frame scans for threading, layer membership searches and canonical XML subtree serialization remain synchronous inside their documented subset limits.
- STEP session output is the explicitly documented constant-size sheet projection. It does not export the complete page content as STEP geometry.
- Existing declared IDML fidelity limits and unsupported source features remain as recorded in interoperability.md. No broader format redesign or interface change was made by this review.
