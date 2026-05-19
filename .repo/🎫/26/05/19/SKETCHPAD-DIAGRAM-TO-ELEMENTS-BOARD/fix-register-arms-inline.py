import re
from pathlib import Path

p = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
text = p.read_text(encoding="utf-8")

text = text.replace("#[macro_export]\n    #[macro_export]", "#[macro_export]")

for macro_name, register_macro in [
    ("gap_surface_family_name_list", "register_gap_surface_family_connections"),
    ("gap_surface_existing_relay_name_list", "register_gap_surface_existing_relay_connections"),
]:
    pattern = (
        rf"macro_rules! {re.escape(macro_name)} \{{\s*"
        r"\(@names\) => \{\s*"
        r"(.*?)"
        r"\s*\};\s*"
        r"\{\} => \{[^}]+\};\s*"
        r"\(@register \$builder:expr\) => \{\{[^}}]+\}\};\s*"
        r"\}"
    )
    m = re.search(pattern, text, re.S)
    if not m:
        raise SystemExit(f"pattern not found for {macro_name}")
    names = m.group(1).strip()
    replacement = f"""macro_rules! {macro_name} {{
        (@names) => {{
        {names}
        }};
        (@register $builder:expr) => {{
            $crate::{register_macro}!($builder,
        {names}
            )
        }};
    }}"""
    text = text[: m.start()] + replacement + text[m.end() :]
    print(f"fixed {macro_name}")

text = text.replace(
    "$crate::schema_gap_surfaces::gap_surface_families! {",
    "gap_surface_families! {",
)
text = text.replace(
    "$crate::schema_gap_surfaces::gap_surface_existing_relays! {",
    "gap_surface_existing_relays! {",
)
text = text.replace(
    "$crate::schema_gap_surfaces::paste::paste!",
    "paste::paste!",
)
text = text.replace(
    "            $crate::gap_surface_family_name_list!(@register $builder)\n",
    "            $crate::gap_surface_family_name_list!(@register $builder)\n",
)
text = text.replace(
    "            $crate::gap_surface_existing_relay_name_list!(@register $builder);\n",
    "            $crate::gap_surface_existing_relay_name_list!(@register $builder)\n",
)

p.write_text(text, encoding="utf-8")
print("done")
