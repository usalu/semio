"""Line-based repair of schema_gap_surfaces name-list macros."""
from pathlib import Path

path = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
lines = path.read_text(encoding="utf-8").splitlines(keepends=True)

def find_line(substr: str, start: int = 0) -> int:
    for i in range(start, len(lines)):
        if substr in lines[i]:
            return i
    raise SystemExit(f"not found: {substr!r} from {start}")

fam_start = find_line("macro_rules! gap_surface_family_name_list")
to_brace_fam = find_line("(@to_brace)", fam_start)
relay_macro = find_line("macro_rules! gap_surface_existing_relay_name_list", to_brace_fam)
to_brace_relay = find_line("(@to_brace)", relay_macro)
with_fam = find_line("macro_rules! with_gap_surface_family_names", to_brace_relay)

# family: keep [fam_start, to_brace_fam), close macro, drop duplicate until relay_macro
head = lines[:to_brace_fam]
if not head[-1].strip().endswith("Input"):
    raise SystemExit("unexpected family list end")
head[-1] = head[-1].rstrip("\n") + "\n"
head.append("        };\n")
head.append("    }\n")
head.append("\n")
tail = lines[relay_macro:]
lines = head + tail

# re-find relay @to_brace after mutation
relay_macro = find_line("macro_rules! gap_surface_existing_relay_name_list")
to_brace_relay = find_line("(@to_brace)", relay_macro)
with_fam = find_line("macro_rules! with_gap_surface_family_names", to_brace_relay)

head = lines[:to_brace_relay]
if "WebsocketBackboneCommand" not in head[-1] and "WebsocketBackboneCommand" not in head[-2]:
    # trim trailing blank
    while head and not head[-1].strip():
        head.pop()
head.append("        };\n")
head.append("    }\n")
head.append("\n")
tail = lines[with_fam:]
lines = head + tail

text = "".join(lines)

# with_gap + module init
old = """    #[macro_export]
    macro_rules! with_gap_surface_family_names {
        (gap_surface_families) => {
            gap_surface_families!(gap_surface_family_name_list! {});
        };
        (register_gap_surface_family_connections, $builder:expr) => {
            register_gap_surface_family_connections! {
                @expand $builder;
                gap_surface_family_name_list! {}
            }
        };
    }"""

new = """    #[macro_export]
    macro_rules! with_gap_surface_family_names {
        (gap_surface_families) => {
            $crate::schema_gap_surfaces::gap_surface_families! {
                $($crate::gap_surface_family_name_list!{}),*
            }
        };
        (register_gap_surface_family_connections, $builder:expr) => {
            $crate::register_gap_surface_family_connections_from_name_list!($builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_family_connections_from_name_list {
        ($builder:expr) => {
            $crate::register_gap_surface_family_connections! {
                @expand $builder;
                $($crate::gap_surface_family_name_list!{}),*
            }
        };
    }"""

if old not in text:
    raise SystemExit("family with_gap not found")
text = text.replace(old, new, 1)

text = text.replace(
    "    with_gap_surface_family_names!(gap_surface_families);\n",
    "    gap_surface_families! { gap_surface_family_name_list!{} };\n",
    1,
)

old_r = """    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {
        (gap_surface_existing_relays) => {
            gap_surface_existing_relays!(gap_surface_existing_relay_name_list! {});
        };
        (register_gap_surface_existing_relay_connections, $builder:expr) => {
            $crate::register_gap_surface_existing_relay_connections! {
                @expand $builder;
                $crate::gap_surface_existing_relay_name_list! {}
            }
        };
    }"""

new_r = """    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {
        (gap_surface_existing_relays) => {
            $crate::schema_gap_surfaces::gap_surface_existing_relays! {
                $($crate::gap_surface_existing_relay_name_list!{}),*
            }
        };
        (register_gap_surface_existing_relay_connections, $builder:expr) => {
            $crate::register_gap_surface_existing_relay_connections_from_name_list!($builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_existing_relay_connections_from_name_list {
        ($builder:expr) => {
            $crate::register_gap_surface_existing_relay_connections! {
                @expand $builder;
                $($crate::gap_surface_existing_relay_name_list!{}),*
            }
        };
    }"""

if old_r not in text:
    raise SystemExit("relay with_gap not found")
text = text.replace(old_r, new_r, 1)

text = text.replace(
    "    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);\n",
    "    gap_surface_existing_relays! { gap_surface_existing_relay_name_list!{} };\n",
    1,
)

path.write_text(text, encoding="utf-8")
print("ok", "@to_brace count", text.count("@to_brace"))
