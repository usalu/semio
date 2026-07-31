"""Splice clean schema_gap_surfaces into current lib.rs (keep operation WIP), build."""
import re
import subprocess
from pathlib import Path

root = Path(r"c:\git\compose")
lib_path = root / "compose/client/lib/rs/lib.rs"

ns: dict = {}
exec(
    open(
        root / ".repo/🎫/26/05/19/SKETCHPAD-DIAGRAM-TO-ELEMENTS-BOARD/finalize-gap-macros.py",
        encoding="utf-8",
    ).read().split("cur_path = root")[0],
    ns,
)


def sanity_ok(text: str) -> bool:
    return all(
        [
            "@apply_families" in text,
            "@do_register_bridge" not in text,
            "__gap_surface_family_name_idents" not in text,
            "with_gap_surface_existing_relay_names!(gap_surface_existing_relays);" in text,
            "\n    gap_surface_existing_relay_name_list!(@apply_relays);\n" not in text,
        ]
    )


def splice_into(text: str) -> str:
    region_start = re.search(r"//#region[^\n]*schema_gap_surfaces", text).start()
    region_end = re.search(r"//#endregion[^\n]*schema_gap_surfaces", text).end()
    region_header = text[region_start : text.index("\n", region_start) + 1]
    region_footer = text[text.rfind("//#endregion", region_start, region_end) : region_end]
    replacement = f"{region_header}\n{ns['new_mod']}\n{region_footer}"
    return text[:region_start] + replacement + text[region_end:]


cur = lib_path.read_text(encoding="utf-8")
cur = splice_into(cur)
if not sanity_ok(cur):
    raise SystemExit("splice produced invalid gap macros")
lib_path.write_text(cur, encoding="utf-8")
print("spliced schema_gap_surfaces into current lib.rs", len(cur.splitlines()), "lines")

for label, cmd, cwd in [
    ("wasm", ["bun", "scripts/build-wasm.script.mjs"], root / "compose/client/lib/rs"),
    ("sketchpad", ["bun", "nx", "run", "@compose/sketchpad:build"], root),
]:
    r = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, encoding="utf-8")
    if r.returncode != 0:
        err = (r.stderr or "") + (r.stdout or "")
        print(err[-12000:] if err else f"{label} failed")
        raise SystemExit(f"{label} failed")
    print(f"{label} ok")
