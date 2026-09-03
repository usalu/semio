# 🔍️ Explore — boolean, euler, sew, offset, blend, sweep, diff root

Read-only audit (Haiku explorer, 2026-09-03). Base: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff`.

## 1. Diff root (`🦀️.rs`, 1290 lines)

`SemioBrepDiff` = 6 `NamedTripleDiff` tables (vertices, edges, loops, faces, shells, solids), sparse field-by-field (209–224). `between` 514–523, `apply` 437–464, binary LEB128 + presence bitmap 980–1056, text codec 895–941. Lossy: loop edges / shell faces / solid shells are whole-value replaces (165–172, 189–192, 196–199); coedges, p-curves, pcurve ranges, knots, tolerances not in the diff. Tests: `between_roundtrip_law` 1153, `field_sweep` 1165, `inverse_law` 1205, `absorb_law` 1215–1268, codec roundtrip 1273.

## 2. Boolean (`🔀️boolean/🦀️.rs`, 612 lines)

`boolean_solid(body,a,b,op,tol,rec)` 38; `compound_cut` 58; `section_solid_by_plane` 75 (in-plane vertices + edge/plane hits → one planar face; tol×10 at 82); `split_solid_by_plane` 123 (tessellate + centroid classify → soup, or AABB-corner hull).
Pipeline: AABB fast path 192–233 (disjoint / contained / intersect-as-box / unite box-ness via volume tol 1e-6 at 246); mesh fallback 254–274: `tessellate_solid(body, solid, tol.max(1e-3))` (256–257), centroid classification via `point_in_solid`, triangle keep/discard 305–317, rebuild `solid_from_triangle_soup` (267) then `make_convex_hull` fallback (269, also 174/178/182). **No exact path.** History: `OpRecorder` threaded.

## 3. Euler (`🔺️euler/🦀️.rs`, 633 lines)

`make_vertex` 38, `make_edge` 46, `make_loop` 56, `add_face` 71, `add_shell` 79, `add_solid` 87, `split_edge` 105, `split_planar_face_by_line` 155 (UV arrangement 182–211, same-edge split 217–239, loop walk 247–254, rebuild two loops with chord 261–279). Callers: sew (45–62), sweep prism builders, blend rebuild, boolean cleanup. Tests: Euler–Poincaré tetra 469, generated records 483, loop ring 493, split_edge cases 504–576, split rectangle 576, rejects non-cutting 596.

## 4. Sew/heal (`🧵️sew/🦀️.rs`, 464 lines)

`sew_faces` 31: quantised vertex map (`resolution = 1/linear`, 38–40), canonical edge pairs, rebuild loops/faces 42–62. `heal_solid` 131: validates + merges near-coincident vertices (133), flags degenerate edges (160); **no topology repair**. `defeature` 176: remove faces + merge coplanar (dot > 0.999, dist < 1e-6 at 289–299). `convert_to_nurbs` 210: plane → 2×2 bilinear NURBS (308), curves via `to_nurbs(range)` (249). Silent continue on missing entities 145–149. Tests 362–444.

## 5. Offset (`↔️offset/🦀️.rs`, 453 lines)

`offset_face` 28: **planar only**, error otherwise (34–36). `offset_solid` 66: tessellate + point offset + AABB corners + hull (76–91), box fast path by volume≈AABB ±1e-3 (205). `thicken_face` 48: planar → extrude; else `thicken_face_hull` 55–61. `shell_solid` 96: inner offset + boolean cut (104–105) or void shell (107). `shell_solid_with_open_faces` 113: hull tools per open face, **silent continue on hull failure (129)**. `draft_angle` 173: **AABB shear for boxes only (184–187), non-box → copy (189), neutral point ignored**, shear 244–260. Tests 401–443.

## 6. Blend (`🎨️blend/🦀️.rs`, 341 lines)

`fillet_edges` 32, `fillet_variable` 40, `chamfer_edges` 51. `EDGE_STATIONS = 5` (68), `ARC_SAMPLES = 5` (69). `sample_blunt_geometry` 191: skip endpoints 202–204, 5 stations 220–221, fillet arc ring 238–246, chamfer inset 230–236, strip triangles 251–263. Rebuild `solid_from_triangle_soup` 183 → `make_convex_hull` 187. Requires non-empty edges (error 75); blend amount < min adjacent edge length (86). Tests 287–333.

## 7. Sweep (`➡️sweep/🦀️.rs`, 508 lines)

`extrude_face` 238: prism polygon 245; cylinder special case 179–227 (single closed circle edge → analytic cylinder 190–192). `revolve_face` 251: steps = ceil(angle/(π/4)) clamped 8–64 (257). `loft_profiles` 270: `_smooth` ignored. `sweep_along_path` 283: 16 samples/edge (285), tangent frame 292–295. `pipe` 310: `_guide` ignored. `helical_sweep` 316: steps = ceil(turns·16) clamped 16–128 (323). Sections must have equal vertex count (408), ≥3 (404); fan caps 425–428; `solid_from_lofted_sections` → `solid_from_triangle_soup` (429). **`curve_point` NURBS branch returns origin (393).** Tests 459–499.

## 8. Helper matrix

| function | helper | lines |
|---|---|---|
| boolean_solid | tessellate_solid / solid_from_triangle_soup / make_convex_hull | 256–257 / 267,176,180 / 174,178,182,269 |
| offset_solid | tessellate_solid / make_convex_hull | 269 / 91,240 |
| shell_solid_with_open_faces | make_convex_hull | 129 |
| blend | solid_from_triangle_soup / make_convex_hull | 183 / 187 |
| sweep | solid_from_triangle_soup | 429 |
| sew_faces | make_edge/add_face/add_shell/add_solid | 45–62 |

## 9. Gaps (audit §6.3–6.6)

No exact SSI-based boolean; no offset surfaces; blends are soup strips; sweeps are lofted soup except the single-circle cylinder case; silent continues at offset 129/61/189 and sweep 393; tolerances hardcoded (1e-3 tess clamp, 1e-6 volumes, 1e-15 normalisation).
