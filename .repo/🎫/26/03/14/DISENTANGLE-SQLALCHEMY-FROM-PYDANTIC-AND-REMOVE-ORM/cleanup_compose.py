"""Second pass: remove all remaining sa_column fields and clean up empty class declarations."""

import re

COMPOSE_PY = "/workspaces/semio/compose/py/compose.py"


def read_file(path: str) -> str:
    with open(path, "r", encoding="utf-8") as f:
        return f.read()


def write_file(path: str, content: str) -> None:
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)


def remove_sa_column_fields(content: str) -> str:
    """Remove all field declarations that contain sa_column."""
    lines = content.split("\n")
    result = []
    i = 0
    removed_count = 0

    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # Detect field with sa_column (multi-line)
        if "pydantic.Field(" in stripped and i + 1 < len(lines):
            # Look ahead to check for sa_column
            lookahead_lines = [line]
            depth = stripped.count("(") - stripped.count(")")
            la = i + 1
            while depth > 0 and la < len(lines):
                lookahead_lines.append(lines[la])
                depth += lines[la].strip().count("(") - lines[la].strip().count(")")
                la += 1

            full_text = "\n".join(lookahead_lines)
            if "sa_column" in full_text:
                removed_count += 1
                i = la
                continue

        # Also catch single-line sa_column fields
        if "sa_column" in stripped and "pydantic.Field" in stripped:
            depth = stripped.count("(") - stripped.count(")")
            while depth > 0 and i + 1 < len(lines):
                i += 1
                depth += lines[i].strip().count("(") - lines[i].strip().count(")")
            i += 1
            removed_count += 1
            continue

        result.append(line)
        i += 1

    print(f"[DEBUG] Removed {removed_count} sa_column field declarations")
    return "\n".join(result)


def clean_empty_class_declarations(content: str) -> str:
    """Clean up empty lines in class declarations like 'TableEntity,\\n\\n):'"""
    # Fix class declarations with trailing empty line before ):
    content = re.sub(r'(TableEntity,)\s*\n\s*\n\s*\):', r'\1\n):', content)
    content = re.sub(r'(Table,)\s*\n\s*\n\s*\):', r'\1\n):', content)
    # Also handle cases like ", \n):" with just whitespace
    content = re.sub(r',\s*\n\s*\):', r',\n):', content)
    return content


def remove_sqlalchemy_imports(content: str) -> str:
    """Remove sqlalchemy imports since we no longer use them in compose.py."""
    lines = content.split("\n")
    result = []
    for line in lines:
        stripped = line.strip()
        if stripped in ("import sqlalchemy", "import sqlalchemy.orm"):
            continue
        result.append(line)
    return "\n".join(result)


def main():
    content = read_file(COMPOSE_PY)
    print(f"[DEBUG] Before cleanup: {content.count(chr(10))} lines")

    content = remove_sa_column_fields(content)
    print(f"[DEBUG] After removing sa_column fields: {content.count(chr(10))} lines")

    # Check for remaining sa_column references
    remaining = content.count("sa_column")
    print(f"[DEBUG] Remaining sa_column references: {remaining}")

    content = clean_empty_class_declarations(content)
    content = remove_sqlalchemy_imports(content)

    write_file(COMPOSE_PY, content)
    print(f"[DEBUG] Final: {content.count(chr(10))} lines")


if __name__ == "__main__":
    main()
