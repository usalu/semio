from pathlib import Path
import re

p = Path(r"c:\git\compose\.repo\🎫\26\05\19\COMPOSE-RS-CLEAN-REFACTOR\rebuild_schema_gap_surfaces.py")
snap = Path(r"c:\git\compose\.repo\🎫\26\05\19\COMPOSE-RS-CLEAN-REFACTOR\rebuild-script-snapshot.txt")
t = snap.read_text(encoding="utf-8")
# find tail from prefix_end line onward in snapshot - read full snapshot
lines = t.splitlines()
# rebuild prefix_markers block
start = next(i for i,l in enumerate(lines) if l.startswith("prefix_markers"))
end = next(i for i,l in enumerate(lines[start:], start) if l.strip() == "]" and "prefix_end" not in l)
# find prefix_end line
end2 = next(i for i,l in enumerate(lines) if l.startswith("prefix_end"))
new_block = [
    "prefix_markers = [",
    '    "\\n    macro_rules! __gap_surface_family_name_idents",',
    '    "\\n    macro_rules! define_gap_surface_families_from_list",',
    '    "\\n    #[macro_export]\\n    macro_rules! gap_surface_families",',
    '    "\\n    macro_rules! gap_surface_families {",',
    '    "\\n    #[macro_export]\\n    macro_rules! gap_surface_family_name_list",',
    "]",
]
fixed = lines[:start] + new_block + lines[end2:]
text = "\n".join(fixed) + "\n"
# remove duplicate garbage if any between ] and prefix_end
text = re.sub(
    r"\]\n.*?(?=\nprefix_end =)",
    "]\n",
    text,
    count=1,
    flags=re.S,
)
p.write_text(text, encoding="utf-8")
print("fixed markers", "re.search" in text)
