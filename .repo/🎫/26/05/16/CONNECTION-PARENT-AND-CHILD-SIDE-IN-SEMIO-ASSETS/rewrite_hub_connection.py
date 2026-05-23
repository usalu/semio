"""Rewrite hub connection column names and JSON keys."""
from pathlib import Path

P = Path(r"c:\git\semio\semio\server\hub\bin.rs")
t = P.read_text(encoding="utf-8")
pairs = [
    ("connected_piece_id", "parent_piece_id"),
    ("connected_design_piece_id", "parent_design_piece_id"),
    ("connected_connector_id", "parent_connector_id"),
    ("connecting_piece_id", "child_piece_id"),
    ("connecting_design_piece_id", "child_design_piece_id"),
    ("connecting_connector_id", "child_connector_id"),
    ('"connected":', '"parent":'),
    ('"connecting":', '"child":'),
    ("c.get(\"connected\")", "c.get(\"parent\")"),
    ("c.get(\"connecting\")", "c.get(\"child\")"),
    ("\"connected_piece_id\"", "\"parent_piece_id\""),
    ("\"connecting_piece_id\"", "\"child_piece_id\""),
]
for a, b in pairs:
    t = t.replace(a, b)
P.write_text(t, encoding="utf-8", newline="\n")
