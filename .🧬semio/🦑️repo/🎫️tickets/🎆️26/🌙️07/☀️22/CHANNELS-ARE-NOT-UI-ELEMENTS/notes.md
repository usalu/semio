# Channels Are Not UI Elements

## Intent
No channel is a UI control. Parametric input is provided only via sources such as sliders.
Components like `math.vector` expose ports only (x/y/z/vector); numbers arrive by wiring sliders into those ports.

## Removed
1. **InputStepper** widget (`Widget::InputStepper`, `NodeChrome::Stepper`, `core.stepper`) — was treating composite values as an editable source with per-field steppers.
2. **DagNodeKind::Stepper** / `DagStepperField` and catalogue/add-node entries for stepper.
3. **Channel param overlays** (`param_overlay_paint_state_json`, `GraphParamOverlays`) — inline editors on unconnected primitive ports (the bug surface for codemo `math.vector` x/y/z).
4. **Stepper overlays** (`stepper_overlay_state_json`, `GraphStepperOverlays`, `setStepperFieldValue`).
5. Stepper from flow manifest (`flow/manifest/dag.manifest.json` regenerated).

## Remaining sources (correct)
- Slider → number dictionary
- Note → text dictionary
- Image → image dictionary
- Variable → named typed dictionary

## Verification
Cargo check was blocked by concurrent agents holding the shared `target/` lock; source trees for flow, dag, procedural, and react no longer reference InputStepper/param overlays.
