# Retained smooth Offset Solid

The registered `brep.solid.offsetSolid` operator now accepts certified spherical and ring-torus solids. Positive and negative distances change the sphere radius or torus section radius while preserving the exact original frame. The torus retains its original major radius. The offset refuses a collapsed sphere or section, exhausted torus opening, and coordinates the native primitive cannot represent. It uses the existing funded topology, retirement, measurement and preview paths.

The cold operation uses sharp corners on planar solids and a rebuild plus corner blending on solids with curved faces. The sphere and ring torus each have one smooth supporting face and no two-face corner to blend. Their surface offset is an exact change of radius. A capped cylinder continues to require a separate mixed analytic boundary for the default corner policy. Selective box Fillet and closed curved Shell would also require owned mixed or cavity topology and funded preview support.

## Verification

- The neutral fixture defines displaced spheres and translated, tilted tori, with expanding and contracting offsets, exact area and volume, and degenerate or topology-changing cases.
- All eight Three.js feature checks passed. The new independent tests measure each authored signed offset and check sample radii on displaced and tilted geometry.
- The isolated Rust test compiled the original offset recipe, native vector/frame and primitive volume code. It passed the exact radii, preserved frame, volume/refusal checks and the 128-byte retained admission budget check.
- The linked original-brep-solids regression constructs and places each primitive through the registered Flow operators, then tests both offset signs, active cancellation, refusal and source survival. It checks native analytic frames, exact volume and component-labelled preview. Linked execution remains pending.
- AST and scoped whitespace checks passed. The Engine closure stayed at 3,995 active lines in the shared tree, with 3,833 in the native registry and 1,849 in the solids integration target.
