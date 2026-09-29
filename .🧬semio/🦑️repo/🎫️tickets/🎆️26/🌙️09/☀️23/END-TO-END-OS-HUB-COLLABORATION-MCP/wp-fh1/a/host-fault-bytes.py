"""🖥️ FH1 family A — the host's effect and import completions refuse with literal codes: `fault_bytes` /
`emit_completed_err` take a `FaultCode` and every caller names its code."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply, regex
H = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/"
E = "⚡️effects/🦀️.rs"
I = "📥️imports/🦀️.rs"
regex(H, E, r'emit_completed_err\(&sink, &ctx_for_task, req, "([^"]+)"', r'emit_completed_err(&sink, &ctx_for_task, req, dsl::FaultCode::new("\1")', 13)
regex(H, I, r'fault_bytes\("([^"]+)"', r'fault_bytes(dsl::FaultCode::new("\1")', 35)
apply(H, [
    (E, '''async fn fault_bytes(code: &'static str, message: impl Into<String>) -> Vec<u8> {
    store::pack_rt::encode_wire_value(&dsl::ToValue::to_value(&dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new(code), message)))''', '''async fn fault_bytes(code: dsl::FaultCode, message: impl Into<String>) -> Vec<u8> {
    store::pack_rt::encode_wire_value(&dsl::ToValue::to_value(&dsl::Fault::new(dsl::FaultOrigin::Os, code, message)))'''),
    (E, '''async fn emit_completed_err<I: EnvelopeInjector>(sink: &Arc<EnvelopeCompletionSink<I>>, ctx: &OperationContext, req: RequestId, code: &str, message: impl Into<String>) {''', '''async fn emit_completed_err<I: EnvelopeInjector>(sink: &Arc<EnvelopeCompletionSink<I>>, ctx: &OperationContext, req: RequestId, code: dsl::FaultCode, message: impl Into<String>) {'''),
    (I, '''async fn fault_bytes(code: &'static str, message: impl Into<String>) -> Vec<u8> {
    let code = code.into();
    store::pack_rt::encode_wire_value(&dsl::ToValue::to_value(&dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new(code), message)))''', '''async fn fault_bytes(code: dsl::FaultCode, message: impl Into<String>) -> Vec<u8> {
    store::pack_rt::encode_wire_value(&dsl::ToValue::to_value(&dsl::Fault::new(dsl::FaultOrigin::Os, code, message)))'''),
])
