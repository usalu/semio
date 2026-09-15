# A Boolean Owns What It Answers With — lane `brep-boolean-input-lifetime` (2026-09-15)

Takes `📓️slider-reevaluation-correctness-2026-09-15.md` §7 item 1, the one defect that lane
witnessed and handed on: after one `brep.bool.fuse` / `brep.bool.cut`, the sibling INPUT solid's
handle resolves to a freed arena slot, so the next evaluation of the same graph answers
`missing handle: b29a89a4…` for a node whose own parameters never moved.

- Kernel law: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/⚙️engine/🧪️tests/🔬️unit/🦀️.rs`
- Delivery law: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🎚️slider-values/🦀️.rs`
  over `🧫️fixtures/🎚️slider-values.json`
- Runtime output: `🗑️generated/bool-lifetime/`

---

## 1. TL;DR

**The boolean was not leaking a handle. It was eating its operands.** The exact
imprint→classify→select→stitch engine splits the operands' OWN faces in place, keeps the pieces the
selection chose, and then `remove_solid_and_orphans` both operands — so every arena entity either
migrated into the result or was freed. Nothing above the kernel could have survived that: the
evaluator serves an unchanged input node from cache and re-presents the SAME `GeometryHandle` on the
next evaluation, and that handle now named a dead slot.

Fixed on the owning layer, as one law: **a boolean owns everything it answers with and frees nothing
it did not create.**

1. The general engine plans on **deep copies** of both operands (`BooleanJob::new` →
   `copy_solid`); the imprints, the classification and the stitch all run on the copies, and the
   stitch's `remove_solid_and_orphans` therefore destroys only the copies.
2. Every **fast-path** branch that "returns an operand" now returns a `copy_solid` of it. The old
   `clone_solid_shells` minted a second solid/shell wrapper over the operand's OWN faces — one face
   in two shells at once, and the operand's lifetime welded to the result's.
3. The post-stitch **orphan sweep is scoped** to the entities the job itself minted. It used to free
   every edge with no coedge and every vertex with no edge across the WHOLE arena — and a bare
   `Wire` handle's member edges carry no coedge at all, so any boolean anywhere silently ate every
   live wire's geometry as collateral.
4. `compound_cut` drops each intermediate it mints as soon as the next round supersedes it, so the
   fold does not trade the operand leak for an intermediate leak.

---

## 2. Root cause, file:line

`✏️s/…/🧊️brep/🧬️schema/🔺️diff/🔀️boolean/🦀️.rs`

| # | site | what was wrong |
|---|------|----------------|
| 1 | `BooleanJob::step`, `BooleanPhase::Stitch` | `remove_solid_and_orphans(body, self.a, …)` and `…self.b…` — the operands themselves, since `self.a`/`self.b` WERE the caller's solids. Every face the selection did not keep was deleted outright; every face it kept was moved into the result's shell. Either way the operand's `SolidId`, its shells and its faces stopped resolving. |
| 2 | `trivial_topology_fast_path` via `clone_solid_shells` | the disjoint/contained shortcuts "returned an operand" by minting a fresh solid + shell over the operand's OWN `FaceId`s. The operand stayed live, but its faces now belonged to two shells, so freeing either one took the other's geometry with it. |
| 3 | `gc_orphan_edges_and_vertices` | an unscoped whole-arena sweep: every edge no coedge references and every vertex no edge references, freed. A `Wire` registered through `Brep::register_wire` holds edges that no coedge references by construction — `Entity::Wire` carries its own label, so `label_of_entity` keeps calling it live while its edges are gone. |

### The evidence, in the order it came in

1. The new kernel law, on the unfixed tree:
   `Unite freed its first input: operation failed: missing entity: shell has no faces` — the sphere's
   shell survived as an empty husk because its faces had migrated into the fused result.
2. Reading the Stitch phase: `remove_solid_and_orphans(body, self.a, &selected_set, rec)` — the
   operand, by id, unconditionally, on the success path.
3. Reading `clone_solid_shells`: `outer_faces(body, solid)` hands back the operand's own face ids and
   `solid_from_outer_faces` wraps them in a second shell. That is aliasing, not cloning.
4. Reading `gc_orphan_edges_and_vertices` against `Brep::register_wire`: a wire's edges have no
   coedges, so the sweep's own definition of "orphan" includes every live wire in the process.

---

## 3. The fix

### 3.1 The general engine plans on copies

