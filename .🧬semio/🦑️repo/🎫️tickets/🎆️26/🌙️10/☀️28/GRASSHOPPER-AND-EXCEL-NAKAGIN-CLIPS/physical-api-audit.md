# Physical Casing / Interior API Audit

Read-only source/permanent-test review during final integration. No production edits. Inputs audit-interior-components.py and generated interior-component-audit.json / audit-physical-door-ports.json retain the bounded checks.

## Sound Contracts

capsule_casing preserves authored centre by symmetric allowances: reference physical2.60×4.20×2.50 occupies X±1.30,Y−.10..4.10,Z0..2.50 around the nominal4m local frame. capsule_solid is explicitly the smaller nominal analytical envelope, while slot_solid builds the **physical** casing envelope, physical glazing midplane and physical attachment bolts. The independent route physical_barriers uses slot_solid; no hidden nominal collision shrink was found.

capsule_entry places back or side doorway frames on physical casing faces. Explicit scalar transforms of all140 current entries, including all33 corrected cases, give maximum point residual1.11e−16m against authored ports, maximum core-plane offset4.44e−16m and normal dot1.0. This independently uses slot axes and core half-side rather than model.place. It verifies world frame alignment; actual open aperture meshes still need their permanent volume/clearance tests.

The permanent casing tests compare actual shell bounds with independent native-derived fixtures, use Rhino transforms on all140 entries/envelopes, inspect actual lining/floor surfaces and test every actual body vertex against the conservative envelope halfspaces. These are meaningful independent geometric assertions, not just two calls returning the same value. Reference interior overrides preserve explicitly supplied interior dimensions; missing controls derive clearances. Current usable default2.30×3.80×2.10 runs Z.20..2.30.

The post-ceiling-fix read-only sweep found **zero out-of-usable-bound vertices in18 reference cases** (three control/override specifications × six families). Ceiling light and smoke detector now mount at the actual2.30m ceiling. This containment result does not establish every internal mount/support.

## Historical Pre-Fix Proof Limitations

This section and the following gap table describe the pre-correction snapshot. Saved fixes and independent results appear under Saved Correction Audit.

The existing “rounded-box SDF” body test evaluates only X/Z. It cannot distinguish a constant rounded X/Z extrusion from a full three-dimensional rounded box. Execution independently measured actual native triangles:1283 points lie on the radius.25 Minkowski surface with core half-extents1.05/1.85/1.00 and centreZ1.05;773 genuine longitudinal-edge/corner witnesses disprove extrusion. Example(−1.05,−2.02677679,−.12677670) has true3D SDF≈6.7e−8 but extruded XZ SDF−.0732233. The physical skin/lining must model the recoverable Y-edge/corner fillets and add a full3D SDF/native witness regression. A larger conservative extruded collision envelope can remain if actual-body containment is independently proved.

Current support tests assert selected floor/ceiling contacts. They do not prove every media/sanitary/mounting component is physically connected. A negative AABB contact graph found disconnected clusters. Positive AABB overlap would not prove actual support, but a disconnected AABB cluster cannot have mesh contact.

## Concrete Support Gaps Found

Default reference dimensions before the requested support correction:

| Part / intended neighboring structure | Actual gap |
|---|---:|
| CRT screen outer ring to bezel |3mm in X (.733→.736) |
| Audio console rear to cabinet |2mm in X (.778→.780) |
| Speaker cone rear to cabinet |16mm in X (.764→.780) |
| Speaker grille to cone |5mm in X (.734→.739) |
| Shelf guardrail underside to shelf top, with no posts |19mm (1.984→1.965) |
| Tub bowl bottom to wetroom floor top |45mm (.290→.245) |
| Rear trap/waste pipe endpoint to wetroom rear wall |5mm in Y (.230→.225) |
| Flush button to cistern face |10mm in X (−.830 versus−.840) |

Shower riser/hose groups also lack a wall bracket/contact path. These are explicit modeled gaps, not speculation about concealed machinery or engineering capacity. Minimal real mounts/continuous molded supports with inferred provenance should join the existing parts without removing cavities/grille openings or weakening containment. Root/execution were notified directly. The graph includes all reference parts and floor/wall/ceiling anchor planes; it is a necessary negative-contact diagnostic, not a substitute for a material-contact or structural analysis.

