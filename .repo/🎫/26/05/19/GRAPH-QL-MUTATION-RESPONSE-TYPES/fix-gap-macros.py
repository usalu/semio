from pathlib import Path

p = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
lines = p.read_text(encoding="utf-8").splitlines(keepends=True)
start = next(i for i, l in enumerate(lines) if "(@apply_families) => {" in l)
end = next(
    i
    for i, l in enumerate(lines)
    if i > start and "macro_rules! gap_surface_existing_relay_name_list" in l
)
replacement = """        (@apply_families) => {
            define_gap_surface_families_from_list!(gap_surface_family_name_list!(@names));
        };
        (@register $builder:expr) => {
            $crate::register_gap_surface_family_connections!(
                @do_register $builder,
                gap_surface_family_name_list!(@names)
            )
        };
    }

"""
lines[start:end] = [replacement]
text = "".join(lines)
if "define_gap_surface_families_from_list" not in text:
    needle = "    macro_rules! gap_surface_families {"
    insert = """    macro_rules! define_gap_surface_families_from_list {
        ($($Name:ident),+ $(,)?) => {
            gap_surface_families! { $($Name),+ }
        };
    }

    macro_rules! define_gap_surface_existing_relays_from_list {
        ($($Name:ident),+ $(,)?) => {
            gap_surface_existing_relays! { $($Name),+ }
        };
    }

    macro_rules! gap_surface_families {"""
    text = text.replace(needle, insert, 1)

relay_chunk = text.split("macro_rules! gap_surface_existing_relay_name_list", 1)[1]
if "(@apply_relays)" not in relay_chunk.split("with_gap_surface_existing_relay_names", 1)[0]:
    text = text.replace(
        """        {} => {
            gap_surface_existing_relay_name_list!(@names);
        };
        (@register $builder:expr) => {{""",
        """        {} => {
            gap_surface_existing_relay_name_list!(@names);
        };
        (@apply_relays) => {
            define_gap_surface_existing_relays_from_list!(gap_surface_existing_relay_name_list!(@names));
        };
        (@register $builder:expr) => {{""",
        1,
    )

p.write_text(text, encoding="utf-8")
print(f"fixed: removed {end - start} lines at {start + 1}-{end}")
