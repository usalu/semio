"""Rebuild schema_gap_surfaces name-list macros from __*_idents helpers (run from repo root)."""
import re
from pathlib import Path

p = Path("semio/client/lib/rs/lib.rs")
text = p.read_text(encoding="utf-8")


def extract_idents(macro_name: str) -> str:
    pat = rf"macro_rules! {macro_name} \{{\s*\(\) => \{{\s*(.*?)\s*\}};"
    m = re.search(pat, text, re.DOTALL)
    if not m:
        raise SystemExit(f"missing {macro_name}")
    return m.group(1).strip()


family_names = extract_idents("__gap_surface_family_name_idents")
relay_names = extract_idents("__gap_surface_existing_relay_name_idents")

block = f"""
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
            register_gap_surface_family_connections!(@expand $builder;
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
            register_gap_surface_existing_relay_connections!(@expand $builder;
        {relay_names}
            )
        }};
    }}

"""

start = text.index("    #[macro_export]\n    macro_rules! gap_surface_family_name_list {")
end = text.index("    macro_rules! with_gap_surface_family_names {")
text = text[:start] + block + text[end:]
p.write_text(text, encoding="utf-8")
print("ok")
