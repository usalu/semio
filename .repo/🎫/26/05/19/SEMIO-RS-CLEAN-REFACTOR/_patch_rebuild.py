from pathlib import Path
import re

p = Path(r"c:\git\semio\.repo\🎫\26\05\19\SEMIO-RS-CLEAN-REFACTOR\rebuild_schema_gap_surfaces.py")
t = p.read_text(encoding="utf-8")
if "re.search" not in t:
    t = t.replace("import json", "import json\nimport re")
    t = re.sub(
        r"region_start = text\.index\([^\n]+\nregion_end = text\.index\([^\n]+\)",
        """_region_start_m = re.search(r"//#region .+ schema_gap_surfaces", text)
_region_end_m = re.search(r"//#endregion .+ schema_gap_surfaces", text)
if not _region_start_m or not _region_end_m:
    raise ValueError("schema_gap_surfaces region markers not found")
region_start = _region_start_m.start()
region_end = _region_end_m.start()""",
        t,
        count=1,
    )
prefix = """prefix_markers = [
    "\\n    macro_rules! __gap_surface_family_name_idents",
    "\\n    macro_rules! define_gap_surface_families_from_list",
    "\\n    #[macro_export]\\n    macro_rules! gap_surface_families",
    "\\n    macro_rules! gap_surface_families {",
    "\\n    #[macro_export]\\n    macro_rules! gap_surface_family_name_list",
]"""
t = re.sub(r"prefix_markers = \[.*?\]", prefix, t, count=1, flags=re.S)
p.write_text(t, encoding="utf-8")

lib = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
lt = lib.read_text(encoding="utf-8")
lt = re.sub(r"//#region .+ schema_gap_surfaces", "//#region \U0001fa79 schema_gap_surfaces", lt)
lt = re.sub(r"//#endregion .+ schema_gap_surfaces", "//#endregion \U0001fa79 schema_gap_surfaces", lt)
lib.write_text(lt, encoding="utf-8")
print("patched")
