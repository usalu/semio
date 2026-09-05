# GIS Map Typed CreateRegion Group Work

## Outcome

The GIS Map inference boundary now prepares one bounded, typed parent+drawing+value work owner for its bounds `CreateRegion` proposal. Map drawing and value child member identities are stable across parent edits; the proposal emits one `SemioDrawingMutation::CreateNode` and one `SemioValueMutation::InsertListItem`, retains their exact inverses, and rejects any parent projection that would change child membership or touch the optional image slot.

This is intentionally the domain planning half of the MAP composition path. It does not claim approval, durable atomic publication, hub fanout, or ledger reconciliation.

## Owned Source

- `✏️s/🔌️plugins/🌍️gis/🧪️fixtures/🧩️map-create-region-group/{🧬️.schema.json,🔣️.json}`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/{🦀️.rs,📸️snapshot/🦀️.rs,💡️inferences/🦀️.rs}`
- `✏️s/🔌️plugins/🌍️gis/🧪️fixtures/💡️inference-control/🔣️.json`
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/{📜️script.ts,📋️project.json}`
- `.vscode/🧩️launch.seed.jsonc`

## Invariants

- Drawing member: `gismap-drawing`; value member: `gismap-value`; neither is content-re-minted.
- The parent proposal embeds its feature identity in both `MapFeature.id` and the descriptor payload.
- `ring` geometry is consumed by the same typed drawing bridge as existing `points` geometry.
- The drawing after-state must equal the before-state plus exactly one final root child; canvas, styles, layer metadata, transform, and all prior children are unchanged.
- The value after-state must equal one typed append to the `regions` list.
- Parent, drawing, and value inverses are retained before publication.
- Canonical typed work is rejected above 64 KiB.
- Any image member, forged stable member, projection mismatch, stale proposal, or out-of-bounds proposal fails before a group can be returned.

## TDD Evidence

The neutral command was registered and run before the Rust owner existed. It failed with:

```text
GIS Map typed group owner missing GisMapCreateRegionGroupWorkV1
```

After the implementation, the independent AJV/Bun oracle passed:

```text
gis-map-create-region-group-check: checks=22 clean; atomic durable publication not claimed
```

The same source gate passed through Nx. The exact native law is registered as `artifacts::gismap::standards::v1::subsets::any::schema::inferences::tests::map_create_region_group_work_stabilizes_parent_drawing_value_without_image`; its final receipt is pending in the active compile at the time of this checkpoint.

## Remaining Boundary

The next MAP packet must open the stable child members through the verified selected member factory, attach their live stores and generations, then feed this work into a prepared durable group visibility transaction. No caller may advertise this planning result as committed or atomic before the durable parent-plus-child receipt exists.
