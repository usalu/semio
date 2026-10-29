# Public Native Geometry Interface R19

The fresh clip baseline exposed visualization consumers calling private model constructors. The model now provides explicitly public profile_geometry, column_geometry, part_outline_at, part_footprint, part_section and part_elevation interfaces. Every affected drawing, mesh exporter and colocated contract calls those native interfaces. The analytical drawing resolves an element with the public element interface. No legacy aliases or alternate geometry implementation were introduced.

The first focused check passes7 frustum/analytical-profile tests plus the native vehicle-profile fixture. A complete visual consumer scan finds no remaining private model API call. Further shared projection recorders are checked after the remaining helper renames. Native frame verification remains separate and unverified.

A direct import of the native axon-test module in the portable interpreter correctly stopped because Manim is unavailable; that attempt did not run the projection checks and is not recorded as a geometry failure or a pass. The established AST/recording projection harness is used for portable geometry checks.

The complete established44-case projection/solid cohort passed43 cases and exposed one integration error while the concurrent roof feature was being authored: new sill/head solids lacked their required local base placement. The roof author corrected those references; the previously failing stair-volume check then passed against the fresh377-element source. All remaining published shared geometry helper consumers are public.
