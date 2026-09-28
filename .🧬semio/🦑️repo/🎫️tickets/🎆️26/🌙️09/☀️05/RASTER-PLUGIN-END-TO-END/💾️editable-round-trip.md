# Editable Document Round-Trip Investigation

The next workflow gate must exercise a populated editable document through the retained archive/output and load APIs. A plain JSON stringify/parse or `RasterSnapshot::encode_pack` test would not establish that gate: the existing Raster fixture explicitly permits synchronous pack output only for an empty shell, while populated snapshots use retained page authority.

Relevant existing surfaces:

- `PluginApp::document_archive`, `begin_document_archive_load`, `poll_document_archive_load`, cancellation and terminal acknowledgement own the end-user archive lifecycle.
- Raster `schema/mutations/binary` already tests populated snapshot output and bounded cancellation/close; its owner factories should be reused for fixture preparation and observation.
- The JSON import leaf converts stdio's structured JSON into `RasterSnapshot`, but does not by itself prove history or editable archive restoration.
- The existing editor `raster_envelope_wire` helper intentionally builds an empty snapshot. It is insufficient for locks, masks, image assets, adjustment parameters, ordering, transforms or history preservation.

Required next test: build a populated document using semantic commands, including nested layers, a pixel asset, a mask, adjustment parameters and protection; capture its editable archive, load it into a fresh app through the retained lifecycle, and compare the materialized document and undo/redo behavior. Use an independent neutral fixture for exact preserved fields. File-picker/download interaction and a real reload remain separate live acceptance requirements.

No archive round-trip implementation or runtime pass is claimed by this investigation.

## Authored Workflow Regression

Added `editor/💾️document` with a neutral layer fixture: transformed group and pixel layer, linked group mask, unlinked/inverted pixel mask, shared embedded PNG, protected brightness/contrast adjustment, and a bilingual rename. The TypeScript test validates the same layer tree with AJV, preserves every field through the first-party parser, and independently decodes the asset with Sharp. The Raster TypeScript target now passes **120 tests, 0 failures** (`raster-editable-fixture-ts-1.log`).

The new native test loads this populated fixture, performs a semantic rename, captures `PluginApp::document_archive`, restores through the public retained load/poll/acknowledge lifecycle into a fresh app, and requires exact snapshot equality followed by exact undo/redo. It is authored and registered, but not yet run. Native export run 5 was already active when this test was added; inspect its actual census before deciding whether another run is needed. The expected Raster census with this test is 310. This does not replace real file UI and reload acceptance.

Native run 5 did include the new test. It failed immediately while the fixture replaced the demo layer vector: dropping the prior adjustment layer bypassed its owned-map retirement. The fixture now explicitly retires the replaced layers through `retire_raster_layers`. This was a fixture setup failure; archive behavior has not yet been observed. Native run 6 includes the repair and backtraces for any subsequent failure.

## Native Run 7 Fixture Correction

Owned layer replacement now proceeds without the prior retirement failure. Run 7 stopped at the pre-archive fixture equality because serde_json distinguishes integer and floating number representations, while the Raster schema serializes transform coordinates as f64. The neutral fixture's transform numbers are now explicitly decimal-valued. This preserves the exact assertion instead of weakening it. Archive loading/history behavior remains pending full native run 8.

Current native checkpoint: the archive test now reaches hydration but fails Identity. Source inspection traced this to its handcrafted initial envelope omitting dialect, whereas normal app creation sets RASTER_DIALECT and retained hydration requires it. The fixture now sets that canonical dialect explicitly; no legacy acceptance or hydration bypass was added. Native run 2 (75362) is verifying the correction.

Full native run 2 (75362) passed all **321 tests**, zero skipped. The corrected archive dialect and complete retirement fixture are verified, together with exact transform undo/redo. Live end-user acceptance remains separate and open.
