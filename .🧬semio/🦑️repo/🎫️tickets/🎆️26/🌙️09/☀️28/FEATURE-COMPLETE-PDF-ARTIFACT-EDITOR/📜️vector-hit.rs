#[allow(dead_code)]
#[path = "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🧭️geometry/🧭️coordinates/🦀️.rs"]
mod coordinates;
use coordinates::{PageViewport, VectorSegment, clip_quad, quad_contains, vector_contains};

fn rect(x: f64, y: f64, width: f64, height: f64) -> Vec<VectorSegment> {
    vec![VectorSegment::Move((x, y)), VectorSegment::Line((x + width, y)),
         VectorSegment::Line((x + width, y + height)), VectorSegment::Line((x, y + height)), VectorSegment::Close]
}

fn main() {
    let mut args = std::env::args().skip(1).map(|s| s.parse::<f64>().unwrap());
    let page = PageViewport::new([0.0, 0.0, 240.0, 160.0], None, 0, None, 0.0);
    let red = rect(40.0, 25.0, 40.0, 30.0);
    let mut green = rect(90.0, 25.0, 40.0, 40.0);
    green.extend(rect(100.0, 35.0, 20.0, 20.0));
    let mut blue = rect(145.0, 25.0, 40.0, 40.0);
    blue.extend(rect(155.0, 35.0, 20.0, 20.0));
    let clip = clip_quad(&[
        VectorSegment::Move((20.0, 20.0)), VectorSegment::Line((40.0, 30.0)),
        VectorSegment::Line((48.0, 46.0)), VectorSegment::Line((28.0, 36.0)), VectorSegment::Close,
    ]).unwrap().map(|(x, y)| [x, 100.0 - y]);
    assert!(quad_contains(clip, 35.5, 67.5));
    assert!(!quad_contains(clip, 22.5, 56.5));
    while let Some(x) = args.next() {
        let p = (x + 0.5, args.next().unwrap() + 0.5);
        let hit = vector_contains(&red, page, p, false, false, Some(3.0)) ||
            vector_contains(&green, page, p, true, true, None) || vector_contains(&blue, page, p, true, false, None);
        println!("{hit}");
    }
}
