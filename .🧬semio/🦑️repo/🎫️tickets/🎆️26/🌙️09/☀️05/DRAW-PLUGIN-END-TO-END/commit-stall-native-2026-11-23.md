# Commit stall, native reproduction attempt (2026-11-23), stopped early

## Status
Stopped at the coordinator's request (memory pressure). The 4096-unit stall was NOT reproduced natively and no fix was made for it.

## Browser parameters found
`🧪️tests/🔬️mounted-grant-archive-publication/🧫️fixtures/🔣️.json` row `browser-mounted-grant`: items 64, copy 32768, capacity 16 MiB, release 1 MiB, depth 64, fuel 2,000,000. INTERACTIVE_COMPONENT (copy 524288, depth 4096) is a different row.

## Native runs (Draw `addLayer`, crate semio-s-artifact-draw-drawing-editor-tests, run from the `drawing` dir)
1. Turn grant narrowed to 512 KiB copy / depth 64: settles only the publication; the maintenance ladder then stops. The demand copy 688784 / capacity 344448 exceeds the 511280 grant. `maintenance_ladder_unit_step` widens a loan only up to `cx.retained_grant()`, so the unit is skipped with `consume_fuel(1)` and no wait cause. This is a real silent-spin shape, but it is the maintenance ladder, not the publication unit.
2. Browser-sized turn grant (7.1M items, 2 GB copy, depth 64) with INTERACTIVE policy: test PASSES.
3. Policy = browser-mounted-grant (32 KiB), armed host grant: harness refusal "preparation receipt grant exceeds its original caller authority" (test-only check; not the stall).
4. Policy = browser-mounted-grant, no armed grant: never completed. Builds repeatedly failed on other agents' in-flight edits (source boundary of decl-app and value-native-owned, pdf crates) and on ENOSPC.

## Changes (compile status UNVERIFIED)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/📃️pending/🦀️.rs`: `PendingPublicationPage::phase_witness`.
- `.../📃️mounted/🦀️.rs`: `MountedPagePublication::phase_witness`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs`: after `advance_pending_result_page`, bump `publication_progress` when the page ladder phase changed (stall-witness fix); otherwise note "ladder moved nothing". The `note_publication_wait` instrumentation is untouched.
- Test added: `add_layer_settles_under_the_browser_grant` in `✏️s/🔌️plugins/🖍️draw/.../✏️editor/🧪️tests/🔬️unit/🖼️canvas-laws/🦀️.rs` (policy = browser-mounted-grant row). Never observed to finish; may fail for reason 1.

## Not done
Root cause fix for the 4096 stall. Idea if it recurs: record a loan shortfall in `close_original_admission` and widen the next unit's loan with `covering_grant` up to the outer cx grant.
