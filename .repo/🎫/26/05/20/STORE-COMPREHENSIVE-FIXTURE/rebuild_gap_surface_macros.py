"""Rebuild gap_surface_* name-list macros without duplicated inline expansions."""
from pathlib import Path
import re

p = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
lines = p.read_text(encoding="utf-8").splitlines(keepends=True)


def extract_names_block(start_marker: str, next_marker: str) -> str:
    text = "".join(lines)
    i0 = text.index(start_marker)
    i1 = text.index(next_marker, i0)
    block = text[i0:i1]
    m = re.search(r"\(@names\)\s*=>\s*\{([^}]+)\}", block, re.S)
    if not m:
        raise SystemExit(f"no @names in {start_marker!r}")
    body = m.group(1).strip()
    if not body.endswith(","):
        body += ","
    return body


family_names = extract_names_block(
    "macro_rules! gap_surface_family_name_list",
    "macro_rules! gap_surface_existing_relay_name_list",
)
relay_names = extract_names_block(
    "macro_rules! gap_surface_existing_relay_name_list",
    "macro_rules! with_gap_surface_family_names",
)

clean = f'''    #[macro_export]
    macro_rules! gap_surface_families {{
        {{ $($Name:ident),* $(,)? }} => {{
            $(gap_surface_family!($Name);)+
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relays {{
        {{ $($Name:ident),* $(,)? }} => {{
            $(gap_surface_existing_relay!($Name);)+
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_family_name_list {{
        (@names) => {{
        {family_names}
        }};
        (@apply_families) => {{
            gap_surface_family_name_list! {{
                @emit_families;
                gap_surface_family_name_list!(@names)
            }};
        }};
        (@emit_families; $($Name:ident),* $(,)?) => {{
            gap_surface_families! {{ $($Name),* }}
        }};
        (@register $builder:expr) => {{
            gap_surface_family_name_list! {{
                @do_register_bridge $builder,
                gap_surface_family_name_list!(@names)
            }}
        }};
        (@do_register_bridge $builder:expr, $($Name:ident),* $(,)?) => {{
            $crate::register_gap_surface_family_connections!(@expand $builder, $($Name),*)
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {{
        (@names) => {{
        {relay_names}
        }};
        (@apply_relays) => {{
            gap_surface_existing_relay_name_list! {{
                @emit_relays;
                gap_surface_existing_relay_name_list!(@names)
            }};
        }};
        (@emit_relays; $($Name:ident),* $(,)?) => {{
            gap_surface_existing_relays! {{ $($Name),* }}
        }};
        (@register $builder:expr) => {{
            gap_surface_existing_relay_name_list! {{
                @do_register_bridge $builder,
                gap_surface_existing_relay_name_list!(@names)
            }}
        }};
        (@do_register_bridge $builder:expr, $($Name:ident),* $(,)?) => {{
            $crate::register_gap_surface_existing_relay_connections!(@expand $builder, $($Name),*)
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
        (@expand $builder:expr, $($Name:ident),* $(,)?) => {{
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

'''

# splice into module
text = "".join(lines)
i0 = text.index("    macro_rules! gap_surface_families")
i1 = text.index("    gap_surface_family_named!(")
new_text = text[:i0] + clean + text[i1:]

# fix relay tail if duplicated
i2 = new_text.index("    gap_surface_family_named!(")
i3 = new_text.index("//#endregion", i2)
tail = new_text[i2:i3]
if "with_gap_surface_existing_relay_names" not in tail:
    relay_tail = '''
    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {
        (gap_surface_existing_relays) => {
            $crate::gap_surface_existing_relay_name_list!(@apply_relays);
        };
        (register_gap_surface_existing_relay_connections, $builder:expr) => {
            $crate::gap_surface_existing_relay_name_list!(@register $builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_existing_relay_connections {
        (@expand $builder:expr, $($Name:ident),* $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! { [<$Name Connection>] }>(); )*
            b
        }};
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! { [<$Name Connection>] }>(); )+
            b
        }};
    }

    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);

'''
    # insert before endregion inside module
    end_mod = new_text.rindex("\n}", 0, i3)
    new_text = new_text[:end_mod] + relay_tail + new_text[end_mod:]

p.write_text(new_text, encoding="utf-8")
print("rebuilt gap macros", len(family_names.splitlines()), "family names")
