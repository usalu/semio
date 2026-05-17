# Metabolism Light Fixture Node And Handle Kind Ids

## Summary

- **`semio/assets/fixtures/metabolism.kit.light.semio.json`**: Each of the 50 `wip.initialKit.types.items[]` rows has `nodeKind` = `semio.metabolism.light.node.<typeUuid>`. Each canonical port under `families[].ports.items[]` (18) and each connector inline `port` object has `handleKind` = `semio.metabolism.light.handle.<portUuid>`.
- **`seed.metabolism.light.kit.neo4j.ts`**: Neo4j seed now sets `Type.nodeKind` and `Port.handleKind` from the fixture when present.
- **`semio/client/lib/rs/lib.rs`**: Contract test `metabolism_light_fixture_kinds_for_types_and_ports` locks the id patterns.
- **`apply-node-handle-kinds.mjs`**: Idempotent transform in this ticket folder (re-run if the fixture is regenerated without kinds).

## Files

| Action | Path |
|--------|------|
| updated | `semio/assets/fixtures/metabolism.kit.light.semio.json` |
| updated | `seed.metabolism.light.kit.neo4j.ts` |
| updated | `semio/client/lib/rs/lib.rs` |
| created | `.repo/🎫/26/05/17/METABOLISM-LIGHT-FIXTURE-NODE-AND-HANDLE-KIND-IDS/apply-node-handle-kinds.mjs` |
