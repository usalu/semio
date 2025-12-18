---
slug: KIT-ENTITIES-LOOKUP
summary: Export individual kit entities
prompt: Export individual kit entities
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-16T17:06:07.897Z'
commit: '0000000000000000000000000000000000000000'
iterations: []
---

# Previously

- `assets/index.ts` already exports the Metabolism fixtures and helper constants but consumers still had to filter for specific entities.
- The docs mentioned the arrays but not the direct lookup helpers that were now available.

# Plan

- Generate lookup maps for designs, types, and interfaces in `assets/index.ts`.
- Announce the lookups in README and AGENTS so devs can discover the per-entity entry points.

# Changes

- Added `buildLookup` helper plus `MetabolismKitTypesByGuid`, `MetabolismKitTypesByName`, `MetabolismKitDesignsByGuid`, `MetabolismKitDesignsByName`, `MetabolismKitInterfacesByGuid`, `MetabolismKitInterfacesByName` exports.
- Documented the lookup tables in `README.md` and `AGENTS.md`.
