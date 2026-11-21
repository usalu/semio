# Retained Closest Point on Derived Analytic Solids

Closest Point now uses the actual analytic boundaries retained by sphere and box Cuts, spherical bands or caps, and conical frustums. A spherical cavity compares the independently placed inner sphere with the authored exterior. Bounded spherical and frustum projections compare the curved support with their finite closure disks.

The shared fixture includes the eccentric sphere cavity, a sphere inside an authored box, a truncated spherical band, and the meridian and cap cases on a frustum. The compact BRep executable passed 66 tests, including the 11 fixture targets. Its derived functions use the production shape supports and exact primitive box branch, with a compact full-sphere evaluator. The independent Three.js run passed two tests and 11 assertions. The registered Flow query fixture also constructs the eccentric cavity through the real retained Cut operation. The complete linked query run remains open.

## Current native and independent verification

The compact native suite reached 71 tests with exact derived nearest and world bounds for the spherical shell, closed cap band, and tilted frustum. The combined Three.js rational and derived witness passed 3 cases and 30 assertions. The actual linked native analysis suite passed all 18 tests before the Section addition, then 19 tests including Section.
