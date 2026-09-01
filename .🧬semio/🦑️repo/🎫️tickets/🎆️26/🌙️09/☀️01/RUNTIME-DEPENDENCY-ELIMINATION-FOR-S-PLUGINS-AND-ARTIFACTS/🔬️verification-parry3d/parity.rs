use p3verify::collision::{intersection_test, TriMesh};
use p3verify::rigid::{Isometry3, Point3, Quaternion, UnitQuaternion, Vector3};
use parry3d::na;
use parry3d::shape::{TriMesh as PTriMesh, TriMeshFlags};

struct Lcg(u64);
impl Lcg {
    fn next_u32(&mut self) -> u32 { self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); (self.0 >> 33) as u32 }
    fn f(&mut self, lo: f32, hi: f32) -> f32 { lo + (self.next_u32() as f32 / u32::MAX as f32) * (hi - lo) }
}

fn cube(h: f32) -> (Vec<[f32; 3]>, Vec<[u32; 3]>) {
    (vec![[-h,-h,-h],[h,-h,-h],[h,h,-h],[-h,h,-h],[-h,-h,h],[h,-h,h],[h,h,h],[-h,h,h]],
     vec![[0,2,1],[0,3,2],[4,5,6],[4,6,7],[0,1,5],[0,5,4],[1,2,6],[1,6,5],[2,3,7],[2,7,6],[3,0,4],[3,4,7]])
}

#[test]
fn intersection_matches_parry3d() {
    let (v, t) = cube(1.0);
    let om = TriMesh::new(v.iter().map(|p| Point3::new(p[0], p[1], p[2])).collect(), t.clone());
    let pm = PTriMesh::with_flags(
        v.iter().map(|p| na::Point3::new(p[0], p[1], p[2])).collect(), t.clone(),
        TriMeshFlags::ORIENTED | TriMeshFlags::MERGE_DUPLICATE_VERTICES);
    let mut rng = Lcg(0x9E3779B97F4A7C15);
    let (mut agree, mut disagree, mut hits) = (0u32, 0u32, 0u32);
    let mut shown = 0;
    for _ in 0..600 {
        let tr = Vector3::new(rng.f(-3.5, 3.5), rng.f(-3.5, 3.5), rng.f(-3.5, 3.5));
        let uq = UnitQuaternion::from_quaternion(Quaternion::new(rng.f(-1., 1.), rng.f(-1., 1.), rng.f(-1., 1.), rng.f(-1., 1.)));
        let mine = intersection_test(Isometry3::identity(), &om, Isometry3::from_parts(tr, uq), &om);
        let q = uq.quaternion();
        let piso = na::Isometry3::from_parts(
            na::Translation3::new(tr.x, tr.y, tr.z),
            na::UnitQuaternion::from_quaternion(na::Quaternion::new(q.w, q.i, q.j, q.k)));
        let ref_ = parry3d::query::intersection_test(&na::Isometry3::identity(), &pm, &piso, &pm).unwrap();
        if mine == ref_ { agree += 1 } else {
            disagree += 1;
            if shown < 8 { println!("  MISMATCH t=({:.3},{:.3},{:.3}) parry={ref_} ours={mine}", tr.x, tr.y, tr.z); shown += 1 }
        }
        if ref_ { hits += 1 }
    }
    println!("agree={agree} disagree={disagree}; parry reported intersecting in {hits}/600 cases");
    assert_eq!(disagree, 0, "{disagree}/600 disagreements with parry3d");
}
