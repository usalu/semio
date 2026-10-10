"""Copies the schema-neutral measuring helpers of the IFC 2x3 oracle case verbatim into the IFC4 case so the new case stands alone.

Usage: python r12-w2-wp17-ifc4-oracle-extract.py <old 🐍️.py> <output prefix file>
"""

import ast
import sys
from pathlib import Path

FUNCTIONS = [
    "identity", "settings", "kernel_volume", "written_volume", "straight", "annotation_rows", "system_key", "item_reference", "type_property_rows", "kernel_bounds",
    "placement_problems", "authored", "unescape", "definition_fields", "template_kind", "format_number", "library_table", "library_problems", "written_value", "same_value", "property_problems", "quantities_of", "close", "keyed",
]
CONSTANTS = ["TOLERANCE", "NEWLINE", "MEASURED", "ENTITIES", "MEASURES", "DEFINITION_KEYS", "LIBRARY_COUNTED", "TYPE_NAMES", "TYPE_COLLECTIONS", "EMPTY_COLLECTIONS"]


def main(old, out):
    source = Path(old).read_text(encoding="utf-8")
    tree = ast.parse(source)
    parts = []
    for node in tree.body:
        if isinstance(node, ast.FunctionDef) and node.name in FUNCTIONS:
            parts.append(ast.get_source_segment(source, node))
        elif isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id in CONSTANTS for target in node.targets):
            parts.append(ast.get_source_segment(source, node))
    Path(out).write_text("\n\n\n".join(parts) + "\n", encoding="utf-8")
    print("copied", len(parts), "definitions")


main(sys.argv[1], sys.argv[2])
