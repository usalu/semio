# Flow Content Through Glass Chips

## Infrastructure

The repo MCP server is not registered in this Codex session: `repo://goals`, `ticket_open`, `ticket_reopen`, and `ticket_close` are unavailable. The on-disk goal catalog was inspected instead. This ticket is associated with the open `🎯r2603` release goal and must be closed through repo MCP when that surface becomes available.

## Ownership

| Lane | Exclusive implementation ownership |
| --- | --- |
| Geometry | Shared silhouette schema/contract, Chrome geometry region, parity fixtures |
| React | WindowChrome, Canvas Mode dock, Window consumers, React inline tests |
| WGPU | DrawList mask/clip, WGPU Dock/Shell geometry, hit testing, Rust tests |
| Harness | Styling fallbacks/tests, existing stories, Storybook adapter, root script/launch/CI |

The coordinator integrates and verifies. No agent uses modifying git commands. Existing concurrent edits are preserved.

## Locked Contract

- Active content spans the complete silhouette and is clipped to body plus owned chips.
- Every visible tab and controls chip samples the same active content plane.
- Gaps contain neither payload, glass, nor window hit regions.
- Edgeless scenes are full-bleed; document content clears chrome at rest and scrolls beneath it.
- Geometry is the single source for content clipping, border, glass regions, containment, and safe clearances.
- Pending geometry exposes only a conservative body region.

## Validation Evidence

Lane reports, command logs, screenshots, and final verification evidence belong in this directory.

## Completion

- Shared geometry, React composition, WGPU composition, styling/accessibility, fixtures, and permanent test orchestration are integrated.
- The combined working-tree diff passes `git diff --check`.
- Ticket-local Cargo and Storybook build outputs are removed; emoji-prefixed output roots are now ignored without masking source `🎯️targets/` directories.
- Verified feature gates and remaining repository-level blockers are recorded in `📓️final-summary.md`.
- The ticket remains open only because repo MCP is unavailable in this session; it must be closed through `ticket_close` when that surface is restored.
