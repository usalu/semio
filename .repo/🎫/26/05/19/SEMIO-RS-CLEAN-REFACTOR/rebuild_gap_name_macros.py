"""Rebuild gap_surface_*_name_list macros with inlined @register."""
from pathlib import Path

path = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
lines = path.read_text(encoding="utf-8").splitlines(keepends=True)

def find_line(substr: str, from_i: int = 0) -> int:
    for i in range(from_i, len(lines)):
        if substr in lines[i]:
            return i
    raise SystemExit(f"missing {substr!r} from {from_i}")

fam_macro = find_line("macro_rules! gap_surface_family_name_list")
relay_macro = find_line("macro_rules! gap_surface_existing_relay_name_list", fam_macro + 1)
with_fam = find_line("macro_rules! with_gap_surface_family_names", relay_macro + 1)

def names_between(start: int, last_ident: str) -> list[str]:
    end = find_line(last_ident, start)
    return [lines[i].rstrip("\n") for i in range(start + 2, end + 1)]

fam_names = names_between(fam_macro, "UpdatedTypeIconInput")
relay_names = names_between(relay_macro, "WebsocketBackboneCommand")

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

text = "".join(lines)
for old, new in [
    ("    macro_rules! gap_surface_families {", "    #[macro_export]\n    macro_rules! gap_surface_families {"),
    ("    macro_rules! gap_surface_existing_relays {", "    #[macro_export]\n    macro_rules! gap_surface_existing_relays {"),
]:
    if new not in text:
        text = text.replace(old, new, 1)
text = text.replace("    #[macro_export]\n    #[macro_export]\n", "    #[macro_export]\n")

lines = text.splitlines(keepends=True)
fam_macro = find_line("macro_rules! gap_surface_family_name_list")
with_fam = find_line("macro_rules! with_gap_surface_family_names", fam_macro)
text = "".join(lines[:fam_macro]) + block + "".join(lines[with_fam:])

for a, b in [
    ("$crate::schema_gap_surfaces::gap_surface_families!", "$crate::gap_surface_families!"),
    ("$crate::schema_gap_surfaces::gap_surface_existing_relays!", "$crate::gap_surface_existing_relays!"),
    (
        "<$crate::schema_gap_surfaces::paste::paste! { [<$Name Connection>] }>()",
        "<paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>()",
    ),
]:
    text = text.replace(a, b)

path.write_text(text, encoding="utf-8")
print("ok", len(fam_names), len(relay_names))
