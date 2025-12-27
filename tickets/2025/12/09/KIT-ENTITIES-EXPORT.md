---
slug: KIT-ENTITIES-EXPORT
summary: Expose kit entities individually from assets index
prompt: Expose kit entities individually from assets index
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.897Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

- `assets/index.ts` only surfaced the Metabolism fixtures and a narrow `MetabolismKitDesigns` filter.
- Documentation did not mention the per-entity exports that the assets package is now capable of providing.

# Plan

- Import the Metabolism kit fixture locally and expose each kit collection (types, designs, ports, qualities, files, folders, authors, tags, concepts, attributes) plus the Nakagin design helper.
- Document the new exports in `README.md` and `AGENTS.md` so the dev docs reflect the current asset entry point.

# Changes

- Added a local Metabolism kit import, the general collection exports, and the Nakagin Capsule Tower helper in `assets/index.ts`.
- Expanded the `@semio/assets` section in `README.md` with a Kits subsection that enumerates the shared exports.
- Noted the new export surface in `AGENTS.md` so the file structure and asset guidance mention the kit tooling surface.
