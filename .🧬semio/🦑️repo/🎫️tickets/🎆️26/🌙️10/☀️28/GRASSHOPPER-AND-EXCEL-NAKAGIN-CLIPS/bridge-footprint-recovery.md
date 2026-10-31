# Bridge Footprint and Native Return Recovery

## Evidence

See source-reference-recovery.md and the retained generated/gluon-bridge-top-planes.png scan projections. The independent calibrated scan places the central deck width near2.21m and centreY near−1.44m, with a southward L-return positively visible at the A end on all three levels. Native CAD2.30m width and1.25×2.50m return are compatible with the scan’s approximate1.4×2.8m return. The model now uses2.30m and centre−1.45m with explicit source provenance; native flat deck/V-frame geometry is retained. Dynamic source-core endpoints remain authoritative.

The main south A guard begins beyond the1.25m return junction; it does not rail across the walk connection. A skew overhead crossbar joins the shifted south mast to the existing north mast above the2.1m body prism. The return has its actual side/end guards. Its declared walking route terminates0.39m before the closed end guard:0.30m body half-width+0.05m guard inset+0.04m rail radius. Strict0.60m passage/2.10m headroom checks remain unchanged. This uses the existing explicit walking_route API rather than ignoring the guard.

Core bridge portals/approaches read the same lateral offset and width as the main deck. bridge_return is now a required independently ground-reachable support terminal and receives actual-mesh flat-route headroom checks. Membership remains unchanged; corrected L placements on A4/5/13 are distinct from the deliberately omitted south-facing bridge-level slots.

The strict mesh gate identified six real chamfer-pier conflicts in the initially straight centreline approaches. Each actual support now declares a full0.6m route through the portal atY−1.00, then bends to deckY−1.45 only outside the core face. The0.08m geometric pier clearance and0.33m exterior transition are explicitly assumed parameters. No pier, capsule or tolerance is removed. Portable cases pin the portal path and an independent actual-mesh gate checks the entire body strip.

## Validation

The widened/returned bridge passes7 independent NumPy/rhino3dm geometry tests (2.838s) and AJV rejection of12 impossible bridge dimension variants. The expanded whole-building circulation gate passes18 tests (24.588s), including all office portal strips, all three native return terminals, full0.6m/2.1m pier-clear approach routes, every stair centreline, exact support-graph oracle agreement and actual all-part mesh barriers. No capsule membership or clearance threshold was changed to obtain this result. The restored L placements separately pass the actual-pose/unique-footprint regression and zero capsule-overlap regression (2.383s).
