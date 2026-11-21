"""📘 Stages the registered application regression for large mixed-resource PDFs."""

import json
from pathlib import Path
import sys

ticket = Path(__file__).resolve().parent
stage = ticket / "🗑️generated/mixed"
base = Path(sys.argv[1]).resolve()
fixture_path = base / "🧪️tests/🧫️fixtures/🎞️controlled-images/🔣️.json"
source_path = base / "✏️editor/🧪️tests/🔬️unit/🦀️.rs"
fixture = json.loads(fixture_path.read_text())
fixture["mixedText"] = json.loads((ticket / "🧫️mixed-resources.json").read_text())["lines"]
(stage / "fixture.json").write_text(json.dumps(fixture, ensure_ascii=False, indent=2) + "\n")

def replace(source, old, new):
    assert source.count(old) == 1, old
    return source.replace(old, new)

source = source_path.read_text()
source = replace(source, 'fn sampled_fixture_bytes(spec: &serde_json::Value)', 'fn sampled_fixture_bytes(spec: &serde_json::Value, mixed: bool)')
source = replace(source, '        format!("<< /XObject << /{name} {object} 0 R >> >>")', '        let font = if mixed { "/Font << /F1 15 0 R >> " } else { "" };\n        format!("<< {font}/XObject << /{name} {object} 0 R >> >>")')
source = replace(source, r'''    let content = |image: &Value| {
        let name = image["name"].as_str().unwrap();
        stream("", format!("q {width} 0 0 {height} {x} {y} cm /{name} Do Q\n").as_bytes())
    };''', r'''    let content = |image: &Value, index: usize| {
        let name = image["name"].as_str().unwrap();
        let mut body = format!("q {width} 0 0 {height} {x} {y} cm /{name} Do Q\n");
        if mixed {
            let text = spec["mixedText"][index].as_str().unwrap();
            body.push_str(&format!("BT /F1 12 Tf 20 102 Td ({text}) Tj ET\n"));
        }
        stream("", body.as_bytes())
    };''')
first = source.index("    let rows = vec![", source.index("fn sampled_fixture_bytes"))
end = source.index("    (pdf_fixture_file(rows), source_samples)", first)
rows = source[first:end].replace("let rows = ", "let mut rows = ")
for index, image in enumerate([0, 1, 2, 0]):
    rows = rows.replace(f"content(&images[{image}])", f"content(&images[{image}], {index})", 1)
assert "content(&images[0])" not in rows
rows += '''    if mixed {
        rows.push(b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>".to_vec());
    }
'''
source = source[:first] + rows + source[end:]
source = replace(source, '    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::reference_image_sample_streams;',
                 '    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::{project_pdf_1_7, reference_image_sample_streams};')
first = source.index("    let (input, samples) = sampled_fixture_bytes(&spec);")
end = source.index("\n}", first)
test = source[first:end]
test = test.replace("sampled_fixture_bytes(&spec)", "sampled_fixture_bytes(&spec, mixed)")
test = test.replace("    let cold = ", '''    let verify_text = |data: &[u8]| {
        if mixed {
            let projected: serde_json::Value = serde_json::from_str(&project_pdf_1_7(data).unwrap().to_string()).unwrap();
            for (page, text) in projected["pages"].as_array().unwrap().iter().zip(spec["mixedText"].as_array().unwrap()) {
                assert_eq!(page["text"].as_array().unwrap().as_slice(), std::slice::from_ref(text));
            }
        }
    };
    verify_text(&input);
    let cold = ''')
test = test.replace("    assert_eq!(mounted.images, cold.images);", "    assert_eq!(mounted.images, cold.images);\n    assert_eq!(mounted.fonts, cold.fonts);")
test = test.replace("    assert_eq!(reference_image_sample_streams(&artifact.data).unwrap(), samples);", "    assert_eq!(reference_image_sample_streams(&artifact.data).unwrap(), samples);\n    verify_text(&artifact.data);")
source = source[:first] + "    for mixed in [false, true] {\n" + "\n".join("    " + line if line else "" for line in test.splitlines()) + "\n    }" + source[end:]
(stage / "editor-tests.rs").write_text(source)
