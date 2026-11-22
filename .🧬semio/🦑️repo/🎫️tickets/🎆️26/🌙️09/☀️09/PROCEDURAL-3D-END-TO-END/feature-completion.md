# Procedural 3D implementation and verification

## Active verification

- [Canonical browser and native run status](./📓️canonical-procedural-run-status.md) tracks both renderers and the exact hover probes. [The Cargo planning watchdog change](./📓️compiler-planning-watchdog.md) is awaiting the corrected WGPU launch.
- [Retained Section](./📓️disconnected-planar-section.md) atomically publishes disconnected native faces. [Cavity sections](./📓️disconnected-void-section.md) cover separate holes in one cavity and a material island inside an annular cavity. The [through-hole fixture](./📓️through-hole-section.md) also covers faces with inner loops in one connected shell. Compact native, independent Manifold and all 23 linked analysis checks pass. The linked Flow Section job remains open.
- [Mesh Boolean expansion](./📓️mesh-longitudinal-prism-booleans.md) covers [multiple intervals on one wall](./📓️mesh-disconnected-longitudinal-supports.md) [grazing contacts](./📓️mesh-grazing-longitudinal-supports.md), and [reflex contacts](./📓️mesh-reflex-longitudinal-support.md). The authored façade, UV and material preservation, closed reimport, and Manifold checks pass on the covered supports. More general operand arrangements remain in scope.
- [Current UI and hover audit](./📓️ui-final-source-audit.md) found no additional confirmed source defect. Verification still needs both mounted browser journeys.
- [Remaining retained BRep routes](./📓️remaining-retained-brep-routes.md) and the [selected route audit](./retained-brep-route-audit.md) track Flow admission and geometry boundaries. [Heal](./📓️retained-heal.md) and [guided straight Pipe](./📓️retained-guided-straight-pipe.md) have authored retained routes with scoped independent geometry checks. The linked retained regressions are running.
- [DWG exchange](./📓️dwg-retained-exchange.md) records the object, block and page codec changes, their independent acadrust checks, and remaining retained work.

## Geometry

The executable playground obtains its geometry operators from the Flow BRep extension. The procedural inference catalogue has separate tests. Results for one catalogue do not establish coverage of the other. The initial live contribution descriptor declared 142 operators.

## Preview and interaction

The editor and viewer resolve a hovered wire or connected inlet to the corresponding evaluated source channels. Graph outline rows publish node, handle and edge hover targets. React and wgpu tree handlers publish pointer exits for shared interaction hover actions. Preview collection visits structured geometry and uses sink inputs when outputs are empty. The editor parses the evaluation once and reuses published meshes by handle. Graph status labels parse once per outline and localize stale values.

The inspector slider uses the shared ranged control, including keyboard increments and localized names. Large texture rosters use the framework's windowed tree so users can reach late textures. The wireframe fallback, BRep work, and mesh work have dedicated tests.

## Verification

The authoring and inspector TypeScript suite passed 372 assertions, including the English and German range-control checks against jsdom and Testing Library. The shared React interpreted-tree group passed 19 tests. The native WGPU retained pointer hover routing test passed. Product native and browser verification are pending. The initial native dependency build stopped in semio-plugin-tool-machine. A separate shared-framework compilation is checking the wgpu event router.

## Live evaluation blocker

The live BRep registry has 99 non-mesh operators. Ninety-six request immediate evaluation, which the retained Flow SDK rejects before invoking the kernel. The three Boolean job operators also lack retained output publication. The geometry agents are investigating the required resource ownership and cancellation paths. See the closed-curve and mesh reports in this ticket for operator-specific evidence.

The torus and UV sphere now share an incremental grid producer across typed inference and the live Flow registry. The workbench fixture checks closed topology and volume through the contributed evaluator.

The native WGPU retained hover routing law and all 19 React interpreted-tree tests passed. Spatial vector markers now have a directional head and a visible origin mark for zero magnitude. The packaged schema and independent Three.js checks passed for torus and UV sphere. Line, Circle and Ellipse now have original-caller publication implementations. They await native verification. The remaining BRep inventory includes unimplemented original admission for extrusion, Boolean publication and the broader operator set. The playground build failed during repository graph resolution before a page could launch.

Packaged mesh coverage now agrees with the typed catalogue for every declared live mesh operation. The catalogue suite passed 44 tests and 86 assertions, and the bilingual terminology checks passed 4,007 assertions before the OBJ label addition. The independent analytic-curve fixture satisfied its Three.js chord tolerance checks. The original session now has a path to reserve one edge buffer, write one curve segment per funded turn and transfer ownership on completion. The connected native test is pending.

The inspector now uses the shared wrapping vector-input component for points, vectors, plane origins and normals, and ordered point/vector items. The updated DOM and command suite passed 388 assertions. The native compiler failure in the shared tool-machine follower was traced to imports of NativeOwnedStorage and NativeSeedTemplate. The follower now depends directly on their owning storage crate. The editor-panel test suite has been restarted.

Graph-canvas wires now publish edge targets through React Flow and WGPU, and the graph can resolve an externally hovered edge. The neutral graph fixture exposed aliasing between wire e1 and the first node. Rebuilding the DAG now assigns separate internal IDs to nodes, ports and edges while preserving their authored IDs. The React publication suite passed 19 tests and 58 assertions. Native graph verification remains queued.

The mesh descriptor now agrees across 53 operations and four packaged copies. The mesh oracle passed six tests and 3,757 assertions, and bilingual terminology passed 4,145 assertions. A native typed-mesh run caught a missing data port in the invalid-input refusal. The owner corrected the refusal and launched a rerun.

The channel tree now represents nulls, arrays, maps and primitive values, with bounded summaries for large values. Generate mode clears stale channel readouts when no generation is selected. Funded BRep validation now accumulates shell incidence one coedge at a time and checks UV winding one sample at a time. An independent Three.js shell oracle covered a closed tetrahedron, a missing face and a reversed face. The six-test validation group passed 234 assertions. Later general-solid validation phases, end-to-end browser playback, and the native compiler runs remain open.

## Latest implementation and verification

The retained registry supports exact affine images for lines and planes, similarity transforms for conics, and certified cylinder transforms that preserve circular cross sections. It also supports copying certified solid primitives. The transform results keep the existing geometryOut output port. The affine oracle passed seven tests and 34 assertions. The native fixture drives the original registry and samples the new published handle.

Certified primitives now expose analytic area and center of mass. The same incremental cursor used for planar volume now integrates planar polygon surface area, using one coedge per turn and subtracting inner rings. The independent Three.js fixture checks volume, surface area, and centroid. The combined solid and topology run passed nine tests and 252 assertions. The focused graph hover test remained at 19 passed. Tree-sitter parsed the 18 native owners reviewed in this pass.

The native compile logs exposed retirement API drift in the BRep retention tests and generic snapshot staging. Both callers now pass the required original owner or requested body budget. Earlier sealed native attempts also saw the old primitive preview export and label formatter. Current sources include both. These fixes still need a compiler run against the current source tree.

The cache collector acquired the exclusive Bazel session collector lease before waiting for the storage image. It now acquires the image first. Both the available and busy collector regression cases passed. The repository doctor reclaimed approximately 1.2 GB of abandoned publications and removed obsolete lease records. The retained storage image remains held by an existing setup process. The host continues to report roughly 1 GiB free. A full native rebuild, current test fixtures, and the interactive browser journey still require recovery. The repository’s Rust cache reclaim task is live.

Full BRep parity still needs funded Boolean topology and publication, broad solid and surface operations, spline geometry, and curved validation and tessellation outside certified shapes. Native mesh and playground verification are still pending. The goal and ticket remain active.

The native shell-incidence test completed and passed. A funded conic face now produces a shaded triangle fan with a closed boundary and exact analytic area. The Three.js tessellation oracle passed two tests and 14 assertions, including independent circle and ellipse disk areas. Dedicated native tests now cover the budgeted preview and the original registry path from ellipse to face, area, and tessellation. They await fresh native verification.

The 3D playground process remains live and has not opened its browser listener. The Rust cache reclaim action failed at the storage reserve. The Nx cache pass freed no space. The cache storage lease belongs to another working setup process. The host is now below 1 GiB free.

Wireframe and points now remove component references and auxiliary attributes for streams they hide. Points reuse edge coordinates when a standalone curve has no vertex buffer. The shared mesh fixture covers the resulting edge and vertex references.

## Rational geometry

The original curve queries now evaluate position, tangent, and curvature from the active rational knot span using fixed scratch. Original surface queries evaluate rational tensor-product points and analytic partials. Same-parameter validation uses the original p-curve, surface and curve evaluators, including rational forms. The portable rational curve, patch, and validation suites passed eight tests and 303 assertions. A separate scalar check matched all 63 fixture components to the Three.js curve jets.

Registered circular arcs now construct a rational control lattice one funded event at a time and publish the curve through the same live geometry index as other analytic curves. Retained spline preview visits each knot span twice to reserve the exact edge buffer and emit the chord samples. The subdivision count bounds chord error using local derivative control hulls and positive weights. At repeated knots it samples both authored span limits. Native fixtures cover curve jets, surface normals, rational same-parameter validation, chord tolerance, arc publication and cancellation. Tree-sitter parsed all 16 edited Rust sources with valid UTF-8. The native suites still need a current compiler run.

The code supports original scratch up to degree 32 and refuses previews that would exceed 200,000 segments. NURBS face tessellation and later curved-body validation phases remain open. The native build host still reports less than 1 GiB free and the authorized playground process has yet to open its listener.

The combined latest rational-curve, rational-patch, validation and tessellation oracle run passed ten tests and 335 assertions. The repeated-knot fixture now verifies separate left and right limits at a discontinuity. The native witness requires exactly the two legal edge segments, with no bridge across the gap.

## Current interaction and verification

