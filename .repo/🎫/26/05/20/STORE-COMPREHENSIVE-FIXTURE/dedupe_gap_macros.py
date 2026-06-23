from pathlib import Path

p = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
lines = p.read_text(encoding="utf-8").splitlines(keepends=True)


def find_contains(substr: str, start: int = 0) -> int:
    for i in range(start, len(lines)):
        if substr in lines[i]:
            return i
    raise SystemExit(f"not found: {substr!r} from {start}")


i_family_macro = find_contains("macro_rules! gap_surface_family_name_list {")
i_apply = find_contains("(@apply_families)", i_family_macro + 1)
i_relay_macro = find_contains("macro_rules! gap_surface_existing_relay_name_list", i_apply + 1)
i_apply_r = find_contains("(@apply_relays)", i_relay_macro + 1)
i_with_family = find_contains("macro_rules! with_gap_surface_family_names", i_apply_r + 1)

family_tail = """        (@apply_families) => {
            gap_surface_family_name_list! {
                @emit_families_from_names => gap_surface_family_name_list!(@names)
            };
        };
        (@emit_families_from_names => $($Name:ident),+ $(,)?) => {
            gap_surface_families! { $($Name),+ }
        };
        (@register $builder:expr) => {{
            gap_surface_family_name_list! {
                @do_register_bridge $builder => gap_surface_family_name_list!(@names)
            };
        }};
        (@do_register_bridge $builder:expr => $($Name:ident),+ $(,)?) => {
            $crate::register_gap_surface_family_connections!(@expand $builder; $($Name),+)
        };
    }

"""
relay_tail = """        (@apply_relays) => {
            gap_surface_existing_relay_name_list! {
                @emit_relays_from_names => gap_surface_existing_relay_name_list!(@names)
            };
        };
        (@emit_relays_from_names => $($Name:ident),+ $(,)?) => {
            gap_surface_existing_relays! { $($Name),+ }
        };
        (@register $builder:expr) => {{
            gap_surface_existing_relay_name_list! {
                @do_register_bridge $builder => gap_surface_existing_relay_name_list!(@names)
            };
        }};
        (@do_register_bridge $builder:expr => $($Name:ident),+ $(,)?) => {
            $crate::register_gap_surface_existing_relay_connections!(@expand $builder; $($Name),+)
        };
    }

"""
with_family_fix = """    #[macro_export]
    macro_rules! with_gap_surface_family_names {
        (gap_surface_families) => {
            $crate::gap_surface_family_name_list!(@apply_families);
        };
        (register_gap_surface_family_connections, $builder:expr) => {{
            $crate::gap_surface_family_name_list!(@register $builder)
        }};
    }

"""

new_lines = (
    lines[:i_apply]
    + [family_tail]
    + lines[i_relay_macro:i_apply_r]
    + [relay_tail]
    + [with_family_fix]
    + lines[i_with_family + 5 :]  # skip old with_gap macro (~5 lines)
)
# Find end of old with_gap block more reliably
j = i_with_family
while j < len(lines) and not lines[j].strip().startswith("macro_rules! register_gap_surface_family_connections"):
    j += 1
new_lines = (
    lines[:i_apply]
    + [family_tail]
    + lines[i_relay_macro:i_apply_r]
    + [relay_tail]
    + [with_family_fix]
    + lines[j:]
)
p.write_text("".join(new_lines), encoding="utf-8")
print("ok", i_apply + 1, i_relay_macro + 1, i_apply_r + 1, i_with_family + 1, j + 1)