## Current Acceptance Boundary

The saved full3D skin/lining and mounts have now been independently reread and tested below. Current window mechanism integration, whole-model gates and fresh integrated cutaways remain pending. No old visual/native pass is promoted across geometry edits.

## Historical Actual Lining Material Findings — Corrected Below

The AABB-contained reference geometry still intersects actual rounded lining material at wetroom floor/enclosure bottoms, bed-front-rail ends and media-wall base. Ray crossing actual lining cap polygons confirms strict fixture-volume probes(−1.119,.50,.201),(−1.119,2.82,.201),(1.1099,1.50,.2001) lie inside lining. This is not merely a usable-box/SDF approximation. Root owns clipping bottom contact profiles to capsule_interior_profile without raising complete fittings.

Execution has fixed the independent explicit-interior corner defect: former rr.01−lining.015 yielded a negative inner radius and four self-crossing caps. An explicit clear inner corner radius.05 now defines the lining profile independently of the outer casing gap; permanent simple-polygon/Rhino/profile tests were added. Full3D casing material/jamb gates are reported16/16 passing, including steel and lining aperture rays across all six families with positive outside-jamb controls. Their subsequently saved mounting/material tests are independently checked below.

## Saved Correction Audit — Current State

The full3D shell and curved jamb corrections are now saved. Execution reports16 casing tests passing, including real steel and lining rays at five heights and centre/near-jamb positions across all six families with outside-jamb positive material controls. The former extrusion-only proof limitation is resolved by native longitudinal-edge/sphere witnesses plus full3D surface/containment checks; this audit does not substitute the conservative collision envelope for physical skin.

Independently reran the saved reference-mount module:4/4 tests PASS in12.943s. The portable fixture contains39 named connection rows (some connect several material pieces), across six families and two control specifications. Its NumPy point/triangle and segment/triangle checks verify actual surface contacts after family adjustment, handed reflection and usable mapping; overlapping bounds are only an early rejection filter. Mounts are closed positive solids with explicit assumed provenance. Speaker-driver positions derive the already-adjusted cone, avoiding the J/L/S shifted-speaker error. Tube/ring supports preserve grille, sanitary and acoustic voids.

The cavity correction clips actual fixture polygons against the clear lining profile, creates closed cut caps and preserves floor datum instead of lifting all fittings. Every tested fixture vertex satisfies the convex faceted clear profile; strict independent material probes at the prior wetroom-floor, bed-rail and cabinet penetrations are now empty. Floor/base contact is retained where the clipped flat interval remains. This resolves the concrete material crossings and listed mounting gaps; it is a geometric connection proof, not verification of original concealed fastening design or structural capacity. The selected two controls are not a proof of arbitrary invalid user dimensions.

Source-visible long wall lamp and fan/window-unit restoration are now saved and materially verified in their dedicated reports; fresh integrated cutaway review remains pending separately. No old native/gallery pass is carried across these source edits.

### Independent Full3D Internal Containment Check

Recounted the current portable fixture:39 connection rows, including the four mixer/leg additions already present in the fresh run. Independently reran test_all_internal_components_stay_within_actual_3d_casing:1/1 PASS in0.996s. It evaluates every actual vertex of shell-adjacent floor, lining, reference fixture and steel-truss parts for all six families against the independent NumPy rounded-box SDF (centre[0,2,1.25], inner half-extents[1.05,1.85,1.00], radius.25). Only enumerated entrance/attachment external hardware is exempt; no truss prefix is exempt. Because this rounded body is convex, containing every facet vertex also contains its planar triangles/edges. This proves no truss spikes outside the physical casing; it does not by itself prove trusses avoid the usable cavity or every intended aperture. Dedicated actual material doorway rays and window/lining tests provide those separate checks. The cavity profile test independently matches profile vertices against actual lining-corner mesh vertices across three control specifications, rather than trusting only the clipping helper's returned polygon.

## Zero And Tiny Radius Final Reread

