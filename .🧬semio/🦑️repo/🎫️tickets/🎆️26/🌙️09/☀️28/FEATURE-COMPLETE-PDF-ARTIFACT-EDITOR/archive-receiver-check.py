"""🧪 Checks the staged receiver with existing native framework libraries."""

from pathlib import Path
import subprocess

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "archive-receiver"
CACHE = Path(".cache/b/r/m/u/l/pc5147e/prototype/build")
VARINT = Path("🧰️framework/🔨️modules/🎒️pack/📐️format/🎮️retained/🧱️segment-and-varint-cursors/🦀️.rs")
LIBRARIES = (
    "semio_framework_replication_codec", "semio_framework_replication_error", "semio_framework_dsl_record",
    "semio_framework_value", "semio_framework_value_native_owned", "semio_framework_pack_json", "serde_json",
)

source = VARINT.read_text(encoding="utf8")
start = source.index("#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n/// 🔢")
end = source.index("/// 🚦", start)
varint = source[start:end]
start = varint.index("impl RetainedVarintFault")
end = varint.index("#[derive(Clone, Copy, Debug, Default", start)
varint = varint[:start] + varint[end:]

harness = r'''
extern crate self as dsl;
extern crate self as pack;
extern crate self as store;
pub mod pack_rt { pub const OP_BINARY_FORMAT: u8 = 1; }
pub mod original_receiving {
    pub use semio_framework_replication_codec::{OpBinaryReceiving, OpBinaryReceivingCursor};
    pub use semio_framework_value_native_owned::OriginalReceivingPrimary;
}
use original_receiving::{OpBinaryReceiving, OpBinaryReceivingCursor};
use semio_framework_replication_codec::OpBinary;
use semio_framework_replication_error::ProtocolError;
use semio_framework_value::{AuthoringWallet, DslValue, FromValue, NativeDecodeControl, RetainedCloneGrant, RetainedCloneProgress, ValueError, ValueRefusalKind};
const MAX_PAYLOAD_BYTES: usize = 64 * 1024 * 1024;
const BINARY_TAG_REGISTRY: &[(&str, &str, u8)] = &[("InsertObject", "insertObject", 6), ("RemoveObject", "removeObject", 7)];
#[derive(Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct PdfMutation(DslValue);
impl FromValue for PdfMutation {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        if !matches!(value["id"]["num"], DslValue::Number(_)) {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "test fixture object number must be numeric"));
        }
        Ok(Self(value))
    }
}
impl OpBinary for PdfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, ProtocolError> { panic!("harness uses original fixture bytes") }
    fn decode_op(_: &[u8]) -> Result<Self, ProtocolError> { panic!("harness uses independent fixture value") }
}
#[path = "5.rs"]
mod receiving;

fn main() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("4.json")).unwrap();
    let json = fixture["json"].as_str().unwrap();
    let mut frame = vec![1u8, 6, 137, 1];
    frame.extend(json.bytes());
    let grant = RetainedCloneGrant {
        maximum_items: 64, maximum_copy_bytes: 1 << 19, maximum_capacity_bytes: 1 << 20,
        maximum_release_bytes: 1 << 20, maximum_depth: 128,
    };
    let stock = RetainedCloneGrant {
        maximum_items: 1 << 20, maximum_copy_bytes: 1 << 26, maximum_capacity_bytes: 1 << 26,
        maximum_release_bytes: 1 << 26, ..grant
    };
    let mut observed_ready = false;
    for end in [0, 1, 2, 3, 4, 5, 32, 150, 180, 250, 350, 450, 2048] {
        let mut wallet = AuthoringWallet::new(stock, 1 << 20);
        let mut active = true;
        let canceled = std::cell::Cell::new(false);
        let mut checkpoint = |_| !canceled.get();
        let mut native = NativeDecodeControl::new(stock.maximum_capacity_bytes, &mut checkpoint);
        let birth = PdfMutation::admit_receiving(&frame, (0, frame.len()), RetainedCloneGrant {maximum_copy_bytes: 0, ..grant});
        assert_eq!(birth.work, 1);
        let (mut cursor, receipt) = birth.result.unwrap().unwrap();
        assert_eq!(receipt.copied_bytes, 0);
        let mut total = RetainedCloneProgress::default();
        let mut quote = cursor.demands(&frame, grant.maximum_copy_bytes).unwrap();
        for index in 0..end {
            if cursor.is_ready() {
                observed_ready = true;
                println!("[DEBUG] specimen became ready after {index} paid advances");
                active = false;
                break;
            }
            quote = cursor.demands(&frame, grant.maximum_copy_bytes).unwrap();
            let denied = RetainedCloneGrant {maximum_copy_bytes: quote.copy_bytes - 1, ..grant};
            let blocked = wallet.run(denied, 1, |effect|
                cursor.step_in(&frame, &mut native, grant.maximum_copy_bytes, effect)).unwrap().unwrap();
            assert_eq!((blocked.output.work, blocked.output.result.unwrap(), blocked.actual), (0, RetainedCloneProgress::default(), None));
            let turn = wallet.run(grant, 1, |effect| cursor.step_in(&frame, &mut native, grant.maximum_copy_bytes, effect)).unwrap().unwrap().output;
            let progress = turn.result.unwrap();
            assert_eq!(turn.work, 1, "turn {index}: {quote:?}");
            wallet.post(progress).unwrap();
            total = total.checked_add(progress).unwrap();
        }
        if active {
            canceled.set(true);
            let denied = wallet.run(grant, 1, |effect| cursor.step_in(&frame, &mut native, grant.maximum_copy_bytes, effect)).unwrap().unwrap().output;
            assert_eq!(denied.work, 0);
            assert_eq!(denied.result.unwrap_err().refusal_kind(), Some(ValueRefusalKind::Canceled));
        } else {
            println!("[DEBUG] fixture bind {quote:?}; decoded physical balance {}", total.retained_capacity_bytes - total.released_bytes);
            let mut output = None;
            let moved = cursor.take_into(&mut output, grant);
            assert_eq!(moved.work, 1);
            let output = output.unwrap();
            assert_eq!(output.0["mutation"].as_str(), Some("insertObject"));
        }
        for _ in 0..4096 {
            if cursor.terminal_is_empty() { break; }
            let (turn, _) = wallet.run_settled(grant, 64, |effect| cursor.close_step(effect)).unwrap().unwrap();
            let progress = turn.result.unwrap().progress();
            assert!(turn.work > 0);
            wallet.post(progress).unwrap();
            total = total.checked_add(progress).unwrap();
        }
        assert!(cursor.terminal_is_empty(), "{end}");
        if active {
            assert_eq!(total.released_bytes, total.retained_capacity_bytes, "{end}: {total:?}");
        }
    }
    assert!(observed_ready);
}
'''

probe = OUT / "probe.rs"
probe.write_text("pub mod format {\n" + varint + "\n}\n" + harness, encoding="utf8")
arguments = ["--edition=2024", str(probe), "-o", str(OUT / "probe")]
for name in LIBRARIES:
    [library] = CACHE.glob(f"*/*/out/lib{name}-*.rlib")
    arguments.extend(("--extern", f"{name}={library.with_suffix('.rmeta')}", "--extern", f"{name}={library}"))
arguments.extend(f"-Ldependency={path}" for path in sorted({path.parent for path in CACHE.glob("*/*/out/*") if path.suffix in (".rmeta", ".rlib", ".dylib")}))
argfile = OUT / "probe.args"
argfile.write_text("\n".join(arguments) + "\n", encoding="utf8")
subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(argfile)], check=True)
subprocess.run(["repo", "capacity", "run", "--memory=64", "--", str(OUT / "probe")], check=True)
