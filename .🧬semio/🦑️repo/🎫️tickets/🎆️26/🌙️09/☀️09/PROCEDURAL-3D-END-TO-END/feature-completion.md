# Procedural 3D implementation and verification

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
