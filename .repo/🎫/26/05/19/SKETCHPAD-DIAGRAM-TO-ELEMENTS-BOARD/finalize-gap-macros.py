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

helper_macros = f"""    macro_rules! __gap_surface_family_name_idents {{
        () => {{
        {family_names}
        }};
    }}

    macro_rules! __gap_surface_existing_relay_name_idents {{
        () => {{
        {relay_names}
        }};
    }}

"""

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
            $crate::gap_surface_family_name_list!(@register $builder)
        }};
    }}

    #[macro_export]
    macro_rules! register_gap_surface_family_connections {{
        ($builder:expr, $($Name:ident),+ $(,)?) => {{{{
            let mut b = $builder;
            $( b = b.register_output_type::<::paste::paste! {{ $crate::schema_gap_surfaces::[<$Name Connection>] }}>(); )+
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
            $crate::gap_surface_existing_relay_name_list!(@register $builder)
        }};
    }}

    #[macro_export]
    macro_rules! register_gap_surface_existing_relay_connections {{
        ($builder:expr, $($Name:ident),+ $(,)?) => {{{{
            let mut b = $builder;
            $( b = b.register_output_type::<::paste::paste! {{ $crate::schema_gap_surfaces::[<$Name Connection>] }}>(); )+
            b
        }}}};
    }}

    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);

"""

# rebuild head_mod internals (keep gap_surface_family_named! block from HEAD)
_start_opts = [
    "    macro_rules! define_gap_surface_families_from_list {",
    "    macro_rules! gap_surface_families {",
]
start = min(head_mod.index(s) for s in _start_opts if s in head_mod)
named_start = head_mod.index("    gap_surface_family_named!(")
relay_macro_start = head_mod.index("    macro_rules! with_gap_surface_existing_relay_names {")
relay_invoke = "    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);"
end = head_mod.index(relay_invoke) + len(relay_invoke) + 1
new_mod = (
    head_mod[:start]
    + macros
    + head_mod[named_start:relay_macro_start]
    + relay_tail
    + head_mod[end:]
)

# replace in current file
cur_path = root / "semio/client/lib/rs/lib.rs"
cur = cur_path.read_text(encoding="utf-8")
region_start = re.search(r"//#region[^\n]*schema_gap_surfaces", cur).start()
region_end = re.search(r"//#endregion[^\n]*schema_gap_surfaces", cur).end()
region_header = cur[region_start : cur.index("\n", region_start) + 1]
region_footer = cur[cur.rfind("//#endregion", region_start, region_end) : region_end]
replacement = f"{region_header}\n{new_mod}\n{region_footer}"
cur_path.write_text(cur[:region_start] + replacement + cur[region_end:], encoding="utf-8")
print("finalized schema_gap_surfaces", len(new_mod))

build = subprocess.run(
    ["bun", "scripts/build-wasm.script.mjs"],
    cwd=root / "semio/client/lib/rs",
    capture_output=True,
    text=True,
    encoding="utf-8",
)
if build.returncode != 0:
    err = (build.stderr or "") + (build.stdout or "")
    print(err[-8000:])
    raise SystemExit(build.returncode)
print("wasm-pack ok")

sketch = subprocess.run(
    ["bun", "nx", "run", "@semio/sketchpad:build"],
    cwd=root,
    capture_output=True,
    text=True,
    encoding="utf-8",
)
if sketch.returncode != 0:
    err = (sketch.stderr or "") + (sketch.stdout or "")
    print(err[-10000:])
    raise SystemExit(sketch.returncode)
print("sketchpad build ok")
