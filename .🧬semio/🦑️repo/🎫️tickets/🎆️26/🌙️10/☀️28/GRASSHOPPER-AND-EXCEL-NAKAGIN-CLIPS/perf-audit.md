# Read-Only Renderer Performance Architecture Audit

No duplicate heavy process, profiler restart or production change. The existing cold/warm profile remains owner-managed and running; elapsed time alone is not a reason to terminate it. Findings below are concrete source costs, not measured attribution of the live process's dominant stack.

## Actual Inputs And Bounded Estimates

Current after-panel export has2368 source Items and470 retained for the inspected A/Slash view. Independently counted retained world-vertex entries3358 and179 distinct normals rounded to the renderer's10digits. This is a single-view exemplar, not every family's/tower's exact count. Repeating it140 times estimates65,800 retained Items and470,120 vertex entries before cores/heads/podium. The older model-complexity.md source fingerprints/counts are historical and must not be treated as the frozen current model's census.

## Actionable Hotspots

| Source path / operation | Concrete cost | Safe direction to investigate |
|---|---|---|
| _section_order lines659–666, before sweep | Dense minimum/maximum arrays sized visibleItems×uniqueNormals; all vertex entries project against128axes per block. Exemplar-scaled140 gives≈481MB decimal for one float64 projection block alone;179normal interval tables add≈188MB, with actual global rotated/core axes potentially more | Broadphase candidate pairs before support projections, or lazy exact pair-axis intervals; reduce block peak if live profile confirms memory pressure. Preserve the exact physical ordering constraints |
| _section_order sweep lines668–684 | Rebuilds active Python list per item, then copies it to NumPy and appliesY overlap. Many long projected wall/facet bounds keep a large active set; potential pair cost approaches quadratic | Reuse projected spatial bins/interval structure for candidate generation, preserving exact overlap/separation checks; do not remove source geometry |
| _section_order cycle cleanup lines685–689 | Broad bbox/support-plane edges are created first; exact polygon-overlap tests run only inside SCCs. Large false-overlap SCCs cause many nested polygon comparisons/allocations | Profile SCC sizes/edge counts in existing owner run; reject clearly nonoverlapping projected pairs earlier or cache exact pair overlap, without centroid-based order waivers |
| _assembly_items per140slots | _world transforms each part; each face builds polygon arrays/buckets and per-item intrinsic hashes before local opaque culling. The2368source Items are reconstructed for repeated family/orientation even when visibility repeats | Cache pure local mesh/Item geometry and view-orientation visibility. Transform only retained draw geometry while keeping all source/semantic part inventory and fresh scene objects |
| _opaque_visible | Re-prepares projected polygons and clips padded fragments against all candidate cover pieces. Actual cover subtraction may fragment heavily; large polygons enter a global candidate set | Cache geometry-only prepared coverage/retained indices by canonical family/orientation/view/padding; monitor fragment/candidate counts. Never infer occlusion from role/part size |
| tower_axon lines1182–1187 | A cold cache miss constructs a full rich template, then immediately copies its entire Mobject tree. Cold timing therefore includes both; warm timing mostly exposes clone/transform cost | Keep the live cold/warm profile. Instrument build/copy separately only in an authorized subsequent run; consider immutable geometry/order templates versus retaining/copying complete rich scene trees. Do not return a mutable cached template directly |
| _TOWERS capacity8 | Keys include selected/tinted/included slot sets, so live teaching states can retain eight complete large building trees, in addition to order/part caches and returned copies | Separate geometry/order cache identity from palette/selection styling where semantically valid; bound rich-object cache memory by observed size rather than blindly removing correctness-preserving state keys |

## Existing Sound Reuse And Constraints

_capsule_parts caches source meshes by dimensions/family; core/head/podium/bridge source parts are cached. Opaque visibility caches retained indices and returns current Items, avoiding old semantic/palette objects. Order caches store index permutations. These are useful and should not be discarded because the cold path is slow.

Any optimization must preserve physical source faces, actual aperture boundaries, current geometric ordering (including the known frame/glazing counterexample), current semantic objects and transparency. No geometry-role filter, proxy body, arbitrary pixel tolerance or runtime third-party dependency is recommended. Requested final-scale stroke padding and later points-only transforms remain part of visibility correctness.

## Sparse Support Replacement Source Review

Root's _candidate_support replacement is mathematically equivalent to the former dense interval lookups: the repeated starts-minus-local-offset indexing reconstructs each candidate's actual vertex slice, then returns its min/max on the second face's representative normal and the second face's min/max on each candidate normal. Substituting these four arrays into forward/backward yields the same predicates and separation error bound. Unknown normals and empty candidates retain the former behavior; the empty/background guard returns original background Items without reductions on empty axes.

The N×allAxes global min/max tables and allVertices×128 projection blocks are removed. Remaining own-axis projection is linear in vertex entries; candidate temporary size is bounded by at most128 candidate axes and that pair block's actual vertices. This is a candidate-count bound, not an absolute byte cap when an individual polygon has many vertices. Existing actual vertices/axes/ordering semantics are retained. Dense NumPy portable cases and red→green results are recorded in sparse-order-support.md; no measured full-tower speedup is claimed before the live profile completes.

## Full-Core Source/Item Census — Completed Without Culling

Retained input audit-core-census.py and generated/core-census.json measure actual full cores at the default axon view, with unchanged model/data/renderer byte fingerprints before/after. Terminal exit0. No _opaque_visible, _section_order, Cairo capture or full tower was called.

| Core | Parts | Source faces / Items | Facing Items | Facing vertex entries | Distinct facing axes |
|---|---:|---:|---:|---:|---:|
| A |1098|7466|3727|15786|29|
| B |948|6433|3211|13589|29|

| Core | Source assembly s | Mesh conversion s | Item construction s | .25-grid entries | Maximum bucket | Duplicated bucket-pair occurrences |
|---|---:|---:|---:|---:|---:|---:|
| A |.132|1.785|.668|59468|40|460346|
| B |.116|1.706|.544|51942|42|407528|

Neither core has an Item above the culler's2048-cell large/global threshold; maximum bbox cells are1256/1080. Occupied cells5179/4669. Bucket-pair occurrences duplicate the same pair across cells and are not exact intersections or a measured cull workload. The census does not establish culling latency.

Actual source/mesh/Item construction takes about2.59sA/2.37sB in this one bounded run, so those phases alone do not explain the observed300+second cold stall. With the live profile still before its global-order marker, actual core opaque coverage/fragment subtraction is a plausible next phase to instrument, not a proven stack attribution. No duplicate expensive culling was attempted, and the live profile was retained unchanged.

## Root Candidate And Refinement Delta

Independently reread _overlap_candidates and both consumers. It yields outer-first/inner-second candidate indices in the original order. Strict bounds maximum>otherlower+margin and otherupper>minimum+margin exactly complement the former<= rejection; inclusive>= complements the former< rejection. Existing1e−10 positive-overlap margin and inclusive narrow-plane bounds are preserved. No new tolerance or geometry approximation is introduced.

Post-sweep refinement snapshots the current cyclic components, counts their directed intra-component edges, then reports actual completededges at0/128/end. Removal affects only the current source’s outgoing edge set, so total accounting remains valid. Cancellation is checked before and after callbacks. The bound is directededges, not wall time or inner polygon-pair count. Root reports observed2reds then4combined tests PASS0.282s, including SciPy candidate oracle, NetworkX cancellation, existing Pillow overlap and actualplane/Cairo controls; AJV14invalids. This source review does not resolve the separately proven strict-kernel split-fragment no-progress defect.
