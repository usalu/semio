"""🎞️ Wait for the offered Original media after metadata reaches readiness."""
from hashlib import sha256
from pathlib import Path
import sys

TICKET = Path(__file__).resolve().parent
PATH = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🖼️render/📥️staging/🏷️reserved-jobs/🦀️.rs")
BEFORE = '        if media.is_none(){return Err(semio_framework_value::ValueError::invariant("reserved media source is absent").into())}'
AFTER = '        if media.is_none(){return Ok(false)}'
DOC = '    /// 🎁️ Moves the supplied media and its paged port into the original input receiver.'
NEXT_DOC = DOC + '\n    /// A turn that first prepares the header can finish before the producer has offered the bytes.'
source = PATH.read_bytes()
text = source.decode("utf8")
if text.count(BEFORE) != 1 or text.count(DOC) != 1:
    raise SystemExit("Unexpected reserved media source")
next_source = text.replace(DOC, NEXT_DOC, 1).replace(BEFORE, AFTER, 1).encode("utf8")
print("[DEBUG] Before", sha256(source).hexdigest())
print("[DEBUG] After", sha256(next_source).hexdigest())
if len(sys.argv) == 2:
    if sha256(source).hexdigest() != sys.argv[1] or PATH.read_bytes() != source:
        raise SystemExit("Reserved media source changed before the edit")
    PATH.write_bytes(next_source)
