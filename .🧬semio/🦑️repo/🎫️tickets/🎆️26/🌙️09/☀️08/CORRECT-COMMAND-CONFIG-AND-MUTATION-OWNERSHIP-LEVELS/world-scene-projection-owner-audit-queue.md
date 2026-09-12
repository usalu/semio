# World Scene Projection Owner Follow-Up

The renderer viewport audit identifies a real remaining boundary: World3dScene.cameraJson combines navigation pose with active projection, lens, and default/attachment semantics. NodeGraph is now a separate Sol implementation task. The world scene migration requires a further bounded Terra read-only audit before its Sol implementation.

## Evidence inspected by root

- OS/plugin WorldProjectionConfig stores the full editable projection configuration, including values for inactive modes so switching back restores them. Its source is currently in the plugin module near the WorldProjection region. The active renderer projection is a distinct derived mode × orientation contract.
- world3d_projection_spec_json lowers configuration into seven active projection modes and cardinal/corner/free orientation forms. Its values include mode-specific optical parameters; it must not be replaced by a binary perspective/orthographic flag.
- world3d_camera_projection_json serializes that derived projection together with position/target/up/zoom. Renderer parseCameraState accepts further implicit coordinate/string variants and fallbacks.
- Renderer WorldParsedCameraState also retains fov and explicitProjection. Missing projection currently has different attach/auto-fit behavior from an explicitly selected mode. Replacing the string cannot silently erase that distinction.
- Default fov differs across paths: the ordinary helper defaults to45, while WorldProjectionConfig defaults to50 for its threePoint mode. The actual domain defaults must be preserved.

## Audit deliverable

Inventory the actual WorldProjectionSpec and native projection/math owners, mode/orientation variants, value/Pack/JSON schemas, renderer interpretation and round-trip actions. Distinguish editable projection configuration (including inactive mode parameters), active projection transport, optical parameters, orbit pose, and attachment/auto-fit state. Identify a domain-neutral first-party owner for each reusable value and the correct app/window owner for mutable preferences.

Decide how the typed World3dScene represents pose and active projection without duplicating fov authority, losing implicit-versus-explicit initialization semantics, or retaining JSON/string adapters. Produce schema-first migration inventory for every real scene producer and native/TypeScript/WGPU consumer, with exact default/lens/projection preservation fixtures. Do not conflate tutorial, authored capture, icon-render request, external Three camera, or renderer math camera concepts.

Write the full audit to world-scene-projection-owner-audit.md. Do not edit production code or run Cargo during the read-only audit. The goal remains active until the resulting scene/action boundary migration is implemented and validated.
