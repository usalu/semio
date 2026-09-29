// #region 🔁️AffineTransforms
/// 🔁️ The kernel-neutral affine-transform vectors — the same rows the `brep_invoke` bridge law
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/📐️brep-invoke`), the CAD `SemioBrepKernel` and OpenCascade (the
/// third-party oracle, `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio` suite) answer.
const AFFINE_TRANSFORMS_FIXTURE: &str = include_str!("../../../../🧫️fixtures/🔁️affine-transforms/🔣️.json");

/// 📐️ What the laws read off a solid: exact mass properties, the tessellation's extent, planar faces' outward normals,
/// topology counts, and the tessellated soup's own signed volume and centroid as `parry3d` (third party) integrates them.
struct AffineMeasure {
    volume: f64,
    center_of_mass: EVec3,
    min: EVec3,
    max: EVec3,
    normals: Vec<EVec3>,
    soup_volume: f64,
    soup_centroid: EVec3,
    topology: [usize; 3],
}

/// 🧮️ A fixture step as `(linear, translation)`, derived here (Rodrigues rotation, Householder reflection) independently of
/// the kernel's own `Affine3`, so the exactness law never checks the kernel against itself.
type AffineMap = ([[f64; 3]; 3], EVec3);

fn affine_fixture() -> serde_json::Value {
    let root: serde_json::Value = serde_json::from_str(AFFINE_TRANSFORMS_FIXTURE).expect("affine-transforms fixture parses");
    assert_eq!(root["schema"].as_str(), Some("s.stdio.semio.brep.affine-transforms/v1"), "fixture schema");
    assert!(!root["cases"].as_array().expect("cases").is_empty(), "the fixture declares at least one case");
    assert!(!root["refusals"].as_array().expect("refusals").is_empty(), "the fixture declares at least one refusal");
    root
}

fn affine_number(value: &serde_json::Value, key: &str) -> f64 {
    value[key].as_f64().unwrap_or_else(|| panic!("fixture field {key} is a number"))
}

fn affine_vec3(value: &serde_json::Value) -> EVec3 {
    let items = value.as_array().expect("fixture vector is an array");
    assert_eq!(items.len(), 3, "fixture vector has three components");
    [items[0].as_f64().expect("x"), items[1].as_f64().expect("y"), items[2].as_f64().expect("z")]
}

fn affine_make(kernel: &mut Brep, solid: &serde_json::Value) -> GeometryHandle {
    match solid["kind"].as_str() {
        Some("box") => kernel.box_prim(affine_number(solid, "width"), affine_number(solid, "depth"), affine_number(solid, "height")),
        Some("sphere") => kernel.sphere_prim(affine_number(solid, "radius")),
        Some("cylinder") => kernel.cylinder_prim(affine_number(solid, "radius"), affine_number(solid, "height")),
        Some("cone") => kernel.cone_prim(affine_number(solid, "radius"), affine_number(solid, "height")),
        other => panic!("unknown fixture solid {other:?}"),
    }
    .expect("fixture primitive builds")
}

fn affine_apply(kernel: &mut Brep, shape: &GeometryHandle, step: &serde_json::Value) -> Result<GeometryHandle, BrepError> {
    match step["kind"].as_str() {
        Some("translate") => kernel.translate(shape, affine_vec3(&step["offset"])),
        Some("rotate") => kernel.rotate(shape, affine_vec3(&step["axis"]), affine_number(step, "angle")),
        Some("rotateAbout") => kernel.rotate_about(shape, affine_vec3(&step["origin"]), affine_vec3(&step["axis"]), affine_number(step, "angle")),
        Some("scale") => kernel.scale(shape, affine_number(step, "factor"), affine_vec3(&step["center"])),
        Some("mirror") => kernel.mirror(shape, affine_vec3(&step["origin"]), affine_vec3(&step["normal"])),
        other => panic!("unknown fixture step {other:?}"),
    }
}

fn affine_unit(v: EVec3) -> EVec3 {
    let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / length, v[1] / length, v[2] / length]
}

fn affine_linear_about(linear: [[f64; 3]; 3], fixed: EVec3) -> AffineMap {
    let moved = affine_times(&linear, fixed);
    (linear, [fixed[0] - moved[0], fixed[1] - moved[1], fixed[2] - moved[2]])
}

fn affine_rotation(axis: EVec3, angle: f64) -> [[f64; 3]; 3] {
    let [x, y, z] = affine_unit(axis);
    let (s, c) = angle.sin_cos();
    let t = 1.0 - c;
    [[t * x * x + c, t * x * y - s * z, t * x * z + s * y], [t * x * y + s * z, t * y * y + c, t * y * z - s * x], [t * x * z - s * y, t * y * z + s * x, t * z * z + c]]
}

fn affine_step_map(step: &serde_json::Value) -> AffineMap {
    let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    match step["kind"].as_str() {
        Some("translate") => (identity, affine_vec3(&step["offset"])),
        Some("rotate") => (affine_rotation(affine_vec3(&step["axis"]), affine_number(step, "angle")), [0.0; 3]),
        Some("rotateAbout") => affine_linear_about(affine_rotation(affine_vec3(&step["axis"]), affine_number(step, "angle")), affine_vec3(&step["origin"])),
        Some("scale") => {
            let f = affine_number(step, "factor");
            affine_linear_about([[f, 0.0, 0.0], [0.0, f, 0.0], [0.0, 0.0, f]], affine_vec3(&step["center"]))
        }
        Some("mirror") => {
            let n = affine_unit(affine_vec3(&step["normal"]));
            let row = |i: usize| [0, 1, 2].map(|j| if i == j { 1.0 } else { 0.0 } - 2.0 * n[i] * n[j]);
            affine_linear_about([row(0), row(1), row(2)], affine_vec3(&step["origin"]))
        }
        other => panic!("unknown fixture step {other:?}"),
    }
}

fn affine_times(m: &[[f64; 3]; 3], v: EVec3) -> EVec3 {
    [0, 1, 2].map(|i| m[i][0] * v[0] + m[i][1] * v[1] + m[i][2] * v[2])
}

fn affine_compose(outer: &AffineMap, inner: &AffineMap) -> AffineMap {
    let linear = [0, 1, 2].map(|i| [0, 1, 2].map(|j| (0..3).map(|k| outer.0[i][k] * inner.0[k][j]).sum::<f64>()));
    let moved = affine_times(&outer.0, inner.1);
    (linear, [moved[0] + outer.1[0], moved[1] + outer.1[1], moved[2] + outer.1[2]])
}

fn affine_point(map: &AffineMap, p: EVec3) -> EVec3 {
    let moved = affine_times(&map.0, p);
    [moved[0] + map.1[0], moved[1] + map.1[1], moved[2] + map.1[2]]
}

fn affine_determinant(m: &[[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

/// 🧭️ An outward normal maps by the inverse transpose — `cofactor / det`, so a reflection keeps it outward.
fn affine_normal(map: &AffineMap, n: EVec3) -> EVec3 {
    let m = &map.0;
    let cofactor = [
        [m[1][1] * m[2][2] - m[1][2] * m[2][1], m[1][2] * m[2][0] - m[1][0] * m[2][2], m[1][0] * m[2][1] - m[1][1] * m[2][0]],
        [m[0][2] * m[2][1] - m[0][1] * m[2][2], m[0][0] * m[2][2] - m[0][2] * m[2][0], m[0][1] * m[2][0] - m[0][0] * m[2][1]],
        [m[0][1] * m[1][2] - m[0][2] * m[1][1], m[0][2] * m[1][0] - m[0][0] * m[1][2], m[0][0] * m[1][1] - m[0][1] * m[1][0]],
    ];
    let det = affine_determinant(m);
    affine_unit(affine_times(&cofactor, n).map(|component| component / det))
}

fn affine_measure(kernel: &mut Brep, shape: &GeometryHandle, tessellation: f64) -> AffineMeasure {
    let mesh = kernel.tessellate(shape, tessellation).expect("the solid tessellates");
    let points: Vec<parry3d::na::Point3<f32>> = mesh.position.chunks_exact(3).map(|p| parry3d::na::Point3::new(p[0], p[1], p[2])).collect();
    let triangles: Vec<[u32; 3]> = mesh.index.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect();
    assert!(!points.is_empty() && !triangles.is_empty(), "tessellation produced an empty mesh");
    let (soup_volume, soup_centroid) = parry3d::mass_properties::details::trimesh_signed_volume_and_center_of_mass(&points, &triangles);
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for point in &points {
        for (axis, value) in [point.x, point.y, point.z].into_iter().enumerate() {
            min[axis] = min[axis].min(f64::from(value));
            max[axis] = max[axis].max(f64::from(value));
        }
    }
    let topology = kernel.deconstruct(shape).expect("the solid deconstructs");
    AffineMeasure {
        volume: kernel.volume(shape).expect("volume"),
        center_of_mass: kernel.center_of_mass(shape).expect("centre of mass"),
        min,
        max,
        normals: mesh.face_infos.iter().filter(|info| matches!(info.surface_kind, SurfaceKind::Plane)).map(|info| info.normal).collect(),
        soup_volume: f64::from(soup_volume),
        soup_centroid: [f64::from(soup_centroid.x), f64::from(soup_centroid.y), f64::from(soup_centroid.z)],
        topology: [topology.faces.len(), topology.edges.len(), topology.vertices.len()],
    }
}

/// 🧷️ Moves every normal of `expected` onto its match in `got` (within `tolerance`), reporting what is left on either side.
fn affine_normal_sets(id: &str, got: &[EVec3], expected: &[EVec3], tolerance: f64, failures: &mut Vec<String>) {
    let mut unmatched = got.to_vec();
    for want in expected {
        match unmatched.iter().position(|n| (0..3).all(|axis| (n[axis] - want[axis]).abs() <= tolerance)) {
            Some(index) => {
                unmatched.remove(index);
            }
            None => failures.push(format!("{id}: outward face normal {want:?} missing from {got:?}")),
        }
    }
    if !unmatched.is_empty() {
        failures.push(format!("{id}: unexpected face normals {unmatched:?}"));
    }
}

/// 🔁️ LAW (exactness): every step maps the solid EXACTLY by its affine map — the tessellated soup's signed volume scales by
/// `|det|` and stays POSITIVE (to f32 precision under an isometry; a non-unit scale re-samples curved faces at the same
/// absolute chord, so there the chordal tolerances apply) (`volume` reports a magnitude, so a reflection that left the solid inside out fails here on the
/// sign alone), its centroid lands where the map sends it, every planar face's outward normal lands where the inverse
/// transpose sends it, and face/edge/vertex counts are preserved. The map is derived in this law, not read from the kernel.
#[test]
fn every_affine_transform_maps_the_solid_exactly() {
    let root = affine_fixture();
    let tessellation = affine_number(&root, "tessellationTolerance");
    let normal_tolerance = affine_number(&root, "normalTolerance");
    let soup_volume_tolerance = affine_number(&root, "soupVolumeRelativeTolerance");
    let soup_centroid_tolerance = affine_number(&root, "soupCentroidTolerance");
    let mut failures = Vec::new();
    for case in root["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let mut kernel = Brep::new();
        let mut shape = affine_make(&mut kernel, &case["solid"]);
        let before = affine_measure(&mut kernel, &shape, tessellation);
        let mut map: AffineMap = ([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]], [0.0; 3]);
        for step in case["steps"].as_array().expect("steps") {
            shape = affine_apply(&mut kernel, &shape, step).unwrap_or_else(|error| panic!("{id}: {error}"));
            map = affine_compose(&affine_step_map(step), &map);
        }
        let after = affine_measure(&mut kernel, &shape, tessellation);
        let scale = affine_determinant(&map.0).abs();
        let isometry = (scale - 1.0).abs() <= 1e-12;
        let (volume_tolerance, centroid_tolerance) = if isometry { (1e-5, 1e-5) } else { (soup_volume_tolerance, soup_centroid_tolerance) };
        if !(after.soup_volume > 0.0 && (after.soup_volume - scale * before.soup_volume).abs() <= volume_tolerance * scale * before.soup_volume) {
            failures.push(format!("{id}: soup signed volume {} != |det| · {} = {}", after.soup_volume, before.soup_volume, scale * before.soup_volume));
        }
        let centroid = affine_point(&map, before.soup_centroid);
        let extent = (0..3).map(|axis| after.max[axis] - after.min[axis]).fold(1.0, f64::max);
        if (0..3).any(|axis| (after.soup_centroid[axis] - centroid[axis]).abs() > centroid_tolerance * extent) {
            failures.push(format!("{id}: soup centroid {:?} != mapped {centroid:?}", after.soup_centroid));
        }
        if before.topology != after.topology {
            failures.push(format!("{id}: faces/edges/vertices {:?} -> {:?}", before.topology, after.topology));
        }
        let mapped: Vec<EVec3> = before.normals.iter().map(|n| affine_normal(&map, *n)).collect();
        affine_normal_sets(id, &after.normals, &mapped, normal_tolerance, &mut failures);
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// 📐️ LAW (closed form): every vector answers its closed form — volume and centre of mass within the fixture's
/// mass-property tolerances, the tessellation's extent within `boundsTolerance`, the soup (parry3d) within its chordal
/// tolerances, every planar face's outward normal exactly — the same rows OpenCascade answers in the `🧠️semio` suite.
#[test]
fn every_affine_transform_vector_meets_its_closed_form() {
    let root = affine_fixture();
    let tessellation = affine_number(&root, "tessellationTolerance");
    let volume_tolerance = affine_number(&root, "volumeRelativeTolerance");
    let center_tolerance = affine_number(&root, "centerOfMassTolerance");
    let bounds_tolerance = affine_number(&root, "boundsTolerance");
    let normal_tolerance = affine_number(&root, "normalTolerance");
    let soup_volume_tolerance = affine_number(&root, "soupVolumeRelativeTolerance");
    let soup_centroid_tolerance = affine_number(&root, "soupCentroidTolerance");
    let mut failures = Vec::new();
    for case in root["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let expect = &case["expect"];
        let mut kernel = Brep::new();
        let mut shape = affine_make(&mut kernel, &case["solid"]);
        for step in case["steps"].as_array().expect("steps") {
            shape = affine_apply(&mut kernel, &shape, step).unwrap_or_else(|error| panic!("{id}: {error}"));
        }
        let after = affine_measure(&mut kernel, &shape, tessellation);
        let volume = affine_number(expect, "volume");
        let mut near = |label: String, got: f64, want: f64, tolerance: f64| {
            if !((got - want).abs() <= tolerance) {
                failures.push(format!("{id}: {label} {got} != {want} (±{tolerance})"));
            }
        };
        near("volume".into(), after.volume, volume, volume_tolerance * volume.abs());
        near("soup signed volume".into(), after.soup_volume, volume, soup_volume_tolerance * volume.abs());
        let center = affine_vec3(&expect["centerOfMass"]);
        let min = affine_vec3(&expect["bounds"]["min"]);
        let max = affine_vec3(&expect["bounds"]["max"]);
        for axis in 0..3 {
            near(format!("centerOfMass[{axis}]"), after.center_of_mass[axis], center[axis], center_tolerance);
            near(format!("soup centroid[{axis}]"), after.soup_centroid[axis], center[axis], soup_centroid_tolerance);
            near(format!("bounds.min[{axis}]"), after.min[axis], min[axis], bounds_tolerance);
            near(format!("bounds.max[{axis}]"), after.max[axis], max[axis], bounds_tolerance);
        }
        if let Some(normals) = expect["faceNormals"].as_array() {
            let expected: Vec<EVec3> = normals.iter().map(affine_vec3).collect();
            affine_normal_sets(id, &after.normals, &expected, normal_tolerance, &mut failures);
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// 🧭️ LAW (refusal): a degenerate step — a zero axis or normal, a zero factor — is refused as invalid input and never
/// answered with a substituted direction; OpenCascade refuses every one of them too (`🧠️semio` suite).
#[test]
fn every_degenerate_affine_transform_is_refused() {
    let root = affine_fixture();
    let mut failures = Vec::new();
    for refusal in root["refusals"].as_array().expect("refusals") {
        let id = refusal["id"].as_str().expect("refusal id");
        let mut kernel = Brep::new();
        let shape = affine_make(&mut kernel, &refusal["solid"]);
        match affine_apply(&mut kernel, &shape, &refusal["step"]) {
            Err(BrepError::InvalidInput(_)) => {}
            other => failures.push(format!("{id}: expected an invalid-input refusal, got {other:?}")),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}
// #endregion 🔁️AffineTransforms
