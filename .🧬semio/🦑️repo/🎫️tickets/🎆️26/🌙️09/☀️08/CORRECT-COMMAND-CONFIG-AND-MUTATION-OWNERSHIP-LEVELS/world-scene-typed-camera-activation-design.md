# World Scene Typed Camera Activation

## Scope

This is the implementation boundary following the accepted neutral projection preferences/spec slice. Root refreshed the actual React scene consumer, scene Rust codec and native helper source. No scene API is switched by this document.

The shared `World3dScene` owns typed presentation data. Replace its opaque `camera_json` String with an optional `Viewport3dOrbit`, the required active `Viewport3dProjectionSpec`, and an explicit scene framing policy. Projection preferences remain with their actual configuration owner; only the active spec belongs in a scene. Orbit has no extra top-level FOV. FOV belongs to the selected projection mode. Renderer initialization flags, control-family overrides and native/Three objects do not belong in these value types.

## Observed Production Defects

The native `world3d_camera_json(position, target, fov)` helper preserves the supplied FOV in a JSON object. Generation3d preview, Lowpoly model and Process3d workpiece supply real configurable values. React parses that FOV but then derives `worldProjectionSpec` from a default threePoint spec whenever the string lacks a nested taxonomy spec. The real projection rig consequently receives the default FOV 50 instead of the configured value, including the common 45 case.

React infers orbit presence from a substring search for `"position"`, uses object/string projection alternatives, and enables ongoing content fitting based on projection-spec presence. Adding a typed active projection to the existing branch without separating framing would therefore start fitting previously fixed authored/persisted cameras. The current `explicitProjection` flag changes a control-family override; it is not an additional projection kind.

## Concrete Ownership Decision

The scene framing policy distinguishes preserving an explicit orbit from fitting content until user interaction. An absent orbit requires a content-derived initialization policy; it must not silently create an arbitrary default camera. A producer with an authored/default/persisted orbit supplies that orbit explicitly and selects preserving behavior unless it explicitly requests content fitting. A producer which intentionally tracks growing content can request fitting until a real user gesture takes ownership. A projection-template action may request a new fit explicitly.

The helper replacement constructs the active threePoint projection with its supplied FOV, the full orbit including up/zoom, and preserving framing. Existing projection-template producers retain their intentional content-fitting policy. The exact-window loader supplies any persisted orbit before the scene becomes ready; presentation must not override a successfully restored owner with a content seed.

User orbit/pan/zoom updates only the exact window's orbit. Projection selection uses the typed active mode and the correct projection-preference owner. A renderer echo neither reattaches nor reframes. A real external orbit/projection replacement applies once and resets only the relevant local interaction state. Content growth may reframe only while that scene explicitly permits it and the user has not taken control. Renderer-local epochs remain ephemeral.

## Required Execution

First add a closed scene schema and shared fixture covering configured FOV 45 and a nondefault configurable FOV, all seven active projection modes, omitted versus explicit orbit, both framing policies, user takeover, self echo, external replacement, growing bounds and two exact same-kind windows. The corpus must specify camera class, FOV, pose ownership, fit count and emitted window mutation identity. Compare real Three camera/projection behavior with the same neutral numeric expectations and native WGPU derivation.

Update the Rust/TypeScript scene codecs, scene spine/lane behavior, all schema facets and reexports, native/plugin helper APIs, MeshView/WindowKit forwarding, every actual producer, React consumer/rig and native scene bridge together. Remove the old camera string API and permissive parser. Do not introduce a compatibility variant, fallback decoder or duplicate FOV. Other scene kinds' camera strings require their own domain types and must not be mechanically converted to a 3D orbit.

Acceptance requires real renderer behavior and exact window reopening for two same-kind instances, typed Pack/text/JSON parity, stable document bytes, no camera publication from pure framing/echo, and terminal close. Preserve the already accepted neutral projection math and viewport owner laws. The full migration remains required and awaits an execution slot; current neutral projection acceptance alone does not establish any of these scene behaviors.
