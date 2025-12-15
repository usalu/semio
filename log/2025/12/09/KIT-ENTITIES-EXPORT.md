---
date:
  created: '2025-12-09T10:21:07.262Z'
  updated: '2025-12-09T10:21:07.262Z'
slug: KIT-ENTITIES-EXPORT
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Expose kit entities individually from assets index
model: gpt-5.1-codex-max
prompts: []
commit: unknown
affectedFiles: []
lines:
  added: 0
  removed: 0
---

# Previously

- `assets/index.ts` only surfaced the Metabolism fixtures and a narrow `MetabolismKitDesigns` filter.
- Documentation did not mention the per-entity exports that the assets package is now capable of providing.

# Plan

- Import the Metabolism kit fixture locally and expose each kit collection (types, designs, interfaces, qualities, files, folders, authors, tags, concepts, attributes) plus the Nakagin design helper.
- Document the new exports in `README.md` and `AGENTS.md` so the dev docs reflect the current asset entry point.

# Changes

- Added a local Metabolism kit import, the general collection exports, and the Nakagin Capsule Tower helper in `assets/index.ts`.
- Expanded the `@semio/assets` section in `README.md` with a Kits subsection that enumerates the shared exports.
- Noted the new export surface in `AGENTS.md` so the file structure and asset guidance mention the kit tooling surface.
