# Hub Fixture Naming Follow-up

Four newly observed fixture directories were reviewed independently: directory event paging is `📅️event-page-route-v1`; an expiring execution-target lease is `⏳️document-execution-target-lease-v1`; frozen GIS binding is `🧊️gis-map-frozen-binding-v1`; and proposal approval is `🗳️gis-map-proposal-approval-v1`. Existing `🗺️gis-inference-job-v1` remains the map-job identity. No fixture payload was altered.

The Hub source script, trusted catalog, inference catalog/runtime, and binary include paths reference the exact renamed fixtures. Two new proposal fixture includes appeared during the repair and were corrected without changing their surrounding implementation.

The focused audit reports 134 files, 105 directories, 222 governed nodes, and zero findings in all eight categories. `bun nx run os-hub:gis-map-frozen-binding-source-check` passes 52 checks. `bun nx run os-hub:directory-event-page-v1-source-check` reaches and reads the renamed fixture, then fails its existing source-hostile assertion: `directory event page source oracle admitted removed fence 3` at `📜️script.ts:5208`. That source-semantic failure is recorded rather than weakened; no native runtime success is claimed for the event-page feature.

## Final Concurrent Additions

The newly added administration page fixture is `📇️directory/🏘️space-administration-page-v1`; inference catalog selection is `🎯️inference-catalog-selection-v1`, distinct from `🪪️execution-target-relay-v1`. Exact source and include references were updated. The lease fixture is now `🔏️document-execution-target-lease-v1`, reflecting a concurrent owner's later semantic choice; that work was preserved.

The final scoped audit covers 140 files, 108 directories, and 231 governed entries with all eight counts zero. `bun nx run os-hub:space-administration-source-check` passes all 31 source checks. `bun nx run os-hub:gis-inference-ledger-oracle` passes 12 inference-catalog-projection checks, then fails in the separately owned GIS controlled-proposal validator on the additional property `proposal/CreateRegion/item/data.id`. The unrelated assertion and payload were not changed.
