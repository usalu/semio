"""🖨️ Converts the Raster native record forest, its text/pack codecs and the owned-map decode to typed `ValueError` refusals."""
import pathlib
import re
import sys

ANY = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any")
RECORD = ANY / "🧬️schema/📸️snapshot/📦️record/🦀️.rs"
SNAPSHOT = ANY / "🧬️schema/📸️snapshot/🦀️.rs"
ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs")


def once(text: str, old: str, new: str) -> str:
    found = text.count(old)
    if found != 1:
        sys.exit(f"anchor count {found} != 1: {old[:90]!r}")
    return text.replace(old, new)


originals = {path: path.read_text() for path in (RECORD, SNAPSHOT, ROOT)}

record = originals[RECORD]
record = once(
    record,
    "use semio_framework_value::NativeDecodeControl;\n",
    "use semio_framework_value::NativeDecodeControl;\n"
    "use semio_framework_value::{ValueError,ValueRefusalKind};\n"
    "fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}\n"
    "fn limit(kind:ValueRefusalKind,message:&str)->ValueError{ValueError::new(kind,message)}\n",
)
if re.search(r"Map<\s*String\s*,\s*String\s*>", record):
    sys.exit("map with String values present")
terminals = record.count(",String>")
record = record.replace(",String>", ",ValueError>")
unwrapped = record.count(".map_err(semio_framework_value::ValueError::into_message)")
record = record.replace(".map_err(semio_framework_value::ValueError::into_message)", "")
record = once(record, 'pending.try_reserve_exact(1).map_err(|_|"Raster borrowed frontier allocation")?', 'pending.try_reserve_exact(1).map_err(|_|limit(ValueRefusalKind::AllocationFailed,"Raster borrowed frontier allocation"))?')
pages = record.count(".admit_one_page().map_err(str::to_owned)")
record = record.replace(".admit_one_page().map_err(str::to_owned)", ".admit_one_page().map_err(|reason|limit(ValueRefusalKind::OwnershipLimit,reason))")
record = once(record, ".map_err(|e|e.reason.to_owned())?", ".map_err(|e|invalid(e.reason))?")
record = once(record, "return Err(rejected.reason.into())", "return Err(invalid(rejected.reason))")
record = once(record, '.ok_or_else(||"Raster native workload overflow".into())', '.ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster native workload overflow"))')
record, ordinals = re.subn(r"\.map_err\(\|e\|e\.to_string\(\)\)", '.map_err(|_|invalid("Raster native ordinal exceeds the platform word"))', record)
record, overflows = re.subn(r'\.ok_or\("([^"]*(?:overflow|exceeds)[^"]*)"\)', r'.ok_or_else(||limit(ValueRefusalKind::WorkLimit,"\1"))', record)
record, limits = re.subn(r'Err\("([^"]*(?:exceeds row limit|map capacity)[^"]*)"\.into\(\)\)', r'Err(limit(ValueRefusalKind::OwnershipLimit,"\1"))', record)
record, missing = re.subn(r'\.ok_or\("([^"]+)"\)', r'.ok_or_else(||invalid("\1"))', record)
record, errs = re.subn(r'Err\("([^"]+)"\.into\(\)\)', r'Err(invalid("\1"))', record)
for residue in ['".into()', '.ok_or("', "e.to_string()", ",String>", "into_message", "str::to_owned", ".reason.into()", ".reason.to_owned()"]:
    hits = [line[:160] for line in record.splitlines() if residue in line]
    if hits:
        sys.exit(f"record residue {residue!r}: {hits[:3]}")

snapshot = originals[SNAPSHOT]
snapshot = once(snapshot, "?.ordinary_snapshot().map_err(|error|semio_framework_diagnostic::TextError::new(error,semio_framework_diagnostic::TextSpan::at(1,1)))", "?.ordinary_snapshot().map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1)))")
snapshot = once(snapshot, ".ordinary_snapshot().map_err(store::PackError::Schema)", ".ordinary_snapshot().map_err(store::PackError::ValueRefusal)")

root = originals[ROOT]
root = once(root, ".map_err(|rejected| semio_framework_value::ValueError::new(rejected.reason))", ".map_err(|rejected| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, rejected.reason))")

for path, text in ((RECORD, record), (SNAPSHOT, snapshot), (ROOT, root)):
    if path.read_text() != originals[path]:
        sys.exit(f"file changed during conversion: {path}")
for path, text in ((RECORD, record), (SNAPSHOT, snapshot), (ROOT, root)):
    path.write_text(text)
print(f"terminals={terminals} unwrapped={unwrapped} pages={pages} ordinals={ordinals} overflows={overflows} limits={limits} missing={missing} errs={errs}")
