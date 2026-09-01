# Interactive-Job Migration Result: 47 of 47 `Migrated` (final)

Supersedes the wave-1 (27/28) version of this report. `addPrimitive` is now migrated too, after the
coordinator edited `🧪️interactive-job/🔣️schema.json` (not owned by this packet): `preparation.maxItems`
raised 1 → 2, and a 7th `oneOf` signature added — `lanes: ["Artifact","Config"]` /
`preparation: ["Artifact","Config"]`. `lanes.maxItems` was already 2, unchanged.

## 1. `addPrimitive` — what changed in the three owned files

Its handler (`➕️add-primitive/🦀️component.rs`, not owned/not modified) unconditionally emits both a
`CreateObject` Artifact mutation and a `SetActiveObject` Config mutation — never just one.

1. New `LowpolyCommandDisposition::ArtifactConfig = 7` variant (7th, alongside the existing six). It
   threads through `workspace_identity()`'s `(u64::from(self.disposition as u8) << 56)` XOR term and the
   checkpoint disposition-byte check (`target[4] = self.disposition as u8`; `restore` rejects
   `checkpoint[4] != self.disposition as u8`) automatically — both are already generic over
   `self.disposition as u8`, no per-variant code to add.
2. `lowpoly_command_disposition`: `"addPrimitive" => ArtifactConfig`.
3. `lowpoly_command_admitted`: `AddPrimitive(payload) => payload.kind.as_deref().is_none_or(field)` (its
   only string field). The match is now exhaustive over all 47 `LowpolyCommand` variants — the trailing
   `_ => false` was removed (would otherwise be an `unreachable_patterns` warning under `-D warnings`).
4. `lowpoly_artifact_mutation_retained_bytes`'s `CreateObject` arm was already exact — confirmed, not
   changed: `Ok(lowpoly_object_retained_bytes(&payload.object))`, where `lowpoly_object_retained_bytes`
   walks id + name + mesh-handle + **all paint layers** (name + blend_mode + pixels per layer), not just
   the id. This helper was added in wave 1 specifically because `CreateObject`'s payload is one whole
   `LowpolyObject`.
5. `PUBLICATION_CONTRACTS` += `{ tool_id: "addPrimitive", lanes: &[Artifact, Config] }`;
   `bounded_first_step_tool_proofs!` += `"addPrimitive" => resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1)`
   (same bounds as every other migrated tool).
