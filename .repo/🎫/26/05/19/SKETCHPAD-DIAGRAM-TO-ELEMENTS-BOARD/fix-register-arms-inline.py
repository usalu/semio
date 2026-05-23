from pathlib import Path

p = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
text = p.read_text(encoding="utf-8")

text = text.replace("#[macro_export]\n    #[macro_export]", "#[macro_export]")


def fix_register_in_list(list_macro: str, register_macro: str) -> None:
    global text
    anchor = f"macro_rules! {list_macro} {{"
    i = text.index(anchor)
    j = text.index("(@names) => {", i)
    names_start = text.index("        Added", j)
    names_end = text.index("        };", names_start)
    names = text[names_start:names_end]

    reg_start = text.index("        (@register $builder:expr)", i)
    reg_end = text.index("        }};", reg_start) + len("        }};")
    new_reg = f"""        (@register $builder:expr) => {{
            $crate::{register_macro}!($builder,
{names}
            )
        }};"""
    text = text[:reg_start] + new_reg + text[reg_end:]
    print(f"fixed @register in {list_macro}")


fix_register_in_list("gap_surface_family_name_list", "register_gap_surface_family_connections")
fix_register_in_list("gap_surface_existing_relay_name_list", "register_gap_surface_existing_relay_connections")

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

p.write_text(text, encoding="utf-8")
print("done")
