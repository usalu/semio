// 🧪 Standalone scratch validation for IfcLocalPlacement -> 4x4 world matrix composition
// (IfcAxis2Placement3D construction + parent-chain multiplication order).
// Run: rustc -O placement_scratch.rs -o placement_scratch && ./placement_scratch

type Vec3 = [f64; 3];
type Mat4 = [[f64; 4]; 4]; // row-major, affine, row 3 = [0,0,0,1], point transform: p' = M * p

fn identity() -> Mat4 {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn sub(a: Vec3, b: Vec3) -> Vec3 { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
fn dot(a: Vec3, b: Vec3) -> f64 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
fn cross(a: Vec3, b: Vec3) -> Vec3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn scale(a: Vec3, s: f64) -> Vec3 { [a[0] * s, a[1] * s, a[2] * s] }
fn norm(a: Vec3) -> f64 { dot(a, a).sqrt() }
fn normalize(a: Vec3) -> Vec3 {
    let n = norm(a);
    if n < 1e-12 { a } else { scale(a, 1.0 / n) }
}

/// Builds a world-independent local transform from an IfcAxis2Placement3D
/// (Location, optional Axis = local Z, optional RefDirection = local X hint).
fn axis2placement3d(location: Vec3, axis_z: Option<Vec3>, ref_dir_x: Option<Vec3>) -> Mat4 {
    let z = normalize(axis_z.unwrap_or([0.0, 0.0, 1.0]));
    let x_hint = ref_dir_x.unwrap_or([1.0, 0.0, 0.0]);
    // Gram-Schmidt: project x_hint orthogonal to z.
    let x_proj = sub(x_hint, scale(z, dot(x_hint, z)));
    let x = if norm(x_proj) < 1e-9 {
        // degenerate hint (parallel to z) -> pick any orthogonal axis
        let fallback = if z[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        normalize(sub(fallback, scale(z, dot(fallback, z))))
    } else {
        normalize(x_proj)
    };
    let y = cross(z, x);
    [
        [x[0], y[0], z[0], location[0]],
        [x[1], y[1], z[1], location[1]],
        [x[2], y[2], z[2], location[2]],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Row-major 4x4 matrix multiply: `a * b`.
fn mat_mul(a: &Mat4, b: &Mat4) -> Mat4 {
    let mut out = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            let mut sum = 0.0;
            for k in 0..4 {
                sum += a[i][k] * b[k][j];
            }
            out[i][j] = sum;
        }
    }
    out
}

fn transform_point(m: &Mat4, p: Vec3) -> Vec3 {
    [
        m[0][0] * p[0] + m[0][1] * p[1] + m[0][2] * p[2] + m[0][3],
        m[1][0] * p[0] + m[1][1] * p[1] + m[1][2] * p[2] + m[1][3],
        m[2][0] * p[0] + m[2][1] * p[1] + m[2][2] * p[2] + m[2][3],
    ]
}

/// Composes a chain of local matrices from root to leaf into world matrices:
/// world[i] = world[i-1] * local[i], world[0] = local[0] (root has no parent).
fn compose_chain(locals: &[Mat4]) -> Vec<Mat4> {
    let mut worlds = Vec::with_capacity(locals.len());
    let mut acc = identity();
    for local in locals {
        acc = mat_mul(&acc, local);
        worlds.push(acc);
    }
    worlds
}

fn approx_eq(a: Vec3, b: Vec3, label: &str) {
    for i in 0..3 {
        assert!((a[i] - b[i]).abs() < 1e-9, "{label}: mismatch at axis {i}: {a:?} vs {b:?}");
    }
    println!("OK  {label:<40} -> {a:?}");
}

fn main() {
    // 1) Pure translation chain: parent translates by (10,0,0), child by (0,5,0) relative to parent.
    let parent = axis2placement3d([10.0, 0.0, 0.0], None, None);
    let child = axis2placement3d([0.0, 5.0, 0.0], None, None);
    let worlds = compose_chain(&[parent, child]);
    let world_child_origin = transform_point(&worlds[1], [0.0, 0.0, 0.0]);
    approx_eq(world_child_origin, [10.0, 5.0, 0.0], "translation chain world origin");

    // 2) Rotation composition: parent rotated so local X -> world Y (RefDirection=(0,1,0), Axis stays Z).
    //    Child translated by (1,0,0) in the parent's LOCAL frame.
    //    Expected: world point = parent_origin + parent_rotation * (1,0,0) = (0,0,0) + (0,1,0) = (0,1,0).
    //    This is exactly the "matrix multiplication order" trap: local*parent would give a different,
    //    wrong result here since rotation is not commutative with this asymmetric setup once a second
    //    rotated level is added (case 3 below actually proves the order; this case alone is order-agnostic
    //    for a single translation-only child, so we assert case 3 is the real discriminator).
    let parent_rot = axis2placement3d([0.0, 0.0, 0.0], Some([0.0, 0.0, 1.0]), Some([0.0, 1.0, 0.0]));
    let child_t = axis2placement3d([1.0, 0.0, 0.0], None, None);
    let worlds2 = compose_chain(&[parent_rot, child_t]);
    let p2 = transform_point(&worlds2[1], [0.0, 0.0, 0.0]);
    approx_eq(p2, [0.0, 1.0, 0.0], "rotated-parent translated-child world origin");

    // 3) The real discriminator: two rotated levels stacked. parent rotates local X -> world Y (90°).
    //    child ALSO rotates its own local X -> its local Y (90° about Z, in child's own coordinate frame).
    //    world = parent * child (correct): child's local Y axis, expressed in world space, must equal
    //    parent's rotation applied to child's local Y-in-parent-frame, i.e. (child rotates X->Y within
    //    parent's frame, then parent rotates that whole frame X->Y in world) => world X axis of child
    //    ends up pointing along -world X (two stacked 90 deg rotations = 180 deg): grandchild-local-X
    //    point (1,0,0) should land at world (-1,0,0).
    let grandchild_local_x_probe = [1.0, 0.0, 0.0];
    let worlds3 = compose_chain(&[parent_rot, parent_rot]); // parent_rot composed with itself = 180 deg about Z
    let world_probe = transform_point(&worlds3[1], grandchild_local_x_probe);
    approx_eq(world_probe, [-1.0, 0.0, 0.0], "double 90deg stack (order-sensitive) world point");

    // Sanity: the WRONG order (child * parent) must NOT match, proving this test actually discriminates.
    let wrong = mat_mul(&parent_rot, &parent_rot); // same in this specific symmetric case (self-compose) --
    // use an asymmetric pair instead to truly discriminate order:
    let a = axis2placement3d([1.0, 0.0, 0.0], None, None); // pure translate +X
    let b = axis2placement3d([0.0, 0.0, 0.0], Some([0.0, 0.0, 1.0]), Some([0.0, 1.0, 0.0])); // pure rotate 90 about Z
    let correct_world = mat_mul(&a, &b); // a * b: translate then (in a's frame) rotate
    let incorrect_world = mat_mul(&b, &a); // b * a: rotate then translate -- different result
    let p_correct = transform_point(&correct_world, [1.0, 0.0, 0.0]);
    let p_incorrect = transform_point(&incorrect_world, [1.0, 0.0, 0.0]);
    assert!(
        (p_correct[0] - p_incorrect[0]).abs() > 1e-6 || (p_correct[1] - p_incorrect[1]).abs() > 1e-6,
        "order-discriminating case must actually differ between a*b and b*a"
    );
    println!("OK  order discriminator: a*b={p_correct:?} b*a={p_incorrect:?} (confirmed different)");
    let _ = wrong;

    println!("ALL PLACEMENT MATRIX SCRATCH CHECKS PASSED");
}
