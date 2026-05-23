import re
from pathlib import Path

p = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
text = p.read_text(encoding="utf-8")

start = text.index("    #[macro_export]\n    macro_rules! gap_surface_family_name_list {")
end = text.index("    #[macro_export]\n    macro_rules! with_gap_surface_family_names {")

block = text[start:end]
fm = re.search(
    r"@names\) => \{\s*(AddedAttributeToConcept,.*?UpdatedTypeIconInput)\s*\};",
    block,
    re.S,
)
rm = re.search(
    r"macro_rules! gap_surface_existing_relay_name_list \{.*?@names\) => \{\s*(AddedAttributeToConceptInput,.*?WebsocketBackboneCommand)\s*\};",
    block,
    re.S,
)
if not fm or not rm:
    raise SystemExit("could not extract name lists from corrupted block")

family_names = fm.group(1).strip()
relay_names = rm.group(1).strip()

replacement = f"""    #[macro_export]
    macro_rules! gap_surface_family_name_list {{
        (@names) => {{
        {family_names}
        }};
        (@register $builder:expr) => {{
            $crate::register_gap_surface_family_connections!($builder,
        {family_names}
            );
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {{
        (@names) => {{
        {relay_names}
        }};
        (@register $builder:expr) => {{
            $crate::register_gap_surface_existing_relay_connections!($builder,
        {relay_names}
            );
        }};
    }}

"""

text = text[:start] + replacement + text[end:]
p.write_text(text, encoding="utf-8")
print("rebuilt name_list macros", len(replacement), "bytes")
