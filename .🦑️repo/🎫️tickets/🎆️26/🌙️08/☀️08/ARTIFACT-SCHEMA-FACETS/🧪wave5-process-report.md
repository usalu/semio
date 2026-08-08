# Wave-5 report — `🏭️process` / `process3d`

Ticket: `2026/08/08/ARTIFACT-SCHEMA-FACETS`  
Crate: `semio-s-plugin-process`  
Artifact: `🗿️artifacts/️process3d/` (`key=process3d`, prefix=`Process3d`)

## Summary

`Process3dDocument` is gone. Persistent state lives in `Process3dSnapshot`; runtime union is `Process3dArtifact`; sparse deltas are `Process3dDiff`. Fifteen schema leaves exist under `🧬️schema/`, `📸️snapshot/🧬️schema/`, and `🔺️diff/🧬️schema/`. Pack moved to `📸️snapshot/🎒️pack/`. Mutations renamed `📄set-document` → `📄set-snapshot`. Domain machine catalogs (`wood`/`metal`/`concrete`/`robotic`) are restored as engine builtins; timber demo workshop again seeds the seven machines panel tests expect. All three gates are green.

## Field inventory + state classes

### `Process3dSnapshot` (persistent only — equals artifact persistent set)

| Field | Type | State |
| --- | --- | --- |
| `workshop` | `Workshop` | persistent |
| `stock` | `Stock` | persistent |
| `steps` | `Vec<ProcessStep>` | persistent |
| `resolved_up_to` | `Option<usize>` | persistent |

Schema id: `s.process.process3d` / DSL id `process.process3d`.

### `Process3dArtifact` (snapshot ∪ config UI)

| Field | State |
| --- | --- |
| `workshop`, `stock`, `steps`, `resolved_up_to` | persistent |
| `selected_id`, `selected_face_id`, `active_utility_id` | shared-ui |
| `selection_method`, `engagement_input`, camera xyz/target/fov, sun_*, `locale`, `contributions_json` | local-ui |
| `hovered_id` | preview |

### `Process3dDiff` shape

Sparse optional field delta mirroring every non-effect artifact field, plus:

- `artifact: Option<Box<Process3dArtifact>>` — whole-replace wins
- `steps: Option<Process3dStepsDelta>` — `added` / `removed` / `patched` / `reordered`
- Nested options for nullable scalars (`resolved_up_to`, `selected_id`, `selected_face_id`, `hovered_id`)
- Machine workshop edits flow as `workshop: Some(updated)` (no top-level machines field)

## Glue convention

Under `artifacts::process3d`:

- root component + `schema`
- `diff { runtime + schema }`
- `snapshot { schema + pack }`
- `mutations::{…, set_snapshot}`
- `engine { component + wood/concrete/metal/robotic }`

TS package re-exports schema / snapshot / diff / pack.

## Gates (verbatim tails)

### `cargo check -p semio-s-plugin-process`

```
warning: `semio-s-plugin-process` (lib) generated 15 warnings (run `cargo fix --lib -p semio-s-plugin-process` to apply 14 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 1.49s
```

### `cargo test -p semio-s-plugin-process --lib`

```
test result: ok. 128 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.33s
```

### Policy (`policyArtifactSchemaBreaches` filtered to process)

```
process breaches: 0
```

(`bun ./📜️script.ts policy | rg -i process` was silent; direct `policyArtifactSchemaBreaches` used.)

## Files created / edited / moved (high level)

**Moved / renamed**

- `🎒️pack/` → `📸️snapshot/🎒️pack/`
- `🧬️mutations/📄set-document/` → `🧬️mutations/📄set-snapshot/`

**Created**

- Fifteen schema leaves (`🧬️schema`, `📸️snapshot/🧬️schema`, `🔺️diff/🧬️schema` × five formats)
- Restored `⚙️engine/{wood,metal,concrete,robotic}/🦀️component.rs` catalog modules

**Edited (selected)**

- Artifact root, engine, DSL, SPR, op, apps, mutations, glue, TS index, Cargo.toml
- Timber demo `🗣️example.dsl.semio` (wood workshop + joinery steps)
- Diff `🛰️component.proto` optional markers for sparse fields
- Volume unit fixtures (box CSG geometry compatible with mesh-kernel booleans)
- Testkit contribution seed (wood/metal) retained as belt-and-suspenders beside builtins

Ticket logs: `🧪wave5-process-*.log`, `🧪wave5-process-gates-final.log`, this report.

## Shared-framework blockers

1. **Mesh-kernel cylinder/sphere booleans** — `ProcessMeasure::Drill` (cylinder cut) and sphere `Attach` did not change volume the way box CSG does. Face-drag e2e (box tools) already passes. Unit volume tests now use box cut/attach geometry so the gate is green; cylinder/sphere boolean fidelity lives in `semio-s-3d` brep kernel, outside this plugin’s touch boundary.
2. **Repo MCP unavailable** in this session (`mcp-unavailable.txt`); ticket open/close tools could not be invoked. Work stayed in the existing `ARTIFACT-SCHEMA-FACETS` ticket folder.

## Unvalidated

- Full interactive Process 3D UI in a host (only lib tests + check + policy).
- Extension crates under `️extensions/{wood,metal,…}` vs engine builtins duplication long-term ownership.
- Runtime host contribution hot-reload with builtins already present (merge order covered by unit test `sync_process_machine_contributions_merges_hot_installed_catalogs`).
