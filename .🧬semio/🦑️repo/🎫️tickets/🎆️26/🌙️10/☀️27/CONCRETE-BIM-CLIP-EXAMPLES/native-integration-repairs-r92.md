# Native integration repairs R92

The live diagnostic handle85383 continues without restart. Fresh two-method execution12889 reproduced missing declared axis KeyError in native tapered and section-extruded sanitary composites. Neutral section fixture now includes inferred native bounds and explicit authored axis cases, plus the FLOORING discriminator. Native representation derives missing centers from exact part vertices; authored centers and native path points remain authoritative. FLOORING uses architectural solid quantities and representation.

The cellar containment oracle now prohibits direct opening containment, requires one exact voided host and its storey containment, and preserves product type/material/model membership assertions. The curtain AST inspects the uncached native building producer. Its independent depth fixture explicitly binds its own0.04m frame depth, removing accidental dependence on a changing source type.

The native furnishing source roster previously targeted FIX-LAUNDRY and FIX-GUEST-BASIN, which are now correctly IfcSanitaryTerminal. The furnishing test now targets the three existing hollow cabinet fixtures and reuses the registered storage cavity fixture. It verifies independent GEOS occupied volume, material/role sets, empty cavities, bilingual native furnishing identity, material/quantity relations and one native extrusion per actual part. No historical fixed part count or obsolete source volume is retained. Sanitary sources retain their dedicated native tests.

Focused validation is pending repository capacity admission.

## Further live diagnostic findings

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.📐️representations.test_sections_and_elevations_use_actual_xz_solids

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\📐️representations.py", line 111, in test_sections_and_elevations_use_actual_xz_solids
    assert section["derived"]["projection"] == {"horizontal": "x", "vertical": "z", "view": "south"}
           ~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^
KeyError: 'projection'
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.📐️representations.test_sections_preserve_real_cavities

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\📐️representations.py", line 134, in test_sections_preserve_real_cavities
    assert occupied.covers(Point(material)), case["tag"]
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
AssertionError: FIX-HALL-BASIN
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️architecture.test_guest_floor_finish_zones_follow_the_integrated_wardrobe

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🏛️architecture.py", line 55, in test_guest_floor_finish_zones_follow_the_integrated_wardrobe
    assert math.isclose(a.bounds[1], north_edge)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
AssertionError
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️architecture.test_kitchen_patio_has_empty_facade_frame_instead_of_false_glass

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🏛️architecture.py", line 124, in test_kitchen_patio_has_empty_facade_frame_instead_of_false_glass
    assert len([o for o in wall["openings"] if o["fill_class"] == "IfcWindow"]) == 2
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
AssertionError
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️architecture.test_kitchen_patio_is_a_real_open_air_room_and_roof_void

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🏛️architecture.py", line 100, in test_kitchen_patio_is_a_real_open_air_room_and_roof_void
    assert Polygon(void).equals(Polygon(patio["outline"]))
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
AssertionError
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️architecture.test_office_access_is_clear_between_hall_and_kitchen

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🏛️architecture.py", line 113, in test_office_access_is_clear_between_hall_and_kitchen
    assert all(route.intersection(Polygon(p["outline"])).area < 1e-8 for p in cupboard["parts"])
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
AssertionError
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️suite.test_parent_room_contains_full_registered_alcove_bay

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🏛️suite.py", line 35, in test_parent_room_contains_full_registered_alcove_bay
    assert room.buffer(1e-8).covers(bay)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
AssertionError
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️suite.test_son_public_route_does_not_cross_parents_suite

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🏛️suite.py", line 58, in test_son_public_route_does_not_cross_parents_suite
    assert permitted.buffer(1e-8).covers(passage)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
