# Raster Child Owner Closure

## Outcome

Raster decoded image materialization is now owned by the exact `ArtifactChild<SemioImageSnapshot>` stored in its snapshot. The process-global `RASTER_SCRATCH` map and its cache accessors were removed.

`mint_raster_asset_child` attaches immutable decoded content with `ArtifactChild::with_local_owner`. `raster_asset` resolves only that exact handle's typed local owner. A wire-decoded matching child identity therefore cannot observe another live snapshot's payload.

The old `mint_and_stash_asset` name was removed from all Raster call sites and documentation; no compatibility alias remains.

## Schema-First Test

`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🧪️fixtures/🎯️child-owner-isolation.json` defines the language-neutral observation:

```json
{
  "ownedHasMaterialization": true,
  "wireIdentityMatches": true,
  "wireHasMaterialization": false
}
```

The Rust test constructs an owned child, performs a third-party `serde_json` wire roundtrip, and compares the observed ownership booleans with that fixture through a test-only oracle interface.

## Validation

- Bun JSON/source validation: exit 0.
- Exact stale/global source anchors: absent.
- Official tool-job source census: the process-global payload ledger decreased from 28 to 27; the repository gate remains red for the other declared blockers.
- Rust compilation and the new test remain pending the exclusive compiler lease; this report does not claim runtime green.
