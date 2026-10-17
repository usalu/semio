"""🏛 OSD: rewrites the module root (read model + facade) of semio-framework-os-directory."""
import re
M = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory"
p = M + "/🦀️.rs"
t = open(p, encoding="utf-8").read()
def sub1(old, new):
    global t
    assert t.count(old) == 1, (old[:70], t.count(old))
    t = t.replace(old, new)

a = t.index("// 🧭️ Resolves relative to THIS file's own physical directory")
b = t.index("use std::collections::BTreeMap;")
t = t[:a] + '''pub use semio_framework_os_directory_schema as schema;
pub use semio_framework_os_directory_access_policy as access_policy;

pub mod io {
    pub use semio_framework_os_directory_binary as binary;
    pub use semio_framework_os_directory_schema::text;
}

''' + t[b:]

CHECK = ["DocumentCheckInPhaseV1", "DocumentCheckInProgressV1", "DocumentCheckInReadyV1", "DocumentCheckInRefusalV1", "DocumentCheckInStatusV1", "DocumentCheckInV1", "DOCUMENT_CHECK_IN_MAX_BYTES", "DOCUMENT_CHECK_IN_SCHEMA_V1", "DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1"]
for name in CHECK:
    n = len(re.findall(r"\b" + name + r"\b,?\s?", t))
    assert n == 1, (name, n)
    t = re.sub(r"\b" + name + r"\b,\s?", "", t, count=1)
sub1("pub use schema::{CanonicalCheckpointPairRefusalV1, CanonicalCheckpointPairV1};", "pub use semio_framework_os_directory_binary::{CanonicalCheckpointPairRefusalV1, CanonicalCheckpointPairV1};\npub use semio_framework_os_directory_check_in::{DocumentCheckInPhaseV1, DocumentCheckInProgressV1, DocumentCheckInReadyV1, DocumentCheckInRefusalV1, DocumentCheckInStatusV1, DocumentCheckInV1, DOCUMENT_CHECK_IN_MAX_BYTES, DOCUMENT_CHECK_IN_SCHEMA_V1, DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1};")
sub1('#[path = "🚪️io/🦀️.rs"]\npub mod io;\n\n', "")
open(p, "w", encoding="utf-8").write(t)
