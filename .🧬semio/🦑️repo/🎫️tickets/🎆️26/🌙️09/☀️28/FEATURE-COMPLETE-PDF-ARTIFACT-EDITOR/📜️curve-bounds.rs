#[allow(dead_code)]
#[path = "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🧭️geometry/🧭️coordinates/🦀️.rs"]
mod coordinates;
fn main() {
    use coordinates::{VectorSegment, bounds, vector_bound_points};
    let path = [VectorSegment::Move((0.0, 0.0)), VectorSegment::Cubic { first: (0.0, 9.0), second: (6.0, 0.0), end: (9.0, 0.0) }];
    let (x, y, width, height) = bounds(&vector_bound_points(&path));
    assert_eq!((x, y, width), (0.0, 0.0, 9.0));
    assert!((height - 4.0).abs() < 1e-12);
    println!("{height}");
}
