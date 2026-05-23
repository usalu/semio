import os
import re
import subprocess
from pathlib import Path

ROOT = Path(r"c:\git\semio")
LIB = ROOT / "semio/client/lib/rs/lib.rs"
RS = ROOT / "semio/client/lib/rs"


def restore_head() -> str:
    r = subprocess.run(
        ["git", "show", "HEAD:semio/client/lib/rs/lib.rs"],
        cwd=ROOT,
        capture_output=True,
        check=True,
    )
    return r.stdout.decode("utf-8")


def finalize(t: str) -> str:
    if not t.startswith("//!"):
        raise SystemExit("not full lib.rs")
    mod_start = t.index("pub mod schema_gap_surfaces {")
    idx = mod_start
    mod_end = None
    while True:
        try:
            pos = t.index("\n//#endregion", idx)
        except ValueError:
            break
        line_end = t.find("\n", pos + 1)
        line = t[pos:line_end if line_end != -1 else len(t)]
        if "schema_gap_surfaces" in line:
            mod_end = pos
        idx = pos + 1
    if mod_end is None:
        raise SystemExit("schema_gap_surfaces endregion not found")
    mod = t[mod_start:mod_end]
    markers = [
        "    macro_rules! define_gap_surface_families_from_list {",
        "    macro_rules! __gap_surface_family_name_idents {",
        "    #[macro_export]\n    macro_rules! gap_surface_family_name_list {",
    ]
    head_end = min(mod.index(m) for m in markers if m in mod)
    fm = re.search(
        r"macro_rules! gap_surface_family_name_list \{\s*\(@names\) => \{\s*(.*?UpdatedTypeIconInput\s*)\};\s*",
        mod,
        re.S,
    )
    if not fm:
        fm = re.search(
            r"macro_rules! __gap_surface_family_name_idents \{\s*\(\) => \{\s*(.*?UpdatedTypeIconInput\s*)\s*\};\s*\}",
            mod,
            re.S,
        )
    rm = re.search(
        r"macro_rules! gap_surface_existing_relay_name_list \{\s*\(@names\) => \{\s*(.*?WebsocketBackboneCommand\s*)\};\s*",
        mod,
        re.S,
    )
    if not rm:
        rm = re.search(
            r"macro_rules! __gap_surface_existing_relay_name_idents \{\s*\(\) => \{\s*(.*?WebsocketBackboneCommand\s*)\s*\};\s*\}",
            mod,
            re.S,
        )
    if not fm or not rm:
        raise SystemExit("name lists not found")
    family = fm.group(1).strip()
    relay = rm.group(1).strip()
    head = mod[:head_end]
    helpers = """    #[macro_export]
    macro_rules! gap_surface_families {
        { $($Name:ident),* $(,)? } => {
            $(gap_surface_family!($Name);)+
        };
    }

    #[macro_export]
    macro_rules! gap_surface_existing_relays {
        { $($Name:ident),* $(,)? } => {
            $(gap_surface_existing_relay!($Name);)+
        };
    }

"""
    if "macro_rules! gap_surface_families" not in head:
        head += helpers
    elif "#[macro_export]\n    macro_rules! gap_surface_families" not in head:
        head = head.replace(
            "    macro_rules! gap_surface_families {",
            "    #[macro_export]\n    macro_rules! gap_surface_families {",
            1,
        )
        head = head.replace(
            "    macro_rules! gap_surface_existing_relays {",
            "    #[macro_export]\n    macro_rules! gap_surface_existing_relays {",
            1,
        )
    named = """    gap_surface_family_named!(
        "ChangedDescriptionInput",
        GapChangedDescriptionInput,
        "ChangedDescriptionInputEdge",
        GapChangedDescriptionInputEdge,
        "ChangedDescriptionInputConnection",
        GapChangedDescriptionInputConnection
    );
    gap_surface_family_named!("Clump", GapClump, "ClumpEdge", GapClumpEdge, "ClumpConnection", GapClumpConnection);
    gap_surface_family_named!(
        "CreatedFixedPieceInput",
        GapCreatedFixedPieceInput,
        "CreatedFixedPieceInputEdge",
        GapCreatedFixedPieceInputEdge,
        "CreatedFixedPieceInputConnection",
        GapCreatedFixedPieceInputConnection
    );
    gap_surface_family_named!("DesignDiff", GapDesignDiff, "DesignDiffEdge", GapDesignDiffEdge, "DesignDiffConnection", GapDesignDiffConnection);
    gap_surface_family_named!(
        "DraggedPieceInput",
        GapDraggedPieceInput,
        "DraggedPieceInputEdge",
        GapDraggedPieceInputEdge,
        "DraggedPieceInputConnection",
        GapDraggedPieceInputConnection
    );
    gap_surface_family_named!("KitDiff", GapKitDiff, "KitDiffEdge", GapKitDiffEdge, "KitDiffConnection", GapKitDiffConnection);
    gap_surface_family_named!(
        "RenamedKitInput",
        GapRenamedKitInput,
        "RenamedKitInputEdge",
        GapRenamedKitInputEdge,
        "RenamedKitInputConnection",
        GapRenamedKitInputConnection
    );
    gap_surface_family_named!("Version", GapVersion, "VersionEdge", GapVersionEdge, "VersionConnection", GapVersionConnection);

"""
    tail = f"""    #[macro_export]
    macro_rules! gap_surface_family_name_list {{
        (@names) => {{
        {family}
        }};
        () => {{
            $crate::gap_surface_family_name_list!(@apply_families);
        }};
        (@apply_families) => {{
            $crate::gap_surface_families! {{
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
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {{
        (@names) => {{
        {relay}
        }};
        () => {{
            $crate::gap_surface_existing_relay_name_list!(@apply_relays);
        }};
        (@apply_relays) => {{
            $crate::gap_surface_existing_relays! {{
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
        (@do_register $builder:expr, $($Name:ident),* $(,)?) => {{
            {{
                let mut b = $builder;
                $( b = b.register_output_type::<::paste::paste! {{ $crate::schema_gap_surfaces::[<$Name Connection>] }}>(); )*
                b
            }}
        }};
    }}

    with_gap_surface_family_names!(gap_surface_families);

{named}    #[macro_export]
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
        (@do_register $builder:expr, $($Name:ident),* $(,)?) => {{
            {{
                let mut b = $builder;
                $( b = b.register_output_type::<::paste::paste! {{ $crate::schema_gap_surfaces::[<$Name Connection>] }}>(); )*
                b
            }}
        }};
    }}

    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);
}}
"""
    new_mod = head + tail
    t2 = t[:mod_start] + new_mod + t[mod_end:]
    if "__gap_surface" in t2:
        raise SystemExit("__gap_surface remains")
    if t2.count("macro_rules! gap_surface_family_name_list") != 1:
        raise SystemExit("duplicate family list macro")
    return t2


def main() -> None:
    t = restore_head()
    t2 = finalize(t)
    tmp = LIB.with_suffix(".rs.tmp")
    tmp.write_text(t2, encoding="utf-8")
    os.replace(tmp, LIB)
    written = LIB.read_text(encoding="utf-8")
    if "gap_surface_family_name_list!();" in written or "gap_surface_existing_relay_name_list!();" in written:
        raise SystemExit("finalize output still contains bare () invocations")
    if "@do_register_bridge" not in written:
        raise SystemExit("finalize output missing @do_register_bridge")
    print("wrote", len(t2), "bytes")
    subprocess.run(["cargo", "build", "--lib"], cwd=RS, check=True)
    print("cargo build --lib OK")
    env = os.environ.copy()
    env.pop("SEMIO_SKIP_WASM_BUILD", None)
    subprocess.run(
        ["bun", "nx", "run", "@semio/rs:build"],
        cwd=ROOT,
        check=True,
        env=env,
    )
    print("@semio/rs:build OK")
    subprocess.run(
        ["bun", "nx", "run", "@semio/sketchpad:build"],
        cwd=ROOT,
        check=True,
    )
    print("@semio/sketchpad:build OK")


if __name__ == "__main__":
    main()
