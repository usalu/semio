# Family-Local Setter RHS Reachability Candidates — 2026-10-05

Read-only current literal family registry and macro-body closure. Shared enter/reset excluded. Shared variable reads in other family owners do not count. Writes are subtracted for common assignment functions; dynamic cs names/aliases, helper-defined keysets and conditional variants need adjudication. Direct named family keysets only in first pass; generic/inherited keysets are not yet exhaustive. No runtime tests executed.

Static family bodies: 240; direct named setter controls examined: 1677; no local read candidates: 15.

| Family | Control | Carrier | Local RHS count | Key source | Owner source |
|---|---|---|---|---|---|
| arch-plan | id | l_semio_viz_dg_col_id_tl | 0 | semio-viz-diagram-architecture.sty:179 | semio-viz-diagram-architecture.sty:299 |
| pm-gantt | lane | l_semio_viz_dg_col_lane_tl | 0 | semio-viz-diagram-process.sty:162 | semio-viz-diagram-process.sty:327 |
| seating | label | l_semio_viz_dg_col_label_tl | 0 | semio-viz-diagram-process.sty:1021 | semio-viz-diagram-process.sty:1135 |
| spatial-layout | group | l_semio_viz_dom_group_tl | 0 | semio-viz-domain.sty:662 | semio-viz-domain.sty:1073 |
| schedule | time | l_semio_viz_dom_id_tl | 0 | semio-viz-domain.sty:1796 | semio-viz-domain.sty:1844 |
| schedule | labels | l_semio_viz_dom_labels_bool | 0 | semio-viz-domain.sty:1796 | semio-viz-domain.sty:1844 |
| version-control | time | l_semio_viz_dom_id_tl | 0 | semio-viz-domain.sty:1825 | semio-viz-domain.sty:1859 |
| version-control | labels | l_semio_viz_dom_labels_bool | 0 | semio-viz-domain.sty:1825 | semio-viz-domain.sty:1859 |
| sci-pathway | mode | l_semio_viz_bio_mode_tl | 0 | semio-viz-scientific-biology.sty:618 | semio-viz-scientific-biology.sty:628 |
| sci-clinical-timeline | laneHeight | l_semio_viz_bio_lane_fp | 0 | semio-viz-scientific-biology.sty:1640 | semio-viz-scientific-biology.sty:1648 |
| sci-circuit | mode | l_semio_viz_eng_mode_tl | 0 | semio-viz-scientific-engineering.sty:42 | semio-viz-scientific-engineering.sty:51 |
| sci-abstract-diagram | mode | l_semio_viz_math_mode_tl | 0 | semio-viz-scientific-mathematics.sty:1017 | semio-viz-scientific-mathematics.sty:1099 |
| sci-density | levels | l_semio_viz_math_levels_int | 0 | semio-viz-scientific-mathematics.sty:2067 | semio-viz-scientific-mathematics.sty:2097 |
| sci-skewt | mode | l_semio_viz_surf_mode_tl | 0 | semio-viz-scientific-surface.sty:42 | semio-viz-scientific-surface.sty:54 |
| namespace | data | l_semio_viz_scn_data_tl | 0 | semio-viz-showcase.sty:227 | semio-viz-showcase.sty:389 |

## Generated Variant and Literal Alias Expansion

Expanded 281 declared variants/aliases; excluded family-specific reset macros too. Re-examined 1677 direct controls; 16 remaining candidates. This supersedes the first-pass candidate count above.

