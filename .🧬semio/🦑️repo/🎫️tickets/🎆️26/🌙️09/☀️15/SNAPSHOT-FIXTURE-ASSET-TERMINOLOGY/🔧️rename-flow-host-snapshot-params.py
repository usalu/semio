#!/usr/bin/env python3
"""Rename FlowHostSnapshot parameter `fixture` → `host_snapshot` in Rust sources."""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
SCAN = [ROOT / "✏️s/🔌️plugins", ROOT / "🧰️framework"]
SKIP_PARTS = ("🧫️fixtures/", "import-document", "export-document")

PARAM = re.compile(r"\bfixture:\s*&(?:mut\s+)?(?:[\w:]+::)?FlowHostSnapshot\b")
PARAM_OWNED = re.compile(r"\bfixture:\s*(?:[\w:]+::)?FlowHostSnapshot\b")
PARAM_MUT = re.compile(r"\bmut\s+fixture:\s*(?:[\w:]+::)?FlowHostSnapshot\b")

BODY = [
    (re.compile(r"\bfixture\."), "host_snapshot."),
    (re.compile(r"\bfixture,"), "host_snapshot,"),
    (re.compile(r"\(fixture\)"), "(host_snapshot)"),
    (re.compile(r"&fixture\b"), "&host_snapshot"),
    (re.compile(r"\bfixture\b(?=\s*\))"), "host_snapshot"),
]


def should_skip(path: Path) -> bool:
    text = str(path)
    return any(part in text for part in SKIP_PARTS)


def rewrite(source: str) -> str:
    if "FlowHostSnapshot" not in source:
        return source
    if not PARAM.search(source) and not PARAM_OWNED.search(source) and not PARAM_MUT.search(source):
        return source
    out = PARAM.sub(lambda m: re.sub(r"\bfixture:", "host_snapshot:", m.group(0), count=1), source)
    out = PARAM_OWNED.sub(lambda m: re.sub(r"\bfixture:", "host_snapshot:", m.group(0), count=1), out)
    out = PARAM_MUT.sub(lambda m: re.sub(r"\bfixture:", "host_snapshot:", m.group(0), count=1), out)
    if "host_snapshot: &FlowHostSnapshot" in out or "host_snapshot: FlowHostSnapshot" in out:
        for pattern, repl in BODY:
            out = pattern.sub(repl, out)
    return out


def main() -> None:
    changed: list[Path] = []
    for base in SCAN:
        if not base.is_dir():
            continue
        for path in base.rglob("*.rs"):
            if should_skip(path):
                continue
            source = path.read_text(encoding="utf-8")
            updated = rewrite(source)
            if updated != source:
                path.write_text(updated, encoding="utf-8")
                changed.append(path)
    print(f"updated {len(changed)} files")


if __name__ == "__main__":
    main()
