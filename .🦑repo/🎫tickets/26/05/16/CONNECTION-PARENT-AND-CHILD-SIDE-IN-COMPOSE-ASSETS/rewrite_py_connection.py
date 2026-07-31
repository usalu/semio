"""Rename Connection connected/connecting to parent/child in compose Python main."""
from __future__ import annotations

import re
from pathlib import Path

P = Path(r"c:\git\compose\compose\client\lib\py\main.py")


def main() -> None:
    t = P.read_text(encoding="utf-8")
    sql = [
        ("connected_side_id", "parent_side_id"),
        ("connected_piece_id", "parent_piece_id"),
        ("connected_port_id", "parent_port_id"),
        ("connected_design_piece_id", "parent_design_piece_id"),
        ("connecting_side_id", "child_side_id"),
        ("connecting_piece_id", "child_piece_id"),
        ("connecting_port_id", "child_port_id"),
        ("connecting_design_piece_id", "child_design_piece_id"),
    ]
    for a, b in sql:
        t = t.replace(a, b)
    t = t.replace('["connected"]', '["parent"]').replace('["connecting"]', '["child"]')
    t = t.replace("['connected']", "['parent']").replace("['connecting']", "['child']")
    t = t.replace('entity["connected"]', 'entity["parent"]').replace('entity["connecting"]', 'entity["child"]')
    t = re.sub(r"^(\s+)connected: Side", r"\1parent: Side", t, flags=re.M)
    t = re.sub(r"^(\s+)connecting: Side", r"\1child: Side", t, flags=re.M)
    t = re.sub(r"^(\s+)connected: SideInput", r"\1parent: SideInput", t, flags=re.M)
    t = re.sub(r"^(\s+)connecting: SideInput", r"\1child: SideInput", t, flags=re.M)
    t = re.sub(r"^(\s+)connected: SideContext", r"\1parent: SideContext", t, flags=re.M)
    t = re.sub(r"^(\s+)connecting: SideContext", r"\1child: SideContext", t, flags=re.M)
    t = re.sub(r"^(\s+)connected: SideOutput", r"\1parent: SideOutput", t, flags=re.M)
    t = re.sub(r"^(\s+)connecting: SideOutput", r"\1child: SideOutput", t, flags=re.M)
    t = re.sub(r"^(\s+)connected: SidePrediction", r"\1parent: SidePrediction", t, flags=re.M)
    t = re.sub(r"^(\s+)connecting: SidePrediction", r"\1child: SidePrediction", t, flags=re.M)
    t = re.sub(r"\bdef connected\(", "def parent(", t)
    t = re.sub(r"\bdef connecting\(", "def child(", t)
    t = re.sub(r"\.connected\b", ".parent", t)
    t = re.sub(r"\.connecting\b", ".child", t)
    P.write_text(t, encoding="utf-8", newline="\n")


if __name__ == "__main__":
    main()
