# P8zb Independent Global Payload-Authority Final Audit

## Verdict

**REJECT — P0.** The P8za source repair resolves the three P8z primary
implementation failures in the native Rust path: the CAD stamp is a persisted
checked counter qualified by the framework operation identity; Puzzle3d carries
the state required to rebuild a fill session in an isolated worker; and Sourcing
scans its document and contribution envelopes before typed deserialization.

Two final-audit blockers remain. The persisted CAD generation is not lossless
across the declared schema surfaces, and Note/Layout retain comments and test
names that promise the deleted working-scene-cache lifecycle. The former breaks
the claimed exact checked-monotonic stamp at legal `u64` values, so it is P0; the
latter is P1 documentation-contract drift explicitly required by this packet.

No production or ticket metadata was changed by this audit.

## Audit Basis

Read in full before source inspection:

- `AGENTS.md`.
- `📓️p8t-independent-remaining-tools-global-audit.md`.
- `📓️p8w-global-payload-authority-repair.md`.
- `📓️p8z-independent-global-payload-authority-audit.md`.
- `📓️p8za-global-payload-authority-repair.md`.

The read-only review then re-attacked CAD, Block3d, Process3d, Sourcing, Note,
Layout, Puzzle3d, and Puzzle5d source and their current descriptor surfaces.

## Blocking Findings

| ID | Finding | Exact current evidence | Required repair |
| --- | --- | --- | --- |
| P0-01 | CAD's persisted `u64` generation has no lossless descriptor contract. A generation may pass Rust's `checked_add` then truncate/reject in GraphQL or lose integer precision in TypeScript, making restart, equality, ABA, and freshness semantics incoherent across the app's declared surfaces. | Rust increments a `u64` with only overflow protection at `…/📐cad/…/✏️editor/🦀️component.rs:354-363`; the stamp remains `u64` at `:780-796`; Proto is `uint64`. But GraphQL exposes `engagementPreviewGeneration: Int!` at `…/🎚️config/🧬️schema/🔗️component.graphql:58`, and TypeScript exposes `number` at `…/🟦️component.ts:78`. No range cap before persistence or serialisation exists. | Use one lossless cross-language representation for the checked counter (for example a decimal string/owned scalar supported by all descriptors), or impose and enforce a documented bound representable by GraphQL `Int` and JavaScript before every persisted increment. Keep Rust, TypeScript, GraphQL, Proto, JSON and conversion code in exact agreement; add max/+1 descriptor-round-trip fixtures. |
| P1-01 | Note and Layout comments/tests still describe working-scene scratch-cache authority although the values are snapshot-owned records. This contradicts the new lifecycle and the P8za claim that these comments were corrected. | Note's real accessor is durable at `…/🗒️note/🦀️component.rs:342-349`, but `:452-467` calls it a "Working-scene cache" and an "uncached handle" staleness gap. Layout's consumer comments call `background_drawing_content` a working-scene scratch cache at `…/📏layout/…/🚪️io/🦀️component.rs:267-272`, `:367-372`, and `:473-493`, despite the accessor reading `LayoutDrawingChild.content` from `LayoutSnapshot`. The Note schema mutation at `…/🗒️note/…/🧬️schema/🦀️component.rs:408` also says it reads the old cache. | Rewrite comments and test identifiers/assertion text to state that the child records own durable snapshot content. Remove the obsolete cache-miss/staleness vocabulary and any dead region naming that promises a cache. |

## Re-Attack Results That Hold At Source Level

### CAD

- `CadPreviewOperationIdentity` contains app instance, parent document,
  operation id, operation generation, and canonical base revision
  (`…/📐cad/…/✏️editor/🦀️component.rs:760-776`).
- `preview_snapshot_of` stamps that identity and advances the persisted counter
  only if the engagement checkpoint changes (`:354-363`); no content hash or
  thread-local sequence remains.
