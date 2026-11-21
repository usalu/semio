# Retained loft between unequal circular profiles

The registered Loft operation now builds an exact conical frustum from two distinct coaxial circular planar faces with matching angular frames. Either profile order and either signed axial direction are supported. The native solid has two exact circular cap rims, two planar caps and one genuine conical lateral face. A single native longitudinal edge closes the periodic trim and appears twice with opposed coedge uses. Coincidence checks certify that the standard cone representation can reproduce both original rims at kernel tolerance.

The two-profile synchronous Loft executes a degree-one longitudinal interpolation in both smooth and ruled modes. The retained construction expresses the same supported coaxial geometry directly through native circles and a Cone surface. Its existing solid publication owner funds each native insertion and rollback. Funded solid tessellation uses the same original rim circles and slope. It retains the exact native face and edge IDs for picking, including re-preview of the selected conical side.

## Verification

- The neutral solid fixture covers translated and tilted axes, both profile orders, decreasing and increasing radii, a negative world-axis travel, and the two-profile smooth option. All six Three.js solid tests passed, including independent frustum cap positions, volume and area.
- The isolated Rust source check passed with the real native frustum recipe and funded sampling chart. It checks same-parameter trims, opposite seam and rim uses, cap placement, exact volume and agreement of sampled area and volume over all three authored cases. Degenerate radii and axial travel are refused.
- The linked original-brep-solids fixture drives registered Circle, Face From Wire, Loft, Volume and Deconstruct through one-item grants. It checks active cancellation, original profile survival, the exact native rims and conic surface, their preview labels and tessellation of the picked side face. Linked execution remains pending the shared native build.
- Rust AST and scoped diff checks passed. Source closure measured 3,942 Construction lines, 3,999 Engine lines, 2,662 Mesh IO lines and 3,877 native registry lines.
