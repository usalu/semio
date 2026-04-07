"""Transform semio.py: remove sqlmodel/SQLAlchemy ORM, use pure Pydantic.

This script reads semio.py and applies systematic transformations:
1. Change SModel base from sqlmodel.SQLModel to pydantic.BaseModel
2. Replace sqlmodel.Field with pydantic.Field (for non-ORM fields)
3. Remove pk/FK fields (sa_column fields)
4. Remove sqlmodel.Relationship lines (back-references)
5. Convert child Relationship fields to pydantic.Field(default_factory=list)
6. Remove table=True from class declarations
7. Remove __table_args__ blocks
8. Update imports
"""

import re
import sys

SEMIO_PY = "/workspaces/semio/semio/py/semio.py"


def read_file(path: str) -> str:
    with open(path, "r", encoding="utf-8") as f:
        return f.read()


def write_file(path: str, content: str) -> None:
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)


def transform(content: str) -> str:
    lines = content.split("\n")
    result = []
    skip_until_paren_close = False
    paren_depth = 0
    skip_table_args = False
    table_args_depth = 0
    i = 0

    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # Skip __table_args__ blocks (multi-line tuples)
        if skip_table_args:
            # Count parens to find end of tuple
            table_args_depth += stripped.count("(") - stripped.count(")")
            if table_args_depth <= 0:
                skip_table_args = False
            i += 1
            continue

        if stripped.startswith("__table_args__"):
            skip_table_args = True
            table_args_depth = stripped.count("(") - stripped.count(")")
            if table_args_depth <= 0:
                skip_table_args = False
            i += 1
            continue

        # Skip pk fields (primary key): detect sa_column with primary_key=True
        # These span multiple lines typically
        if skip_until_paren_close:
            paren_depth += stripped.count("(") - stripped.count(")")
            if paren_depth <= 0:
                skip_until_paren_close = False
            i += 1
            continue

        # Detect pk/FK field declarations (multi-line sqlmodel.Field with sa_column)
        if re.match(r'\s+\w+:\s+typing\.Optional\[int\]\s*=\s*sqlmodel\.Field\(', stripped) or \
           re.match(r'\s+\w+Pk:\s+typing\.Optional\[int\]\s*=\s*sqlmodel\.Field\(', line.strip()):
            # Check if this is a sa_column field (pk or FK)
            # Look ahead to see if sa_column is in this or next lines
            lookahead = stripped
            la_idx = i + 1
            temp_depth = stripped.count("(") - stripped.count(")")
            while temp_depth > 0 and la_idx < len(lines):
                lookahead += " " + lines[la_idx].strip()
                temp_depth += lines[la_idx].strip().count("(") - lines[la_idx].strip().count(")")
                la_idx += 1

            if "sa_column" in lookahead:
                # This is a pk/FK field - skip it
                skip_until_paren_close = True
                paren_depth = stripped.count("(") - stripped.count(")")
                if paren_depth <= 0:
                    skip_until_paren_close = False
                i += 1
                continue

        # Also detect single-line pk fields
        if re.match(r'\s+pk:\s+typing\.Optional\[int\]\s*=\s*sqlmodel\.Field\(', stripped):
            if "sa_column" in stripped:
                paren_depth = stripped.count("(") - stripped.count(")")
                if paren_depth > 0:
                    skip_until_paren_close = True
                i += 1
                continue

        # Remove sqlmodel.Relationship lines that are back-references (single parent refs)
        # Pattern: varname: "Model" = sqlmodel.Relationship(back_populates="...")
        if "sqlmodel.Relationship(" in stripped and not stripped.startswith("#"):
            # Check if it's a list relationship (child collection) or single (back-ref)
            # List relationships: list[X] = sqlmodel.Relationship(...)
            # Single back-refs: "Model" = sqlmodel.Relationship(...)
            field_match = re.match(r'(\s*)(\w+):\s*(.*?)\s*=\s*sqlmodel\.Relationship\((.*)', line)
            if field_match:
                indent = field_match.group(1)
                fname = field_match.group(2)
                ftype = field_match.group(3).strip()

                # Gather full line if multi-line
                full_line = line
                temp_depth = stripped.count("(") - stripped.count(")")
                la_idx = i + 1
                while temp_depth > 0 and la_idx < len(lines):
                    full_line += "\n" + lines[la_idx]
                    temp_depth += lines[la_idx].strip().count("(") - lines[la_idx].strip().count(")")
                    la_idx += 1

                is_list = "list[" in ftype or "List[" in ftype

                if is_list:
                    # Child collection - convert to pydantic.Field
                    result.append(f"{indent}{fname}: {ftype} = pydantic.Field(default_factory=list)")
                else:
                    # Back-reference - remove entirely
                    pass

                # Skip all lines of this field
                temp_depth = stripped.count("(") - stripped.count(")")
                while temp_depth > 0:
                    i += 1
                    if i < len(lines):
                        temp_depth += lines[i].strip().count("(") - lines[i].strip().count(")")
                i += 1
                continue

        # Remove __tablename__ lines
        if stripped.startswith("__tablename__"):
            i += 1
            continue

        # Transform the line
        transformed = line

        # Replace sqlmodel.SQLModel with pydantic.BaseModel in class declarations
        transformed = transformed.replace("sqlmodel.SQLModel, abc.ABC", "pydantic.BaseModel, abc.ABC")
        transformed = transformed.replace("sqlmodel.SQLModel", "pydantic.BaseModel")

        # Remove table=True from class declarations
        # Handle both ", table=True," and "table=True,"
        transformed = re.sub(r',?\s*table\s*=\s*True\s*,?', '', transformed)
        # Clean up trailing comma before closing paren in class def
        transformed = re.sub(r',\s*\):', '):', transformed)

        # Replace sqlmodel.Field with pydantic.Field
        transformed = transformed.replace("sqlmodel.Field(", "pydantic.Field(")

        # Replace sqlmodel.Column with sqlalchemy.Column (for any remaining usages)
        transformed = transformed.replace("sqlmodel.Column(", "sqlalchemy.Column(")

        result.append(transformed)
        i += 1

    return "\n".join(result)


