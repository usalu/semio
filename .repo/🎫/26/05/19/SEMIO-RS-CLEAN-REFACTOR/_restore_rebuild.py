from pathlib import Path
import re

p = Path(r"c:\git\semio\.repo\🎫\26\05\19\SEMIO-RS-CLEAN-REFACTOR\rebuild_schema_gap_surfaces.py")
# restore from snapshot if syntax broken
snap = Path(r"c:\git\semio\.repo\🎫\26\05\19\SEMIO-RS-CLEAN-REFACTOR\rebuild-script-snapshot.txt")
t = snap.read_text(encoding="utf-8")
lines = t.splitlines()
start = next(i for i, l in enumerate(lines) if l.startswith("prefix_markers"))
end2 = next(i for i, l in enumerate(lines) if l.startswith("prefix_end"))
new_block = [
    "prefix_markers = [",
    '    "\\n    macro_rules! gap_surface_families {",',
    '    "\\n    macro_rules! __gap_surface_family_name_idents",',
    '    "\\n    macro_rules! define_gap_surface_families_from_list",',
    '    "\\n    #[macro_export]\\n    macro_rules! gap_surface_families",',
    '    "\\n    #[macro_export]\\n    macro_rules! gap_surface_family_name_list",',
    "]",
]
fixed = lines[:start] + new_block + lines[end2:]
text = "\n".join(fixed) + "\n"
text = re.sub(r"\]\n.*?(?=\nprefix_end =)", "]\n", text, count=1, flags=re.S)
# ensure import re and region regex from fixed snapshot
if "import re" not in text:
    text = text.replace("import json", "import json\nimport re")
if "re.search" not in text:
    text = text.replace(
        'region_start = text.index("//#region',
        "_region_start_m = re.search(r\"//#region .+ schema_gap_surfaces\", text)\n_region_end_m = re.search(r\"//#endregion .+ schema_gap_surfaces\", text)\nif not _region_start_m or not _region_end_m:\n    raise ValueError(\"schema_gap_surfaces region markers not found\")\nregion_start = _region_start_m.start()\nregion_end = _region_end_m.start()\n# was region_start = text.index(\"//#region",
    )
p.write_text(text, encoding="utf-8")
import py_compile
py_compile.compile(str(p), doraise=True)
print("script ok")
