from pathlib import Path

lib_path = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
tmp_path = Path(r"c:\git\semio\.repo\tmp_lib.rs")
lib = lib_path.read_text(encoding="utf-8").splitlines(True)
tmp = tmp_path.read_text(encoding="utf-8").splitlines(True)


def schema_gap_span(lines: list[str]) -> tuple[int, int]:
    start = next(
        i
        for i, line in enumerate(lines)
        if "//#region" in line and "schema_gap_surfaces" in line
    )
    end = next(
        i
        for i, line in enumerate(lines)
        if i > start and "//#endregion" in line and "schema_gap" in line
    )
    return start, end


ls, le = schema_gap_span(lib)
ts, te = schema_gap_span(tmp)
patch = tmp[ts : te + 1]
text = "".join(patch)
if "@do_register" not in text:
    text = text.replace(
        "macro_rules! register_gap_surface_family_connections {\n"
        "        ($builder:expr, $($Name:ident),+ $(,)?) => {{",
        "macro_rules! register_gap_surface_family_connections {\n"
        "        (@do_register $builder:expr, $($Name:ident),* $(,)?) => {{\n"
        "            let mut b = $builder;\n"
        "            $( b = b.register_output_type::<::paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )*\n"
        "            b\n"
        "        }};\n"
        "        ($builder:expr, $($Name:ident),+ $(,)?) => {{",
        1,
    )
text = text.replace(
    "(@do_register $builder:expr, $($Name:ident),* $(,)?) => {\n"
    "            let mut b = $builder;\n"
    "            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! { [<$Name Connection>] }>(); )*\n"
    "            b\n"
    "        };",
    "(@do_register $builder:expr, $($Name:ident),* $(,)?) => {{\n"
    "            let mut b = $builder;\n"
    "            $( b = b.register_output_type::<::paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )*\n"
    "            b\n"
    "        }};",
)
new = lib[:ls] + text.splitlines(True) + lib[le + 1 :]
lib_path.write_text("".join(new), encoding="utf-8")
print(f"replaced {le - ls + 1} lines with {len(text.splitlines())} lines")
