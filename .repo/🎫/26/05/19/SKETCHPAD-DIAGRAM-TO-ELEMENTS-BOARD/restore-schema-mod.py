import subprocess
from pathlib import Path

root = Path(r"c:\git\semio")
head = subprocess.check_output(
    ["git", "show", "HEAD:semio/client/lib/rs/lib.rs"],
    cwd=root,
    text=True,
    encoding="utf-8",
)
cur_path = root / "semio/client/lib/rs/lib.rs"
cur = cur_path.read_text(encoding="utf-8")


def extract_mod(text: str, mod_name: str) -> str:
    start = text.index(f"pub mod {mod_name} {{")
    depth = 0
    for idx in range(start, len(text)):
        ch = text[idx]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return text[start : idx + 1]
    raise RuntimeError(f"unclosed {mod_name}")


def replace_region(text: str, region_tag: str, mod_name: str, mod_src: str) -> str:
    region_start = text.index(f"//#region {region_tag}")
    region_end = text.index(f"//#endregion {region_tag}") + len(f"//#endregion {region_tag}")
    replacement = f"//#region {region_tag}\n\n{mod_src}\n\n//#endregion {region_tag}"
    return text[:region_start] + replacement + text[region_end:]


head_mod = extract_mod(head, "schema_gap_surfaces")
new_cur = replace_region(cur, "🩹 schema_gap_surfaces", "schema_gap_surfaces", head_mod)

# apply fixes on restored mod text
head_mod_fixed = head_mod
# ensure gap_surface_families use brace + @names pattern from working version
if "gap_surface_family_name_list" not in head_mod_fixed:
    raise SystemExit("HEAD missing name_list macro")

cur_path.write_text(new_cur, encoding="utf-8")
print("restored schema_gap_surfaces from HEAD, len", len(head_mod))
