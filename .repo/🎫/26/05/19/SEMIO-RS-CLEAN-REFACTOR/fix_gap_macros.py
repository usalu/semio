"""Insert @register arms into gap name-list macros."""
from pathlib import Path

path = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
text = path.read_text(encoding="utf-8")

def insert_register(macro_name: str, register_macro: str) -> None:
    global text
    anchor = f"        {{}} => {{\n            {macro_name}!(@names);\n        }};"
    if anchor not in text:
        raise SystemExit(f"anchor not found for {macro_name}")
    start = text.index(f"macro_rules! {macro_name}")
    names_start = text.index("(@names) => {", start)
    names_end = text.index("\n        };", names_start)
    names_body = text[names_start + len("(@names) => {") : names_end]
    register_arm = f"""
        (@register $builder:expr) => {{
            $crate::{register_macro}! {{
                @expand $builder;{names_body}
            }}
        }}"""
    text = text.replace(anchor, anchor + register_arm, 1)
    print(f"ok {macro_name}")

insert_register("gap_surface_family_name_list", "register_gap_surface_family_connections")
insert_register("gap_surface_existing_relay_name_list", "register_gap_surface_existing_relay_connections")
path.write_text(text, encoding="utf-8")
