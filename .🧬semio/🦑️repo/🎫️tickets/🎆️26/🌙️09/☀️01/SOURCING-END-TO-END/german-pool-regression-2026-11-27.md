# German Pool regression (2026-11-27)

## Cause
Not the panel label. Tabs/panels in de are "Pool", "Kuratiert", "Vorschau", "Raster" (probe: probe-german-pool.ts). The product's German demo catalog name changed deliberately and correctly:
`beam-kvh-c24` de = "Konstruktionsvollholz C24 100×200" (was "KVH C24 100×200"), in
`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧾️domain/🦀️.rs` (CATALOG_KIND_NAMES).
The journeys looked for the old name, so the Pool row assertion and every row-click step timed out; the "Pool" tabpanel wait in the log was the row filter inside it.

## Fix (journey side only, product text is correct)
- journey-kit.ts: stockNames.de.kinds KVH -> "Konstruktionsvollholz C24 100×200" (feeds n()).
- journey-history.ts: kvh is now language-aware.
- journey-surfaces.ts: stockRows de for beam-kvh-c24.

## Results (against 6081)
- de core journey: 17/17 PASS (de-fix-core.log)
- de editing journey: 20/20 PASS (de-fix-editing.log)
- surfaces en: "settings language switch flips every sourcing label at runtime and back" now PASS (27 pass total).
- de history journey not rerun (kvh fix applied).

## Open (not label related, en surfaces, 7 FAIL)
typology option filter (beams shows all rows), curated sorts, curate row action, Empty example keeps 12 rows (navbar and Set Active Example), preview empty state, grid reuse GLB. Look like product/example-switch behaviour (Empty example not clearing), cascading from it; see de-fix-surfaces-en.log.