The editor now projects vertex, edge and face hovers to the owning graph output while a component tool is active. Graph hover remains visible in that mode. Wire hover now carries an explicit edge identifier through the graph scene, Flow WASM argument contract, React ledger and native WGPU graph. A wire with an empty source-port field highlights the whole source dictionary, matching Flow's existing transport semantics. The graph publication suite passed 19 tests and 60 assertions. Twelve edited hover sources passed syntax and whitespace checks.

The renewed solid work introduced retained exact Boolean paths for certified boxes, coaxial cylinders and centered sphere containment. The two Manifold fixture tests passed 12 assertions. The general Boolean topology path remains incomplete. The polygon work now includes a shared retained native wire owner for rectangle, regular polygon and polyline, with face and extrusion work continuing. The current mesh fixture runs passed 64 tests and 4,093 assertions.

The plain repository Rust reclaim action reached the idle-image compaction path. macOS refused to detach the volume because a pre-existing teaching proctor still has its executable mapped. The process began before this goal and has live sockets. The latest typed mesh and workbench attempts stopped during disk admission at 1.2 GiB and 462.94 MiB of free space, respectively. The authorized playground process remains live, but the listener is still closed. Test evidence above is scoped to the commands that actually ran.

## Planar and bore preview

The retained planar preview visits one native coedge, allocation, crossing check, or triangulation event per grant. It handles concave single-ring faces, planar shells, prisms, and the outer and cavity shells of certified rectilinear results. Shared edges use one render segment and one persistent pick ID. The face winding, normal, and vertex IDs follow the published topology. Uncertified solids still pass the existing validation gate.

Conic disks now publish persistent face, rim edge, and seam vertex labels. The through-bore preview uses the exact box corners and common cap, wall, and rim samples. It goes through the same live provenance check and budgeted transfer as the existing analytic primitives. Native registry fixtures cover both previews, including rendered bore volume and component table completeness.

Three.js passed three tessellation tests and 23 assertions. The bore sampler passed a separate 100-case numerical comparison, with a worst relative volume error of 0.078 percent. Rust syntax and scoped whitespace checks passed on 12 preview owners and fixtures. Native type checking and browser interaction still need verification. The latest host readings fluctuated between about 0.15 and 1.3 GiB free against the repository's 15 GiB build floor. A pre-existing executable mapping still blocks Rust image compaction.

Remaining work includes live Wire/Edge preview, general holes and curved or spline surface previews, general Boolean imprint and sewing, broader BRep operators, and full product verification. The registry was split into its own native crate during this work. The preview adapters have been updated to read the new registry ownership paths.

An isolated Rust harness compiled the actual planar cursor against a small in-memory topology contract. The neutral fixture passed for concavity, both windings, a collinear corner, persistent picks, and rejection of self-crossing and backtracking boundaries. This exercises the cursor but does not substitute for the native workspace suite.

## Wire, sweep, and rational surface previews

The retained Wire and Edge paths now handle line and trimmed circular or elliptic members in both orientations with stable native edge and vertex references. The retained circular Revolve path publishes a positioned torus for one complete turn around an external coplanar axis. Manifold validated its volume, bounds, and genus in eight assertions.

Standalone rational surfaces now preview over their authored knot rectangles. The retained job bounds tessellation error from homogeneous derivative hulls, reserves one buffer per grant, and emits one triangle per turn. Neighboring continuous patches share their rendered seam points. Full-multiplicity knots keep distinct left and right limits. The independent Three.js surface suite passed three tests and 41 assertions. The isolated Rust harness compiled the actual rational jet, patch-bound and sampling code and passed both chord error and disconnected-span checks.

The retained Nurbs Grid publication now fits small sample grids using the same interpolation objective as the synchronous solver. Its live fixture and full product tests still need the native workspace run. The browser launch reached more than 11,000 owner checks without starting a listener. The host storage floor still blocks native workspace verification.

## Planar trims and hover clearing

The planar tessellator now bridges multiple disjoint inner rings under caller grants. It samples exact line, circle, and ellipse edges at a chord-error bound, then keeps each curve's segments under one persistent pick label. The isolated Rust source harness passed three cases covering ordinary contours, two polygonal holes, and a circular hole. The six independent Three.js tessellation tests passed 27 assertions. A standalone vertex now uses the same funded topology preview path and carries its native pick label.

Reactive hover clearing is wired in React. Viewport exit removes the projected graph highlight. A newer pointer or keyboard hover takes precedence and delayed worker results cannot restore an old highlight. The React hover and publication suites passed 23 tests and 103 assertions.

The retained suite also now includes signed planar Thicken, exact planar conic Offset Face, 8×8 interpolated Nurbs Grid, and Coons patches with compatible low-degree boundaries. The native crate tests and browser playback remain open. The last port probe found no listeners on the intended playground ports.

## Analytic surfaces and projection

Standalone cylinder, cone, sphere, and torus surfaces now share the funded parametric preview cursor with rational surfaces. Periodic endpoints use identical samples. Sphere poles share their exact positions. The datum chart shows a cylindrical length of two radii and one positive model unit of a cone. The isolated surface harness passed the rational chord, disconnected-span, and finite cylinder witnesses. The Three.js surface suite passed four cases and 42 assertions.

Retained Curve Closest Parameter and Surface Closest UV now publish all declared channels under the original dictionary grants. Closest Point shares the bounded analytic projection. The nearest-point path supports lines, circles, ellipses, and analytic surfaces. Three.js passed three cases and eight checks, and the isolated Rust projection harness passed the same neutral fixture. General spline nearest-point traversal remains open.

The retained Bounding Box path now publishes exact 3D bounds for supported analytic geometry. Mesh Source vertices now use the shared accessible XYZ field, with the existing field keys and blur commits. The widget suite passed 405 checks. General BRep operations and full native and browser verification remain open.

## Scoped validation, grouped topology, and native interaction

The original preview now walks the selected native roots before validation. The validator schedules only their reachable rows, sizes its winding and shell-incidence backing from the selection, and transfers the first retained finding to the preview caller. An unrelated solid with reversed orientation cannot invalidate the selected valid solid. The isolated Rust harness passed the scope planning and compact-index witness. The portable validation, compound, tessellation, and surface run passed 18 tests and 309 assertions.

Retained Compound now collects one live solid root per grant and publishes a group without copying topology. The planar preview follows each member and preserves its face, edge, and vertex labels. The isolated planar harness passed four tests, including separate compound roots and both polygonal and circular inner boundaries. Compounds with unsupported nonplanar boundaries still reach the generic sampler.

WGPU now preserves a newer local graph-pointer hover on a delayed viewport update and clears a hover still owned by the viewport on exit. The shared React/WGPU witness passed five Bun tests and all seven isolated Rust transition checks. The native retained test awaits build storage.

The retained operation paths also now include Vertex, direct point-list Planar Face, least-squares Curve Approximate, certified solid point classification and box distance, and exact certified Box Shell. The latest registry declares 93 kernel-mapped BRep operations. Many general operation families still need retained publication and native coverage.

The idle graph-image reclaim ran through the repository owner and recovered zero bytes. The Rust image remains mapped by an existing process. Host free space remains near 1.4 GiB. The playground has no listener and the original launch has no surviving terminal receipt. Workspace-native and browser playback remain unverified.

## Live validation, analytic component picks, and build admission

Native validation records the selected original shape's findings through the existing text output. The job sizes one buffer for the exact escaped JSON payload, writes one scalar per grant and transfers the buffer to the output dictionary. The first finding retires a failed preview before sampling. The full report continues under the original caller until it has all supported findings or reaches its bounded output extent. The neutral serializer fixture covers quotes and control characters and the isolated Rust harness passed the real token writer.

Original validation counts shell memberships before allocating one contiguous shell frontier. It retains the first finding inline and grows later findings in separately funded fixed-size pages. This removed a 424-byte page-metadata copy from the first shell allocation, which previously stalled the native test with a 64-byte policy. The fixture keeps one-, three-, and 64-byte sampling widths and specifies an independent 4,096-byte grant for fixed retirement metadata. The full native validation run passed 28 tests. Three.js agreed with the 11 neutral planar triangle cases, including coincident intersection chords, coplanar overlap, isolated point contact, and shared edges. The full native run preceded the last three triangle witnesses. The retained pair pass covers planar triangular boundaries. General curved and polygonal self-intersection still requires native predicates and a spatial index.

Standalone original curves now attach one persistent reference to every rendered segment. Plane and surface previews attach one original reference to every rendered triangle. The references remain decimal strings even above JavaScript's safe integer range. Picking metadata has its own retained allocations and publishes with the mesh. Reflected sphere poles preserve the position and use the same handedness as the adjacent parametric samples. Three.js passed the seven-case tessellation suite with 36 assertions. Product confirmation of the curve and surface picks still requires the native tessellation and Flow runs.

The team also implemented straight Sweep and Pipe for selected line-path axial circles and planar line profiles, two-profile Loft for congruent translated sections, exact affine transforms for planar polygon faces and analytic similarity routes, and ordered Shells and Explode with original labels and retained aliases. The focused member reports record the independent fixtures and limitations.

Native Cargo can run again after disk space recovered. The Flow test reached the repository source-admission limit at 4,296 active lines for the native registry. The solid publication transaction now has a dedicated owner. The repository's actual source counter reports 3,844 lines for the registry. Fresh native Flow and tessellation checks are underway. The React launch is still inside graph inventory and has yet to expose a listener.

The validation native rerun with the extended triangle fixture passed all 28 tests. The portable validation and tessellation suites together passed 15 tests and 284 assertions. The tessellation library reached a successful compiler result in an isolated check. The repository declined its build receipt because the shared framework manifest changed during that run.

The Flow plugin reached its own source boundary after the native registry passed. Its retained geometry jobs now have a separate owner and the plugin source count fell to 2,844 active lines and 161,427 bytes. The linked Flow BRep package test is queued through the original repository capacity gate. Tessellation tests now have an integration target for public API checks, and five tests that inspect private owned backing remain in the library target. The package retains the same fixture inputs.

