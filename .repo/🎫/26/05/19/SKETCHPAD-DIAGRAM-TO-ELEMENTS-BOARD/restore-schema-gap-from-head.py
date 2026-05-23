import re
import subprocess
from pathlib import Path

root = Path(r"c:\git\semio")
head = subprocess.check_output(
    ["git", "show", "HEAD:semio/client/lib/rs/lib.rs"],
    cwd=root,
    text=True,
    encoding="utf-8",
)
cur_path = root / "semio/client/lib/rs/lib.rs"
cur = cur_path.read_text(encoding="utf-8")

m_head = re.search(
    r"//#region .*schema_gap_surfaces\n.*?//#endregion .*schema_gap_surfaces",
    head,
    re.S,
)
m_cur = re.search(
    r"//#region .*schema_gap_surfaces\n.*?//#endregion .*schema_gap_surfaces",
    cur,
    re.S,
)
if not m_head or not m_cur:
    raise SystemExit("regions not found")

new = cur[: m_cur.start()] + m_head.group(0) + cur[m_cur.end() :]
cur_path.write_text(new, encoding="utf-8")
print("restored schema_gap_surfaces from HEAD", len(m_head.group(0)), "bytes")
