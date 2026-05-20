#!/usr/bin/env python3
"""Restore gap-surface macros from .repo/lib.rs.head roster into lib.rs."""
from pathlib import Path

HEAD = Path(r"c:\git\semio\.repo\lib.rs.head")
LIB = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")


def extract_idents(lines: list[str], marker: str) -> list[str]:
    i = next(x for x, ln in enumerate(lines) if marker in ln)
    capture = False
    idents: list[str] = []
    for ln in lines[i:]:
        if "(@names) => {" in ln:
            capture = True
            continue
        if capture:
            if ln.strip() in ("};", "}"):
                break
            s = ln.strip().rstrip(",")
            if s and s.isidentifier():
                idents.append(s)
    return idents


head_lines = HEAD.read_text(encoding="utf-8").splitlines(keepends=True)
family_idents = extract_idents(head_lines, "macro_rules! gap_surface_family_name_list")
relay_idents = extract_idents(head_lines, "macro_rules! gap_surface_existing_relay_name_list")

lines = LIB.read_text(encoding="utf-8").splitlines(keepends=True)
mod_start = next(i for i, ln in enumerate(lines) if ln.startswith("pub mod schema_gap_surfaces"))
for marker in (
    "macro_rules! define_gap_surface_families_from_list",
    "macro_rules! __gap_surface_family_name_idents",
    "macro_rules! gap_surface_families",
    "macro_rules! gap_surface_family_name_list",
):
    try:
        block_start = next(
            i for i, ln in enumerate(lines) if i > mod_start and marker in ln
        )
        break
    except StopIteration:
        continue
else:
    raise SystemExit("could not find gap-surface macro block start")
endregion_idx = next(
    i for i, ln in enumerate(lines) if i > block_start and "//#endregion" in ln
)


def ident_lines(idents: list[str], indent: str) -> str:
    return "".join(f"{indent}{n},\n" for n in idents)


block = f"""    macro_rules! __gap_surface_family_name_idents {{
        () => {{
{ident_lines(family_idents, "        ")}        }};
    }}

    macro_rules! __gap_surface_existing_relay_name_idents {{
        () => {{
{ident_lines(relay_idents, "        ")}        }};
    }}

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
        (@names) => {{ __gap_surface_family_name_idents!() }};
        () => {{
            gap_surface_families! {{ __gap_surface_family_name_idents!() }}
        }};
        (@register $builder:expr) => {{
            $crate::register_gap_surface_family_connections!(
                $builder,
                __gap_surface_family_name_idents!()
            )
        }};
    }}

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {{
        (@names) => {{ __gap_surface_existing_relay_name_idents!() }};
        () => {{
            gap_surface_existing_relays! {{ __gap_surface_existing_relay_name_idents!() }}
        }};
        (@register $builder:expr) => {{
            $crate::register_gap_surface_existing_relay_connections!(
                $builder,
                __gap_surface_existing_relay_name_idents!()
            )
        }};
    }}

    #[macro_export]
    macro_rules! with_gap_surface_family_names {{
        (gap_surface_families) => {{ $crate::gap_surface_family_name_list!(); }};
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
        (gap_surface_existing_relays) => {{ $crate::gap_surface_existing_relay_name_list!(); }};
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

}}

"""

out = "".join(lines[:block_start] + [block] + lines[endregion_idx:])
tmp = LIB.with_suffix(".rs.tmp")
tmp.write_text(out, encoding="utf-8")
tmp.replace(LIB)
print(f"restored {len(family_idents)} family + {len(relay_idents)} relay idents")