The React serve attempt was cancelled after about 15 minutes in graph inventory, with 774 owner checks and no listener. The graph process held about 2.4 GB while the native test and several cache maintenance tasks waited for host capacity. The repository’s canonical development serve owner resolves the same generation3d target. The unowned script entry rejects calls without the repository’s live invocation, so browser verification remains open.

## Linked build and selected geometry verification

The native Flow suite completed all 833 source-admission units after split ownership moved the DSL JSON Schema projection to the existing record façade and the shared preview reach cursor to BRep Engine. The DSL model uses 255,359 bytes against its 262,144-byte limit. The first tessellation compilation diagnosed private prism limits, the post-split Box Shell bounds representation, and retirement traits on fixed topology aliases. Those sources are corrected and a new tessellation run is queued. The entire linked Flow compile is still in progress.

The former Flow original-close integration source reached approximately 330 KB including its fixtures. The same 51 tests now build as four bounded test targets with shared original-caller helpers. The repository counts 41,937 bytes for lifecycle, 159,891 for curves and faces, 139,018 for solids, and 101,358 for queries and topology. No registered fixture was removed. The native tessellation private and public targets also fit the source gate independently.

The standalone Shell selection keeps legal boundary edges. Solid preview still requires each boundary edge to be paired. The Three.js validation suite agrees on open manifold incidence and closed solid incidence and passed eight tests with 250 assertions. The native test for the same selected source under the two root types is queued.

The graph now uses its local hover when an older component hover is still present during a cross-surface handoff. Both node-port projection and highlight IDs use that decision. Component selection remains intact while the old component hover is hidden. The jsdom focus and source fixture passed all six scenarios and 12 assertions.

Exact supported additions include original cylinder and cone side previews with one shared seam, planar positive-weight spline contours with continuous trimmed spans, certified primitive placements, per-label Deconstruct, and Box Section through the original faces list. The focused owners' reports retain their source harnesses and neutral geometry witnesses. General intersections, edge blends, arbitrary surface operations, BRep interchange, and browser verification remain open.

## Current verification state

The open-shell native validation rerun passed 29 tests. The DSL record façade's JSON Schema projection passed all eight native tests after the ownership move. The combined portable validation, tessellation and graph hover suites passed 16 tests and 298 assertions. Nineteen affected Rust sources parsed and the scoped whitespace check passed.

Production source counting shows 93 kernel-mapped BRep node IDs. The direct registration calls still route 20 identified operators through immediate admission, including helical sweep, compound cut, the three patterns, the six fillet and chamfer variants, draft, defeature, Sew, Heal, Convert to NURBS, and four BRep imports. The generated direct-operator inventory covers 66 registration calls and therefore leaves further catalogue entries to verify. Existing retained routes also have geometric input limits. Any catalogue-wide completion claim requires successful registered evaluation and preview through the linked runtime.

The linked Flow suite moved beyond source admission and into native compilation. The four original Flow integration targets have separate owners for lifecycle, solids, faces, and queries. The repository queued guarded graph cache recovery as free storage approached 3 GB. The native tessellation rebuild is still waiting for its capacity grant.

Original Closest Point now uses the authored range of circular edges, including ranges that cross the periodic seam. It compares exact endpoints when the nearest circle angle falls outside the trim. For a full circular or elliptical native face, it projects inside points to the disk plane and outside points to the conic boundary. Closest UV solves the original plane axes' dual basis so nonorthogonal or scaled coordinates map back to the exact 3D nearest point. The independent Three.js fixture passed five tests and 16 assertions. The registered fixture with imported native plane and edge roots is awaiting the queries integration build.

The latest exact scoped validation result remains 29 native passes, and the unrelated DSL JSON Schema repair has eight native passes. The browser launch remains unverified. At the latest disk check the host had about 3.6 GB free, and the original linked Flow build was still using four compiler jobs. The official graph and Rust cache recovery operations remain subject to the host scheduler and active image ownership.

The closest-point path also covers all five certified primitive families on their boundaries. Box projection chooses a boundary face for interior points. Cylinders and cones compare their lateral meridians with their end disks. Spheres and tori use their original analytic frames. All six Three.js projection tests passed with 26 assertions. The isolated Rust harness compiled the actual math and agreed with the registered witness for skewed planes, trimmed circular edges, filled disks, and ten primitive points.

The earlier linked Flow command reached Rust compilation, then reported an unresolved spline-trim import from source added after the command planned its dependency graph. The current manifests and the framework, Hub and Flow locks include the dependency. A fresh faces integration command owns that verification. The earlier tessellation command then refused admission at 3.46 GiB free against its 5.5 GiB floor. An official fresh tessellation command started after measured free capacity recovered above that floor. The authenticated idle graph cache was retired by the repository recovery command. The browser still has no verified run.







## Mesh packaging and preview provenance

The combined portable Flow mesh, modeling jobs, production tessellation and clipping suites passed 43 tests and 8,412 assertions. The packaging audit walks declared Cargo path edges from each of 31 workspace roots. It checks only the owners each workspace can reach, then compares their locked direct edges against the shared fixture and SQLite. This exposed nine stale source-hold edges and two stale mesh-document development edges, now corrected. The Space and GIS workspaces have no path to these mesh owners.

The native tessellation compiler reached both test targets and found retired test signatures, a missing named type, and a test helper left behind during target extraction. The tests now drive one caller-funded retirement effect while observing physical allocator receipts. The current native rebuild remains capacity-gated.

A pending channel now reads mesh fallback from the matching widget, channel and item in the retained convergence history. Editor and viewer deduplicate the actual geometry source, and a stale mesh does not publish a new analytic component identity. The viewer's paint signature includes the resolved handle and the cache epoch, so an equal-size live mesh replaces the fallback. The Flow session moves one previous convergence into history, includes it in funded reset and close, and retires the displaced older backing through the publication owner.

The eight provenance cases passed against the actual native Pack JSON decoder and the mounted source resolver. The witness also verified that convergence preserved the prior allocation address. Plane values now render one oriented square and a normal anchored at the authored origin. Wireframe mode keeps isolated native points and their exact component references. Three.js passed 61 assertions across the provenance, plane and wireframe fixtures. A native marker witness passed the four plane cases.

Inspector output rows now publish the same graph-channel hover action as the canvas outline. Connected input rows resolve hover to the exact source port, or to the node when the transport carries the whole dictionary. The existing navigation button still activates graph selection. The new retained fixture covers both output forms and both source targets.

The fresh linked faces invocation compiled the BRep registry and engine. It stopped in the Flow mesh operator crate on an omitted OperatorJobStep import and a conflicting progress borrow. Both are corrected. The repository denied the official tessellation rerun with 2.17 GiB free against its 5.5 GiB floor. The next shared solids invocation subsequently reached native compilation after automatic recovery. Cached artifact testing can reuse matching Rust metadata and object files, and a tessellation test binary build is queued behind the native solids run.

The retained planar Closest Point and certified linear, circular, and grid patterns now have isolated native geometry witnesses and independent Three.js checks. The pattern owner stages disjoint components and commits one Compound. The retained Surface Surface Intersect route now publishes the exact line between two standalone planes through its declared wires list. Focused cargo builds have yet to run these newest registry fixtures.

## Viewport hit routing and point-only rendering

React now uses the same line segment hit translation for edge clicks and hover. With edge targeting active, the route preserves the authored persistent edge label even on a standalone wire, including labels above JavaScript's safe integer range. Plain object mode still maps the hit to the source channel. The native authority now sends vertex hover to the geometry cursor on scenes without vortex geometry. Pointer leave clears the previously published lane.

React and native WGPU now render point-only data without requiring a topology vertex ID and still allow whole-object hover. Authored vertex picks continue to depend on vertex IDs. Wireframe and points preview remove surface attributes and materials whose triangle and corner domains no longer exist. The original shaded payload keeps those fields.

The React component suite ran through Vitest and jsdom and passed all 13 tests, including labeled and unlabeled point rendering. The isolated Three.js checks passed 23 assertions across analytic line and point-only hit cases. The inspector jsdom witness passed 12 assertions and the vortex route fixture passed six. Twenty-six edited Rust sources parsed and the scoped whitespace check passed. The independent pattern, box Split and OBJ fixtures passed five tests with 54 assertions.

The linked solids run found a missing LabelSource import in the evolving paired publication owner. The current registry source includes that import. The repository's source reader currently counts exactly 4,000 active lines in that registry. The resumed official tessellation run is still live in the repository's command boot phase. The selected cached-artifact run is still live in the host capacity queue. The OBJ and Sew agents reached their execution limits during their latest assignments. Their work requires fresh review before claiming either registered operator. General BRep parity and browser verification remain open.

## Export channel scope and native tessellation diagnostics

Whole-dictionary export connections now retain all source geometry channels. A matching named channel is subsumed, duplicate requests are removed, and the source remains in its first graph position. The neutral fixture passed three SQLite assertions and the isolated native selector passed. The associated source audit also identified why a LOD change currently reuses a mesh cached at the previous tolerance.

A cached native public tessellation binary exercised 32 tests against the compiled production dependencies. Twenty-eight passed, including the planar spline trims, axial trims, rational surface bounds, reflected sphere picks, and vertex labels. One failing test found that the standalone curve reservation counted progress twice. The redundant wrapper increment has been removed. Three retirement tests found that an authoring effect lacks the original producer invocation. Their shared support now uses the original retirement pump, with a declared test dependency. Fresh linked verification remains pending.

## Preview detail, tessellation delivery and retained exchange

The editor requests a new preview tick when the effective detail changes. A resident mesh records its source tolerance and remains visible until a fully received replacement can publish. In-flight handles stay pinned across graph edits, and the addressed command closes stale replies before they can replace a current mesh or settle a different request. The neutral SQLite preview fixture passed three tests and 24 assertions. A cached native witness compiled the live completed receiver and passed the admission fixture. The linked preview and stale-delivery tests are pending.

