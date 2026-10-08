# r7 design: one model graph (label `z-graph`)

Ticket `26/10/08/BIM-PLUGIN`. Inputs: `r2-design.md` section 0 and 5, `r5-audit-inferences.md` (F01-F07, F11-F13, F17-F19, F21, F22, F25, F14), `r1-explore-inferences.md`. API for consumers: `r7-api-model-session.md`. Result report: `r7-exec-z-graph.md`.
`S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `I` = `S/🧬️schema/💡️inferences`, `F` = `🧰️framework/🛍️products/💻️os/🔨️modules`.

## 1. Decision

**One `InferredField` of the artifact (`s.bim.model.inference.model-graph`), keyed by a typed node enum, valued by a typed value enum, with the real cross-entity edges as engine parents.** The framework engine is *not* given cross-field parent values: the engine already passes the values of *all* parents of a step (`compute(snapshot, key, parents: &[Value])`), the limit found in r1 was that every BIM field was a separate `InferredField` whose parents had to be the same field. A single field whose `Value` is an enum removes the limit without touching the engine's data model. One additive engine change is made for F12 (see 7).

Why not extend the engine with cross-field parents: it would add a second value-routing mechanism (typed heterogeneous parents) to a generic driver that has one planner, one hash chain and one cache; a node enum keeps all of that and costs nothing. The node enum is also the unit the cache, the cursor (progress, cancellation) and the diff gate already understand.

## 2. Nodes

`ModelNode` (key, `Ord + Hash`, `ToValue/FromValue`) and `ModelValue { key, data }`. Every value carries the key it was computed for, so `compute` finds its parents by key, not by position, and a cached value can never be handed to a different node (the key is also part of every dependency input).

| Node | Parents (real edges) | Own dependency (`dep_input`) | Value |
|---|---|---|---|
| `Storey(id)` | storey below in the stacking | `storey_levels::dependency` | `StoreyLevel` |
| `Band(wall)` | - | axis, location line, type layers | `Option<Band>` (axis, left, right; `None` for a zero-length axis) |
| `WallLayout(wall)` | own `Storey`, target `Storey` of a `Storey` top, own `Band`, the `Band` of **every wall within two touches** | wall record, type layers | `WallLayout` |
| `CurtainLayout(curtain)` | own and target `Storey` | axis, base offset, top, spacings | `CurtainLayout` |
| `Cut(opening)` | - | opening record, window/door type record | resolved size + cut rectangle |
| `Host(id)` (a wall/curtain wall that hosts openings) | own `Storey`, its `WallLayout` / `CurtainLayout` | axis, building origin and rotation | `HostExtent` (axis, base, height, thickness, length, faces, datum, placement, **trimmed extent**) |
| `OpeningFrame(opening)` | its `Host`, its own `Cut`, the `Cut` of every sibling on the host | opening record, window/door type | `OpeningFrame` |
| `StairRun(stair)` | own and target `Storey` | stair record | `StairRun` |
| `Solid(family, id)` | walls: own `Storey`, `WallLayout`, the `OpeningFrame` of every hosted opening; curtain walls: `Storey`, `CurtainLayout`; fillers: `Storey`, `OpeningFrame`; stairs: `Storey`, `StairRun`; columns: own/target `Storey`; beams, slabs, roofs, railings: own `Storey` | element record, type record, building placement | `ElementSolid` + roof fallback note |
| `Room(storey)` (storeys with spaces) | own `Storey`, the `WallLayout` of every wall of the storey | spaces, curtain walls, columns (+types), slabs above (+types) | `StoreyRooms` |
| `Plan(storey)` | own/target `Storey`, `WallLayout`s, `CurtainLayout`s, hosted `OpeningFrame`s, `StairRun`s, `Room(storey)` | beams, columns, slabs, roofs, railings, spaces, grids, storey record (+ the types they read) | `PlanLinework` |
| `Quantity(element)` | wall: `WallLayout`, hosted `OpeningFrame`s; curtain wall: `CurtainLayout`, hosted frames, `Solid`; column: `Storey`s; opening: `OpeningFrame`, `Solid`; stair: `StairRun`, `Solid`; roof, railing: `Solid`; space: `Room` | element/type record, densities of the materials it uses | `ElementQuantity` |
| `Totals(scope)` (`Storey`, `Building`, `Project`) | the `Quantity` of the elements of the scope | - | `QuantityTotals` |
| `Diagnostics(scope)` (`Storey`, `Building`, `Model`) | storey: level `Storey`s, `WallLayout`s, hosted `OpeningFrame`s, `StairRun`s, `Room`, roof `Solid`s; building: all its `Storey`s, `WallLayout`s, `StairRun`s; model: - | the elements and referenced types the checks read; model: the reference digest (ids and reference fields only) | `Vec<Diagnostic>` |

Order of `plan()`: storeys (stacking), bands, cuts, curtain layouts, wall layouts, hosts, frames, runs, solids, rooms, plans, quantities, totals, diagnostics. Each parent precedes its children (the engine rejects any violation loudly).

### Joins without cycles and without O(W^2) (F01, F02)

Joins are symmetric, so neighbours cannot be parents of each other. The authored data of the neighbours enters the layout through the **`Band` roots**: a `Band` has no parents, a `WallLayout` has the bands of its neighbourhood as parents. The neighbourhood is computed once per run in `plan()`: per storey one sweep over the inflated axis bounding boxes (`joins::touching`), an exact test per candidate pair (tip on axis within `JOIN_TOLERANCE`, axes crossing in both interiors), and `T2(w)` = the walls touching `w` plus the walls touching those. Two touches are needed and enough: the end of a neighbour butts against the lowest-id wall it touches, which is decided by the walls touching *that neighbour*, so the join of `w` with `n` depends on `n`'s neighbours. `joins::join` over the bands of `{w} + T2(w)` equals `joins::join` over the whole storey (tested bit for bit against the full-storey join on the house, the 30-wall join fixture and a 500-wall grid). Moving one wall changes its `Band`, hence the layouts of the walls within two touches, and nothing else; moving a wall into or out of contact changes a parent list, hence a dependency hash.

### One resolver per concept (F02, F03, F04, F06, F25)

* top constraint: `storey_levels::top_of` / `vertical_of`, used by layouts, columns, runs, bodies, IFC; the copies in `wall_layout`, `element_solids`, `bodies`, `stair_runs` are deleted.
* host: `opening_frames::HostExtent::of_wall` / `of_curtain` computed once in the `Host` node from the layout; solids, fillers, plan, diagnostics read frames/hosts from parents. One `profile_extents` for the mullion depth (frames, solids, spaces, plan).
* validity (F06): `opening_frames::issues_of(cut, host)` is the only rule. With `lo..hi` the full-thickness extent of the join-trimmed host (`HostExtent::trim`) and one tolerance `LENGTH_EPS = 1e-9`: `OutsideHostExtent` (beyond the axis), `BelowHostBase`, `AboveHostTop` (above the top), and the new `OutsideTrimmedExtent` (not beyond, but not strictly inside `lo..hi` and strictly below the top: the hole would touch the border of the face). A frame is `valid` iff it has no issue. The wall solid cuts **exactly the valid frames** of its host (the old private predicate `usable` is gone), the filler is built exactly for valid frames, quantities subtract exactly the valid cuts (valid cuts never overlap: two overlapping openings are both invalid), diagnostics report each issue.
* `SolidFamily` is the only family enum; the key of a filler is `(Window|Door, opening)`; voids have no solid node.

### Incremental behaviour

`dep_input` of a node covers what `compute` reads that is not a parent; a parent contributes through its dependency hash. Consequences (tested): changing a storey height recomputes the storey, the layouts/runs/solids/rooms/plans/quantities that resolve against it; moving one wall recomputes its `Band`, the layouts of the walls within two touches, the hosts and frames of their openings, the wall and filler solids, the plan and room of the storey, the quantities of those elements and the totals of the storey, building and project; a material colour edit recomputes nothing but the density-reading quantities.

## 3. Values and cost

`ModelValue.data` holds `Arc` payloads for everything bigger than a `Copy` struct, so the engine's per-parent clone and the cache hit are pointer copies and a projection into `ModelInference` is a `try_unwrap` or one clone of the *changed* nodes. `value_bytes` accounts mesh and loop payloads for the cache budget.

## 4. Projections

`ModelInference.<field>` = the nodes of that kind, keyed by element id (`Storey(id)` -> `storey_levels`, `WallLayout` -> `wall_layout`, `Solid` (non-empty) -> `element_solids`, `Room(storey)` flattened -> `spaces`, `Quantity`/`Totals` -> `quantities`, all `Diagnostics` ordered -> `diagnostics`, ...). `ModelInference::infer` = run the graph without a cache and project. `compute_*` helpers run the same graph restricted to the kinds they return plus their ancestors (kind-level closure, `ModelGraph<WANT>` with a const mask) and project one field: they are thin and share the field id, so a warm cache serves them too.

## 5. Session

`ModelInferenceSession { cache (enabled, byte budget), engine session, last values, ModelInference, report }`. `update` -> `protocol::infer_field_after_diff` (tier-1 gate on `reads`, then the dependency-hashed walk); `refresh` the same with the whole model as touched. After the run the new values are compared with the previous ones (`Arc::ptr_eq` / `==` for `Copy` payloads): only changed entries are copied into the held `ModelInference`, entries of vanished nodes are removed. `UpdateReport.computed` is the engine call counter (a thread-local tally incremented in `compute`, per kind).

## 6. Fixes by finding

| Finding | Where |
|---|---|
| F01 | `joins::touching` sweep + `Band` roots + `WallLayout` parents; no scan of a storey per wall anywhere (`storey_bands`, `layout_of` over the storey, the neighbour scan in `dep_input` are gone). |
| F02, F03, F04 | section 2 "one resolver". Solids/plan/bodies/diagnostics/spaces/quantities read `WallLayout`, `Host`, `OpeningFrame`, `StairRun` from parents. |
| F05 | `plan_kit` is the one place for plan conversions (`point`, `mark`, `seg`, `rectangle`, `bulged`, `placed`, `profile_polygon`, `profile_extents`, flight outlines); the copies in `bodies`, `plan-linework`, `columns`, `wall-layout`, `opening-frames` are deleted. |
| F06, F07 | section 2 "validity"; `OutsideTrimmedExtent`; `quantities` uses valid cuts only (sum of disjoint rectangles; a test with two overlapping windows and a window at a wall end). |
| F11 | `ModelInferenceSession`; `infer_field_after_diff` + enabled cache are in production; node-count test. |
| F12 | engine: `dep_input` and hashing only when a cache is enabled (or the diff-gated driver, whose session root folds the hashes). `Diagnostics(Model)` depends on `references::ReferenceView` (ids and reference fields, not geometry); type maps are scoped to the types the node's elements reference (`scope` helpers), so a type edit invalidates only the nodes that use the type. |
| F13 | the whole table of section 2. |
| F14 | `Solid(Roof)` carries the `RoofFallback`; `Diagnostics(Storey)` reports a `Warning` per fallback (`RoofFallback*` codes, en + de). |
| F17 | `Plan(storey)` has `Room(storey)` as parent and draws the rooms from it; the second boolean arrangement is gone. |
| F18 | `plan()` builds `host -> openings` once; frames take sibling `Cut` parents, wall solids take frame parents; no scan per wall or per opening. |
| F19 | obstacles are the `WallLayout` footprints (parents); bounding boxes are computed once per run of `rooms_from` and rooms test boxes before any boolean. |
| F21 | graph laws and one cache-transparency test per projection, gating tests for levels and quantities, determinism/default for runs and rooms. |
| F22 | `OpeningIssue::HostDegenerate` (zero-length host axis) replaces the silent `(1,0)` tangent; `frame_of` returns the frame with the issue and no placement. |
| F25 | section 2. |
| F16 | done by `z-mutations` (`placement::storey_elevation`). |

## 7. Engine change (additive, domain neutral)

`F/💡️inference/🦀️.rs`: `infer_field_step`/`try_infer_field` evaluate `dep_input` and hash only if the cache is enabled; `infer_field_after_diff` always hashes (its session root is a fold of the hashes). No signature changes, no behaviour change for any output; `InferenceCursor::hash` answers only for hashed runs. New engine test `dependencies_are_hashed_only_when_a_cache_can_use_them`.

## 8. Not done / limits

* Clashes (`Diagnostics(Building)`) recompute for the whole building when any of its bodies changes (the sweep is `O(n log n + k)`, a per-pair node would add a node per overlapping pair).
* The plan keeps drawing every opening (also invalid ones, as before): validity is a solid/quantity/diagnostic rule.
* `T2` is a superset-safe neighbourhood: a move of a second-order neighbour recomputes the layout (its value is unchanged) because the engine hashes dependency chains, not values.

## 9. Progress and cancellation

`ModelGraph<WANT>` is an ordinary `InferredField`; the stepped driver (`InferenceCursor`, `infer_field_step`) gives progress (`cursor.fraction()`) and cancellation (`cursor.cancel()`) for free and finished nodes stay valid. The session uses the unbounded driver (interactive size); a stepped session API is the follow-up if a model needs it.
