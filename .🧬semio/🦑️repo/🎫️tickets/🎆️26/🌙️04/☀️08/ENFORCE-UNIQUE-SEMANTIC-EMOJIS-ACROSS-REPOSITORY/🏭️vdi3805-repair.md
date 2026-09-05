# VDI 3805 Hand Review

The 19 mutation payload schemas and actual scenario mutation payloads were read individually. The chosen root is `🏭️vdi3805`, describing manufacturer product data, and its aggregate suite is `🏭️mutate-vdi3805-1`. Stable domain IDs and all numeric/product fixture data remain unchanged.

These are case-by-case decisions, not an executable migration palette. The beach, animal and arbitrary color choices are replaced by the actual operation and scenario roles. Existing meaningful refusal and rename cases are retained.

| Original Operation | Chosen Operation | Chosen Scenario |
| --- | --- | --- |
| 🏖️rename-product | 🏷️rename-product | 🏷️retitles-vlv-50-001-and-resyncs-its-index-tags |
| 🦋change-strict-mode | 🔒️change-strict-mode | 🔒️turns-strict-mode-on |
| 🐢delete-curve | 📉️delete-curve | 🚫️removes-the-curve-kvs-flow-curve |
| 🏝️create-curve | 📈️create-curve | 📈️adds-the-curve-dp-pressure-drop-curve |
| 🐳delete-product | 🗑️delete-product | 🚫️removes-vlv-50-001-and-its-index-entry |
| 🗻replace-product-configuration | 🎛️replace-product-configuration | 📏️reparameterises-vlv-50-001-to-dn-80-and-resyncs-index-dn |
| 🏕️update-manufacturer-file | 🏭️update-manufacturer-file | ✏️renames-the-header-manufacturer-to-acme |
| 🐌replace-geometry-parameters | 🧮️replace-geometry-parameters | ➗️rescales-geom-valve-50-to-half-and-adds-clearance |
| 🏟️resize-geometry | 📐️resize-geometry | 📐️doubles-the-geom-valve-50-bounding-box |
| 🏔️remove-geometry-connection | ✂️remove-geometry-connection | 🔌️detaches-the-out-connection-from-geom-valve-50 |
| 🪵create-product | 📦️create-product | 📦️appends-vlv-80-002-and-its-index-entry |
| 🐝change-edition-profile | 🔖️change-edition-profile | 🆕️switches-sheet-8-from-legacy-to-current |
| ⛰️remove-edition-profile | 🧹️remove-edition-profile | 🧹️clears-the-sheet-8-legacy-override |
| 🐬delete-geometry | 🚮️delete-geometry | 🚫️removes-the-geom-valve-50-definition |
| 🏞️replace-curve-points | 📍️replace-curve-points | 📍️resamples-curve-kvs-onto-three-points |
| 🦈update-limits | 🚧️update-limits | 🛡️tightens-every-untrusted-input-limit |
| 🏜️change-correction-as-of | 📅️change-correction-as-of | 📅️advances-the-correction-cut-off-to-2025-03 |
| 🐞add-geometry-connection | 🔌️add-geometry-connection | 🚰️attaches-the-drain-connection-to-geom-valve-50 |
| 🦭create-geometry | 🧊️create-geometry | 🧊️adds-the-geom-valve-80-definition |

All 19 operation directories and reviewed scenario directories, the family root and aggregate suite have been moved. Exact imports, descriptor identities, Python/Gherkin vector coordinates and oracle source/scenario metadata were repaired, with explicit central schema/test membership. Add-connection uses `🔌️`, avoiding the sibling `🔗️.graphql` file. The Python adapter's declared vector root now resolves its actual owning subset; 76 native includes also lost an incorrect duplicated standards/subsets prefix.

Verification: all 116 captured immutable JSON/schema hashes are unchanged; 291 recursively inspected directories resolve to semantic kinds; 144 JSON documents parse. All 232 literal native include/path dependencies resolve. The scoped audit covers 362 files, 296 directories, 658 governed entries with all eight categories zero. The production feature parser resolves 77 declared assets. Actual independent Python execution passes all 38 mutation/inverse scenarios. The remaining `identity-round-trip` scenario still fails with its already-documented missing carrier grammar; no assertion was weakened and no fixture was rewritten to conceal that existing limitation. The final global and shared Norm catalog checks remain pending.
