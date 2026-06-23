"""Final fix for gap surface macros on nearly-clean lib.rs."""
from pathlib import Path

path = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
text = path.read_text(encoding="utf-8")

def names_block(macro: str, last: str) -> str:
    start = text.index(f"macro_rules! {macro}")
    ns = text.index("(@names) => {", start)
    ne = text.index(last, ns)
    ne = text.index("\n", ne)
    lines = []
    for ln in text[ns:ne].splitlines()[1:]:
        s = ln.strip()
        if s and s[0].isupper():
            lines.append(ln)
    return "\n".join(lines)

fam = names_block("gap_surface_family_name_list", "UpdatedTypeIconInput")
relay = names_block("gap_surface_existing_relay_name_list", "WebsocketBackboneCommand")

apply_fam = f"""
        (@apply_families) => {{
            gap_surface_families! {{
{fam}
            }};
        }}"""

apply_relay = f"""
        (@apply_relays) => {{
            gap_surface_existing_relays! {{
{relay}
            }};
        }}"""

register_fam = f"""
        (@register $builder:expr) => {{
            $crate::register_gap_surface_family_connections! {{
                @expand $builder;
{fam}
            }}
        }}"""

register_relay = f"""
        (@register $builder:expr) => {{
            $crate::register_gap_surface_existing_relay_connections! {{
                @expand $builder;
{relay}
            }}
        }}"""

import re

def patch_name_list(macro: str, apply: str, register: str) -> None:
    global text
    pat = rf"(    \[#\[macro_export\]\]\s+macro_rules! {macro} \{{.*?)(\n    \[#\[macro_export\]\]\s+macro_rules! with_gap_surface)"
    m = re.search(pat, text, re.S)
    if not m:
        # try without next macro anchor
        start = text.index(f"macro_rules! {macro}")
        end = text.index("\n    #[macro_export]", start + 1)
        block = text[start:end]
    else:
        block = m.group(1)
    # rebuild minimal
    names = fam if "family" in macro else relay
    new_block = f"""    #[macro_export]
    macro_rules! {macro} {{
        (@names) => {{
{names}
        }};
        {{}} => {{
            {macro}!(@names);
        }};{apply}{register}
    }}
"""
    if m:
        text = text[: m.start(1)] + new_block + text[m.start(2) :]
    else:
        text = text[:start] + new_block.split("macro_rules!", 1)[1] + text[end:]

# simpler: replace from macro start to with_gap
for macro, apply, reg, next_m in [
    ("gap_surface_family_name_list", apply_fam, register_fam, "with_gap_surface_family_names"),
    ("gap_surface_existing_relay_name_list", apply_relay, register_relay, "with_gap_surface_existing_relay_names"),
]:
    start = text.index(f"macro_rules! {macro}")
    end = text.index(f"macro_rules! {next_m}", start)
    names = fam if "family" in macro else relay
    new_block = f"""    #[macro_export]
    macro_rules! {macro} {{
        (@names) => {{
{names}
        }};
        {{}} => {{
            {macro}!(@names);
        }};{apply}{reg}
    }}

"""
    text = text[:start] + new_block + text[end:]

text = text.replace(
    """    macro_rules! gap_surface_families {
        { $($Name:ident),* $(,)? } => {
            $(gap_surface_family!($Name);)+
        };
    }

    macro_rules! gap_surface_existing_relays {
        { $($Name:ident),* $(,)? } => {
            $(gap_surface_existing_relay!($Name);)+
        };
    }""",
    """    #[macro_export]
    macro_rules! gap_surface_families {
        { $($Name:ident),* $(,)? } => {
            $(gap_surface_family!($Name);)+
        };
    }

    #[macro_export]
    macro_rules! gap_surface_existing_relays {
        { $($Name:ident),* $(,)? } => {
            $(gap_surface_existing_relay!($Name);)+
        };
    }""",
)

text = text.replace(
    """    #[macro_export]
    macro_rules! with_gap_surface_family_names {
        (gap_surface_families) => {
            $crate::schema_gap_surfaces::gap_surface_families! {
                $crate::gap_surface_family_name_list!(@names)
            }
        };
        (register_gap_surface_family_connections, $builder:expr) => {
            $crate::gap_surface_family_name_list!(@register $builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_family_connections {
        (@do_register $builder:expr, $($Name:ident),* $(,)?) => {
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! { [<$Name Connection>] }>(); )*
            b
        };
        ($builder:expr, $($Name:ident),+ $(,)?) => {
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! { [<$Name Connection>] }>(); )+
            b
        };
    }

    with_gap_surface_family_names!(gap_surface_families);
""",
    """    #[macro_export]
    macro_rules! with_gap_surface_family_names {
        (register_gap_surface_family_connections, $builder:expr) => {
            $crate::gap_surface_family_name_list!(@register $builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_family_connections {
        (@expand $builder:expr; $($Name:ident),* $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )*
            b
        }};
    }

    gap_surface_family_name_list!(@apply_families);
""",
)

text = text.replace(
    """    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {
        (gap_surface_existing_relays) => {
            $crate::schema_gap_surfaces::gap_surface_existing_relays! {
                $crate::gap_surface_existing_relay_name_list!(@names)
            }
        };
        (register_gap_surface_existing_relay_connections, $builder:expr) => {
            $crate::gap_surface_existing_relay_name_list!(@register $builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_existing_relay_connections {
        (@do_register $builder:expr, $($Name:ident),* $(,)?) => {
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! { [<$Name Connection>] }>(); )*
            b
        };
        ($builder:expr, $($Name:ident),+ $(,)?) => {
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! { [<$Name Connection>] }>(); )+
            b
        };
    }

    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);
""",
    """    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {
        (register_gap_surface_existing_relay_connections, $builder:expr) => {
            $crate::gap_surface_existing_relay_name_list!(@register $builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_existing_relay_connections {
        (@expand $builder:expr; $($Name:ident),* $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )*
            b
        }};
    }

    gap_surface_existing_relay_name_list!(@apply_relays);
""",
)

path.write_text(text, encoding="utf-8")
print("done")
