# Semantic Module Refactor Lease Snapshot

## Control Plane

- Governing goal: `🎯aioptimizedrepo/🎯singlefilerepo`.
- Ticket: `DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` is open.
- The checked-in repository MCP launcher completed the `2025-06-18` handshake and served `repo://goals` through raw stdio. The managed connector transport remains unavailable; no tracked infrastructure change is required.

## Quarantined Dirty Paths

The following paths were dirty when the Wave 0 lease was recorded. They are not writable by this refactor until their owner releases them and their diff is reread.

| Path | Added | Removed | Lease disposition |
| --- | ---: | ---: | --- |
| `.🦑️repo/💬️prompts/🐙️ueli.md` | 13 | 0 | User-owned; excluded from scope |
| `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` | 9 | 3 | Quarantined |
| `🧰️framework/🔨️modules/🔄️machine/🟦️component.ts` | 25 | 8 | Quarantined |
| `🧰️framework/🔨️modules/🖥️platform/🟦️component.ts` | 15 | 5 | Quarantined |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🟦️boot.ts` | 9 | 3 | Quarantined |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Canvas2dHost/🟦️component.tsx` | 15 | 5 | Quarantined |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/WorldTerrainLayer/🟦️component.tsx` | 9 | 3 | Quarantined |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts` | 26 | 9 | Quarantined central export |

## Active Leases

- `central-taxonomy-wave-1`: Sol coordinator owns taxonomy, discovery, command, validation, and local test changes. It must not write the quarantined library export.
- `gltf-discovery`: Luna performs read-only graph and responsibility classification.
- `active-scope-census`: Luna performs read-only owner and validation census.

The initial snapshot intentionally has no timestamp so it remains a stable control record.