`BooleanJob::new` became `(&mut Body, …, &mut OpRecorder) -> Result<Self, KernelError>` and opens with

```rust
let protected_edges: HashSet<EdgeId> = body.edges.iter().map(|(id, _)| id).collect();
let protected_vertices: HashSet<VertexId> = body.vertices.iter().map(|(id, _)| id).collect();
let a = copy_solid(body, a, rec)?;
let b = copy_solid(body, b, rec)?;
```

`copy_solid` is `transform_solid` under `Affine3::IDENTITY` — the kernel's existing deep copy, fresh
`PersistentLabel`s for every vertex/edge/coedge/p-curve/loop/face/shell, recorded generated in `rec`.
From there the walk is unchanged: the plan, the weld table, the imprints, the classification and the
stitch all address the copies, and the stitch's two `remove_solid_and_orphans` calls destroy the
copies. The caller's operands are read exactly twice — once by the fast paths' probes, once by
`copy_solid` — and never written.

This also upgrades the cancellation contract. The struct's docstring used to admit that a cancelled
or failed job leaves its partial imprints on the operands; now those imprints live on the job's own
copies, so a cancelled boolean leaves its inputs byte-identical and leaves behind only unreachable
garbage the next compaction reclaims.

### 3.2 The fast paths return copies

`clone_solid_shells` is deleted. `BooleanOp::Cut` on disjoint operands, and all four
"one operand contains the other" branches, return `copy_solid(body, operand, rec)?`. The two branches
that assemble a shell out of loose faces — the disjoint union, and the cut whose tool is wholly
inside the target — go through the new `detached_copy_of_outer_faces`, which deep-copies the operand,
takes the copy's outer faces and then drops the copy's own solid/shell wrapper, leaving faces that
belong to nobody and are free to be re-shelled.

### 3.3 The orphan sweep frees only what the job minted

`gc_orphan_edges_and_vertices` takes `(&HashSet<EdgeId>, &HashSet<VertexId>)` — every edge and vertex
that already existed when the job was planned — and skips them. Everything the job created (the two
deep copies, every speculative imprint edge, every welded vertex) is still swept exactly as before.

### 3.4 `compound_cut` retires its own intermediates

Each fold round frees the previous intermediate unless it is the caller's `target`. Before the fix
that happened by accident, because the next round consumed it.

---

## 4. Laws, with output

### 4.1 The kernel law (new)

`cd /Users/ueli/Documents/semio && RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-stdio-semio --lib -- a_boolean_leaves_both --test-threads=1 --nocapture`

**Before** (same law, unfixed tree):

```
thread '…a_boolean_leaves_both_of_its_input_solids_alive_for_the_next_evaluation' panicked at …🔬️unit/🦀️.rs:288:79:
Unite freed its first input: operation failed: missing entity: shell has no faces
test result: FAILED. 0 passed; 1 failed
```

**After:**

```
[DEBUG] boolean input lifetime Unite: inputs 7.238229/3.375000 survive, result 9.708451 reproduces as 9.708451
[DEBUG] boolean input lifetime Cut: inputs 7.238229/3.375000 survive, result 6.333451 reproduces as 6.333451
[DEBUG] boolean input lifetime Intersect: inputs 7.238229/3.375000 survive, result 0.905379 reproduces as 0.905379
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2523 filtered out; finished in 0.25s
```

The law walks all three `BooleanOp`s over `sphere_prim_sync(1.2)` and `box_prim_sync(1.5, 1.5, 1.5)`
— the `sphere-box-fuse` example's own operands, whose committed analytic volume is 9.70845078963702 —
and for each one asserts, in this order: the boolean answers; BOTH inputs still resolve; both inputs'
volumes are unchanged to 1e-9; a `retain` naming exactly the three live handles keeps all three
registry entries; all three still tessellate AFTER that compaction; and the same graph evaluated a
SECOND time from the same cached input handles answers the same solid. That last step is the
defect's own shape: not a leak, a re-evaluation.

### 4.2 The delivery law

`cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- every_moved_slider_converges --test-threads=1`

```
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 479 filtered out; finished in 49.62s
```

The fixture's `handedOn` map is now `{}` and the two `sphere-box-fuse` rows that were required to
FAIL (`max-end · radius = 3`, `min-end · radius = 0.2`) are required to pass. They do: both publish
`["eval-fuse@solid#0"]` against an oracle of `["eval-fuse@solid#0"]`, phase `idle`, no errors.

