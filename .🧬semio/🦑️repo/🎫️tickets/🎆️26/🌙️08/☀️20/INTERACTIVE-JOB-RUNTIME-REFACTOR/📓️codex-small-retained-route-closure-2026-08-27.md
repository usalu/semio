# Small Retained-Route Closure

Date: 2026-08-27  
Owner packet: Codex small-app source-only executor packet  
Ticket: `2026/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`

## Result

The six assigned r13 gaps are closed and admitted by the official tool-job inventory:

| App | Route | Publication lane | Exact preparation |
| --- | --- | --- | --- |
| GIS Terrain | `setCamera` | Config | `Gis3dConfigStorePreparationFactory` |
| GIS Terrain | `setExaggeration` | Artifact | `Gis3dArtifactStorePreparationFactory` |
| GIS Terrain | `setLocale` | Config | `Gis3dConfigStorePreparationFactory` |
| Demonstrator | `changeSchema` | Artifact | `PlaygroundStorePreparationFactory` |
| Puzzle3d | `setLocale` | Config | `Puzzle3dConfigStorePreparationFactory` |
| Puzzle3d | `setTerminology` | Config | `Puzzle3dConfigStorePreparationFactory` |

The r13 baseline contained all six rows in `remainingCommands`. The generated r14 report contains zero scoped remaining rows and all six exact file/route pairs in `acceptedCommandRows`.

## Implementation

### Retained execution

- GIS Terrain and Demonstrator retain the framework's app-owned `ArtifactRetainedCommandJob`; Puzzle3d retains its app-owned `RetainedPuzzleCommandJob`.
- Each route has an exact publication contract and bounded-first-step proof.
- Puzzle3d now classifies `setLocale` and `setTerminology` as migrated, owns exact scalar work, emits exact typed Config mutations, and rejects unsupported values before retained work begins.
- GIS Terrain rejects non-object camera JSON, non-finite exaggeration, unsupported locale values, and oversized retained input.
- Demonstrator admits only its typed `ChangeSchema` mutation within the fixed schema envelope.
- Both retained shells keep cursors/checkpoints on the operation-owned job, report progress, validate replay authority, honor cancellation, close incrementally, and prove terminal emptiness.

### Store publication

Each persistent lane now has an app-local `ArtifactStoreOneItemPreparationFactory`, not a generic fake publication hook. Every factory:

- preflights only the exact mutation variants and `HistoryLane::Document`;
- validates exact operation, generation, base revision, actor, and description authority;
- bounds retained base/mutation bytes;
- computes the typed post root and inverse from the exact base root;
- yields a first `Progress` checkpoint from its operation-owned phase/candidate;
- seals exactly one typed edit through `ArtifactStoreOneItemLiveAuthority::prepare_one_item` on the second grant;
- preserves the prepared owner for Store ACK/retry transfer;
- honors cancellation and returns candidate, mutation, description, exact `SnapshotRead`, and authority one owner per close step;
- reports terminal empty only after all retained owners are returned.

Puzzle3d's Config schema gained exact `SetLocale` and `SetTerminology` diff/inverse variants. Its existing unrelated process-global object-id counter was not changed: it belongs to object/attraction creation routes outside this packet. No retained-route cursor, payload, candidate, or publication owner was placed in process-global scratch.

## Language-Neutral Contract and Oracles

- `🔣️codex-small-retained-routes-v1.json` is the strict six-route language-neutral fixture.
- `🔣️codex-small-retained-routes-v1.schema.json` is its `additionalProperties: false` Ajv schema.
- `📜️script.ts` runs Ajv plus an independent semantic/source oracle and hostile mutations covering missing contracts, wrong factories, stale generation acceptance, missing progress, weakened Store work bounds, wrong lanes, missing routes, and disabled cancellation.
- Puzzle's permanent publication-authority fixture/schema and Bun/Ajv audit now include the two Config routes, exact lane contracts, exact Store preparation ownership, progress, freshness, cancellation, incremental close, and terminal-empty witnesses.

## Verification

Passed:

1. `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📜️script.ts`
   - `validated 6 small retained routes with Ajv, an independent semantic oracle, and hostile mutations`
2. `bun ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts publication-authority-audit`
   - admitted Puzzle3d `openAddObjectDialog`, `worldPointerDown`, `setLocale`, and `setTerminology`; Ajv and independent oracle clean.
3. `bun ./📜️script.ts verify interactivity tool-jobs --self-test`
   - `self-tests=486 clean`
4. Official inventory generation:
   - `bun ./📜️script.ts verify interactivity tool-jobs --format json --output .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📊️codex-small-retained-routes-official-r14-2026-08-27.json`
   - scoped assertion: r13 remaining = 6, r14 remaining = 0, r14 accepted = 6.
   - the repository-wide report still lists 527 unrelated fail-closed command registrations and 11 aggregate failures; none names an assigned file/route pair.
5. `git diff --check` over all packet-owned source, fixture, schema, audit, and ticket files.

No Cargo, Nx, rustfmt, or browser test was run. This packet is source-only while shared stdio is incomplete, and none of the six changes alters browser-rendered UI behavior.

## Files

- GIS Terrain editor root: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- Demonstrator editor root: `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- Puzzle3d editor root and Config mutation schema: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs`
- Puzzle permanent audit: `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts`, `✏️s/🔌️plugins/🧩️puzzle/🧪️publication-authority/🔣️component.json`, `✏️s/🔌️plugins/🧩️puzzle/🧪️publication-authority/🔣️schema.json`
- Ticket fixture/oracle: `🔣️codex-small-retained-routes-v1.json`, `🔣️codex-small-retained-routes-v1.schema.json`, `📜️script.ts`
- Official report: `📊️codex-small-retained-routes-official-r14-2026-08-27.json`
