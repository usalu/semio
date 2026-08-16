# W3-A — Pilot P1: the repo's first composite mutation

Written by the coordinator. The lane landed its code and then stalled in a poll loop against the external
`semio-s-plugin-stdio` breakage without writing a report; this documents what is actually on disk, reviewed
file-by-file, and states honestly what could and could not be verified.

## What landed

`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️duplicate-widget/`
in the **composite** shape — `{🦠️mutation, 🧩️plan}`, with no `🔺️diff` and no `↩️inverse`, exactly as the taxonomy
now requires of a composite.

- `🦠️mutation/🦀️component.rs` — `DuplicateWidget { source_id, new_id, synapse_id, from_port, to_port }`,
  `#[derive(dsl_derive::CompositeMutation)]` with `#[composite(snapshot = FlowSnapshot, op = FlowMutation)]`,
  and `impl CompositeMutationKind<FlowSnapshot, FlowMutation>` whose `SEMANTICS` is
  `{verb: "duplicate", entity: "widget", kind: "duplicate-widget", record: "DuplicatedWidget"}`.
- `🧩️plan/🦀️component.rs` — the plan body plus a shared `precondition`.

Also: `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎮️commands/📋️duplicate-widget/` so the composite is reachable from
the UI rather than only from tests.

## The part that matters — a mutation calling other mutations

```rust
planner.call(FlowMutation::CreateWidget(CreateWidget { index: scene.widgets.len(), widget: copy }))?;

let wired = flow_working_scene(planner.base());
planner.call(FlowMutation::ConnectWidgets(ConnectWidgets { index: wired.synapses.len(), id: payload.synapse_id.clone(), … }))?;
```

Both steps are the artifact's **existing leaf kinds**, invoked through one shared `Planner`, and the second
reads `planner.base()` — the snapshot as advanced by the first — rather than the original base. That is the
whole point of the mechanism: the composite reimplements nothing, and its `diff`/`inverse` are folded from
this sequence by `fold_plan_diff`/`fold_plan_inverse` via the derive.

`precondition` is shared by `plan` (mapped to a typed `PlanError`) and `CompositeMutationKind::validate` (the
pre-dispatch check), so a bad payload is a typed rejection on both paths and never a panic.

## Tests written

In `🦠️mutation`: label/target correctness; `SEMANTICS.kind`/`verb` match the directory name.
In `🧩️plan`: fold-vs-handwritten equivalence and inverse restoration against a real base snapshot
(`fold_plan_diff`/`fold_plan_inverse` imported directly).

## Gate status — honest

**`cargo test -p semio-s-plugin-flow --lib` could not be run.** Every `semio-s-plugin-*` crate depends on
`semio-s-plugin-stdio`, which is mid-restructure by ticket `26/08/16/FULL-STDIO-…`: its gltf inference tree
currently fails with `unresolved imports self::adjacency, self::area_volume, … self::topology` and two
`include_str!` files that do not exist (`🔗️component.graphql`, `🛰️component.proto`). Verified by
`cargo check -p semio-s-plugin-stdio` failing identically with nothing from this ticket in the error set.
No pass is claimed for this crate.

**What WAS verified:** `bun ./📜️script.ts policy` accepts the new composite shape — `👯️duplicate-widget`
produces **zero** `mutation-migration/triad-completeness` rows (the `{🦠️mutation,🧩️plan}` branch works on a
real directory, not just in the gate's unit logic) and zero `mutation-migration/impl-presence` rows (the
`impl … CompositeMutationKind` detection works). The only rows it produces are two
`mutation-migration/ts-mirror` entries — which every one of flow's 30 pre-existing mutation leaves also
produces (repo-wide total moved 1277 → 1279). The composite matches the artifact's prevailing state; it did
not regress it.
