from pathlib import Path
p = Path(r"c:\git\compose\.repo\🎫\26\05\19\COMPOSE-RS-CLEAN-REFACTOR\rebuild_schema_gap_surfaces.py")
t = p.read_text(encoding="utf-8")
t = t.replace(
    "    gap_surface_existing_relay_name_list!(@apply_relays);\n}}\n\n\"\"\"",
    "    gap_surface_existing_relay_name_list!(@apply_relays);\n}\n\n\"\"\"",
)
p.write_text(t, encoding="utf-8")
print("fixed tail brace")
