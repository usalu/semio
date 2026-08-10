# Status — Artifact Io Facets

## Done
- W0: ticket, owner-table (54), normative-spec, fanout-brief
- W1: closed 26-format `mimes.csv`, `MediaFormat` enum, catalog parity policy
- W2: neutral models + codecs + round-trip tests in mesh module
- W3: taxonomy `🚪️io` + twins + `policyArtifactIoBreaches` + launch seed gate
- W4: `ArtifactImport`/`ArtifactExport`/`ArtifactIo`, `required_media_formats`, unified OS handlers
- W5: note + cad pilots compile end-to-end
- W6: all 54 artifacts have `🚪️io` leaves; glue + engine wired; old register_* removed
- W7: TS host bridges for note/cad; accept filter helper; formats derived from facet for pilots
- W8: `policyArtifactIoBreaches` = 0; note/cad `cargo check` green

## Notes
- Leaves map Snapshot ↔ JSON (and specialized SVG/DWG for note) via framework codecs
- Temporary generator: `e6_generate_io_facets.py`
