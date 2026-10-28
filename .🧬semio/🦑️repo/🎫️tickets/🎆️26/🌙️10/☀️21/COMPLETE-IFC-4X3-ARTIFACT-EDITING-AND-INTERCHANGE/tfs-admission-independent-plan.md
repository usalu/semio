# TFS Admission Independent Preparation

The ticket-only closed32-row contract is `tfs-admission-cases.json` (SHA256 b7e45af97bc002e637b07775f5238972a1e671f615cce296944fb8f77ecd59bf), schema `tfs-admission.schema.json` (f977bfa34e47e218af33c2a521e015fb7947c665b87164a875dfefcbdb95adcc). Actual strict Ajv2020 validates all32. The retained author is `tfs-admission-author.py`; no production or registered tests changed.

## Authority

Pinned buildingSMART Standards-builds610ee3f95db9d0380b1eb8de4dc6e17833531518 lexical pages were fetched unchanged. `generated/tfs-admission/originals.json` records URLs and SHA256: TriangulatedFaceSet d2b6a716260a9c0aab989bcd4d2210571d71c0d11fcdddfc44fa3694e566acf9; CartesianPointList3D480c9fb707190356b4da32bde73a195fbaeff42d4597cf4663019504f0e732e4. Local immutable ADD2 EXPRESS declarations are lines4775–79,11567–78,11830–47.

Coordinates is owned by IfcTessellatedFaceSet. Normals, Closed, CoordIndex and PnIndex are owned by IfcTriangulatedFaceSet. CoordList is owned by IfcCartesianPointList3D. Query owner/status must remain explicit; unknown or ambiguous owner is not a flat positional success. TIN is a real subtype with additional Flags and NotClosed; current exact-only dispatch must explicitly diagnose or deliberately admit it with its own declared requirements.

## Rules Without Invented Equalities

CoordList is LIST1 of exactly3 IfcLengthMeasure. CoordIndex is LIST1 of exactly3 IfcPositiveInteger. PnIndex is OPTIONAL LIST1 of IfcPositiveInteger, not UNIQUE. Normals is OPTIONAL LIST1 of exactly3 IfcParameterValue, not a vector direction entity and has no EXPRESS unit-length rule. No formal TFS WHERE establishes equal counts of coordinates, normals, mappings and triangles.

HTML explicitly states raw CoordIndex indexes Normals; point lookup uses either the same raw index directly or PnIndex[raw] into Coordinates. It explicitly permits duplicate point collections to represent different normals. Therefore normal lookup must occur BEFORE mapping; exact-coordinate geometry validation may canonicalize private keys but must preserve source rows and normal associations. Any output sharing must respect distinct normals. Do not normalize/mutate the source or silently weld by epsilon.

Validate all present mapping values, including unused entries, as PositiveInteger and actual coordinate locations; all present normal/point rows retain finite numeric triples. A referenced raw normal index must exist. Additional valid unused rows are represented as projection-policy admission, not certified full informal conformance: the HTML's illustrative N-count sentence is not an EXPRESS equality rule and must not be transformed into a fabricated universal cardinality check. Missing optional values differ from present empty aggregates. Logical.UNKNOWN is not optional IfcBoolean.

The matrix's integer coordinate case reflects REAL numeric-domain acceptance; REAL-kind index carriers remain invalid PositiveInteger even if mathematically integral. Its kind fields instruct the builder to construct explicit central value variants rather than relying on JSON numeric spelling.

ClosedTRUE activates released BRep constraints: unique faces, every edge used exactly twice, topological normals away from material. Exact coordinate aliases must be used for geometric identity while original source IDs remain intact. ClosedFALSE and omitted Closed do not authorize silently imposing a global solid/intersection/manifold policy; validate declared triangle shapes and nondegenerate geometry, then preserve surface geometry. A crossing-open-surface policy needs its own primary evidence, not reuse of a stricter validator by convenience.

## Existing Reader and Control Plan

Current manual branch in IO geometry.ts reads flat slots, maps coordinate values, accepts real-kind numeric indices through ifcNumber, ignores Normals/Closed and does not reject unused malformed PnIndex locations. Move it into one owner-query cooperative cursor; admit every source-sized coordinate, normal, mapping and triangle row under the same captured original controls. Preflight position/index/normal output capacities before growth; carry exact source entry/identity rather than build a shadow ordinal index. Keep initial/interior/completion original callback failure and cancellation gates and failed-item publication boundaries. The glTF importer currently authors Closed unset, which must remain distinct from a proven closed-solid claim.

## Independent Originals

Actual installed IfcOpenShell0.8.5 parsed four unchanged original ADD2 files and full EXPRESS returned zero statements for each. Original SHA/entity IDs/counts/Closed/Normals/PnIndex and coordinate-index maxima are in `generated/tfs-admission/originals.json`, authored by `tfs-admission-independent.py`.

- triangulated-item: TFS1021,8 coordinates,12 triangles, ClosedTRUE.
- slab-tessellated-unique-vertices: TFS41,12 coordinates,20 triangles, ClosedTRUE.
- straight and curved I-shape originals are recorded with their original indices and duplicated coordinate populations.

These are physical/formal observations, not an independent proof of all informal closure/mesh requirements. No native execution, full MVD, normal rendering fidelity, or generic tessellation completeness is claimed. New neutral formal oracle results are kept separately by `tfs-admission-neutral-oracle.py` so deliberately malformed constructor/schema inputs are not described as valid physical geometry.

## Neutral Formal Replay

`generated/tfs-admission/neutral-formal.json` is the completed independent installed0.8.5 replay of all32 rows, constructing replacements on a fresh parse of the unchanged original triangulated-item graph each time.29 constructors succeeded,3 refused;11 constructed rows produced full EXPRESS diagnostics. Construction/schema diagnostics and the planned geometry-policy decision are deliberately separate. In particular, ClosedTRUE open/inward/duplicate geometry can have no formal diagnostic; a REAL index may be rejected at the Python typed constructor rather than become physical STEP. No authored neutral geometry is declared valid from this formal pass alone. All replay processes have terminated.
