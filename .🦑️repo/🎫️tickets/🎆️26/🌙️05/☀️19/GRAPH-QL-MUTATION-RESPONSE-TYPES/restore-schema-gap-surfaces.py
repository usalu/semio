"""Restore pub mod schema_gap_surfaces with working inline-list macros."""
import re
from pathlib import Path

lib = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
head = Path(r"c:\git\compose\.repo\🎫️\26\05\19\GRAPH-QL-MUTATION-RESPONSE-TYPES\lib-head.rs")
text = lib.read_text(encoding="utf-8")
head_text = head.read_text(encoding="utf-8")

start_m = re.search(r"//#region .+ schema_gap_surfaces", text)
end_m = re.search(r"//#endregion .+ schema_gap_surfaces", text)
if not start_m:
    raise SystemExit("start marker not found in lib.rs")
start = start_m.start()
if end_m:
    end = end_m.end()
else:
    meta_m = re.search(r"//#region .+ meta", text[start:])
    if not meta_m:
        raise SystemExit("neither endregion nor meta marker found")
    end = start + meta_m.start()

mod_start = head_text.index("pub mod schema_gap_surfaces {")
mod_end = head_text.index("//#endregion", mod_start)
mod_body = head_text[mod_start:mod_end]

fam_m = re.search(
    r"macro_rules! gap_surface_family_name_list \{\s*\(@names\) => \{\s*(.*?)\s*\};\s*\(\)",
    mod_body,
    re.DOTALL,
)
relay_m = re.search(
    r"macro_rules! gap_surface_existing_relay_name_list \{\s*\(@names\) => \{\s*(.*?)\s*\};\s*\(\)",
    mod_body,
    re.DOTALL,
)
if not fam_m or not relay_m:
    raise SystemExit("could not extract name lists from HEAD")
family_names = fam_m.group(1).strip()
relay_names = relay_m.group(1).strip()

# Keep helper macros through gap_surface_existing_relays (drop broken bridge macros from HEAD).
cut_markers = [
    "    macro_rules! define_gap_surface_families_from_list",
    "    #[macro_export]\n    macro_rules! __gap_surface_family_name_idents",
    "    #[macro_export]\n    macro_rules! gap_surface_family_name_list",
]
helpers_end = min(mod_body.index(m) for m in cut_markers if m in mod_body)
helpers = mod_body[:helpers_end]
# Remove optional define_* helpers from HEAD if present.
helpers = re.sub(
    r"\n    macro_rules! define_gap_surface_families_from_list \{.*?\n    \}\n\n    macro_rules! define_gap_surface_existing_relays_from_list \{.*?\n    \}\n\n",
    "\n",
    helpers,
    count=1,
    flags=re.DOTALL,
)

tail_start = mod_body.index("    with_gap_surface_family_names!(gap_surface_families);")
tail = mod_body[tail_start:]

# Fix paste paths in tail
tail = tail.replace(
    "$crate::schema_gap_surfaces::paste::paste!",
    "::paste::paste! { $crate::schema_gap_surfaces::",
)
tail = re.sub(
    r"::paste::paste! \{ \$crate::schema_gap_surfaces::\{ \[<\$Name Connection>\] \}",
    r"::paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }",
    tail,
)

name_macros = f"""
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
            $crate::gap_surface_family_name_list!(@register $builder)
        }};
    }}

    #[macro_export]
    macro_rules! register_gap_surface_family_connections {{
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<::paste::paste! {{ $crate::schema_gap_surfaces::[<$Name Connection>] }}>(); )+
            b
        }};
    }}

"""

relay_macros = """
    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {
        (gap_surface_existing_relays) => {
            $crate::gap_surface_existing_relay_name_list!(@apply_relays);
        };
        (register_gap_surface_existing_relay_connections, $builder:expr) => {
            $crate::gap_surface_existing_relay_name_list!(@register $builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_existing_relay_connections {
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<::paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )+
            b
        }};
    }

"""

# tail includes with_gap_surface_family_names!(gap_surface_families) — replace HEAD's broken with/register in tail
tail = re.sub(
    r"    #\[macro_export\]\s*macro_rules! with_gap_surface_existing_relay_names \{.*?\n    \}\n\n    #\[macro_export\]\s*macro_rules! register_gap_surface_existing_relay_connections \{.*?\n    \}\n\n",
    "",
    tail,
    count=1,
    flags=re.DOTALL,
)
tail = re.sub(
    r"    #\[macro_export\]\s*macro_rules! with_gap_surface_family_names \{.*?\n    \}\n\n    #\[macro_export\]\s*macro_rules! register_gap_surface_family_connections \{.*?\n    \}\n\n    with_gap_surface_family_names!",
    "    with_gap_surface_family_names!",
    tail,
    count=1,
    flags=re.DOTALL,
)

new_mod = (
    "//#region 🩹️ schema_gap_surfaces\n\n"
    + helpers
    + name_macros
    + "    with_gap_surface_family_names!(gap_surface_families);\n\n"
    + re.sub(
        r"^\s*with_gap_surface_family_names!\(gap_surface_families\);\s*\n+",
        "",
        tail,
        count=1,
        flags=re.MULTILINE,
    ).lstrip()
)
# Insert relay with/register before with_gap_surface_existing_relay_names in tail
if "with_gap_surface_existing_relay_names!" in new_mod:
    new_mod = new_mod.replace(
        "    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);",
        relay_macros + "    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);",
        1,
    )

new_mod = new_mod.rstrip() + "\n}\n//#endregion 🩹️ schema_gap_surfaces\n"

lib.write_text(text[:start] + new_mod + text[end:], encoding="utf-8")
print("restored schema_gap_surfaces", len(new_mod), "chars")
