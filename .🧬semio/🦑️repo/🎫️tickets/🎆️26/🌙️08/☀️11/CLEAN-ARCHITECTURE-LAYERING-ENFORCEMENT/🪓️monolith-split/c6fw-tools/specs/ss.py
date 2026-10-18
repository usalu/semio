from specs.lib import *
SS = B3 + '/💡️queries/✂️intersect/🏄️surface-surface'
CR = B3 + '/💡️queries/✂️intersect/🦀️.rs'
CH = [
    ch('📈', 'pcurves-and-curve-finishing', 'pcurves_and_curve_finishing', [(93, 296)],
       'P-curves and curve finishing: the closed-form and sampled p-curves on each support, their error measurement and the finishing of an intersection curve with its finite domain.'),
    ch('✈', 'plane-pairs', 'plane_pairs', [(302, 589)],
       'Plane pairs: the exact plane intersections with planes, cylinders, cones, spheres and tori, the meridian level cases and the sphere and sphere intersection.'),
    ch('🌀', 'conics-coaxial-and-cylinders', 'conics_coaxial_and_cylinders', [(595, 834)],
       'Conics, coaxial pairs and cylinders: the conic solver, the coaxial meridian profile intersection and the cylinder and cylinder cases with their Steinmetz ellipses.'),
    ch('🧭', 'general-marching', 'marching_tracer', [(840, 1130)],
       'General marching: the finite domain and cell helpers, the Gauss-Newton seeds, the tangent predictor and corrector march and the general marching tracer.'),
]
SPEC = dict(orig='ss.rs', file=SS + '/🦀️.rs', children=CH, wiring='block', external=mounted(SS + '/🦀️.rs', orig='ss.rs'),
            crate_files=crate_files(CR, SS + '/🦀️.rs'), rebase_super=True)