- Freshness requires identity equality plus a strictly greater generation
  (`:794-797`), and `gesture_preview` reconstructs the persisted stamp from
  configuration (`:960-969`). The source fixtures cover repeat reads, A→B→A,
  cold config serialization, and cross-app counter collision. They were not run.

### Puzzle3d

- `FillWorkerState` owns the fill job, scene, mesh sources, fill checkpoint,
  remaining-step cursor, revision/generation/preview sequence, observation and
  last emitted checkpoint (`…/🧩puzzle/…/⏳precompute/🦀️component.rs:62-82`).
- Admission captures that full state (`:822-849`). Restore enforces byte, URL,
  mesh-count, individual/aggregate mesh-cardinality caps, rebuilds collision
  meshes and scene, reconstructs/configures `FillBuilder`, restores its
  checkpoint, and rejects operation/generation mismatch before returning a
  session (`:851-892`).
- The worker restores admission/checkpoint state and fails closed when a restored
  request differs before it drives a slice (`:1001-1032`). The cold-reopen and
  two-operation/ABA fixtures exist in the same source and were not run.
- This path does not use the deleted mesh/session registries. `OnceLock<Instant>`
  in the precompute module is only a clock origin, not payload authority.

### Sourcing And Contributions

- `set-artifact-json` calls the raw byte/depth/string/cardinality scanner before
  `serde_json::from_str::<CurateSnapshot>` (`…/🪵sourcing/…/set-artifact-json/🦀️component.rs:18-25`).
- The scanner performs checked raw bounds without creating `serde_json::Value`
  (`…/🧬schema/🦀️component.rs:576-647`). Both outer contributions and nested
  typology/kinds payloads are scanned before `parse_contributions`, topic
  decoding, or typed JSON decode (`:649-675`). Process has the corresponding
  outer/nested guard before contribution and machine decoding.
- Contributed Process/Sourcing catalog strings are now owned `String` and the
  traits return self-borrowed `&str`; no `Box::leak`, `into_boxed_str`, or
  `leak_str` remained in the seven repaired cohorts.

### Other Payload Authority And Descriptors

- Block3d persists `vortex_kind_extra`; Process snapshot/diff carry
  `stock_payload` and `step_payloads`; Sourcing extras include durable identity
  fields. The inspected text/binary/proto/JSON surfaces agree at source level.
- The broad global scan found no mutable `thread_local!`, `OnceLock<Mutex<_>>`,
  or `LazyLock<Mutex<_>>` payload authority in CAD, Block, Process, Sourcing,
  Note, Layout, or Puzzle. Remaining `OnceLock`/`LazyLock` matches in these
  trees are immutable examples, descriptors/catalogues, or the Puzzle3d clock
  origin.

## Static Commands

All commands were read-only/static.

```text
bun ./📜️script.ts verify interactivity
=> exit 0; deny-mode clean in its declared four UI roots.

bun ./📜️script.ts verify interactivity tool-jobs --format json
=> exit 1; 9 admitted rows, 34 global candidates outside this cohort,
   12 reserved framework routes pending factories, and 875 live registrations
   pending disposition. This global P8 gate remains red and is not evidence
   against the repaired cohort specifically.

JSON.parse each of 9 changed CAD/Process/Sourcing/Puzzle JSON descriptors
=> all 9 parsed.

git diff --check -- <seven repaired plugin trees>
=> exit 0.
```

## Unrun Mandatory Gates

No Cargo command, compilation/type/borrow/Send gate, build, test execution,
native or release runtime, actual worker launch, Wasm build/execution, descriptor
discovery, generated-code regeneration, cache deletion, git mutation, or ticket
metadata operation was performed.

After both findings are repaired, the cohort still needs native/release/Wasm
compilation and execution of CAD equality/ABA/collision/restart/two-app tests;
Puzzle3d first tick, checkpoint, cold restart, cancellation and two-operation
isolation; Sourcing/Process exact max/+1 envelopes before decode; and descriptor
round-trip/discovery gates. The repository-wide fail-closed P8 command ledger
also remains independently mandatory.
