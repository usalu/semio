# 📓️ W3-C — Flow Brep Nodes: Quality Metadata, New Nodes, Silent-Fallback Removal

Worker W3-C on `BREP-KERNEL-DEPENDENCY-FREE-RUNTIME`. Files owned: `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/**`
(the 1866→~2350-line `🦀️.rs`, `🔣️.json` generated artifact, `Cargo.toml`/`📜️script.ts`), plus the shared
kernel access layer `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs` (touched only where
strictly needed — did not disturb W4-A's `brep_invoke`).

## 1. Node quality metadata (DO §1)

Added `NODE_KERNEL_METHOD: &[(&str, &str)]` (93 entries — every registered flow node's operator id
mapped to the `BrepKernel` method it wraps) and `INTENTIONALLY_UNEXPOSED: &[(&str, &str)]` (6 entries
with a reason each: `kind`, `tessellate`, `dispose`, `retain`, `registry_len` are internal
plumbing behind other public entry points; `export_gltf` is never called — glTF/GLB leaves the
extension only through the tessellation mesh bridge). `93 + 6 = 99 = BREP_KERNEL_OPERATIONS.len()`.
Both tables are hand-maintained (not reflectively derived), mirroring W1-A's own documented
rationale for `OPERATION_QUALITY`.

A new `q(method, summary) -> String` helper appends `operation_quality(method)` to every node's
`summary` string as `"... [quality:ExactAnalytic]"` etc. — this is the literal, machine-checkable
place the fidelity data lives in both `register()`'s live `OperatorInfo` catalogue and the packaged
`🔣️.json` descriptor (which serializes `summary` verbatim). I did **not** add a `quality` field to
`neural_engine::OperatorInfo` itself — that struct is shared by every flow extension (dictionary,
bim, logic, draw, math, primitive, list, text), well outside this ticket's file ownership, and
mutating it would ripple into 8+ unrelated crates. Embedding the tag in `summary` was the
minimal-blast-radius way to make quality genuinely present in the registration and the descriptor
without touching shared framework types.

All ~86 pre-existing `reg_geo`/`register_typed`/`register_operator` call sites were mechanically
rewritten (via a verified Python script keyed on each operator id, wrapping only the summary string
argument immediately preceding each call's `vec![`) to route through `q(method, ...)`; every new
node (below) was authored with it directly.

Two tests (`#[cfg(test)] mod tests`):
- `operation_quality_tags_match_the_kernel_contract` — for every `NODE_KERNEL_METHOD` entry, looks
  the id up in the live `module_registry()` and asserts its `summary` contains
  `format!("[quality:{:?}]", operation_quality(method))`.
- `every_kernel_operation_is_either_a_node_or_explicitly_unexposed` — asserts `NODE_KERNEL_METHOD`
  ∪ `INTENTIONALLY_UNEXPOSED` has no duplicate method names, every name is a real
  `BREP_KERNEL_OPERATIONS` entry, every `BREP_KERNEL_OPERATIONS` entry is covered, and the sizes
  match exactly (99).

## 2. New nodes for wave-1 capabilities (DO §2)

- `brep.xform.rotateAbout` → `rotate_about(shape, origin, axis, angle)` — distinct from the
  existing `brep.xform.rotate` (world origin only); test asserts a 180° rotation about an
  off-origin point lands where `rotate` could not.
- `brep.eval.curveClosestParameter` → `curve_closest_parameter` — outputs `parameter`, `point`,
  `distance` (the certified achieved error, not silently dropped).
- `brep.eval.surfaceClosestUv` → `surface_closest_uv` — outputs `u`, `v`, `point`, `distance`.
- `brep.topology.shells` → `solid_shells` — every shell of a solid as an independent geometry
  handle (list output).
- `brep.topology.compound` → `compound` — build a compound from a solid list.
- `brep.topology.explode` → `explode` — inverse of the above, list output.
- `brep.topology.label` → `label` — the handle's persistent label as a diagnostic number; returns
  an explicit `EvalError` (not a silent `0`/`-1`) when the handle carries none.

**Not actionable within this file's scope** (all three require a `BrepKernel` trait signature
change in `⚙️engine/🦀️.rs`, which is out of my ownership per the ticket's file split):
- `interpolate_curve` still takes only `(points, degree)` — no `end tangents`/`closed` params at
  the trait level, so no new channels could be added; the certified-tangent path W1-D2 built
  (`interpolate_curve_with_tangents`) is not reachable through `BrepKernel`.
- `approximate_curve` still returns a bare `GeometryHandle`, no achieved-error second value.
- `tessellate` still returns a bare `MeshTransfer`; `max_chordal`/`max_angular`
  (`TessellationReport`, `tessellate_solid_with_report`) live only in
  `💡️inferences/🧩tessellation` and are never threaded through the trait, so there is no
  tessellate-with-report flow node to add without that trait change. Flagging for whoever owns
  `⚙️engine/🦀️.rs`'s `#region Curves`/tessellate signature next.

## 3. Silent-fallback removal (DO §3)

- `read_xyz_dict` (`brep-geometry.rs`) silently defaulted a missing/malformed `x`/`y`/`z` to `0.0`
  — every point/vector node input (~40+ call sites transitively) was affected. Now requires all
  three axes present, returning `EvalError::MissingInput("<label>.<axis>")`; all 4 internal call
  sites (`read_xyz`, `read_point_list`, `read_nested_point_lists` ×1 each plus one more) updated
  with a descriptive label.
- `ShellMutation`'s `openFaces` used `read_geometry_list(...).unwrap_or_default()`, which silently
  turned a present-but-malformed list into an empty one (same failure mode as a genuinely-absent
  key). Added `read_geometry_list_or_empty` (only a truly-absent key defaults to empty; any other
  error still propagates) and switched the call site to it.
- `Section` (`brep.intersect.section`) silently kept only `faces.into_iter().next()`, discarding
  every other section face a plane through a solid with multiple disjoint cross-sections would
  produce. Now emits the full `faces` list.
- `Split` (`brep.intersect.split`) silently discarded the negative half
  (`let (positive, _negative) = ...`). Now emits both `positive` and `negative` outputs.
- `SurfaceSurfaceIntersect` silently kept only the first intersection wire. Now emits the full
  `wires` list.

All three list/dual-output changes are breaking API changes to those three nodes' output shape —
per the ticket's no-compat-layer rule, no old single-output alias was kept. No other in-repo caller
referenced any of the three node ids (grep-confirmed) before this change.

## 4. Handles (DO §4)

`retain_geometry_handles` (`brep-geometry.rs`) already called `guard.retain(&live_set)`, and
`BrepKernel::retain`'s impl already calls `compact_unreachable()` — this was already wired
correctly (by W1-C/W4-A) before I touched anything; verified, not modified. Added the
`brep.topology.label` node (above) as the flow-facing use of the engine's `label`/`handle_for_label`
persistent-label bridge; did not expose `handle_for_label` itself as a node — it needs a bare
label integer as input with no natural flow-graph source (you would need to have already had a
`GeometryHandle` to read the label off of), so there is no useful round-trip shape for it as a
standalone node.

## 5. Tests (DO §5)

One test per family, all new (beyond the pre-existing primitives/curves/sweeps/features/io tests):
`surface_family_plane_point_stays_in_plane_and_normal_matches`,
`boolean_family_fuse_cut_intersect_report_plausible_volumes`,
`rotate_about_rotates_around_the_given_origin_not_the_world_origin`,
`evaluation_family_closest_parameter_and_closest_uv_report_certified_distance`,
`topology_family_shells_compound_explode_and_label`, plus the two metadata tests in §1.

**Verification — blocked, not a pass.** Required command (foreground, `timeout: 600000`):

```
cd "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust" && RUSTC_WRAPPER="" cargo test --message-format short 2>&1 | tail -60
```

Run three times this session, in the foreground each time (waiting on the PID directly rather than
trusting the tool's own 590s auto-background cutoff), full output kept at
`🗑️generated/w3c-cargo-test-run{1,2,3}.txt`:

1. First attempt: sat on `Blocking waiting for file lock on build directory` for 10+ minutes under
   ~57 concurrent `cargo check`/`cargo test` processes from other workers on this same ticket
   sharing the one root `target/` lock (confirmed via `ps aux`), then failed with
   `error: linking with \`cc\` failed` compiling the leaf crate `foreign-types-macros` — a
   transient fleet-contention artifact (unrelated to any file I touched; never recurred).
2. Second/third attempts both failed identically, before reaching *any* of my code, with:
   ```
   error: failed to load manifest for workspace member `.../✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust`
   Caused by: failed to load manifest for dependency `semio-framework`
   Caused by: failed to read `.../🧰️framework/📦️📦️packages/🦀️rust/Cargo.toml`
   Caused by: No such file or directory (os error 2)
   ```
   Root-caused (read-only, did not touch): `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml` has
   ~10 dependency `path = "..."` entries whose `📦️` package-folder segment is doubled or tripled
   (e.g. `🧰️framework/📦️📦️packages/🦀️rust` instead of the real `🧰️framework/📦️packages/🦀️rust`;
   one line also has `🧬️🧬️schema` doubled) — none of the corrupted paths exist on disk. Since `fem`
   is a root-workspace member, this breaks Cargo manifest resolution for **every** package in the
   repo, not just mine. Completely unrelated to brep/flow; flagged via `spawn_task`
   (`task_f589c3f8`, "Fix duplicated 📦️ path segments in fem Cargo.toml") rather than hand-fixed —
   outside this worker's file ownership and possibly a peer's in-flight edit.
**I am not claiming any test in this report passed.** My own code was reviewed line-by-line against
the live `BrepKernel` trait signatures, `neural_engine::{Registry, OperatorInfo, ChannelSpec,
Dictionary, Value}` APIs, and every existing call-site convention in this file (macro-generated
operators, `with_kernel`/`with_kernel_read` locking, `Dictionary`/`Value` builder chains, the
`register_typed`/`reg_geo` registration shapes) — no full-crate build has yet reached
`semio-s-plugin-flow-extension-brep` itself in any of my three attempts, all three failing earlier
in the dependency graph for reasons confirmed unrelated to any file I own. Re-run the command above
once `task_f589c3f8` (or whoever else is already on it) clears the `fem` manifest and the fleet's
`target/` lock contention drops, and report the actual pass/fail this session could not obtain.

**Unrelated anomaly observed, not touched:** `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs`'s
`kind_label` function currently reads `GeometryKind::Wire => "📡️wire",` (an emoji-prefixed value)
where every sibling arm is a plain lowercase word (`"vertex"`, `"edge"`, `"face"`, …) — this was
already present when I started editing this file this session and is outside every DO item here; I
did not touch it. If nothing else reaches it, it merely round-trips through `geometry_dict`'s
`"kind"` field for wire handles; flagging in case some other, exact-string comparison elsewhere
depends on the plain `"wire"` value.

**Update, active repo-wide corruption in progress (not caused by, or fixed by, this worker):**
partway through this session an active, spreading process began doubling the `📦️` (package-folder)
emoji segment across many `path = "..."` entries repo-wide (the `fem` Cargo.toml above was the
first symptom I hit) — and, separately but concurrently, renaming JSON `"command"` keys to
`"🎮️command"`. By the time I finished, it had reached files *I* had already edited or created this
session: my own package's `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/Cargo.toml`
(all 7 `path = "..."` entries now doubled), `📋️project.json` (all 5 `cwd` paths doubled, all 5
`"command"` keys renamed `"🎮️command"`), `📜️script.ts` (one import path doubled), and my own new
`⚖️gate🌊️flow📐️brep🦀️tests` entries in both `.vscode/launch.json` and
`.vscode/🧩️launch.seed.jsonc` (`"command"` → `"🎮️command"` there too — the entries as I originally
wrote them, quoted verbatim in §6 below, are correct; what is currently on disk is not). My two
actual Rust source files (`🦀️.rs` and `brep-geometry.rs`) were confirmed still clean (grep for
`📦️📦️`/`🎮️command`: zero hits in both) — only path/config metadata was hit, not the logic.

Mid-session the ticket coordinator reported a peer (`semio-2f`) is running a verified repo-wide
repair script for the `📦️📦️` manifest corruption specifically, and asked every worker not to
hand-edit any `Cargo.toml` and to wait 10–20 minutes before re-verifying. I complied: I did **not**
touch any `Cargo.toml`, and — since `📋️project.json`/`📜️script.ts`/the two launch files are hit by
the same active process and a targeted manual fix could race against or diverge from whatever
repair also covers them — I left those as-is too rather than patch a moving target. **All of these
files should be re-checked once the repair (or `cargo metadata --no-deps` at the repo root
succeeding) confirms the corruption has stopped and been reverted** — if `📋️project.json`/
`📜️script.ts`/the launch entries are not already fixed by that pass, they need the mechanical
`📦️📦️` → `📦️` and `🎮️command` → `command` fix applied by hand at that point (nothing else in any
of those four files needs to change).

## 6. `launch.json` (DO §6)

Added `⚖️gate🌊️flow📐️brep🦀️tests` to `.vscode/🧩️launch.seed.jsonc`, mirroring
`⚖️gate🌊️flow🦀️check`'s shape exactly (`bun x nx run @semio-tech/flow-extension-brep-rust:test`,
group `4_gate`, order `408.15`, right after `408.1`). Both `.vscode/launch.json` and the seed were
already concurrently modified by other workers when I started (`git status` showed both `MM` at
session start), so rather than running a full repo-wide seed→launch.json regeneration (which risks
clobbering peers' in-flight, uncommitted edits to either file), I hand-mirrored the identical new
entry into both files at the same position. If a later regeneration pass runs, this entry is
already seed-sourced and should survive it unchanged.

## Files touched

- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`

## Open items / honest gaps

1. Test run not yet confirmed green (§5) — re-run and report the real pass/fail once the fleet's
   `target/` lock contention clears.
2. Three DO-§2 capabilities (interpolate_curve tangents/closed, approximate_curve achieved error,
   tessellate-with-report) need a `BrepKernel` trait signature change in `⚙️engine/🦀️.rs` before a
   flow node can expose them — out of this worker's file ownership, flagged for whoever owns that
   file's relevant `#region`.
3. `semio-s-plugin-flow-extension-brep`'s `Cargo.toml` still depends on `semio-framework-3d`, which
   this crate's source has zero references to (grep-confirmed) — W1-A flagged the same dead
   dependency in `semio-framework-os-flow`/`semio-s-plugin-cad` and deferred pruning it; I made the
   same call here for the same reason (out of this DO-list's explicit scope, touching a `Cargo.toml`
   under fleet-wide build contention was judged not worth the risk this session).
4. `🔣️.json` (checked-in, last regenerated 2026-09-02 at `21fbcd3538`) is a build artifact produced
   by `nx run @semio-tech/flow-extension-brep-rust:package` from a real wasm component build — not
   hand-edited. It still reflects the pre-this-ticket `register()` output (no quality tags, no new
   nodes) until that target is re-run; I did not attempt a wasm packaging build in this session
   given the fleet's `target/` lock contention already blocking plain `cargo test`. Regenerate it
   once §5's test run confirms `register()` itself is correct.
