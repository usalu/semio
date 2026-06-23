import re
from pathlib import Path

p = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
text = p.read_text(encoding="utf-8")


def ensure_register_arm(macro_name: str, register_macro: str) -> None:
    global text
    if f"macro_rules! {macro_name}" not in text:
        raise SystemExit(f"missing {macro_name}")
    if f"(@register $builder:expr)" in text.split(f"macro_rules! {macro_name}")[1].split(
        "macro_rules!", 1
    )[0]:
        print(f"{macro_name}: @register exists")
        return
    pattern = (
        rf"(macro_rules! {re.escape(macro_name)} \{{\s*"
        r"\(@names\) => \{\s*"
        r"(.*?)"
        r"\s*\};\s*)"
        r"(\{\} =>)"
    )
    m = re.search(pattern, text, re.S)
    if not m:
        raise SystemExit(f"could not parse {macro_name}")
    prefix, names, empty_arm = m.group(1), m.group(2).strip(), m.group(3)
    arm = f"""
        (@register $builder:expr) => {{
            $crate::{register_macro}!($builder,
        {names}
            );
        }};
        """
    replacement = prefix + arm + empty_arm
    text = text[: m.start()] + replacement + text[m.end() :]
    print(f"{macro_name}: inserted @register")


ensure_register_arm("gap_surface_family_name_list", "register_gap_surface_family_connections")
ensure_register_arm(
    "gap_surface_existing_relay_name_list",
    "register_gap_surface_existing_relay_connections",
)

text = re.sub(
    r"\(register_gap_surface_family_connections, \$builder:expr\) => \{[^}]+\};",
    """(register_gap_surface_family_connections, $builder:expr) => {
            $crate::gap_surface_family_name_list!(@register $builder);
        };""",
    text,
    count=1,
)
text = re.sub(
    r"\(register_gap_surface_existing_relay_connections, \$builder:expr\) => \{[^}]+\};",
    """(register_gap_surface_existing_relay_connections, $builder:expr) => {
            $crate::gap_surface_existing_relay_name_list!(@register $builder);
        };""",
    text,
    count=1,
)

p.write_text(text, encoding="utf-8")
print("done")
