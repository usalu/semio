"""Patch gap_surface_*_name_list macros using __gap_*_idents helpers already in lib.rs."""
import re
import subprocess
from pathlib import Path

root = Path(r"c:\git\compose")
lib_path = root / "compose/client/lib/rs/lib.rs"
text = lib_path.read_text(encoding="utf-8")


def extract_ident_macro(src: str, name: str) -> str:
    m = re.search(
        rf"macro_rules! {re.escape(name)} \{{\s*\(\) => \{{\s*(.*?)\s*\}};\s*\}}",
        src,
        re.S,
    )
    if not m:
        raise SystemExit(f"{name} not found")
    return m.group(1).strip()


family = extract_ident_macro(text, "__gap_surface_family_name_idents")
relay = extract_ident_macro(text, "__gap_surface_existing_relay_name_idents")

name_list_macros = f"""    #[macro_export]
    macro_rules! gap_surface_family_name_list {{
        (@names) => {{ __gap_surface_family_name_idents!() }};
        (@apply_families) => {{
            gap_surface_family_name_list! {{
                @emit_families;
                __gap_surface_family_name_idents!()
            }}
        }};
        (@emit_families; $($Name:ident),* $(,)?) => {{
            $crate::schema_gap_surfaces::gap_surface_families! {{ $($Name),* }}
        }};
        (@register $builder:expr) => {{
            gap_surface_family_name_list! {{
                @do_register_bridge $builder;
                __gap_surface_family_name_idents!()
            }}
        }};
        (@do_register_bridge $builder:expr; $($Name:ident),* $(,)?) => {{
            $crate::register_gap_surface_family_connections!($builder, $($Name),*)
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {{
        (@names) => {{ __gap_surface_existing_relay_name_idents!() }};
        (@apply_relays) => {{
            gap_surface_existing_relay_name_list! {{
                @emit_relays;
                __gap_surface_existing_relay_name_idents!()
            }}
        }};
        (@emit_relays; $($Name:ident),* $(,)?) => {{
            $crate::schema_gap_surfaces::gap_surface_existing_relays! {{ $($Name),* }}
        }};
        (@register $builder:expr) => {{
            gap_surface_existing_relay_name_list! {{
                @do_register_bridge $builder;
                __gap_surface_existing_relay_name_idents!()
            }}
        }};
        (@do_register_bridge $builder:expr; $($Name:ident),* $(,)?) => {{
            $crate::register_gap_surface_existing_relay_connections!($builder, $($Name),*)
        }};
    }}

"""

start = text.index("    #[macro_export]\n    macro_rules! gap_surface_family_name_list {")
end = text.index("    #[macro_export]\n    macro_rules! with_gap_surface_family_names {")
text = text[:start] + name_list_macros + text[end:]

# ensure register macros use expression braces
text = text.replace(
    """    macro_rules! register_gap_surface_family_connections {
        ($builder:expr, $($Name:ident),+ $(,)?) => {
            {
                let mut b = $builder;
                $( b = b.register_output_type::<::paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )+
                b
            }
        };
    }""",
    """    #[macro_export]
    macro_rules! register_gap_surface_family_connections {
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<::paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )+
            b
        }};
    }""",
)

text = re.sub(
    r"#\[macro_export\]\s*\n\s*#\[macro_export\]\s*\n\s*macro_rules! register_gap_surface_existing_relay_connections",
    "#[macro_export]\n    macro_rules! register_gap_surface_existing_relay_connections",
    text,
    count=1,
)

text = re.sub(
    r"macro_rules! register_gap_surface_existing_relay_connections \{\s*\(\$builder:expr,.*?\}\s*;\s*\}",
    """#[macro_export]
    macro_rules! register_gap_surface_existing_relay_connections {
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<::paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )+
            b
        }};
    }""",
    text,
    count=1,
    flags=re.S,
)

lib_path.write_text(text, encoding="utf-8")
print("patched name_list macros")

build = subprocess.run(
    ["bun", "scripts/build-wasm.script.mjs"],
    cwd=root / "compose/client/lib/rs",
    capture_output=True,
    text=True,
    encoding="utf-8",
)
if build.returncode != 0:
    print((build.stderr or "") + (build.stdout or ""))[-6000:]
    raise SystemExit(build.returncode)
print("wasm-pack ok")

sketch = subprocess.run(
    ["bun", "nx", "run", "@compose/sketchpad:build"],
    cwd=root,
    capture_output=True,
    text=True,
    encoding="utf-8",
)
if sketch.returncode != 0:
    print((sketch.stderr or "") + (sketch.stdout or ""))[-8000:]
    raise SystemExit(sketch.returncode)
print("sketchpad build ok")
