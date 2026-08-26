# Process3d Retained Command Honesty

Date: 2026-08-26  
Owner boundary: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**`.  
Validation constraint: source and non-Cargo static gates only while the coordinator serializes Cargo/Nx.

## Audit result

The prior implementation registered all 33 migrated routes through one `BoundedFirstStepCommandJobFactory`, assigned every route the same resumable contract, and returned `Some(1)` for every extent. That was not truthful for document encoding, machine/capability scans and clones, JSON parsing/serialization, config text, interaction selection, or media conversion.

The repaired split is exact:

- Bounded first step, 7 routes: `setCursor`, `stepCursor`, `stepCursorBack`, `stepCursorForward`, `engagementAbort`, `setCamera`, `loadModelRequest`.
- Resumable, 26 routes:

| Cursor | Routes |
|---|---|
| Document | `setSnapshot`, `setActiveExample`, `setStock` |
| Workshop | `addStep`, `addWorkshopMachine`, `removeWorkshopMachine`, `updateWorkshopMachine`, `worldPointerDown`, `worldFaceDragEnd` |
| Step | `removeStep`, `removeSelectedStep`, `moveStep`, `updateStep`, `setStepEnabled` |
| Inspector | `patchInspector` |
| Config | `engagementSubmit`, `setActiveUtility`, `engagementInput`, `toggleSun`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity`, `setLocale`, `setContributions` |
| Media | `importModelFile`, `exportModel` |

`Process3dBoundedCommandJobFactory` and `Process3dResumableCommandJobFactory` register disjoint exact key sets. The 33 proof rows now state the real per-route contract. There is no fallback, compatibility wrapper, or restored blanket factory.

## Resumable ownership

The custom `Process3dResumableCommandWork` admits at most 8,192 raw bytes and 64 semantic work items. Extents are checked sums of only the state each command uses:

- payload strings in 256-byte units, including JSON/config/media text;
- workshop machines, capabilities, parameters, and rules;
- document step/tool collections and stock identity where document/media commands consume them;
- live config text used by catalog, engagement, world, and sun commands;
- framework-owned selection identifiers for `removeSelectedStep`.

One input unit is observed per step before the original reducer runs. The final reducer retains the original typed-command semantics. Exact 64-item admission succeeds and 65-item admission fails.

The fixed 40-byte `P3C1` checkpoint owns disposition, completion bit, cursor, digest, stable tool identity, and declared extent. Restore rejects malformed records, another tool in the same category, and extent mismatch. Step rejects cursor/extent drift. Progress and checkpoint cadence are every step; cancellation remains per operation; close is blocked until `begin_close` and then becomes terminal without retaining a private heap buffer.

## TDD coverage

The editor-local tests cover:

- exact 7/26 disposition census over all 33 routes;
- bounded/resumable contract shapes, per-operation cancellation, and one-step progress/checkpoint cadence;
- exact maximum and maximum-plus-one admission;
- progress, checkpoint, same-category cross-tool rejection, replay equivalence, bounded close, and terminal emptiness;
- all seven bounded reducers and maximum admitted representatives for document, workshop, step, inspector, config, and media cursors below 8 milliseconds per step.

## Validation ledger

| Gate | Result |
|---|---|
| Direct `rustfmt --edition 2021 --check` | PASS, exit 0 |
| Scoped `git diff --check` | PASS, exit 0 |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS, exit 0, `self-tests=464 clean` |
| Full JSON tool-job verifier | Expected repository-wide exit 1; Process3d accepted `33`, factory contracts `2`, remaining `0` |
| Cargo/Nx/Wasm | Not started; compiler slot remains coordinator-owned |

The full static report is `📊️process3d-tool-jobs-live-2026-08-26.json`. Repository totals at this checkpoint are `commandRows=774`, `boundedRows=194`, `forbiddenRows=1`, `remaining=744`, `productionFactories=24`, and `productionRegistrations=199`. The only three failures are repository-global aggregate deficits; none names Process3d.
