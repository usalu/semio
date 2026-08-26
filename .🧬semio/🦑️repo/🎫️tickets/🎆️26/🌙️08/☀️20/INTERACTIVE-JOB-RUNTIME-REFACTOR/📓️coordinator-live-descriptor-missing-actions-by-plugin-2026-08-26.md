# Live Descriptor Action Disposition Cohorts

Date: 2026-08-26  
Method: metadata-derived read of every `✏️s/🔌️plugins/**/🔣️descriptor.json`, counting `manifest.apps[].windowKinds[].actions[].semantics.execution.interactiveJob`.  
Scope: descriptor disposition only. This is not production-command admission, runtime reachability, or end-to-end acceptance.

## Result

The live descriptor set has 4,754 action rows: 1,973 declare `interactiveJob = "migrated"` and 2,781 remain without that exact disposition. This reproduces the authoritative all-app action census and provides file-disjoint execution cohorts.

| Plugin | Actions | Migrated disposition | Missing disposition |
| --- | ---: | ---: | ---: |
| norm | 675 | 0 | 675 |
| flow | 243 | 0 | 243 |
| architect | 208 | 0 | 208 |
| space (`pluginId = s`) | 204 | 0 | 204 |
| remodel | 184 | 0 | 184 |
| shooting | 127 | 0 | 127 |
| lowpoly | 123 | 0 | 123 |
| note | 123 | 0 | 123 |
| sequence | 121 | 0 | 121 |
| forms | 103 | 0 | 103 |
| raster | 96 | 0 | 96 |
| layout | 85 | 0 | 85 |
| imperative | 84 | 0 | 84 |
| gis | 79 | 0 | 79 |
| dag | 75 | 0 | 75 |
| vcs | 69 | 0 | 69 |
| draw | 58 | 0 | 58 |
| mathematical | 53 | 0 | 53 |
| reasoning-mindmap | 41 | 0 | 41 |
| writer | 48 | 18 | 30 |
| energy | 54 | 54 | 0 |
| fem | 152 | 152 | 0 |
| sourcing | 141 | 141 | 0 |
| procedural | 446 | 446 | 0 |
| animate | 49 | 49 | 0 |
| demonstrator | 808 | 808 | 0 |
| cad | 241 | 241 | 0 |
| process | 64 | 64 | 0 |

The four CAD extension descriptors declare no independent action rows and remain parent-activation surfaces rather than zero-evidence exemptions.

The largest cohort is already naturally file-sliced: Norm has 27 Rust files containing `app_commands!`/roster references across its standards, while Flow has seven. Norm's 675 rows are 30 app contexts (15 editor + 15 viewer) with 32 editor and 13 viewer action placements, but only 16 unique action IDs. Three are owner-local (`setSnapshot`, `evaluate`, `setSelectedCheckIndex`); the other 13 are repeated shared shell/history/clipboard/tutorial routes. This means the cohort should be closed by proving the three local retained command families once per standard plus proving the shared routes once at their shared owner, then joining every repeated descriptor placement. Norm's repeated `set-snapshot` modules and existing pre-migration byte fixtures still require semantic retained-job audit; the compression is an ownership map, not permission for a descriptor-only bulk edit.

Across all 2,781 missing placements there are 359 unique IDs: 64 occur in at least two plugins and 295 are plugin-local. Thirteen shared routes recur across 19–20 plugins (`checkoutCheckpoint`, `commitCheckpoint`, `copy`, `createAlternative`, `cut`, `noteShellCommand`, `paste`, `redo`, `recordTutorial`, `revertToCommand`, `setHistoryCommandFilter`, `switchAlternative`, `undo`). Six selection/interaction routes recur across 18 plugins (`clearSelection`, `interactionHover`, `interactionSelect`, `selectAll`, `setInteractionGranularity`, `setSelectionMode`). These 19 routes are the highest-leverage shared-owner acceptance packet; per-plugin wrappers would violate the domain-neutral shared-route architecture.

The last accepted shared-route ledger already contains seven of those action IDs (`undo`, `redo`, `commitCheckpoint`, `createAlternative`, `switchAlternative`, `checkoutCheckpoint`, `revertToCommand`) plus the non-action `configuration-binary` route. The 12 still-unaccepted high-reuse action IDs account for 934 of the 2,781 missing descriptor placements: 114 each for `copy`, `cut`, `paste`, `noteShellCommand`, and `setHistoryCommandFilter`; 112 for `recordTutorial`; and 42 each for the six selection/interaction routes. Acceptance still requires both shared production ownership and regenerated descriptor disposition; route admission alone cannot erase the disposition gate.

## Acceptance warning

Adding `interactiveJob = "migrated"` is never sufficient by itself. Each row must join to an owner-local accepted production command or an accepted shared reserved route, and every expensive command must satisfy retained instance ownership, bounded semantic steps, cancellation/freshness, preview/progress, close/handback, hostile fixtures, native/Wasm parity, and real app reachability. Cohorts above are therefore ordered implementation inventory, not a mechanical descriptor-edit queue.
