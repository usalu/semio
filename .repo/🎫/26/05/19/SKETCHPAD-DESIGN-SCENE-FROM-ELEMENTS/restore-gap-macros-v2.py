from pathlib import Path
import re

lib = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
text = lib.read_text(encoding="utf-8")

family_start = text.index("#[macro_export]\n    macro_rules! gap_surface_family_name_list")
named_start = text.index("    gap_surface_family_named!(", family_start)
relay_macros_start = text.index("#[macro_export]\n    macro_rules! with_gap_surface_existing_relay_names", named_start)
mod_tail_end = text.index("    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);", relay_macros_start)
mod_tail_end += len("    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);")

family_block = text[family_start:named_start]
fm = re.search(r"\(@names\) => \{\s*(.*?)\s*\};", family_block, re.S)
if not fm:
    raise SystemExit("family @names not found")
family_names = fm.group(1).strip()

relay_list_start = text.index("#[macro_export]\n    macro_rules! gap_surface_existing_relay_name_list", family_start)
relay_block = text[relay_list_start:named_start]
rm = re.search(r"\(@names\) => \{\s*(.*?)\s*\};", relay_block, re.S)
if not rm:
    raise SystemExit("relay @names not found")
relay_names = rm.group(1).strip()

named_section = text[named_start:relay_macros_start]

tail = f"""    #[macro_export]
    macro_rules! gap_surface_family_name_list {{
        (@names) => {{
        {family_names}
        }};
        {{}} => {{
            gap_surface_family_name_list!(@names);
        }};
        (@register $builder:expr) => {{
            $crate::register_gap_surface_family_connections!(
                @do_register $builder,
                gap_surface_family_name_list!(@names)
            )
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {{
        (@names) => {{
        {relay_names}
        }};
        {{}} => {{
            gap_surface_existing_relay_name_list!(@names);
        }};
        (@register $builder:expr) => {{
            $crate::register_gap_surface_existing_relay_connections!(
                @do_register $builder,
                gap_surface_existing_relay_name_list!(@names)
            )
        }};
    }}

    #[macro_export]
    macro_rules! with_gap_surface_family_names {{
        (gap_surface_families) => {{
            $crate::schema_gap_surfaces::gap_surface_families! {{
                $crate::gap_surface_family_name_list!(@names)
            }}
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
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! {{ [<$Name Connection>] }}>(); )+
            b
        }};
    }}

    with_gap_surface_family_names!(gap_surface_families);

{named_section}
    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {{
        (gap_surface_existing_relays) => {{
            $crate::schema_gap_surfaces::gap_surface_existing_relays! {{
                $crate::gap_surface_existing_relay_name_list!(@names)
            }}
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
"""

new_text = text[:family_start] + tail + text[mod_tail_end + 1 :]
lib.write_text(new_text, encoding="utf-8")
print("rebuilt macros tail", len(tail), "chars")
