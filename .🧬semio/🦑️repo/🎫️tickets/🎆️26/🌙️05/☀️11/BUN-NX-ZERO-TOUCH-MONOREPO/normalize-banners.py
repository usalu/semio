#!/usr/bin/env python3
"""🧩️ Idempotent banner + scope reference tweaks for compose/graphql/target.schema.graphql."""

from __future__ import annotations

import importlib.util
from pathlib import Path


def _load_schema_cleanup():
    here = Path(__file__).resolve().parent
    path = here / "schema_cleanup.py"
    spec = importlib.util.spec_from_file_location("schema_cleanup", path)
    if spec is None or spec.loader is None:
        raise SystemExit("schema_cleanup.py not found beside normalize-banners.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def main() -> None:
    sc = _load_schema_cleanup()
    text = sc.SCHEMA.read_text(encoding="utf-8")
    text = sc.ensure_diff_banners(text)
    text = sc.narrow_placeholder_operation_scopes(text)
    sc.SCHEMA.write_text(text, encoding="utf-8")
    print("OK", sc.SCHEMA)


if __name__ == "__main__":
    main()
