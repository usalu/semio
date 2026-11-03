# 📖️ README draft: "Views" (en)

Proposal only, from slot 20 (haiku-docs). The coordinator integrates it into `L/README.md` after the owners land their work. Items marked **(pending)** depend on unlanded work. Sources: work board D1, D5–D11, D17–D19, D21–D27; `🔨️modules/🎨️depiction/🔭️views/🟦️.ts` (the cameras); `🔗️interfaces.md` §2–§5 for the fidelity, codes and resolver; `📖️terminology.md` for the words (its row labels are T1–T26 and TS1–TS11). Decision D22 (symbol semantics) is used in the fidelity table.

---

## Views

Every asset is drawn in three orthographic viewpoints and seven representations. One evaluated state per instant feeds all of them, so the three views of a sheet always show the same figure at the same pose, time, scale and environment.

### Viewpoints and cameras

| Viewpoint | Name (en / de) | Seen from | Looking along | Paper x | Paper up | Nearer to the viewer |
|---|---|---|---|---|---|---|
| `elevation` | Side elevation / Seitenansicht | the asset's right (−z) | +z | x (ahead is right) | y | smaller z |
| `front` | Front elevation / Vorderansicht | ahead (+x) | −x | z (the asset's left is paper right) | y | larger x |
| `plan` | Plan (top view) / Draufsicht | above (+y) | −y | x | z (the asset's left is paper up) | larger y |

- The world frame is metres, x ahead, y up, z to the left of the asset (+z), ground at y = 0. With these axes the frame is **left-handed**. Decided (D1, D6).
- A viewpoint changes only the camera. Pose, time, root motion, scale, environment and attachments are the same in all three. Decided (interfaces §2).
- Three views (Drei Ansichten) form one sheet (D7):
  - the plan sits above the side elevation and shares its x axis;
  - the front elevation sits beside the side elevation and shares its y axis;
  - the front's paper x runs in the same direction as the plan's paper up (+z).
  Decided (D7).
- Every cell of the sheet is cut from the same millimetres-per-metre drawing, so one metre measures the same in all three. **(pending: viewer sheet, sonnet-viewer.)**

### Representations

The seven representations are dark line, dark solid, light line, light solid, thermal, forces and moments (Darstellungen). The word "view" is not used for them. Decided (D6, terminology).

### Time

- `timeS` is the absolute time. `clipTimeS` is the wrapped clip phase. Thermal history, environment timelines and transport use `timeS`. Decided (D11).
- A sheet is produced by one `resolveViews` call: it evaluates once per instant, projects the three viewpoints from that single state and solves the analysis once. Consumers must not call `evaluate` per viewpoint. Decided (interfaces §5). Exports may call `resolveDrawing` per viewpoint, as interfaces §5 allows.

### Scale and dimensions

- Scale maps model metres to paper millimetres only: `mm = m · 1000 / denominator`. It never changes dimensions, masses, temperatures or loads.
- `dimensions` is the physical envelope of everything that belongs to the asset: held items, attachments, plates and lids. It excludes stroke width, plumes and annotations. Decided (D18).
- The extent of each viewpoint is measured at the default pose, excluding the ground line. It must agree with the dimension within max(2 %, 15 mm). Decided (D18, interfaces §4). Validator codes `extent-front-*`, `extent-plan-*` and `symbol-extent-*`: landed (per contracts report).

### Unavailable scales and symbols

- A scale that cannot be drawn is an explicit `unavailable` decision with a required `cause`: `below-printable-feature` (a real feature is smaller than the profile can print) or `no-physical-depiction` (no meaningful physically scaled drawing exists). Decided (D8, interfaces §3). Landed (per worker report).
- An unavailable scale is unavailable in all three viewpoints. A half-available scale does not exist. Decided (interfaces §3).
- An asset of at least 3 mm overall on paper must be drawable at that scale. Below 3 mm it may be unavailable with a truthful cause. Sixteen bands must be authored (`📝️haiku-inventory-unavailable.md`). Decided (D17).
- A true-extent symbol with all three views that passes the print minimum and the extent check satisfies D17. This covers city-bicycle and scissor-lift at 1:500. Decided (D27).
- Each band declares a support range `validRange`. Coverage includes both ends of the range, which reach 1:1000. Decided (D10).
- An alternative is one of three kinds: a `scale`, a `symbol`, or an `asset`.
  - An `asset` alternative is valid only if its target is drawable at that scale and is the same kind of thing. Chains of alternatives are forbidden. Decided (D9).
  - Every alternative carries its fidelity (below). Decided (interfaces §3).
- Fidelity is returned by the resolver and never stored. Symbol `semantics` keeps its three values; consumers show the semantics to the reader. Calling a true-extent symbol a "conventional sign" is not a contradiction (D22).

| Source | Semantics | Fidelity | Units | Meaning |
|---|---|---|---|---|
| authored scale | (none) | `physical` | model metres | real size |
| symbol | `true-extent` | `physical` | model metres | real size |
| symbol | `true-position` | `conventional-symbol` | paper millimetres | its size carries no dimension; its `dimensionalNote` says so |
| symbol | `nominal` | `conventional-symbol` | paper millimetres | as above |

- There is no silent fallback: no other scale, no other viewpoint and no blank drawing. A missing combination returns a named failure: `scale-unavailable`, `scale-unsuitable`, `view-missing`, `symbol-unknown` or `symbol-scale`. Decided (interfaces §5). Landed (per contracts report) in `🔨️modules/🔭️multiview`.
- A symbol must have elevation, front and plan geometry. Decided (interfaces §3). Landed (per worker report).

### Shapes and solids

- A solid (`solid`, the revolved or extruded placement of a shape) has its `axisBone` equal to the shape's own bone or one of its ancestors. Decided (D21).
- The revolve axis is local to its bone: `axisX` and `axisZ` are offsets from the rest origin of the axis bone. Decided (D24). This supersedes the deferral in D16. **(pending: rigs fixes `solid3` and migrates every revolved solid in one coordinated pass.)**
- Solids are allowed in every view, elevation included. This withdraws the earlier rule that rejected solids on the elevation. Decided (D26).
- There are no elevation twins. The validator code `view-twin-missing` is removed, because front and plan shapes take their depth from their own 3D carry. Decided (D25).
- Shapes on spread or twisted bones must be solids. The validator code `lateral-ambiguous` enforces this. Decided (D25). **(pending: validator rule, sonnet-contracts.)**

### Course casts

- Every cast row requires `viewpoint`: `elevation`, `front` or `plan`. There is no default. Decided (D5). Landed per exports report (schema, exporter and README; cast-export suite 66 pass, 1 skip). The course Python re-run is **(pending)**.
- `mirror` is allowed only with `elevation`. Decided (D5).
- A mirror of a laterally asymmetric asset needs a reason in the guard's allowlist, and every mirror must be visible at the call site. Decided (D19). **(pending: the guard allowlist update, course).**
- Casts are sized from their real dimensions, with no magic constants and no non-uniform stretch. Decided (D19). **(pending: course code audit.)**
- A plan rotation is allowed only as a documented yaw. Decided (D19).

### Moment and force signs

Decided (D23, legend rule). The legend and the docs state, in English and German:

- The components are those of M = r × F in the (x, y, z) frame.
- The frame is left-handed, so the arrow's sense follows the left-hand rule.
- The curl sense in each view: positive mz is counter-clockwise in the side elevation (seen from −z); positive mx in the front elevation and positive my in the plan are clockwise.
- A moment vector is drawn as an arrow with two heads pointing the same way.
- Axial force N > 0 is tension; bending moment M > 0 is sagging (existing README convention). Compression is warm, tension cool, with hatching as a colour-independent reading (existing README convention).

**(pending)**: the legend text in the structural view and the doc in `🏗️structure`, owned by sonnet-structural.

**Reconcile**: the existing README sentence "joint moments are CCW positive" applies to mz in the side elevation, as D23 states. It must not be read as a rule for mx or my, which are clockwise in the front and plan.

### Terminology

| Concept | en | de |
|---|---|---|
| side viewpoint | Side elevation | Seitenansicht |
| front viewpoint | Front elevation | Vorderansicht |
| plan viewpoint | Plan (top view) | Draufsicht |
| the three viewpoints | Three views | Drei Ansichten |
| render mode | representation | Darstellung |
| front cut | Section A-A | Schnitt A-A |
| plan cut | Plan section B-B | Grundriss-Schnitt B-B |

Grundriss is used only for a cut plan section. Decided (D6, approved terminology).

**(pending)**: the i18n keys, captions, validator messages and tests still show the old wording; the owners apply the terminology proposal first.

---

### Notes for the integration

- The existing "Front and plan views" section stays; this section is the overview, and that section keeps the shape rules (`solid`, `followTip`, spread and twist).
- The existing README "Conventions" line on the frame and "World x right" is rewritten by sonnet-contracts per D6. This draft does not touch it.
- The README "Course casts" table row `viewpoint` should be replaced by the cast rule above when the README is integrated.
