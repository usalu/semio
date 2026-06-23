"""Restore gap_surface name idents and splice-based list macros."""
from pathlib import Path
import re

p = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
text = p.read_text(encoding="utf-8")

def body_of(macro_name: str, source: str) -> str:
    m = re.search(
        rf"macro_rules!\s+{re.escape(macro_name)}\s*\{{\s*\(\)\s*=>\s*\{{([^}}]+)\}};",
        source,
        re.S,
    )
    if not m:
        raise SystemExit(f"missing {macro_name}")
    return m.group(1).strip()


family = body_of("__gap_surface_family_name_idents_legacy", text)
relay = body_of("__gap_surface_existing_relay_name_idents", text)

i0 = text.index("    macro_rules! __gap_surface_family_name_idents {")
i1 = text.index("    #[macro_export]\n    macro_rules! with_gap_surface_family_names")
if "with_gap_surface_family_names" not in text[i0:i1]:
    i1 = text.index("    macro_rules! with_gap_surface_family_names", i0)

replacement = f'''    macro_rules! __gap_surface_family_name_idents {{
        () => {{
        {family}
        }};
    }}

    macro_rules! __gap_surface_existing_relay_name_idents {{
        () => {{
        {relay}
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_family_name_list {{
        (@names) => {{ __gap_surface_family_name_idents!() }};
        (@apply_families) => {{
            gap_surface_family_name_list! {{
                @emit_families;
                gap_surface_family_name_list!(@names)
            }}
        }};
        (@emit_families; $($Name:ident),* $(,)?) => {{
            gap_surface_families! {{ $($Name),* }}
        }};
        (@register $builder:expr) => {{
            gap_surface_family_name_list! {{
                @do_register_bridge $builder,
                gap_surface_family_name_list!(@names)
            }}
        }};
        (@do_register_bridge $builder:expr, $($Name:ident),* $(,)?) => {{
            register_gap_surface_family_connections!($builder, $($Name),*)
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {{
        (@names) => {{ __gap_surface_existing_relay_name_idents!() }};
        (@apply_relays) => {{
            gap_surface_existing_relay_name_list! {{
                @emit_relays;
                gap_surface_existing_relay_name_list!(@names)
            }}
        }};
        (@emit_relays; $($Name:ident),* $(,)?) => {{
            gap_surface_existing_relays! {{ $($Name),* }}
        }};
        (@register $builder:expr) => {{
            gap_surface_existing_relay_name_list! {{
                @do_register_bridge $builder,
                gap_surface_existing_relay_name_list!(@names)
            }}
        }};
        (@do_register_bridge $builder:expr, $($Name:ident),* $(,)?) => {{
            register_gap_surface_existing_relay_connections!($builder, $($Name),*)
        }};
    }}

'''

text = text[:i0] + replacement + text[i1:]
text = re.sub(
    r"\n    macro_rules! __gap_surface_family_name_idents_legacy \{.*?\n    \}\n",
    "\n",
    text,
    count=1,
    flags=re.S,
)
p.write_text(text, encoding="utf-8")
print("fixed", len(family.splitlines()), "family names")
