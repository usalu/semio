from pathlib import Path
import re

lib = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
text = lib.read_text(encoding="utf-8")

mod_start = text.index("pub mod schema_gap_surfaces {")
mod_end = text.index("//#endregion 🩹️ schema_gap_surfaces", mod_start)
mod_text = text[mod_start:mod_end]

fm = re.search(
    r"macro_rules! gap_surface_family_name_list \{\s*\(@names\) => \{\s*(.*?)\s*\};\s*\(\) =>",
    mod_text,
    re.S,
)
rm = re.search(
    r"macro_rules! gap_surface_existing_relay_name_list \{\s*\(@names\) => \{\s*(.*?)\s*\};\s*\(\) =>",
    mod_text,
    re.S,
)
if not fm or not rm:
    raise SystemExit("could not extract @names blocks")

family_names = fm.group(1).strip()
relay_names = rm.group(1).strip()

named_start = mod_text.index("    gap_surface_family_named!(")
relay_with = mod_text.index("    #[macro_export]\n    macro_rules! with_gap_surface_existing_relay_names")
named_section = mod_text[named_start:relay_with]

head_end = mod_text.index("    #[macro_export]\n    macro_rules! gap_surface_family_name_list")
head = mod_text[:head_end]

tail = f"""    #[macro_export]
    macro_rules! gap_surface_family_name_list {{
        (@names) => {{
        {family_names}
        }};
        () => {{
            $crate::gap_surface_family_name_list!(@apply_families);
        }};
        (@apply_families) => {{
            gap_surface_families! {{
        {family_names}
            }}
        }};
        (@register $builder:expr) => {{
            $crate::gap_surface_family_name_list! {{
                @do_register_bridge $builder;
        {family_names}
            }}
        }};
        (@do_register_bridge $builder:expr; $($Name:ident),* $(,)?) => {{
            $crate::register_gap_surface_family_connections!(@do_register $builder, $($Name),*)
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {{
        (@names) => {{
        {relay_names}
        }};
        () => {{
            $crate::gap_surface_existing_relay_name_list!(@apply_relays);
        }};
        (@apply_relays) => {{
            gap_surface_existing_relays! {{
        {relay_names}
            }}
        }};
        (@register $builder:expr) => {{
            $crate::gap_surface_existing_relay_name_list! {{
                @do_register_bridge $builder;
        {relay_names}
            }}
        }};
        (@do_register_bridge $builder:expr; $($Name:ident),* $(,)?) => {{
            $crate::register_gap_surface_existing_relay_connections!(@do_register $builder, $($Name),*)
        }};
    }}

    #[macro_export]
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

{named_section}
    #[macro_export]
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
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! {{ [<$Name Connection>] }}>(); )+
            b
        }};
    }}

    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);
}}
"""

new_mod = head + tail
lib.write_text(text[:mod_start] + new_mod + text[mod_end:], encoding="utf-8")
print("rebuilt mod with inlined name lists", len(new_mod), "chars")
