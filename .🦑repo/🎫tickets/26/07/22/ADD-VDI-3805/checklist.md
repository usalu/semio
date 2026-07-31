# VDI 3805 implementation checklist

## Integration
- [x] `norm/vdi/3805/` scaffold (project.json, script.ts, rs/Cargo.toml, rs/lib.rs)
- [x] Workspace member in root Cargo.toml
- [x] `NormFamilyId::Vdi3805` in norm_core
- [x] Plugin DocumentApp `vdi3805` + playground ports 6105/6205
- [x] `🧪test📏norm` launch entry includes `-p norm_vdi_3805`
- [x] norm/AGENTS.md updated

## Crate regions
- [x] Shared — VdiValue, VdiUnit, LocalizedText, ExtensionBag, ManufacturerFile, ProductIdentity, relationships, SecurityLimits
- [x] Schema — SheetId, EditionId, SchemaStatus, Domain, SheetEntry (1–100), CorrectionOverlay (parts 2–32), SchemaRegistry
- [x] Part1 — record families 010–970.41, parse/serialize native text, building-system number, validate_structure
- [x] Geometry — parametric bbox, connections, clearance
- [x] Functions — linear interpolation, characteristic curves
- [x] Catalog — index, filter, merge stats
- [x] Io — JSON round-trip
- [x] Validate — diagnostics with severity
- [x] SheetParts — `part_1` + `part_02`..`part_100` via macro
- [x] Session — Document, reference_fixture, evaluate, Vdi3805Family

## Tests
- [x] evaluate reaches operative sheet families
- [x] native text round-trip
- [x] correction overlay applicability
- [x] reserved sheet N/A
- [x] geometry, functions, catalog unit tests
- [x] norm_host + NormFamilyId
