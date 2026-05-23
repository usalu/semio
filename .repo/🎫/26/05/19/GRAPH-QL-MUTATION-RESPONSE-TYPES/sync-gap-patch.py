"""Copy schema_gap_surfaces region from lib.rs into canonical patch file."""
from pathlib import Path

lib = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs").read_text(encoding="utf-8")
start = lib.index("//#region 🩹 schema_gap_surfaces")
end = lib.index("//#endregion schema_gap_surfaces") + len("//#endregion schema_gap_surfaces")
patch = Path(r"c:\git\semio\.repo\🎫\26\05\19\GRAPH-QL-MUTATION-RESPONSE-TYPES\schema_gap_surfaces-patch.rs")
patch.write_text(lib[start:end] + "\n", encoding="utf-8", newline="\n")
print("patch synced", patch.stat().st_size, "bytes")
