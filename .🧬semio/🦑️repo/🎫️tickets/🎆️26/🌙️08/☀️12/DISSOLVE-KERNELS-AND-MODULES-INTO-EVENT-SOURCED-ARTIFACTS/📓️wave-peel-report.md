# 📉 Wave PEEL — brep ops batch into `✳️brep`

> Written by the coordinator: the executing agent's tool environment blocked it from
> writing `.md` files, so it delivered the report inline. Content is its own, verbatim
> in substance; the coordinator added the concurrency reconciliation in §4.

## 1. Measured dependency graph

Built by grepping every `crate::brep::X` reference across all 35 `📐️brep/*/🦀️component.rs`.
Load-bearing rule: a module may move once **nothing that stays behind references it**.
Modules being moved may freely reference modules staying behind — that is the legal
`stdio → semio-framework-3d` forward edge.

Confirmed for batch 1 (ops) = {boolean, offset, sweep, blend, sew, heal, imprint,
tessellate, classify}: no module in batches 2–4 references any of them.

**New finding — batch 4 is NOT flat.** Two further indivisible cycles, in addition to the
already-documented topology SCC:
- `{polynomial, bspline, curve, surface}` — mutually recursive through `bspline`
- `{vector, tolerance, predicates}` — `vec → tolerance → predicates → vec`

## 2. Batch 1 landed — symbol parity

| module | new home | verified symbols |
|---|---|---|
| boolean | `diff::boolean` | `boolean_solid`, `compound_cut`, `section_solid_by_plane`, `split_solid_by_plane`, `BooleanOp` |
| offset | `diff::offset` | `offset_face`, `thicken_face`, `offset_solid`, `shell_solid_with_open_faces`, `draft_angle` |
| sweep | `diff::sweep` | `extrude_face`, `revolve_face`, `loft_profiles`, `sweep_along_path`, `pipe`, `helical_sweep` |
| blend | `diff::blend` | `fillet_edges`, `fillet_variable`, `chamfer_edges` |
| sew + heal | `diff::sew` (folded) | `sew_faces`, `heal_solid`, `defeature`, `convert_to_nurbs`, `HealingReport` |
| imprint | `diff::euler` (folded) | `split_planar_face_by_line` |
| tessellate | `inferences::tessellation` | `tessellate_solid`, `tessellate_face`, `tessellate_wire`, `sample_edge_polyline` |
| classify | `inferences::classification` | `point_in_loop`, `point_in_face_uv`, `point_in_solid` |

Foldings are deliberate and documented in the destination docstrings: no pre-allocated stub
existed for `heal` or `imprint`; `heal_solid` calls `sew_faces` directly so it shares that
subdir (same "one compute subdir, not 1:1" precedent `✂️intersect` set for int-cc/cs/ss);
`imprint` is built entirely on Euler operators.

After deleting the 9 source dirs, `grep -rln` for every symbol above under `📐️brep`
returns nothing — **no duplication window left open**.

**Pre-existing bug fixed in transit**: `🖋️imprint`'s
`resolve_edge_containing_param_picks_survivor_after_split` sat *outside* `mod tests {}` due
to a malformed brace, so it was never `#[cfg(test)]`-gated. Moved inside; same logic.

## 3. Verification (exact mandated form, run sequentially against the shared target dir)

```
cargo check -p semio-framework-3d  --all-targets → 0 errors
cargo test  -p semio-framework-3d  --lib         → 273 passed, 0 failed
cargo check -p semio-s-plugin-stdio --all-targets → 0 errors
cargo test  -p semio-s-plugin-stdio --lib         → 3003 passed, 5 failed
```
The 5 stdio failures are byte-identical by name to the pre-existing set in
`scratch-w0-baseline-failures-sorted.txt` (binary/extent, dwg, dxf/bounds, ifc, zip) — none in brep.

## 4. Test arithmetic — and a stale-baseline correction

The agent grep-counted `#[test]` per file before deleting and after writing: **exactly 46
removed from framework-3d, exactly 46 added to stdio.**

It then flagged that the brief's framework-3d baseline of **396 was stale**: 273 + 46 = **319**,
not 396. It was right, and the coordinator reconciles the gap:

```
396 (coordinator measurement)
 −77  🎬️scene relocated to 🖱️ui by the CONCURRENT wave MESH   (77 #[test] fns at both
      the session baseline and the new path — exact match)
 = 319 (this wave's true starting point)
 −46  ops batch → stdio
 = 273 ✅
```
Sum conserved against the agent's own verified baseline: **319 + 2957 = 3276 → 273 + 3003 = 3276.**
Nothing lost, nothing duplicated. The lesson is the standing one — a baseline is a timestamp,
and this tree had two of my own waves running concurrently.

## 5. `⚙️engine` addendum — investigated, named, NOT done

- **`📦️mesh-io` → `🚪️io/`**: declined. `🚪️io/🦀️component.rs` is actively scaffolded by another
  session ("🚧 scaffolded by W1b… W4 adds the real import/export leaves") with no pre-mounted
  subdir for mesh bridging; moving it means adding a new `#[path]` to a file every stdio session
  touches. Its imports were still repointed, since it calls the relocated `tessellate_solid`.
- **`BrepKernel` trait removal**: declined, with the consumers named rather than hand-waved —
  `🌊️flow/📐️brep-geometry`'s `static KERNEL: OnceLock<RwLock<Box<dyn BrepKernel + Send + Sync>>>`
  (a process-global swappable session shared across procedural3d/playbook/flow-wasm hosts so
  linked operator crates see the same handles), the packaged `flow/🧩️extensions/📐️brep` operator
  crate, process3d's `&mut dyn BrepKernel` CSG replay, and four cad call sites.
  **This is a genuine trait-object facade use, not an incidental habit** — removing it means
  restructuring that OnceLock concurrency pattern plus the cad/process call sites.

## 6. Blocked gates — foreign, not attributable to this wave

Both are dangling `#[path]` mounts left by another session's `🎮️commands` directory renames:
```
📐️cad     glue.rs:601 → 🎮️commands/🎥️set-camera/  (on disk: 🎥️camera)
🏭️process glue.rs:568 → 🎮️commands/🗂️text-select/ (on disk: 🗂️selection)
```
Neither touches brep. Reported to the owning session rather than fixed.

## 7. Remainders

1. Batches 2 (queries), 3 (topology SCC), 4 (foundations, containing the two newly-found
   sub-cycles) unstarted — ~10,827 LOC across 26 subdirs.
2. `cad`/`process` gates unmeasurable until the foreign dangling mounts are fixed.
3. `⚙️engine`'s mesh-io relocation and `BrepKernel` removal named but not attempted.
