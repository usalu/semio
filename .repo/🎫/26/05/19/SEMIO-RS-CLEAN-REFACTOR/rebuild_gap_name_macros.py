"""Rebuild gap_surface_*_name_list macros with inlined @register."""
from pathlib import Path

path = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
lines = path.read_text(encoding="utf-8").splitlines(keepends=True)

def find_line(substr: str, from_i: int = 0) -> int:
    for i in range(from_i, len(lines)):
        if substr in lines[i]:
            return i
    raise SystemExit(f"missing {substr!r}")

fam_macro = find_line("macro_rules! gap_surface_family_name_list")
with_fam = find_line("macro_rules! with_gap_surface_family_names", fam_macro)

fam_end = find_line("UpdatedTypeIconInput", fam_macro)
fam_names = [lines[i].rstrip("\n") for i in range(fam_macro + 2, fam_end + 1)]

relay_macro = find_line("macro_rules! gap_surface_existing_relay_name_list", with_fam)
relay_end = find_line("WebsocketBackboneCommand", relay_macro)
relay_names = [lines[i].rstrip("\n") for i in range(relay_macro + 2, relay_end + 1)]

def render(name: str, names: list[str], register: str) -> str:
    body = "\n".join(names)
    return f"""    #[macro_export]
    macro_rules! {name} {{
        (@names) => {{
{body}
        }};
        {{}} => {{
            {name}!(@names);
        }};
        (@register $builder:expr) => {{
            $crate::{register}! {{
                @expand $builder;
{body}
            }}
        }};
    }}

"""

block = render("gap_surface_family_name_list", fam_names, "register_gap_surface_family_connections")
block += render("gap_surface_existing_relay_name_list", relay_names, "register_gap_surface_existing_relay_connections")

# patch gap_surface_families macros to macro_export
text = "".join(lines)
text = text.replace(
    "    macro_rules! gap_surface_families {",
    "    #[macro_export]\n    macro_rules! gap_surface_families {",
    1,
)
text = text.replace(
    "    macro_rules! gap_surface_existing_relays {",
    "    #[macro_export]\n    macro_rules! gap_surface_existing_relays {",
    1,
)
text = text.replace("    #[macro_export]\n    #[macro_export]\n", "    #[macro_export]\n", 1)

lines = text.splitlines(keepends=True)
fam_macro = find_line("macro_rules! gap_surface_family_name_list")
with_fam = find_line("macro_rules! with_gap_surface_family_names", fam_macro)
new_lines = lines[:fam_macro] + [block] + lines[with_fam:]

text = "".join(new_lines)
text = text.replace(
    """        (gap_surface_families) => {
            $crate::schema_gap_surfaces::gap_surface_families! {
                $crate::gap_surface_family_name_list!(@names)
            }
        };""",
    """        (gap_surface_families) => {
            $crate::gap_surface_families! {
                $crate::gap_surface_family_name_list!(@names)
            }
        };""",
)
text = text.replace(
    """        (gap_surface_existing_relays) => {
            $crate::schema_gap_surfaces::gap_surface_existing_relays! {
                $crate::gap_surface_existing_relay_name_list!(@names)
            }
        };""",
    """        (gap_surface_existing_relays) => {
            $crate::gap_surface_existing_relays! {
                $crate::gap_surface_existing_relay_name_list!(@names)
            }
        };""",
)
text = text.replace(
    "$crate::schema_gap_surfaces::paste::paste!",
    "paste::paste! { $crate::schema_gap_surfaces::",
)
# botched paste replace - do properly
text = text.replace(
    "paste::paste! { $crate::schema_gap_surfaces::paste! { [<$Name Connection>] }",
    "paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>]",
)
text = text.replace(
    "<paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>()",
    "<paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>()",
)

path.write_text(text, encoding="utf-8")
print("ok", len(fam_names), len(relay_names))
