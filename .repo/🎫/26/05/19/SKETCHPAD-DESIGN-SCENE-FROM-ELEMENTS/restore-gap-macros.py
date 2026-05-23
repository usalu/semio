from pathlib import Path
import subprocess
import re

repo = Path(r"c:\git\semio")
lib = repo / "semio/client/lib/rs/lib.rs"
text = lib.read_text(encoding="utf-8")
head = subprocess.check_output(
    ["git", "-C", str(repo), "show", "HEAD:semio/client/lib/rs/lib.rs"],
).decode("utf-8")

start_marker = "//#region 🩹 schema_gap_surfaces"
end_marker = "//#endregion 🩹 schema_gap_surfaces"
s = text.index(start_marker)
e = text.index(end_marker, s) + len(end_marker)

hs = head.index(start_marker)
he = head.index(end_marker, hs) + len(end_marker)
section = head[hs:he]

section = section.replace(
    """        (@register $builder:expr) => {{
            $crate::register_gap_surface_family_connections! {
                @expand $builder;
                gap_surface_family_name_list!(@names)
            }
        }};""",
    """        (@register $builder:expr) => {{
            $crate::register_gap_surface_family_connections!(
                @do_register $builder,
                gap_surface_family_name_list!(@names)
            )
        }};""",
)

section = section.replace(
    """        (@register $builder:expr) => {{
            $crate::register_gap_surface_existing_relay_connections! {
                @expand $builder;
                gap_surface_existing_relay_name_list!(@names)
            }
        }};""",
    """        (@register $builder:expr) => {{
            $crate::register_gap_surface_existing_relay_connections!(
                @do_register $builder,
                gap_surface_existing_relay_name_list!(@names)
            )
        }};""",
)

section = section.replace(
    "macro_rules! register_gap_surface_family_connections {\n        (@expand $builder:expr;",
    "macro_rules! register_gap_surface_family_connections {\n        (@do_register $builder:expr,",
    1,
)
section = section.replace(
    "macro_rules! register_gap_surface_existing_relay_connections {\n        (@expand $builder:expr;",
    "macro_rules! register_gap_surface_existing_relay_connections {\n        (@do_register $builder:expr,",
    1,
)

if "macro_rules! with_gap_surface_family_names" in section and "#[macro_export]\n    macro_rules! with_gap_surface_family_names" not in section:
    section = section.replace(
        "    macro_rules! with_gap_surface_family_names {",
        "    #[macro_export]\n    macro_rules! with_gap_surface_family_names {",
        1,
    )

lib.write_text(text[:s] + section + text[e:], encoding="utf-8")
print("restored schema_gap_surfaces", len(section), "chars")
