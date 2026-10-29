# Prismatic Chimney Construction Audit R50

Read-only independent construction audit,2026-10-29. Product source untouched. Candidate native occupancy verification pending candidate delivery; this report distinguishes computed source constraints from verified candidate outcomes.

## Registered Construction Scope

R48 actual author-plan pixels support hollow-square terminal morphology and rounded0.35m outer/0.15m clear-bore reconstruction. Coordinates are nominal salon(13.01,1.22),kitchen(0.89,5.32)m with±0.10m practical registration allowance for publication. Tops9.25/9.60m with±0.40m are authoring assumptions, not a calibrated photo measurement. The source text summarized in prismatic-chimneys-r44.md supports rendered hollow-brick construction with reinforcement. Neither the plan's double line nor7–15px original photographic silhouettes measure render thickness, brick unit cavities, reinforcement size/spacing, liner, separate skins or cap. A0.10m nominal wall follows chosen envelopes, not a surveyed brick section.

## Actual Source Support Levels

Inspected SL-R and LS-ROOF directly in memory; no checked-in source copy. Salon center falls in R-PRIMARY-SL-R-10; kitchen in R-PRIMARY-SL-R-6. Both are0.17m composite structural regions with top_offset−0.09m from roof6.60m, giving structural top6.51m and nominal bottom6.34m. Four footprint corners plus center were checked; all remain in their respective region. The local compression layer is0.05m, so its nominal vertical range is6.46..6.51m. Global slab top_offset−0.03m does not override the regions' local−0.09m top.

LS-ROOF includes a0.09m weathering/paving reserve. Its actual occupied parts near these footprints must be checked before resolving ring ownership between6.51 and6.60m. Source layer declaration alone is not proof of local actual occupancy.

## Bore Versus Footprint

The chosen outer area is0.1225m², bore0.0225m², masonry ring0.1000m². A0.37m through-all-layers opening removes0.1369m²: all possible annular support plus0.0144m² outside the modeled footprint. A stack beginning6.60m above that full-footprint hole has no bottom-face roof support. The fact that an opening/host relation exists does not make the annulus supported.

A bore-only0.15m passage preserves0.1000m² annular structural bearing, provided the full ring lies on occupied compression concrete. Roof-only chimney material beginning6.51m can directly meet that support. However, if the weathering reserve occupies the same ring up to6.60m, it must be trimmed only over the exterior footprint or assigned distinct ownership; otherwise chimney and weathering overlap. A full-footprint hole through all structural layers loses bearing again. Starting only at6.60m may contact finish if present, but must not be described as direct structural bearing without a modeled support neck or valid underlying material ownership.

No full vertical continuation into rooms or unobserved structural support can be invented to solve this local interface. Adopt a consistent roof-emergence reconstruction and label its base detail inferred.

## Required Native Verification

For each prepared proposal, inspect actual occupied parts, not just envelope: ring walls must close continuously at corners, leave0.15m bore open at every chimney height, and meet supporting roof material over the annulus at their bottom. Test a horizontal slice just below/above6.51m and6.60m; compare chimney/slab material intersection and missing support area separately. Inspect both bore through compression and finish and any full-footprint finish-only trim. Confirm IFC parts preserve the same ring void and base elevation. Expected ring volume before any ownership trimming is0.1000×height cubic metres; this arithmetic is a construction invariant, not historic dimension validation.

Parent and chimney agent received these source-level constraints. No candidate has yet been certified by this report.

## Prepared Patch Independent Check

Read generated/prismatic-patch-r44.json directly after R40 delivery. Independent rectangle-bound intersection arithmetic yields:

```json
[
  {
    "tag": "CHIMNEY-SALON-ROOF",
    "count": 12,
    "area": 0.10000000000000077,
    "volume": 0.274000000000002,
    "pairOverlap": 0,
    "boreArea": 0.022499999999999774,
    "boreMaterialOverlap": 0,
    "bottom": 6.51,
    "top": 9.25,
    "neck": 0.009000000000000069,
    "materials": {
      "MAT-RENDER": 0.03726400000000293,
      "MAT-HOLLOW-CLAY": 0.21919999999999773,
      "MAT-PLASTER": 0.01753600000000138
    }
  },
  {
    "tag": "CHIMNEY-KITCHEN-ROOF",
    "count": 12,
    "area": 0.09999999999999978,
    "volume": 0.30899999999999933,
    "pairOverlap": 0,
    "boreArea": 0.02250000000000004,
    "boreMaterialOverlap": 0,
    "bottom": 6.51,
    "top": 9.6,
    "neck": 0.00899999999999998,
    "materials": {
      "MAT-RENDER": 0.04202399999999944,
      "MAT-HOLLOW-CLAY": 0.2472000000000001,
      "MAT-PLASTER": 0.0197759999999998
    }
  }
]
```

Both12-part rings are pairwise disjoint, leave the exact0.15m-square bore empty, start6.51m and match proposed top elevations. Neck ring volume0.009m³ each agrees with required finish displacement. This verifies prepared source patch geometry only; actual API slab and emitted native ownership remain pending their selected output.

## Actual API Candidate Geometry Audit

Independently inspected selected actual API export generated/prismatic-native-r44.json using polygon rectangle clipping and shoelace area arithmetic, separately from producer calculations. Results:

```json
[
  {
    "tag": "CHIMNEY-SALON-ROOF",
    "structuralContactArea": 0.10000000000000053,
    "structuralOverlapVolume": 0,
    "exclusiveFinishOverlapVolume": 0.009000000000000036,
    "boreStructuralVolume": 9.059419880941274e-16,
    "boreFinishVolume": 4.796163466380669e-16,
    "ownedFinishVolume": 0,
    "actualVolume": 0.2739999999999991,
    "apiQuantity": {
      "part_count": 12,
      "volume": 0.274,
      "material_quantities": [
        {
          "material": "MAT-RENDER",
          "volume": 0.037264,
          "unit_weight": {
            "value": 20,
            "unit": "kN/m³",
            "basis": "assumed"
          },
          "weight": 0.74528,
          "mass": 75.997409921
        },
        {
          "material": "MAT-HOLLOW-CLAY",
          "volume": 0.2192,
          "unit_weight": {
            "value": 8,
            "unit": "kN/m³",
            "basis": "assumed"
          },
          "weight": 1.7536,
          "mass": 178.817435108
        },
        {
          "material": "MAT-PLASTER",
          "volume": 0.017536,
          "unit_weight": {
            "value": 18,
            "unit": "kN/m³",
            "basis": "assumed"
          },
          "weight": 0.315648,
          "mass": 32.187138319
        }
      ],
      "weight": 2.814528,
      "mass": 287.001983348
    },
    "sharedCount": 0
  },
  {
    "tag": "CHIMNEY-KITCHEN-ROOF",
    "structuralContactArea": 0.10000000000000142,
    "structuralOverlapVolume": 0,
    "exclusiveFinishOverlapVolume": 0.009000000000000114,
    "boreStructuralVolume": 0,
    "boreFinishVolume": 0,
    "ownedFinishVolume": 0,
    "actualVolume": 0.30899999999999955,
    "apiQuantity": {
      "part_count": 12,
      "volume": 0.309,
      "material_quantities": [
        {
          "material": "MAT-RENDER",
          "volume": 0.042024,
          "unit_weight": {
            "value": 20,
            "unit": "kN/m³",
            "basis": "assumed"
          },
          "weight": 0.84048,
          "mass": 85.705108268
        },
        {
          "material": "MAT-HOLLOW-CLAY",
          "volume": 0.2472,
          "unit_weight": {
            "value": 8,
            "unit": "kN/m³",
            "basis": "assumed"
          },
          "weight": 1.9776,
          "mass": 201.659078279
        },
        {
          "material": "MAT-PLASTER",
          "volume": 0.019776,
          "unit_weight": {
            "value": 18,
            "unit": "kN/m³",
            "basis": "assumed"
          },
          "weight": 0.355968,
          "mass": 36.29863409
        }
      ],
      "weight": 3.174048,
      "mass": 323.662820637
    },
    "sharedCount": 0
  }
]
```

ACTUAL CANDIDATE FAILURE: both rings contact structural material over0.1000m² at6.51m with zero structural overlap and empty bores, but exclusive finish overlaps EACH chimney neck by0.009m³. No shared_finish_parts entry has either chimney as owner. Thus the delivered API export has not applied the intended finish partition, regardless of whether the producer code or candidate assembly caused it. Do not publish until exclusive finish overlap becomes zero and owned bookkeeping records0.009m³ per chimney. Parent and R40 were notified immediately. Quantities0.274/0.309m³ agree with rings. This check is selected API geometry, not IFC mesh/full-building collision/render certification.

The delivered reproduction script prismatic-physical-r44.py line6 assigns finish_voids through api.element('SL-R',m), the public isolated-copy API. This likely leaves the actual m.elements slab unchanged; the existing test_element_returns_independent_resolved_copies explicitly protects that behavior. Mutate the actual candidate source object before computing geometry, then rerun/export. This diagnosis does not relax the failed occupancy gate.

## Corrected Candidate Re-Audit

R40 corrected the isolated-copy mutation in candidate assembly and regenerated the selected actual export. Independently reread that fresh JSON and repeated the same clipping arithmetic, without reusing prior geometry.

```json
[
  {
    "tag": "CHIMNEY-SALON-ROOF",
    "structuralContactArea": 0.10000000000000053,
    "structuralOverlapVolume": 0,
    "exclusiveFinishOverlapVolume": 7.993605777301114e-16,
    "boreStructuralVolume": 9.059419880941274e-16,
    "boreFinishVolume": 0,
    "ownedFinishVolume": 0.008999999999999954,
    "ownedFinishCount": 27,
    "ownedMaterial": 3,
    "referenceFinishRingVolume": 0.009000000000000036
  },
  {
    "tag": "CHIMNEY-KITCHEN-ROOF",
    "structuralContactArea": 0.10000000000000142,
    "structuralOverlapVolume": 0,
    "exclusiveFinishOverlapVolume": 1.1990408665951672e-16,
    "boreStructuralVolume": 0,
    "boreFinishVolume": 0,
    "ownedFinishVolume": 0.009000000000000072,
    "ownedFinishCount": 21,
    "ownedMaterial": 3,
    "referenceFinishRingVolume": 0.009000000000000114
  }
]
```

PASS for the corrected selected API candidate: both rings have0.1000m² structural bearing, zero structural overlap, zero exclusive-finish overlap, empty0.15m bores, and0.009m³ explicitly owner-assigned shared finish per chimney. Full_finish_parts retain0.009m³ semantic reference envelope per ring while exclusive finish does not duplicate it. Prior failure is retained above as evidence of the repaired candidate-assembly error. This verifies selected material partition; R40/root remain responsible for independent native IFC meshes, actual emitted slab ownership and broad whole-building collision/render gates.
