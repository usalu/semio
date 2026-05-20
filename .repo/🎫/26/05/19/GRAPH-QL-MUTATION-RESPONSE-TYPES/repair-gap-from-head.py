"""Replace corrupted schema_gap_surfaces tail with clean inline-list macros from lib-head names."""
import re
from pathlib import Path

lib = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
head = Path(r"c:\git\semio\.repo\🎫\26\05\19\GRAPH-QL-MUTATION-RESPONSE-TYPES\lib-head.rs")
text = lib.read_text(encoding="utf-8")
head_text = head.read_text(encoding="utf-8")

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
    raise SystemExit("could not extract @names from lib-head")
family_names = fam_m.group(1).strip()
relay_names = relay_m.group(1).strip()

block = f"""
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
"""

pat = re.compile(
    r"    #\[macro_export\]\s*macro_rules! gap_surface_family_name_list \{.*?"
    r"with_gap_surface_existing_relay_names!\(gap_surface_existing_relays\);\s*",
    re.DOTALL,
)
new_text, n = pat.subn(block, text, count=1)
if n != 1:
    raise SystemExit(f"replace failed count={n}")

lib.write_text(new_text, encoding="utf-8")
print("ok", len(family_names.splitlines()), "family", len(relay_names.splitlines()), "relay")
