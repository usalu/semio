# Exact TFS Alias Closure and Normal Facts

The independent ticket input `tfs-exact-closure-oracle.py` reads the four unchanged original physical files through installed IfcOpenShell0.8.5. It strictly resolves CoordIndex to PnIndex if present, then to finite three-component coordinates. Exact numeric tuples form geometric keys; negative and positive zero compare equal. It performs no tolerance, context-precision rounding, normalization or coordinate mutation. Full original SHA256 and raw owner observations are retained in `generated/tfs-admission/exact-closure.json`.

| Original | TFS | Source / exact coordinates | Triangles | Geometric edges | Bad edges |
|---|---:|---:|---:|---:|---:|
| triangulated-item |1021|8 /8|12|18|0|
| slab-tessellated-unique-vertices |41|12 /12|20|30|0|
| beam-straight-i-shape-tessellated |201|128 /48|92|138|0|
| beam-curved-i-shape-tessellated |64|4058 /4058|8112|12168|0|

Each edge has exactly two owners in opposite directions. Every original reports ClosedTRUE, no identity-degenerate or exact-zero-cross-product triangle, no duplicate geometric triangle, and no unused coordinate row. All four omit PnIndex and Normals; therefore there are no unused mapping/normal rows to assess in these original inputs. The detailed output includes every exact alias group and source indices. The straight beam's duplicated source coordinates close EXACTLY; context precision or an EPS weld is unnecessary. This is edge/triangle incidence evidence, not an independent solid self-intersection/nesting/whole-geometry proof.

## Normal Admission Versus Rendering

The primary pinned TFS page says normals provide directions, describes raw CoordIndex addressing the normal list, and explicitly permits non-unique vertex collections to support differing normals. Its narrative says identical normals should be omitted; it does not provide an EXPRESS unit-length or nonzero-vector predicate. The actual released Normals declaration is OPTIONAL LIST1 of LIST3 of IfcParameterValue, a REAL defined type with no such WHERE rule. No formal universal equality of normal/coordinate/mapping counts appears in the entity declaration.

The independent `tfs-normal-formal-oracle.py` substitutes eight zero vectors and separately eight nonunit(2,0,0) vectors into fresh parses of the unchanged triangulated-item graph. IfcOpenShell0.8.5 full EXPRESS emits no statements for either (`generated/tfs-admission/normal-formal.json`). These are deliberately authored formal-domain observations, not normative sensible shading or physical direction certification. A rendering implementation may need explicit refusal/diagnostic for zero directions and correct transformation/normalization of nonunit normals, but that must be an announced rendering policy rather than an invented IfcParameterValue schema restriction.

Existing glTF normal dropping is a separately diagnosed exchange limitation; TFS admission should retain raw source semantics without pretending that normals have already been transported. ClosedFALSE/unset should not gain a new global surface-intersection condition simply through reuse of a stricter kernel. No production edit, native execution or full MVD claim is made.
