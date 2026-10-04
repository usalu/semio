"""🖨️ Converts the Raster handwritten SQLite owner to typed `ValueError` refusals (peer value-refusal migration, S4-INFRA pattern)."""
import pathlib
import re
import sys

FILE = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs")
text = FILE.read_text()
before = text


def once(old: str, new: str) -> None:
    global text
    found = text.count(old)
    if found != 1:
        sys.exit(f"anchor count {found} != 1: {old[:90]!r}")
    text = text.replace(old, new)


once(
    "use std::collections::BTreeMap;\n",
    "use std::collections::BTreeMap;\n"
    "use semio_framework_value::{ValueError,ValueRefusalKind};\n"
    "fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}\n"
    "fn work(message:&str)->ValueError{ValueError::new(ValueRefusalKind::WorkLimit,message)}\n"
    "fn positioned(error:ValueError)->semio_framework_diagnostic::TextError{semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1))}\n",
)
if re.search(r"Map<\s*String\s*,\s*String\s*>", text):
    sys.exit("map with String values present")
terminals = text.count(",String>")
text = text.replace(",String>", ",ValueError>")
for old, new in {
    "fn ordinal(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|e|e.to_string())}": "fn ordinal(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|_|work(\"Raster ordinal overflow\"))}",
    "validate_sqlite_database_schema(database,SCHEMA,control.limits()).map_err(|e|e.to_string())?;let mut rows": "validate_sqlite_database_schema(database,SCHEMA,control.limits())?;let mut rows",
    "u32::try_from(row.integer(index)?).map_err(|e|e.to_string())": "u32::try_from(row.integer(index)?).map_err(|_|invalid(\"Raster scalar exceeds unsigned32\"))",
    ".map_err(|e|e.reason.to_owned())?;": ".map_err(|e|invalid(e.reason))?;",
    "return Err(rejected.reason.into())": "return Err(invalid(rejected.reason))",
    "super::record::RasterNativeDocument::__dsl_from_record_controlled(record,native)?.into_snapshot(maximum_rows,native).map_err(|error|semio_framework_diagnostic::TextError::new(error,semio_framework_diagnostic::TextSpan::at(1,1)))": "super::record::RasterNativeDocument::__dsl_from_record_controlled(record,native).map_err(positioned)?.into_snapshot(maximum_rows,native).map_err(positioned)",
    "super::record::RasterNativeDocument::from_snapshot(self,maximum_rows,native).map_err(|error|semio_framework_diagnostic::TextError::new(error,semio_framework_diagnostic::TextSpan::at(1,1)))?;\n   document.__dsl_to_record_controlled(native)": "super::record::RasterNativeDocument::from_snapshot(self,maximum_rows,native).map_err(positioned)?;\n   document.__dsl_to_record_controlled(native).map_err(positioned)",
    "->store::io_schema::IoResult<()>{\n  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;": "->store::io_schema::IoResult<()>{\n  let io=store::io_schema::IoError::from_value_error;\n  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0).map_err(io)?;",
    'return Err(format!("unrecognized Raster semantic dialect {}",dialect.to_coordinate()).into())': 'return Err(io(invalid(format!("unrecognized Raster semantic dialect {}",dialect.to_coordinate()))))',
    "validate_sqlite_database_schema(database,SCHEMA,control.limits()).map_err(|e|e.to_string())?;\n  Ok(store::io_schema::IoOutcome::clean(()))": "validate_sqlite_database_schema(database,SCHEMA,control.limits()).map_err(io)?;\n  Ok(store::io_schema::IoOutcome::clean(()))",
}.items():
    once(old, new)
text, overflows = re.subn(r'\.ok_or\("(Raster (?:intrinsic )?frontier overflow)"\)', r'.ok_or_else(||work("\1"))', text)
text, missing = re.subn(r'\.ok_or\("([^"]+)"\)', r'.ok_or_else(||invalid("\1"))', text)
text, lazy = re.subn(r'\.ok_or_else\(\|\|"([^"]+)"\.into\(\)\)', r'.ok_or_else(||invalid("\1"))', text)
text, errs = re.subn(r'Err\("([^"]+)"\.into\(\)\)', r'Err(invalid("\1"))', text)
for residue in ['".into()', '.ok_or("', "e.to_string()", ",String>", "TextError::new(", ".reason.into()", ".reason.to_owned()"]:
    hits = [line[:160] for line in text.splitlines() if residue in line]
    if hits:
        sys.exit(f"residue {residue!r}: {hits[:3]}")
if FILE.read_text() != before:
    sys.exit("file changed during conversion")
FILE.write_text(text)
print(f"terminals={terminals} overflows={overflows} missing={missing} lazy={lazy} errs={errs}")
