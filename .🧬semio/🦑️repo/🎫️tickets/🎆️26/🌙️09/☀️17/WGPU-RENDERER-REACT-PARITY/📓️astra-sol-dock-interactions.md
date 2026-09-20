# Physical Docking Acceptance Adapter

## Result

Added a ticket-owned, schema-first Playwright adapter for the missing physical docking matrix. It covers four body split zones, tab merge, tab reorder, Escape cancellation, and a Display-template transfer that must retain the orthographic top-view configuration.

The adapter sends only Playwright mouse and keyboard input to the app. It does not call app actions or mutate app state through page JavaScript. Page evaluation is limited to reading React DOM state, reading the existing WGPU diagnostics beacon, and the isolated `test` command's neutral trusted-event oracle.

Files:

- `🔬️dock-interactions/🧬️schema/🔣️.json`
- `🔬️dock-interactions/🧫️fixtures/🔣️.json`
- `🔬️dock-interactions/📜️script.ts`

## Contract

Each case starts in a fresh browser context so a committed layout cannot make a later case pass accidentally.

| Case | Physical input | Required consequence |
| --- | --- | --- |
| `split-left` | Tab handle drag to the left body zone | Source is a lone active stack left of the target; identities preserved |
| `split-right` | Tab handle drag to the right body zone | Source is a lone active stack right of the target; identities preserved |
| `split-top` | Tab handle drag to the top body zone | Source is a lone active stack above the target; identities preserved |
| `split-bottom` | Tab handle drag to the bottom body zone | Source is a lone active stack below the target; identities preserved |
| `tab-merge` | Tab handle drag after another tab | One fewer stack, target-before-source order, source active |
| `tab-reorder` | Physical merge followed by a second handle drag | Source moves before target with the same window and stack sets |
| `escape-cancel` | Promote drag, press Escape, then release pointer | Exact topology and near-exact geometry restored; release commits nothing |
| `template-configuration` | Display tree transfer handle drag after a tab | Exactly one window added in target stack with orthographic projection, direction `[0,0,1]`, and up `[0,1,0]` |

React grounding is `computeModeDropZone`, `applyModeDrop`, `ModeDockTabBar`, and `COMPOSE_WINDOW_TEMPLATE_MIME` in `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx`. WGPU grounding is `compute_dock_drop_zone`, `DockState::apply_drop`, and the exact `dock.tab.<path>.<window>.drag` hit route in the Dock and Shell WGPU implementations. The WGPU Display roster exposes the `orthographic` template as an orthographic top camera; React carries the equivalent configuration in its `world-projection:` template value.

## Fail-closed host assumptions

The probe fails rather than substituting discovery when any required surface is unavailable:

- The selected host must expose at least two windows in at least two visible, single-tab stacks after introduction dismissal.
- React must publish `mode-dock-stack`, `mode-dock-tab`, physical `drag-handle`, and camera data attributes. The handle centre must resolve to its owning tab.
- WGPU must expose armed `dumpChrome` and `dumpMeshStats` diagnostics, exact `.drag` hits, physical window body hits, and a camera record for the created window.
- The Display category must expose an expandable window-kind tree and the exact `projection.parallel.orthographic` transfer row.
- A promoted drag must temporarily dock the source window out before any drop or Escape assertion is accepted.
- Every commit must preserve the original window identity set, except the template case, which requires exactly one new identity.
- Split direction is verified from resulting body geometry; merge and reorder are verified from resulting tab order. Screenshots and incremental receipts are written only by an app run under `🗑️generated/dock-interactions/<renderer>`.

## Validation

The neutral contract validation passed. Its independent Chromium page observed trusted `dragstart`, `dragover`, `drop`, pointer, keyboard, and pointer-release events; the configured MIME payload survived the physical HTML drag, and Escape left zero committed pointer actions.

```sh
bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️dock-interactions/📜️script.ts' test
```

Result: `PASS`, all eight schema cases, trusted HTML drag and pointer cancellation, `cancelledCommits: 0`.

```sh
bun x tsc '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️dock-interactions/📜️script.ts' --noEmit --module esnext --moduleResolution bundler --target es2022 --allowImportingTsExtensions --skipLibCheck
```

Result: exit `0` with no diagnostics.

Run against already-active canonical hosts with:

```sh
SEMIO_DOCK_BOOT_MS=180000 bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️dock-interactions/📜️script.ts' run react 'http://127.0.0.1:6313/?plugin=puzzle3d'
SEMIO_DOCK_BOOT_MS=180000 bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️dock-interactions/📜️script.ts' run wgpu 'http://127.0.0.1:6213/?plugin=puzzle3d'
```

Those app runs were intentionally not executed while checkpoint 16 owned activation. The green result above validates the fixture, schema, type boundary, and trusted browser mechanics; it is not a React/WGPU physical parity claim.
