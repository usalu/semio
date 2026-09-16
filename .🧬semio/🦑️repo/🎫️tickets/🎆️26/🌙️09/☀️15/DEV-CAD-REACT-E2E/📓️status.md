# Status: Dev Cad React E2E

**Status:** closed 2026-09-16
**Goal:** Get dev cad react working end to end (🛠️dev📐️cad⚛️react, port 6020).
**Bookkeeping:** manual on disk — repo MCP failed to connect (connection closed) in both sessions.

## Result
- Build → activate → serve → boot → real Concrete Forest geometry + reference image in all four panes → pick/hover through the `cad` domain (viewport and Artifact tree) → inspection of the selection → reference mutations (width/origin/lock) landing in the store with undo → Dislocate gumball arming. Full report: [cad-react-e2e-2026-09-16.md](./📓️cad-react-e2e-2026-09-16.md).
- Native: `semio-s-artifact-cad-cad` 297 passed / 21 failed (all framework test-harness debt, itemised in the report); `semio-s-plugin-cad` 5/5.

## Open
- Object mutations (translate/rotate/scale/patch/add/delete) stay documented no-ops until the composed-child re-materialisation seam lands (peer work in `🏪️store` composition).
- Document-tree paging has no `setPanelPage` navigation yet (plain `+N` continuation).

## Notes
- 2026-09-15: [dev-cad-react-e2e-2026-09-15.md](./📓️dev-cad-react-e2e-2026-09-15.md), [cad-ui-surface-capacity-2026-09-15.md](./📓️cad-ui-surface-capacity-2026-09-15.md) (superseded).
- Probes: [🐍️console-dump-probe.mjs](./🐍️console-dump-probe.mjs), [🐍️cad-interact-probe.mjs](./🐍️cad-interact-probe.mjs), [🐍️cad-tree-reference-probe.mjs](./🐍️cad-tree-reference-probe.mjs), [🐍️cad-utility-probe.mjs](./🐍️cad-utility-probe.mjs); serve/restage: [📜️serve-cad-react.sh](./📜️serve-cad-react.sh), [📜️activate-cad-react.sh](./📜️activate-cad-react.sh).
