#!/usr/bin/env python3
"""🔬️ LB2 p19 scratch-only probe: appends (or `--remove`s) an ignored shipped_fleet test that writes every stdio package's
natively described descriptor as JSON into `$LB2_DESCRIPTOR_OUT/<package>.json`. Never applied to the live tree."""
import sys

root = sys.argv[1]
if root.rstrip("/") == "/Users/ueli/Documents/semio":
    sys.exit("the dump probe is scratch-only")
path = f"{root}/✏️s/🔌️plugins/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs"
PROBE = '''
#[test]
#[ignore = "LB2 p19 scratch probe"]
fn lb2_probe_dump_descriptors() {
    let out = std::path::PathBuf::from(std::env::var("LB2_DESCRIPTOR_OUT").expect("the dump directory"));
    for package in packages() {
        let value = semio_framework::to_dsl_value(&package.descriptor).expect("descriptor value");
        let json = semio_framework_os_kernel::json::to_string_pretty(&semio_framework_os_kernel::json::from_dsl_value(&value));
        std::fs::write(out.join(format!("{}.json", package.id)), json).expect("the descriptor dump");
    }
}
'''
text = open(path, encoding="utf-8").read()
if "--remove" in sys.argv:
    text = text.replace(PROBE, "")
elif PROBE not in text:
    text += PROBE
open(path, "w", encoding="utf-8").write(text)
print("probe", "removed" if "--remove" in sys.argv else "present")
