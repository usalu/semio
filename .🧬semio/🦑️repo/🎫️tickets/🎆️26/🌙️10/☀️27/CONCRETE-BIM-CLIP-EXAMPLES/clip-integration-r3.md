# Clip Integration Audit R3

## Implemented

- Updated selected ground support from removed B3/C3 to authored C2 in objects, models, exchange and existing presentation fixtures.
- Roof consistency selects only supports present in roof tributary data, using actual upper support tags; removed old hardcoded reaction areas, force values and complete-grid counts from bilingual narration.
- Replaced exchange's rectangular equal-bay cell construction with the actual clipped nearest-support polygons.
- Corrected object type concrete totals to sum per-occurrence volumes across different ground/upper heights.
- Corrected northwest facade narration and model-derived compass direction in representations; removed y=0 support lookup after grid reorientation.
- Reworked room chapter for nineteen explicit spaces and derived areas; removed mandatory reference_area accesses.
- Preserved canonical pipe geometry. Models creates a copied hypothetical DP-1 routing scenario beside authored C2 for teaching coordination, with bilingual hypothetical label; independent tests separate clean source and scenario.
- Updated README, source narration and accessible bilingual transcripts.
- Portable fixture now records independent authored class counts, ground/upper support tag sets, reoriented grid and envelope, all wall/opening/window/room tags, and mesh counts derived independently from rectilinear source void intersections:96 slab nodes,70 shells,16 ground members/supports.
- Updated existing numerical tests for sparse supports, individual heights, door/window filling chains, four-flight connected ramps, polygon clipping, and source assumption provenance. Independent NumPy/SciPy/Shapely checks remain explicit.

## Validation and Remaining Work

Runtime validation remains blocked: repo --help fails because the concurrent native launcher reads runtime registry restart with an incompatible type. No non-repo shell binaries were used. The current changes need the full contract suite and six chapter presentation render probes after repo is restored.

Likely runtime-assisted follow-up: footer loss record/group counts and final source loss mapping; exact clipped roof-zone sampling boundaries; source shape assumptions in older geometry/STEP tests; hypothetical scenario upper/lower supports both sharing the same axis; clip native backend projections. Parent coordinator owns analysis and final validation.

Writes encountered transient Windows truncation/rename failures; existing-file handle writes were used. A few source files have harmless trailing spaces preserving the old file length when truncation was denied. No duplicate source files remain in the ticket.

## Changed Files

- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/📊️information/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧬️objects/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/👁️representations/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🔌️exchange/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🔗️consistency/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️cases/🔣️.json`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🎭️presentation/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🎭️presentation/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/README.md`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🗣️transcript.en.md`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🗣️transcript.de.md`

## Consumer Follow-Up R5

Corrected objects._tags to select only fill_class IfcWindow with fill_tag, excluding doors and unfilled openings. Portable fixtures explicitly separate35 standard walls,1 curved screen,19 doors and the OP-SC-FRAME unfilled landscape aperture. Main geometry tests now require nullable fill identity for that aperture and valid fill class/GlobalId for doors and windows. Quantity-pair checks follow actual filling-to-opening relationships rather than zipping unequal lists. Wall quantities accept zero glazing and compare layers only for the actual resolved material. Three added ground enclosure return walls bring the independently recorded IfcWall count to36. Runtime suite execution delegated to parent isolated Python runtime.

Independent Node assertion run caught concurrent source growth from36 to41 walls (five parent-suite boundary/return walls). Fixture refresh is waiting for the model source freeze signal. This is a detected fixture mismatch, not a passing numerical runtime claim.

## Runtime-Assisted Follow-Up

Final frozen source assertions pass independently in Node:164elements,41walls total (40standard+1screen),29fillings+one unfilled landscape aperture. Updated failing Python oracles from first isolated run: true clipped roof zone via Shapely, window-only glazing, separate entrance-glazing paths, actual two flights per ramp, selected C2 coordinates, ground-only vehicle checks and standalone arrival radius. Corrected sparse cell kinds to4interior/12edge and IDS window fire-rating expectations to4present/6missing. Information narration/transcripts now distinguish exterior forecourt arrival from source-informed glass geometry. Parent will rerun actual Python suite.

## Consumer Follow-Up R6

Information inset now draws all actual disconnected entrance glazing paths with a derived entrance-gap dimension, fitted to geometry bounds. Removed obsolete concentric car/glass equation and fabricated quarter-circle clipping. Main turning circle uses actual arrival curve_centre. Drive legend distinguishes source entrance geometry; pinch vignettes identify column clearance. Objects composition and recap now call shared villa_elevation; stage aliases preserve selected wall/opening/window identity while contextual geometry includes actual ground enclosure, both column storeys and roof landscape frame. Removed hardcoded composition callout coordinates and thumbnail cut heights. Added portable language-neutral AST contracts plus bilingual source/transcript consistency test in clip/🧪️tests/🧩️chapter-source. Native render remains outstanding; parent runs actual Python static contracts.

## Final Detail Wave Contracts

Reviewed explicit final counts:210elements,21spaces,33columns (29grid pilotis,2off-grid hall columns,2canopy posts),3slabs,46walls,33openings,21doors. Portable cases now distinguish all upper columns from13 primary roof supports and encode each authored type occurrence set. Two apertures have no filling: landscape frame and serving hatch. Independent rectilinear mesh-cell recomputation includes actual first-floor and primary-roof support coordinates plus all four first-floor voids:194slab nodes,150shells,18ground members/supports. Main contracts now validate off-grid source axes, actual type radii, each column’s own slab soffit, selected-type quantity/relation totals and separate canopy footprint. Objects occurrence animation excludes hall/canopy columns when demonstrating TY-PILOTI; narration and bilingual transcripts state the different types. Consistency resolves the primary roof slab through LK-ROOF identity. README distinguishes main roof/canopy and the three column types. Parent owns independent Shapely analysis support-selection tests and actual WASM suite execution.
