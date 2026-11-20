from pathlib import Path
import json
import os

ticket = Path(__file__).parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").exists())
host = next(root.glob("*framework/*products/*os/*modules/*renderer/*engine/*elements/*IconRenderHost"))
projection = host / "📐️projection"
output = ticket / "🗑️generated" / "projected-linework"
output.mkdir(parents=True, exist_ok=True)
source = output / "probe.rs"

def rust_array(value):
    if isinstance(value, list):
        return "[" + ", ".join(rust_array(item) for item in value) + "]"
    return str(float(value))

geometry_fixture = json.loads(next(host.glob("*tests/*fixtures/*line-visibility/🔣️.json")).read_text())
linework_fixture = json.loads(next(projection.glob("*tests/*fixtures/🔣️.json")).read_text())

def modules():
    result = []
    for name in ("frustum", "visibility", "linework"):
        module = next(projection.glob(f"*{name}/🦀️.rs"))
        result.append(f'#[path = "{os.path.relpath(module, output)}"] mod {name};')
    return result

def clip_cases():
    for sample in geometry_fixture["frustum"]:
        expected = "None" if sample["clipped"] is None else f'Some({rust_array(sample["clipped"])})'
        yield f'assert_eq!(frustum::clip_line({rust_array(sample["input"])}), {expected});'

def visibility_cases():
    width = geometry_fixture["width"]
    height = geometry_fixture["height"]
    for sample in geometry_fixture["cases"]:
        yield f'check_visibility(({width}, {height}), {rust_array(sample["line"])}, &{rust_array(sample["triangles"])}, [&{rust_array(sample["visible"])}, &{rust_array(sample["hidden"])}]);'

def linework_cases():
    for sample in linework_fixture["linework"]:
        Rust_bool = str(sample["merge"]).lower()
        path = f'path(&{rust_array(sample["segments"])}, {Rust_bool}, {float(sample["easing"])})'
        if "path" in sample:
            yield f'assert_eq!({path}, {json.dumps(sample["path"])});'
        else:
            yield f'let rendered = {path};'
            yield f"assert_eq!(rendered.matches('M').count(), {sample['moves']});"
            yield f"assert_eq!(rendered.matches('Q').count(), {sample['curves']});"

def tests(name, clauses):
    return "#[test] fn " + name + "() {\n" + "\n".join(clauses) + "\n}"

source.write_text("\n".join([*modules(), """
fn check_visibility(dimensions: (u32, u32), points: [[f32;3];2], triangles: &[[[f32;3];3]], expected: [&[[f32;2]];2]) {
    let mut occluders = visibility::Occluders::new(dimensions);
    for &triangle in triangles { occluders.insert(triangle); }
    let mut line = occluders.line(points);
    let mut spans = [Vec::new(), Vec::new()];
    loop {
        match line.next(&occluders) {
            visibility::LineStep::Complete => break,
            visibility::LineStep::Pending => (),
            visibility::LineStep::Segment { points: segment, hidden } => spans[usize::from(hidden)].push(segment.map(|point| (point[0] - points[0][0]) / (points[1][0] - points[0][0]))),
        }
    }
    for (spans, expected) in spans.into_iter().zip(expected) {
        assert_eq!(spans.len(), expected.len());
        for (actual, expected) in spans.into_iter().flatten().zip(expected.iter().flatten()) {
            assert!((actual - expected).abs() < 1e-6);
        }
    }
}

fn path(segments: &[[[f32;2];2]], merge: bool, radius: f32) -> String {
    let mut graph = linework::LineGraph::default();
    for segment in segments { graph.insert(segment.map(|[x, y]| [x, y, 0.0])); }
    let mut path = String::new();
    while let Some(part) = graph.next(merge, radius) { path.push_str(&part); }
    assert!(graph.is_empty());
    path
}
""", tests("clip_geometry", clip_cases()), tests("visibility", visibility_cases()), tests("linework", linework_cases())]) + "\n")
print(source.relative_to(root))
