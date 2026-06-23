from pathlib import Path
import re

p = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
text = p.read_text(encoding="utf-8")

def patch_list(name: str, families_macro: str, register_macro: str) -> None:
    global text
    marker = f"macro_rules! {name} {{"
    start = text.index(marker)
    end = text.index("\n    #[macro_export]", start + len(marker))
    if name == "gap_surface_existing_relay_name_list":
        for needle in (
            "\n    #[macro_export]\n    macro_rules! with_gap_surface_family_names",
            "\n    macro_rules! with_gap_surface_family_names",
            "\n    gap_surface_family_named!(",
        ):
            try:
                end = text.index(needle, start)
                break
            except ValueError:
                continue
    block = text[start:end]
    nm = re.search(r"\(@names\) => \{\s*(.*?)\s*\};", block, re.S)
    if not nm:
        raise SystemExit(f"no @names in {name}")
    names = nm.group(1).strip()
    new_block = f"""macro_rules! {name} {{
        (@names) => {{
        {names}
        }};
        (@apply_families) => {{
            {families_macro}! {{
        {names}
            }}
        }};
        (@apply_relays) => {{
            {families_macro}! {{
        {names}
            }}
        }};
        (@register $builder:expr) => {{
            $crate::{name}! {{
                @do_register_bridge $builder;
        {names}
            }}
        }};
        (@do_register_bridge $builder:expr; $($Name:ident),* $(,)?) => {{
            $crate::{register_macro}!(@do_register $builder, $($Name),*)
        }};
    }}"""
    if "relay" in name:
        new_block = new_block.replace("(@apply_families)", "(@apply_relays)", 1).replace(
            "gap_surface_families!", "gap_surface_existing_relays!", 1
        )
    text = text[:start] + new_block + text[end:]

patch_list(
    "gap_surface_family_name_list",
    "gap_surface_families",
    "register_gap_surface_family_connections",
)
patch_list(
    "gap_surface_existing_relay_name_list",
    "gap_surface_existing_relays",
    "register_gap_surface_existing_relay_connections",
)

text = text.replace(
    "    gap_surface_family_name_list!(@apply_families);",
    "    with_gap_surface_family_names!(gap_surface_families);",
)
text = text.replace(
    "    gap_surface_existing_relay_name_list!(@apply_relays);",
    "    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);",
)

text = re.sub(
    r"macro_rules! register_gap_surface_family_connections \{\s*\(@expand[^}]+\}\};\s*\}",
    """macro_rules! register_gap_surface_family_connections {
        (@do_register $builder:expr, $($Name:ident),* $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! { [<$Name Connection>] }>(); )*
            b
        }};
    }""",
    text,
    count=1,
    flags=re.S,
)

text = re.sub(
    r"macro_rules! register_gap_surface_existing_relay_connections \{\s*\(@expand[^}]+\}\};\s*\}",
    """macro_rules! register_gap_surface_existing_relay_connections {
        (@do_register $builder:expr, $($Name:ident),* $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! { [<$Name Connection>] }>(); )*
            b
        }};
    }""",
    text,
    count=1,
    flags=re.S,
)

if "macro_rules! with_gap_surface_family_names" in text and "#[macro_export]\n    macro_rules! with_gap_surface_family_names" not in text:
    text = text.replace(
        "    macro_rules! with_gap_surface_family_names {",
        "    #[macro_export]\n    macro_rules! with_gap_surface_family_names {",
        1,
    )

p.write_text(text, encoding="utf-8")
print("patched")
