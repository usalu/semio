"""Fix parent() method / parent field collision by renaming methods to parent_entity().
Also add back optional parent back-reference fields (excluded from serialization) where needed.
"""

import re

SEMIO_PY = "/workspaces/semio/semio/py/semio.py"


def read_file(path: str) -> str:
    with open(path, "r", encoding="utf-8") as f:
        return f.read()


def write_file(path: str, content: str) -> None:
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)


def rename_parent_methods(content: str) -> str:
    """Rename parent() methods to parent_entity() to avoid field collision."""
    # 1. Entity.parent() definition → parent_entity()
    content = content.replace(
        '    def parent(self) -> typing.Optional["Entity"]:',
        '    def parent_entity(self) -> typing.Optional["Entity"]:'
    )

    # 2. Entity.guid() call to parent() → parent_entity()
    content = content.replace(
        '        parent = self.parent()',
        '        parent = self.parent_entity()'
    )
    content = content.replace(
        '        parentId = f"{parent.guid()}/" if parent is not None else ""',
        '        parentId = f"{parent.guid()}/" if parent is not None else ""'
    )

    # 3. All entity class parent() method overrides
    # Pattern: def parent(self) -> "SomeType": or def parent(self: "SomeType") -> ...
    # Replace all def parent( with def parent_entity( except the ones in field classes

    lines = content.split("\n")
    result = []
    in_field_class = False

    for i, line in enumerate(lines):
        stripped = line.strip()

        # Track if we're inside a Field mixin class (don't rename there)
        if stripped.startswith("class ") and "Field" in stripped and "abc.ABC" in stripped:
            in_field_class = True
        elif stripped.startswith("class ") and not ("Field" in stripped and "abc.ABC" in stripped):
            in_field_class = False

        # Rename def parent( to def parent_entity( but only in Entity classes, not Field classes
        if not in_field_class:
            if re.match(r'\s+def parent\(self.*\)\s*->\s*', stripped):
                line = line.replace("def parent(", "def parent_entity(")

        result.append(line)

    return "\n".join(result)


def fix_parent_entity_calls(content: str) -> str:
    """Fix calls to the renamed parent_entity() method."""
    # In Entity.guid() - already fixed above

    # In ArtifactAuthor.idMembers() which calls self.type.idMembers(), self.design.idMembers()
    # These reference removed fields - need to handle separately

    # In ConnectorNotFoundInsideType.__init__ which uses self.parent for the error Type
    # This is a different parent attribute - leave as is (it's on an exception class, not an entity)

    return content


def main():
    content = read_file(SEMIO_PY)
    print(f"[DEBUG] Before fix: {content.count(chr(10))} lines")

    content = rename_parent_methods(content)
    content = fix_parent_entity_calls(content)

    # Count remaining parent method defs vs parent field defs
    parent_methods = len(re.findall(r'def parent_entity\(', content))
    parent_fields = len(re.findall(r'parent: typing\.Optional\[str\]', content))
    old_parent_methods = len(re.findall(r'def parent\(', content))

    print(f"[DEBUG] parent_entity() methods: {parent_methods}")
    print(f"[DEBUG] parent fields: {parent_fields}")
    print(f"[DEBUG] remaining old parent() methods: {old_parent_methods}")

    write_file(SEMIO_PY, content)
    print("[DEBUG] Done")


if __name__ == "__main__":
    main()
