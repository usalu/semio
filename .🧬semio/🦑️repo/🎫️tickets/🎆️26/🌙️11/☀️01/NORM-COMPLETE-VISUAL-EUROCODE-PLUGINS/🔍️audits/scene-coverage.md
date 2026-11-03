# 🧊 Scene Coverage

Read-only audit of each Eurocode scene crate (`🎬️scene/`) against its target spec visualisation and realtime-editor catalogue (`📐️specs/*-target.md`), and against its visual report (`🔍️audits/vis-*.md`, `wp12-en1993-vertical.md`).

Status legend: **implemented** = entity/diagram/handle present and driven by check or snapshot results; **partial** = present but incomplete or approximated (reason given); **missing** = absent, with the blocking snapshot field or "scene work only".

Caveat for all codes: no scene crate has been compiled, and every golden fixture under `🧪️tests/🧫️fixtures/🏛️scene/` is 0 bytes. "Implemented" therefore means code present, not verified by a passing test. Findings are from code reading only; no builds were run.

Scene root for each code: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/<code>/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/`.

## Summary by code

| Code | Implemented | Partial | Missing | Main approximations |
|---|---|---|---|---|
| EN 1990 | 3 | 11 | 7 | sinusoidal and parabolic shapes, fixed alpha for FORM, glyph magnitude without member influence |
| EN 1991 | 9 | 25 | 27 | wind 3D uses tabulated c_pe,10 (disagrees with 2D), evenly spaced positions, literal positions |
| EN 1992 | 5 | 12 | 9 | evenly spread bars, displayed ULS demand with hard-coded factors, analytic per-station moment shape |
| EN 1993 | 0 | 22 | 5 | sinusoidal buckling modes, support factor possibly applied twice, fixed cold-formed profile |
| EN 1994 | 5 | 15 | 17 | uniform stud stations, analytic deflection shape, fixed hogging and cracked zones |
| EN 1995 | 10 | 14 | 18 | sinusoidal vibration mode with fixed amplitude, fixed load positions, evenly distributed fasteners |
| EN 1996 | 5 | 16 | 12 | evenly spaced openings, fixed bar pitch, uniform wind moment |
| EN 1997 | 0 | 16 | 5 | fixed 40 kPa drawing scale, linear load-settlement scaling, index-based pile slots |
| EN 1998 | 0 | 11 (V) + 7 (ED) | 3 (V) + 2 (ED) | mode amplitude 0.03 H, fixed beta and eta, first mode only, joint colour from one worst check |
| EN 1999 | 3 | 12 | 13 | sinusoidal buckling shapes, support factor possibly doubled, fixed shell constants, schematic cold-formed sheets |

---

## EN 1990 (⚖️en1990)

Scope: spec 4.1 (V90.1-V90.8), 4.3, 5.1.

| Spec item | Status | Evidence | Blocking field or reason |
|---|---|---|---|
| V90.1 Combination matrix | partial | `📈️diagrams/🦀️.rs:64-125`, `🧮️combinations/🦀️.rs:120-140` | Click-to-3D routing not wired (`🕹️handles/🦀️.rs:110-125`); no sort by governing member |
| V90.2 psi and gamma explainer | partial | `diagrams.rs:127-172`, `⚓️actions/🦀️.rs:180` | No psi edit handles; no gamma-per-state table |
| V90.3 Reliability | partial | `diagrams.rs:178-263` | RC targets drawn as lines, not bands; FORM uses fixed alpha; K_FI absent |
| V90.4 Member utilisation, 3D | partial | `🏗️members/🦀️.rs:74,110-120`, `🧰️kit/🦀️.rs:141-148` | Colour is max over all checks, not eta_STR; EQU stabilising arrows absent (`rd_equ_stab/destab` never read) |
| V90.5 Deflection | partial | `members.rs:15-18,128-169` | Parabolic shape (approx); L/n limit as text; half-sine mode (approx) |
| V90.6 Bridge SLS | partial | `members.rs:222-290` | Acceleration contour and twist arrows absent; blocking: spatial acceleration and twist distribution |
| V90.7 Load case stacks | partial | `⚓️actions/🦀️.rs:111-144`, `kit.rs:224` | Stack spacing even (approx); magnitude ignores member influence |
| V90.8 Annex D | partial | `diagrams.rs:265-285` | No sample dot-plot; blocking: Annex D test-results list |
| 4.3 3D affordances | partial | `kit.rs:227-231`, `members.rs:110` | No formula in hover; camera, sections, export are viewport work |
| 5.1 Wizard, annex, CC/RC/DSL/IL | missing | absent | Editor-side, no snapshot blocker |
| 5.1 Action table, drag into ExclusiveGroup | missing | `combinations.rs:128` | Blocking: exclusive-group id per action |
| 5.1 psi chips with sliders, NA-DE lock | missing | no handle | Blocking: psi override fields and verified/override flag |
| 5.1 Matrix cells clickable, contribution bar | partial | `diagrams.rs:80-103` | No per-cell stacked contribution series |
| 5.1 G_k and Q_k sliders (live eta, <16 ms) | implemented | `handles.rs:83-108` | Timing gate unmeasured |
| 5.1 beta slider | implemented | `diagrams.rs:188-201` | None |
| 5.1 FORM run, progress and cancel, K_FI suggestion | missing | fixed alpha (`diagrams.rs:218`) | Blocking: resistance and action distributions (kind, mean, sigma or CoV); converged FORM exists in `💡️inferences` but cannot be fed |
| 5.1 Remedies | missing | none | Scene work only |
| 5.1 Preset library | missing | absent | Catalogue asset, no blocker |

Approximations: parabola for deflection (`members.rs:24-26`); half-sine for eigenmodes (`members.rs:28-30`, drawn whenever deck acceleration > 0); glyph magnitude = gamma*psi x characteristic value, member influence only gates visibility (`actions.rs:118-122`); fixed alpha_E -0.7 and alpha_R 0.8 (`diagrams.rs:218`); fatigue on one range ratio (`diagrams.rs:289-296`); k_n interpolated linearly in 1/n (`diagrams.rs:17`); utilisation threshold 0.8 vs spec 0.7 (`diagrams.rs:43-45`).

Missing snapshot fields: resistance and action distributions (FORM); psi override values and verified flag; exclusive-group id per action; spatial deck acceleration and twist samples or mode array; Annex D test-result list; computed K_FI suggestion. Not a snapshot gap: EQU arrows and remedies (scene work).

Claimed vs actual: vis-en1990 describes glyph magnitude as gamma*psi*E, but the code omits member influence; the SLS shape is a parabola, not disclosed.

---

## EN 1991 (🏋️en1991)

Scope: spec 4.2 (E11-E17, E2-E4), 4.3, 5.2.

| Spec item | Status | Evidence | Blocking field or reason |
|---|---|---|---|
| E11 floors by category, q_k slab | partial | `gravity.rs:37-78`; footprint and levels approximated (`gravity.rs:32-34`) | None |
| E11 partitions, Q_k pin, wheel patches | missing | absent | Partition geometry; concentrated-load position (`assumed_partitions`, `assumed_qk_concentrated` exist, unread) |
| E11 self-weight stack | partial | `gravity.rs:80-141` | No sum, no floor legend |
| E12 theta-t chart, t_eq marker | partial | `diagrams.rs` fire_curves | Marker at design duration only; no t_eq, no R bands; blocking: R-class field |
| E12 compartment box, openings | partial | `thermal-fire.rs:80-102` (square box) | Openings not drawn; blocking: opening geometry |
| E12 plume, beam flux, exposure arrows | missing | absent | Localised-fire fields, beam members |
| E13 roof planes, drift wedges | partial | `snow.rs:44-124` | Colour by role, not by s |
| E13 eaves, sliding arrows, snow guard, cylindrical roof | missing | `envelope.rs:13-19` has no cylinder | snowGuard, roofType |
| E13 load-case switcher | partial | `snow.rs:69` | Fixed factors, no switcher |
| E13 map picker, s_k vs altitude | missing | only label (`snow.rs:197`) | Scene work |
| E14 c_pe zones A-J | partial | `wind.rs:33-101` | Colour by role not sign of w; tabulated c_pe,10 used (`wind.rs:45`); hip roofs have no zones |
| E14 internal pressure arrows c_pi | implemented | `wind.rs:75-100` | None |
| E14 q_p(z) profile | partial | `wind.rs:102-122`, `diagrams.rs` qp_profile | No overlay on elevation |
| E14 compass, terrain wheel, mixed profile | missing | `envelope.rs:189` theta fixed at +X | windDirection, terrainSectors[] |
| E14 dynamics, free canopies, bridge F_W | missing | absent | naturalFrequency, mass, damping, canopy kind |
| E15 deck gradient shading | partial | `thermal-fire.rs:55-71` | Uniform glyph only; non-linear profile field |
| E15 joint arrows, bowing, Fig 6.1 | missing | absent | jointLength, bearing |
| E16 Gantt, staged 3D, formwork | missing | no execution module | constructionStages[], formwork (construction_activity unread) |
| E17 impact patch, vehicle, approach | partial | `accidental.rs:25-41` | Fixed height (`accidental.rs:8`); impactHeight |
| E17 tie network, notional removal, key element | missing | absent | ties[], removedElements, keyElement |
| E17 explosion with venting | partial | `accidental.rs:43-68` | Vent areas; energy chart is scene work |
| E2 notional lanes | partial | `traffic.rs:103-119` | Lanes neutral colour |
| E2 LM1 UDL, LM4 crowd, footbridge crowd | implemented | `traffic.rs:114,154,155` | None |
| E2 LM1 tandems | partial | `traffic.rs:120-128` (span/2) | No 0.4 m footprint; no drag handle; blocking: tandem position |
| E2 LM2 axle, LM3 strips | partial | `traffic.rs:136-153` | Fixed positions; LM3 4 axles fixed |
| E2 braking, centrifugal arrows, influence map | missing | absent; CHECKS values unused | Scene work |
| E2 rail LM71/SW, Phi, alpha | missing | no rail kind | structureKind rail, railClass, speed |
| E3 runway, crane bridge, trolley | partial | `traffic.rs:170-195`; fixed runway `layout.rs:22-23` | Runway span, trolley position |
| E3 wheel loads, H_L/H_T/H_S, buffers | partial | `traffic.rs:211-219` | Both wheels equal; Q_r,min and H_T, H_S absent |
| E3 phi2 chart, group selector | missing | absent | None |
| E4 silo shell by p_hf, p_wf, p_vf | partial | `silo.rs:37-52,70` | No pressure colour, no p_vf |
| E4 rect or polygon silo, hopper | missing | `silo.rs:42` cylinder only | siloPlanShape, hopper angle |
| E4 patch load adjustable | partial | `silo.rs:71-91` | Fixed mid-height; blocking: patchHeight, patchAngle |
| E4 p(z) filling and emptying | partial | `diagrams.rs` silo_pressure | Filling only; discharge fields |
| 4.3 hover, click, camera, sections | partial | `kit.rs:227-231`, `wind.rs:72-74` | Camera and section are viewport work |
| 5.2 handles: altitude, wind zone, terrain, eaves, pitch, parapet, snow obstruction, floor area, layer, face, span, width, lane, hoisting, silo, fire height | implemented | `handles.rs:70-176,187` | None |
| 5.2 lane drag with 3 m snap | partial | `handles.rs:153-154`; `SnapRule::NONE` (`handles.rs:53`) | Snap rule |
| 5.2 crane wheel group, span, load tabs | partial | `handles.rs:163-165` | Runway span, wheel positions |
| 5.2 t_eq marker moves | partial | duration marker | t_eq field |
| 5.2 footprint vertices | missing | dimension grips only (`handles.rs:70-72`) | footprintPolygon |
| 5.2 openings drag, dominant opening | missing | absent | openings[] |
| 5.2 compass, terrain wheel, n1 banner | missing | absent | windDirection, terrainSectors, naturalFrequency |
| 5.2 snow guard, zone switch | missing | absent | snowGuard |
| 5.2 floor polygons, partitions | missing | floor area grip only (`handles.rs:118`) | Floor polygon, partitions |
| 5.2 fire A_v, h_eq, q_f,k, R-class | missing | height grip only (`handles.rs:187`) | Fire opening, q_f,k, R-class |
| 5.2 thermal deck, surfacing, joint length | missing | `handles.rs:202` returns none | surfacing thickness, jointLength |
| 5.2 execution timeline | missing | none | constructionStages[] |
| 5.2 accidental patch, ties, removal | missing | `handles.rs:202` | As above |
| 5.2 LM1 tandem drag, alpha sliders, rail class | missing | `kit.rs:22-48` layers only | Tandem position, alpha fields |
| 5.2 silo patch drag, bulk material | missing | none | Patch position |

Approximations: wind 3D zones and c_pi use tabulated c_pe,10 (`wind.rs:45,59,76`) while the 2D cpe diagram uses face values, so 2D and 3D disagree; wind direction fixed (`envelope.rs:189,286`); duo snow factors 0.5 and 1.0 (`snow.rs:69`); obstruction literals 0.5 m, 8 m, 0.3 (`snow.rs:12-14`, `handles.rs:107`); SNOW_MU2_CAP 1.6 vs spec 2.0 (`diagrams.rs:15`); floor plate scaled by sqrt(A/A_footprint) and storey elevations h/n*i (evenly spread) (`gravity.rs:32-34`); stack plan 4.0 m and legend 80 kN/m3 (`gravity.rs:11-12`); fire corner margin 1.0 m (`thermal-fire.rs:10,80`); impact height 1.2 m, spacing 0.8 m, cases at width*i/(n+1) (`accidental.rs:8,10,17`); tandem and LM2/LM3 axles at span/2 +/- literals (`traffic.rs:123,137,140`); runway 18 x 7 m literals (`layout.rs:22-25`); silo shell 0.02 m, patch 0.4 R at mid-height (`silo.rs:10,12,73`); design_horizontal_force ignores annex (CHECKS ~:858). No sin/cos mode shapes, linspace or random positions; only trig is yaw rotation (`layout.rs:37`).

Missing snapshot fields: windDirection; terrainSectors[]; footprintPolygon; openings[]; partitions[] geometry; concentratedLoads[] position; localisedFire {diameter, heatRelease, position}; requiredResistanceClass; constructionStages[] and formwork fields; ties[]; keyElement; removedElements[]; impactHeight; jointLength; bearingType; deck surfacing thickness; naturalFrequency, mass, damping; structureKind rail, railClass, speed; runway span; trolley or bridge position; Q_r,min; H_T and H_S; siloPlanShape; hopper angle; discharge mode; patchHeight; patchAngle; snowGuard; eave overhang; cylindrical roofType. Unread but present: assumed_partitions, assumed_qk_concentrated, construction_activity, assumed_h_net, assumed_silo_pressure, assumed_silo_patch.

Claimed vs actual: vis-en1991 says wind zones show q_p c_pe,10 from face data; code uses tabulated values. It says markers are draggable; bridge-influence markers have binding None (`diagrams.rs` influence). All statuses are code-reading only.

---

## EN 1992 (🏛️en1992)

Scope: spec 5 (VZ-01 to VZ-14) and 6.

| Spec item | Status | Evidence | Blocking field or reason |
|---|---|---|---|
| VZ-01 Rebar cage member | partial | `🏛️members/🦀️.rs:105-147,164-192,203-260` | No per-zone stirrup spacing, lap zones, section-cut slider; blocking: `stirrups.zones[]` |
| VZ-02 Tendon profile | partial | `members.rs:262-301`; `diagrams.rs:90-104` | Analytic parabola, constant P0 and Pm,inf; blocking: `prestress.friction`, `wobble`, `slip`, `profile[]` |
| VZ-03 N-My-Mz surface | partial | `members.rs:401-433`; `diagrams.rs:49-68` | My-N only; Mz absent (`LoadCaseActions` has only m_k) |
| VZ-04 2-D stress block | partial | `diagrams.rs:70-88`; `mechanics.rs:107-118` | Rectangular block only; no Fc/Fs/z labels |
| VZ-05 Punching perimeters | partial | `🕳️punching/🦀️.rs:57-99` | No uout, stud rings, openings, rout; blocking: `punching.studRings[]`, `openings[]`, `rOut` |
| VZ-06 Crack pattern | partial | `members.rs:353-380`; `mechanics.rs:307-319` | No Ac,eff shading; theta and early-age restraint absent |
| VZ-07 Deflection shape | partial | `diagrams.rs:106-122`; `members.rs:149-162` | No curvature or creep t0..tinf; blocking: creep.relativeHumidity, ageAtLoading, evaluationAge |
| VZ-08 Fire isotherms | partial | `diagrams.rs:124-139`; `members.rs:382-399`; `mechanics.rs:344-356` | Semi-infinite only; no contours; blocking: tabulated EN 1992-1-2 field |
| VZ-09 Strut-and-tie | missing | absent | `stm.nodes[]`, `stm.members[]` |
| VZ-10 Shear/torsion truss | missing | absent | `shear.cotTheta`; `stirrups.zones[]`; no VEd(x) |
| VZ-11 Anchor group | partial | `⚓️anchors/🦀️.rs:27-84` | No breakout wedge, splitting, group layout; blocking: `anchorGroup.positions[]` |
| VZ-12 Utilisation heat map | partial | `members.rs:136-144`; `kit.rs:115-123` | Member colour only; no heat-map view or legend |
| VZ-13 Material laws | partial | `diagrams.rs:141-150` | Steel horizontal branch only; no creep or S-N curves |
| VZ-14 Interaction 2-D | partial | `diagrams.rs:49-68`; `mechanics.rs:209-218` | Mz-N, M-kappa, N-kappa absent |
| BarMoved (drag rebar) | missing | grips for count and diameter only (`members.rs:213-216`, `handles.rs:74-108`) | `BarLayer.positions[]` |
| SectionVertexMoved / Inserted | missing | rectangle only (`members.rs:116`) | Section polygon on RcMember |
| BarDiameterChanged / BarAdded | implemented | `handles.rs:74-108`, `members.rs:213-215` | None |
| ConcreteGradeAssigned, steel grade | missing | no grip | Scene work only |
| LoadPreview, NEd/MEdy/MEdz | partial | glyph `members.rs:420-428`, no grip; Mz fixed 0 (`mechanics.rs:273`) | Mz demand |
| LinkSpacingChanged | partial | `members.rs:250` | `shear.cotTheta` |
| TendonControlPointMoved | partial | `members.rs:268-284` | `prestress.profile[]` |
| TendonStressingChanged | missing | absent | `prestress.jackingStress`, `lockSlip` |
| ColumnFootprintResized, opening move | partial | `punching.rs:71-76` | `punching.openings[]` |
| StudRingAdded | missing | absent | `punching.studRings[]` |
| StmNodeMoved | missing | absent | STM model |
| AnchorMoved | partial | `anchors.rs:41-43`; fixed slots `layout.rs:91-92` | `anchorGroup.positions[]` |
| FireTimePreview, R-class scrub | missing | enum only (`mechanics.rs:322-329`) | Scene work only |
| Creep age and RH scrub | missing | absent | creep.relativeHumidity, ageAtLoading |
| ExposureAssigned | missing | `RcMember.exposure` not shown | Scene work only |

Approximations: bars evenly spread (`reinforcement.rs:73,32`); anchors on fixed slots of 6 m (`layout.rs:91-92`); displayed ULS demand with hard-coded 1.35/1.5/1.05 and Mz = 0 (`mechanics.rs:254-274`), feeding the design-point eta_u label, not the check; per-station utilisation = governing ratio times analytic moment shape (`members.rs:199`, `mechanics.rs:282-289`); uncracked EI with display scale clamped 1-500 (`mechanics.rs:303`, `members.rs:157,161`); semi-infinite isotherm with constant diffusivity 0.35e-6 (`mechanics.rs:11,344-355`); zone threshold 0.4, evenly spaced cracks, apex 0.05 m (`members.rs:364,374,375`); wk drawn 10x (`members.rs:18`); tendon parabola and constant P0 (`diagrams.rs:93,102`); control point at 2e (`members.rs:285`); fallback constants 0.9 when bars absent (`mechanics.rs:110,225`); anchor schematic constants (`anchors.rs:32-35`); steel horizontal branch (`diagrams.rs:144`). The oracle (`🔮️oracles/🐍️.py:16-22`) and feature file (`🥒️.feature:22-31`) use the same formula as the scene, so the isotherm check validates code against itself; spec section 7 requires third-party oracles, none used.

Missing snapshot fields: `BarLayer.positions[]`; section polygon; `prestress.friction`, `wobble`, `slip`, `profile[]`, `jackingStress`, `lockSlip`; `stirrups.zones[]`, `shear.cotTheta`; `punching.studRings[]`, `openings[]`, `rOut`; `stm.nodes[]`, `stm.members[]`; `anchorGroup.positions[]`; LoadCaseActions Mz; `creep.relativeHumidity`, `creep.ageAtLoading`, `creep.evaluationAge`; early-age restraint fields.

Claimed vs actual: report says the design-point eta_u label is the check utilisation; it uses illustrative demand. Per-station flexure values are scaled, not computed. Golden `🧪️tests/🧫️fixtures/🏛️office-frame/🔣️.json` is 0 bytes.

---

## EN 1993 (🔩️en1993)

Scope: spec 3 (views 3.1-3.13) and 4.1 (interactions 4.1.1-4.1.11). Aliases: M = members, D = diagrams, K = kit, J = joints, P = plates-and-shells, L = loads-and-supports, H = handles, Y = layout.

| Spec item | Status | Evidence | Blocking field or reason |
|---|---|---|---|
| 3.1 3D heat map, envelope | partial | `M:47-108`; `K:155-161` | No per-station colour (`M:72`, `K:161`); legend viridis to 1.2 |
| 3.2 Extruded sections, overlays | partial | `M:63-66`; `K:181-190` | No radii, taper, stress, class or axis glyphs |
| 3.3 Buckling modes, animated | partial (approx) | `M:9,20,41` sinusoid x 0.04 L; mode 1 fixed | No mode vector or alpha_cr |
| 3.4 LTB shape, moment, Mcr, zg | missing | `D:156-190` shows lambda_LT, chi_LT, M_b,Rd only | Numeric zg absent; no Mcr output |
| 3.5 Joints, bolts, T-stub, prying | partial | `J:24-110` | No T-stub lengths, prying Q, row forces |
| 3.6 Fatigue S-N, spectrum, rainflow | partial | `D:265-290` | Pre-binned spectrum only; no rainflow matrix |
| 3.7 M-N interaction, biaxial, M-V | partial | `D:197-236` | Simplified capacity (`D:205`); no 6.3.3 curves |
| 3.8 Classification blocks | partial | `D:238-263` | No c/t bar; be1/be2 not computed |
| 3.9 Fire curves, theta_cr | partial | `D:292-343` | Nb,fi,t,Rd absent; constants 1.0, 0.7, 1.0 (`D:301`) |
| 3.10 Buckling and LTB curves | partial | `D:57-190` | LTB 6.3.2.2 only (`D:141`); 6.3.2.3 unused |
| 3.11 Moment-rotation Sj,ini | missing | none | `SteelJoint` has no Sj,ini or Mj,Rd |
| 3.12 Plated panels | partial | `P:62-82` | Stiffeners empty (`P:71`); no stress field or mode |
| 3.13 Shells, silos, cable, crane, tower | partial | `P:83-121`; `L:62-100` | Real Janssen pressure; no buckling lobes, cable sag |
| 4.1.1 Section picker | partial | `H:55-65` (snaps to catalogue height) | No list, badges, optimise |
| 4.1.2 Parametric h, b, tf, tw | missing | only h (`H:61`) | Scene work; fields exist |
| 4.1.3 Supports and restraints | partial | `H:67-79`; `M:123` | Supports not draggable; no support position data |
| 4.1.4 Load editing, zg gizmo | missing | fixed points `L:43,46-56` | Load position and numeric zg (`snapshot:538-545`) |
| 4.1.5 Bolt grid | partial | `H:97-103` | No add, box-select, e1/e2 checks |
| 4.1.6 Sliders | partial | `H:44-65,93-94,114` | No bolt diameter, grade, fire time, cycles handles |
| 4.1.7 Inline feedback | partial | `K:155-161,205-207`; `M:100` | No formula with values |
| 4.1.8 Remedies | partial | in editor crate | Scene exposes no list |
| 4.1.9 Annex/NDP panel | partial | `D:79,147,199` | No per-NDP source/verified view |
| 4.1.10 Method switches | missing | annex only | No LTB method field (`snapshot:514-529`) |
| 4.1.11 No raw JSON | not assessed | editor scope | |

Approximations: sinusoidal modes, amplitude 0.04 x L (`M:9,20`), mode 1 always (`M:41`); local-buckling sinusoid, half-waves = round(L/depth) clamped 1-12, amplitude 0.004 L (`members:220-221`); lateral-only LTB overlay (`M:30`); end release from member_type string (`M:12-17`); support types derived from member_type (`M:133,137-141`); restraints at k x spacing, k up to 12 (`M:10,145-156`); unplaced rows at fixed y and slot i x 6 m (`Y:17-23,71`); load shift 0.2 x (index mod 4) m (`L:43`); loads at member end or mid (`L:46-56`); silo pressure zone half-width 0.2 R (`L:68`); fire glyph -0.2 m (`L:103`); fatigue detail at 0.75 L, +0.35 m (`L:119`); bolt shank -0.6 d (`J:81`); clearance 2/3 mm (`J:95`); cable 3.0 m, cold-formed 1.5 m, nominal wire 1 GPa (`P:9-11`); cee profile b = 0.6 b_bar, c = 0.2 b_bar (`P:132`); M-N capacity linear in |N| (`D:205`); unprotected fire constants (`D:301`); section outline ignores corner radius (`K:186`); cold-formed sheets drawn with a fixed schematic profile (`sheets-and-shells:8,11-26`), not disclosed. Oracle `🔮️oracles/🐍️.py:13-20` covers checks curves only, not Ncr, Mcr or M_b,Rd. Golden fixture `🧪️tests/🧫️fixtures/🏛️scene/🔣️.json` is 0 bytes; golden test (`unit:295-303`) fails until regenerated.

Missing snapshot fields: load position x and numeric zg per action; joint Sj,ini, Mj,Rd, rotation capacity; mode vectors, alpha_cr, Ncr/Mcr per mode; per-station utilisation; T-stub lengths, prying Q, row forces; rainflow time series on FatigueDetail; plated-panel stress field and stiffeners; cable length, sag, pretension; support positions and types; numeric fire time and Nb,fi,t,Rd; LTB method selector and per-NDP metadata.

Claimed vs actual: WP12 lists the M-N polygon as delivered without disclosing simplified capacity (`D:205`). "Not done" omits that LTB is 6.3.2.2 only, the moment overlay and Mcr readout are absent, and most section 4.1 interactions are not in the scene. Cold-formed profile, cable length, silo zone width and fixed fatigue/fire glyph positions are not disclosed.

---

## EN 1994 (🧩️en1994)

Scope: spec 5.1-5.5, 6.1-6.12, live-target EN 1994 rows.

| Spec item | Status | Evidence | Blocking field or reason |
|---|---|---|---|
| 5 preamble: camera, layers, key map | partial | `kit.rs:18-29,278` | Layers only; camera, key map absent |
| 5.1 steel I member, class tint | partial | `beams.rs:22-29,94-122` | Welded plates absent; no class tint |
| 5.1 concrete slab above ribs | implemented | `beams.rs:141-154` | Concrete inside ribs not drawn (disclosed) |
| 5.1 sheeting profile | partial | `beams.rs:169-202`; `facts.rs:172-199` | Schematic pitch; no re-entrant option |
| 5.1 reinforcement bars | partial | `beams.rs:243-283` | Hogging bars fixed 0.85-1.15 L (`:268`) |
| 5.1 studs per rib, zone colour | partial | `beams.rs:204-241`; `kit.rs:164-175` | Uniform station pitch (`facts:112,146-151`); colour = ratio x \|1-2u\|, not per-stud; `per_station` None |
| 5.1 effective-width ghost | implemented | `beams.rs:156-167,401-406` | None |
| 5.1 PNA plane | partial | `beams.rs:286-303,419-423` | No drag handle |
| 5.1 elastic neutral axis | missing | absent | Scene work (E_s constant) |
| 5.1 supports, restraints | implemented | `beams.rs:324-345` | Restraints only for unpropped construction |
| 5.1 force-flow arrows | missing | absent | Scene work |
| 5.1 connector force chart | missing | absent | Scene work |
| 5.1 stress diagrams at cut plane | partial | `diagrams.rs:58-107` | No cut plane, strain, elastic stress |
| 5.1 shear surfaces a-a..d-d | missing | absent | Per-surface shear results not in scene |
| 5.1 deflection, creep parts | partial | `beams.rs:124-139`; `diagrams.rs:202-236` | Analytic shape; no creep or shrinkage parts |
| 5.2 column 2-D section, handles | partial | `columns.rs:17-72`; `handles.rs:165-208` | No 2-D diagram; shape inferred from steel area (`facts:218-236`) |
| 5.2 stress blocks, strain plane | missing | absent | Scene work only |
| 5.2 M-N chart, A-C-D-B polygon | partial | `diagrams.rs:288-337` | Absolute kN axes (`:320`) |
| 5.2 demand points, hatch | partial | `diagrams.rs:301,322-331,305` | Single demand; hatch marks alpha_M polygon |
| 5.2 second-order point | missing | absent | No second-order field |
| 5.2 biaxial charts | missing | absent | `ColumnAction` has only m_k_nm |
| 5.2 slenderness panel | implemented | `diagrams.rs:339-380` | None |
| 5.2 table alternative | partial | `diagrams.rs:335` | Polygon ring only, no demands |
| 5.4 utilisation heat map | missing | `kit.rs:134,173` | Per-station results not in report |
| 5.5 stress blocks, M-N, chi | implemented | `diagrams.rs:58-107,296-380` | None |
| 5.5 S-N, categories | missing | absent | Fatigue category table; `fatigue_detail` string only |
| 5.5 Mcr vs L | missing | absent | No Mcr output; `ltb_length_m` exists |
| 5.5 creep deflection | missing | absent | No creep fields |
| 5.5 m-k regression | missing | `diagrams.rs:382-416` plots m_Ed(L) only | Test point data absent |
| 5.5 fire T-t, k0.2 | missing | absent | `fire_rating` string only |
| 5.5 effective-width distribution | missing | absent | Scene work |
| 6.1 span, supports, Le/beff | partial | `handles.rs:71,73-83` | Support markers and Le/Ls absent |
| 6.2 catalogue, grade band | partial | `handles.rs:15,151-161` | 11 HEB only; no steel grade field |
| 6.3 slab, sheeting, gallery | partial | `handles.rs:84-94,139-149` | No gallery; no profile catalogue field |
| 6.4 stud layout | partial | `handles.rs:106-138`; `beams.rs:424-429` | No add-row, fill-to-eta, per-stud record; kt not computed |
| 6.5 PNA gizmo, stages | missing | residual text `beams.rs:298` | No gizmo; stage is free string |
| 6.6 rebar layers, As,min | missing | `beams.rs:264-283,305-316` | No handle; no layer position or cover |
| 6.7 column morph, rebar | partial | `handles.rs:170-199`; `diagrams.rs:296-380` | Size, length, wall only |
| 6.8 slab m-k, punching footprint | partial | `handles.rs:216-226` | Span only; point loads skipped (`slabs.rs:131`) |
| 6.9 fire R-class | missing | absent | Needs EN 1994-1-2 tables |
| 6.10-6.12 chips, remedies, undo | missing | `handles.rs:37-39` | Scene work; undo and report are editor layers |
| Live-target table | partial | `kit.rs:69-83` | No incremental dependency graph |

Approximations: stud stations equal pitch L/n (`facts:112,146-151`); stud colour = governing ratio x |1-2u| (`kit.rs:164-175`); peak 4V/L and shear-flow assume uniform load (`diagrams.rs:244-246`), uniform stud resistance n*P_Rd/(L/2) (`:245`); deflection shape analytic 3.2(u-2u^3+u^4) scaled by a display constant (`beams.rs:135,138`, `diagrams.rs:199,210`); hogging bars and cracked zone fixed at 0.85-1.15 L (`beams.rs:268,314`); column bars fixed 20 mm (`facts.rs:258`); column rebar evenly spread (`columns.rs:31-33,39,66-68`); column section inferred from area with h = 0.5 outer (`facts.rs:210-234`); rib pitch 2 b0 + e (`facts.rs:176-189`); OSF slip constants 0.709/0.4 not calibrated, P_Rk back-computed (`diagrams.rs:13-14,162`); M-N ring anchored at origin (`diagrams.rs:289`).

Missing snapshot fields: per-station and per-zone check results (blocks 5.1 colour, 5.4 heat map); biaxial My, Mz; second-order k; steel grade per product; rebar layer position and cover; sheeting geometry; creep and shrinkage inputs; fire inputs; slab point-load footprint; individual stud records; fatigue detail category table.

Claimed vs actual: approximation list omits fixed hogging and cracked zones, uniform stud stations, analytic deflection shape, fixed 20 mm bars and uniform resistance limit. Deflection overlay follows SLS only in magnitude. Golden fixture dir empty; `golden_scenes_match_the_committed_bytes` (`unit:374-383`) fails until generated.

---

## EN 1995 (🪵️en1995)

Scope: spec section 9 (V-T1 to V-T8) and 11.1.

| Spec item | Status | Evidence | Blocking field or reason |
|---|---|---|---|
| V-T1 Member 3D, extruded section | implemented | `members.rs:43-56,91-150` | None |
| V-T1 taper, curved, pitched, cambered | missing | `section_taper: None` (`members.rs:119,205`) | taper, curve, apex/pitch, camber absent (`schema:465-515`) |
| V-T1 glulam lamellae, CLT layers | partial | `members.rs:15,47-56` (fixed 40 mm) | No CLT layup field |
| V-T1 notches, holes | partial | `members.rs:328-364` | Single end notch; hole fields absent |
| V-T1 utilisation by station | partial | `kit.rs:116`, `members.rs:104` | Member-level only; per_station None |
| V-T1 grain texture, rebar | missing | none | Scene work |
| V-T2 stress overlays | missing | bearing box only (`members.rs:282-308`) | Kernel stress profile not in CheckResult |
| V-T3 connection layers, fasteners | partial | `connections.rs:57-81,14-21` | Washers, rings, plates, rods absent; blocking: fastener type enum, threadLengthM |
| V-T3 a1, a3, a4 dimension chains | implemented | `connections.rs:172-177` | a2 absent |
| V-T3 spacing zones tinted | implemented | `connections.rs:134-137,24-28` | None |
| V-T3 load arrows, Johansen, block shear | missing | generic failure glyph (`connections.rs:190-192`) | Kernel failure geometry; load angle |
| V-T3 connection placement | partial | fixed row at y = -4 m (`layout.rs:18`) | Connection placement not in snapshot |
| V-T4 char and zero-strength layers | implemented (static) | `members.rs:190-244` | None |
| V-T4 time slider | partial | `members.rs:127-139`; `diagrams.rs:173-203` | No animation |
| V-T4 corner rounding, eta(t), cladding | missing | uniform inset (`members.rs:192`) | Cladding thickness; per-time outputs |
| V-T5 f1 over span | implemented | `diagrams.rs:205-221`, `facts.rs:179-181` | None |
| V-T5 velocity response | implemented | `diagrams.rs:231-245` | Limit b = 100 constant (`facts.rs:189`) |
| V-T5 mode shape | partial | 1-D sine, fixed amplitude (`members.rs:76-82`) | Plate dimensions for 2-D mode |
| V-T5 f1 band, w(1 kN) bowl | partial | single limit line (`diagrams.rs:221`) | Unit-load deflection output |
| V-T6 k_c vs lambda_rel | implemented | `diagrams.rs:84-122` | None |
| V-T6 k_crit vs lambda_rel,m | implemented | `diagrams.rs:124-140` | None |
| V-T6 k_mod table, k_def | partial | `diagrams.rs:55-82` | k_def not drawn |
| V-T6 f_h vs angle | partial | `diagrams.rs:250-266` (literal alpha, `:252`) | Scene work |
| V-T6 Johansen vs t1/d | partial | `diagrams.rs:269-286` | Scene work |
| V-T6 n_ef vs a1/d | missing | no n_ef call in facts | Kernel curve |
| V-T6 gamma-method stress | missing | absent | Kernel output |
| V-T6 creep u(t) | missing | `diagrams.rs:142-171` deflection bars only | Time-series output |
| V-T6 d_char(t) | implemented | `diagrams.rs:173-203` | None |
| V-T7 assembly views | missing | none | truss, diaphragm, layup, slip fields |
| V-T8 footbridge | partial | `diagrams` bridge branch, `facts.rs:174`; `members.rs:181-186` | Prestress, planks, fatigue absent |
| 11.1 span handle | implemented | `members.rs:44-54` | None |
| 11.1 b, h handles, lamella snap | partial | `members.rs:61-62,41,72-73` | No 20 mm commercial sizes |
| 11.1 taper, apex, camber handles | missing | no fields | Snapshot fields |
| 11.1 notch and hole placement | partial | `members.rs:102-126` | Hole fields |
| 11.1 restraint markers snap | partial | `members.rs:89-101,318-325` | Node positions |
| 11.1 load arrows draggable | missing | `handles: Vec::new()` (`members.rs:385`); fixed L/2, L/4, L (`members.rs:403-408`) | Per-action position (`schema:420-445`) |
| 11.1 place first fastener, pattern drag | missing | none | Per-fastener coordinates |
| 11.1 rows x columns, traffic light | partial | `connections.rs:155,158-159`; binary tone `diagrams.rs:298` | No amber band |
| 11.1 edge and end distance | implemented | `connections.rs:156-157` | a2 absent |
| 11.1 steel plate vertex drag | missing | rectangle (`connections.rs:69`) | Plate outline |
| 11.1 connection type switch | missing | falls back to dowel (`facts.rs:15`) | Scene work |
| 11.1 remedies | missing | none | Kernel remedies not projected |
| 11.1 utilisation badge, ramp | partial | `kit.rs:116,246` | Viridis 0-1.2 ramp; no badge text |
| 11.1 hover check table | missing | `kit.rs:116` carries check_id only | CheckResult.steps not projected |
| 11.1 panels | missing | none | Scene and UI work |

Approximations: vibration mode sin(pi t) with fixed amplitude 0.04 x span, not eigenvector; only f1 kernel-driven; unit test checks frequency only (`members.rs:80`, unit tests:195). Deformed shape analytic x w_fin (`members.rs:84-87`), two-span shape uses sin (`members.rs:68-74`). Restraints at k x spacing, capped at 12 (`members.rs:11,318-325`). Point loads at fixed L/2, L/4, L (`members.rs:403-408`). Lamella 40 mm fixed (`members.rs:15,47-56`). Char removed uniformly (`members.rs:192`). Bolt class inferred from f_u,k > 450 MPa (`connections.rs:18`). Layers sized to fastener extent (`connections.rs:69-71`). Unknown fastener becomes dowel (`facts.rs:15`). Fasteners per row = round(number/rows) (`facts.rs:35`). a2 = table minimum (`facts.rs:38`). rho default 350 (`facts.rs:36`). Floor strip 0.05 m, deck 0.022 m, zeta 0.01 (`facts.rs:170,172`). Literal arguments at `diagrams.rs:252,290`. Viridis ramp, not spec thresholds (`kit.rs:246`).

Missing snapshot fields: member taper, curve, apex/pitch, camber; CLT layup; holes and second notch; action position along span; per-fastener positions; a2 spacing; fastener type enum with washer, ring, plate, rod, thread length; steel plate outline; connection placement reference; cladding thickness; plate dimensions and orthotropy; prestress, planks, deck geometry; truss, diaphragm, shear-wall geometry; kernel outputs not projected (per-station utilisation, stress profiles, n_ef, gamma-method, creep series, eta(t), remedies).

Claimed vs actual: vis-en1995 lists the vibration mode as implemented; it is an analytic sine (`members.rs:80`), and the unit test checks only the frequency. Truthfulness flags omit restraint cap, fixed load positions, even fastener distribution, bolt-class inference and dowel fallback. Report says Phase 1 complete; crate never compiled, fixture absent.

---

## EN 1996 (🪨️en1996)

Scope: spec section 10 (V-M1 to V-M9) and 11.2.

| Spec item | Status | Evidence | Blocking field or reason |
|---|---|---|---|
| V-M1 zones, openings, lintels, floors | partial | `walls.rs:40-127` | Opening positions evenly spread (`layout.rs:60`); floor stub schematic (`walls.rs:115-118`) |
| V-M1 running bond, overlap | partial | `bond.rs:21-79`, overlap `:70,77` | Units touching openings dropped, not cut (`bond.rs:61`) |
| V-M1 header, English, Flemish bonds | missing | half-unit offset hard-coded (`bond.rs:58`) | No bond-pattern field |
| V-M1 ring beams, piers | missing | absent | No ring-beam or pier data |
| V-M1 instancing, LOD | missing | budget fallback only (`lib.rs:56`, `bond.rs:45`) | Renderer work |
| V-M2 e(z) top, mid, bottom | implemented | `diagrams.rs:96-124` | None |
| V-M2 Phi(z) | partial | `diagrams.rs:126-144` | Three stations, joined linearly (`:139`) |
| V-M2 N_Ed vs N_Rd | partial | `diagrams.rs:146-170` | Three stations (`:163-164`) |
| V-M2 governing sections | implemented | `diagrams.rs:128-133` | None |
| V-M2 slenderness graphic | partial | `diagrams.rs:187,196` | No buckling shape; `supportSides` count only |
| V-M3 yield-line pattern | partial | `loads.rs:92-100,112-122` | Corner offset heuristic (`loads.rs:113`); ridge at mid-height (`loads.rs:94`); mu unused |
| V-M3 moment contours, arching | missing | absent | No arching input |
| V-M3 wind arrows | partial | `loads.rs:104-111` | One uniform zone |
| V-M3 utilisation shading, supports | partial | `loads.rs:114-122`, `walls.rs:135-143` | Supports by side count only |
| V-M4 shear wall | partial | `walls.rs:146-156` | Flange width and thickness absent |
| V-M5 60-degree spread, bearing handle | partial | `loads.rs:84-90,72-80`; `handles.rs:94-109` | Load position even (`loads.rs:61`); no `positionM` |
| V-M5 bearing area, beta, crushing | missing | `bearingAreaM2` unused | Scene work |
| V-M6 bars and layers | partial | `reinforcement.rs:8-51` | Fixed pitch 0.6 m, layer 0.4 m; no count or spacing fields |
| V-M6 M-N curve | partial | `diagrams.rs:332-355` | Moment w h^2/8 (`:339`); one steel row (`:337`) |
| V-M6 anchorage, strain | missing | absent | No anchorage field |
| V-M7 cavity wall | missing | absent | No cavity fields |
| V-M8 fire REI t_min | implemented | `diagrams.rs:303-330` | Missing check gives 0 mm silently (`:317`) |
| V-M8 plaster thickness | missing | absent | No plaster field |
| V-M9 f_k over f_b, f_m | partial | `diagrams.rs:202-230` | 2-D curves only |
| V-M9 Phi vs e/t | partial | `diagrams.rs:172-200` | Axes inverted vs spec |
| V-M9 f_vk line with cap | implemented | `diagrams.rs:231-263` (cap `:238`) | None |
| 11.2 wall footprint polyline | missing | rectangles only (`layout.rs:107-118`) | Footprint and placement fields |
| 11.2 height, thickness drag | partial | `handles.rs:50-51` | Snap not on unit module |
| 11.2 opening drag, brick snap | partial | `handles.rs:63-65` | No brick grid; no opening x field |
| 11.2 edge support cycle | partial | `handles.rs:52`; `walls.rs:135-143` | `supportSides: u8` |
| 11.2 floor bearing depth | implemented | `handles.rs:69-82` | None |
| 11.2 concentrated-load footprint | partial | `handles.rs:94-109` | No load position |
| 11.2 reinforcement placement | missing | none | Bar layout fields |
| 11.2 cavity tie auto-layout | missing | absent | Cavity and tie fields |
| 11.2 pickers, fk readout | partial | `diagrams.rs:225` | No pickers; no derivation text |
| 11.2 Phi gauges, N_Rd bar | partial | `diagrams.rs:139,163-164` | Line plots, not gauges |
| 11.2 slenderness explanation | partial | `diagrams.rs:187,196` | Value only |
| 11.2 unit length live update | partial | `bond.rs:70,77` | No handle for `unitLengthM` |
| 11.2 remedies | missing | none | Scene work |

Approximations: openings evenly between piers (`layout.rs:60-67`); concentrated loads at i/(n+1) L (`loads.rs:59-61`, `handles.rs:97-102`); yield-line ridge h/2 and corner offset a = min(l/2, h/2 sqrt(m2/m1)) (`loads.rs:94,113`); uniform w_e over face (`loads.rs:104-108`); bars at fixed 0.6 m pitch, diameter from A/n, clamped 4-32 mm (`reinforcement.rs:8,15,28-30`); layers fixed 0.4 m (`reinforcement.rs:9,40-45`); wind moment w h^2/8 regardless of edges (`diagrams.rs:339`); single steel row at t/2 (`diagrams.rs:337`); half-unit offset and dropped units (`bond.rs:58,61`); zones are fixed thirds of height (`walls.rs:47-51`); constants 0.2 m slab, 0.2 m lintel, 0.25 m bearing (`walls.rs:9-12,115-118`); fire marker 0 mm fallback (`diagrams.rs:317`); thresholds 0.9 vs 0.8 and duplicated zone logic (`kit.rs:146-148`, `diagrams.rs:37`); snap grids 0.05 m and 0.005 m (`handles.rs:49-65`); fixed 3 m gap (`layout.rs:17,115`). No TODO, placeholder or linspace.

Missing snapshot fields: wall footprint and placement; `WallOpening` x-position; `ConcentratedLoad` position; per-edge support kinds; bond-pattern enum; ring-beam and pier data; flange width and thickness; cavity width, leaf, tie type, density, positions; bar diameter, spacing, layer spacing, anchorage length; plaster thickness.

Claimed vs actual: vis-en1996 says units are cut by openings; `bond.rs:61` drops them. Golden fixture 0 bytes. Zone ratios recomputed in scene (`walls.rs:61`), and the test compares against that duplicate, not the report's result.

---

## EN 1997 (🌍️en1997)

Scope: V97-1 to V97-10, ED-1 to ED-10, section 6. No item fully implemented.

| Spec item | Status | Evidence | Blocking field or reason |
|---|---|---|---|
| V97-1 Soil strata, foundation | partial | `ground.rs:26-48`; `footings.rs:36-74,171`; `handles.rs:59-124` | L' absent (`mechanics.rs:40-43`); no z_lim; no DIN 4023 hatch; no eccentricity grip |
| V97-2 Bearing mechanism | partial | `footings.rs:102-140`; `mechanics.rs:67-95` | Failure mode always Other (`footings.rs:137`); s, i, b, d and q'/gamma B' absent |
| V97-3 Earth pressure | partial | `walls.rs:80-103`; `diagrams.rs:38-84` | No water pressure (`mechanics.rs:120-122`); no shear or moment curves |
| V97-4 Slip circles | partial | `slopes.rs:59-90`; `mechanics.rs:168-233` | Grid heat map not emitted; no centre or radius grip |
| V97-5 Pile group 3D | partial | `piles.rs:29-74`; `handles.rs:154-165` | No NSF, neutral plane, per-pile N_i or positions |
| V97-6 Settlement contour | partial | `settlement.rs:21-69` | No contour lines; isobars unused (`mechanics.rs:240`); no t90; exaggeration fixed (`settlement.rs:8`) |
| V97-7 Hydraulic, uplift | partial | `uplift.rs:11-54` | No flow net, seepage, exit gradient |
| V97-8 Anchored wall | missing | absent | No anchor entity |
| V97-9 Dewatering | missing | absent | No wells entity |
| V97-10 Ground investigation | partial | `diagrams.rs:86-125,215-216` | fs, Rf, Ic absent; `cpt_qc`, `spt_n` scalars |
| ED-1 Drag B, L, d | partial | `handles.rs:120-122`; `footings.rs:142-162` | Eccentricity core absent |
| ED-2 Drag load tip | partial | `handles.rs:124` (H only) | No grip for M or V |
| ED-3 Layer interface, chips | partial | `handles.rs:59-88` | No inline chips or characteristic badge |
| ED-4 Drag water level | partial | `handles.rs:90-105` | No water term; no gamma' swap (`ground.rs:25`) |
| ED-5 Design approach, annex | missing | absent | Scene work only |
| ED-6 Paste CPT/SPT table | missing | absent | No per-depth table field |
| ED-7 Wall vertices, slope, delta | partial | `handles.rs:130-140` | No backfill-slope field; delta slider absent |
| ED-8 Wall toe, prop | partial | `handles.rs:137` | No prop field; no moment diagram |
| ED-9 Slip circle centre, radius | missing | absent | No centre or radius field |
| ED-10 Pile length, diameter, count | partial | `handles.rs:160-165,107-113` | No add-pile, no per-pile move |
| Section 6 incremental, progress, cancel | partial | `kit.rs:74-91` | `slopes.rs:60` runs synchronously (168 circles); no progress or cancel |

Approximations: fixed 40 kPa per m drawing scale (`walls.rs:10,80-87`, visual only, disclosed); increased active = mean(Ka, K0) replacing DIN 4085 (`mechanics.rs:110`, disclosed); no groundwater term (`mechanics.rs:120-122`); passive wedge bound to active check, undisclosed (`walls.rs:74`); slab 0.12 H heuristic (`walls.rs:11-15`); WALL_LENGTH 6.0 m constant (`layout.rs:29`); passive wedge at 45-phi/2 (`walls.rs:76-79`); phi default 30 degrees, failure mode Other (`footings.rs:105,137`); L' not computed (`footings.rs:34`); bearing resistance drained-only, undisclosed (`diagrams.rs:172-173`); load-settlement curve is linear scaling of one point, undisclosed (`diagrams.rs:177-180`); oedometric settlement with 10 sublayers replaces SLS-01 elastic (`mechanics.rs:245-259`); exaggeration 25 and 21x13 grid constants (`settlement.rs:8-10`); one unit shaft resistance for every layer; every pile gets group utilisation (`piles.rs:31,44,57`); even i/n pile grid (`mechanics.rs:268-273`); index-based slots (`layout.rs:111-128,23-26`); fixed 2.0 m depth and 4 m side, uniform pore pressure (`uplift.rs:9,33`, `layout.rs:30`); all slices use governing layer properties (`slopes.rs:17-20,31`); 12 slices and 168 trial circles (`mechanics.rs:7,218-222`).

Missing snapshot fields: placements origin and yaw (`handles.rs:110` writes `placements[id].origin` but `En1997Snapshot` has no such field); pile positions, N_i, lateral load; per-layer skin friction, negative skin friction, neutral plane; CPT fs, Rf, Ic and per-depth tables; cv and drainage; wall backfill slope, surcharge extent, wall length, props, vertices; slope vertices, water polyline, slip centre and radius; anchors, wells, seepage geometry; failure-mode result (check output not read).

Claimed vs actual: vis-en1997 lists load-settlement without noting the linear scaling and drained-only resistance. Report says slip-circle heat map needs snapshot fields; it needs none (`mechanics.rs:215-233`). Golden `🧪️tests/🧫️fixtures/🌍️compliant-scene.json` directory is empty, so `golden_scene_matches_committed_bytes` (`tests/unit:101-108`) will fail.

---

## EN 1998 (🫨️en1998)

Scope: V98-1 to V98-14 (5.2) and ED-5 and ED-11 to ED-18 (6). No item fully implemented: viewports 11 partial, 3 missing; editor 7 partial, 2 missing.

| Spec item | Status | Evidence | Blocking field or reason |
|---|---|---|---|
| V98-1 3-D building, masses, CoM/CoS, e0 | partial | `🏢️buildings/🦀️.rs:54-199`; `handles.rs:75-84` | No torsional radius ellipse; e_ai labels only (`buildings.rs:191-198`); blocking: storey torsional stiffness |
| V98-2 Mode shapes, animated | partial | `buildings.rs:40-46,117,137`; `🧮️response/🦀️.rs:92-101`; eigen solve `schema:541-600` | Real Jacobi solve, but static envelope, constant amplitude, X only, first mode only |
| V98-3 Storey drift bars | partial | `buildings.rs:217-236`; `diagrams.rs:135-180` | Bars = ratio x 2.0 m, not per mille; only 1.0 limit line (`diagrams.rs:165`) |
| V98-4 Response spectrum | partial | `diagrams.rs:43-133` | No log axes, ground-type ghosts, displacement spectrum; beta 0.2 literal (`diagrams.rs:44`); q from first system (`diagrams.rs:67`) |
| V98-5 Seismic zone map | partial | `diagrams.rs:182-200` | Scatter of a_gR over zones; no map; blocking: municipality dataset |
| V98-6 Capacity-design joint | partial | `buildings.rs:153-174` | Boxes only; blocking: beam and column M_Rd (`En1998Member`, snapshot 571-590) |
| V98-7 Plastic mechanism | missing | absent | Blocking: member M_Rd |
| V98-8 Pushover curve | missing | absent | Blocking: capacity curve, target displacement |
| V98-9 Tank, silo sloshing | partial | `vessels-and-towers.rs:35-52` | Shell and content only; no impulsive or convective masses |
| V98-10 Liquefaction | missing | absent | Blocking: CSR/CRR, groundwater, N1,60 |
| V98-11 Mononobe-Okabe | partial | `ground-and-bridges.rs:49-64` | Thrust line only; no static or dynamic pressure, no kh slider |
| V98-12 Tower, mast | partial | `vessels-and-towers.rs:66-92` | Cantilever CHS; no mass nodes, mode shape, shear or moment; blocking: per-node masses |
| V98-13 Bridge | partial | `ground-and-bridges.rs:66-93` | No displacement arrows or gap bars; geometry hard-coded |
| V98-14 Seismic joint gap | partial | `buildings.rs:238-258` | Gap and red style only; no displaced profiles |
| ED-5 Annex switch | partial | `change-annex` mutation; no handle | No factor re-apply or mu diff |
| ED-11 Storey mass, stiffness, height | partial | `handles.rs:75,78` | No stiffness handle; regularity flags not drawn |
| ED-12 q slider | partial | q at `diagrams.rs:67`, `response.rs:77` | No q grip; no capacity-design toggle |
| ED-13 Zone, ground, importance | partial | `diagrams.rs:186-188` | No ground-combo or importance binding |
| ED-14 Drag CoM/CoS | partial | `handles.rs:81-84`; `buildings.rs:187-189`; `response.rs:82-87` | Regularity test absent; blocking: torsional stiffness |
| ED-15 Member M_Rd at joints | missing | absent | Blocking: M_Rd per member |
| ED-16 Mode scrubbing | partial | first mode, `buildings.rs:40-46`, `response.rs:98` | No scrub, no M_eff table; blocking: modal combination rule |
| ED-17 Tank fill | partial | `handles.rs:93-100`; `vessels-and-towers.rs:47-52` | No filling-ratio grip |
| ED-18 Liquefaction groundwater, N | missing | absent | Blocking: groundwater and N inputs |

Approximations: mode amplitude 0.03 x total height replaces u = A phi sin(wt) (`buildings.rs:15,117,137`); mode along X only, scale and participation 1.0 (`buildings.rs:43-45`); stiffness_x only, first mode only (`response.rs:93,98`); mode chip uses eigen period, T1 marker uses ct or given T1, so one view can show two periods (`buildings.rs:45`, `diagrams.rs:80`); columns and joints evenly spread i/(n-1) (`buildings.rs:140-141,159-160`); drift bar = ratio x 2.0 m, height 0.6 h, z 0.2 h (`buildings.rs:221-223`); 1.3 capacity-design rule stated as fact, not evaluated (`buildings.rs:166`); vertical spectrum 0.9 and corners hard-coded (`diagrams.rs:12-13`, `schema:504`); beta 0.2, eta 1.0 (`diagrams.rs:44`); utilisation bands 0.8 and 1.0 vs spec 0.7/0.9/1.0 (`diagrams.rs:36`); joint demand uses drift_x sums for both directions (`response.rs:104-106`); torsion L_e for along_y uses plan length, possibly swapped vs EN 1998-1 4.3.3.2.2 (`response.rs:81`, verify); footing, wall, deck, pier constants (`ground-and-bridges.rs:9-16,36,75`); tower diameter clamp(h/15, 0.6, 6) and base shear M/(2/3 h) (`vessels-and-towers.rs:70,82`); every joint coloured by building-wide worst check (`buildings.rs:155,164`); column size default 0.4 m (`buildings.rs:28`).

Missing snapshot fields: storey torsional stiffness or radius; M_Rd, M_Rb, M_Rc per member (`part_1::strong_column_ratio` at `schema:624` has no inputs); pushover capacity curve and target displacement; liquefaction CSR/CRR, groundwater, N1,60, fines; bridge span and pier positions; tower per-node masses; column grid positions per storey; modal combination rule and mode count; placements passed as empty slice by editor. Effective mass, Mononobe-Okabe kh, sloshing masses and regularity flags are computable from existing fields.

Claimed vs actual: report cites `🗑️generated/vis-en1998/check.txt`, which does not exist; fixtures empty. Report says joints coloured by governing member check; code uses one building-wide worst check. Report says torsional ellipse and pushover curve are not required; spec 5.2 lists both (V98-1, V98-8).

---

## EN 1999 (🪶️en1999)

Scope: spec 5.3-5.5 and section 6 (EN 1999 parts of `en1994-en1999-target.md`).

| Spec item | Status | Evidence | Blocking field or reason |
|---|---|---|---|
| 5 preamble: camera, keyboard map | missing | none; layers only (`kit.rs:8-21`) | Viewer and editor work |
| 5.3 Extrusion outline with ids | partial | `kit.rs:120-136`; label id and designation only (`members.rs:293`) | No thickness labels; Rect fallback (`kit.rs:134`) |
| 5.3 HAZ bands, rho_o,haz ramp | partial | `members.rs:159-202` (style 0.55 + Dots, `:188`) | One Weld colour; no ramp |
| 5.3 Weld run centreline with bead | missing | one fillet line per connection (`connections.rs:64-83`) | Weld-run path (`WeldGroup` `snapshot:178-184`) |
| 5.3 bhaz dimension handle | partial | `handles.rs:169` on connection welds only | Extrusion HAZ width not editable |
| 5.3 Effective section ghost | implemented | `members.rs:204-249`; `facts.rs:57-81` | Mode overlay approximate |
| 5.3 Local buckling shapes | partial | `members.rs:220-221` sinusoid | No mode-shape field; no internal and outstand split |
| 5.3 Flexural, LTB shapes, slider | partial | `members.rs:18,33,30` | No mode vectors; no LTB twist; no slider |
| 5.3 Classification colour mode | missing | class only in 2-D (`diagrams.rs:246-288`) | Scene work |
| 5.3 Section wall-thickness handles | implemented | `handles.rs:123,125-128` | Covers I, box, channel, tube |
| 5.3 Section vertex drag | missing | absent | Outline vertices (`snapshot:56-73` kind and elements only) |
| 5.3 Section presets, symmetry | missing | `kit.rs:124-134` | Scene and editor work |
| 5.4 Utilisation along length | missing | `stations: Vec::new()` (`members.rs:179,236,277`) | Per-station output not produced |
| 5.4 Check type selection | partial | `kit.rs:166-174` | Per-type selection absent |
| 5.4 Sortable utilisation table | missing | per-diagram twins only | Editor work |
| 5.5 Buckling curves, marker | implemented | `diagrams.rs:92-135` | Support-factor issue (below) |
| 5.5 M-N polygon | partial | `diagrams.rs:196-244` | N-My only; M_z absent |
| 5.5 S-N with categories | partial | `diagrams.rs:15,376` | Single range only (`schema:224-236`) |
| 5.5 Mcr vs L | partial | `diagrams.rs:173-191` | Plots M_b,Rd only; M_cr computed at `:153` |
| 5.5 HAZ strength map | partial | `diagrams.rs:317-365` | Curves, not 2-D map |
| 5.5 Fire T-t, k0.2 | implemented | `diagrams.rs:401-451` | theta_cr literal fallback 550 (`:411`) |
| 6 Extrusion vertex drag | missing | width and depth only (`handles.rs:125-126`) | Outline vertices |
| 6 Weld run placement | missing | per-plate grip (`handles.rs:130-134`) | Weld-run field |
| 6 HAZ width override | partial | `handles.rs:169` writes `welds.hazExtent` | Extrusion HAZ derived (`facts.rs:56`) |
| 6 Member length, supports | partial | length (`handles.rs:111`) | Support condition not editable |
| 6 Bolt pattern designer | partial | `handles.rs:155-161` | Limits hard-coded (0.02-0.4 m); capacities not projected |
| 6 S-N detail picker | partial | `fire-and-fatigue.rs:39-63` | No icon; fixed position 0.75 L (`:42`) |
| 6 Shell radius, thickness, BC | partial | `handles.rs:194-196`; `diagrams.rs:453-476` | No BC field (`schema:268-278`) |
| 6 Alloy, temper picker | missing | reference table only (`editor catalogue.rs:24-33`) | Scene and editor work |
| 6 Spectrum edit | missing | single-range fatigue | Spectrum block list |

Approximations: buckling shape sin(pi t), amplitude 0.04 L (`members.rs:11,18,33`); local-buckling sinusoid, half-waves round(L/depth) clamped 1-12, amplitude 0.004 L (`members.rs:220-221`); lateral-only LTB (`members.rs:30`); support factor k (0.7, 2.0, 1.0 literals) multiplies buckling length again, suspected double application (`diagrams.rs:49-55,96-100`), verify; shell alpha_x fixed to class B, beta 0.60, eta 1.0, lambda_0 0.20 (`diagrams.rs:457-459`); theta_cr literal 550.0 fallback (`diagrams.rs:411`); cold-formed sheets drawn with one schematic top-hat (`sheets-and-shells:8,11-26`), not the real profile; unmatched kinds fall back to Rect or Circle (`kit.rs:128,134`); fixed fractions of L for labels and glyphs (`members.rs:194,208`, `fire-and-fatigue.rs:18,42`); bolt shank 0.6 d (`connections.rs:137`); no per-station data (`members.rs:179,236,277`).

Missing snapshot fields: section outline vertices or holes; weld run path (`WeldGroup`); per-station check results and mode vectors; spectrum block list; fatigue detail location on member; shell boundary condition; cold-formed profile dimensions; placement persistence (scene accepts placements, snapshot does not persist, report 4.4).

Claimed vs actual: report says cold-formed sheets are a ThinWalled top-hat profile; it is a fixed schematic, not disclosed in section 4. Report says every diagram has a bound check id; several markers have `bound_check: None` (`diagrams.rs:298,335-336,444`). Report's approximations omit the support-factor double application, fixed shell constants and theta_cr literal.

---

## Consolidated missing snapshot fields per artifact

- **EN 1990**: resistance and action distributions (FORM); psi override values and verified flag; exclusive-group id; spatial deck acceleration and twist samples or mode array; Annex D test-result list; computed K_FI.
- **EN 1991**: windDirection; terrainSectors[]; footprintPolygon; openings[]; partitions[] geometry; concentratedLoads[] position; localisedFire; requiredResistanceClass; constructionStages[] and formwork; ties[]; keyElement; removedElements[]; impactHeight; jointLength; bearingType; surfacing thickness; naturalFrequency, mass, damping; rail structure fields; runway span; trolley or bridge position; Q_r,min; H_T, H_S; siloPlanShape; hopper angle; discharge mode; patchHeight, patchAngle; snowGuard; eave overhang; cylindrical roofType.
- **EN 1992**: BarLayer.positions[]; section polygon; prestress friction, wobble, slip, profile[], jackingStress, lockSlip; stirrups.zones[]; shear.cotTheta; punching studRings[], openings[], rOut; stm.nodes[], stm.members[]; anchorGroup.positions[]; LoadCaseActions Mz; creep relativeHumidity, ageAtLoading, evaluationAge; early-age restraint.
- **EN 1993**: load position and numeric zg; joint Sj,ini, Mj,Rd, rotation capacity; mode vectors, alpha_cr, Ncr/Mcr; per-station utilisation; T-stub lengths, prying Q, row forces; rainflow time series; plated-panel stress field and stiffeners; cable length, sag, pretension; support positions and types; numeric fire time and Nb,fi,t,Rd; LTB method selector.
- **EN 1994**: per-station and per-zone check results; biaxial My, Mz; second-order k; steel grade per product; rebar layer position and cover; sheeting geometry; creep and shrinkage inputs; fire inputs; slab point-load footprint; individual stud records; fatigue detail category table.
- **EN 1995**: member taper, curve, apex/pitch, camber; CLT layup; holes and second notch; action position along span; per-fastener positions; a2 spacing; fastener type enum (washer, ring, plate, rod, thread length); steel plate outline; connection placement reference; cladding thickness; plate dimensions and orthotropy; prestress, planks, deck geometry; truss, diaphragm, shear-wall geometry; kernel outputs (per-station utilisation, stress profiles, n_ef, gamma-method, creep series, eta(t), remedies).
- **EN 1996**: wall footprint and placement; WallOpening x-position; ConcentratedLoad position; per-edge support kinds; bond-pattern enum; ring-beam and pier data; flange width and thickness; cavity width, leaf, tie type, density, positions; bar diameter, spacing, layer spacing, anchorage length; plaster thickness.
- **EN 1997**: placements origin and yaw (written but not in snapshot); pile positions, N_i, lateral load and stiffness; per-layer skin friction, negative skin friction, neutral plane; CPT fs, Rf, Ic and per-depth tables; cv and drainage; wall backfill slope, surcharge extent, length, props, vertices; slope vertices, water polyline, slip centre and radius; anchors, wells, seepage geometry; failure-mode result.
- **EN 1998**: storey torsional stiffness or radius; M_Rd, M_Rb, M_Rc per member; pushover curve and target displacement; liquefaction CSR/CRR, groundwater, N1,60, fines; bridge span and pier positions; tower per-node masses; column grid positions per storey; modal combination rule and mode count; placements passed as empty slice.
- **EN 1999**: section outline vertices or holes; weld run path; per-station check results and mode vectors; spectrum block list; fatigue detail location; shell boundary condition; cold-formed profile dimensions; placement persistence.

## Cross-cutting findings

1. No scene has been compiled or tested; all golden fixtures are empty, so the golden tests fail as committed.
2. Evenly spread positions (i/n) for bars, columns, piles, openings, loads, anchors or stations are used in EN 1991, 1992, 1994, 1995, 1996, 1997 and 1998.
3. Sinusoidal or analytic mode and deflection shapes replace eigenvectors or elastic lines in EN 1990, 1993, 1994, 1995, 1998 and 1999. Only EN 1998's eigen solve is real, and only as a static envelope.
4. Per-station or per-point check results are not projected anywhere; utilisation colours are governing-check values or fixed ramps.
5. Several visual reports understate approximations. Notable cases: EN 1990 glyph magnitude, EN 1992 design-point label, EN 1993 cold-formed profile, EN 1995 vibration mode, EN 1997 load-settlement scaling, EN 1998 joint colour, EN 1999 support factor.