The official framework Cargo command passed all 40 tests, comprising five internal and 35 public tests, including Engine ownership cases. The analytic progress regression and three exact physical retirement laws passed. The subsequent linked Flow BRep run has reached the repository disk-capacity gate.

Certified Sew joins six oriented rectangular faces into one box solid. Certified Compound Cut publishes one connected orthogonal solid after subtracting one or two box tools. Split publishes both named halves together under a one-item turn. The retained Surface Surface intersection now returns exact circles for plane and standalone sphere or perpendicular axial cylinder cases. Production-math probes, Three.js and Manifold topology witnesses passed within those supported domains. Linked Flow tests for these operations remain pending. Details are in the closed-curve BRep, native Split and original Compound Cut reports.

The native exchange path now imports OBJ and binary or ASCII STL through the shared funded mesh publication stage. The cached native witnesses exercised shell and solid publication, input identity, cancellation rollback and exact receipts. Three.js also validated the neutral import witnesses. The tessellation extension now waits behind another original request without taking over the cursor or spending its work. The isolated native matcher proved completion and cancellation interleavings under the host's round-robin scheduling. The linked import and extension scheduling fixtures remain pending. See the retained OBJ import, retained STL import and tessellation backpressure reports.

The preview failure ledger now settles routed faults, prevents repeated requests at the failed tolerance, and leaves other tolerances available. Native original-controller execution passed fault settlement and stale reply handling under one-item turns. The preview and hover SQLite fixtures passed 38 assertions across five tests. The cross-window hover reducer preserves another window’s or the inspector’s hover when it receives a delayed pointer leave.

Certified Chamfer now supports any selected edge of the native axis-aligned box. OBJ and STL export now use the retained supported primitive and planar tessellation paths, with exact writer and round-trip witnesses. The composition tessellation fixtures have moved to the original reply and extension lanes. Linked Flow and browser verification, general trimmed intersections, and the remaining finishing and exchange coverage are still open.

Draft now supports one selected native box face and a perpendicular principal-axis pull. The retained solid publication also uses the reserved face-label range so exact preview picks address the authored faces. The independent Three feature suite passed seven tests and 75 assertions. The full framework tessellation run preceded this face-label correction. The linked Flow attempt stopped at its disk gate with 5.22 GiB free against a 5.5 GiB minimum. Repository collection and removal of 112 MiB of superseded ticket output have been recorded. Retained STEP work continues. The registered Draft and current application routes still need linked or browser verification.

## Nested preview, all hovered instances and exact exchange

The preview fallback now uses the same evaluated channel side and structural value path, including nested lists and records. The old compact ordinal fails the inserted-sibling fixture. Both inline Mesh payloads borrow their existing evaluated strings. The SQLite fixtures passed ten assertions and the isolated actual Rust path passed three tests.

Edit, generate and view now author every hovered graph instance in the scene selection projection. Hover changes no mesh or instance document. The full React World3d component suite passed 15 tests, including mounted paint for three instances across two channels, same-mesh sharing, an unrelated instance, local ray precedence, clear and geometry identity. The native draw path passed six fixture cases in an isolated Rust run and an independent Three.js instanced mesh oracle. The registered world-host suite remains pending.

Retained STEP import certifies directly authored cuboids from the Open CASCADE fixtures. Exact STEP export preserves single-shell planar topology with inner bounds and curved-source refusal. Open CASCADE readback passed for a translated box, oblique prism and orthogonal union. The connected orthogonal material certificate now enables chained retained Fuse, Cut and Intersect. Retained Offset and Thicken preserve a circular opening within their certified planar sheet domains. Their linked Flow tests remain pending. The official Flow build cleared disk admission and is compiling its dependency closure.

Composition evaluation fixtures now use the original typed receiver and the actual raw BRep and Math evaluator. Their three portable tests passed with 25 assertions. General BRep parity, remaining exchange breadth and current browser/native app verification remain open.

The live shared Flow Cargo handle was polled and remains active. The repository lease ledger also reports an independent dependency-setup process holding the dependency image exclusively at rank 7 while requesting binding and generation leases at rank 5. The canonical mount path is under review. This process is separate from the live Flow compile.

Inline spatial markers now omit points outside the renderer's Float32 range and preserve a visible origin marker when a vector's entire tip rounds to zero. The previous marker code failed three boundary cases. The corrected code and independent Three.js Float32 check each passed all 12 cases. See the preview marker report.

Edit, generate and view now fit the camera from their typed, filtered position and edge buffers. The viewer caches those bounds with its geometry. Seven cases passed in both the isolated Rust implementation and Three.js. The linked window tests remain pending.

The retained zero-pitch closed helical sweep now produces a native torus for one signed full turn. Signed sphere and ring-torus solid offsets preserve the original frame. STEP import and export now round-trip planar prisms and certified orthogonal shells in the isolated native check and Open CASCADE. See the closed helical sweep, smooth solid offset and retained STEP reports for the measured domains.

Protected dependency setup now releases its exclusive image lease before retrying a busy JavaScript binding acquisition. Seven focused Bun tests and three independent SQLite cases passed. The already-running setup process remained in place. The Flow host session source closure exceeded admission during concurrent geometry changes. Consolidating duplicated field documentation reduced it to 259,214 bytes across 22 sources against the 262,144-byte limit. The official Flow suite still compiles, and the mounted procedural browser path remains unverified.

The edit render now borrows the evaluated document text during projection. The live Flow build last reported 552 compile entries and had not reached test execution.

The real Chromium World3d renderer painted three authored instances and preserved the unrelated one. A browser ray published the qualified graph-port hover address and overrode the authored paint locally. Pointer leave published an empty target list and cleared all marks. Geometry revision stayed fixed. See the Chromium hover report for the mounted renderer scope.

Edit, generate and view now share one parsed evaluation across geometry, value rows and status. Seven native and SQLite neutral cases passed. Authored Generate form images now use the framework Image component with an accessible label and localized missing state. The native UI branch and PNG oracle each passed both locales.

The linked Flow build exited 101 before testing. It had compiled Construction before a concurrent fillet change and later saw the new solid-transaction import of the fillet module. The module and all its current production forwards now exist. Host available space dropped to about 2.9 GiB. The official Rust cache reclaim operation completed with a skipped outcome and zero recovered bytes. A fresh linked suite still needs its disk gate to admit.

New dedicated verification reports cover authored exact STEP cylinder import and export, native perforated extrusion and sweep, and GLB roundtrip of split corner UVs and normals. The official linked tests and complete app path remain pending.

The registered image check now goes through the actual Generate window from authored Flow InputImage widgets and an existing selected generation. The direct UI-source witness independently checks authored description and field-label fallbacks. The current multi-edge fillet, funded GLB intake and typed generation-variable work are still in progress.

The three preview modes now share the framework's expandable channel tree. The production projection and real Rust TreeWindows passed a late-array and nested-record witness without changing the graph port address. SQLite independently matched all three values. See the expandable preview channel values report.

Retained BRep Fillet now accepts up to four certified parallel box edges. Retained Loft has an exact coaxial conical-frustum domain. Dense GLB scene traversal and geometry rows are caller funded. The large GLB witness verified 4,100 triangles, normal and UV transforms, and paid cancellation after 160 rows. Sparse accessors remain in progress. The repository dry run found no collectable cache entries in the chosen areas, so the linked suite remains below disk admission.

Async Flow graph picks now await the current pointer result and the native hover write instead of reading the previous coordinates. A newer gesture or scene revision refuses stale work. Pointer cancellation releases the pick latch across graph and paint surfaces. Nineteen renderer tests and nine independent RxJS cases passed. The real Chromium World3d hover regression also passed after the probe waited for its backing dimensions. Native detail and micro channel-row hit coverage is still being implemented.

Typed generation forms now submit text, numbers, booleans, points and vectors through original Flow ports. Indexed coordinate edits preserve their other axes, and incoming wires keep authority. The scoped native and AJV checks passed. BRep conic–plane queries and GLB sparse accessors have dedicated native and Three.js reports. The full linked suite and mounted app remain open.

The viewport now orders object, component and vortex hover on one publication queue. Old queued hits are discarded on scene changes and unmount. Leaving clears the viewport's local mark after any already-issued hover write. The renderer suite passed 71 tests, the independent RxJS oracle passed eight cases, and Chromium verified the mounted viewport. Native detail and micro port rows now participate in DAG picking, with connector precedence preserved. The isolated Rust path and all eight Three.js cases passed. The linked native applications remain below disk admission.

Retained GLB import now validates mixed BIN and inline Base64 buffers and URI-only files one quartet at a time. The focused native and Three.js witnesses passed. Funding the remaining wide metadata path is in progress. Original line–conic intersections and partial Revolve continue in their owning native crates.

The framework now carries hover window ownership into procedural preview selection. An identical instance group from another window can take over while the viewport pointer stays still, and a delayed echo from the viewport's own window keeps the current ray. Native graph and viewport hover actions include their actual window ids. Native leaves release local paint and clear outstanding publication in pointer order. Chromium, 16 React tests, 13 independent RxJS assertions and scoped native checks passed. See the source-aware bidirectional hover report for coverage and remaining mounted checks.

Retained BRep Revolve now covers exact signed partial circular profiles and meridional rectangles. Circle and ellipse contacts on intersecting planes pass independent Three.js witnesses and scoped native regressions. Retained Section covers spheres and certified cylinders, including cap-clear oblique sections and axial rectangular sections. Cap-crossing oblique cylinder sections remain in progress. Retained Mesh decimation now retires discarded buffers under caller funding and cancels without publishing partial geometry. The 64-component native witness released 3,518,711 bytes over 51,966 retirement turns, and Manifold independently matched its output. These cases have separate ticket reports. The latest free space reading was 2.81 GiB, below linked build admission.

The editor Flow graph now consumes the same hover source as its preview, with schema round trips for omitted, panel and named window origins. Twenty-six graph tests, 12 independent RxJS assertions and nine native schema and reconciliation tests passed. Exact coplanar conic contacts now retain up to four points, and oblique cylinder Section now also keeps one or both cap chords with native elliptic trims. Dedicated reports cover their third-party and isolated native verification. The full native and canonical app runs remain open.

