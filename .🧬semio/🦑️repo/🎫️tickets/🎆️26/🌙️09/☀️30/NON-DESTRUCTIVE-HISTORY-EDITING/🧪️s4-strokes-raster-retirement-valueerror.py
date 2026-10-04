"""🖨️ Converts the Raster binary retirement / clone / digest / one-item cursors to typed `ValueError` refusals (the framework `close_step` contract)."""
import pathlib
import re
import sys

FILE = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs")
text = FILE.read_text()
before = text


def once(old: str, new: str) -> None:
    global text
    found = text.count(old)
    if found != 1:
        sys.exit(f"anchor count {found} != 1: {old[:90]!r}")
    text = text.replace(old, new)


once(
    "use protocol::{Mutation, OpBinary};\n",
    "use protocol::{Mutation, OpBinary};\n"
    "use semio_framework_value::{ValueError, ValueRefusalKind};\n\n"
    "/// 🧯️ A retirement or clone cursor found its own bookkeeping inconsistent.\n"
    "fn invariant(message: &str) -> ValueError {\n"
    "    ValueError::new(ValueRefusalKind::InvariantViolated, message)\n"
    "}\n",
)
signatures, count = re.subn(r"(-> Result<[^;{]*?), String>( \{)", r"\1, ValueError>\2", text)
text = signatures
if count != 16:
    sys.exit(f"expected 16 String-error signatures, found {count}")
text, errs = re.subn(r'Err\("([^"]+)"\.into\(\)\)', r'Err(invariant("\1"))', text)
text, codes = re.subn(r"Err\(code\.into\(\)\)", "Err(invariant(code))", text)
text, lazy = re.subn(r'\.ok_or_else\(\|\| "([^"]+)"\.to_string\(\)\)', r'.ok_or_else(|| invariant("\1"))', text)
for residue in ['".into())', "Err(code.into())", '".to_string())', ", String> {"]:
    hits = [line.strip()[:160] for line in text.splitlines() if residue in line]
    if hits:
        sys.exit(f"residue {residue!r}: {hits[:3]}")
if FILE.read_text() != before:
    sys.exit("file changed during conversion")
FILE.write_text(text)
print(f"signatures={count} errs={errs} codes={codes} lazy={lazy}")
