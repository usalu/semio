# Playbook, Trinity Jack, and Block2d Retained Route Cohort

## Scope and Census

The official live-r8 working census assigned 27 retained command routes across exactly three app owners. This change admits 10 routes and keeps 17 routes fail-closed. Every admitted route publishes exactly one `Config` lane. No route in this cohort claims `Artifact`, `Transient`, or `HostOnly` publication.

| App | Baseline | Migrated | Fail-closed |
| --- | ---: | ---: | ---: |
| Playbook | 9 | 2 | 7 |
| Trinity Jack | 9 | 8 | 1 |
| Block2d | 9 | 0 | 9 |
| Total | 27 | 10 | 17 |

The schema-first source of truth is `🔣️playbook-trinity-jack-block2d-retained-v1.schema.json`; the instance/oracle is `🧪️playbook-trinity-jack-block2d-retained-v1.json`.

## Admitted State Machines

### Playbook

`setLocale` and `setContributions` are owned by `PlaybookRetainedCommandJobFactory`. Each job owns its command payload, exact document/config/history/interaction roots, operation context, completion owner, retained work, and input wire page. There is no process-global payload owner. Admission is bounded to 8,192 raw bytes, 64 decoded items, 64 work items, 16,384 output bytes, and 7,500 microseconds per step. Checkpoint wire owners and oversized wire pages are rejected.

Both commands reduce to app-owned config mutations. `PlaybookOneItemPreparationFactory` prepares one exact config mutation against the retained base root in two checkpointed phases: bounded semantic candidate construction, then exact-authority publication. It rejects a config root or mutation whose serialized representation exceeds the fixed 32,768-byte Store envelope, supports cancellation between phases, and retires the candidate, prepared value, mutation, base root, and authority through bounded close steps. Publication is exactly `Config`.

### Trinity Jack

`setViewport`, `textEdit`, `textSelect`, `requestCompletions`, `setLodMode`, `editorEngagementInput`, `graphEngagementInput`, and `resultsEngagementInput` are owned by `JackRetainedConfigJobFactory`. The same retained ownership and 8,192/64/64/16,384/7,500 contract applies. The reducer calls the app-owned Jack command functions directly and produces only Jack config mutations.

`JackConfigPreparationFactory` owns exact two-phase config preparation, live authority, cancellation, resumable close, and root return. Candidate construction and publication are separate checkpoints, and the 32,768-byte serialized Store envelope bounds both mutation preflight and exact-base preparation. Each publication contract is exactly `Config`.

## Fail-closed Routes and Missing Seams

### Playbook

`addStep`, `removeStep`, `moveStep`, `addBlock`, `removeBlock`, and `moveBlock` mutate composed child storage that does not expose an exact retained item cursor. `updatePlaybook` likewise lacks a resumable document-mutation preparation cursor over its exact retained child root. Promoting these routes would hide document-wide child work behind one retained step, so all seven remain `BatchOnlyPendingRewrite` with no publication claim.

### Trinity Jack

`patchNodes` obtains a whole composed scene through `fixture.nodes()` and then scans node identifiers. Jack exposes neither a retained scene-root node cursor nor a checkpointable patch phase. It remains `BatchOnlyPendingRewrite` with no publication claim.

### Block2d

All nine routes remain fail-closed. Block2d command dispatch is async, while the current shared retained command-work seam is synchronous. `addHandle` and `addHandleKind` additionally scan collections to discover the next identifier. `edit` performs nested document diff scans before emitting one monolithic mutation, and `setActiveExample` reuses that diff. There is no app-owned checkpoint cursor for those phases. The remaining scalar routes cannot be admitted until an async retained-work seam exists. No Block2d retained factory or publication contract was added.

Exact routes: `patchNodeKind`, `addHandleKind`, `removeHandleKind`, `addHandle`, `removeHandle`, `addCompatibilityRule`, `removeCompatibilityRule`, `setActiveExample`, and `edit`.

## Language-agnostic Verification

The third-party oracle was Ajv 2020 in strict mode, run with Bun against the schema-first fixture:

```text
apps: 3
routes: 27
migrated: 10
batchOnlyPendingRewrite: 17
publication: Config
```

Three adversarial variants were rejected: a migrated route without publication, a migrated route with a blocker, and a fail-closed route claiming `Artifact` publication.

An independent Bun source oracle checked all 27 unique owner/route pairs against the three Rust manifests, checked the two exact owner factories, config preparation hooks, Config publication contracts, and verified that Block2d remains factory-free. Result: 27 checked, 10 migrated, 17 fail-closed.

No Cargo, Nx, rustfmt, compiler, or task-runner command was run because the coordinator holds the exclusive compiler lease.

## Files

- `✏️s/🔌️plugins/📖️playbook/🗑️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗑️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/🗑️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `🔣️playbook-trinity-jack-block2d-retained-v1.schema.json`
- `🧪️playbook-trinity-jack-block2d-retained-v1.json`
- `📓️sol-playbook-trinity-jack-block2d-retained-cohort-2026-08-27.md`
