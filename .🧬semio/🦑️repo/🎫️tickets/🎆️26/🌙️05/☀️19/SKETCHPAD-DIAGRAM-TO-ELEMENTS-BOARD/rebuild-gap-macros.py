import re
from pathlib import Path

p = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
text = p.read_text(encoding="utf-8")

start = text.index("    macro_rules! gap_surface_families_from_names {")
named_start = text.index('    gap_surface_family_named!(\n        "ChangedDescriptionInput",')
relay_start = text.index("    macro_rules! gap_surface_existing_relay_name_list {")
mod_close = text.index("\n}\n\n//#endregion 🩹️ schema_gap_surfaces")

block = text[start:mod_close]
fm = re.search(
    r"AddedAttributeToConcept,.*?UpdatedTypeIconInput\)?",
    block,
    re.S,
)
rm = re.search(
    r"AddedAttributeToConceptInput,.*?WebsocketBackboneCommand\)?",
    block,
    re.S,
)
if not fm or not rm:
    raise SystemExit("could not extract name lists")

family_names = fm.group(0).rstrip(")")
relay_names = rm.group(0).rstrip(")")

replacement = f"""    macro_rules! gap_surface_families {{
        () => {{
            gap_surface_families!(gap_surface_family_name_list!());
        }};
        ($($Name:ident),+ $(,)?) => {{
            $(gap_surface_family!($Name);)+
        }};
    }}

    macro_rules! gap_surface_existing_relays {{
        () => {{
            gap_surface_existing_relays!(gap_surface_existing_relay_name_list!());
        }};
        ($($Name:ident),+ $(,)?) => {{
            $(gap_surface_existing_relay!($Name);)+
        }};
    }}

    macro_rules! gap_surface_family_name_list {{
        () => {{
        {family_names}
        }};
    }}

    macro_rules! gap_surface_existing_relay_name_list {{
        () => {{
        {relay_names}
        }};
    }}

    #[macro_export]
    macro_rules! with_gap_surface_family_names {{
        (gap_surface_families) => {{
            gap_surface_families!();
        }};
        (register_gap_surface_family_connections, $builder:expr) => {{
            register_gap_surface_family_connections!($builder, gap_surface_family_name_list!())
        }};
    }}

    #[macro_export]
    macro_rules! register_gap_surface_family_connections {{
        ($builder:expr, $($Name:ident),+ $(,)?) => {{{{
            let mut b = $builder;
            $( b = b.register_output_type::<paste::paste! {{ [<$Name Connection>] }}>(); )+
            b
        }}}};
    }}

    gap_surface_families!();

"""

relay_tail = """    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {
        (gap_surface_existing_relays) => {
            gap_surface_existing_relays!();
        };
        (register_gap_surface_existing_relay_connections, $builder:expr) => {
            register_gap_surface_existing_relay_connections!($builder, gap_surface_existing_relay_name_list!())
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_existing_relay_connections {
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<paste::paste! { [<$Name Connection>] }>(); )+
            b
        }};
    }

    gap_surface_existing_relays!();
}

"""

named_section = text[named_start:relay_start]
new_text = text[:start] + replacement + named_section + relay_tail + text[mod_close + 1 :]
p.write_text(new_text, encoding="utf-8")
print("rebuilt", len(text), "->", len(new_text))
