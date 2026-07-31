#!/usr/bin/env python3
"""Restore gap-surface macro block in compose/client/lib/rs/lib.rs (dedupe + bridge pattern)."""
from pathlib import Path

p = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
text = p.read_text(encoding="utf-8")
lines = text.splitlines(keepends=True)

start = next(i for i, ln in enumerate(lines) if "macro_rules! __gap_surface_family_name_idents" in ln)
end = next(
    i
    for i, ln in enumerate(lines)
    if i > start and ln.strip() == "//#endregion" and "schema_gap_surfaces" in ln
)

family_idents = []
relay_idents = []
mode = None
for ln in lines[start:end]:
    s = ln.strip().rstrip(",")
    if "macro_rules! __gap_surface_family_name_idents_removed" in ln:
        mode = "family"
        continue
    if "macro_rules! __gap_surface_existing_relay_name_idents" in ln and mode != "relay_capture":
        if mode == "family":
            mode = "relay"
            continue
    if mode == "family" and s and s not in ("() => {", "};", "{"):
        if s.isidentifier():
            family_idents.append(s)
    if mode == "relay" and s and s not in ("() => {", "};", "{"):
        if s.isidentifier():
            relay_idents.append(s)

if not family_idents:
    raise SystemExit("could not extract family idents")
if not relay_idents:
    raise SystemExit("could not extract relay idents")

def ident_block(idents: list[str], indent: str) -> str:
    return "\n".join(f"{indent}{name}," for name in idents) + "\n"

replacement = f"""    macro_rules! __gap_surface_family_name_idents {{
        () => {{
{ident_block(family_idents, "        ")}        }};
    }}

    macro_rules! __gap_surface_existing_relay_name_idents {{
        () => {{
{ident_block(relay_idents, "        ")}        }};
    }}

    macro_rules! gap_surface_families {{
        {{ $($Name:ident),* $(,)? }} => {{
            $(gap_surface_family!($Name);)+
        }};
    }}

    macro_rules! gap_surface_existing_relays {{
        {{ $($Name:ident),* $(,)? }} => {{
            $(gap_surface_existing_relay!($Name);)+
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_family_name_list {{
        (@names) => {{ __gap_surface_family_name_idents!() }};
        (@apply_families) => {{
            gap_surface_family_name_list! {{
                @emit_families;
                gap_surface_family_name_list!(@names)
            }}
        }};
        (@emit_families; $($Name:ident),* $(,)?) => {{
            gap_surface_families! {{ $($Name),* }}
        }};
        (@register $builder:expr) => {{
            gap_surface_family_name_list! {{
                @do_register_bridge $builder;
                gap_surface_family_name_list!(@names)
            }}
        }};
        (@do_register_bridge $builder:expr; $($Name:ident),* $(,)?) => {{
            register_gap_surface_family_connections!($builder, $($Name),*)
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {{
        (@names) => {{ __gap_surface_existing_relay_name_idents!() }};
        (@apply_relays) => {{
            gap_surface_existing_relay_name_list! {{
                @emit_relays;
                gap_surface_existing_relay_name_list!(@names)
            }}
        }};
        (@emit_relays; $($Name:ident),* $(,)?) => {{
            gap_surface_existing_relays! {{ $($Name),* }}
        }};
        (@register $builder:expr) => {{
            gap_surface_existing_relay_name_list! {{
                @do_register_bridge $builder;
                gap_surface_existing_relay_name_list!(@names)
            }}
        }};
        (@do_register_bridge $builder:expr; $($Name:ident),* $(,)?) => {{
            register_gap_surface_existing_relay_connections!($builder, $($Name),*)
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
        ($builder:expr, $($Name:ident),+ $(,)?) => {{{{
            let mut b = $builder;
            $( b = b.register_output_type::<::paste::paste! {{ $crate::schema_gap_surfaces::[<$Name Connection>] }}>(); )+
            b
        }}}};
    }}

    with_gap_surface_family_names!(gap_surface_families);

"""

# Keep gap_surface_family_named! block from original file
named_start = next(
    i
    for i, ln in enumerate(lines)
    if i > end and 'gap_surface_family_named!(' in ln and "ChangedDescriptionInput" in ln
)
relay_macros_start = next(
    i
    for i, ln in enumerate(lines)
    if i > named_start and "macro_rules! with_gap_surface_existing_relay_names" in ln
)

named_block = lines[named_start:relay_macros_start]

relay_tail = """    #[macro_export]
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
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<::paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )+
            b
        }};
    }

    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);

"""

new_lines = lines[:start] + [replacement] + named_block + [relay_tail] + lines[end:]
p.write_text("".join(new_lines), encoding="utf-8")
print(f"fixed gap macros: {len(family_idents)} families, {len(relay_idents)} relays, lines {start}-{end}")
