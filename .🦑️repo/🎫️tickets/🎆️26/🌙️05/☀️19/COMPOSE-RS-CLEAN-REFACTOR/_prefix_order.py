from pathlib import Path
import re

p = Path(r"c:\git\compose\.repo\🎫️\26\05\19\COMPOSE-RS-CLEAN-REFACTOR\rebuild_schema_gap_surfaces.py")
t = p.read_text(encoding="utf-8")
prefix = """prefix_markers = [
    "\\n    macro_rules! gap_surface_families {",
    "\\n    macro_rules! __gap_surface_family_name_idents",
    "\\n    macro_rules! define_gap_surface_families_from_list",
    "\\n    #[macro_export]\\n    macro_rules! gap_surface_families",
    "\\n    #[macro_export]\\n    macro_rules! gap_surface_family_name_list",
]"""
t = re.sub(r"prefix_markers = \[.*?\]", prefix, t, count=1, flags=re.S)
p.write_text(t, encoding="utf-8")
print("updated prefix order")
