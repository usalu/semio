from pathlib import Path
import re

p = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
t = p.read_text(encoding="utf-8")

def names_after(marker: str) -> str:
    i = t.index(marker)
    j = t.index("UpdatedTypeIconInput", i)
    k = t.index("};", j)
    block = t[i:k]
    m = re.search(r"@names\) => \{\s*(.*)", block, re.S)
    if m:
        return m.group(1).strip() + "\n        UpdatedTypeIconInput"
    raise SystemExit(f"names missing for {marker}")

def relay_names_after(marker: str) -> str:
    i = t.index(marker)
    j = t.index("WebsocketBackboneCommand", i)
    k = t.index("};", j)
    block = t[i:k]
    m = re.search(r"@names\) => \{\s*(.*)", block, re.S)
    if m:
        return m.group(1).strip() + "\n        WebsocketBackboneCommand"
    raise SystemExit(f"relay names missing for {marker}")

family = names_after("macro_rules! gap_surface_family_name_list")
relay = relay_names_after("macro_rules! gap_surface_existing_relay_name_list")

family_tail = f"""        () => {{
            $crate::gap_surface_family_name_list!(@apply_families);
        }};
        (@apply_families) => {{
            gap_surface_families! {{
        {family}
            }}
        }};
        (@register $builder:expr) => {{
            $crate::gap_surface_family_name_list! {{
                @do_register_bridge $builder;
        {family}
            }}
        }};
        (@do_register_bridge $builder:expr; $($Name:ident),* $(,)?) => {{
            $crate::register_gap_surface_family_connections!(@do_register $builder, $($Name),*)
        }};"""

relay_tail = f"""        () => {{
            $crate::gap_surface_existing_relay_name_list!(@apply_relays);
        }};
        (@apply_relays) => {{
            gap_surface_existing_relays! {{
        {relay}
            }}
        }};
        (@register $builder:expr) => {{
            $crate::gap_surface_existing_relay_name_list! {{
                @do_register_bridge $builder;
        {relay}
            }}
        }};
        (@do_register_bridge $builder:expr; $($Name:ident),* $(,)?) => {{
            $crate::register_gap_surface_existing_relay_connections!(@do_register $builder, $($Name),*)
        }};"""

t = re.sub(
    r"(macro_rules! gap_surface_family_name_list \{[\s\S]*?UpdatedTypeIconInput\s*\};)\s*[\s\S]*?(?=\n    #\[macro_export\]\n    macro_rules! gap_surface_existing_relay_name_list)",
    r"\1\n" + family_tail,
    t,
    count=1,
)

t = re.sub(
    r"(macro_rules! gap_surface_existing_relay_name_list \{[\s\S]*?WebsocketBackboneCommand\s*\};)\s*[\s\S]*?(?=\n    #\[macro_export\]\n    macro_rules! with_gap_surface_family_names)",
    r"\1\n" + relay_tail,
    t,
    count=1,
)

t = t.replace(
    "        (@do_register $builder:expr, $($Name:ident),* $(,)?) => {\n            let mut b = $builder;",
    "        (@do_register $builder:expr, $($Name:ident),* $(,)?) => {{\n            let mut b = $builder;",
)
t = t.replace(
    "            b\n        };\n    }\n\n    with_gap_surface_family_names!",
    "            b\n        }};\n    }\n\n    with_gap_surface_family_names!",
)
t = t.replace(
    "            b\n        };\n    }\n\n    with_gap_surface_existing_relay_names!",
    "            b\n        }};\n    }\n\n    with_gap_surface_existing_relay_names!",
)

if "macro_rules! with_gap_surface_family_names" in t and "#[macro_export]\n    macro_rules! with_gap_surface_family_names" not in t:
    t = t.replace(
        "    macro_rules! with_gap_surface_family_names {",
        "    #[macro_export]\n    macro_rules! with_gap_surface_family_names {",
        1,
    )

p.write_text(t, encoding="utf-8")
print("fixed arms")