Point and vector variables now publish markers under their original Flow output channels in all three preview modes. The native projection and 23 independent Three.js checks passed. Retained Mesh OBJ export now preserves independently indexed corner normals and explicit zero UVs. Coaxial through-cylinder Cut now publishes an annular solid, and bounded rational NURBS–plane contacts honor authored trims. Each has a focused report and an outstanding linked test under the same disk gate.

The Generate viewport binds geometry and Values to the selected generation. Current generated evaluations leave Edit's convergence history intact, and render borrows the document topology. Four native transient tests, three scoped native policy tests and nine independent AJV and TypeScript tests passed. The linked evaluator and mounted Generate journey remain open. See the Generate preview ownership report.

Retained OBJ import now preserves indexed corner UVs and normals, quotes buffer growth exactly and pauses on a short grant. Retained coaxial BRep Cut now covers blind and enclosed bores, and Fuse supports one or two annular shoulders. The original nearest ports now share bounded rational spline projection. Focused native and independent witnesses passed. The respective reports record the remaining linked coverage.

Generate now uses a localized semantic form landmark with a scrollable viewport in both editors. Chromium verified labelled controls, short-pane navigation, and text, slider and toggle edits through the real shell intent bridge in English and German. The shared native builder compiled and its focused witness passed. See the [generation form viewport report](📓️generation-form-viewport.md) for remaining verification.

Retained Section now covers bounded elliptic, parabolic and hyperbolic cone and frustum cuts. Retained curve measurement now covers rational spline length, trimmed elliptical edges, mixed wires and vertex-to-curve distance in both input orders. Retained Mesh Orient handles disconnected closed components independently, and Triangulate preserves raw corner UVs, authored normals, seams, shading flags and arbitrary vertex and face channels. The focused native and third-party checks passed. The detailed operation reports record the registered linked tests still pending storage admission.

Retained Fields now associate visible labels with their actual controls, including noncanonical vector axes. The 120 editable-control tests, the staged form test, 42 Bun tests, Chromium in both locales and the focused WGPU projection witness passed. The [field association report](📓️retained-field-association.md) records the outstanding broader checks and the size of the remaining form publication gap.

Exact apex cuts now produce cone triangles and frustum trapezoids through the retained polygon publisher. Vertex-to-surface and vertex-to-face Distance run in both port orders, including finite planar faces and holes. Mesh Subdivide now interpolates authored normals and UVs while carrying seams, face flags and generic attributes. Their native and independent third-party reports cover focused verification. The official linked runs remain pending storage admission.

Closed spherical Shell now publishes a signed inward cavity and exact material volume. Mixed Section faces now support cancellable Closest Point and Distance over the original arcs and chords. Mesh Weld preserves authored UVs, hard normals and both sides of new seams. Focused native and third-party checks passed. Their linked Flow fixtures remain pending.

The shared Generate renderer now seats whole fields on record-budgeted pages while retaining original question addresses and bindings. Its actual Rust planner and the independent partition agreed on all nine fixtures. See the [form page admission report](📓️generation-form-page-admission.md) for the still-open navigation and linked composition checks.

Concentric and partially overlapping sphere Booleans now use exact retained boundaries and signed evaluation. Retained Area measures planar faces, including holes and mixed conic sections. Retained Mesh Decimate carries authored normals, shading and face-specific UVs through a collapse. The focused reports record native or metadata verification and independent geometry results. Native Decimate runtime and the official linked suites remain pending.

The Generate form now publishes Previous and Next, a live page indicator and a separate field viewport. Navigation is local to the addressed window and bypasses artifact command dispatch. Chromium exercised both pages and an original question edit in both languages. All 136 related React tests and the focused native presenter and scheduler passed. The [navigation report](📓️generation-form-navigation.md) records the remaining linked composition and canonical WGPU checks.

Retained sphere Cut now supports eccentric containment. Certified Box/Cylinder Booleans support radial containment with axial clipping, and native spherical measurements cover signed solid shapes and extracted faces. Retained Mesh Combine publishes two components with their matching indexed data and bounded retirement. The individual reports record scoped evidence and the remaining linked verification.

The preview now opens complete text in the shared read-only editor. Long values load in UTF-8 pages with the original graph hover target. Two native checks, six independent UTF-8 checks, 22 React checks and a Chromium probe passed. The browser probe also verified fixes for scene option decoding, authored read-only behavior, embedded leading byte-order marks and retained window row counts. The canonical application and linked Generation3d suite remain gated. See the [channel text preview report](📓️complete-channel-text-preview.md).

Retained BRep Intersect now covers side-clipped box and cylinder footprints and offset parallel-cylinder lenses, with exact circular trims and stable preview. Blind and enclosed cylindrical bores and authored spherical-face closest queries have focused native and independent witnesses. Mesh Combine carries authored materials and textures, Mirror carries authored corner and seam data, and Extrude and Inset preserve cap attributes while creating wall and border UV charts. Their individual reports describe the certified inputs and outstanding linked checks.

Exact dictionary keys now open in localized Key and Value rows across the three preview modes. Three focused native checks, the independent UTF-8 oracle and Chromium passed, including long keys with identical summaries and different requested suffixes. Preview mesh history now resolves the recorded direction and nested path even when an older evaluation contains a same-named output. The focused Rust and SQLite checks passed. See the [dictionary key report](📓️channel-dictionary-keys.md) and the [geometry channel direction report](📓️geometry-channel-direction.md).

Exact parallel-cylinder Fuse and through-notch Cut now have retained circular topology. Mesh Knife Cut and Loop Cut preserve authored surface channels, affine transforms preserve authored point normals, and component Rotate and Scale admit partially selected faces with authored directions. The individual reports document independent and native evidence plus the remaining linked verification.

Edit, Generate and View now publish spatial inputs as well as outputs from preview-enabled widgets, with stable directional world instances, shared retained mesh buffers and tessellation demand. The focused paired Rust builders, eight SQLite-backed checks and twelve hover regression checks passed. The registered native preview evaluation build cleared source admission and began compiling. See the [two-direction spatial preview report](📓️two-direction-spatial-preview.md) for the open graph-address and mounted verification cases.

Retained Sphere/Box now supports exact one-plane clipping and an opposed-plane spherical band. Converted NURBS cuboids can publish spline STEP edges and patches, and the independent STEP readback passed. Mesh Delete Faces and Merge Coplanar preserve the surviving authored surface channels. Their individual reports describe the exact invariants and linked verification still pending.

Same-named graph inputs and outputs now have distinct hover addresses through the Flow canvas, outline, value tree and world preview. The focused Rust projection and browser adapter checks passed. The registered native rebuild is compiling dependencies, with canonical mounted hover checks still open. See the [directional graph port hover report](📓️directional-graph-port-hover.md).

The BRep work also added exact eccentric cylindrical bores, analytic annular area and center of mass, and STEP round trips for certified spline cuboids. Mesh Orient and Flip now preserve authored surface channels while correcting winding. Individual reports separate native, independent and remaining linked verification.

A hovered Flow wire now marks both directional endpoints in Edit, Generate and View. Whole-dictionary endpoints expand only the corresponding side, and inlet hover still follows the producer. The isolated native projection and SQLite tests passed. See the [wire hover report](📓️wire-hover-endpoints.md).

Certified Sphere/Cylinder Booleans now include finite-cylinder separation, containment, exact cap and band intersections, and single-cap sphere cuts. Mesh Bevel preserves untouched authored attributes and builds cap charts. Scoped native and independent checks passed. See the [sphere and cylinder report](📓️sphere-cylinder-certified-booleans.md) and the [Mesh Bevel report](📓️mesh-bevel-authored-surface.md).

Procedural overlap hover now publishes every authored address for identical viewport geometry, including exact inline meshes, points, vectors and planes. Chromium verified grouped hover, single selection, pointer exit, independent mesh reuse and alias revisions. The focused native builders, SQLite and React suites passed. The [grouped viewport report](📓️procedural-grouped-viewport-hover.md) records the remaining linked and canonical runs.

The Mesh Orient ownership schema now matches the funded component traversal. Native tests verified the copy receipts and zero progress under a short grant. See the [ownership report](📓️mesh-orientation-owner-contract.md).

Inspector connected rows now publish the actual inlet address and the viewport follows it upstream. Outputs and variable rows use canonical directional addresses, including reserved and empty output keys. The DOM oracle and related SQLite and adapter tests passed. See the [Inspector report](📓️inspector-directional-hover.md).

Retained perpendicular Cylinder Split now produces two exactly capped solids and commits the pair atomically. Focused Rust and independent Manifold tests passed. See the [Cylinder Split report](📓️original-cylinder-plane-split.md).

The [canonical boot readiness report](📓️canonical-procedural-boot-readiness.md) identifies the current React and browser WGPU owners. Both require fresh runtime activation for an end-to-end app check.

Edit and Generate previews now offer the localized level-of-detail selector next to shading and sun. The same editor setting remains reachable from Flow. A focused native production-route witness and the independent detail checks passed. See the [preview detail report](📓️generate-preview-detail-control.md).

Four native grouped-hover tests also passed against production WGPU world libraries. The focused run exercised picking and action publication without opening the canonical WGPU window. See the [native report](📓️native-grouped-hover.md).

Orthogonal Boolean results now support exact point classification, including cavity walls and internal seams. Selected-vertex Weld now preserves authored surface data through compaction. Focused and independent tests passed. See the [classification report](📓️orthogonal-solid-point-classification.md) and the [selected Weld report](📓️mesh-selected-weld-authored-surface.md).

Boundary vertex dissolve now refuses either adjacent authored seam before publishing. Native red/green verification and all 13 portable Mesh checks passed. See the [boundary seam report](📓️mesh-dissolve-boundary-seams.md).

