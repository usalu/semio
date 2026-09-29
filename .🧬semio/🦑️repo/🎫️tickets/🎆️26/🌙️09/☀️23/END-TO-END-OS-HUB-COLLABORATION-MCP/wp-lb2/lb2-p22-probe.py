#!/usr/bin/env python3
"""🔬️ LB2 p22 scratch-only probe: appends (or `--remove`s) an ignored shipped_fleet test that round-trips the jpg and gltf demo
documents through their DSL and pack codecs and writes both sides (neutral JSON + the codec bytes) into `$LB2_PROBE_OUT`, so the
first differing field and a third-party reading (PIL for the JPEG bytes) can be compared. Never applied to the live tree."""
import sys

root = sys.argv[1]
if root.rstrip("/") == "/Users/ueli/Documents/semio":
    sys.exit("the probe is scratch-only")
path = f"{root}/✏️s/🔌️plugins/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs"
PROBE = '''
#[test]
#[ignore = "LB2 p22 scratch probe"]
fn lb2_probe_round_trips() {
    use semio_framework_os_kernel::{ArtifactDsl, ArtifactPack, ToValue};
    let out = std::path::PathBuf::from(std::env::var("LB2_PROBE_OUT").expect("the probe directory"));
    let json = |value: semio_framework_os_kernel::DslValue| semio_framework_os_kernel::json::to_string_pretty(&semio_framework_os_kernel::json::from_dsl_value(&value));
    let jpg = <semio_s_artifact_stdio_jpg::JpgSnapshot as ArtifactDsl>::parse_dsl(semio_s_artifact_stdio_jpg::examples::demo::PRIMARY_TEXT).expect("jpg demo parses");
    let printed = jpg.print_dsl();
    let reparsed = <semio_s_artifact_stdio_jpg::JpgSnapshot as ArtifactDsl>::parse_dsl(&printed).expect("jpg reparses");
    std::fs::write(out.join("jpg-a.json"), json(jpg.to_value())).unwrap();
    std::fs::write(out.join("jpg-b.json"), json(reparsed.to_value())).unwrap();
    std::fs::write(out.join("jpg-printed.dsl"), &printed).unwrap();
    println!("[LB2-PROBE] jpg dsl equal={} pack equal={}", jpg == reparsed, <semio_s_artifact_stdio_jpg::JpgSnapshot as ArtifactPack>::decode_pack(&jpg.encode_pack()).is_ok_and(|reopened| reopened == jpg));
    let gltf = <semio_s_artifact_stdio_gltf::GltfSnapshot as ArtifactDsl>::parse_dsl(semio_s_artifact_stdio_gltf::examples::demo::PRIMARY_TEXT).expect("gltf demo parses");
    let pack = gltf.encode_pack();
    let reopened = <semio_s_artifact_stdio_gltf::GltfSnapshot as ArtifactPack>::decode_pack(&pack).expect("gltf reopens");
    std::fs::write(out.join("gltf-a.json"), json(gltf.to_value())).unwrap();
    std::fs::write(out.join("gltf-b.json"), json(reopened.to_value())).unwrap();
    std::fs::write(out.join("gltf.pack"), &pack).unwrap();
    println!("[LB2-PROBE] gltf pack equal={} dsl equal={}", gltf == reopened, <semio_s_artifact_stdio_gltf::GltfSnapshot as ArtifactDsl>::parse_dsl(&gltf.print_dsl()).is_ok_and(|again| again == gltf));
}
'''
text = open(path, encoding="utf-8").read()
if "--remove" in sys.argv:
    text = text.replace(PROBE, "")
elif PROBE not in text:
    text += PROBE
open(path, "w", encoding="utf-8").write(text)
print("probe", "removed" if "--remove" in sys.argv else "present")
