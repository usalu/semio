# Checkpoint 18 Settings Presentation

## Finding

The missing Settings panel in checkpoint 18 steps 10–16 is a stale presented frame, not an incorrect current World3d reservation and not evidence that the World pass paints over a current Settings body.

The retained control snapshot at step 12 already has the new geometry. `puzzle3d-main-perspective` is `[437.6, 60.8, 849.59985, 907.2]`, so its right edge is `1287.19985`. The Settings document begins at `x=1300`: its toggle and all four NumberSteppers occupy width `293.59998`. The two current authorities do not overlap.

The screenshot still shows the earlier full-width World frame through the rightmost 300 pixels. This is the previously presented frame remaining on the surface while the retained registry has advanced.

## Runtime sequence

The console and journey receipts establish the transition:

- Step 10 still reports the pre-panel perspective World at `[539.73334, 60.8, 1053.8666, 907.2]`, ending at `1593.59994`.
- Step 11 waits 15,015 ms and does not yet observe the four app controls.
- At `259787`, frame generation 10 remains blocked in `Uploads`. It becomes terminal at `260038`.
- Generation 11 is admitted at `260087`.
- The next World build records the reserved geometry: top `422x907+6,61` at `261382`, perspective `850x907+438,61` at `262594`.
- `puzzle3d.panel.settings` ingresses at `263245`; the engine surface registry includes it at `269905`.
- Step 12's retained registry contains the Settings toggle, all four NumberSteppers, and the reserved `849.59985`-wide perspective World, but its screenshot still shows the old full-width frame.
- Generations 12–16 are admitted between `270035` and `270326`. Generation 16 does not report its blocked state until `349600` and reaches terminal at `349722`.
- Step 17 waits 15,173 ms and its screenshot finally paints the Settings body at `x≈1298..1597`, while the World ends at the reserved boundary. The same retained geometry was already present in the earlier step snapshots.

This sequence matches the missing Worker continuation defect: logical layout and hit publication can advance while the surface keeps the last presented packet. Once a new packet completes, the correct reservation and panel both appear without a geometry or paint-order change.

## Paint-order source check

The retained DrawList records each scene pass with `layer_index`, `ui_watermark`, and `vector_watermark`. The batch renderer consumes those watermarks in `render_interleaved_layers`, drawing the UI prefix, then the World pass, then the remaining UI in that layer.

The prepared scalar command traversal currently visits layer scalars before scene-pass scalars. That is a separate ordering asymmetry worth a bounded regression if overlapping same-layer UI is introduced or physically reproduced. It does not explain this Settings receipt: the current World scissor ends before the Settings document begins, and the later completed frame visibly paints the panel correctly. No renderer production change is justified from checkpoint 18 alone.

## Verification boundary

The worker-wake packet must be validated by checkpoint 19 on a fresh bundle. The decisive receipt is that opening Settings both publishes the retained controls and presents the reserved World geometry plus Settings body in the same settled step. If the current frame still omits the panel after the worker continuation is green, the next test should exercise the prepared command ladder with an actually overlapping scene/UI fixture before changing paint order.

