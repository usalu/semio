# 📓️ W3-T-SPATIAL Report: Gumball, Transform and Brush Gestures as Tool Machines

Executor W3-T-SPATIAL. Scope: 🎥️shooting, 🏗️fem 2d/3d, 💠️lowpoly, plus the World3d and Canvas2d gumball hosts and the
manifest's gumball bracket rule. Layout moved to W3-T-LAYOUT and generation3d to W3-T2-PROCEDURAL; for both, the
host protocol below stays stable.

Status: IN PROGRESS. This file is rewritten as each slice lands.

## 1. Census (before)

| # | Plugin | Gesture | Entry verbs | How it committed | Verdict |
|---|---|---|---|---|---|
| G1 | shooting | asset gumball drag / rotate / scale | `dragAssets`, `rotateAssets`, `scaleAssets` (+ selection fallback) | one `Emit::amend` per tick under a coalesce key; the host sent `transformBegin`/`transformEnd` around it | tool machine, ONE transaction holding the net relative leaf |
| G2 | fem 2d | Canvas2d gumball translate / rotate / scale | `translateSelection`, `rotateSelection`, `scaleSelection` | absolute whole-record `ReplaceNode`/`ReplaceRegion` per tick, coalesced; region holes never moved | new relative `move-selection` leaf + tool machine |
| G3 | fem 3d | World3d live gumball | same three verbs + empty `transformBegin`/`transformEnd` handlers | absolute `ReplaceNode`/`ReplaceSolid` per tick under `gumball-*` coalesce keys | new relative `move-selection` leaf + tool machine; brackets deleted |
| G4 | lowpoly | World3d gumball | `transformBegin`, the three verbs, `transformEnd` | `LowpolyScratch` transform scratch, then one absolute `CreateMesh` (whole half-edge JSON) via `Emit::commit` | relative mesh transform leaf + tool machine; scratch and brackets deleted |
| G5 | lowpoly | paint stroke (World3d + UV canvas) | `paintStrokeBegin`, `paintAt`/`paintStroke`/`canvasPointerDown`/`canvasPointerMove`, `paintStrokeEnd` | stroke pixel scratch in the transient, then `paintStrokeEnd` diffs whole buffers over a bounded cursor into absolute `EditPaintLayer` runs | relative paint-stroke leaf (points and brush) + tool machine; scratch and brackets deleted |
| H1 | World3dHost | gumball drag | `transformBegin`, pose deltas, `transformEnd` | brackets around per-tick deltas; non-live hosts previewed locally and sent one delta | non-live: one one-shot delta (no `phase`); live (`gumballLiveDispatch`): `phase` stream/commit/abort |
| H2 | Canvas2dGumballOverlay | 2d gumball drag | per-tick deltas | per-tick one-shot deltas | `phase` stream/commit/abort |
| H3 | wgpu World3d | gumball drag | one delta on release | already one one-shot | unchanged (one one-shot transaction) |

## 2. Host protocol (shared by every executor)

- A dispatch without `phase` is a one-shot: ONE tool transaction.
- `phase: "stream"` adds a tick to the window's open transaction (it opens on the first tick).
- `phase: "commit"` folds the last tick in and commits the net leaf. An empty tail commits with an identity delta
  (`gumballIdentityDelta`).
- `phase: "abort"` plus a `reason` drops the open gesture with zero trace. Reasons: `blur`, `captureLost`,
  `baseMoved`, `frozen`, `retired`; an absent reason means `tool`.
- World3dHost aborts on window blur and unmount (`captureLost`) once a tick was streamed. The Canvas2d overlay aborts
  `captureLost` on pointercancel / lostpointercapture / unmount and `blur` on window blur.
- The `transformBegin`/`transformEnd` brackets are gone from World3dHost, fem 3d and puzzle 3d (puzzle via
  W3-T-PUZZLE). Lowpoly is next.

## 3. Design deviation: FEM gesture state is an artifact transient keyed by window

The FEM editors keep the open gumball gesture in an artifact-level, local-only transient
(`FemGumballTransient { gestures: window id → FemGumballGesture }`, defined once in fem 2d and reused by fem 3d) rather
than a window transient. The SIBLING window paints the preview too: the results window re-solves the moved structure
while the drag goes on. Each gesture is keyed by its owning window id, so a host abort clears only the owner's entry,
and two windows never clobber each other. Never history, never shared.

## 4. Leaf naming

`transform` is not an approved semantic verb, so the relative FEM leaves are `move-selection` (verb `move`): one leaf
carries offset, angle (+ axis in 3d) and factors about a pivot, and the label names the motion actually present
("Move 1 node by (0.5, 0)", "Rotate 2 nodes by 90°", "Scale 1 solid by (2, 1, 1)", en + de).
