import re
import subprocess
from pathlib import Path

root = Path(r"c:\git\semio")
head = subprocess.check_output(
    ["git", "show", "HEAD:semio/client/lib/rs/lib.rs"],
    cwd=root,
    text=True,
    encoding="utf-8",
)


def extract_mod(text: str, mod_name: str) -> str:
    start = text.index(f"pub mod {mod_name} {{")
    depth = 0
    for idx in range(start, len(text)):
        if text[idx] == "{":
            depth += 1
        elif text[idx] == "}":
            depth -= 1
            if depth == 0:
                return text[start : idx + 1]
    raise RuntimeError(f"unclosed {mod_name}")


head_mod = extract_mod(head, "schema_gap_surfaces")

fm = re.search(
    r"macro_rules! gap_surface_family_name_list \{\s*"
    r"\(@names\) => \{\s*(AddedAttributeToConcept,.*?UpdatedTypeIconInput)\s*\};",
    head_mod,
    re.S,
)
rm = re.search(
    r"macro_rules! gap_surface_existing_relay_name_list \{\s*"
    r"\(@names\) => \{\s*(AddedAttributeToConceptInput,.*?WebsocketBackboneCommand)\s*\};",
    head_mod,
    re.S,
)
if not fm or not rm:
    raise SystemExit("name lists not found in HEAD mod")

family_names = fm.group(1).strip()
relay_names = rm.group(1).strip()

macros = f"""    macro_rules! gap_surface_families {{
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
        (@names) => {{
        {family_names}
        }};
        (@apply_families) => {{
            gap_surface_families! {{
        {family_names}
            }}
        }};
        (@register $builder:expr) => {{
            $crate::register_gap_surface_family_connections!($builder,
        {family_names}
            )
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {{
        (@names) => {{
        {relay_names}
        }};
        (@apply_relays) => {{
            gap_surface_existing_relays! {{
        {relay_names}
            }}
        }};
        (@register $builder:expr) => {{
            $crate::register_gap_surface_existing_relay_connections!($builder,
        {relay_names}
            )
        }};
    }}

    #[macro_export]
    macro_rules! with_gap_surface_family_names {{
        (gap_surface_families) => {{
            $crate::gap_surface_family_name_list!(@apply_families);
        }};
        (register_gap_surface_family_connections, $builder:expr) => {{
            $crate::gap_surface_family_name_list!(@register $builder);
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

relay_tail = f"""    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {{
        (gap_surface_existing_relays) => {{
            $crate::gap_surface_existing_relay_name_list!(@apply_relays);
        }};
        (register_gap_surface_existing_relay_connections, $builder:expr) => {{
            $crate::gap_surface_existing_relay_name_list!(@register $builder);
        }};
    }}

    #[macro_export]
    macro_rules! register_gap_surface_existing_relay_connections {{
        ($builder:expr, $($Name:ident),+ $(,)?) => {{{{
            let mut b = $builder;
            $( b = b.register_output_type::<paste::paste! {{ [<$Name Connection>] }}>(); )+
            b
        }}}};
    }}

    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);

"""

# rebuild head_mod internals
start = head_mod.index("    macro_rules! gap_surface_families {")
end = head_mod.index("    with_gap_surface_existing_relays!();") + len(
    "    with_gap_surface_existing_relays!();\n"
)
new_mod = head_mod[:start] + macros + relay_tail + head_mod[end:]

# replace in current file
cur_path = root / "semio/client/lib/rs/lib.rs"
cur = cur_path.read_text(encoding="utf-8")
region_start = cur.index("//#region 🩹 schema_gap_surfaces")
region_end = cur.index("//#endregion 🩹 schema_gap_surfaces") + len(
    "//#endregion 🩹 schema_gap_surfaces"
)
replacement = f"//#region 🩹 schema_gap_surfaces\n\n{new_mod}\n\n//#endregion 🩹 schema_gap_surfaces"
cur_path.write_text(cur[:region_start] + replacement + cur[region_end:], encoding="utf-8")
print("finalized schema_gap_surfaces", len(new_mod))
