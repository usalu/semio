import json
import subprocess
import sys
from pathlib import Path

lane = Path(sys.argv[1]).resolve()
macro = Path(sys.argv[2]).resolve()
ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
fixtures = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🧪️tests/🧫️fixtures").resolve()
source = generated / "layout-native-tagged-host-probe.rs"
objects = generated / "layout-native-host-deps"
objects.mkdir(parents=True, exist_ok=True)
suite = json.loads((fixtures / "🏷️tagged.json").read_text())
lookup = {}
for candidate in lane.rglob("lib*"):
    if candidate.is_file() and candidate.suffix in (".rmeta", ".rlib", ".dylib", ".so"):
        lookup.setdefault(candidate.name, candidate)
dsl = next(key for key in lookup if key.startswith("libsemio_os_dsl-") and key.endswith(".rmeta"))
header = lookup[dsl].with_name(dsl + ".semio-deps")
dependencies = json.loads(header.read_text())["dependencies"]

def extern(alias, dependency):
    target = objects / ("lib" + dependency)
    return ["--extern", f"{alias}={target}.rmeta", "--extern", f"{alias}={target}.rlib"]

for name, real in lookup.items():
    link = objects / name
    link.unlink(missing_ok=True)
    link.symlink_to(real)
value = next(name for name, _ in dependencies if name.startswith("semio_framework_value-"))
protocol = next(name for name, _ in dependencies if name.startswith("protocol-"))
text = r'''
use protocol::{OpBinary, ProtocolError};
use semio_os_dsl::original_receiving::{RetainedCloneGrant, ReceivingStock, receive_to_completion};
use semio_os_dsl::tagged_value_binary::{self, VariantTag};

const PROTOCOL: &str = include_str!(r#"PROTOCOL_PATH"#);
#[derive(Debug, PartialEq, semio_framework_value::FromValue, semio_framework_value::ToValue,
    semio_framework_value::RetireOwned, semio_framework_value_derive::NativeReceive)]
enum Mutation { Remove { text: String }, Rename { text: String } }

impl OpBinary for Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, ProtocolError> {
        tagged_value_binary::encode_op(PROTOCOL, VariantTag::Key, self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, ProtocolError> {
        tagged_value_binary::decode_op(PROTOCOL, VariantTag::Key, bytes)
    }
}
semio_os_dsl::original_pack_value_operation_receiving!(Mutation, tagged_key = PROTOCOL);

fn main() {
    let quantum = RetainedCloneGrant {
        maximum_items: 64, maximum_copy_bytes: 1 << 19,
        maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128,
    };
    let stock = ReceivingStock {
        stock: RetainedCloneGrant {
            maximum_items: 1 << 20,
            maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24,
            maximum_release_bytes: 1 << 24, ..quantum
        },
        quantum, fuel: 1 << 20,
    };
    let small = SMALL_WIRE;
    let expected = Mutation::decode_op(&small).unwrap();
    let mut surrounded = vec![0xff, 0xff];
    surrounded.extend_from_slice(&small);
    surrounded.push(0xff);
    let result = receive_to_completion::<Mutation>(&surrounded, (2, 2 + small.len()), stock);
    assert_eq!(result.unwrap(), expected);
    println!("Host funded the short authored golden");
    let Mutation::Rename { text } = expected else { panic!("authored tag must select Rename") };
    let large = Mutation::Rename { text: text.repeat(4096) };
    let wire = large.encode_op().unwrap();
    let bounded = ReceivingStock {
        quantum: RetainedCloneGrant { maximum_copy_bytes: 1 << 15, ..quantum }, ..stock
    };
    assert!(wire.len() > bounded.quantum.maximum_copy_bytes);
    assert_eq!(receive_to_completion::<Mutation>(&wire, (0, wire.len()), bounded).unwrap(), large);
    println!("Host funded a copy-bounded {}-byte tagged operation", wire.len());
}
'''
source.write_text(text.replace("PROTOCOL_PATH", str(fixtures / "🏷️protocol.semio"))
    .replace("SMALL_WIRE", str(suite["packWire"])))
binary = generated / "layout-native-tagged-host-probe"
subprocess.run(
    ["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
     "--edition=2024", "--crate-name=layout_native_tagged_host_probe",
     "-L", f"dependency={objects}", "--extern", f"semio_framework_value_derive={macro}",
     *extern("semio_os_dsl", dsl[3:-6]), *extern("semio_framework_value", value),
     *extern("protocol", protocol), str(source), "-o", str(binary)],
    check=True,
)
subprocess.run(["repo", "capacity", "run", "--memory=64", "--", str(binary)], check=True)
