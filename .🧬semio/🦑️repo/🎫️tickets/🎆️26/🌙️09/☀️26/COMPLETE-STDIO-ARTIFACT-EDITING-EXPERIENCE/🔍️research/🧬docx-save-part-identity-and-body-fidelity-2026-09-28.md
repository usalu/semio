# DOCX Save Part Identity and Body Fidelity

## Source repairs

The DOCX importer follows the package office-document relationship and its styles relationship, including relative and absolute targets. Export previously synchronized the fixed `word/document.xml` and `word/styles.xml` paths. On a package whose main part lived elsewhere, a text edit wrote a new default part while the root relationship still selected the original unchanged content. Export also skipped styles synchronization whenever the typed styles list became empty, so deleting the last style was undone on reopen.

`sync_main_part` now follows the same actual relationship targets as import and synchronizes an existing styles part even for an empty list. New styles use the main document’s directory. A three-case neutral fixture covers default paths, a relative parent-directory styles target, and an absolute custom styles target. The authored native law edits the main run, clears all styles, verifies the source remains unchanged by save, reopens the native bytes, and independently checks the physical ZIP paths and XML payloads with the existing workspace ZIP test library. Rust formatting and scoped diff checking are the available source checks; the new law has not yet run.

## Remaining body fidelity gap

Import maps only paragraph/table blocks, only direct paragraph runs, and only text plus three boolean run properties. The export `document_into_part` retains root attributes/prolog and unknown body children but regenerates every paragraph/table and appends unknown body children after them. A text edit therefore still loses nested hyperlink/bookmark/drawing/run-attribute content and changes the interleaving of unknown body nodes. Styles preserve unmodeled style children but reorder them around regenerated name/basedOn nodes. These are source-backed gaps in end-user fidelity, not native failures exercised in this slice.

The long-term solution must make complete document XML the persisted authority and expose semantic projections/commands that edit addressed XML nodes, following the canonical OPC metadata design. Adding a second copied XML tree beside the typed body would introduce competing authorities. This remains required work before claiming complete Word editing.

## Owner-local Relationship Identity Repair

The follow-up audit found fixed rId1/rId2 allocation could collide with unrelated retained relationships. The save-part fixture now covers both occupied owner namespaces and expects rId3. DOCX uses the shared OPC generated-relationship helper; XLSX and PPTX use that helper for missing package-main relationships and share the same allocator for generated sheet/slide identities. Static audit, formatting, and scoped diff checking passed. The new native assertions remain pending.
