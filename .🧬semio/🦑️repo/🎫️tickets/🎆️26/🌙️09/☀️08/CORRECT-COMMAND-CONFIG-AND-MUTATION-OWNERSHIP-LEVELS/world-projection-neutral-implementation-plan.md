# World Projection Neutral Implementation Plan

## Approved First Slice

Implement the neutral projection value owners and pure derivation first. This slice does not change World3dScene or migrate its producers, so it can proceed while NodeGraph/Pack/FEM native validation uses the current scene API. The final typed scene activation remains required by the goal and follows this slice.

Read `world-scene-projection-owner-audit.md`, including its corrected renderer trace. Reuse the existing strict Viewport3dOrbit. Add Viewport3dProjectionPreferences and Viewport3dProjectionSpec at the neutral UI viewport 3D schema owner, with Rust/TypeScript implementations and existing schema facets. Preserve the full editable preset bank separately from the active mode. The neutral mode and orientation taxonomy covers every current renderer alternative. No Three/native math object, app command, window identity or renderer initialization flag belongs in these value types.

Do not create Viewport3dSceneProjection or an implicitPerspective compatibility variant. Root confirmed that the old explicitProjection boolean only supplies an optional control-family override; its fallback derives the actual camera class. The renderer rig independently chooses the concrete camera and FOV from the active spec. A historical parser branch is not an additional mathematical projection kind.

## Schema And Behavioral Laws

Start with strict schemas and a shared neutral corpus. Cover all seven active modes and all cardinal/corner/free orientation alternatives, optional corner hemisphere omission versus explicit null, inactive preference retention, default full preferences and active derivation, invalid enum/tag/field/range/non-finite values, and serialization equality in Rust/TypeScript/Pack. Follow the strict existing viewport Serde/FromValue agreement, including object-only input and unknown-field refusal. Keep full type/codec exports owned by this codebase.

The pure derivation uses configured dimetric/trimetric angles. Do not copy the existing native helper's nested-mode lookup error into the neutral implementation. Isometric derives30/30; dimetric derivesA/A; trimetric derivesA/B. Preserve current full-preference defaults, including shared perspective FOV50 and the inactive curvilinear parameters. The active orientation permits lower hemisphere even though current native preferences do not select it; do not invent a new stored preference just to exercise that active capability.

Use existing Ajv/JSON Patch and an independent renderer/math oracle where applicable. A fixture-to-fixture comparison alone does not verify the native codec or derivation. Add meaningful native runtime laws and clear DEBUG output. Register a canonical Bun/Nx verification target and matching launch/seed entry; use a ticket validation facade to avoid the unrelated workspace graph hang. Coordinate Cargo with root. Do not run a second workspace compiler batch without handoff.

## Separate Scene Activation Decision

The next slice must remove the opaque camera string across the actual scene codecs, producers, React and WGPU paths. It must preserve orbit absence, use typed active projection data, and express any necessary automatic framing policy at the scene/renderer boundary. It must not discard configured lens values merely because the current React host overwrites them.

Root verified that the plain camera helper accepts an arbitrary FOV. Generation3d preview, Lowpoly model and Process3d workpiece pass real config FOV values, not only a hardcoded45. The native bridge recognizes that FOV. In React, World3dHost currently substitutes the default threePoint50 spec when no projection spec exists; WorldProjectionRig then mounts a makeDefault PerspectiveCamera using50, despite the parsed camera value. This mismatch needs a concrete fixture and correction at scene activation. A45 renderer result would be a deliberate correction of the overwritten configured value, not evidence of preserved React behavior.

Do not add a second top-level FOV to orbit. The eventual helper must produce a perspective active spec containing its configured FOV. If that makes framing policy independent of projection presence, model that policy at the scene/renderer level rather than wrapping the mathematical projection taxonomy. Root will finalize this after executable renderer evidence.

No production source or tests were changed by this planning document. It is the bounded assignment for the next Sol execution slot.
