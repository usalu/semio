from pathlib import Path
import re

p = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
t = p.read_text(encoding="utf-8")
mod_start = t.index("pub mod schema_gap_surfaces {")
mod_end = t.index("//#endregion 🩹 schema_gap_surfaces", mod_start)
mod = t[mod_start:mod_end]

head_end = mod.index("    #[macro_export]\n    macro_rules! gap_surface_family_name_list {")
head = mod[:head_end]

fm = re.search(
    r"macro_rules! gap_surface_family_name_list \{\s*\(@names\) => \{\s*(.*?)\s*\};\s*\(\) =>",
    mod,
    re.S,
)
if not fm:
    fm = re.search(
        r"macro_rules! gap_surface_family_name_list \{\s*\(@names\) => \{\s*(.*?)\s*\};\s*\(@apply",
        mod,
        re.S,
    )
if not fm:
    raise SystemExit("family @names not found")
family = fm.group(1).strip()

rm = re.search(
    r"macro_rules! gap_surface_existing_relay_name_list \{\s*\(@names\) => \{\s*(.*?)\s*\};\s*\(\) =>",
    mod,
    re.S,
)
if not rm:
    rm = re.search(
        r"macro_rules! gap_surface_existing_relay_name_list \{\s*\(@names\) => \{\s*(.*?)\s*\};\s*\(@apply",
        mod,
        re.S,
    )
if not rm:
    raise SystemExit("relay @names not found")
relay = rm.group(1).strip()

named_section = """    gap_surface_family_named!(
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
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! {{ [<$Name Connection>] }}>(); )*
            b
        }};
    }}

    with_gap_surface_family_names!(gap_surface_families);

{named_section}    #[macro_export]
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
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! {{ [<$Name Connection>] }}>(); )*
            b
        }};
    }}

    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);
}}
"""

new_mod = head + tail
t = t[:mod_start] + new_mod + t[mod_end:]
p.write_text(t, encoding="utf-8")
print("finalized", len(new_mod))
