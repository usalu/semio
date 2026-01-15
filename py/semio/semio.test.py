# region Header

# py/semio/semio.test.py

# 2025 Ueli Saluz <ueli@semio-tech.com>

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Lesser General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Lesser General Public License for more details.

# You should have received a copy of the GNU Lesser General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# endregion Header

import json
import os
import math
import pytest
from semio import Kit, validateKit, flattenDesignDict, _applyDesignDiff, ValidationResult

TOLERANCE = 0.001
ASSETS_DIR = "../../assets/semio"

def load_kit(filename: str) -> dict:
    path = os.path.join(ASSETS_DIR, filename)
    if not os.path.exists(path):
        raise FileNotFoundError(f"Asset not found: {path}")
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)

def load_validation(filename: str) -> dict:
    path = os.path.join(ASSETS_DIR, filename)
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)

def is_close(a, b):
    return abs(a - b) < TOLERANCE

def vectors_equal(v1, v2):
    if v1 is None or v2 is None: return v1 == v2
    return is_close(v1.get("x",0), v2.get("x",0)) and \
           is_close(v1.get("y",0), v2.get("y",0)) and \
           is_close(v1.get("z",0), v2.get("z",0))

def planes_equal(p1, p2):
    if p1 is None or p2 is None: return p1 == p2
    return vectors_equal(p1.get("origin"), p2.get("origin")) and \
           vectors_equal(p1.get("xAxis"), p2.get("xAxis")) and \
           vectors_equal(p1.get("yAxis"), p2.get("yAxis"))

def centers_equal(c1, c2):
    if c1 is None or c2 is None: return c1 == c2
    return is_close(c1.get("u",0), c2.get("u",0)) and \
           is_close(c1.get("v",0), c2.get("v",0))

def find_design(kit: dict, name: str, parent_name: str = None) -> dict:
    parent_guid = None
    if parent_name:
        for d in kit.get("designs", []):
            if d.get("name") == parent_name:
                parent_guid = d.get("guid")
                break
        if not parent_guid:
            raise ValueError(f"Parent {parent_name} not found")
            
    for d in kit.get("designs", []):
        if d.get("name") == name:
            p = d.get("parent")
            if parent_guid:
                if p and p.get("guid") == parent_guid:
                    return d
            else:
                if not p:
                    return d
    raise ValueError(f"Design {name} not found")

def test_kit_serialization_roundtrip():
    kit_dict = load_kit("kit_metabolism.json")
    kit = Kit.parse(kit_dict)
    
    kit_output = kit.dump()
    kit_dict2 = kit_output.model_dump()
    
    kit2 = Kit.parse(kit_dict2)
    kit_output2 = kit2.dump()
    
    assert kit_output.model_dump() == kit_output2.model_dump()

def check_flatten(design_name: str, parent_name: str = None):
    kit_dict = load_kit("kit_metabolism.json")
    design = find_design(kit_dict, design_name, parent_name)
    
    expected_design = None
    design_guid = design.get("guid")
    for d in kit_dict.get("designs", []):
         if d.get("name") == "Flat":
             pg = d.get("parent", {}).get("guid")
             if pg == design_guid:
                 expected_design = d
                 break
    assert expected_design is not None, f"Expected Flat design for {design_name} not found"
    
    diff = flattenDesignDict(kit_dict, design_guid)
    _applyDesignDiff(design, diff)
    
    pieces = design.get("pieces", [])
    expected_pieces = expected_design.get("pieces", [])
    
    for p in pieces:
        ep = next((x for x in expected_pieces if x.get("name") == p.get("name")), None)
        assert ep is not None, f"Piece {p.get('name')} not found in expected design"
        
        assert planes_equal(p.get("plane"), ep.get("plane")), f"Plane mismatch for {p.get('name')}"
        if p.get("center") or ep.get("center"):
             assert centers_equal(p.get("center"), ep.get("center")), f"Center mismatch for {p.get('name')}"

def test_flatten_nakagin_capsule_tower():
    check_flatten("Nakagin Capsule Tower")

def test_flatten_nakagin_capsule_tower_slanted():
    check_flatten("Slanted", "Nakagin Capsule Tower")

def test_flatten_nakagin_capsule_tower_twisted():
    check_flatten("Twisted", "Nakagin Capsule Tower")

def test_flatten_nakagin_capsule_tower_dancing():
    check_flatten("Dancing", "Nakagin Capsule Tower")

def test_flatten_capsule_dream():
    check_flatten("Capsule Dream")

def test_validation_metabolism():
    kit_dict = load_kit("kit_metabolism.json")
    kit = Kit.model_validate(kit_dict)
    res = validateKit(kit)
    assert len(res.problems) == 0

def test_validation_invalid_kit():
    kit_dict = load_kit("kit_invalid.json")
    kit = Kit.model_validate(kit_dict)
    res = validateKit(kit)
    
    expected_dict = load_validation("validation.json")
    assert len(res.problems) == len(expected_dict.get("problems", []))
