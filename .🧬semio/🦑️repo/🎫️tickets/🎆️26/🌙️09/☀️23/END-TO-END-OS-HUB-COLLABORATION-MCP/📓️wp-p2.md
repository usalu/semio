# WP-P2 — Replace Real Stubs With Complete Implementations

Slice: P2. Scratch: `.tmp-ticket/wp-p2/`.

## Status

| Item | Status | Evidence |
|------|--------|----------|
| 1. stdio SDK Wave 3 + IFC/STEP/PNG | PARTIAL | Stdio already on framework SDK builder/analyzer (no local Wave 3 traits). IFC/STEP/PNG oracle fixture proof not re-executed this slice. |
| 2. architect register upserts | DONE (code) | Event-sourced `absorb_register_mutation` + CSV endpoint fidelity tests (prior continuation). |
| 3. lowpoly dwg/gltf/las/stl | DONE | Mesh resolution via `world_parts`/`snapshot_from_parts`; DSL lossless carriers; `cargo test -p semio-s-artifact-lowpoly-lowpoly --lib io::` 13/13; `descriptor_is_fresh` ok after `describe`. |
| 4. forms csv/xlsx/zip | DONE | DSL carriers (CSV row / XLSX sheet / ZIP `forms.dsl`); `csv_xlsx_zip_dsl_carriers_round_trip_forms` ok; xlsx+zip deps added. |
| 5. shared framework txt codec | DONE | `io_mechanism::{serialize_dsl_txt,deserialize_dsl_txt}`; ~28 plugin txt stubs wired Lossless. |

## Logs
- `wp-p2/generated/lowpoly-io-all.log`, `lowpoly-descriptor-retest.log`, `forms-roundtrip2.log`