6. `lowpoly_retained_reduce`: `AddPrimitive(payload) => add_primitive::handle(payload, &doc, &cfg, &mut bounded)`,
   using the shared `bounded = LowpolyScratch::default()` (NOT the `threaded!` macro — `ArtifactConfig`
   has no `Transient` lane, and `threaded!` unconditionally emits an `ephemeral.transient` entry, which
   would fault against a lane set that doesn't include `Transient`). Its `Result<Emit, Fault>` flows
   through the generic `let emit = match {...}?; Ok(Complete(emit))` tail — `Complete` already publishes
   whatever the `Emit` carries to every lane its `PUBLICATION_CONTRACTS` entry declares, so a `Complete`
   holding both `artifact_mutations` and `config_mutations` naturally satisfies "publish to both lanes";
   no `CompleteWithEphemeral` needed. The only remaining `_`-shaped arm is `PaintStrokeEnd(_) => return
   Err("lowpoly-paint-stroke-end-routes-through-dedicated-step")` — renamed from the old
   `"lowpoly-batch-only-command-entered-retained-reducer"` (no longer accurate: nothing is batch-only any
   more). `PaintStrokeEnd` is intercepted by `LowpolyRetainedCommandWork::step` before this function is
   ever called, so this arm only exists to keep the match exhaustive.
7. `LOWPOLY_BATCH_ONLY_TOOL_IDS` deleted entirely (was down to `["addPrimitive"]`, now empty) — including
   its doc comment. `LOWPOLY_MIGRATED_TOOL_IDS` now has all 47 ids. The `.action_interactive_job("addPrimitive", ...)`
   manifest line flipped to `Migrated`. Both self-check test assertions adapted: the partition test now
   builds `partition` from `LOWPOLY_MIGRATED_TOOL_IDS` alone (the `.chain(LOWPOLY_BATCH_ONLY_TOOL_IDS)` and
   the `assert!(LOWPOLY_BATCH_ONLY_TOOL_IDS...is_none())` line were removed, since there is nothing left to
   chain/assert); `bounded_first_step_tool_proofs().len()` and `PUBLICATION_CONTRACTS.len()` both 46 → 47.
8. `🧪️interactive-job/🔣️component.json`: `addPrimitive`'s route flipped to `Migrated`, lanes
   `["Artifact","Config"]`, preparation `["Artifact","Config"]`, `blocker: null`.
9. `📦️packages/🟦️typescript/📜️script.ts`: counts `46/1` → `47/0`; added `"Artifact+Config|Artifact+Config"`
   to the signature allowlist; **also fixed a now-stale hostile-fixture case** — hostile[2] used to mutate
   every `BatchOnlyPendingRewrite` route's `blocker` to `""`, which is a no-op once there are zero such
   routes (the mutated fixture would equal the valid one, and the test would wrongly fail asserting a
   "hostile" fixture was accepted). Replaced it with a non-null `blocker` on a `Migrated` route — the
   schema's `if classification===Migrated then blocker: {type:"null"}` rejects it the same way, so
   coverage of "blocker/classification mismatch" is preserved, just via a case that still exists at 47/0.

### Residual risk carried forward (not asked for, flagging for completeness)

`addPrimitive`'s handler reaches `session::build_doc`, exactly like the 21 `ArtifactTransient` mesh-edit
commands from wave 1 — `build_doc` needs `LowpolyScratch.mesh_workspace` to already have a byte-identical
entry for every existing object, or `LowpolyDocument::reload_meshes` fails `StaleMeshWorkspace` and the
call silently no-ops (`Ok(Emit::default())`... via `add_primitive::handle`'s own `let Some(...) = build_doc(...) else { return Ok(Emit::default()) }`).
Because `ArtifactConfig` has no `Transient` lane, `addPrimitive`'s reduce arm uses a blank
`LowpolyScratch::default()`, not the live `context.transient`. In practice: on a document that has never
been mesh-edited (the default seed matches), or immediately after another `addPrimitive`/no mesh edits,
this is fine; after any `extrude`/`inset`/etc. has run, `addPrimitive` would silently do nothing until a
`Transient` lane and the same rehydrate-then-republish pattern wave 1 gave the other 21 commands is added.
This was implemented exactly to the coordinator's spec (`lanes: &[Artifact, Config]`, no `Transient`, per
their explicit schema/oneOf signature); raising it here rather than silently deviating from the instruction.

## 2. Verification (Task B)

Per the coordinator's finding: another session is mid-refactor on `semio-s-plugin-stdio` (1055 modified
files, currently failing to compile), which lowpoly's io layer depends on — `cargo check -p
semio-s-plugin-lowpoly` cannot go green until that lands, independent of anything in this packet. Per
instruction, no `cargo` command was attempted this round. What was actually run:

| Check | Result |
|---|---|
| `bunx nx run "@semio-tech/lowpoly-js:test"` | **PASS** — `[DEBUG] lowpoly interactive-job owned source/fixture ok: 47 Migrated, 0 BatchOnlyPendingRewrite` and `[DEBUG] lowpoly interactive-job Ajv hostile oracle ok: duplicate, missing lane, non-null blocker on migrated, lane/preparation mismatch rejected`, `Successfully ran target test` |
| `rustfmt --edition 2021 --check` on the owned `component.rs` | Parses cleanly — `0` `error` lines; exit 1 is the pre-existing formatting-style diff only (unchanged from wave 1) |
| Internal consistency (script-verified, not cargo): `LOWPOLY_MIGRATED_TOOL_IDS` / `PUBLICATION_CONTRACTS` / `bounded_first_step_tool_proofs!` entries / `.action_interactive_job(..., Migrated)` calls | All four sets = 47, pairwise equal, zero duplicates |
| `cargo check` / `cargo check --target wasm32-wasip2` / `cargo clippy -D warnings` / `cargo test --lib` | **Still blocked** — pinned on the peer `semio-s-plugin-stdio` refactor landing, not attempted this round per instruction |

Nothing above is claimed without having actually been run. The four `cargo`-based checks remain the one
open item, blocked on the peer session, not on anything in this packet.
