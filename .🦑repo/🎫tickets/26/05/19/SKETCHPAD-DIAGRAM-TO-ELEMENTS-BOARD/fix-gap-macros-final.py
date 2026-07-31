from pathlib import Path
import re

p = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
text = p.read_text(encoding="utf-8")

text = text.replace("UpdatedTypeIconInput)", "UpdatedTypeIconInput")

start = text.index("    #[macro_export]\n    macro_rules! gap_surface_family_name_list {")
relay_bad = text.index("    #[macro_export]\n    macro_rules! gap_surface_existing_relay_name_list {")
named = text.index('    gap_surface_family_named!(\n        "ChangedDescriptionInput",')

fm = re.search(
    r"\{\s*\n\s*AddedAttributeToConcept,.*?UpdatedTypeIconInput\s*\n\s*\};",
    text[start:relay_bad],
    re.S,
)
if not fm:
    raise SystemExit("family list not found")
family_block = fm.group(0)

relay_section = text[relay_bad:named]
rm = re.search(
    r"AddedAttributeToConceptInput,.*?WebsocketBackboneCommand\)?",
    relay_section,
    re.S,
)
if not rm:
    raise SystemExit("relay list not found")
relay_names = rm.group(0).rstrip(")")

core = f"""    macro_rules! gap_surface_families {{
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
        {family_block}
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {{
        {{}} => {{
        {relay_names}
        }};
    }}

    #[macro_export]
    macro_rules! with_gap_surface_family_names {{
        (gap_surface_families) => {{
            $crate::schema_gap_surfaces::gap_surface_families! {{
                $crate::gap_surface_family_name_list! {{}}
            }}
        }};
        (register_gap_surface_family_connections, $builder:expr) => {{
            $crate::register_gap_surface_family_connections!(
                $builder,
                $crate::gap_surface_family_name_list! {{}}
            )
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

    with_gap_surface_family_names!(gap_surface_families);

"""

relay_tail = """    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {
        (gap_surface_existing_relays) => {
            $crate::schema_gap_surfaces::gap_surface_existing_relays! {
                $crate::gap_surface_existing_relay_name_list! {}
            }
        };
        (register_gap_surface_existing_relay_connections, $builder:expr) => {
            $crate::register_gap_surface_existing_relay_connections!(
                $builder,
                $crate::gap_surface_existing_relay_name_list! {}
            )
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

    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);
}

"""

mod_start = text.index("    macro_rules! gap_surface_families {")
mod_end = text.index("\n}\n\n//#endregion 🩹 schema_gap_surfaces")
named_section = text[named:mod_end]

named_section = re.sub(
    r"\s*#\[macro_export\].*?with_gap_surface_existing_relay_names.*?with_gap_surface_existing_relay_names!\(gap_surface_existing_relays\);\s*",
    "\n",
    named_section,
    flags=re.S,
)
named_section = re.sub(
    r"\s*#\[macro_export\].*?register_gap_surface_existing_relay_connections.*?\n\s*\}\s*;\s*\n",
    "\n",
    named_section,
    flags=re.S,
)
named_section = re.sub(
    r"\s*with_gap_surface_family_names!\(gap_surface_families\);\s*",
    "\n",
    named_section,
)
named_section = re.sub(
    r"\s*macro_rules! gap_surface_families_brace_wrap.*?gap_surface_existing_relays_brace_wrap.*?\}\s*;\s*\n",
    "\n",
    named_section,
    flags=re.S,
)
named_section = re.sub(
    r"\s*#\[macro_export\].*?register_gap_surface_family_connections_expand.*?\n\s*\}\s*;\s*\n",
    "\n",
    named_section,
    flags=re.S,
)
named_section = re.sub(
    r"\s*#\[macro_export\].*?with_gap_surface_family_names.*?\n\s*\}\s*;\s*\n",
    "\n",
    named_section,
    flags=re.S,
)
named_section = re.sub(
    r"\s*#\[macro_export\].*?register_gap_surface_family_connections.*?\n\s*\}\s*;\s*\n",
    "\n",
    named_section,
    flags=re.S,
)

text = text[:mod_start] + core + named_section + relay_tail + text[mod_end + 1 :]
p.write_text(text, encoding="utf-8")
print("done")