def fix_imports(content: str) -> str:
    """Update imports: remove sqlmodel dependency, keep sqlalchemy for schema."""
    # Remove the sqlmodel monkey-patching block (Python 3.13 compat)
    # This is the block from "if sys.version_info >= (3, 13):" that patches sqlmodel
    lines = content.split("\n")
    result = []
    skip_block = False
    indent_level = 0

    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # Skip sqlmodel._compat patching blocks
        if "sqlmodel._compat" in stripped or "sqlmodel.main" in stripped:
            i += 1
            continue

        # Remove sqlmodel import line
        if stripped == "import sqlmodel":
            i += 1
            continue

        # Remove graphene_sqlalchemy import
        if stripped == "import graphene_sqlalchemy":
            i += 1
            continue

        result.append(line)
        i += 1

    return "\n".join(result)


def fix_semio_table(content: str) -> str:
    """Fix the Semio metadata table class to be a simple marker."""
    # The Semio class was: class Semio(sqlmodel.SQLModel, table=True)
    # After transform it becomes: class Semio(pydantic.BaseModel)
    # We need it as a simple dataclass or remove it
    content = content.replace(
        'class Semio(pydantic.BaseModel):\n    """Metadata table',
        'class Semio(pydantic.BaseModel):\n    """Metadata marker'
    )
    return content


def main():
    content = read_file(SEMIO_PY)
    print(f"[DEBUG] Original file: {len(content)} chars, {content.count(chr(10))} lines")

    content = transform(content)
    print(f"[DEBUG] After transform: {len(content)} chars, {content.count(chr(10))} lines")

    content = fix_imports(content)
    print(f"[DEBUG] After fix_imports: {len(content)} chars, {content.count(chr(10))} lines")

    content = fix_semio_table(content)

    write_file(SEMIO_PY, content)
    print("[DEBUG] Done writing transformed semio.py")

    # Verify no sqlmodel.Field or sqlmodel.Relationship remain
    remaining_field = content.count("sqlmodel.Field")
    remaining_rel = content.count("sqlmodel.Relationship")
    remaining_table = content.count("table=True")
    print(f"[DEBUG] Remaining sqlmodel.Field: {remaining_field}")
    print(f"[DEBUG] Remaining sqlmodel.Relationship: {remaining_rel}")
    print(f"[DEBUG] Remaining table=True: {remaining_table}")


if __name__ == "__main__":
    main()