Independent targeted radius-control rerun:1/1 PASS in11.142s. Portable values0,.0005,.0012,.125,.25m exercise all six full-detail families. Every nonexempt actual component vertex remains inside the physical rounded box under an independent NumPy3D SDF, and real steel patches remain valid/closed in Rhino. Source now retains finite truss section cover even at radius0: max(radius/2,member/2+skin), with corresponding end cover, instead of centring full members on the sharp outside boundary. Floor build-up and seam locations likewise retain finite insets. This verifies the authored sharp-box and tiny-radius demonstration stages without treating source-default.25 as a universal minimum. External attachment/entry hardware remains explicitly exempt for its actual purpose.

## Exact Perforated Boundary Repair — Independently Verified

The black front-corner slots found in the baseline are repaired in current source. Independent actual-geometry checks recover all four flat front corners with zero coordinate residual in exterior16 and full48 LOD. Summed actual outer cap triangle area differs from the independently calculated rectangle minus unchanged regular aperture polygon by8.88e−16 and2.22e−15m² respectively.

_perforated_profile preserves every exact outer-profile vertex, inserts aperture-ray edge fractions into both corresponding loft boundaries, and leaves the inner N-angle aperture polygon unchanged. Sector cap chains include crossed boundary corners; only actual outer and hole sidewalls close the material, avoiding internal radial walls. The lining now derives its end perimeter from the same actual quarter-band profile instead of regular-ray shortcuts. Directed edge incidence/balance, positive volume, Rhino closure, exact cap area/volume and material rays are portable regressions across six families, both LODs and authored radii. Source canonicalization keeps clipped shared boundary coordinates consistent.

Execution's stable physical135 and root structural52 share the recorded model AST87255d8965e795fc9c5a00fed16fb8de67184b94552a72006f5d4959cdbaa084. This resolves the concrete physical corner-coverage finding; it does not promote old before-panel renderer controls. After-panel unsafe-LOD/all-source-face coverage, fresh images and native acceptance remain separate pending gates. No additional physical source defect was established by this reread.

## Fifth Rendering-Only Input Scope — Independently Verified

Reread every actual JSON read and local dynamic import in all eleven physical domains, plus the model loader. The model reads its adjacent model JSON. Each domain consumes its own portable case/model data; panel reuses casing, unit reuses mounts, mounts reuses circulation, and light reuses mounts. None imports the renderer or reads visual/occlusion fixtures.

The acceptance filter is exact membership exclusion of five individual paths: renderer Python; occlusion case JSON/test Python; visuals test Python; and the newly added visuals case JSON. It uses no wildcard, prefix or directory exclusion. The fifth JSON is therefore outside this physical dependency boundary; excluding it does not exclude physical cases or permit future unknown file drift. The earlier strict failure correctly retained it until reviewed.

Independently read the current acceptance record: success135, exact complete domain contract, unchanged/current consumed maps, SHA256d6a698db05c1459dec7538f2b62e9784ac1db4aaead3176ef23a532e38f3f4dd. This verifies only the physical-versus-rendering scope boundary; rendering, native and final visual acceptance remain separate. No tests were rerun and no production/acceptance source was changed by this review.

The subsequent path-only portability correction canonicalizes manifest keys to forward slashes in memory and future writers; the original raw135-run evidence remains untouched. Independently confirmed five exact canonical paths and current success. Current canonical physical map SHA256: 4307178a632167490a69cf730bb869333bb79ca47d63b79a7972f6206f4820af. The older d6a698db map hash above is the pre-normalization representation; dependency scope and physical inputs are unchanged.

## Additional Exact Rendering-Only Scope Paths

Re-read all11 physical domain entrypoints and actual helper imports/read_text calls after root’s candidate/refinement schema+CJS change. They load sharedM/modelJSON and their own fixtures; panel→casing, unit→mounts→circulation, light→mounts remain the only reused physical helper chains. None consumes the renderer occlusion schema or JavaScript validator. Adding precisely these two paths to the existing five-path physical evidence exclusion is legitimate: 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/👁️occlusion/🔣️.schema.json and 🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/👁️occlusion/🟨️.cjs. This produces seven finite exact exclusions, with no directory/wildcard scope. Contract update and current recheck remain root-owned; no new acceptance hash is asserted before their result.
