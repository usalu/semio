from pathlib import Path
p = Path(r"c:\git\semio\.repo\🎫\26\05\19\SEMIO-RS-CLEAN-REFACTOR\rebuild_schema_gap_surfaces.py")
t = p.read_text(encoding="utf-8")
# restore }} in f-string tail (fix broken replace)
t = t.replace(
    "    gap_surface_existing_relay_name_list!(@apply_relays);\n}\n\n\"\"\"",
    "    gap_surface_existing_relay_name_list!(@apply_relays);\n}}\n\n\"\"\"",
)
p.write_text(t, encoding="utf-8")
import py_compile
py_compile.compile(str(p), doraise=True)
print("restored f-string")
