"""Restore lib.rs from HEAD and apply finalized schema_gap_surfaces only."""
import re
import subprocess
from pathlib import Path

root = Path(r"c:\git\compose")
lib_path = root / "compose/client/lib/rs/lib.rs"

# reuse finalize-gap-macros.py by setting cur from HEAD before region replace
finalize_path = root / ".repo/🎫️/26/05/19/SKETCHPAD-DIAGRAM-TO-ELEMENTS-BOARD/finalize-gap-macros.py"
code = finalize_path.read_text(encoding="utf-8")
code = code.replace(
    "cur = cur_path.read_text(encoding=\"utf-8\")",
    "cur = subprocess.check_output(\n"
    '    ["git", "show", "HEAD:compose/client/lib/rs/lib.rs"],\n'
    "    cwd=root,\n"
    "    text=True,\n"
    "    encoding=\"utf-8\",\n"
    ")",
)
exec(compile(code, str(finalize_path), "exec"), {"__name__": "__main__"})
