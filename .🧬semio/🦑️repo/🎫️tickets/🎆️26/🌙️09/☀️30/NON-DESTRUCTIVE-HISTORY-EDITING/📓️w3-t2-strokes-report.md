# 📓️ W3-T2-STROKES Report: wfc Bitmap Stroke, Raster Stroke, Remodel Import, Process3d Cursor and World Gestures

Executor W3-T2-STROKES. Scope: audit WP-5 (`📓️audit-remaining-tools.md` §7 row 5, §5.1 wfc/raster, §5.4, §5.5),
minus the wfc 2d/3d node-graph moves (they wait for the flow executor's node-graph record).

Status: IN PROGRESS (census written first, per the W3-T brief step 1).

## 1. Census (before)

| # | Plugin | Gesture / path | Entry verbs | How it commits today | Verdict |
|---|---|---|---|---|---|
| S1 | wfc bitmap | paint stroke on the input sample | `stroke-begin{x,y}`, `stroke-extend{x,y}`, `stroke-commit` | ticks grow a bounding box in the pane's **window config** (`BitmapInputWindowConfig.stroke`, coalesce key `wfc-bitmap-stroke`, one config edit per gesture); commit fills the WHOLE box with the armed colour as ONE absolute `set-input-pixels{x,y,w,h,base64}` + a config write | G: tool machine, stroke scratch in the window transient, parametric stroke leaf |
| S1h | wfc bitmap | host binding | none | no host sends the stroke verbs (React `Canvas2dHost` and wgpu `Scenes` dispatch only `canvasPointerDown/Move/Up` in SCREEN pixels, without the camera); the stroke is reachable from MCP / palette / tests only | host gap, see §6 |
| S2 | raster | brush / eraser stroke on a pixel layer | `editPixels{layerId, expectedImageKey, operation:{kind:"stroke",points≤2048,size,opacity,hardness,color,erase}, selection}` | React `Paint2dHost` keeps the points host-locally and dispatches ONCE at release; the retained work rasterizes, PNG-encodes and publishes `change-layer-pixels`(detach) / `remove-layer-asset` / `add-layer-asset{png}` / `change-layer-pixels`(attach): history edits opaque PNG blobs, the stroke intent is thrown away | G: parametric stroke leaf + one-shot stroke tool |
| S2m | raster | brush / eraser stroke on a layer mask | `editMask{..., operation:{kind:"alphaStroke",...}}` | same shape on the mask asset | G: same leaf, `target: mask` |
| S3 | remodel | host-decoded video import | `importVideoFramePayload` ×N + `importVideoDone` | one `Emit::amend` per decoded frame (`create-asset` + `create-stream` / `add-stream-frame`) under key `remodeling-import:{stream}`, closed by `importVideoDone`; a cancelled import leaves half the frames in the document; per-frame ops are announced live | I: import tool machine, ONE transaction per import |
| S3s | remodel | image-sequence import (file picker, `multiple`) | `importFramePayload{index,total}` ×N | same per-file amend under the same key; no done verb (the last file is `index == total-1`) | I: same machine, commit at the last file |
| S3b | remodel | in-process video bytes fallback | `importVideoBytesPayload` | already ONE `Emit` | O: stamp it as one transaction too |
| S4 | process3d | replay cursor | `setCursor`, `stepCursor`, `stepCursorBack`, `stepCursorForward`, engagement `back`/`forward`/`all` | ONE document edit per click (`change-cursor` leaf on `Process3dSnapshot.resolved_up_to`); `create-step`/`delete-step` builders also move it | V: view state, out of the document |
| S5 | process3d | push/pull face drag (`select` utility) | `worldFaceDragEnd{normal,startPoint,distance,faceExtent}` | host-local drag, ONE dispatch at release, `insert_step_mutations` (+ cursor) as a plain edit | O → one-shot tool transaction (`TransactionRef`) |
| S6 | process3d | click-to-cut / drill / attach | `worldPointerDown{position}` | one plain edit | O → same one-shot tool transaction |
