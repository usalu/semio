import re
from pathlib import Path

p = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
text = p.read_text(encoding="utf-8")

family_m = re.search(
    r"macro_rules! __gap_surface_family_name_idents \{\s*\(\) => \{\s*(.*?)\s*\};\s*\}",
    text,
    re.DOTALL,
)
relay_m = re.search(
    r"macro_rules! __gap_surface_existing_relay_name_idents \{\s*\(\) => \{\s*(.*?)\s*\};\s*\}",
    text,
    re.DOTALL,
)
if not family_m or not relay_m:
    raise SystemExit("missing __idents blocks")
family_names = family_m.group(1).strip()
relay_names = relay_m.group(1).strip()

family_list = f"""
    #[macro_export]
    macro_rules! gap_surface_family_name_list {{
        (@names) => {{ __gap_surface_family_name_idents!() }};
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
        (@names) => {{ __gap_surface_existing_relay_name_idents!() }};
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

"""

text = re.sub(
    r"    #\[macro_export\]\s*macro_rules! gap_surface_family_name_list \{.*?\n    \}\n\n    #\[macro_export\]\s*macro_rules! gap_surface_existing_relay_name_list \{.*?\n    \}\n\n",
    family_list,
    text,
    count=1,
    flags=re.DOTALL,
)

p.write_text(text, encoding="utf-8")
print("ok")
