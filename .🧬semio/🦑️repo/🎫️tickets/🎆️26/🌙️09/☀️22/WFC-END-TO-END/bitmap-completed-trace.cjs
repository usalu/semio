
const fs=require('fs'),path=require('path');
let repositoryRoot=__dirname;
while(!fs.existsSync(path.join(repositoryRoot,'.git'))){const parent=path.dirname(repositoryRoot);if(parent===repositoryRoot)throw Error('Repository root required');repositoryRoot=parent;}
process.chdir(repositoryRoot);
const base='✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/',output=base+'✏️editor/🎭️modes/✏️edit/🪟️windows/🧩️output/',testFile=output+'🧪️tests/🔬️unit/🦀️.rs',fixture=path.relative(path.dirname(testFile),'✏️s/🔌️plugins/🀄️wfc/🧪️tests/🧫️completed-preview/🔣️.json').replaceAll('\\','/');
const originalTests=fs.readFileSync(testFile,'utf8'),runtimeFile=output+'🦀️.rs',originalRuntime=fs.readFileSync(runtimeFile,'utf8');
const oldTrace='for event in &payload.trace {',newTrace='for event in payload.trace.iter().filter(|_| !payload.done) {';
const begin=originalRuntime.indexOf('pub fn layers_from_fill_payload('),end=originalRuntime.indexOf('pub fn contradiction_layers(');
if(begin<0||end<=begin||originalRuntime.split(oldTrace).length!==2||originalRuntime.indexOf(oldTrace)<begin||originalRuntime.indexOf(oldTrace)>=end||originalRuntime.includes(newTrace))throw Error('Expected original bitmap trace frontier required');
const traceStart=originalRuntime.indexOf("    let mut undone = vec![false; cells];",begin),traceEnd=originalRuntime.indexOf('    crate::bitmap_layers_json("out", payload.width',traceStart);
if(traceStart<0||traceEnd<=traceStart||!originalRuntime.slice(traceStart,traceEnd).includes(oldTrace)||!originalRuntime.includes("    let mut markers = pins.to_vec();\n    let mut undone = vec![false; cells];"))throw Error("Expected original contiguous bitmap trace and discard-marker block required");
if(!originalTests.includes('use crate::editor::bitmap::modes::edit::windows::output::*;')||!originalTests.includes('encode_base64'))throw Error('Expected output test owner required');
const shared=path.resolve('✏️s/🔌️plugins/🀄️wfc/🧪️tests/🧫️completed-preview/🔣️.json');
if(JSON.parse(fs.readFileSync(shared,'utf8')).cases.length!==4)throw Error('Four portable completed-preview cases required');
let s=originalTests;if(s.includes('completed_payload_ignores_trace_pixels_and_discard_markers'))throw Error('bitmap trace test already applied');s+=`
/// 🏁️ Final pixels ignore every search trace, while partial pixels retain tried colors and discard markers.
#[test]
fn completed_payload_ignores_trace_pixels_and_discard_markers() {
    use crate::editor::bitmap::modes::edit::tools::fill::{BitmapFillPayload, BitmapFillTraceEvent};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("${fixture}")).unwrap();
    let mut document = crate::BitmapSnapshot::reference_scene();
    document.output.width = 1;
    document.output.height = 1;
    for case in fixture["cases"].as_array().unwrap() {
        let payload = BitmapFillPayload { pixels: encode_base64(&[case["assigned"].as_u64().unwrap() as u8]), decided: encode_base64(&[1]), width: 1, height: 1, contradiction: false, done: case["done"].as_bool().unwrap(), trace: vec![BitmapFillTraceEvent { index: 0, color: case["traced"].as_u64().unwrap() as u32, discarded: case["discarded"].as_bool().unwrap() }] };
        let layers: serde_json::Value = serde_json::from_str(&layers_from_fill_payload(&document, &payload, &[])).unwrap();
        let layers = layers.as_array().unwrap();
        let painted = layers.iter().find(|layer| layer["id"] == "out-0-0").unwrap();
        let color: Vec<f64> = serde_json::from_value(painted["fill"]["color"].clone()).unwrap();
        assert_eq!(color, document.input.palette[case["expected"].as_u64().unwrap() as usize].to_unit_rgba().to_vec(), "{}", case["id"]);
        assert_eq!(layers.iter().any(|layer| layer["id"] == "out-pin-0-0"), case["showDiscard"].as_bool().unwrap(), "{}", case["id"]);
    }
}
`;
if(fs.readFileSync(testFile,'utf8')!==originalTests||fs.readFileSync(runtimeFile,'utf8')!==originalRuntime)throw Error('Bitmap context changed before apply');
if(process.argv.includes('--check-context')){console.log('Prepared bitmap completed-trace patch: both original contexts valid, four cases present, no source writes.');process.exit(0);}
fs.writeFileSync(testFile,s);
const file=runtimeFile;s=fs.readFileSync(file,'utf8');if(s!==originalRuntime)throw Error('Runtime bitmap context changed after test write; do not overwrite concurrent work');const before='for event in &payload.trace {';if(!s.includes(before))throw Error('bitmap trace frontier missing');s=s.slice(0,traceStart)+"    if !payload.done {\n"+s.slice(traceStart,traceEnd).split("\n").map(line=>line?"    "+line:line).join("\n")+"    }\n"+s.slice(traceEnd);fs.writeFileSync(file,s);console.log('Applied prepared bitmap completed trace guard and shared portable pixel/marker regressions.');