Graph addresses now round-trip authored widget and channel separators. DAG wire reconstruction preserves raw operands, and the active React Flow canvas carries input and output identity through picking, hover and selection. Five scoped native and six mounted React tests passed. See the [graph identity report](📓️procedural-graph-identity.md).

Retained spherical and cylindrical Boolean results now support exact point classification against their trimmed boundaries. The scoped native and independent tests passed. See the [spherical report](📓️spherical-solid-point-classification.md) and [cylindrical report](📓️cylindrical-boolean-point-classification.md).

Unwrap UV now updates the active authored channel while preserving secondary indexed UVs. Its ordinary preview releases replaced index storage with the correct grant. The Mesh Document closure also passes its existing source limits. See the [Unwrap report](📓️mesh-authored-uv-unwrap.md) and the [source boundary report](📓️mesh-document-source-boundary.md).

The WGPU Flow projection now restores and publishes side-aware handles from its admitted scene. Focused native tests passed. The [WGPU Flow report](📓️wgpu-flow-directional-targets.md) records the pending linked canvas regression, and the [hover granularity audit](📓️graph-hover-granularity-audit.md) records the remaining node, wire and handle collision.

Certified Box Chamfer now uses funded retained construction with cancellable progress and atomic publication. It preserves the Flow operator's triangular corner caps. Eight focused Rust tests and the independent Manifold oracle passed. See the [Chamfer report](📓️retained-all-edge-box-chamfer.md).

The repaired preview test retry was released before admission because its memory request was holding smaller verification jobs. Bridge's focused native run and portable modeling checks then passed. The complete linked preview suite and canonical runtime mounts remain open.

Bridge is now registered for two explicitly anchored, disjoint mesh boundary loops. Its retained job preserves authored rim data, adds wall charts, and refuses invalid selections atomically. The native and 17 portable checks passed. See the [Bridge report](📓️mesh-bridge-boundary-loops.md).

Retained Fuse now covers radially contained, offset parallel cylinders with one or two projecting ends. Independent Manifold and 19 native source checks passed. See the [eccentric shoulder report](📓️eccentric-cylinder-shoulder-fuse.md).

Generation3d now scopes node, wire and handle interaction identities so matching authored text cannot cross-highlight them. Focused native Edit/View, React and WebGL tests passed. See the [namespace report](📓️graph-interaction-namespaces.md). The Flow source witness separately passed five cases for directional native picks, hover and restore. See the [Flow report](📓️flow-native-directional-channel.md).

The combined portable preview, graph, LOD and kernel oracle run passed 32 tests and 297 assertions. The paired focused Rust Edit/View projection ran four tests against the cached Pack JSON library. The source witness exercised the current directional mesh and inline identity builders.

Bridge now aligns unequal boundary loops with triangles and quads while preserving authored rims. The focused native verification and 17 portable tests passed. See the [unequal Bridge report](📓️mesh-bridge-unequal-loops.md).

Exact shared-axis rotated Box Intersect preserves authored surface planes. Classify accepts placed boxes and clipped prisms, and the Boolean set identities cover contained Fuse, fully covered Cut and disjoint or contact-only Cut. The 22 focused Rust tests and Manifold oracle passed. See the [intersection report](📓️rotated-shared-axis-box-intersection.md) and [set identity report](📓️rotated-box-classification-and-set-identities.md).

Perpendicular retained Cone Split produces a tip cone and base frustum with a matching cut circle and signed publication order. Four focused Rust cases and 101 independent assertions passed. See the [Cone Split report](📓️retained-cone-plane-split.md).

Bevel now supports an isolated locally convex edge on a concave closed mesh where every profile cut stays within its certified endpoint neighborhood. Two native tests and 18 portable tests passed. See the [local Bevel report](📓️mesh-bevel-concave-local-edge.md).

Spatial preview now uses explicitly authored geometry, topology, point, vector, plane and mesh types. Nested typed geometry survives inside metadata, while incidental coordinates or handle-like text do not create instances. Five focused Rust and 43 portable checks passed. See the [typed spatial preview proof](📓️typed-spatial-preview-proof.md).

Generation3d's typed catalogue now exposes retained Bridge and Combine using the same kernel jobs as Flow. Two focused native tests and the complete 58-test portable catalogue and mesh check passed. See the [schema Mesh report](📓️schema-mesh-bridge-and-combine.md).

Polygon capture and JSON export now advance in funded steps and preserve authored winding, attributes, materials, textures and surface channels. The native export proof matched the prior encoding byte for byte. See the [capture report](📓️mesh-polygon-source-capture.md) and [JSON export report](📓️mesh-export-json-responsive.md).

Component editing retains visible reference geometry and graph-originated input marks while ray and marquee selection stay on editable outputs. Chromium verified input paint and exact editable picks. The Flow canvas also restores every directional port in a grouped viewport hover. See the [component report](📓️component-preview-pickability-proof.md) and [Flow hover report](📓️grouped-flow-graph-hover.md).

Retained all-edge Box Fillet now builds true cylindrical and spherical boundaries, with exact measures, funded preview, OBJ/STL and exact STEP round-trip. Open CASCADE independently opened all three STEP shells. See the [Fillet report](📓️retained-all-edge-box-fillet.md) and [STEP report](📓️retained-rounded-box-step.md).

Shared-axis, equal-span rotated Cut and Fuse now publish exact connected, holed and disconnected results. A compound can publish up to four pieces atomically, sum exact measures and expose every child preview. See the [connected results report](📓️rotated-box-cut-and-fuse.md) and [compound report](📓️disconnected-rotated-box-publication.md).

Boundary extrusion is registered in Flow and Generation3d with authored surface transfer and independent modeling checks. Convex Mesh Intersect has a funded two-source clipping stage with proof of contact and separation, exact topology and interpolation provenance. Authored reconstruction and widget wiring are in progress. See the [extrusion report](📓️mesh-extrude-boundary-chain.md) and [clipping report](📓️mesh-convex-intersection-clipping.md).

Generate now keeps long Unicode answers editable through the shared Input control and renders retained image sources through the shared Image control. Chromium passed both paths, and image carriers count toward the existing page budget. See the [text report](📓️retained-generate-text-proof.md) and [image report](📓️retained-generate-image-sources.md).

The canonical React dry run moved from waiting on the older graph leader into a fresh repository inventory. A shared cache reset reclaimed the earlier build lane. The app plan, toolchain-dependent linked suites and canonical React and WGPU mounts remain open.

## Typed relay viewport preview

The shared Edit, View and Generate census now visits typed Flow relays carrying geometry. The neutral fixture covers both directions, a scalar relay without world geometry, and the existing hidden export scope. Compact native checks passed five cases, and SQLite passed seven cases and 25 assertions. [Verification](📓️typed-relay-viewport-preview.md)

## Typed mesh intersection and authored surface transfer

The Generation3d catalogue and Flow expose closed convex mesh intersection with retained authored surface transfer, typed refusal for incompatible source channels, and a canonical empty result for contact. The paired typed mesh jobs yield on partial retained grant exhaustion. Current native owner checks passed three intersection geometries at three fuel sizes and cancellation during capture and production. Independent catalogue and mesh geometry checks passed 58 tests and 361 assertions. [Typed adapter](📓️typed-mesh-intersection.md) · [Native authored mesh and Flow](📓️mesh-convex-intersection-authored.md)

## Hover for independently authored coincident previews

Exact equal retained surfaces now publish the same hover group across different mesh handles at the identity pose, preserving mesh buffers and click targets. The grouped payload passed six compact native checks, independent SQLite comparison, and a mounted WebGL regression with distinct mesh IDs. [Verification](📓️independent-retained-preview-hover.md)

## Typed Mesh Difference

The Generation3d and Flow catalogues expose retained single-plane convex Difference, including oblique cuts, containment and contact. Multiple planes return a typed refusal. Three work-fuel settings and cancellation passed in the current native owners. Exact catalogue parity and independent geometry passed 58 tests and 371 assertions. [Typed adapter](📓️typed-mesh-difference.md) · [Authored mesh kernel](📓️mesh-convex-difference.md)

## Exact long channel hover addresses

Shared Values, Outline and Inspector use direct semantic target fields. The focus binding carries a 250-byte authored ID whose batch form exceeded the UI text limit. Existing renderer batch clears still take precedence. Real native UI arguments, SQLite and mounted React intent projection passed. [Verification](📓️direct-channel-hover-addresses.md)

## Typed Mesh Union

The closed convex Union operator supports containment, separated shells and complete opposing-face stitches, including face contact. Current compact native owners verified geometry and cancellation. The portable catalogue and geometry suites passed 58 tests and 381 assertions. [Verification](📓️typed-mesh-union.md)

## Oversized Generate image resources and retained BRep coverage

Both Generate forms mount exact image resources beyond the tree-window node limit. Chromium and the native upload proof verified 2,360,852 bytes, cancellation and all pixels through six bounded native tiles. Retained Classify supports certified frusta, torus sectors and annular solids. Exact Box Section supports oriented stock, clipped halves and convex prismatic Boolean results. [Image paging](📓️large-window-image-resource-proof.md) · [Classification](📓️retained-revolved-solid-classification.md) · [Box Section](📓️retained-oriented-box-section.md) · [Prism Section](📓️retained-convex-prism-section.md)

## Spatial input hover and nonconvex mesh corners

Literal Inspector coordinate groups use the same input-direction hover as connected rows. Real semantic Rust projection verified four coordinate groups and 12 edit actions, the complete React Interpreter passed 202 tests, and independent checks passed eight tests and 38 assertions. Typed Mesh Difference now accepts two intersecting tool faces, including a closed nonconvex corner result. The current retained mesh owner and job-context witness passed across three fuel sizes. Portable catalogue and geometry verification passed 58 tests and 383 assertions. [Spatial hover](📓️literal-spatial-input-hover.md) · [Corner Difference](📓️mesh-corner-difference.md)

## Cylindrical Shell and axial Offset

