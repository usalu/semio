"""🧾️ Stage the PDF Pack Record seam without enabling the typed Store capability."""
from difflib import unified_diff
from pathlib import Path
from subprocess import run

TICKET = Path(__file__).resolve().parent
ROOT = Path.cwd()
SOURCE = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot")
MODULE = SOURCE / "🦀️.rs"
RECEIVER = SOURCE / "🫴️receiving" / "🦀️.rs"
OUT = TICKET / "🗑️generated" / "pdf-reopen-record"
OUT.mkdir(parents=True, exist_ok=True)

def diff(relative, before, after):
    assert len(str((ROOT / relative)).encode("utf-16-le")) // 2 <= 256
    prefix = f"diff --git a/{relative} b/{relative}\n"
    if before is None:
        prefix += "new file mode 100644\n"
    return prefix + "".join(unified_diff(
        [] if before is None else before.splitlines(keepends=True),
        after.splitlines(keepends=True),
        "/dev/null" if before is None else f"a/{relative}",
        f"b/{relative}",
    ))

before = MODULE.read_text()
anchor = "#[allow(unused_imports)]"
assert before.count(anchor) == 1 and not RECEIVER.exists()
after = before.replace(anchor, '#[allow(dead_code)]\n#[path = "🫴️receiving/🦀️.rs"]\nmod receiving;\n\n' + anchor)
wire = (TICKET / "pdf-paged-receive.rs").read_text()
output = OUT / "pdf-paged-record.patch"
output.write_text(diff(MODULE, before, after) + diff(RECEIVER, None, wire))
run(["git", "apply", "--check", str(output)], check=True)
