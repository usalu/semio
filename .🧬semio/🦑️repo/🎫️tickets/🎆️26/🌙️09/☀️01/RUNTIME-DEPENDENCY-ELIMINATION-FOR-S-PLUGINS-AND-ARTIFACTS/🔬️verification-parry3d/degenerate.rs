use p3verify::collision::{intersection_test, TriMesh};
use p3verify::rigid::{Isometry3, Point3, UnitQuaternion, Vector3};
use parry3d::na;
use parry3d::shape::{TriMesh as PTriMesh, TriMeshFlags};

fn cube(h: f32) -> (Vec<[f32; 3]>, Vec<[u32; 3]>) {
    (vec![[-h,-h,-h],[h,-h,-h],[h,h,-h],[-h,h,-h],[-h,-h,h],[h,-h,h],[h,h,h],[-h,h,h]],
     vec![[0,2,1],[0,3,2],[4,5,6],[4,6,7],[0,1,5],[0,5,4],[1,2,6],[1,6,5],[2,3,7],[2,7,6],[3,0,4],[3,4,7]])
}
fn ours(v: &[[f32;3]], t: &[[u32;3]]) -> TriMesh { TriMesh::new(v.iter().map(|p| Point3::new(p[0],p[1],p[2])).collect(), t.to_vec()) }
fn theirs(v: &[[f32;3]], t: &[[u32;3]]) -> PTriMesh {
    PTriMesh::with_flags(v.iter().map(|p| na::Point3::new(p[0],p[1],p[2])).collect(), t.to_vec(),
        TriMeshFlags::ORIENTED | TriMeshFlags::MERGE_DUPLICATE_VERTICES)
}
fn iso(x: f32, y: f32, z: f32) -> (Isometry3, na::Isometry3<f32>) {
    (Isometry3::from_parts(Vector3::new(x,y,z), UnitQuaternion::identity()),
     na::Isometry3::from_parts(na::Translation3::new(x,y,z), na::UnitQuaternion::identity()))
}

#[test]
fn degenerate_configurations_match_parry3d() {
    let (v1,t1) = cube(1.0);
    let (vs,ts) = cube(0.25);
    let (ob, pb) = (ours(&v1,&t1), theirs(&v1,&t1));
    let (osm, psm) = (ours(&vs,&ts), theirs(&vs,&ts));
    let cases: Vec<(&str, (f32,f32,f32), bool)> = vec![
        ("nested small-in-big",        (0.0,0.0,0.0),   true),
        ("nested offset",              (0.5,0.0,0.0),   true),
        ("deep overlap",               (1.0,0.0,0.0),   true),
        ("exact face contact",         (2.0,0.0,0.0),   false),
        ("just inside face",           (1.999,0.0,0.0), false),
        ("just outside face",          (2.001,0.0,0.0), false),
        ("exact edge contact",         (2.0,2.0,0.0),   false),
        ("exact corner contact",       (2.0,2.0,2.0),   false),
        ("coplanar side-by-side",      (2.0,1.0,0.0),   false),
        ("disjoint far",               (9.0,0.0,0.0),   false),
    ];
    let mut bad = 0;
    for (name, (x,y,z), _) in &cases {
        for (label, om, pm) in [("big-vs-big", &ob, &pb), ("big-vs-small", &osm, &psm)] {
            let (oi, pi) = iso(*x,*y,*z);
            let mine = intersection_test(Isometry3::identity(), &ob, oi, om);
            let refr = parry3d::query::intersection_test(&na::Isometry3::identity(), &pb, &pi, pm).unwrap();
            let mark = if mine == refr { "ok " } else { bad += 1; "DIFF" };
            println!("{mark} {name:24} {label:14} parry={refr:5} ours={mine}");
        }
    }
    println!("\ndegenerate disagreements: {bad}");
    assert_eq!(bad, 0, "{bad} degenerate-case disagreements with parry3d");
}

#[test]
fn contains_point_matches_parry3d_point_query() {
    use parry3d::query::PointQuery;
    let (v,t) = cube(1.0);
    let (om, pm) = (ours(&v,&t), theirs(&v,&t));
    let mut bad = 0; let mut n = 0;
    for i in 0..12 { for j in 0..12 { for k in 0..12 {
        let p = [-2.0 + i as f32 * 0.36, -2.0 + j as f32 * 0.36, -2.0 + k as f32 * 0.36];
        let mine = p3verify::collision::contains_point(Isometry3::identity(), &om, Point3::new(p[0],p[1],p[2]));
        let refr = pm.contains_point(&na::Isometry3::identity(), &na::Point3::new(p[0],p[1],p[2]));
        n += 1;
        if mine != refr { bad += 1; if bad <= 6 { println!("  DIFF at ({:.2},{:.2},{:.2}) parry={refr} ours={mine}", p[0],p[1],p[2]); } }
    }}}
    println!("contains_point: {} points, {bad} disagreements", n);
    assert_eq!(bad, 0);
}
