# Fix: procedural 3D preview empty

## Root cause
Flow graph evaluation moved to in-WASM `flow_core` registry. Geometry handles were created in `flow_core`'s linked brep kernel, but 3D preview tessellation still used standalone `@flow/module-brep` WASM — a separate kernel instance where those handles do not exist.

## Fix
1. Restored `session.setEvalBridge(createFlowEvalBridge(extensionHost))` in `FlowCanvas` so brep ops run through the same `@flow/module-brep` WASM used for tessellation.
2. Added shared `tessellate_geometry_json` / `dispose_geometry` on `flow_module_brep` and wasm exports on `flow_core` for a future switch back to in-WASM eval without eval bridge.

## Verify
- Open procedural play, add sphere or torus, confirm 3D preview mesh appears.
- `[DEBUG] procedural play eval outputs` should include geometry handles like `solid-*`.
