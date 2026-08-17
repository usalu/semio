"""Patch schema_gap_surfaces macros and build before concurrent edits land."""
import re
import subprocess
from pathlib import Path

root = Path(r"c:\git\compose")
lib = root / "compose/client/lib/rs/lib.rs"
text = lib.read_text(encoding="utf-8")

text = text.replace(
    "$crate::schema_gap_surfaces::__gap_surface_family_name_idents!()",
    "$crate::gap_surface_family_name_list!(@names)",
)
text = text.replace(
    "$crate::schema_gap_surfaces::__gap_surface_existing_relay_name_idents!()",
    "$crate::gap_surface_existing_relay_name_list!(@names)",
)
text = text.replace("__gap_surface_family_name_idents!()", "gap_surface_family_name_list!(@names)")
text = text.replace(
    "__gap_surface_existing_relay_name_idents!()",
    "gap_surface_existing_relay_name_list!(@names)",
)

text = re.sub(
    r"\$crate::gap_surface_family_name_list!\(\s*@emit_families;\s*[^)]+\)\s*;",
    "$crate::gap_surface_family_name_list! {\n                @emit_families;\n                $crate::gap_surface_family_name_list!(@names)\n            }",
    text,
    count=1,
)
text = re.sub(
    r"\$crate::gap_surface_existing_relay_name_list!\(\s*@emit_relays;\s*[^)]+\)\s*;",
    "$crate::gap_surface_existing_relay_name_list! {\n                @emit_relays;\n                $crate::gap_surface_existing_relay_name_list!(@names)\n            }",
    text,
    count=1,
)

text = text.replace(
    "$crate::register_gap_surface_family_connections!(@expand $builder; $($Name),*)",
    "$crate::register_gap_surface_family_connections!($builder, $($Name),*)",
)
text = text.replace(
    "$crate::register_gap_surface_existing_relay_connections!(@expand $builder; $($Name),*)",
    "$crate::register_gap_surface_existing_relay_connections!($builder, $($Name),*)",
)

text = re.sub(
    r"macro_rules! register_gap_surface_family_connections \{\s*@expand[^}]+\}\s*;\s*\}",
    """macro_rules! register_gap_surface_family_connections {
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
text = re.sub(
    r"macro_rules! register_gap_surface_existing_relay_connections \{\s*@expand[^}]+\}\s*;\s*\}",
    """macro_rules! register_gap_surface_existing_relay_connections {
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

text = text.replace(
    "gap_surface_existing_relay_name_list!(@apply_relays);",
    "with_gap_surface_existing_relay_names!(gap_surface_existing_relays);",
)
text = text.replace(
    "gap_surface_family_name_list!(@apply_families);",
    "with_gap_surface_family_names!(gap_surface_families);",
)

lib.write_text(text, encoding="utf-8")
print("patched lib.rs")

for label, cmd, cwd in [
    ("wasm-pack", ["bun", "scripts/build-wasm.script.mjs"], root / "compose/client/lib/rs"),
    ("sketchpad", ["bun", "nx", "run", "@compose/sketchpad:build"], root),
]:
    r = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, encoding="utf-8")
    if r.returncode != 0:
        print((r.stderr or "") + (r.stdout or ""))[-6000:]
        raise SystemExit(f"{label} failed")
    print(f"{label} ok")
