# Layout Canvas2d catalogue pointer transfer

## Proven gap

The fresh Layout React physical adapter reached the catalogue transfer handle and proved that a real primary-pointer gesture armed the row's draggable ancestor. The source Tree intentionally prevents native HTML drag on that pointer path and records the encoded catalogue payload in `catalogueTreeDragController.pointerPaletteDrag`. `Canvas2dHost` listened only to native `dragover`, `dragleave`, and `drop`, so the pointer journey produced ordinary `canvasPointerMove` actions but never `canvasDragOver` or `canvasDrop`.

Evidence:

- `📓️astra-layout-physical-adapter.md`
- `🗑️generated/astra-runtime/layout-react-4/layout/react/failure.json`

## Neutral contract

The schema and fixture under `ui/{🧬️schema,🧪️fixtures}/🛒️canvas-catalogue-pointer-transfer` define three bounded terminal cases:

- release inside the canvas publishes over, leave, then drop;
- leaving and releasing outside publishes over then one leave;
- cancellation publishes over then one leave and no drop.

Every point is a client-space point against a nonzero surface origin. The expected drop preserves the raw catalogue payload and converts client coordinates to the exact surface-local point.

Native HTML drag remains a separate input. Its existing mounted test still drives a real `DataTransfer`-shaped event and validates the same leave-before-drop terminal order.

## Fail-first and repair

Focused mounted React execution first produced 18 passing tests and three intended failures. Each pointer-transfer case received no actions.

The repair gives pointer transport its own ephemeral payload owner beside the existing general/native catalogue payload. A pointer begin publishes both; pointer cancel/end retires both; native drag start explicitly leaves the pointer owner empty. `Canvas2dHost` listens for the pointer-owned payload at the window boundary:

- pointer move inside the mounted host publishes the existing throttled `canvasDragOver` shape with the canonical MIME;
- moving outside retires an existing preview exactly once;
- pointer up inside captures raw payload, bounds, and coordinates synchronously, then dispatches `canvasDragLeave` before `canvasDrop`;
- pointer up outside or pointer cancel emits only terminal leave when a preview exists.

The existing native handlers and the pointer handlers share the same over and terminal helpers, so raw payload and coordinate semantics cannot drift.

Terra's listener-order review found a second, narrower race after the first repair: the mounted canvas window listener precedes the Tree gesture's dynamically installed listener. The first move that crosses Tree's six-pixel threshold therefore reaches the canvas before Tree publishes the pointer payload. An immediate release originally emitted only `canvasDragLeave, canvasDrop`.

The added mounted law renders a real `Catalogue` transfer handle beside `Canvas2dHost`, sends exactly one threshold-crossing move into the canvas, and releases at that same point. Its fail-first receipt was 21 passing and one failing test, with actual actions `canvasDragLeave, canvasDrop` instead of `canvasDragOver, canvasDragLeave, canvasDrop`. The terminal helper now captures the payload, bounds, and point synchronously and, only when the pointer path has no prior preview, awaits `canvasDragOver` at that exact terminal point before leave and drop. Preview refusal still reaches the nested terminal cleanup, and native HTML drop does not synthesize a preview.

Focused command:

```sh
SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun x vitest run '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🧪️tests/🖱️input-contract/🟦️.tsx' --config '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts'
```

Final receipt: 22 passed, 0 failed. This includes the actual Tree listener-order law plus the three direct ownership terminal cases. This is a mounted jsdom transport proof using Testing Library and Ajv. A fresh activated Layout browser journey remains root-owned; no physical runtime pass is claimed here.

## Draw diagnostic retirement

Draw10 captured both bounded pointer records with `shapeRect`, distinct correct bounds, and `cancelled: false`. That evidence remains under `🗑️generated/astra-runtime/draw-react-10`; the temporary `Canvas2dHost` diagnostic and its Shell diagnostics dependency are removed from production source.
