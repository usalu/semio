#!/usr/bin/env python3
"""Re-apply glue/io/builder after git facet restore (no absorb)."""
from __future__ import annotations

import importlib.util
import json
import subprocess
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("w6m", TICKET / "generators/w6_batch1c_migrate.py")
w6 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(w6)  # type: ignore

BATCH = json.loads((TICKET / "generators/w6-batch1c.json").read_text(encoding="utf-8"))
ROOT = w6.ROOT


def main() -> None:
    checks = {}
    for entry in BATCH:
        row = w6.owner_row(entry["plugin"], entry["artifact"])
        art = ROOT / row["path"]
        plugin_path = art.parent.parent
        slugs = row.get("import") or row.get("stdio_artifacts") or []
        snap, mut, diff, schema_const = w6.sniff_types(art, entry["rust_mod"])
        w6.write_builder(art, entry["rust_mod"], snap, mut, diff)
        w6.write_io(art, entry["rust_mod"], snap, schema_const, slugs)
        w6.patch_glue(plugin_path, entry["artifact"], entry["rust_mod"], slugs, entry.get("extras") or [], art)
        print("repatch", entry["crate"])
    for entry in BATCH:
        ok, _ = w6.cargo_check(entry["crate"])
        checks[entry["crate"]] = ok
    print(checks)
    sys.exit(0 if all(checks.values()) else 1)


if __name__ == "__main__":
    main()
