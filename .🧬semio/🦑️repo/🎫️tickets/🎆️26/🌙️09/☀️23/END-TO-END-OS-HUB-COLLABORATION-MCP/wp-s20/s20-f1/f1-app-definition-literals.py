"""🧯️ S20 F1 (faults overlay): every `AppDefinition { … }` struct literal without a `..rest` gains `faults: Vec::new(),`
(the new manifest field). Skips fn bodies (`-> AppDefinition {`), struct/impl headers and patterns. Idempotent.
Usage: python3 f1-app-definition-literals.py <root> [--dry-run]"""
import re
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
DRY = "--dry-run" in sys.argv
LITERAL = re.compile(r"\b(?:[A-Za-z_][A-Za-z0-9_]*::)*AppDefinition \{\n")
SKIP_DIRS = {"target", "node_modules", ".git", "🗑️generated"}


def matching(text: str, start: int) -> int:
    depth, i, in_string = 1, start, False
    while i < len(text) and depth:
        character = text[i]
        if in_string:
            if character == "\\":
                i += 2
                continue
            if character == '"':
                in_string = False
        elif character == '"':
            in_string = True
        elif character == "{":
            depth += 1
        elif character == "}":
            depth -= 1
        i += 1
    return i - 1


def rewrite(text: str) -> tuple[str, int]:
    edits: list[tuple[int, str]] = []
    for match in LITERAL.finditer(text):
        line_start = text.rfind("\n", 0, match.start()) + 1
        prefix = text[line_start:match.start()]
        if "->" in prefix or "struct " in prefix or "impl" in prefix or "enum " in prefix or prefix.strip().startswith("//"):
            continue
        close = matching(text, match.end())
        body = text[match.end():close]
        if re.search(r"(^|\n)\s*\.\.", body) or re.search(r"\n\s*faults\s*:", body) or "=>" in text[close:close + 6]:
            continue
        first_field = re.search(r"\n?([ \t]+)[a-z_]+\s*[:,]", body)
        indent = first_field.group(1) if first_field else "    "
        closing_line_start = text.rfind("\n", 0, close) + 1
        edits.append((closing_line_start, f"{indent}faults: Vec::new(),\n"))
    for position, insert in sorted(edits, reverse=True):
        text = text[:position] + insert + text[position:]
    return text, len(edits)


def main() -> None:
    total = files = 0
    for path in ROOT.rglob("*.rs"):
        if SKIP_DIRS.intersection(path.relative_to(ROOT).parts):
            continue
        text = path.read_text(errors="replace")
        if "AppDefinition {" not in text:
            continue
        new, count = rewrite(text)
        if count:
            total += count
            files += 1
            if not DRY:
                path.write_text(new)
    print(f"{'dry-run' if DRY else 'applied'}: {total} literals in {files} files")


if __name__ == "__main__":
    main()
