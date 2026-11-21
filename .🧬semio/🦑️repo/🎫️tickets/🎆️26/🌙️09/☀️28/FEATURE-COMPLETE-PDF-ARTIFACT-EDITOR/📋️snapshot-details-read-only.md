# Observed PDF Snapshot Details

The snapshot UI uses the existing typed Details provider and granular mutation table. Every authored document collection has a typed edit path or an explicit retained-object edit. The PDF schema identity, file header declaration and admitted stream roles represent the editor's schema, the native document's observed header and its physical stream provenance. The granular mutation table cannot accept direct writes to these three values. The current Details UI presents editable controls for two of those values and their descendants.

The candidate marks the three fields with JSON Schema `readOnly` and adds bilingual labels for stream roles and marked-content properties. The shared Details provider follows the active field schema along its actual path, including referenced schema and inherited read-only status. Small immutable scalar values render as plain display text. Immutable collections remain browsable without insert, remove, move or editable-source fallback controls. The remaining typed fields retain their normal controls and mutations.

Ajv verified the four neutral write-policy cases. The before and after PDF schemas compare identically after removing the added annotations. The candidate's Details test closure measures 22 files, 3,233 active lines and 180,021 bytes under macOS, Linux, Windows and WASI settings. The linked Rust law passed with the real UI arena and observed that the header renders as display text without a text input. The test also exercises inherited collection protections and ordinary editable fields. The mounted surface test remains pending. The production sources remain staged for a subsequent capture.

- [Neutral read/write cases](🔣️snapshot-details-read-only.json)
- [Independent Ajv verification](📜️snapshot-details-read-only-oracle.ts)
- [Reviewed source stage](📜️snapshot-details-read-only-stage.py)
