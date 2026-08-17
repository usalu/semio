import re
from pathlib import Path

p = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
text = p.read_text(encoding="utf-8")

fm = re.search(
    r"macro_rules! gap_surface_family_name_list \{\s*\(\) => \{\s*(AddedAttributeToConcept,.*?UpdatedTypeIconInput)\s*\};",
    text,
    re.S,
)
rm = re.search(
    r"macro_rules! gap_surface_existing_relay_name_list \{\s*\(\) => \{\s*(AddedAttributeToConceptInput,.*?WebsocketBackboneCommand)\s*\};",
    text,
    re.S,
)
if not fm or not rm:
    raise SystemExit("name lists not found")

family_names = fm.group(1)
relay_names = rm.group(1)

with_family = f"""    #[macro_export]
    macro_rules! with_gap_surface_family_names {{
        (gap_surface_families) => {{
            gap_surface_families! {{
                {family_names}
            }};
        }};
        (register_gap_surface_family_connections, $builder:expr) => {{
            register_gap_surface_family_connections!($builder, gap_surface_family_name_list!())
        }};
    }}

"""

with_relay = f"""    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {{
        (gap_surface_existing_relays) => {{
            gap_surface_existing_relays! {{
                {relay_names}
            }};
        }};
        (register_gap_surface_existing_relay_connections, $builder:expr) => {{
            register_gap_surface_existing_relay_connections!($builder, gap_surface_existing_relay_name_list!())
        }};
    }}

"""

wf_start = text.index("    #[macro_export]\n    macro_rules! with_gap_surface_family_names {")
wf_end = text.index("    #[macro_export]\n    macro_rules! register_gap_surface_family_connections {")
wr_start = text.index("    #[macro_export]\n    macro_rules! with_gap_surface_existing_relay_names {")
wr_end = text.index("    #[macro_export]\n    macro_rules! register_gap_surface_existing_relay_connections {")

text = text[:wf_start] + with_family + text[wf_end:wr_start] + with_relay + text[wr_end:]

# simplify gap_surface_families / relays - remove broken () arms
text = text.replace(
    """    macro_rules! gap_surface_families {
        () => {
            gap_surface_families! { gap_surface_family_name_list!() };
        };
        { $($Name:ident),+ $(,)? } => {
            $(gap_surface_family!($Name);)+
        };
    }

    macro_rules! gap_surface_existing_relays {
        () => {
            gap_surface_existing_relays! { gap_surface_existing_relay_name_list!() };
        };
        { $($Name:ident),+ $(,)? } => {
            $(gap_surface_existing_relay!($Name);)+
        };
    }
""",
    """    macro_rules! gap_surface_families {
        { $($Name:ident),* $(,)? } => {
            $(gap_surface_family!($Name);)+
        };
    }

    macro_rules! gap_surface_existing_relays {
        { $($Name:ident),* $(,)? } => {
            $(gap_surface_existing_relay!($Name);)+
        };
    }
""",
)

p.write_text(text, encoding="utf-8")
print("fixed with macros")
