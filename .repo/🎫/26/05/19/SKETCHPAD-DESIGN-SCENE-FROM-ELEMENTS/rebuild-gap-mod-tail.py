from pathlib import Path
import re

p = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
t = p.read_text(encoding="utf-8")

mod_start = t.index("pub mod schema_gap_surfaces {")
mod_end = t.index("//#endregion 🩹 schema_gap_surfaces", mod_start)
mod = t[mod_start:mod_end]

markers = [
    mod.find("    #[macro_export]\n    macro_rules! gap_surface_family_name_list {"),
    mod.find("    macro_rules! gap_surface_families {\n        { $($Name:ident),* $(,)? } => {\n            $(gap_surface_family!($Name);)+\n        };\n    }\n\n    macro_rules! gap_surface_existing_relays"),
    mod.find("    macro_rules! __gap_surface_family_name_idents"),
]
head_end = min(i for i in markers if i >= 0)
head = mod[:head_end]

def body_after(marker: str, src: str) -> str:
    i = src.index(marker)
    rest = src[i:].split("() => {", 1)[1]
    return rest.rsplit("};", 1)[0].strip()

family = body_after("macro_rules! __gap_surface_family_name_idents", mod)
relay = body_after("macro_rules! __gap_surface_existing_relay_name_idents", mod)

named_start = mod.rfind("    gap_surface_family_named!(")
version = mod.rfind('gap_surface_family_named!("Version"', named_start)
named_end = mod.index("\n", version) + 1
named_section = mod[named_start:named_end]

tail = f"""    #[macro_export]
    macro_rules! gap_surface_family_name_list {{
        (@names) => {{
        {family}
        }};
        () => {{
            $crate::gap_surface_family_name_list!(@apply_families);
        }};
        (@apply_families) => {{
            gap_surface_families! {{
        {family}
            }}
        }};
        (@register $builder:expr) => {{
            $crate::gap_surface_family_name_list! {{
                @do_register_bridge $builder;
        {family}
            }}
        }};
        (@do_register_bridge $builder:expr; $($Name:ident),* $(,)?) => {{
            $crate::register_gap_surface_family_connections!(@do_register $builder, $($Name),*)
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {{
        (@names) => {{
        {relay}
        }};
        () => {{
            $crate::gap_surface_existing_relay_name_list!(@apply_relays);
        }};
        (@apply_relays) => {{
            gap_surface_existing_relays! {{
        {relay}
            }}
        }};
        (@register $builder:expr) => {{
            $crate::gap_surface_existing_relay_name_list! {{
                @do_register_bridge $builder;
        {relay}
            }}
        }};
        (@do_register_bridge $builder:expr; $($Name:ident),* $(,)?) => {{
            $crate::register_gap_surface_existing_relay_connections!(@do_register $builder, $($Name),*)
        }};
    }}

    #[macro_export]
    macro_rules! with_gap_surface_family_names {{
        (gap_surface_families) => {{
            $crate::gap_surface_family_name_list!(@apply_families);
        }};
        (register_gap_surface_family_connections, $builder:expr) => {{
            $crate::gap_surface_family_name_list!(@register $builder)
        }};
    }}

    #[macro_export]
    macro_rules! register_gap_surface_family_connections {{
        (@do_register $builder:expr, $($Name:ident),* $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! {{ [<$Name Connection>] }}>(); )*
            b
        }};
    }}

    with_gap_surface_family_names!(gap_surface_families);

{named_section}    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {{
        (gap_surface_existing_relays) => {{
            $crate::gap_surface_existing_relay_name_list!(@apply_relays);
        }};
        (register_gap_surface_existing_relay_connections, $builder:expr) => {{
            $crate::gap_surface_existing_relay_name_list!(@register $builder)
        }};
    }}

    #[macro_export]
    macro_rules! register_gap_surface_existing_relay_connections {{
        (@do_register $builder:expr, $($Name:ident),* $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! {{ [<$Name Connection>] }}>(); )*
            b
        }};
    }}

    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);
}}
"""

new_mod = head + tail
t = t[:mod_start] + new_mod + t[mod_end:]
p.write_text(t, encoding="utf-8")
print("rebuilt tail", len(new_mod))
