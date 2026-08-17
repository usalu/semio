"""Reassemble full schema_gap_surfaces module (prefix from HEAD + name-list tail)."""
import json
from pathlib import Path

lib = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
text = lib.read_text(encoding="utf-8")
ticket = Path(__file__).parent
prefix = (ticket / "gap_mod_prefix.rs").read_text(encoding="utf-8")
names = json.loads((ticket / "gap_surface_names.json").read_text(encoding="utf-8"))


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
            $($crate::gap_surface_family!($Name);)+
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relays {{
        {{ $($Name:ident),* $(,)? }} => {{
            $($crate::gap_surface_existing_relay!($Name);)+
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_family_name_list {{
        (@names) => {{
{fam}
        }};
        (@apply_families) => {{
            $crate::gap_surface_families! {{
{fam}
            }}
        }};
        (@register $builder:expr) => {{
            $crate::gap_surface_family_name_list! {{
                @do_register_bridge $builder;
{fam}
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
        (@do_register $builder:expr, $($Name:ident),* $(,)?) => {{{{
            let mut b = $builder;
            $( b = {{ b.register_output_type::<::paste::paste! {{ $crate::schema_gap_surfaces::[<$Name Connection>] }}>() }}; )*
            b
        }}}};
    }}

    crate::gap_surface_family_name_list!(@apply_families);

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
        (@do_register $builder:expr, $($Name:ident),* $(,)?) => {{{{
            let mut b = $builder;
            $( b = {{ b.register_output_type::<::paste::paste! {{ $crate::schema_gap_surfaces::[<$Name Connection>] }}>() }}; )*
            b
        }}}};
    }}

    crate::gap_surface_existing_relay_name_list!(@apply_relays);
}}

//#endregion schema_gap_surfaces
"""

rs = text.index("//#region")
while "schema_gap_surfaces" not in text[rs : text.index("\n", rs)]:
    rs = text.index("//#region", rs + 1)
re = rs
while True:
    re = text.index("//#endregion", re + 1)
    line_end = text.index("\n", re)
    if "schema_gap_surfaces" in text[re:line_end]:
        break
re = line_end + 1
region_hdr = text[rs : text.index("\n", rs) + 1]
mod_body = prefix.rstrip() + "\n\n" + tail
new_text = text[:rs] + region_hdr + mod_body + "\n\n" + text[re:]
lib.write_text(new_text, encoding="utf-8")
print("reassembled", len(names["families"]), "families")
