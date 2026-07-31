"""Add @apply_families / @apply_relays arms and fix with_gap + paste paths."""
from pathlib import Path

path = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
text = path.read_text(encoding="utf-8")

def extract_names(macro: str, last: str) -> str:
    start = text.index(f"macro_rules! {macro}")
    ns = text.index("(@names) => {", start)
    ne = text.index(last, ns)
    ne = text.index("\n", ne)
    return text[ns : ne + 1]

fam_body = extract_names("gap_surface_family_name_list", "UpdatedTypeIconInput")
relay_body = extract_names("gap_surface_existing_relay_name_list", "WebsocketBackboneCommand")

# strip (@names) => { header from fam_body for inline use
def strip_header(body: str) -> str:
    lines = body.splitlines()
    out = []
    for ln in lines[1:]:
        if ln.strip().startswith("Added") or ln.strip().startswith("Updated") or ln.strip().startswith("Removed") or ln.strip()[0].isupper():
            out.append(ln)
    return "\n".join(out)

fam_names = strip_header(fam_body)
relay_names = strip_header(relay_body)

apply_fam = f"""
        (@apply_families) => {{
            gap_surface_families! {{
{fam_names}
            }}
        }};"""

apply_relay = f"""
        (@apply_relays) => {{
            gap_surface_existing_relays! {{
{relay_names}
            }}
        }};"""

if "(@apply_families)" not in text:
    text = text.replace(
        "        (@register $builder:expr) => {",
        apply_fam + "\n        (@register $builder:expr) => {",
        1,
    )
if "(@apply_relays)" not in text:
    idx = text.index("macro_rules! gap_surface_existing_relay_name_list")
    text = text.replace(
        "        (@register $builder:expr) => {",
        apply_relay + "\n        (@register $builder:expr) => {",
        1,
    )

text = text.replace(
    """        (gap_surface_families) => {
            $crate::schema_gap_surfaces::gap_surface_families! {
                $crate::gap_surface_family_name_list!(@names)
            }
        };""",
    """        (gap_surface_families) => {
            gap_surface_family_name_list!(@apply_families)
        };""",
)
text = text.replace(
    """        (gap_surface_existing_relays) => {
            $crate::schema_gap_surfaces::gap_surface_existing_relays! {
                $crate::gap_surface_existing_relay_name_list!(@names)
            }
        };""",
    """        (gap_surface_existing_relays) => {
            gap_surface_existing_relay_name_list!(@apply_relays)
        };""",
)

text = text.replace(
    "<$crate::schema_gap_surfaces::paste::paste! { [<$Name Connection>] }>()",
    "<paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>()",
)

path.write_text(text, encoding="utf-8")
print("ok")
