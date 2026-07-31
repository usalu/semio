"""Rewrite entire schema_gap_surfaces region in lib.rs (atomic, no partial merge)."""
import re
from pathlib import Path

lib = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
head = Path(r"c:\git\compose\.repo\🎫️\26\05\19\GRAPH-QL-MUTATION-RESPONSE-TYPES\lib-head.rs")
text = lib.read_text(encoding="utf-8")
head_text = head.read_text(encoding="utf-8")

start_m = re.search(r"//#region[^\n]*schema_gap_surfaces", text)
end_m = re.search(r"//#endregion[^\n]*schema_gap_surfaces", text)
if not start_m or not end_m:
    raise SystemExit("region markers missing")
start, end = start_m.start(), end_m.end()

mod_start = head_text.index("pub mod schema_gap_surfaces {")
helpers_end = head_text.index("    macro_rules! define_gap_surface_families_from_list", mod_start)
helpers = head_text[mod_start:helpers_end]

fam_m = re.search(
    r"macro_rules! gap_surface_family_name_list \{\s*\(@names\) => \{\s*(.*?)\n        \};\s*\(\)",
    head_text,
    re.DOTALL,
)
relay_m = re.search(
    r"macro_rules! gap_surface_existing_relay_name_list \{\s*\(@names\) => \{\s*(.*?)\n        \};\s*\(\)",
    head_text,
    re.DOTALL,
)
if not fam_m or not relay_m:
    raise SystemExit("name lists missing in lib-head")
family_names = fam_m.group(1).strip()
relay_names = relay_m.group(1).strip()

tail = f"""
    macro_rules! gap_surface_families {{
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
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<::paste::paste! {{ $crate::schema_gap_surfaces::[<$Name Connection>] }}>(); )+
            b
        }};
    }}

    with_gap_surface_family_names!(gap_surface_families);

    gap_surface_family_named!(
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

    #[macro_export]
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
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<::paste::paste! {{ $crate::schema_gap_surfaces::[<$Name Connection>] }}>(); )+
            b
        }};
    }}

    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);
}}
"""

new_region = "//#region schema_gap_surfaces\n\n" + helpers + tail + "\n//#endregion schema_gap_surfaces\n"
out = Path(r"c:\git\compose\.repo\🎫️\26\05\19\GRAPH-QL-MUTATION-RESPONSE-TYPES\schema_gap_surfaces-patch.rs")
out.write_text(new_region, encoding="utf-8")
merged = text[:start] + new_region + text[end:]
lib.write_text(merged, encoding="utf-8", newline="\n")
print("rewrote schema_gap_surfaces", len(new_region), "chars")
