# Roof Door Spatial Audit R54

Read-only independent audit,2026-10-29. No source changes. Actual finite-cut candidate dump pending R11 delivery; no native collision or route pass is claimed yet.

## Raw Candidate Reviewed

Read roof-door-candidate-r49.json directly. Approved register is X3.06171694865m,Y8.98..10.15223949926m, opening height2.43m over6.60m roof datum, therefore head9.03m matching enclosure soffit. Overall assembly interval is1.17223949926m. It lies along the right vertical author-plan cubicle face under the R48 image registration, rather than the old top-horizontal source interval. Plan-position uncertainty remains±0.15m.

Type uses central MAT-WOOD panel, MAT-METAL frame and MAT-TEXTURED-GLASS surround. Six panel cells are explicitly partitioned: left lower/upper fixed glass,u0..0.2; central swing timber,u0.2..0.8,v0..0.88; right lower/upper fixed glass,u0.8..1; full-width transom,u0..1,v0.88..1. Sidelights split atv0.528, exactly60percent of leaf height. This matches R49 actual modern Ramos figure11 morphology. Ratios20/60/20 and88percent leaf height are inferred, not measured original dimensions. The source text supports the historic wooden leaf/translucent surround distinction; the modern photo supplies layout only.

Frame0.025m width/0.04m depth, timber0.035m thickness, textured glass0.004m, panel member0.025m and inset0.21m remain inferred. The photograph does not prove a wired-glass product, section sizes, original leaf width or swing direction. Operation swing identifies the leaf family, not an observed swing handedness.

## Required Actual Occupancy and Route Checks

R11 is preparing finite planar end cuts so a jamb reaching a path vertex cannot remove wall material past that endpoint or leave a corner wedge inside filling material. Actual occupied W-STAIR-ROOF parts, native opening parts and filling parts must be compared before a pass. Touching faces at jamb/soffit are valid; positive-volume wall/filling overlap is not.

The route must begin on actual ST-FR final-tread/arrival-floor support and reach the central movable leaf interval, not the entire rough opening including fixed sidelights. Its centerline may be an inferred route; body width is a chosen evaluation parameter, not an original source measurement. Do not impose an arbitrary0.8m historical/code-width claim or widen the modeled leaf to satisfy one. Report measured available clearance and support against actual native guard/floor-void material. A closed leaf is intentionally an obstacle; traversal is assessed through the leaf's open-state aperture with no invented swing-sweep clearance.

Root and R11 have the raw-candidate findings. Actual geometry validation remains pending.

## Final Layered Actual Geometry Audit

Inspected regenerated final candidate geometry export from source f4f7b26dfbb46749c1c93a4bb2ec94be229615e118d77c99a15a923e45152020 plus approved door fields. Parent subsequently published the candidate in dc4a8306. Independent reproducible Node clipping/interval analysis is retained in roof-door-independent-r54.mjs. Results:

```json
{
  "source_hash": "f4f7b26dfbb46749c1c93a4bb2ec94be229615e118d77c99a15a923e45152020",
  "parts": 66,
  "wall_overlap_m3": 1.7266188478970432e-14,
  "self_overlap_m3": 0,
  "head": 9.03,
  "materials": [
    "MAT-METAL",
    "MAT-TEXTURED-GLASS",
    "MAT-PLYWOOD",
    "MAT-WOOD"
  ],
  "leaf_clear": [
    0.6483436995572673,
    2.103900000000001
  ],
  "route": {
    "assumed": [
      [
        5.032144828882639,
        9.665220372262652
      ],
      [
        3.0667169486516617,
        9.566119749631056
      ]
    ],
    "support_intervals": [
      [
        0,
        1
      ]
    ],
    "parallel_sample_support": [
      [
        [
          0,
          1
        ]
      ],
      [
        [
          0,
          1
        ]
      ],
      [
        [
          0,
          1
        ]
      ]
    ],
    "obstacle_intervals": [],
    "minimum_edge_clearance": 0.40499999999999936
  },
  "volumes": {
    "MAT-METAL": 0.011736174798232624,
    "MAT-TEXTURED-GLASS": 0.004764349215013825,
    "MAT-PLYWOOD": 0.013640503094986677,
    "MAT-WOOD": 0.005344487399119599
  }
}
```

PASS for actual66-part assembly: wall/filling volume overlap is only1.73e−14m³ numerical noise and self-overlap0; top9.03m equals soffit. Actual materials are plywood skins, perimeter timber battens, metal and translucent textured glass. No solid air/core is emitted. This corrects the earlier raw solid-timber candidate description. Leaf clear aperture0.6483437×2.1039m is separately identified from the larger rough opening. The finite cut leaves no positive-volume filling/corner-wall collision; remaining miter material outside the finite end plane stays present in the actual wall. Parent separately reports independent native SDK verification; this audit did not reproduce that SDK result.

An explicitly assumed straight approach runs from the ST-FR final-tread center(5.0321448,9.6652204) to the inner side of central leaf(3.0667169,9.5661197). Exact polygon-line intersections show continuous support over its complete0..1parameter range and no native wall/guard intersection. Parallel support samples atY±0.04m also remain continuously supported. Nearest approach edge distance to wall/guard material is0.405m; the leaf aperture independently limits passage to0.6483437m clear. This is a supported approach/centerline proof and8cm-wide foot-strip sampling, not a code/accessibility certification, a0.8m-body proof or a door swing-sweep proof. Closed movable leaf material is intentionally excluded from approach obstruction; fixed sidelines are not claimed traversable.

The initial independent GEOS attempt stalled in a global union; its obsolete worker was terminated after observation timeout. No result from that attempt was claimed. The bounded Node calculation uses actual polygons and vertical intersections, preserving all actual wall/guard candidates rather than dropping geometry by guessed tags.
