"""🧯️ FH1 family A: the store's schema-JSON decoders and the wgpu renderer's asset probe name their private refusal
helper `diagnose`/`refuse` — they build decode diagnostics / probe steps, never a declared `Fault`, so the fault
census must not read them as `.fault(code, text)` declarations."""
import re, sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply, regex
M = "🧰️framework/🛍️products/💻️os/🔨️modules/"
S = "🏪️store/🦀️.rs"
regex(M, S, r'self\.fault\("schema-json', 'self.diagnose("schema-json', 35)
apply(M, [
    (S, "    fn fault(&mut self, code: &'static str, token: OwnedSchemaToken, field_index: Option<u8>) -> OwnedSchemaRecordStep {", "    fn diagnose(&mut self, code: &'static str, token: OwnedSchemaToken, field_index: Option<u8>) -> OwnedSchemaRecordStep {"),
    (S, "    fn fault(&mut self, code: &'static str, token: OwnedSchemaToken, field_index: Option<u8>) -> OwnedSchemaNestedRecordStep {", "    fn diagnose(&mut self, code: &'static str, token: OwnedSchemaToken, field_index: Option<u8>) -> OwnedSchemaNestedRecordStep {"),
    (S, "    fn fault(&self, code: &'static str) -> OwnedSchemaHexStep {", "    fn diagnose(&self, code: &'static str) -> OwnedSchemaHexStep {"),
])
O = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults/"
R = O + M + "📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs"
lines = open(R).read().split("\n")
start = next(i for i, line in enumerate(lines) if "fn fault(&mut self, detail: &'static str) -> RendererAssetProbeStep {" in line)
impl_start = max(i for i in range(start) if lines[i].startswith("impl "))
count = 0
for i in range(impl_start, start + 1):
    new = lines[i].replace("self.fault(", "self.refuse(").replace("fn fault(&mut self, detail: &'static str) -> RendererAssetProbeStep {", "fn refuse(&mut self, detail: &'static str) -> RendererAssetProbeStep {")
    count += new != lines[i]
    lines[i] = new
assert count == 10, count
open(R, "w").write("\n".join(lines))
print("renderer lines renamed", count, "impl at", impl_start + 1)
