"""Rebuild schema_gap_surfaces mod tail from gap_surface_names.json snapshot."""
import json
import re
from pathlib import Path

path = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
names_path = Path(__file__).with_name("gap_surface_names.json")
text = path.read_text(encoding="utf-8")
names = json.loads(names_path.read_text(encoding="utf-8"))

_region_start_m = re.search(r"//#region .+ schema_gap_surfaces", text)
_region_end_m = re.search(r"//#endregion .+ schema_gap_surfaces", text)
if not _region_start_m or not _region_end_m:
    raise ValueError("schema_gap_surfaces region markers not found")
region_start = _region_start_m.start()
region_end = _region_end_m.start()
prefix_markers = [
    "\n    macro_rules! gap_surface_families {",
    "\n    macro_rules! __gap_surface_family_name_idents",
    "\n    #[macro_export]\n    macro_rules! gap_surface_family_name_list",
    "\n    #[macro_export]\n    macro_rules! gap_surface_families",
]
prefix_end = min(text.index(m, region_start) for m in prefix_markers if m in text[region_start:region_end])


def fmt_names(idents: list[str]) -> str:
    return "\n".join(f"        {n}," for n in idents[:-1]) + f"\n        {idents[-1]}"


fam = fmt_names(names["families"])
relay = fmt_names(names["relays"])

named_block = """
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
"""

tail = f"""    #[macro_export]
    macro_rules! gap_surface_families {{
        {{ $($Name:ident),* $(,)? }} => {{
            $(gap_surface_family!($Name);)+
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relays {{
        {{ $($Name:ident),* $(,)? }} => {{
            $(gap_surface_existing_relay!($Name);)+
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_family_name_list {{
        (@names) => {{
{fam}
        }};
        (@apply_families) => {{
            gap_surface_families! {{
{fam}
            }}
        }};
        (@register $builder:expr) => {{
            $crate::register_gap_surface_family_connections! {{
                @expand $builder;
{fam}
            }}
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {{
        (@names) => {{
{relay}
        }};
        (@apply_relays) => {{
            gap_surface_existing_relays! {{
{relay}
            }}
        }};
        (@register $builder:expr) => {{
            $crate::register_gap_surface_existing_relay_connections! {{
                @expand $builder;
{relay}
            }}
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
        (@expand $builder:expr; $($Name:ident),* $(,)?) => {{{{
            let mut b = $builder;
            $( b = b.register_output_type::<paste::paste! {{ $crate::schema_gap_surfaces::[<$Name Connection>] }}>(); )*
            b
        }}}};
    }}

    gap_surface_family_name_list!(@apply_families);

{named_block}
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
        (@expand $builder:expr; $($Name:ident),* $(,)?) => {{{{
            let mut b = $builder;
            $( b = b.register_output_type::<paste::paste! {{ $crate::schema_gap_surfaces::[<$Name Connection>] }}>(); )*
            b
        }}}};
    }}

    gap_surface_existing_relay_name_list!(@apply_relays);
}}

"""

new_text = text[:prefix_end] + tail + text[region_end:]
path.write_text(new_text, encoding="utf-8")
print("rebuilt", len(names["families"]), "families", len(names["relays"]), "relays")