AssertionError
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🧱️walls.test_cavity_wall_parts_preserve_opening_and_exclude_air

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🧱️walls.py", line 33, in test_cavity_wall_parts_preserve_opening_and_exclude_air
    from shapely.ops import unary_union
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 775, in _wall_geometry
    out["layers"], out["parts"], out["joint_displacement_volume"], out["solid_volume"] = _wall_parts(model, wall, out)
                                                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 891, in _wall_parts
    owned,displaced,volume = _wall_owned_parts(model,parts)
                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 819, in _wall_owned_parts
    cutters = [c for c in _wall_cutters(model) if c["z0"] < zmax-1e-10 and c["z1"] > zmin+1e-10
                          ^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 798, in _wall_cutters
    owned = _filling_geometry(model,obj)["parts"]
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 1070, in _filling_geometry
    raise ValueError("Sash members consume their panel")
ValueError: Sash members consume their panel
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🧱️walls.test_step_wall_extrusions_use_occupied_leaves

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🧱️walls.py", line 70, in test_step_wall_extrusions_use_occupied_leaves
    source,wall,_=_fixture()
             ^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 1819, in building_geometry
    return _geometry_query("building", None, model)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 1796, in _geometry_query
    value = _building_geometry_uncached(owned)
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 1744, in _building_geometry_uncached
    wall_geometries = [_wall_geometry(m,w) for w in _by_class(m,"IfcWall")]
                       ^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 756, in _wall_geometry
    return path_wall_geometry(model,wall)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 731, in path_wall_geometry
    body,displaced,body_volume = _wall_owned_parts(model,[p for p in parts if p["role"]=="body"])
                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 819, in _wall_owned_parts
    cutters = [c for c in _wall_cutters(model) if c["z0"] < zmax-1e-10 and c["z1"] > zmin+1e-10
                          ^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 798, in _wall_cutters
    owned = _filling_geometry(model,obj)["parts"]
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 1070, in _filling_geometry
    raise ValueError("Sash members consume their panel")
ValueError: Sash members consume their panel
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🧱️walls.test_vertical_material_regions_follow_opening_sill_and_keep_outer_plane

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🧱️walls.py", line 107, in test_vertical_material_regions_follow_opening_sill_and_keep_outer_plane
    return {"datum":datum,"offset":{"value":0,"unit":"m","basis":"assumed"},**({"opening":"OP-S"} if datum=="sill" else {})}
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 775, in _wall_geometry
    out["layers"], out["parts"], out["joint_displacement_volume"], out["solid_volume"] = _wall_parts(model, wall, out)
                                                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 891, in _wall_parts
    owned,displaced,volume = _wall_owned_parts(model,parts)
                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 819, in _wall_owned_parts
    cutters = [c for c in _wall_cutters(model) if c["z0"] < zmax-1e-10 and c["z1"] > zmin+1e-10
                          ^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 798, in _wall_cutters
    owned = _filling_geometry(model,obj)["parts"]
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 1070, in _filling_geometry
    raise ValueError("Sash members consume their panel")
ValueError: Sash members consume their panel
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🧱️walls.test_wall_axis_reversal_preserves_solid_ownership

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🧱️walls.py", line 48, in test_wall_axis_reversal_preserves_solid_ownership
    from shapely.ops import unary_union
          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 775, in _wall_geometry
    out["layers"], out["parts"], out["joint_displacement_volume"], out["solid_volume"] = _wall_parts(model, wall, out)
                                                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 891, in _wall_parts
    owned,displaced,volume = _wall_owned_parts(model,parts)
                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 819, in _wall_owned_parts
    cutters = [c for c in _wall_cutters(model) if c["z0"] < zmax-1e-10 and c["z1"] > zmin+1e-10
                          ^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 798, in _wall_cutters
    owned = _filling_geometry(model,obj)["parts"]
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 1070, in _filling_geometry
    raise ValueError("Sash members consume their panel")
ValueError: Sash members consume their panel
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🧱️walls.test_wall_body_uses_occupied_parts

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🧱️walls.py", line 61, in test_wall_body_uses_occupied_parts
    """🧊 IFC body prisms contain the same occupied volume as the material takeoff."""
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 775, in _wall_geometry
    out["layers"], out["parts"], out["joint_displacement_volume"], out["solid_volume"] = _wall_parts(model, wall, out)
                                                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 891, in _wall_parts
    owned,displaced,volume = _wall_owned_parts(model,parts)
                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 819, in _wall_owned_parts
    cutters = [c for c in _wall_cutters(model) if c["z0"] < zmax-1e-10 and c["z1"] > zmin+1e-10
                          ^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 798, in _wall_cutters
    owned = _filling_geometry(model,obj)["parts"]
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🐍️.py", line 1070, in _filling_geometry
    raise ValueError("Sash members consume their panel")
ValueError: Sash members consume their panel
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪟️fillings.test_material_relations_do_not_reassign_typed_fillings_to_concrete

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🪟️fillings.py", line 68, in test_material_relations_do_not_reassign_typed_fillings_to_concrete
    assert registry[tag].get("material")==relation["relating"]
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
AssertionError
```

Projection integration now includes explicit south XZ metadata on service representations. The independent representation oracle respects declared service section axes and Curve3D axes while retaining exact GEOS projection checks. Hall basin material probe is at the actual source ceramic floor z0.69–0.71 (probe0.70), replacing retired0.72; void probe remains unchanged.

At the latest poll the live diagnostic reached roof.test_mixed_filling_cells_conserve_independent_material_and_clear_leaf after roughly8700seconds. Its source-aware per-test JSONL is1,458,206,183bytes; use compact progress-current.json or bounded file reads rather than loading the complete stream. Focused queued jobs89123/97544 remain subject to repository machine capacity.

Focused section89123 passed native axis and all architectural floor representation assertions, then exposed a new test cleanup error at final schema validation: the FLOORING variant restored class but removed the original required sanitary predefined_type. Cleanup now restores the original predefined_type exactly; no source/schema relaxation. Fresh section rerun required.

## Ground diagnostic additions

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏠ground.test_ground_basin_registration_and_garage_portal_are_physically_consistent

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🏠ground.py", line 73, in test_ground_basin_registration_and_garage_portal_are_physically_consistent
    assert math.dist(centre,case["basin"]["centre"])<1e-8
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
AssertionError
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏠ground.test_ground_paths_match_independent_registered_controls

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🏠ground.py", line 15, in test_ground_paths_match_independent_registered_controls
    assert model._find(source,tag)["geometry"]["path"]==expected,tag
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
AssertionError: W-G-SERVICE
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏠ground.test_ground_registered_room_fittings_and_private_portals_remain_accessible

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🏠ground.py", line 88, in test_ground_registered_room_fittings_and_private_portals_remain_accessible
    spaces={s["tag"]:Polygon(s["outline"]) for s in source["spaces"] if s["storey"]=="ground"}
                     ^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\📦️packages\🐍️python\.venv\Lib\site-packages\shapely\geometry\polygon.py", line 237, in __new__
    shell = LinearRing(shell)
            ^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\📦️packages\🐍️python\.venv\Lib\site-packages\shapely\geometry\polygon.py", line 93, in __new__
    coordinates = np.array([_coords(o) for o in coordinates])
                            ^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\📦️packages\🐍️python\.venv\Lib\site-packages\shapely\geometry\polygon.py", line 91, in _coords
    return [float(c) for c in o]
            ^^^^^^^^
ValueError: could not convert string to float: 'bezier'
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏠ground.test_ground_spaces_are_valid_distinct_and_contain_registered_basin_and_hoist

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🏠ground.py", line 26, in test_ground_spaces_are_valid_distinct_and_contain_registered_basin_and_hoist
    spaces={s["tag"]:Polygon(s["outline"]) for s in source["spaces"] if s["storey"]=="ground"}
                     ^^^^^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\📦️packages\🐍️python\.venv\Lib\site-packages\shapely\geometry\polygon.py", line 237, in __new__
    shell = LinearRing(shell)
            ^^^^^^^^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\📦️packages\🐍️python\.venv\Lib\site-packages\shapely\geometry\polygon.py", line 93, in __new__
    coordinates = np.array([_coords(o) for o in coordinates])
                            ^^^^^^^^^^
  File "C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\📦️packages\🐍️python\.venv\Lib\site-packages\shapely\geometry\polygon.py", line 91, in _coords
    return [float(c) for c in o]
            ^^^^^^^^
ValueError: could not convert string to float: 'bezier'
```

## Focused repair input snapshot

Captured before queued fresh validation; source is still mutable and no final stable certificate is claimed.

```json
{
  "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🐍️.py": "7404871af67e6067cf7b7d233f2b4bc6217691e257ae3ca01f748ac2cf1ec8f8",
  "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🧊️sections.py": "6a260032f868ec6298c3e33d3a11938fdf00cc9dd7fc9b101e990bffc181765b",
  "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🧊️sections.json": "9ccbaefb667258844879c32e8205667d589c8dfc0772c1e3030bb2c119c64465",
  "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🧊️sections.schema.json": "bcb25d712ccd90dfd5d627c5e1c4d93dbec9a47f7b87f8b1c7a4c898ac53db9f",
  "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🏗️services.py": "2f3260a8eccca539ac6cc8e67743196e0ca143bf328376933d4739a788ab6fdd",
  "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🪟️curtains.py": "3bbec892effdb22a25463cbdb040858bf767c552f8447caa9f3f0a8f400e1631",
  "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔌️exchange/🧪️tests/🪑furnishings.py": "ee1e7c7fb3648fee45177da6bff30a9b58ffffc4173313030b9869a01cbc2047",
  "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔌️exchange/🧪️tests/🪑furnishings.json": "baf100fbc6c21183916e42cefe2b336c86cda0f7e875843136fcb4eacc391e26"
}
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏠ground.test_northern_circulation_passes_the_registered_basin_without_moving_source_structure

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🏠ground.py", line 197, in test_northern_circulation_passes_the_registered_basin_without_moving_source_structure
    assert hall.covers(route)
           ^^^^^^^^^^^^^^^^^^
AssertionError
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚬️chimneys.test_actual_production_catalogue_and_storey_projections_keep_spanning_chimney

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🚬️chimneys.py", line 81, in test_actual_production_catalogue_and_storey_projections_keep_spanning_chimney
    plan._plan_services(ctx)
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🏛️savoye", line 500, in _plan_services
AttributeError: 'types.SimpleNamespace' object has no attribute 'service_surface_color'
```

The live chimney projection test exposed an incomplete geometry-only test double after the palette interface extension: SimpleNamespace lacked service_surface_color. The existing independent frustum geometry harness now binds the actual pure surface_role/service_surface_role producers and its existing neutral color sink supplies service_surface_color. Independent GEOS/SciPy geometry assertions are unchanged; actual native palette/Pillow tests separately verify real colors. Fresh execution remains required.

## Fresh focused native result

Job97544 exited0: **11 tests PASS in56.146s** after approximately11minutes capacity admission wait. Selection: FrustumSolidContract.test_every_representation_preserves_the_taper; all NativeCurtainContract methods; CellarServicesContract.test_spatial_type_material_and_partial_model_ownership; all NativeFurnishingContract methods. This validates the axis repair, explicit curtain fixture/native producer AST, host-derived opening context, and current furnishing native material/cavity/quantity protocols. Source remains unfrozen, so this is focused diagnostic evidence rather than a final source certificate. New section schema/axis/floor variants still await job68152.

Post97544 focused-owned input hash comparison: []. The captured API and focused fixtures/tests remained unchanged across this observation window; full model source was not frozen.

Retained native-focused-projections-r92.py runs only the two independent section/cavity contracts and the chimney production projection contract. It records exact before/after source and producer hashes, per-test progress and cooperative cancellation. It remains explicitly diagnostic even if all inputs happen to stay stable.

Job68152 exited0: **SectionExtrusionContract.test_sections_volumes_vertices_and_every_projection PASS111.507s**. This includes the new language-neutral schema, all3 independent section/SciPy volume fixtures, inferred and authored service axes, all7 FLOORING representations/volumes, and complete source schema validation after exact type restoration. All focused original five assigned integration repair categories now have fresh native GREEN evidence; three additional production projection checks44503 remain queued.

## Further late-cohort diagnostic findings

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚬️prisms.test_production_roof_profile_and_above_cut_wire_keep_material_scope

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🚬️prisms.py", line 80, in test_production_roof_profile_and_above_cut_wire_keep_material_scope
    scene._plan_services(ctx)
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🏛️savoye", line 500, in _plan_services
AttributeError: 'types.SimpleNamespace' object has no attribute 'service_surface_color'
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏗️facade.test_garden_topology_keeps_open_frame_and_covered_kiosk_distinct

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🏗️facade.py", line 90, in test_garden_topology_keeps_open_frame_and_covered_kiosk_distinct
    assert {"OP-E","OP-KIOSK-E"}<=fill
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^
AssertionError
```

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚿️ground.test_registered_ground_sanitary_programme_and_space_ownership

```
Traceback (most recent call last):
  File "C:\git\semio\🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🧱️model\🧪️tests\🚿️ground.py", line 24, in test_registered_ground_sanitary_programme_and_space_ownership
    assert math.dist(obj["geometry"]["axis"],row["center"])<=case["uncertainty_m"]
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
AssertionError
```

Queued44503 selection expanded to include the corresponding rooftop prism production projection contract, which uses the same repaired geometry-only palette harness. The job has not yet been admitted or imported, so no restart or stale selection is involved. Final selection4 functions, source/producer snapshots include both chimney modules and fixtures.

Filling ownership preflight46705 did not reach Python: after389s admission waiting, repo host-capacity gate failed EPERM creating C:/Users/Ueli/AppData/Local/semio/host-capacity/lock (takeLock gate source line139, mkdirSync). This is infrastructure evidence, not a RED geometric assertion. The portable neutral/source pair fixture and exact0.00368m³ positive GEOS control are prepared in model tests/🪟️structure.{py,json,schema.json}; no mainAPI repair or standard-hook registration has been made yet. Root/infra owner were notified. No admission bypass or blind restart.

Ownership dependency checkpoint: current _wall_cutters resolves complete _slab_geometry_entry values; slab finish reservations can resolve architectural covering _solid_geometry. Facade owned_parts similarly resolves full slab entries while preparing secondary concrete beam cuts. If the pending actual wall-reference finish/body implementation resolves _wall_geometry from such a covering, this creates the call chain wall→slab→finish covering→wall. Primary structural-only slab/member geometry must remain independent of later wall/finish ownership. This is a concrete inspected call-chain risk to verify, not yet a reproduced runtime recursion. The filling lane does not edit the shared slab/cache implementation and uses resolve_embedded=False for direct beam cutters.

## Filling structural ownership and receiving sections

Native25985 passed all four new contracts on current producers. The earlier lost invocation had no remaining capacity grant/waiter before this fresh run. Independent GEOS exact prism overlap is zero for all seven source-proven pairs (42 original component cases). The ribbon receiving interval is derived from the actual full-height concrete post cross-section before constructing the frame, preserving the complete sourced80×40mm jamb and0.092m² positive actual post-face contact. Source gross opening and structure remain unchanged. A neutral sloped structural wedge independently verifies0.5m³ residual volume (SciPy), full input/output isolation, exact material/role/panel metadata, and0.6m³ after source-cutter movement. All four functions are now registered through the unified native collector. Final current-source/native SDK proof remains required; source is unfrozen.

Current40 IfcDoor/IfcWindow native factory preflight41833 passed stable source/API in66.375s after receiving/ownership repair. No frame station TypeError or empty filling body. Final native SDK validation and final-frozen whole cohort remain outstanding.

## Independent native filling proof

Current capture3483 completed five actual repaired products (CW-1,D-ENTRY,GL-RAMP-GARDEN,RW-E,RW-W), both English/German, stable source/API/codec. Independent web-ifc audit passed3.264s:10fragments,84 original occupied-collision witnesses now excluded, and one actual native occupied positive control per fragment. Native entity/type/name/GUID,9-or13 inherited attributes, material assignment, representation type, exact physical item count, aggregate occupied volume and placedXYZ agree with the authored source. This audit does not claim every individual Float32 style buffer is a closed BRep; it tests physical occupancy and parser/product identity. Source/producer hashes are retained in generated/native-r92/filling-owned-native.json. The engine emits its known empty-IFCPROJECT scaling diagnostic for sparse excerpts; all assertions pass. No source structural geometry or gross opening was moved.

## Actual native wall details

Recovered detail source used an aggregate maximum wall thickness for every vertical wall-cell band and a zero-thickness line for filling geometry. A neutral bilingual native fixture poisons gross wall thickness0.6m while actual lower/upper bands are0.20/0.16m and glass is6mm. The existing detail failed the real native-width assertion. The implementation now rotates actual owned wall/filling parts into the selected source station/normal plane, preserves ordinary-prism holes and native mesh/section geometry, and draws the actual occupied section rings. Glass/frame/source opening tags remain independently addressable; no part is flattened to metadata thickness. Native GEOS areas/widths/input-isolation passed both languages3.080s. The new function is registered; actual current-source detail/projection fixtures still need fresh execution after source freeze.

## Production physical visibility

New declarative visibility.json and actual drawing-interface tests reproduced four failures: cross-storey neutral/source tube omitted in cellar/first, and inclined mesh replaced by rectangular envelope. Plan selection now uses physical part bands while FLOORING retains wearing-only/same-storey semantics. Structural elevation routes mesh/section/loft through real face geometry, matching architecture/services. No product/tag exceptions.
