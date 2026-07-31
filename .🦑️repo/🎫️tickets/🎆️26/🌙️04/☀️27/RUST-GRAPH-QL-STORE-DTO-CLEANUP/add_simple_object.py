"""Add async_graphql::SimpleObject to #[derive(...)] before pub struct *Dto."""
from __future__ import annotations

import pathlib
import re

LIB = pathlib.Path(__file__).resolve().parents[6] / "compose" / "rs" / "lib.rs"


def main() -> None:
    text = LIB.read_text(encoding="utf-8")

    def repl(m: re.Match[str]) -> str:
        inner = m.group(1)
        rest = m.group(2)
        if "SimpleObject" in inner:
            return m.group(0)
        if "Serialize" not in inner or "Deserialize" not in inner:
            return m.group(0)
        return f"#[derive({inner}, async_graphql::SimpleObject)]{rest}"

    new = re.sub(
        r"#\[derive\(([^\]]+)\)\](\s*\n\s*pub(?:\([^)]*\))?\s+struct\s+\w+Dto\b)",
        repl,
        text,
    )
    if new == text:
        print("no changes")
    else:
        LIB.write_text(new, encoding="utf-8")
        print("updated", LIB)


if __name__ == "__main__":
    main()