Retained BRep now shells finite cylinders with closed cavities, either end opening and both cap openings. Exact Offset Solid handles cylinders, cones and frusta in the authored frame. Native topology and independent geometry proofs passed. [Cylindrical Shell](📓️retained-cylindrical-shell.md) · [Axial Offset](📓️retained-axial-solid-offset.md)

## Shared mesh publication, slab Difference and transformed edges

Hover-only refreshes reuse one cached tessellated mesh publication across Edit, Generate and View. Exact session and resolved source identity prevent viewport leakage between sessions with matching local epochs. Seven compact native cases and 25 SQLite assertions passed. The typed Difference path also supports middle slabs with two separately closed residual shells. The current paired mesh suite passed three fuels and portable catalogue and geometry checks passed 58 tests and 387 assertions. Selective retained Box Fillet and Chamfer support rotated and reflected stock. [Cache](📓️shared-preview-mesh-publication.md) · [Slab Difference](📓️mesh-slab-difference.md) · [Selected Fillet](📓️retained-oriented-selected-fillet.md)

## Mesh octants, through-tunnels and oriented chamfers

Typed mesh Difference now covers certified three-plane octants and four-wall through-tunnels. The compact native suite completed all 23 selected geometry fixtures at three fuel sizes within its bounded work limit. Independent catalogue and geometry checks passed 58 tests and 393 assertions. Exact retained all-edge Chamfer now uses the box's rotated or reflected frame with the canonical 26-face topology. [Octants](📓️mesh-octant-difference.md) · [Tunnels](📓️mesh-tunnel-difference.md) · [Selected Chamfer](📓️retained-oriented-selected-chamfer.md) · [All-edge Chamfer](📓️retained-oriented-all-edge-chamfer.md)

## Canonical runtime admission

The actual React target now owns the graph inventory after stopping the diagnostic dry run. Direct dependency-excluded serve planning returned a graph-service key error. Managed Rust type checking remains active. [Runtime status](📓️canonical-procedural-run-status.md)

## Blind pocket Difference and late cancellation

The typed Mesh Difference path now covers five-face blind pockets with a closed stock result, a single opening and separate authored wall and floor channels. The portable catalogue and geometry suite passed 58 tests and 395 assertions. The current compact native suite passed 24 selected cases at three work fuels, early cancellation and cancellation during authored projection of both a through-tunnel and a blind pocket. [Blind pocket](📓️mesh-blind-pocket-difference.md)

## Enclosed mesh cavities and oriented Box Shell

Typed Mesh Difference can preserve the exact stock exterior and subtract a strictly enclosed cutter as a negative inner shell. A touching cutter follows the pocket path when its opening is certified. The current typed witness passed 25 geometry cases at three fuel levels and both early and late cancellation. Catalogue and independent geometry passed 58 tests and 397 assertions. Retained BRep Shell now supports rotated and reflected Box stock, all six face openings, through tunnels and closed cavities. [Enclosed mesh cavity](📓️mesh-enclosed-cavity-difference.md) · [Oriented Box Shell](📓️retained-oriented-box-shell.md)

## Evaluated image channels

Flow Values displays the evaluated Image channel through the shared Image control, with expandable metadata and independent semantic addresses for input and output. Mounted Chromium verified both addresses, the exact PNG bytes, and Blob retirement on collapse. Compact native and independent PNG checks covered the publisher and page reassembly. [Verification](📓️evaluated-channel-image-preview.md)

## Bundled procedural examples

The portable example suite passed all 33 tests and 622 assertions across nine bundled example files. Coverage includes wire extrusion, face sweep, analytic sphere, box and torus, rounded and shelled stock, the hexagonal column and mesh workbench. [Verification](📓️bundled-procedural-examples.md)

## Different-frame Box Intersect

Retained Box Intersect keeps a single oblique support through the existing ClippedBox and preserves all active authored planes on a general certified convex result. Compact native suites passed 48 tests. Manifold independently verified 18 tests and 168 assertions, including reflected and asymmetric overlaps. The registered Flow test covers swapped operands and cancellation. [Single support](📓️retained-skew-box-intersection.md) · [Multiple supports](📓️retained-multisupport-skew-box-intersection.md)

## Exact Inspector values and editing

Evaluated text uses the shared retained Text Editor pages, including selected Flow variable outputs. The Inspector preserves canonical integer spelling. Notes, unconnected text inputs and indexed collection text use the shared retained Input and existing mutation limit. Compact Rust and mounted Chromium verified exact text and address reconstruction, editing, cancellation and both locales. [Verification](📓️inspector-exact-nonspatial-values.md)

## Partial-overlap Mesh Union and skew Box Cut

Mesh Union now joins certified partial overlaps, preserving native authored charts and giving shared outward patches one owner. Compact Mesh and retained Flow verification passed. Independent Manifold and packaging passed 24 tests and 4,672 assertions, and the paired typed mesh witness passed at three fuel settings. Reflected and rotated Box Cut covers the exact one-plane complement and strictly enclosed skew cavities with an inward retained shell. Compact native BRep and Manifold checks passed. [Mesh Union](📓️mesh-partial-overlap-union.md) · [Box Cut](📓️retained-skew-box-difference.md)

## Large directional hover

Coincident viewport hover now carries every associated handle beyond the 4 KiB action threshold. The typed Flow browser adapter and compact native decoder support the complete 1,024-port roster. The shared highlight style projects incoming hover to Values and Inspector. Managed Preview and Mesh type checking passed on the earlier admitted source snapshot. Preview geometry laws have an independently registered linked test target under the source budget. [Verification](📓️oversized-directional-hover-verification.md)

## Sewn skew Box recess

Native BRep Cut now sews the two authored inward tool walls and exposed stock patches into one closed shell. Compact Rust and Manifold agree on the retained case. The independent Flow verification target covers exact native topology, preview, Measure and cancellation. [Verification](📓️retained-skew-box-recess.md)

## Native planar Classify

The retained Classify route now recognizes bounded closed planar shells, including the skew recess and enclosed cavity. Compact native verification and independent Manifold and Three.js checks agree on interior, exterior and Euclidean boundary tolerance. [Verification](📓️retained-planar-classification.md)

## Separated skew Union and five-support Mesh Union

Native BRep Union now publishes strictly separated skew boxes as two independently placed compound members. Mesh Union spans more than three crossing planes through exact anchored edge interpolation. The five-support case yields 28 live vertices, 48 edges and 22 exterior polygons. Compact native checks and independent Manifold measures passed. Mesh packaging preservation now checks the named Rust laws instead of counting assertions added in unrelated tests. [BRep compound](📓️retained-separated-skew-union.md) · [Mesh Union and packaging](📓️mesh-five-support-union.md)

## Native Section of skew convex intersections

Section now consumes the planar topology retained by skew convex Box Intersect and uses the existing bounded edge cursor. Compact Rust and independent Manifold checks agree on two cross-sections, including reflected stock. The linked BRep query check is queued. [Verification](📓️retained-skew-convex-section.md)

Mesh Difference now preserves triangular through and blind openings, including exact reversed source ownership and authored UV projection. Native and independent Manifold checks passed. [Triangular Mesh Difference](📓️mesh-prismatic-difference.md)

Mesh blind pockets now span coplanar stock faces while preserving their authored trim, material and UV ownership. The compact proof passed exact reimport, 29 Generation3d geometry fixtures and 11 retained Flow tests. [Coplanar pocket opening](📓️mesh-coplanar-pocket-opening.md)

Section now publishes the original concave cut through the sewn skew Box recess, with signed winding for both plane normals. The compact Rust proof and Manifold's independent oracle pass. [Concave planar Section](📓️retained-concave-planar-section.md)

The retained Distance query accepts rotated and reflected rectangular placements with certified edge separation, overlap and contact. The compact source proof passed all 40 cases and Manifold reproduced the gap in both orders. Section also supports planes that clear an enclosed planar cavity. [Placed Box Distance](📓️retained-oriented-box-distance.md)

Mesh Difference now certifies a connected cutter-wall cycle for the six-wall tunnel and seven-support pocket. The compact proofs passed 31 Generation3d cases, 11 retained Flow tests, and the authored ownership and reimport checks. Convex Union reuses the funded exterior-plane step. [Many-wall Mesh Difference](📓️mesh-many-wall-difference.md)

Mesh Difference now certifies nonprismatic recesses opening through two or three stock faces. The original native clipping suite rejects pinched exterior vertices and passes all seven tests. Authored projection, 33 paired Generation3d fixtures, 12 retained Flow tests and the independent Manifold and Three.js checks passed. [Nonprismatic Mesh recesses](📓️mesh-nonprismatic-recess.md)

A three-support skew Cut now yields one exact closed BRep shell with 25 vertices, 38 edges and 15 faces. The retained source proof and registered Manifold metrics passed. [Three-support skew recess](📓️retained-three-support-skew-recess.md)

- [Retained placed Box Union and pocket](📓️retained-placed-box-union-and-pocket.md) accepts overlapping skew and distinct-span original Boxes as one exact solid in both Union operand orders and publishes the certified five-support blind pocket for Cut. The compact native suite passed 44 tests and independent Manifold and Three fixtures passed 19 tests with 226 assertions. The managed Flow integration run remains in progress.

- [Six-support skew pocket](📓️retained-six-support-skew-pocket.md) extends the exact BRep Cut arrangement to every authored tool plane. All 45 compact native cases and 20 independent Manifold and Three cases passed. The disconnected reflected-stock Cut still needs compound publication.

- [Disconnected reflected-stock Cut](📓️retained-disconnected-skew-box-cut.md) now represents three exact original Box remnants, including the genus-one closed member. The compact native suite, independent Manifold and Three fixtures, and scoped whitespace checks passed. The Flow compound rollback, explode and shaded preview fixture awaits linked verification.

- [Disconnected Mesh Difference](📓️mesh-disconnected-difference.md) accepts four diamond-separated corner solids and three skew reflected-stock remnants, preserving authored materials, UVs, and closed topology through both Generate3d and Flow. Eight registered Rust Clipping tests, the 16-test independent Clipping and packaging suite, the two authored facade proofs, and both scoped typed consumer suites passed.

