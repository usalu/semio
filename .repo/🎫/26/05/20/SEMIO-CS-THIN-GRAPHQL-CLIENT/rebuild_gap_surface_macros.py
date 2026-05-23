#!/usr/bin/env python3
"""Replace duplicated gap-surface macro block in lib.rs with single-source idents + apply/register."""
from pathlib import Path

p = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
lines = p.read_text(encoding="utf-8").splitlines(keepends=True)

family_start = next(i for i, ln in enumerate(lines) if "macro_rules! gap_surface_family_name_list" in ln)
family_idents: list[str] = []
capture = False
for ln in lines[family_start:]:
    if "(@names) => {" in ln:
        capture = True
        continue
    if capture:
        if ln.strip() == "};":
            break
        s = ln.strip().rstrip(",")
        if s and s.isidentifier():
            family_idents.append(s)

relay_start = next(i for i, ln in enumerate(lines) if "macro_rules! gap_surface_existing_relay_name_list" in ln)
relay_idents: list[str] = []
capture = False
for ln in lines[relay_start:]:
    if "(@names) => {" in ln:
        capture = True
        continue
    if capture:
        if ln.strip() == "};":
            break
        s = ln.strip().rstrip(",")
        if s and s.isidentifier():
            relay_idents.append(s)

block_start = next(
    i
    for i, ln in enumerate(lines)
    if "macro_rules! gap_surface_families" in ln and i < family_start
)
named_start = next(i for i, ln in enumerate(lines) if i > family_start and "gap_surface_family_named!(" in ln)
relay_with_start = next(i for i, ln in enumerate(lines) if "macro_rules! with_gap_surface_existing_relay_names" in ln)
endregion = next(i for i, ln in enumerate(lines) if "//#endregion" in ln and "schema_gap_surfaces" in ln)


def ident_lines(idents: list[str], indent: str) -> str:
    return "".join(f"{indent}{n},\n" for n in idents)


replacement = f"""    macro_rules! __gap_surface_family_name_idents {{
        () => {{
{ident_lines(family_idents, "        ")}        }};
    }}

    macro_rules! __gap_surface_existing_relay_name_idents {{
        () => {{
{ident_lines(relay_idents, "        ")}        }};
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
            gap_surface_families! {{ __gap_surface_family_name_idents!() }}
        }};
        (@register $builder:expr) => {{
            register_gap_surface_family_connections!($builder, __gap_surface_family_name_idents!())
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {{
        (@names) => {{ __gap_surface_existing_relay_name_idents!() }};
        (@apply_relays) => {{
            gap_surface_existing_relays! {{ __gap_surface_existing_relay_name_idents!() }}
        }};
        (@register $builder:expr) => {{
            register_gap_surface_existing_relay_connections!($builder, __gap_surface_existing_relay_name_idents!())
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

tail = """    #[macro_export]
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

new_lines = (
    lines[:block_start]
    + [replacement]
    + lines[named_start:relay_with_start]
    + [tail]
    + lines[endregion + 1 :]
)
p.write_text("".join(new_lines), encoding="utf-8")
print(f"rebuilt: {len(family_idents)} family, {len(relay_idents)} relay idents")
