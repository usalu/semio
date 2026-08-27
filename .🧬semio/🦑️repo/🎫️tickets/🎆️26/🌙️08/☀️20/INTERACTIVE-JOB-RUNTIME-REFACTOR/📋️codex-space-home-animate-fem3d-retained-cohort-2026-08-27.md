# Space Home, Animate Present, and FEM3d Retained Command Cohort

Date: 2026-08-27. Existing ticket: INTERACTIVE-JOB-RUNTIME-REFACTOR. Source-only executor: spacehome_animate_fem3d.

## Outcome and Verification Boundary

The source now declares 16 migrated command routes and 36 explicit BatchOnlyPendingRewrite routes across 52 live app commands. This is not 100% migration. These are source-level declarations and static evidence, not a claim that Rust compilation or mounted runtime behavior has passed.

| App | Live Commands | Migrated | BatchOnly |
| --- | ---: | ---: | ---: |
| Space Home | 16 | 10 | 6 |
| Animate Present | 18 | 4 | 14 |
| FEM3d | 18 | 2 | 16 |
| Total | 52 | 16 | 36 |

No Cargo, Nx, rustfmt, compiler, or task runner was run. The coordinator owns the compiler queue. No git mutation was used. Repo MCP was unavailable to this executor; no ticket/goal lifecycle was changed.

## Ownership and Publication

Each migrated route has an exact app-owned retained factory, exact TOOL_IDS, exact PUBLICATION_CONTRACTS, an owner builder, and bounded payload admission. The shared retained command job owns wire pages, checkpoints, cancellation, progress, and terminal cleanup. Factory keys are instance-owned vectors; no process-global command-job owner was introduced. Existing global catalog/file ports remain only behind explicitly retained Space Home routes.

FEM3d's previous setAnalysisSettings migration claim was removed: a scalar reducer does not make the complete FEM artifact root clone resumable. The two genuine FEM3d migrated routes publish Config only. Home publishes eight HostOnly and two Config routes. Animate publishes two HostOnly and two Config routes. There are no new Artifact or Child claims.

## Store Preparation Envelope

The coordinator confirmed that production Store advance and close grants are exactly one item and 4096 bytes. The initial 64 KiB gate was replaced before this handoff. Each app now uses a three-phase Config preparation:

1. Validate the exact borrowed base root and construct a bounded post root plus targeted inverse.
2. Build the semantic edit and measure its actual serde_json encoding through a bounded writer that stops above 4096 bytes. Admit only when serialized edit bytes + post-root string bytes + 512 bytes of record allowance fit the complete 4096-byte envelope.
3. Ask the immutable Store authority to seal the edit and post root. The canonical hashing pass only receives an already bounded edit.

Mutation strings are capped at 512 aggregate UTF-8 bytes. Borrowed base config strings are capped at 512 aggregate bytes. Description/actor fields retain the Store's 256-byte cap and are additionally covered by the complete serialized-envelope check. Each phase requires the production 4096-byte grant, never a larger impossible grant. Cleanup releases one owned record per grant; the immutable base read is returned to its registry separately. Cancellation blocks preparation and preserves the owners for resumable close.

Larger valid config values or roots are intentionally unfinished and fail closed. This includes Home once its directory JSON exceeds 512 aggregate config bytes, large camera JSON, and text that expands beyond the complete JSON envelope. The 512-byte preflight boundary does not promise final admission for every escaped string: final canonical encoding is checked separately. Supporting those larger configurations needs chunked root and edit/hash ownership, not a raised shared grant.

The Config publication lanes use targeted inverses. Home and FEM3d no longer make a whole-config Snapshot inverse in these retained preparation owners. Store operation, generation, base revision, actor, sequence, and clock authority are checked/reused rather than invented.

## Tests and Static Evidence

