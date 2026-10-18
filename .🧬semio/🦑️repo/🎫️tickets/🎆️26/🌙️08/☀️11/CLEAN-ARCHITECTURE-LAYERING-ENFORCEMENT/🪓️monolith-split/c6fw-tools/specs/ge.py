from specs.lib import *
GE = B3 + '/⚙️engine'
CR = B3 + '/⚙️engine/🦀️.rs'
H = 'impl Brep {'
B = [620, 777, 1008, 1205]
CH = [
    ch('🧠', 'brep-session-state', 'brep_session_state', [(61, 200)],
       'B-Rep session state: the live entity enum with its retirement, the native session struct, the boolean job, admission and step records and the session defaults.'),
    ch('🧾', 'conversions-and-registry', 'conversions_and_registry', [(206, 448)],
       'Conversions and registry: the point and vector conversions with their finite and positive argument checks, the entity labels and roots and the entity registry of the session.'),
    ch('🧱', 'primitive-and-curve-constructors', 'primitive_and_curve_constructors', [(454, 620)],
       'Primitive and curve constructors: the revision bookkeeping and the synchronous primitive, curve, wire and surface constructors of the session.'),
    ch('🔀', 'boolean-transform-and-copy', 'boolean_transform_and_copy', [(622, 780)],
       'Boolean, transform and copy: the synchronous fuse, cut, boolean and boolean job steps, the intersection, the transforms and the shape copy.'),
    ch('🔍', 'intersection-evaluation-and-measure', 'intersection_evaluation_and_measure', [(782, 1014)],
       'Intersection, evaluation and measure: the curve and surface intersections, the curve and surface evaluation, the measures, the closest point, the classification, the validation and the vertex, face and sew constructors.'),
    ch('🩺', 'healing-exchange-and-tessellation', 'healing_exchange_and_tessellation', [(1016, 1214)],
       'Healing, exchange and tessellation: the solid healing and NURBS conversion, the deconstruction, the mesh import and export, the kind lookup, the solid face loops and the tessellation jobs.'),
    ch('🏷', 'validation-gate-and-labels', 'validation_gate_and_labels', [(1216, 1325)],
       'Validation gate and labels: the validation gate, the disposal, the persistent labels and handles, the shells, compounds and explosion and the edge enumeration.'),
    ch('🔌', 'brep-kernel-implementation', 'brep_kernel_implementation', [(1342, 1741)],
       'B-Rep kernel implementation: the kernel trait implementation of the session that delegates every operation to the synchronous API.'),
    ch('💾', 'solid-exchange-codecs', 'solid_exchange_codecs', [(1747, 1818)],
       'Solid exchange codecs: the solid exporter and importer traits with their STL, OBJ and GLB implementations.'),
]
SPEC = dict(orig='ge.rs', file=GE + '/🦀️.rs', children=CH, wiring='block', external=mounted(GE + '/🦀️.rs', orig='ge.rs'),
            pre=split_impl(H, B, at=454), expect_extra=split_extra(H, B), crate_files=crate_files(CR, GE + '/🦀️.rs'))
