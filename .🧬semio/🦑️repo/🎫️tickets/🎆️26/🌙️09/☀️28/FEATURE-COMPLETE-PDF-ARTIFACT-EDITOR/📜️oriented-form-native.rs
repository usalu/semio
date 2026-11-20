//! 🎛️ Runs the native clip algorithm against command-line polygon coordinates.

#[path = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖌️canvas2d/🧮️polygon-clipping/🦀️.rs"]
mod polygon_clipping;

fn main() {
    let points = std::env::args().skip(1).map(|arg| arg.parse::<f64>().unwrap()).collect::<Vec<_>>();
    let quad = [
        [points[0], points[1]],
        [points[2], points[3]],
        [points[4], points[5]],
        [points[6], points[7]],
    ];
    for [x, y, w, h] in polygon_clipping::polygon_clip_bands(&[quad], [0.0, 0.0, points[8], points[9]]) {
        println!("{x} {y} {w} {h}");
    }
}
