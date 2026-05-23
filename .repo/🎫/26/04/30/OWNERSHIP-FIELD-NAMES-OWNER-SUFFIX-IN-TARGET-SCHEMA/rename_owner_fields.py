"""One-off: ownerFoo -> fooOwner in target.schema.graphql (owner suffix second)."""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]  # .../.repo/🎫/…/ticket → repo root
PATH = ROOT / "semio" / "graphql" / "target.schema.graphql"

PAT = re.compile(r"\bowner([A-Z][a-zA-Z0-9]*)\b")


def to_suffix_owner(rest: str) -> str:
    return rest[0].lower() + rest[1:] + "Owner"


def main() -> None:
    text = PATH.read_text(encoding="utf-8")
    new_text, n = PAT.subn(lambda m: to_suffix_owner(m.group(1)), text)
    PATH.write_text(new_text, encoding="utf-8")
    print(f"wrote {PATH} ({n} replacements)")


if __name__ == "__main__":
    main()
