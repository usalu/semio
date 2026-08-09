# Wave 5 Report — Imperative (`semio-s-plugin-imperative`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/📜️imperative/**` plus this ticket folder.
Key `imperative`, prefix `Imperative`, schema id `s.imperative.imperative`. Former snapshot type `ImperativeDocument` → `ImperativeSnapshot`.

## 1. Field inventory (final)

| Field | State | Notes |
| --- | --- | --- |
| `schema` | persistent | document schema id (`imperative.document`) |
| `path` | persistent | `Path` step tree |
| `seed` | persistent | `BTreeMap<String, Value>` scope seed |
| `selectedStepIds` | shared-ui | from `ImperativeConfig` |
| `locale` | local-ui | BCP-47 |
| `contributionsJson` | local-ui | `imperative.module` contribution JSON |
| `runOutputJson` | effect | last run scope JSON (not in `ImperativeDiff`) |

Snapshot facet = `schema`, `path`, `seed` only. No `DocumentApp::Draft` (`NoDraft`).

## 2. Diff-delta shape

`ImperativeDiff` sparse field delta:

- `artifact: Option<Box<ImperativeArtifact>>` — whole replacement wins
- persistent: `schema`, `path: Option<ImperativePathDelta>`, `seed: Option<BTreeMap<String, Value>>`
- shared-ui: `selectedStepIds: Option<ImperativeStringList>`
- local-ui: `locale`, `contributionsJson`
- no effect fields

`ImperativePathDelta` / `ImperativeStepsDelta` for identified step collections at a `PathRef`. `MutationDiff<ImperativeSnapshot>` applies persistent entries; `apply_to_artifact` applies UI classes.

## 3. Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]`. Nested:

- `artifacts::imperative::schema`
- `artifacts::imperative::snapshot::{schema, pack}`
- `artifacts::imperative::diff::{component, schema}`

`extensions` module wires in-process native registrars (WASM `extension_exports!` gated to `wasm32` only).

## 4. Other structural changes

- Pack moved to `📸️snapshot/🎒️pack/`
- `ImperativeEngine` owns `ImperativeArtifact` + cached `ImperativeSnapshot`
- Config envelope `imperative.config`; document envelope `imperative.imperative`
- `default_imperative_contributions_json()` shared by engine bootstrap and `ImperativeConfig` default (avoids render sync wiping registry)
- Native registrar plugin id `imperative-extension-core` matches effect contribution
- Example `🗣️example.dsl.semio` — two-step `state.set` + `log.print`

## 5. Gate tails (verbatim)

### cargo check -p semio-s-plugin-imperative

```
    |    ^^^^^^

warning: `semio-s-plugin-imperative` (lib) generated 18 warnings (run `cargo fix --lib -p semio-s-plugin-imperative` to apply 12 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.93s
```

### cargo test -p semio-s-plugin-imperative --lib

```
test artifacts::imperative::engine::tests::host_set_step_params_updates_existing_step ... ok

test result: ok. 75 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```

### bun ./📜️script.ts policy 2>&1 | rg -i 'imperative'

```
(no output — zero imperative breaches; confirmed via policyArtifactSchemaBreaches filter)
```

## 6. Shared framework blockers

None recorded. In-process extension bootstrap required `#[cfg(target_arch = "wasm32")]` on `extension_exports!` in five extension components to avoid duplicate linker symbols when mounting extensions through plugin glue on native targets.

## 7. Not validated

- Full plugin WASM component build / wasm-pack pipeline
- End-to-end IDE launch.json session (devs use launch configs per AGENTS.md)
- Repo MCP ticket close / `repo://goals` (MCP unavailable in agent environment)
