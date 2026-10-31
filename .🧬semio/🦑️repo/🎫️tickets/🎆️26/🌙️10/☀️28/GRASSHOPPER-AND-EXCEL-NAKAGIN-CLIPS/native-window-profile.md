# Native Aperture and Glazing Profile

The retained inspect-window-profile.py decodes actual TRIANGLES/POSITION values in place. It excludes wireframes, accessor bounds and CAD control boxes. NumPy groups actual vertices by wall-normal plane and radial distance about the independently fitted window centre. No source asset is copied.

| Native family | Outer aperture plane | Inner reveal plane | Glazing plane | Glazing recess |
| --- | --- | --- | --- | --- |
| Slash/Backslash |Y2.10000 |Y2.03333 |Y2.03433 |≈.06567m |
| P/S |X1.30000 |X1.23333 |X1.23488 |≈.06512m |

Every selected aperture/reveal/glazing vertex has radius≈.65000m in the sampled.60–.85m radial band. These are physical mesh positions, not inflated circle control bounds. The native representation does not supply a separate80mm radial trim in this sample; this is a limitation of that representation, not proof the historic building had no trim.

Current reconstructed glazing inset was derived from assumed frame_depth/2−glass_thickness/2, yielding.034m. The source mesh instead supports about65–66mm physical recess. Execution is reviewing a source-bearing explicit recess/reveal parameter and actual-mesh regression so the glazing plane is not indirectly guessed from an unrelated frame extrusion depth.

The nativeP/S local longitudinal centre is separately confirmed at.950000008m with radius.650000008m andZ1.050000007 above the native reference origin (bodybottom−.20m). Raw photogrammetry currently contradicts the longitudinal offset. That conflict is being resolved against actual untruncated body components; native.95 is not automatically as-built truth. The world vertical centre normalization is consistent with modeled1.25m above the normalized outerfloor.

Generated numeric output: native-window-profiles.json. The first script run had a Python conditional-spacing typo; it was corrected before the successful measurement run. No production geometry was changed by this inspection.
