# P8w Global Payload Authority Repair

## Scope

Source-only repair for P8t P0-13/P0-14 and P1-02/P1-03 in the CAD, Block,
Process, Sourcing, Note, Layout, and Puzzle plugins. Framework plugin/runtime
files and the Puzzle P4/P7 solver algorithms were not intentionally changed.
The worktree already contained concurrent edits; this report describes this
cohort's semantic changes, not ownership of every dirty line under these plugin
trees.

## Changed cohorts

### Block3d

- Removed the thread-local vortex-kind catalog scratch authority.
- Persisted the vortex-kind name alongside the existing bounded extra rows and
  reconstructs the catalog directly from snapshot-owned data.
- Replaced scratch seeding with payload validation and updated mutation test
  call sites.

### Process3d

- Removed process-global contributed-machine catalog and last-contribution JSON
  slots. Catalogs are parsed from the current configuration for each operation.
- Removed stock and process-step scratch maps. The process snapshot/artifact now
  owns flattened stock handles plus the stock payload and step payload records.
- Updated handcrafted text/binary snapshot codecs, artifact conversion,
  diff/apply/absorb, and Rust/TypeScript/GraphQL/Proto/JSON descriptors for the
  same field shape.
- Updated contribution-isolation source tests and call sites so two
  configurations do not share contributed catalogs.

### Sourcing

- Removed the thread-local catalog scratch map. Object-kind extras now own the
  name and module id required to reconstruct a stock catalog.
- Removed global contributed-module and last-contribution JSON slots.
  Contributions are parsed from the invoking configuration.
- Updated snapshot/artifact/diff descriptors for the durable extra fields.
- Added pre-deserialization JSON limits to set-artifact-json: 256 KiB aggregate,
  4 KiB string leaves, and 4,096 aggregate values/items, with checked traversal
  and typed command faults instead of defaulting after decode failure.
- Updated contribution-isolation source tests and configuration/render call
  sites.

### Note

- Removed the note-text thread-local scratch map.
- Replaced the alias-only text child with a snapshot-owned record containing
  the child handle and durable paragraph payload.
- Updated import, IO, inference, mutation, diff, and test call sites to read the
  owned record.

### Layout

- Removed the background-drawing thread-local scratch map.
- Replaced the alias-only drawing child with a snapshot-owned record containing
  the child handle and durable drawing snapshot.
- Updated IO and handcrafted snapshot text/binary codecs to serialize the
  complete record.

### CAD

- Removed the thread-local preview sequence. Preview generation is now a
  deterministic fingerprint of the serialized engagement operation checkpoint,
  so it has no process identity and can be freshness-checked against the
  operation generation.
- Removed the adjacent contribution `OnceLock<Mutex<String>>` revealed by the
  exact scan. CAD computer contributions are validated per invoking
  configuration without a process-global last-payload slot.
- Updated dispatch context and command call sites.

### Puzzle3d

- Removed the global mesh registry; mesh registration remains operation/app
  owned and now rejects URL leaves above 4 KiB and position/index arrays above
  196,608 elements before registration.
- Removed global play-session maps and the atomic runtime session id.
- Play/precompute state is stack/operation owned; fill sessions restore from
  the persisted checkpoint into their owning precompute session.
- Updated configuration descriptors and isolation/configuration test sources.

### Puzzle5d

- Removed the thread-local play session; each invocation derives a local app
  from the current document/configuration.
- Removed the thread-local kind-catalog scratch map. Part, fastener, and rope
  extra rows now persist names and reconstruct directly from snapshot-owned
  data.
- Updated snapshot descriptors and mutation/editor call sites.

## Static evidence

- `rustfmt --edition 2021` completed with exit 0 for changed Rust source.
- Exact scan for every removed registry name, `runtime_session_id`, and the
  adjacent CAD contribution slot completed clean.
- Broad owned-tree global scan found no mutable `thread_local!`,
  `OnceLock<Mutex<_>>`, or `LazyLock<Mutex<_>>` payload authority in these
  repaired cohorts.
- All changed JSON descriptors in the owned Process/Sourcing/Puzzle scope parsed
  successfully with `JSON.parse`.
- `git diff --check` over the seven owned plugin trees completed with exit 0.
- Added-line debug scan found no new `[DEBUG]`, `println!`, `eprintln!`,
  `dbg!`, `console.log`, or `console.debug` sites.
- `bun ./📜️script.ts verify interactivity` completed with exit 0: deny mode
  clean over its declared four UI roots.
- `bun ./📜️script.ts verify interactivity tool-jobs --format json` completed
  with exit 1 on repository-wide pre-existing gates: 34 global-payload
  candidates outside this assigned cohort, 12 framework-reserved routes pending
  factories, and 875 live registrations pending disposition. None of its
  global-payload findings points into the seven repaired cohorts.

## Exact residual global candidates in owned trees

These are immutable fixture/descriptor/catalog caches or a clock origin, not
mutable document/app/operation authority:

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs:1019`:
  immutable forest example snapshot.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛍️set-active-example/🦀️component.rs:25-27`:
  immutable empty/concrete-forest/Nakagin example snapshots.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:99-102,297`:
  immutable example JSON, fixtures, and example-operation templates.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:80-85,294`:
  immutable example JSON/documents and example-operation templates.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️component.rs:80`:
  immutable monotonic clock origin used only to compute elapsed step budget.
- Other broad-scan matches are immutable language specifications, inference/IO
  descriptors, interaction catalogs, and example-source constants.

## Known pre-existing debug scan sites

The owned-tree debug scan still reports package-script status output, Cargo
build-script rerun output, two test-only snapshot printers, and two CAD IO error
messages containing `[DEBUG]`. The added-line scan is clean; this repair did
not add those sites.

## Gates intentionally not run

Per assignment, no Cargo command, build, unit/integration/Wasm test, runtime
launch, generated-code regeneration, cache deletion, git mutation, or ticket
metadata mutation was performed. Consequently this is source/static
audit-ready, not a compile/runtime-pass claim. Worker migration, checkpoint
restart, cancel/supersede, ABA/stale rejection, and max/+1 behavior were not
runtime-executed in this cohort.

## P8z corrective source pass

The independent P8z rejection superseded four claims above. The corrective pass
replaced the CAD content hash with a checked persisted generation stamped by the
framework's exact app/document/operation/operation-generation/base-revision
identity; replaced Puzzle3d's three-integer worker request with a bounded full
fill-worker state/checkpoint; moved Sourcing document caps before typed
deserialization; and bounded Process/Sourcing outer and nested contribution
envelopes before decode while replacing all config-derived leaked strings with
owned `String` plus self-borrowed `&str` trait results.

Exact max/+1, CAD equality/ABA/collision/restart/two-app, and Puzzle3d cold
reopen/two-operation source fixtures were added. They were not executed because
Cargo/tests remain prohibited. See
`📓️p8za-global-payload-authority-repair.md` for the corrective static evidence
and current residual gates.
