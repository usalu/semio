# Wave 5 Report — Demonstrator (`semio-s-plugin-demonstrator`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/🎪️demonstrator/**` plus this ticket folder.
Artifact key `playground`, prefix `Playground`, schema id `s.demonstrator.playground`.
Former snapshot type `PlaygroundDocument` → `PlaygroundSnapshot` (moved out of `🧬️mutations` into `📸️snapshot/🧬️schema`).

This artifact was one of only two structurally incomplete facet sets in the repo. Almost every facet
had to be created from scratch rather than renamed.

## 1. Field inventory (final)

| Field | State | Notes |
| --- | --- | --- |
| `schema` | persistent | document schema id (`playground.playground`) |

Honest inventory: the previous `PlaygroundDocument` only carried `schema: String`. There is no
`DocumentApp` / `Config` / `Draft` owned by demonstrator for this artifact — pane apps come from the
six source plugins. Snapshot facet = exactly that one persistent field. No shared-ui / local-ui /
preview / effect fields.

## 2. Diff-delta shape

`PlaygroundDiff` sparse field delta:

- `artifact: Option<Box<PlaygroundArtifact>>` — whole replacement wins
- persistent: `schema: Option<String>`

`MutationDiff<PlaygroundSnapshot>` applies persistent entries; `apply_to_artifact` applies all.
`absorb` merges field-wise. Mutations: `NoMutation` (empty diff) and `SetSnapshot { snapshot }`
(whole-replacement via `diff_set_snapshot`). Folder `📄set-document` renamed to `🖼️set-snapshot`.

## 3. Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]` (same as existing demonstrator
panes glue). Nested:

- `artifacts::playground::schema`
- `artifacts::playground::snapshot::{schema, pack}`
- `artifacts::playground::diff::{component, schema}` (runtime `pub use super::schema::*;`)
- `artifacts::playground::{op, mutations, dsl, spr, engine}`

`extern crate` aliases for `dsl` / `store` / `protocol` / `schema`. TypeScript `📦️index.ts` mirrors
schema / snapshot / diff / pack / op / spr / engine exports.

## 4. Components created from scratch

Previously present (stubs / incomplete): root `🦀️component.rs`, `⚙️engine` (3-line stub),
`🔧️op` (reexport only), `🧬️mutations` (with `PlaygroundDocument` inside), `📚️examples`.

**Created from scratch:**

- `🧬️schema/` — all five leaves (`PlaygroundArtifact`)
- `📸️snapshot/🧬️schema/` — all five leaves (`PlaygroundSnapshot`)
- `📸️snapshot/🎒️pack/` — rust + ts + protocol (`segment Snapshot kind 3`)
- `🔺️diff/` — runtime + grammar + ts + five schema leaves (`PlaygroundDiff`)
- `🗣️dsl/` — runtime + grammar + ts
- `📡️spr/` — runtime + protocol + ts
- op grammar + ts (runtime rewritten)
- engine rewritten to own real `PlaygroundArtifact` + `PlaygroundArtifactEngine`
- engine `🟦️component.ts` stub
- mutation TS stubs for `🫙no-mutation` and `🖼️set-snapshot`
- mutations grammar; set-snapshot mutation/diff/inverse leaves

## 5. Gate tails (verbatim)

### cargo check -p semio-s-plugin-demonstrator

```
    Checking semio-s-plugin-demonstrator v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 26.69s
```

### cargo test -p semio-s-plugin-demonstrator --lib

```
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s
```

### bun ./📜️script.ts policy 2>&1 | rg -i 'playground|demonstrator'

```
(empty — no lines matched)
```

Direct `policyArtifactSchemaBreaches()` filter: **demonstrator/playground artifact-schema breaches: 0**.

### bun nx run @semio-tech/plugin-registry:check 2>&1 | rg -i 'playground'

```
(empty — no lines matched)
```

## 6. Shared-surface / concurrent-agent notes

- Temporarily blocked by `semio-s-plugin-sourcing` incomplete `SetSnapshot` rename in
  `🎮️commands/📄️document` (`snapshot` unbound + leftover `set_document` call). Minimal two-line
  fix applied so workspace cargo could load; belongs to the sourcing wave-5 agent.
- Pane `🏭️bearbeiten` updated for peer rename `Process3dDocument` → `Process3dSnapshot`.
- Registry still reports undeclared empty plugin facet stubs (`🔌️plugin/{apps,manifest,setup,capabilities}`)
  — same class of residual as green draw; not playground-related. Playground filter is empty.

## 7. Could not validate

- No demonstrator-owned `DocumentApp` / live UI session for playground (bundle still only hosts the
  six source-plugin apps). Codec/engine/mutation laws covered by unit tests only.
- Did not run TypeScript vitest (package test script is a stub echo).