| Family | Control | Carrier | Local RHS count | Key source | Owner source |
|---|---|---|---|---|---|
| arch-plan | id | l_semio_viz_dg_col_id_tl | 0 | semio-viz-diagram-architecture.sty:179 | semio-viz-diagram-architecture.sty:299 |
| pm-gantt | lane | l_semio_viz_dg_col_lane_tl | 0 | semio-viz-diagram-process.sty:162 | semio-viz-diagram-process.sty:327 |
| seating | label | l_semio_viz_dg_col_label_tl | 0 | semio-viz-diagram-process.sty:1021 | semio-viz-diagram-process.sty:1135 |
| spatial-layout | group | l_semio_viz_dom_group_tl | 0 | semio-viz-domain.sty:662 | semio-viz-domain.sty:1073 |
| schedule | time | l_semio_viz_dom_id_tl | 0 | semio-viz-domain.sty:1796 | semio-viz-domain.sty:1844 |
| schedule | labels | l_semio_viz_dom_labels_bool | 0 | semio-viz-domain.sty:1796 | semio-viz-domain.sty:1844 |
| version-control | time | l_semio_viz_dom_id_tl | 0 | semio-viz-domain.sty:1825 | semio-viz-domain.sty:1859 |
| version-control | labels | l_semio_viz_dom_labels_bool | 0 | semio-viz-domain.sty:1825 | semio-viz-domain.sty:1859 |
| infographic-list | width | l_semio_viz_dg_w_fp | 0 | semio-viz-infographic.sty:125 | semio-viz-infographic.sty:189 |
| sci-pathway | mode | l_semio_viz_bio_mode_tl | 0 | semio-viz-scientific-biology.sty:618 | semio-viz-scientific-biology.sty:628 |
| sci-clinical-timeline | laneHeight | l_semio_viz_bio_lane_fp | 0 | semio-viz-scientific-biology.sty:1640 | semio-viz-scientific-biology.sty:1648 |
| sci-circuit | mode | l_semio_viz_eng_mode_tl | 0 | semio-viz-scientific-engineering.sty:42 | semio-viz-scientific-engineering.sty:51 |
| sci-abstract-diagram | mode | l_semio_viz_math_mode_tl | 0 | semio-viz-scientific-mathematics.sty:1017 | semio-viz-scientific-mathematics.sty:1099 |
| sci-density | levels | l_semio_viz_math_levels_int | 0 | semio-viz-scientific-mathematics.sty:2067 | semio-viz-scientific-mathematics.sty:2097 |
| sci-skewt | mode | l_semio_viz_surf_mode_tl | 0 | semio-viz-scientific-surface.sty:42 | semio-viz-scientific-surface.sty:54 |
| namespace | data | l_semio_viz_scn_data_tl | 0 | semio-viz-showcase.sty:227 | semio-viz-showcase.sty:389 |

## Registry Accounting Correction and Direct Adjudications

The initial 240 literal body count included the public command signature `\NewDocumentCommand \SemioVizFamily {m +m}` as a false registration. Excluding that non-family leaves 239 literal SemioVizFamily registrations. semio-viz-mark.sty3010–3024 adds seven direct family_define registrations (mark-point/line/area/arc/connector/region/annotation) at begin document: exact total246. Those route through fam_run:nnn, shared family/mark keys (variant/data/x/y), and unknown keys forwarded to native mark keys. Dynamic mark-kind draw dispatch needs explicit owner adjudication, so the 1677-control direct named scan is not a universal all-public-controls proof.

Confirmed meaningful source gaps in remaining direct candidates:

* sci-clinical-timeline/laneHeight: biology1630 documents lane spacing7mm; setter binds bio_lane_fp but owner1648 normalizes -laneIndex through shared my, window bottom=-numberLanes-.5/top=-.2, and no reachable lane_fp read. Lane labels/events/intervals must all shift consistently with laneHeight. Shared range is also ignored because this owner constructs y window directly, while domain is read directly.
* sci-density/levels: mathematics2056 documents contour count5; setter2080 binds math_levels_int; owner2097 joint branch only density_raster + frame + points. Raster helper fills estimated cells, no contour extraction/draw; all reachable modes lack levels reads. Add actual level-dependent contour paths over the Gaussian density lattice with independent test oracle; changing raster colour alone does not satisfy documented contour count.
* schedule/time and version-control/time: domain1796/1825 bind dom_id_tl as the promised time column, but shared series extent/run/point pipeline1611–1767 uses row count and equidistant row index x=xa+(row-1)*step. Nonuniform authored time columns currently have no x effect.
* schedule/labels and version-control/labels: documented captions switch true but shared series pipeline emits no caption nodes and never reads labels_bool; dom_frame443 only computes xa/ya/xb/yb. Other domain owners read this variable, hiding these local omissions from global scan.
* infographic-list/width: infographic125 accepts frame width, but badge/list positions are literal x3/x7 and only height determines row spacing; no body width read. Existing width control needs actual horizontal geometry/layout effect, e.g. documented wrapping/frame contract, then neutral PDF glyph-bound proof.

