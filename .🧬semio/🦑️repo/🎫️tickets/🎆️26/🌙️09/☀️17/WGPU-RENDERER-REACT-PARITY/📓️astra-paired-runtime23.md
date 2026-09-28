# Paired Runtime 23

## React Diagnostic Reference — September 27, 21:09 UTC

Root restarted only its port6013 Vite process with `VITE_SEMIO_RUNTIME_DIAGNOSTICS=1` and HMR disabled. The existing Nx launcher confirmed one staged component matches sources and activation receipt. The reloaded tab rendered Concrete Forest and emitted the new accepted-frame console diagnostics. The WGPU23 build is still active, so this is reference-only evidence.

The final Top camera is OrthographicCamera, viewport417.09375×627.265625, zoom6.179166666666666, position `[5.405405345916749,2.3406067869663256,21.35721375944614]`, target `[5.405405345916749,2.3406067869663243,1.5015]`, up `[0,1,0]`. Grid step is10, LOD distance108.847644542974, orbit distance19.85571375944614. Perspective uses FOV50, the same target, eye `[16.69166687085947,-8.945654737976396,9.778091784957995]`, orbit17.979470792924886 and grid step10. Both report content center `[7,0,.005]`, half extent `[25,25,.5]`, and authored reference origin `[7,0,.01]`, width50.

The physical Top screenshot places the reference image at approximately x27..335 (309px wide, center181), while the model centers near214. Given the camera target and reported authored origin, a reference centered at world x7 should appear near225. This suggests the authored origin may differ from the actual reference render transform. Sol is auditing that actual React mesh path; no artificial WGPU offset is authorized by this observation. The grid's visible pitch is approximately62px, consistent with the diagnostic step10 and zoom6.17917.

## Fresh WGPU Publication And First Controls

WASM23 passed and published after15m12s. The existing prepare and activate targets passed; activation reused the same current Puzzle component content. A twelve-file seal records renderer WASM135568013 bytes, SHA256 `74001cb7f85aadb8d3fbb578fac7f5966dbc3aff5d15e2085add9dfd288fd140`; the shared Puzzle core remains122549589 bytes, SHA256 `9c8e4d391b146130d863e82573451cde34f84085a56d5890c84b845d7e4a9821`.

The WGPU server restarted with diagnostics enabled. DOM meta content is1, but page console inspection exposes only Vite logs, no Rust World receipts. Worker logging scope/accepted-receipt dispatch remains under investigation; the meta-only test is not evidence of end-to-end diagnostics.

The loaded WGPU Top view still places the image nearx70..379, matching authored origin7. Independent source audit found the React defect: the pose layout effect ran while media was null and the group absent, and omitted media from its dependencies. Sol owns the narrow React lifecycle fix and deferred-media mounted matrix oracle.

The WGPU first accepted screenshot showed the Perspective full reference plane and small loaded model. After opening only Top Window Options, Perspective displayed the close mesh-fit view. This is not yet a proven permanent fit failure; accepted-frame diagnostics must distinguish normal late completion from a missing invalidation. The Options panel physically shows compact checked controls with the erroneous wide selection outlines removed, consistent with fullUI11's716/716 pass. Projection-pane convergence, hidden-sibling retention, spawned-title verification and exact grid cadence remain open acceptance items.

## Window Focus And Projection Actions

The fresh WGPU focus/unfocus journey passed: Top fills the body, then both prior split rectangles return. Perspective retains its close model/reference view instead of losing the model or resetting its camera; its accessibility owner returns with the same generation2. This physically validates the committed-dock ownership repair.

Opening Projection publishes all15 template rows and marks Orthographic selected in accessibility, but visually paints only blank bordered strips atx120..420,y300..659. The blank retained pane is a real product failure, consistent with the focused native guard failure. Pressing its accessible Curvilinear row does change the native tab title and projection, so action routing succeeds while pane paint remains broken.

The same actual React Curvilinear action reports PerspectiveCamera/FOV120 but keeps the prior orthographic zoom6.1791666667 and Top eye `[5.4054053459,2.340606787,21.357213759]`; target is `[0,0,0]` rather than the prior model target. React Perspective also reported target0 after focus/unfocus, though its eye retained the prior fit pose. These observations explain why mathematically matching source projection formulas do not reproduce the reference camera output. Terra audits camera/control remount ownership; Sol owns a precise React lifecycle repair and mounted oracle. No WGPU zoom/offset workaround is used to mimic accidental stale React state.

The maximized WGPU Top view visibly includes approximately62px grid intervals, with alternating intensity. The earlier124px estimate may have counted only stronger lines. Exact accepted grid receipts are still needed before claiming a shader cadence defect.

## Projection cap correction — 21:24 UTC

A fresh screenshot and AX capture after the earlier physical cap click confirms the Projection pane is folded, with all 15 row controls removed. The previous immediate AX capture lagged the accepted frame. Cap activation works; the independently observed blank row paint remains a real defect.

## Artifact integrity — 21:26 UTC

All twelve pre-journey artifact hashes matched the post-journey seal. No renderer WASM, shared Puzzle component, activation receipt, or bridge artifact changed during the physical observations. Generated seal: `🗑️generated/astra-runtime/gate34/paired-23-postjourney-artifact-seal.json`.

## Runtime 23: Orthographic palette drag

A physical drag from the expanded Display palette Orthographic handle `(291,585)` to the right side of the Perspective body `(1190,360)` created a third window. Its visible tab and AX name are both **Orthographic**, with the matching grid icon and orthographic world view. Existing Curvilinear and Perspective sibling windows remain visible. New AX surface identity: `puzzle3d-main-2-6-0`. This confirms the spawned-title repair in the sealed WASM23 artifact through actual input and screenshot; worker console receipts remain unavailable pending the diagnostics bridge fix.

During setup the exposed AX secondary Expand action for Parallel did not change the row even after a later capture; the actual visible chevron click expanded it immediately. Orthographic itself incorrectly exposed collapsed/Expand despite having no children. These separate accessibility issues were forwarded to SolCanvas.

## React source checkpoint, server 8 — 21:47–21:48 UTC

After the source repairs and restarting only our owned React server, actual screenshots show the Top reference plane at x70…379, matching authored origin7 and the WGPU23 reference position. The settled Top camera still targets `[5.405405345916749,2.3406067869663243,1.5015]`, zoom6.1791666667. Physical Focus→Unfocus restores both panes, and actual console camera receipts now preserve this target in **both** live controls objects; Perspective retains eye `[16.69166687085947,-8.94565473797639,9.778091784957994]`, FOV50, zoom1. This verifies the reference-mount and controls-replacement repairs through runtime behavior. WGPU remains the sealed older WASM23 artifact.

A further mode-transition divergence remains: selecting Curvilinear through the actual React pane produces PerspectiveCamera/FOV120 with the correct target, but retains Top zoom6.1791666667 and eye/distance19.8557137594. WGPU currently re-frames content for this selection with zoom1/distance101.25. Terra is tracing the local menu’s intended camera authority before either implementation is changed.

## Local projection transition authority clarified

Terra traced the actual handler: local kind selection retains the accepted view and never arms `projectionFramePending`; unchanged orientation preserves position, target, up, and zoom through `worldProjectionTransitionPose`. Therefore React’s Curvilinear zoom6.1791667 is intentional current behavior. The WGPU local-action path is the mismatch: it resets perspective zoom and re-arms content framing. SolFlow will separate that local transition from the initial/external delivered-projection framing lane and replace the obsolete native laws.
