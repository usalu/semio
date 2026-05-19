"""Patch schema_gap_surfaces macros for compile-safe single name list."""
from pathlib import Path
import re

path = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
text = path.read_text(encoding="utf-8")


def names_block(macro: str, last: str) -> str:
    start = text.index(f"macro_rules! {macro}")
    ns = text.index("(@names) => {", start)
    ne = text.index(last, ns)
    ne = text.index("\n", ne)
    lines = []
    for ln in text[ns:ne].splitlines()[1:]:
        s = ln.strip().rstrip(",")
        if s and s[0].isupper():
            lines.append(ln.rstrip())
    return "\n".join(lines)


fam = names_block("gap_surface_family_name_list", "UpdatedTypeIconInput")
relay = names_block("gap_surface_existing_relay_name_list", "WebsocketBackboneCommand")


def render_name_list(macro: str, names: str, apply_tag: str, apply_body: str, register_macro: str) -> str:
    return f"""    #[macro_export]
    macro_rules! {macro} {{
        (@names) => {{
{names}
        }};
        ({apply_tag}) => {{
{apply_body}
        }};
        (@register $builder:expr) => {{
            $crate::{register_macro}! {{
                @expand $builder;
{names}
            }}
        }};
    }}

"""


fam_block = render_name_list(
    "gap_surface_family_name_list",
    fam,
    "@apply_families",
    f"""            gap_surface_families! {{
{fam}
            }};""",
    "register_gap_surface_family_connections",
)
relay_block = render_name_list(
    "gap_surface_existing_relay_name_list",
    relay,
    "@apply_relays",
    f"""            gap_surface_existing_relays! {{
{relay}
            }};""",
    "register_gap_surface_existing_relay_connections",
)


def replace_macro(macro: str, next_anchor: str, block: str) -> None:
    global text
    start = text.index(f"macro_rules! {macro}")
    end = text.index(f"macro_rules! {next_anchor}", start)
    text = text[:start] + block + text[end:]


replace_macro("gap_surface_family_name_list", "gap_surface_existing_relay_name_list", fam_block)
replace_macro("gap_surface_existing_relay_name_list", "with_gap_surface_family_names", relay_block)

if "    #[macro_export]\n    macro_rules! gap_surface_families" not in text:
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
        1,
    )

with_fam_pat = re.compile(
    r"    #\[macro_export\]\r?\n    macro_rules! with_gap_surface_family_names \{.*?\r?\n    with_gap_surface_family_names!\(gap_surface_families\);\r?\n",
    re.S,
)
with_fam_new = """    #[macro_export]
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

"""
m = with_fam_pat.search(text)
if not m:
    raise SystemExit("with_gap_surface_family_names block not found")
text = text[: m.start()] + with_fam_new + text[m.end() :]

with_relay_pat = re.compile(
    r"    #\[macro_export\]\r?\n    macro_rules! with_gap_surface_existing_relay_names \{.*?\r?\n    with_gap_surface_existing_relay_names!\(gap_surface_existing_relays\);\r?\n",
    re.S,
)
with_relay_new = """    #[macro_export]
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

"""
m = with_relay_pat.search(text)
if not m:
    raise SystemExit("with_gap_surface_existing_relay_names block not found")
text = text[: m.start()] + with_relay_new + text[m.end() :]

path.write_text(text, encoding="utf-8")
print("patched", len(fam.splitlines()), "families", len(relay.splitlines()), "relays")
