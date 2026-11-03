# Compile review O: EN 1998 (wave 2)

Nothing compiled. Scripts reused (niced, sequential): rsck, exh, depck, littype, argtype, devdep, cfgfeat, arrlen from `🗑️generated/compile-review-{a,i}`, plus new `🗑️generated/compile-review-o/freeid.py` (free fn/const identifiers across split crates). Logs in `🗑️generated/compile-review-o/`.

## Findings fixed
1. Typed enum fallout (coordinator request): `part_1::fundamental_period_s` now takes `T1Method` by value and matches variants (no string parsing); callers fixed (`scene/🧮️response` x2, `part-evaluators`, `building-regularity-and-base-shear-checks`). `T1Method::as_str` added (snapshot rs) for the four report texts that formatted `t1_method` with `{}`; `to_ascii_lowercase() == "given"` replaced by enum compare; `scene/📈️diagrams` had `b.t1_method == T1Method::Given.then(..)` (precedence, compares enum with Option) now parenthesised.
2. `part_1::drift_theta` takes `DriftLimitClass` by value but three callers passed `&..` (scene diagrams, storey-response, storey-forces-and-drift): fixed.
3. `{}` Display on enums without Display: `a.limit_state` (part-evaluators DE text) -> `.token()`; `system.direction` x2 (system-checks horizontal-elements) -> `.token()`; `k.device` (DE bearing detail) and `d.material` (DE diaphragm detail) -> `{:?}`.
4. `-more` mutation crates (7: buildings, buildings-storey, buildings-system, bridges-bearings, bridges-piers, members-joints, members-wall-rows): leaves gate serde on feature `test-support`, but the crates declared none and parents did not forward it. The aggregate `En1998Mutation` derives serde under `test-support`/test, so leaves would lack Serialize. Added `[features] test-support` to each `-more` crate and `"<more>/test-support"` to the parents' `test-support`. (en1993 `joint-*-more` and vdi3805 `geometry-more` show the same pattern: routed below.)

## Verified clean by reading/scripts
- rsck: only known false positives remain (derive-generated `Diff`/`ReferenceDiff`/`FromValue`/`ToValue`, enum variants flagged as names, closures named like fns, `use crate::part_1` inside inline mods, extern aliases).
- exh: no non-exhaustive match (`StiffnessBasis`, `T1Method`, `DriftLimitClass`, `MemberKind`, `BearingDevice`).
- depck: every `use semio_*` has a Cargo dep (incl. details, structure-checks, scene-analysis, scene-structures, edit-rules); no missing path deps. `missing 📋️project.json/📜️script.ts` is endemic to every crate (not en1998-specific).
- `#[path]` mounts of inferences/details/full-seismic-check/scene/scene-analysis/structure-checks/mutations all resolve.
- details split: part-details and geotechnical-details names resolve through `use super::*` + `seismic_helpers::*`; private helpers (`status_of`, `q_moment`, ...) are only used in their own file; cross-crate items are `pub`. freeid.py (1030 module-level fn/const names) found only 3 false positives (`as` aliases, closure var).
- scene/scene-analysis/scene-structures: every `capacity::/response::/geotechnical::/storey_response::/structure_views::` path hits a `pub` item; `Marker` literals have all 8 fields; `Mark` methods are pub.
- Snapshot/`joints`/`walls`/`reliability_class`: `En1998Snapshot` literal and `to_snapshot/from_snapshot` carry `joints` and `placements`; `reliabilityClass` was removed everywhere by another agent during this review (no remnants in en1998 now). `walls` is only a module name.
- v14 code (storey-shear/drift, bridge pier/bearing/deck-mode, tank pressure layer): signatures of `column`, `axis`, `Affine`, `FieldBinding`, `UtilisationBinding`, `WallPressure` fields, `tank::surface_elevation_m`, `Vec3::{ZERO,X}` match; the only errors were the Display-on-enum ones above.

## Open / not verified
- en1993 `joint-extension-more`/`joint-beam-more` and vdi3805 `geometry-more` crates have the same missing `test-support` feature if their leaves gate serde on it (see requests.md).
- Goldens need re-bless after the next green build (v14).