- [Linked Preview and Flow OBJ verification](📓️linked-preview-fixture-and-flow-obj.md) records the first real linked Preview test outcomes, the fixture corrections, and the repaired OBJ output-variant dispatch. Both managed retries remain in progress.

- [Mesh Union of placed stock](📓️mesh-placed-union.md) accepts skew and distinct-span positive overlaps in both orders. The retained Flow and Generate3d witnesses passed with authored material, UV and color checks in the native facade.
- [Retained planar center of mass](📓️retained-planar-centroid.md) integrates exact signed moments for the tilted pocket and the three disconnected reflected-stock solids. The original compact traversal and independent Manifold and Three fixture checks agree. Linked Flow verification remains pending.

- [Retained planar compound classification](📓️retained-planar-compound-classification.md) quotes every member's original topology and distinguishes the retained interiors, the removed gap, and a closed boundary on the reflected-stock Cut. The 49 compact native checks and 20 independent oracle cases passed.

- [Section at authored vertices and edges](📓️retained-section-at-authored-vertices-and-edges.md) preserves exact shared vertex and edge positions in the concave ring. Both normals passed the 51-test native source suite. The independent skew fixture suite passed 20 tests and 288 assertions.

- [Section at a coincident interior pocket floor](📓️retained-section-at-authored-vertices-and-edges.md) now returns the exact area-16 rectangle and preserves the existing refusal for boundary-only contact. The 52-test native and 20-test independent suites passed.

- [Retained planar solid and compound Closest Point](📓️retained-planar-solid-nearest.md) uses one original face cursor at a time. The compact native suite passed 53 tests, and the independent oracle agreed for retained material and for the removed gap.

- [Section on a handled shell](📓️retained-section-on-handled-shell.md) accepts the exact genus-one Cut member, preserves previously supported divided edges, and agrees with the independent single-ring oracle.
- [Retained planar Bounding Box](📓️retained-planar-bounding-box.md) accumulates exact world extrema across the three-member reflected Cut. The compact source suite passed 55 tests and Manifold independently reproduced all six coordinates.

- [Mesh Booleans with a contained concave prism](📓️mesh-contained-concave-prism.md) certifies original caps and reciprocal walls, accepts rotated placement, and handles all three operations in both operand orders. The direct corner transfer preserves nonaffine UVs. Native Clipping, Flow, Generate3d, and independent Manifold checks passed.

- [Transverse Mesh Booleans on a concave prism](📓️mesh-transverse-prism-booleans.md) preserves one partial Intersection and both surviving Difference pieces. The 10-test Clipping, 17-test Flow, 8-test Generate3d and independent Manifold checks passed.
- [Retained rational plane nearest](📓️retained-rational-plane-nearest.md) recovers exact finite patch parameters for weighted interior, edge and corner cases. Native inverse and Three.js projection checks passed.

- [Closest Point on derived analytic solids](📓️retained-derived-solid-nearest.md) covers eccentric sphere cavities with sphere or box exteriors, clipped spherical bands, and frustums. The 66-test compact BRep suite and independent Three.js oracle passed.

## Latest managed and scoped verification

- [Canonical procedural run status](📓️canonical-procedural-run-status.md) records automatic Flow registry preparation and the remaining canonical launch stages.
- [Longitudinal prism Booleans](📓️mesh-longitudinal-prism-booleans.md) records the newly verified opposing-wall Intersect and Difference, including authored channel samples.
- [Native Section voids](📓️retained-section-with-native-voids.md) records connected pockets, exact inner trims and cancellation.
- [Spherical strip Section](📓️retained-spherical-strip-section.md) records full circles and clipped conic contours.

- [Certified sphere distance](./📓️certified-sphere-distance.md) covers sphere pairs and sphere versus an oriented box through the existing retained query. The native dispatch witness and Three.js fixture pass.

- [Exact WGPU hover browser proof](./📓️wgpu-exact-hover-browser-proof.md) covers the exact shell port and preview instance in both directions. The native projection and Zod oracle pass. The canonical browser journey is waiting for boot components.
- [Canonical launch and compiler recovery](./📓️canonical-procedural-run-status.md) records the actual WGPU runtime publication, the React component retry, the native Preview compiler blocker, and the current launch probes.

- [Disconnected planar BRep Section](./📓️disconnected-planar-section.md) separates exterior faces and their holes, with an atomic retained list publisher. Compact geometry and the independent Manifold cases pass. Registered Flow and full browser verification remain in progress.


- [Selected DWG drawing pages and records](./📓️dwg-retained-exchange.md) records 13 focused native tests, the independent acadrust handle-map comparison, and the staged interchange widget. Strict retained Flow, document-picker import, and linked full-file verification are open.
- [Reflex Mesh Difference tails](./📓️mesh-reflex-difference-tails.md) records native clipping, authored closed reimport, and the Manifold geometric check for both material directions.
- [Geometry channel and document-import audit](./geometry-channel-dwg-route-audit.md) records React and WGPU source coverage and the document-preview rearm defect. The mounted renderer runs still require both servers.

- [Independent longitudinal reflex supports](./📓️mesh-reflex-independent-supports.md) records native and authored coverage for simultaneous reflex contacts, nonparallel supports and transverse tails. Contacts on another support plane and overlapping removal contours remain open.

- [Canonical renderer recovery audit](./canonical-renderer-recovery-audit.md) records the exact production boot closure, safe managed cache reuse and the ongoing WGPU run after the React and linked Flow ENOSPC failures.


- [Bounded DWG widget transport and projection](./📓️dwg-retained-exchange.md) covers ownership transfer, 4,096-byte base64 turns, native entity projection, and position, face and normal ticks. The focused tests passed against independent Python, acadrust and Three.js oracles. The strict retained Flow and linked full-file package remain open.
- [Joined reflex Intersect with moving caps](./📓️mesh-joined-reflex-transverse-caps.md) and [joined reflex Difference with retained tails](./📓️mesh-joined-reflex-retained-tails.md) record native, authored and Manifold tests for the moved-cap cases.
- [Mounted probe freshness](./mounted-watch-corrections.md) and [WGPU paint observation](./wgpu-paint-observation.md) record run-specific evidence, terminal verdicts, and the independent compositor screenshot check. Tests passed on synthetic results. The new probe has yet to run on the mounted WGPU app.

- [Interior longitudinal support crossings](./📓️mesh-interior-support-crossings.md) records verified native, authored and Manifold geometry for two moved transverse caps, including exact generated-position provenance. A stationary concave cap still requires authored face sampling.


- [R2004 DWG directory and page checkpoints](./📓️dwg-retained-exchange.md) records the bounded physical decoder used by the actual drawing reader, one-row directory steps, and byte-exact independent acadrust comparisons on the AutoCAD fixture.
- [Stationary Mesh cap sampling](./📓️mesh-stationary-cap-sampling.md), [three-support crossings](./📓️mesh-three-support-crossings.md) and [three nonparallel supports](./📓️mesh-three-nonparallel-supports.md) record native, authored and independent Manifold coverage through the actual Mesh operators. Stationary triangular Difference apertures remain in progress.
- [Canonical WGPU storage recovery](./📓️canonical-procedural-run-status.md) records the exhausted build and the live new launch with the storage guard. The mounted browser proof remains open.


- [R2007 DWG directory and physical decode](./📓️dwg-retained-exchange.md) records bounded AC21 LZ77, Reed–Solomon page reads, Unicode descriptors and independence checks through the drawing reader. Strict retained Flow still needs controlled ownership and publication.
- [Stationary Mesh apertures](./📓️mesh-stationary-cap-openings.md) and [rectangular Mesh interior cuts](./📓️mesh-four-support-interior-cuts.md) record exact blind cuts, cavities and through-openings with Manifold and authored mesh reimport.
- [Canonical storage and recovery](./📓️canonical-procedural-run-status.md) records the terminal guarded WGPU attempt, corrected live-volume reserve and timeline loss handling, plus the managed collector's blocked claim creation on the full Rust partition.

- [Grouped local viewport hover](./📓️local-grouped-viewport-paint.md) records React component pointer paint, the shared native draw law, and the RxJS oracle. [Canonical renderer status](./📓️canonical-procedural-run-status.md) records the APFS kernel unmount refusal holding the full browser proof.
- [Bounded DWG logical projection](./📓️dwg-retained-exchange.md) records retained entity turns and the shared checked triangle rules for native and Semio meshes. The [picker audit](./dwg-document-picker-custody.md) also identifies the existing TXT continuation test that the synchronous original import cannot satisfy.
- [Convex Mesh support rings](./📓️mesh-convex-support-rings.md) and [remote stock contacts](./📓️mesh-remote-stock-contacts.md) record exact cutters through six supports, typed actual-contact refusal, and authored reimport with independent Manifold comparisons.
- [Matching concave Intersect and Difference](./📓️mesh-matching-concave-prisms.md), [matching concave Union](./📓️mesh-matching-concave-union.md), and [mismatched nested concave profiles](./📓️mesh-nested-concave-prisms.md) record the authored Boolean coverage and retained materials, UVs, and vertex data. Crossing mismatched profile boundaries remain outside those certificates.
- [Retained blind-bore repair](./retained-defeature-verification.md) documents both opening directions, including imported selected floor and wall labels. [Certified sphere clearance](./📓️certified-sphere-distance.md) documents orthogonal Boolean and rotated shell surfaces. Full linked Flow coverage for these newer BRep changes remains pending.
- [Separated concave Mesh prisms](./📓️mesh-separated-concave-prisms.md) documents both operand orders, independent Manifold comparisons, and authored closed-shell reimport with original UVs, materials and vertex data.
- [Retained orthogonal Closest Point](./📓️retained-planar-closest-point.md) records exact projections to exposed material faces, including notch and cavity walls and rigidly placed shells. The focused native source witness and Manifold/Three.js passed. The registered Flow test remains unlinked.
