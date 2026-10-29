# Curtain Entrance Material Contact Audit R56

Read-only independent audit,2026-10-29. No source changes or tolerance relaxation. Pre/post occupied-part dump requested from R11; exact collision-cause and repaired-candidate verification pending delivery.

## Established Actual Failure

Inspected curtain-portal-contact-r49.py and actual output generated/curtain-portal-contact-r49.json, source c53b7cbe645fb7660d4662799f9e3d79c928dc91d1b3d700bfff7693f5e2e208. The reproduction integrates GEOS occupied polygon intersections over shared positive height and ignores disjoint AABBs. It compares actual437curtain parts against29entrance and9service-door parts. Its positive self-control is2.564869515e−6m³.

Entrance has37positive material pairs above1e−12m³. Maximum is CWpart282/entrancepart0=2.6402401201e−6m³; another major pair is CW293/entrance8=2.4226940789e−6m³. The service-door maximum2.49625217e−16m³ is numerical noise, with no positive pair under the same threshold. Therefore the entrance issue is real occupied material duplication, not simply a projected-plan difference.

## Interpretation Boundary

Generated parking-curtain-delta-r55.json reports316→320frame obstacles and nearly equal total projected areas, with removed0.0000653388707m² and added0.0000653383357m². Parent localizes these to endpoints8.64/10.5,Y4.6. These projected deltas cannot establish an old header's3Dheight, historic opening section or exact material ownership by themselves. Exact pre/post material polygons and vertical extents are required for the final diagnosis.

Door owns its actual frame and casing. Native IFC overall rough dimensions need not include the casing envelope. A correction should assign actual material ownership at portal/end planes, preserving adjoining curtain and door solids without widening nominal rough dimensions to evade the collision. No inference of new source-measured casing or hidden construction is authorized.

## Required Independent Re-Audit

Check all actual curtain/entrance part intersections at their shared height, not merely the former maximum pair or whole assemblyAABB. Repeat the same calculation for service door as an unaffected control. Positive-volume overlap must become numerical zero without changing tolerance, dropping source pieces, or treating a thin miter wedge as empty solely because its area is small. Compare retained outside-portal/end-plane material and actual door casing bounds so a gap or deleted jamb is not substituted for overlap. This report does not yet claim that a repair passed.

## Independent Actual Pre-Fix Part Audit

Read generated/curtain-contact-before-r56.json and reproduced every pair with independent Sutherland-Hodgman planar clipping against actual convex door-part outlines, integrated over exact shared height. Retained reproduction curtain-entry-independent-r56.mjs handles the rotated service-door control with polygon clipping, not an axis-aligned bounding-box substitute. Results:

```json
[
  {
    "tag": "D-ENTRY",
    "curtain_parts": 437,
    "filling_parts": 29,
    "maximum": [
      0.0000026402401146707884,
      282,
      0
    ],
    "total": 0.000009597623353752025,
    "positive": 37
  },
  {
    "tag": "D-SERVICE",
    "curtain_parts": 437,
    "filling_parts": 9,
    "maximum": [
      1.8518520050747615e-15,
      331,
      1
    ],
    "total": 4.893863092547688e-15,
    "positive": 0
  }
]
```

Exact maximum pair: CW282 is MAT-METAL mullion station CW-MEMBER-32 on PATH-HALL, z1.0125..2.675m. Its finite/miter polygon reaches X8.652499561697 and Y4.622967936868. D-ENTRYpart0 is MAT-METAL wing_skin, rectangleX8.64..8.642,Y4.56..4.64,z0..2.675m. The actual curtain miter intrudes into the door-owned2mm metal skin over positive height. Counterpart skin atX10.498..10.5 participates at the opposite entrance endpoint. Thus a finite rough aperture alone does not resolve occupied casing material; the actual door-owned parts must take precedence. No rough dimension, section or source coordinate needs enlargement merely to assign that ownership.

## Corrected Actual Post-Fix Audit

Independently read generated/curtain-contact-after-r56.json and repeated all material pairs with the same polygon clipping and1e−12reporting threshold. Both exports retain source c53b7cbe and437curtain/29entrance/9service parts. Results:

```json
{
  "source_before": "c53b7cbe645fb7660d4662799f9e3d79c928dc91d1b3d700bfff7693f5e2e208",
  "source_after": "c53b7cbe645fb7660d4662799f9e3d79c928dc91d1b3d700bfff7693f5e2e208",
  "before": [
    {
      "tag": "D-ENTRY",
      "curtain_parts": 437,
      "filling_parts": 29,
      "maximum": [
        0.0000026402401146707884,
        282,
        0
      ],
      "total": 0.000009597623353752025,
      "positive": 37
    },
    {
      "tag": "D-SERVICE",
      "curtain_parts": 437,
      "filling_parts": 9,
      "maximum": [
        1.8518520050747615e-15,
        331,
        1
      ],
      "total": 4.893863092547688e-15,
      "positive": 0
    }
  ],
  "after": [
    {
      "tag": "D-ENTRY",
      "curtain_parts": 437,
      "filling_parts": 29,
      "maximum": [
        0,
        -1,
        -1
      ],
      "total": 0,
      "positive": 0
    },
    {
      "tag": "D-SERVICE",
      "curtain_parts": 437,
      "filling_parts": 9,
      "maximum": [
        3.703704010149523e-15,
        331,
        1
      ],
      "total": 1.540101379760017e-14,
      "positive": 0
    }
  ],
  "before_volumes": {
    "curtain_volume": 0.30199192181809703,
    "entry_volume": 0.09854499999999863,
    "service_volume": 0.02897999999999968,
    "entry_self_overlap": 0
  },
  "after_volumes": {
    "curtain_volume": 0.3019823241947554,
    "entry_volume": 0.09854499999999863,
    "service_volume": 0.02897999999999968,
    "entry_self_overlap": 0
  },
  "curtain_removed": 0.000009597623341628392,
  "pre_overlap_sum": 0.000009597623353752025,
  "maximum_group_volume_conservation_error": 9.593714740658892e-15,
  "entrance_unchanged": true,
  "service_unchanged": true
}
```

PASS: all entrance/curtain occupied intersections are exactly zero in independent arithmetic. Service control remains numerical noise below4e−15m³ maximum. Door assemblies are byte-for-byte identical in their derived JSON. Removed curtain volume9.59762334e−6m³ equals independently integrated pre-fix duplicated material9.59762335e−6m³ within1.3e−14m³. Grouping by stable path/member/station/role/material identity, each group volume change equals its own pre-fix occupied overlap within1.9e−14m³. Losses are confined to PATH-HALL metal sill/transom and entrance endpoint mullion groups; no glass/material-family loss or changed door dimensions was required. Polygon splitting/reordering means array index comparisons are not geometric identity.

This confirms the narrow material ownership repair for the actual derived pair set with no tolerance relaxation. It does not claim old parking projections proved a former header elevation or independently certify a full rendered/native SDK frame. Parent owns updated parking/native publication gates.
