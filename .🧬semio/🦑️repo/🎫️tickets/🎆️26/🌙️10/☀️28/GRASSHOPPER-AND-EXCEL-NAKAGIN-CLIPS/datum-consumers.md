# Datum Consumer Integration

## Final Grid and Pose Snapshot

This report supersedes the earlier 2.90 m integration checkpoint. The repeated primary-section pitch is 2.80 m; the earlier 2.90 m glyph reading was withdrawn after independent scan registration. Current podium levels are 0/4.00/7.72 m above the chosen ground, with separate coping. A roof/head heights are 42.14/11.36 m. B uses derived working values 36.62/10.83 m. The ground and B datums retain their declared uncertainty.

The shared model has 140 capsules:76 in A,64 in B;75 ragged attachment branches comprise 65 pairs and 10 singletons. Proposed program labels are A36/B35/C33/D36. A floor 3 has N/J and W/Slash capsules in phase 1; B’s nonzero phase order is swapped and its core topology mirrors A across X. Observed A4/5/13 east L capsules are restored; the erroneous copied J pose was separately corrected to the actual L centre/orientation. All 140 window-bearing poses now have positive GLUON capture evidence. A4, A5 and A13 each have seven positively identified distinct occupied positions matching their count of seven, so the omitted position is established by exhaustion; concealed entry hands and interiors retain separate reconstruction uncertainty. See all140-window-coverage.md and all-storey-pose-evidence.md.

All snapshots were derived independently from raw JSON floor/group/item records, assignment weights, trigonometric frame conversion and counters, without calling the teaching implementation to manufacture expected outputs. Existing collision, doorway, support and headroom predicates were preserved. The final full model gate passes the unchanged capsule-overlap check.

## Grasshopper

The over slider binds towers.B.roof_clearance and displays Roof clearance / Dachfreiraum. Selecting A rewrites both its value and source path. The ch slider similarly binds towers.B.capital_height and switches per tower. The removed capital.height and core.service_overrun fields are not used by these consumers. All bound raw snapshots and pinned fixtures were refreshed.

The Series starts at the 7.72 m podium roof and advances 2.80 m. Actual port offsets and the List Item crown table decompose relative to that roof datum, rather than Tower.base_elevation (the minimum first occupied capsule base,8.6533333333m). This distinction is essential for the measured sparse first floor. Per-tower clearances 2.0533333333 m A/2.1333333333 m B are derived from the actual highest crown. The source/target handed port construction still reproduces every canonical frame and world box against the Rhino oracle.

