# TS and fixtures fork results

Done: TS manifest removal, fixture edits, oracle test. bun tests green: manifest-id-only (8), sqlite snapshot (26), mutate-jack-1 + query inference (17), dev graph (3).

Test-only manifest ids that Rust tests must register with Manifest::register (definitions kept in fixtures):
- test-retained: top-level manifests.test-retained in C:\semio\✏️s\🧑‍💻dev\🔱️trinity\🪆️graph\🧪️tests\🧫️fixtures\🪆️retained.json
- mini, gizmo: top-level manifests map in lsp tests protocol 🔣️.json

Fixture key changes the Rust side must follow:
- sqlite fixtures 🔣️.json: nativeCase.manifest, valueTypes, propertyKinds, directions, optionalIdentifiers, expressions, sourceControl.deep/cancelAt removed; tableWidths/tableRowCounts have 3 tables; invalidSqlEdits keep 5 document/camera/child edits. nativeCase.manifestId stays "independent-registry-id" (opaque in sqlite).
- executor resumable-query: the inlineManifest case lost its inlineManifest/manifestId:null keys (now a plain nakagin case); drop the Rust inlineManifest branch.
- 🗄️.sql still has 13 tables (Rust/SQL owner); the TS JACK_SQLITE_SCHEMA is 3 tables with manifest_id TEXT NOT NULL; keep them equal.
