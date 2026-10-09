# GIS Schema Oracle Ownership

The only caller of the GIS Map JSON/Ajv compiler is the authored CreateRegion group contract test. Its implementation is moved from the artifact schema root into explicit test support. The caller imports that test owner directly; the semantic root no longer reads JSON files or imports Ajv. The existing neutral fixture and independent schema oracle are retained unchanged. Registered runtime verification remains required.

## Files

- Removed ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧬️schema/🟦️.ts
- Created ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/🧰️schema/🟦️.ts
- Updated ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/🧩️map-create-region-group/🟦️.ts