Mode carriers pathway/circuit/abstract-diagram require explicit schema and stock-variant contract adjudication. Namespace/data is explicitly recorded-only in current design and is not a newly asserted defect. Column controls arch-plan/id, pm-gantt/lane, seating/label and spatial-layout/group still need source/descriptor semantics adjudication before defect claim. Generated aliases were expanded (281); third-party runtime tests have not been run by this lane.

Mode adjudication against current schema and native owner comments: sci-circuit mode explicitly promises selection of symbol vocabulary; schematic|logic|ladder|wiring|block|signal-flow is accepted but no local read, so same authored elements/options ignore that selection. Existing stock examples can differ because their elements differ; stock difference does not prove mode effect. sci-abstract-diagram native comment promises glyph vocabulary for commutative|string|tensor|hasse|cayley|dynkin|proof|rewrite; mode ignored while edge style/nodeShape drive all output. sci-pathway schema promises which member preset renders, but direct owner only edges/nodeShape/unit determine output. These require explicit same-input mode contracts rather than blanket identity-carrier exemption.

The schema currently observed in this pass already has axes=false/grid=false for these scientific owners; the historical91 default contradictions should not be asserted current after Root's repair. Exact final defaults need refreshed stage binding when production stops moving.

Further column contract adjudication:

* spatial-layout/group is a confirmed promised palette channel gap: domain649 says categorical channel driving palette slot, native sl_dot1050 paints palette(slot=rowIndex), never reads bound group column. Neutral two columns with distinct group assignments and identical positions/sizes must change repeated colours consistently; swapping row order must preserve category-to-colour mapping under the declared category-order rule.
* pm-gantt/lane native process137 documents optional swimlane column, acceptedsetter166; no local carrier read. Existing task rows are not grouped by authored lanes, so nondegenerate interleaved task fixture must test real lane grouping/separators and dependency links after placement.
* seating/label declares label column but current body neither reads nor prints it. A body legend/caption contract must be explicit; merely outer stock title does not consume this channel. This is accepted data-channel omission, with effect semantics less precise than categorical palette or swimlane.
* arch-plan/id is unused identity column. Source draws polygon rows directly with no link/identity-dependent rendering promise, unlike palette/swimlane. Do not call a visible geometry defect merely because this informational identity carrier is not needed to draw a polygon. Verify any identity/probe/admission promise separately; no blanket exemption for unrelated effects.

## Retained Source Bindings

Exact retained inputs in authored-inputs/family-local-reachability-before. Captured before new local-control repairs; source is moving, not final freeze.

| Input | SHA256 |
|---|---|
| semio-viz-scientific-biology.sty | DF3CB79F21EC2C097492DF3851DCC72384F8F33703B7880EBE75FE96E9F5916D |
| semio-viz-scientific-engineering.sty | 660A1321BEBA2AE1D379757389B1094C9E61D1225DD86CBC1187ABD96A567969 |
| semio-viz-scientific-mathematics.sty | 0D68F6393B8C80FA7DCE37C7846CBEB0C593394DD864C1B2124A10BF639EEB01 |
| semio-viz-scientific-surface.sty | C646E80C3FB83C8AE7BD578134F95B871F4353BE6F549D9E1002AB58A983852A |
| semio-viz-domain.sty | 50088141083B0A2EFA5B9E209BB9F88F566F8A37DD16C7A1D08DA13E76D33E09 |
| semio-viz-diagram-architecture.sty | 7D4F4191A32852BCEB324795206289E0932B3E345D5E25EB999A50D51DCB2BD9 |
| semio-viz-diagram-process.sty | B7EDC5544C7117C1C5933E8BD866CA3D412FF97F04EE465D1DAC259BE0B4E6A0 |
| semio-viz-infographic.sty | AE1C0D5B4C592BE5FB8627B2B5556406AF531E1BCE315A98F2755E8C4EDF83F8 |
| semio-viz-mark.sty | 90FF0671AE5C3CBC9581CCFD1E286FF39C5A923C74B48AAEE55BB23D125AC446 |
| semio-viz-catalog.sty | F3E1F8D02BD1ED5028DA49120FF74289959E61C904B40FDA7E3454880F4E3CB7 |

Current schema census: exact246 family descriptors contain4997 public option descriptors. Direct named native setters1677 are a subset; shared canvas keys, shared diagram kernels, forwarded mark keys, choice/code/meta setters and registry identity admission account for additional surfaces and require mapped source reads or explicit semantics. Do not interpret the direct-control count as all4997 effects audited. Counts are current source census, no runtime proof.
