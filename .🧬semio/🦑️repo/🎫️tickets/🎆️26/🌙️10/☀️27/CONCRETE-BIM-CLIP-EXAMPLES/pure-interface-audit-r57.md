# Extracted Pure Interface Audit R57

Read-only bounded audit,2026-10-29. Scope limited to new axon/primitives, core/geometry, drafting/geometry and their renderer imports. No source changes.

## Concrete Public External-Type Exposure

New 📐️drafting/📏️geometry.py:8 publicly annotates P(point)→np.ndarray and returns np.zeros at10–14. This is a direct public NumPy class dependency with no explicit owned reexport. signed_area returns a builtin float correctly.

New 🏛️savoye/🧱️core/📐️geometry.py:83–90 public wall_frame returns its origin,direction,normal as NumPy arrays. The lack of an annotation does not remove its runtime type dependency. part_faces converts actual faces and normals to lists on both branches, so that boundary is already builtin-container based. geometry,role,membership return owned dictionaries, strings and sets; no concrete external type issue was established for those functions.

New 🏛️savoye/🧊️axon/🧩️primitives.py:28–35 public Prim stores faces without normalization. The native face constructor _poly38–39 builds face rings and normals as NumPy arrays; public prims228 returns Prim instances carrying these arrays. A client must therefore understand NumPy objects indirectly through the owned Prim API. Public UP13 is itself a NumPy array. _ring3's ndarray annotation is private; it is not by itself a public interface issue, but its values feed the public faces. Prim.foot constructed from numpy means can also retain NumPy scalar entries even though the container is tuple.

All three pure modules import NumPy, which was already used by the original renderer modules. Extraction does not introduce a new installed runtime dependency. The concrete issue is the new public boundary exposing a dependency type without the explicit reexport required by the user instruction. No explicit owned external type reexport or public contract exemption was found in these files. Recommend builtin float/list/tuple outputs at the pure API boundaries and renderer-local conversion; alternatively an explicit owned external-type reexport would need to be deliberate under the user's stated exception, not implied merely by import numpy as np. Do not export an unannotated NumPy object and call it neutral.

## Renderer Imports and Ownership

Axon renderer16–50 imports and binds authoritative primitive functions/class/constants from the pure module; core renderer22–28 binds shared geometry functions; drafting renderer14–15 binds P. These are direct internal reexports of the actual implementations, not alternate fallback algorithms or compatibility wrapper functions. They do not themselves explicitly reexport np.ndarray as an owned API type. No duplicated helper definitions remain at these bindings.

The pure modules contain no direct Manim import; the renderer modules retain the existing Manim boundary. No new external package was added by the inspected files. The public type exposures above were communicated to root immediately before freeze. This audit does not expand to unrelated historical renderer APIs or certify every framework external-type use.

## Frozen Source and Scoped Import Follow-Up

Fresh source read contains452elements, languagesen/de. Every element has both names and nonempty metadata.domains; every discipline membership matches the actual model element_tags in both directions. All21element classes have explicit bilingual _discipline_rows labels; IfcChimney is now correctly chimney assemblies/Schornsteinbaugruppen. All local JSONschema $refs resolve. These are independent reference/label checks, not a replacement for full schema validation.

The known new pure files import only importlib/math/NumPy and owned core/model/measure modules. Their renderer files retain existing Manim/NumPy imports. No package manifest, AGENTS file or source JSON was edited by this audit; filesystem read-only inspection cannot establish unrelated concurrent modification history, so no repository-wide clean-tree claim is made. R40 owns the pending builtin-return repair and signature-equivalence check; actual runtime public-return re-audit remains pending that delivery.

## Independent Real JSONschema Cross-Validator

Ran the updated cached in-process harness with real jsonschema4.23.0, without importing product source. Draft202012Validator.check_schema accepted the actual schema, then iter_errors returned zero errors for all452elements/source SHA256c53b7cbe645fb7660d4662799f9e3d79c928dc91d1b3d700bfff7693f5e2e208. Validation took11.2697s. Retained reproduction source-schema-independent-r57.py; actual result/progress JSON is under generated/source-schema-independent-r57.json and source-schema-progress-r57.json. This independently confirms the current source/schema pair, alongside the parent-owned AJV2020 check; no schema validity was inferred merely from parse success.

## Independent Actual Runtime Re-Audit After Repair

Imported the corrected actual pure modules through the allowed in-process Python interpreter. Tested public P with NumPy float/array input, public UP and wall_frame, then built CURRENT actual public prims and recursively inspected EVERY Prim slot: tag,item,stage,foot,faces,merge,alias,sub,z,base. The checker uses exact type(x) membership in builtin str/int/float/bool/None/list/tuple/dict/set/frozenset, not isinstance, so NumPy scalar subclasses cannot evade it.

```json
{
  "primitive_count": 11870,
  "strict_builtin_issues": [],
  "manim_loaded": false,
  "sample_types": {
    "P": "<class 'tuple'>",
    "UP": "<class 'tuple'>",
    "wall_frame": "<class 'tuple'>"
  },
  "seconds": 66.56874459999995
}
```

PASS: zero non-builtin public-field values across11,870current primitives, P/UP/wall_frame are tuples and no Manim module loaded. Actual build/check took66.5687s; the55s observation timeout did not restart or cancel the worker, and its completed result was read afterward before disposal. Retained reproduction pure-return-independent-r57.py.

R40 separately reports24focused tests passing and exact numerical equality for12,094rows built from the SAME cached before/after geometry. Its canonical numeric signature55da25a7cc71829d0e13b96eb33fddeb9032b8b78c0a204d662b78fccb31e970 is identical before/after; raw JSON changes only integer-versus-float formatting. That cached count is not claimed as the current source cohort: this independent fresh build produced11,870primitives. The public type boundary is independently verified on the current build; numerical baseline equivalence remains the separate same-input R40 evidence. The initial concrete NumPy public leaks identified above are resolved in the repaired boundaries.
