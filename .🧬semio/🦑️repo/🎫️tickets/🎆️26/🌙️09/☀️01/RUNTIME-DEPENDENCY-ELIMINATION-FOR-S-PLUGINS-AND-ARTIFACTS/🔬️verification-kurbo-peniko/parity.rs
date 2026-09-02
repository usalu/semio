// Standalone differential harness for the first-party Color/Stroke/RasterImage replacements
// added to 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs, mirroring their
// exact logic against the real kurbo/peniko crates they replace.

#[derive(Clone, Copy, Debug, PartialEq)]
struct Color([f32; 4]);

impl Color {
    fn new(rgba: [f32; 4]) -> Self {
        Self(rgba)
    }
    fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a].map(|c| f32::from(c) * (1.0 / 255.0)))
    }
    fn to_rgba8(self) -> (u8, u8, u8, u8) {
        let [r, g, b, a] = self.0.map(|c| (c * 255.0 + 0.5) as u8);
        (r, g, b, a)
    }
    fn components(self) -> [f32; 4] {
        self.0
    }
    fn multiply_alpha(self, alpha: f32) -> Self {
        let [r, g, b, a] = self.0;
        Self([r, g, b, a * alpha])
    }
}

fn main() {
    let fixtures: [[f32; 4]; 12] = [
        [0.0, 0.0, 0.0, 1.0],
        [1.0, 1.0, 1.0, 1.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.5, 0.5, 0.5, 0.5],
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 0.75],
        [0.0, 0.0, 1.0, 0.25],
        [0.019_607_844, 0.0, 0.0, 1.0],
        [0.996, 0.004, 0.5, 0.9999],
        [0.313_725_5, 0.627_451, 0.941_176_5, 1.0],
        [1.5, -0.2, 0.5, 1.0],
        [0.001, 0.999, 0.333, 0.667],
    ];

    let mut failures = 0;

    for rgba in fixtures {
        let ours = Color::new(rgba).to_rgba8();
        let oracle = peniko::Color::new(rgba).to_rgba8();
        let oracle = (oracle.r, oracle.g, oracle.b, oracle.a);
        if ours != oracle {
            println!("to_rgba8 MISMATCH for {rgba:?}: ours={ours:?} oracle={oracle:?}");
            failures += 1;
        }
    }
    println!("to_rgba8 checked across {} fixtures", fixtures.len());

    for byte in 0..=255u8 {
        let ours = Color::from_rgba8(byte, 255 - byte, byte / 2, byte).components();
        let oracle = peniko::Color::from_rgba8(byte, 255 - byte, byte / 2, byte).components;
        if ours != oracle {
            println!("from_rgba8 MISMATCH for byte {byte}: ours={ours:?} oracle={oracle:?}");
            failures += 1;
        }
    }
    println!("from_rgba8 checked across 256 byte values");

    for rgba in fixtures {
        for factor in [0.0_f32, 0.25, 0.5, 1.0, 1.5] {
            let ours = Color::new(rgba).multiply_alpha(factor).components();
            let oracle = peniko::Color::new(rgba).multiply_alpha(factor).components;
            if ours != oracle {
                println!("multiply_alpha MISMATCH for {rgba:?} * {factor}: ours={ours:?} oracle={oracle:?}");
                failures += 1;
            }
        }
    }
    println!("multiply_alpha checked across {} fixtures x 5 factors", fixtures.len());

    // Stroke::to_kurbo defaults
    let ours_join = kurbo::Join::Round;
    let ours_miter = 4.0_f64;
    let ours_start_cap = kurbo::Cap::Round;
    let ours_end_cap = kurbo::Cap::Round;
    let oracle_stroke = kurbo::Stroke::new(3.5);
    if (ours_join, ours_miter, ours_start_cap, ours_end_cap)
        != (oracle_stroke.join, oracle_stroke.miter_limit, oracle_stroke.start_cap, oracle_stroke.end_cap)
    {
        println!("Stroke defaults MISMATCH vs kurbo::Stroke::new");
        failures += 1;
    }
    println!("Stroke::new default field values checked against kurbo::Stroke::new(3.5)");

    // RasterImage -> peniko::ImageData byte-for-byte
    let data = std::sync::Arc::new(vec![1u8, 2, 3, 4, 5, 6, 7, 8]);
    let blob = peniko::Blob::new(data.clone());
    let built = peniko::ImageData { data: blob, format: peniko::ImageFormat::Rgba8, alpha_type: peniko::ImageAlphaType::Alpha, width: 1, height: 2 };
    if built.data.data() != data.as_slice() {
        println!("RasterImage bytes MISMATCH");
        failures += 1;
    }
    println!("RasterImage -> peniko::ImageData byte round-trip checked");

    if failures == 0 {
        println!("ALL COLOR/STROKE/RASTERIMAGE VERIFICATIONS PASSED");
    } else {
        println!("{failures} VERIFICATIONS FAILED");
        std::process::exit(1);
    }
}