A schema-first language-neutral fixture/schema pair lives beside each app editor. The strict 2020-12 JSON Schema pins every command, exact disposition, lane, blocker, and capacity. The Bun source audit used existing Ajv with strict: true and allErrors: true; all three golden fixtures passed. Twelve negative fixtures (extra property, foreign lane, wrong Store grant, false migration) were rejected. Existing fast-deep-equal independently compared extracted live commands, manifest dispositions, and publication contracts with the golden fixture output.

Static Bun result:

```json
{"status":"passed","apps":[{"app":"home","live":16,"migrated":10,"retained":6},{"app":"animate-present","live":18,"migrated":4,"retained":14},{"app":"fem3d","live":18,"migrated":2,"retained":16}],"strictAjv":true,"negativeSchemaCases":12,"storeGrant":4096}
```

Six Rust tests were added but not executed: each app has a fixture/serde_json round-trip/preflight boundary test and a cancellation/4096-byte cleanup/bounded-writer frontier test. The third-party serde_json oracle uses existing dependencies; no new external runtime dependency was introduced. Space's Cargo manifest gained the existing in-repo semio-framework-job workspace dependency needed by its retained factory types (the neighboring Space engine already referenced the same crate).

Scoped git diff --check passed with exit code 0 for the three Rust editor owners and the Space Cargo manifest. JSON schema parsing/validation also covered the six new fixture files. Official retained-route verifier, Rust compilation/tests, mounted runtime logs, and actual host-effect publication remain coordinator work.

## Exact Routes and Blockers

### home

| Command | Disposition | Lane / Exact Blocker |
| --- | --- | --- |
| createStudio | BatchOnlyPendingRewrite | Creates and resolves studio catalog/draft/backbone owners through process-global ports without resumable external I/O. |
| bindSpaceFile | BatchOnlyPendingRewrite | Binds a file-backed studio through blocking file/backbone work and process-global port ownership. |
| importSpace | BatchOnlyPendingRewrite | Mixed request/import branches parse a whole document and publish external storage without retained decoding or cancellation. |
| openSpace | Migrated | HostOnly |
| navigateVirtualFileSystemNode | Migrated | HostOnly |
| deleteVirtualFileSystemNode | BatchOnlyPendingRewrite | Traverses global catalog owners and performs external deletion without an app-owned resumable cursor. |
| goHome | Migrated | HostOnly |
| setActivePanelTab | Migrated | Config |
| createSpace | Migrated | HostOnly |
| deleteSpace | Migrated | HostOnly |
| renameSpace | BatchOnlyPendingRewrite | The dialog branch decodes and searches the whole directory read model; the shared command route has no bounded cursor. |
| shareSpace | Migrated | HostOnly |
| copyInviteLink | Migrated | HostOnly |
| foldDirectoryEvents | BatchOnlyPendingRewrite | Parses the complete event JSON array and repeatedly clones/folds/serializes the directory model in one dispatch. |
| presenceHeartbeat | Migrated | HostOnly |
| setClient | Migrated | Config |

### animate-present

