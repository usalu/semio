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

The same source gate passed through Nx. The exact native law is registered as `artifacts::gismap::standards::v1::subsets::any::schema::inferences::tests::map_create_region_group_work_stabilizes_parent_drawing_value_without_image`.

The September 5 native attempt completed **RED before the selected test ran**. The exact-law build at `🗑️generated/gis-map-create-region-group-exact/exact-cargo-laws-2hMTLS/00` failed compiling `semio-s-plugin-stdio`: 131 errors, with a missing XML protocol include and 10 mutation-owner authority failures causing 120 downstream trait-bound errors. No native pass or typed runtime behavior is claimed. Those diagnostics are being checked against the concurrently changing worktree before a rerun.

## Image Membership Follow-up

Read-only review found misleading helper documentation: a supplied typed image member is preserved, rather than cleared. This is now documented explicitly. Preserving the handle avoids silently discarding artifact membership; the current image-free proposal scope continues to deny that snapshot without mutating it.

The neutral corpus now has four actual membership scenarios: accepted image-free input, forged drawing identity, forged value identity, and a supplied image passed through the derived-children helper. AJV and the independent JavaScript predicate agree on all four. The native law consumes the same corpus, creates a real typed image handle, checks preservation through derivation, verifies rejection reasons, and asserts unchanged input snapshots. The native additions have not yet run.

TDD: the registered Nx source target first failed with `GIS Map native law does not consume membership cases`; after adding the native corpus consumer it passed with **26 checks**. The ordinary root `bun nx` wrapper was temporarily blocked by an unrelated `schema/🌍️change-annex` taxonomy collision. The same registered task was executed through `bun x --no-install nx`, still using Bun and Nx, without changing or disabling taxonomy validation.

## Remaining Durable Boundary

The next MAP packet must open the stable child members through the verified selected member factory, attach their live stores and generations, then feed this work into a prepared durable group visibility transaction. No caller may advertise this planning result as committed or atomic before the durable parent-plus-child receipt exists.

The exact Map work requires **three** participants (parent, drawing, value). A two-member kernel pilot does not establish atomic execution of this proposal. See `📓️terra-map-durable-group-current-frontier.md` for the current port/WAL/read-root gaps and proposed crash/recovery laws.