Derived thirds use 10 decimal places. [McNeel’s slider documentation](https://developer.rhino3d.com/api/grasshopper/html/Properties_T_Grasshopper_GUI_Base_GH_SliderBase.htm) permits 0–12 places; the portable schema now matches that range and rejects 13. Casting fixture numeric distinctions such as 11.0 versus 11 were preserved by targeted primitive edits.

## Excel

All 108 persisted scalar quantity bindings still agree with shared values and provenance. Their input values remain unchanged because structural datums are not worksheet thermal inputs. The 140 Program table rows were refreshed in canonical raw-record order, including identities, facades, item indices and proposed type labels. Counts, lookup results and bilingual displays follow A36/B35/C33/D36; aggregate area and heating totals remain unchanged.

The Program table persists identity/program columns rather than XYZ columns. A separate 140-row portable XYZ snapshot now verifies every evaluated capsule ID against the canonical placement. A NumPy oracle independently derives the back-plane offset from raw centres/rotations and checks the three-phase Z formula. No extra worksheet UI columns were invented.

## Shared Fixtures and Text

Shared portable cases now contain 75 branches,10 singletons, final type counts, current sample planes, corrected absence paths, per-tower base/roof/head datums, scaled roof-corner profiles, podium levels and the−1.45 m bridge centre. The restored_a_l case and its distinct-footprint regression were preserved. The final stale literal 8-singleton assertion was replaced by the portable 10-singleton expectation; physical predicates were untouched.

Both series and the shared model README now distinguish sourced heights, registered placement evidence and provisional/inferred choices. The shared README no longer claims 74 branches, an assumed floor 3 arrangement, the former 7.5 m roof/2.75 m pitch/shared 2.8 m overrun, or an open rear on the now four-sided machinery head. All four bilingual transcripts were regenerated from the nine authored chapter narrations. The reusable transcript tool resolves model-dependent casting constants without scene imports.

## Static Gates

| Gate | Result |
|---|---|
| Shared model Python |59 tests PASS,49.879 s; includes restored L poses and all original overlap/doorway/headroom predicates |
| Shared strict AJV/independent JavaScript |29 invalid models rejected;1336 provenance-bearing leaves verified;75 branches/140 capsules and energy oracles reproduced |
| Grasshopper Python |62 tests PASS,5.433 s; raw JSON/NumPy datum pipeline and Rhino transforms/world boxes |
| Grasshopper strict AJV/JavaScript |28 invalid models rejected;23 tree/mapper cases;14 casts and 49 documented rules |
| Excel Python |85 tests PASS,43.629 s;140-position NumPy oracle plus existing openpyxl/formulas checks |
| Excel strict AJV/JavaScript |18 invalid workbooks rejected;108 bound inputs,31 structured references,10 A1 translations and 5 ICU formats verified |
| Capsule detail Python |9 tests PASS,6.551 s; actual rounded body, rear apertures, cavities and hardware |
| Capsule/bridge domain AJV |9 impossible fixture dimensions and 12 impossible bridge dimensions rejected |
| Transcript regeneration |All nine chapters, both languages PASS |

Execution separately reports the final bridge/core route proof:18 circulation tests PASS24.588 s,bridge 7 PASS2.838 s,roof 9 PASS6.550 s,podium 10 PASS1.712 s. Its final change bends portal body routes before joining the wider native bridge; the broad slab/door geometry and all capsule poses remain unchanged.

## Final Native Gate

Native runs are held until the root’s post-cleanup source-freeze signal. Each scene/language runs in a fresh process. validate-scenes.py now records UTC start time and semantic input fingerprints with every durable subtest/result and refuses a successful exit if those inputs change during a run. It sets sys.dont_write_bytecode before repository imports; further direct suites use Python -B. Earlier native records belong to earlier geometry and are not presented as passes for this final snapshot.

All owned source/case/doc edits are stable. Exact atomic replacement is used; no one-off migration script was retained. The reusable audit/transcript/native validation inputs and Markdown evidence remain in the ticket. Final generated outputs stay under the ticket’s generated directory.



## Native Verification Progress

The first presentation run exposed a real readability failure in the tower height view (factor .73413 below the unchanged .75 floor). The roof-clearance control was grouped with the other three left-column inputs; component semantics, wires, numeric precision and the threshold remain unchanged. A full presentation retry is queued.

The recorder initially had a series-folder scope bug; this was fixed before the replay batch. Two repository admission attempts failed before Python with host-capacity mkdir EPERM and were requeued normally, without bypassing admission. Completed Anatomy EN, Capsule EN/DE and Casting EN lifecycle assertions are not counted as final passes because source fingerprints changed during those runs. Pre-fix Casting DE and Data Trees EN errors are retained as historical records; current on-disk fixes are being verified by fresh processes. Final native acceptance requires both a successful lifecycle result and unchanged semantic inputs.

The final circulation audit completed with matching before/after semantic fingerprints: 1084 of 1084 actual convex support faces are reachable; all 140 capsule landings, 26 lift approaches, 6 bridge approaches, 3 bridge decks, 3 L-return decks, 2 office thresholds, 2 exterior ground seeds and both roof returns are reached. Actual-mesh walking clearance reports zero conflicts. The report also checks full oriented body-strip union coverage with the independent NumPy area oracle. See circulation.md for current fingerprint and coverage details.


Fresh Anatomy EN336.322s and Casting DE322.836s now have successful target records, successful process exits and unchanged semantic input maps. The remaining final native pairs are still in progress; these two are the first accepted GH replay records.


### Native Source Drift at 18:06 UTC

The seven previously successful Grasshopper pairs used the same source map, but that map is now superseded. Independent Python AST hashing detected later semantic writes to Anatomy (17:57:52 UTC), Casting (17:58:45 UTC), Tower (18:00:00 UTC), and Capsule (18:02:54 UTC). Data Trees EN completed its assertions in 1002.163 s but correctly recorded `source_unchanged=false`; it is excluded from final acceptance. Shared physical model/data/renderer fingerprints remain unchanged. The two Tower backups and full twelve-contract retry are retained through completion, but their earlier source map cannot certify the newer scene sources. No source write was performed by this verification worker during these runs. Final native acceptance remains pending stable-source replays.


### Ordered Animation Phase Regression

The newer Casting source failed the real German lifecycle after 235.291 s with unchanged inputs: its `fail` clause passed a lone `PartMorph` to `then`, while `step` expands the queued phase into `Scene.play`. The authored singleton tuple was missing its comma. The fix adds only that comma, preserving the newer animation sequence. `animation-phases.json` and `check-animation-phases.py` specify and verify sequence-shaped arguments for every authored ordered phase; the independent Manim lifecycle is the runtime oracle. Current-map native replays are required after this fix at 18:13:32 UTC. The older Tower backups passed assertions in 1100.257 s (EN) and 1096.247 s (DE), but their source maps changed and they remain excluded.


### Dependency-Scoped Native Provenance

The earlier whole-topic-map acceptance policy is superseded for new runs. A native pair now records its selected scene plus the complete visual provenance manifest, recursive literal local imports, actual loaded repository implementations, nested widgets, shared font/art/drafting/geometry/configuration code, bundled font bytes, model/data/schema, native harness and its case inputs, and the recorder/helper implementation. Unrelated discovered topic sources are excluded unless an actual selected dependency imports them. Full presentation contracts retain every topic; Excel contracts also include the transcript files they read. Unused portable engine fixtures are excluded from native scope. Manim library/user/folder configuration files are included when present; package/Python/platform identity and initial versus declared native render settings are recorded explicitly.

Both start and end dependency maps are durably written per subtest and final result. The final ledger requires their equality, current matching dependency hashes, unchanged runtime identity, a unique successful target-scene record and a completed successful result. Historical whole-map records remain unmodified; source-changed records lacking explicit endpoint maps are not retrospectively accepted.

Six portable provenance tests passed in 1.669 s, including unrelated-topic immunity, selected/shared/transitive rejection, binary and external configuration mutation, declared render configuration capture, recorder syntax and an independent NetworkX dependency-closure oracle. The final no-animation smoke captured thirty Grasshopper Anatomy dependency inputs and exited successfully. `scene-provenance.json`, `scene-provenance.py`, `test-scene-provenance.py`, `validate-scenes.py` and `verify-native-ledger.py` are reusable verification inputs.

The Tower program demonstration now explicitly selects `interior="study"` for both B and C. Its portable scene-linked regression passed in 0.059 s: actual C furniture includes a sofa and excludes reference bed/mattress/pillow parts; Rhino validates the closed meshes, and NumPy signed volumes agree with Rhino bounding-box volumes for the sofa solids. The ordered Casting phase regression passed in 0.087 s. Corrected Casting native assertions passed EN in 299.778 s and DE in 329.594 s on unchanged inputs before subsequent unrelated/common-scope changes; they remain historical evidence pending the final frozen-model scoped gate. New native launches are held for the coordinating agent's roof-frame correction.

## Physical Reference and Corrected Pose Integration

All 140 capsule-row XYZ snapshots and shared sampled planes were rebuilt independently from the final raw authored centres and yaw after the 33 positive pose/family corrections. IDs, phases, programs and nominal energy values remain unchanged. GH63 passed (4.466 s), Excel85 passed (35.907 s); both JavaScript/AJV suites passed (GH28 invalid models, Excel18). Physical/nominal dimension basis and analytical-volume tests passed (2 tests, 1.881 s). Current closed-head/glass occlusion proof:6 tests, 8.872 s. See occlusion.md for algorithm, cache safety, exact scope and pending final gates. Native verification remains on hold.

The casting orientation example now consistently uses the actual west-facing A-floor3 Slash capsule, replacing the old B-west L slot whose measured window correctly faces north. Its canonical frame, physical casing and glazing normal are independently checked against raw authored geometry/NumPy. Full GH64 passed4.794s. Workbook scene narration now explicitly distinguishes nominal2.50×4.00m energy/program controls from the physical2.60×4.20m reference dimensions in English and German; transcripts were regenerated with the authored dynamic figure helpers.


## Current Physical Reference Integration

The Capsule scene now targets physical casing controls (2.6 × 4.2 × 2.5 m), includes the −0.1 m rear datum when placing its centre, and displays the actual steel-face/glazing-recess dependency chain. Its eight-slot List Item exercise uses ordinary per-slot parameter trees: each reconstructed historical family is retained while four proposed A–D program variants are selected. Placement animations use canonical body planes, not side-entry door frames. Rear bolt markers now bind the physical rear datum. Nominal program and thermal controls remain separate. New portable control/tree cases were written before these corrections; their queued runs have not yet supplied a baseline-red result.

Exact previsibility source-face coverage passed two tests in 48.038 s on the frozen physical model, including all six families and both core assemblies at full/exterior detail. The exterior core source intentionally excludes stair treads; its required named roles are corner piers, landings and lift shafts. The full-source face oracle separately covers full stair geometry.

Fresh after-panel raster export/render (handle 81330), cold/warm profile (19735), reference helper/tree tests (3290), default tessellation/core inventory (41396), and hidden-hardware sentinel (38477) are retained under repository admission. At this checkpoint the host itself is below the configured free-memory floor, with zero repository jobs running; queued work is not a test pass. Native lifecycle checks remain on hold.


### Complete Core Source Correction

The earlier exterior-only core choice was insufficient: real stair faces can be visible through authored portals. `_core_parts` now uses the complete shared full assembly for both public core and building drawings; visibility then uses actual view/opaque coverage. The public inventory contract requires every full-source part, including stair treads, lift shaft and cabin floor. This supersedes the preceding exterior-only inventory paragraph. No shared physical model changed.

Queued duplicate helper/query requests 94581, 3290 and 94343 were cancelled before admission (non-passes), superseded by the complete portable GH suite 78215 with an explicit repository 2 GB grant, and syntax-only check 68674 with a 256 MB grant. Renderer controls retain their 6 GB admission. No governor is bypassed.


Current Grasshopper JavaScript gate passed: authored schema accepted, 31 invalid models rejected, five canvases consistent, 23 tree/Path Mapper cases reproduced independently, and 14 numeric/domain casts reproduced. All 49 casting cases retain documented rule identities.