| Command | Disposition | Lane / Exact Blocker |
| --- | --- | --- |
| seedGrid | BatchOnlyPendingRewrite | Grid dimensions drive whole tile-vector generation and artifact inverse/root preparation without a checkpointed tile cursor. |
| addTile | BatchOnlyPendingRewrite | Working-scene extraction and artifact inverse/root preparation clone tile content without a retained cursor. |
| deleteTile | BatchOnlyPendingRewrite | Tile lookup and artifact inverse/root preparation scan or clone whole tile vectors. |
| deleteSelection | BatchOnlyPendingRewrite | Selection validation and per-tile deletion scan the whole scene and assemble an unbounded mutation batch. |
| renameTiles | BatchOnlyPendingRewrite | Selection validation and rename output scan tile vectors and assemble an unbounded mutation batch. |
| patchTileCrops | BatchOnlyPendingRewrite | Patch parsing, tile validation, and batch construction lack per-tile retained cursors. |
| setSource | BatchOnlyPendingRewrite | The route may generate a replacement tile grid and requires unbounded artifact inverse/root preparation. |
| setFrame | BatchOnlyPendingRewrite | The reducer is scalar but artifact publication clones the complete presentation root; no bounded root-scalar preparation exists. |
| setActiveExample | Migrated | HostOnly |
| clearTiles | BatchOnlyPendingRewrite | Replacing the tile list requires a whole-list inverse and root clone without a retained preparation cursor. |
| engagementSubmit | BatchOnlyPendingRewrite | Mixed keyword branches scan, generate, rename, delete, or serialize the complete tile scene in one dispatch. |
| resetGrid | BatchOnlyPendingRewrite | Builds a complete default tile grid and requires whole-list inverse/root preparation. |
| engagementInput | Migrated | Config |
| canvasPointerDown | BatchOnlyPendingRewrite | Hit testing extracts and scans the whole tile scene before publishing selection effects. |
| setLocale | Migrated | Config |
| noMutation | Migrated | HostOnly |
| copyPrompt | BatchOnlyPendingRewrite | Builds a complete tile-morph prompt by extracting and serializing all tile content in one dispatch. |
| exportVideoFromDeck | BatchOnlyPendingRewrite | Headless video rendering and whole-output export do not expose a resumable frame/encoder owner to the command job. |

### fem3d

| Command | Disposition | Lane / Exact Blocker |
| --- | --- | --- |
| addNode | BatchOnlyPendingRewrite | Fresh-ID discovery scans nodes and artifact publication clones the whole FEM model. |
| addBar | BatchOnlyPendingRewrite | Node/material/section validation and fresh-ID discovery scan model collections before whole-root preparation. |
| addFrame | BatchOnlyPendingRewrite | Node/material/section validation and fresh-ID discovery scan model collections before whole-root preparation. |
| addMaterial | BatchOnlyPendingRewrite | Fresh-ID discovery scans materials and artifact publication clones the whole FEM model. |
| addSection | BatchOnlyPendingRewrite | Fresh-ID discovery scans sections and artifact publication clones the whole FEM model. |
| addSupport | BatchOnlyPendingRewrite | Node/support lookup scans model collections and whole-root preparation is not resumable. |
| addNodalLoad | BatchOnlyPendingRewrite | Load-case and load lookup clone/scan collections before whole-root artifact preparation. |
| addMemberUdl | BatchOnlyPendingRewrite | Member/load-case lookup clones or scans collections before whole-root artifact preparation. |
| addAreaLoad | BatchOnlyPendingRewrite | Area/load-case lookup clones or scans collections before whole-root artifact preparation. |
| addSolid | BatchOnlyPendingRewrite | Solid construction and fresh-ID discovery scan/allocate model collections before whole-root preparation. |
| addLoadCase | BatchOnlyPendingRewrite | Fresh-ID discovery scans load cases and artifact publication clones the whole FEM model. |
| addCombination | BatchOnlyPendingRewrite | Combination term parsing and ID discovery scan/allocate collections before whole-root preparation. |
| setSelfWeight | BatchOnlyPendingRewrite | Load-case lookup scans the case collection and artifact publication clones the whole FEM model. |
| setAnalysisSettings | BatchOnlyPendingRewrite | The reducer is scalar but artifact publication clones the complete FEM model; no bounded root-scalar preparation exists. |
| removeSelection | BatchOnlyPendingRewrite | Nested selection/model scans assemble an unbounded removal batch and whole-model inverse/root state. |
| setActiveExample | BatchOnlyPendingRewrite | Builds and serializes an entire replacement FEM document into a host effect without a retained encoder owner. |
| setCamera | Migrated | Config |
| setResultDisplay | Migrated | Config |

## Files

- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🎯️retained-command-limits.schema.json`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🎯️retained-command-limits.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🎯️retained-command-limits.schema.json`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🎯️retained-command-limits.json`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🎯️retained-command-limits.schema.json`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🎯️retained-command-limits.json`
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml`

Reserved media import/export hooks were not credited as migrated commands. They retain their existing fail-closed framework dispositions and are outside the 52 command-row count above.

