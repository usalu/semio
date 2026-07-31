#!/usr/bin/env python3
from pathlib import Path

p = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
t = p.read_text(encoding="utf-8")
start = t.index("    #[macro_export]\n    macro_rules! gap_surface_family_name_list {")
end = t.index("    #[macro_export]\n    macro_rules! with_gap_surface_family_names {", start)
replacement = """    #[macro_export]
    macro_rules! gap_surface_family_name_list {
        (@names) => { __gap_surface_family_name_idents!() };
        (@apply_families) => {
            gap_surface_families! { __gap_surface_family_name_idents!() }
        };
        (@register $builder:expr) => {
            register_gap_surface_family_connections!($builder, __gap_surface_family_name_idents!())
        };
    }

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {
        (@names) => { __gap_surface_existing_relay_name_idents!() };
        (@apply_relays) => {
            gap_surface_existing_relays! { __gap_surface_existing_relay_name_idents!() }
        };
        (@register $builder:expr) => {
            register_gap_surface_existing_relay_connections!($builder, __gap_surface_existing_relay_name_idents!())
        };
    }

"""
t = t[:start] + replacement + t[end:]
t = t.replace(
    "    #[macro_export]\n    #[macro_export]\n    macro_rules! with_gap_surface_existing_relay_names",
    "    #[macro_export]\n    macro_rules! with_gap_surface_existing_relay_names",
    1,
)
# Fix register macro block braces if broken
t = t.replace(
    """    macro_rules! register_gap_surface_family_connections {
        ($builder:expr, $($Name:ident),+ $(,)?) => {
            {
                let mut b = $builder;
                $( b = b.register_output_type::<::paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )+
                b
            }
        };
    }""",
    """    macro_rules! register_gap_surface_family_connections {
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<::paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )+
            b
        }};
    }""",
    1,
)
p.write_text(t, encoding="utf-8")
print("patched")
