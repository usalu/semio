# WGPU Engine Attach

The product shell check reported nine removed field accesses in the renderer's engine attachment code. Flow now commits declared cameras through `FlowHost::set_camera`, which updates the host snapshot, the DAG camera, and the interaction projection together. The tiled-map renderer uses the same size and camera methods as its browser host. The paint and text-editor renderers update their live hosts through the domain viewport setters.

Hover reconciliation calculates the desired change while borrowing the current Flow port address, then updates the separate hover cache fields. This preserves the hover decision and avoids copying the current port.

The existing Flow fixture now re-feeds a changed viewport through the production attachment entry point and checks both the host snapshot and DAG camera. Rustfmt parsed the edited files and `git diff --check` passed. The shared Cargo check continues with the repaired attachment package.