Two things had to move together for that. Flipping the tags alone made the law say

```
sphere-box-fuse · max-end · radius = 3: converges now, but the fixture still hands it on to brep-input-handle-lifetime — delete the tag and the reason: …
sphere-box-fuse · min-end · radius = 0.2: converges now, but the fixture still hands it on to brep-input-handle-lifetime — …
```

which is the hand-on doing its job. But the same run also turned three `sphere-cut-with-torus` rows
red with

```
sphere-cut-with-torus · inside · slider_2 = 3.5: the kernel cannot build this value ({…"[void-shell-not-inverted] solid-13-void-shell-14…"}) and the surface names NOTHING — it publishes 0 meshes under phase idle with errors {"$kernel": ""}
```

Those rows used to pass for the WRONG reason. `press_faults` accepts a value the kernel cannot build
as long as the surface NAMES a fault, and before this fix the fault they named was this lane's own
`missing handle` on a consumed input. With the input lifetime fixed, they reach the kernel's genuine
refusal instead (§7 item 2 of the previous lane, `void-shell-not-inverted`) — and the law's reader
could not see it, because the preview status publishes
`diagnostics: [{handle, issues: [{entity, code, message}]}]` and `delivered_preview` was reading
`entity`/`message` off the OUTER entry, one nesting too shallow, recording every validate-gate
refusal as the contentless `$kernel: ""`. The reader now walks `issues` and records
`entity → "[code] message"`. That is a law-reader fix, not a product change: the surface was naming
the fault all along.

### 4.3 Regression suites

| suite | result | note |
|---|---|---|
| `semio-s-artifact-procedural-generation3d --test example-geometry` | **18 passed, 0 failed** | the committed per-example geometry + budget oracle, including both boolean examples |
| `semio-s-artifact-stdio-semio --lib -- …brep::schema::diff` | **134 passed, 0 failed** | every boolean/transform/euler/sweep/blend/offset/intersect law |
| `semio-s-artifact-stdio-semio --lib -- …brep::schema::engine` | **29 passed, 0 failed** | includes the previous lane's scoped-gate law and `retain_compacts_everything_not_kept` |
| `--test brep_procedural_example_booleans` | 2 passed, 0 failed | |
| `--test brep_extrude_orientation` | 12 passed, 0 failed | |
| `--test brep_tessellation_jobs` | 10 passed, 0 failed | |
| `--test brep_analytic_blend` | 7 passed, 0 failed | |
| `--test brep_shell_orientation` | 2 passed, 0 failed | |
| `semio-s-artifact-stdio-semio --lib -- …subsets::brep` | 550 passed, **32 failed** | all 32 are `committed_json_is_canonical` / `produces_committed_diff` / `*_round_trips_through_snapshot` under the peer schema sweep `📓️slider-reevaluation-correctness-2026-09-15.md` §4.4 documents. None reaches `diff::boolean` or `schema::engine`, both of which are green above. |
| `semio-s-artifact-procedural-generation3d --lib` | 471 passed, **9 failed** | byte-identical to the set the previous lane documented: 4 pre-existing + 5 on a peer's half-landed `flow.fixture → flow.host_snapshot` rename, in files this lane never opened |

---

## 5. Runtime, in the browser

*(filled in below)*

---

## 6. Files

Updated:
- `✏️s/…/🧊️brep/🧬️schema/🔺️diff/🔀️boolean/🦀️.rs` — `BooleanJob::new` deep-copies both operands and
  snapshots the protected edge/vertex sets; the fast paths return `copy_solid`s;
  `clone_solid_shells` deleted and `detached_copy_of_outer_faces` added;
  `gc_orphan_edges_and_vertices` scoped; `compound_cut` retires its intermediates; module header and
  the `BooleanJob` cancellation contract restated
- `✏️s/…/🧊️brep/🧬️schema/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` — the kernel law
- `✏️s/…/🧊️generation3d/…/✳️any/🧫️fixtures/🎚️slider-values.json` — the two `sphere-box-fuse` rows
  flipped from required-failing to required-passing, `handedOn` emptied
- `✏️s/…/✳️any/✏️editor/🧪️tests/🎚️slider-values/🦀️.rs` — `delivered_preview` reads the status's
  nested `diagnostics[].issues[]`
- `📓️brep-boolean-input-lifetime-2026-09-15.md` (this report)

---

## 7. Not claimed

*(filled in below)*
