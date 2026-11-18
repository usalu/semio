# Dependency ready identity under changed installer inputs

## Root cause
`runJavascriptDependencyStoreGenerationsV1` first calls `reclaimJavascriptDependencyPreservationsUnderPressureV1`. Under pressure it emitted the `reclaiming` event and ran `retireJavascriptDependencyPreservationsInStoreV1`, which calls `readJavascriptDependencyCurrentBindingsV1` (storage/🟦️.ts line ~185).
That reader demanded `generation === planned.generation` and that the receipt text equal the receipt derived from the CURRENT inputs. After a peer edited `nx@23.2.0.patch` the planned generation is new, the linked generation old, so it threw "Current complete Source owner ready identity required" before the preservation capsules (~20 GiB) were reclaimed. The generation reclaim itself never reached the reader.

## Fix
`readJavascriptDependencyCurrentBindingsV1(..., identity = "planned" | "recorded")`. `recorded` accepts the generation a link holds by its own complete ready receipt (prefix matches the link target, scope matches, receipt schema valid), takes the bindings from that receipt (must match the planned link paths), and skips the planned-execution verification. The preservation retirement passes `"recorded"`; the publication/selected-retirement callers keep the strict default.

## Files
- storage/🟦️.ts (reader and retirement call)
- storage/🧪️tests/👓️reading/🟦️.ts (new test "recorded identity observes a generation superseded by changed installer inputs")

## Follow-up: why the dependencies image stayed full
- The "6 kept" generations were not superseded pins: they are the 6 CURRENT generations (one per scope pointer), correctly kept. `readdir` of `m/j` showed 12 generation dirs plus 66 `.p<8hex>` capsules.
- The image content is 34.6 GiB: `j` 31.25 GiB (6 current generations about 11 GiB plus 14 capsules of 1.5 to 1.7 GiB, about 20 GiB), `bun-cache` 3.28 GiB. The usage measure (hardlink-deduped, allocated bytes) is correct; the figure agrees with a per-directory walk.
- The big `.p*` entries are retention capsules written by the lock path (`retain-current-unready-dependency-generation`): every interrupted lock or sync moves its never-ready generation aside as `i`/`p`/`r`. Only the setup-all selected-retirement command ever removed them. The collector listed only `^[a-f0-9]{20}$` names, and the preservation retirement walks capsules beside Source `node_modules`, not inside `j`, so neither could free them.
- Pins: there are no stale pins involved. Pointers have empty `retired` lists, so rollback keeps nothing, and the pressure grace is 0 ms.
- Image size: 36 GiB is `dependencies.capacityBytes` (38654705664) in `💾️storage/🔒️enforcement/📦️budget/🔣️policy.json`. The 31.23 GiB quota is the machine-wide weighted split (host free-space floor lowers the effective total, see `🧾️ledger/🎚️machine`). The 8.06 GiB to evict is used (34.61) minus the content limit of that quota; the accounting is right, the excess was the capsules. No quota bug found.

## Fix 2: collector retires unready retention capsules
- `🚦️concurrent/🟦️.ts`: `concurrentUnreadyCapsulePatternV1` and pure `concurrentUnreadyCapsuleRetirableV1` (exactly `i`,`p`,`r`; intent purpose, `readyAbsent`, directory and receipt names agree; generation not current; no `bindings.json` in the payload).
- `💾️storage/🟦️.ts` `collectJavascriptDependencyGenerationsV1`: qualifying capsules become collection rows (grace 6 h, 0 under pressure, 16 per run), are retired through the existing tombstone path (`.x.p<8hex>`, stale tombstones swept at the next start), under an exclusive pin with a synthetic per-capsule name.
- Test: `🚦️concurrent/🧪️tests/🟦️.ts` "concurrent collector retires only complete non-current unready retention capsules" passes. In the same file 4 tests fail that do not touch this change: the Python oracle, pointer switch, wiring laws (counts 3 `acquireConcurrentLeaseV1("bindings"` where 1 is expected, from the earlier reclaim edit) and maintenance (passes alone, fails in the full run).

## Direct capsule retirement (script `retire-capsules.ts` in the ticket folder)
Used the same predicate, current-pointer check, exclusive pin and tombstone-and-remove path as the collector, restricted to capsules older than 6 h. Result: 52 capsules removed, 11 younger than 6 h left alone, 3 not qualifying or pinned. C: free space went from 5.3 GiB (df, before) to 24.17 GiB (after).
