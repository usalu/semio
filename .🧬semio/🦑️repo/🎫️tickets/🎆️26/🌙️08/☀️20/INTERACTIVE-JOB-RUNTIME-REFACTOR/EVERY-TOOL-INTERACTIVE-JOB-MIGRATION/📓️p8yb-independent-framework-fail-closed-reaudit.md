# P8yb Independent Framework Fail-Closed Reaudit

## Verdict

**REJECT — P0.** The repair closes the P8x registry-less history/clipboard bypass and adds useful
contract/cancellation enforcement, but production activation does not preserve the proof table's
owner identity. A different owner can inherit a bounded-first-step proof by sharing its document
schema and command id. The stated owner+factory+tool+schema admission invariant is therefore not
true on the production route.

This was a bounded read-only source/static audit. No production source was edited, and no Cargo,
native runtime, Wasm, cache, or ticket-metadata operation was run.

## P0 — Runtime Proof Admission Drops `owner_file`

The nine proof rows are owner-qualified data: each has `owner_file`, `factory`, `tool_id`, and
`document_schema` at
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:11203-11275`.
The static verifier correctly keys its proof identity by all four fields in
`📜️script.ts:946-956,1005,1150-1154`, and its synthetic same-id/different-owner fixture is clean.

The actual activation path does not use that identity:

- `bounded_first_step_contract(document_schema, id)` at plugin `:11277-11279` matches
  `document_schema`, `tool_id`, and the generic factory name; `owner_file` is only checked for being
  non-empty. It is never compared with the caller's source/owner/controller identity.
- `AppActionRegistry::tool_job_registration` at `:10889-10897` takes only `app_id`,
  `document_schema`, and generated ids. For every migrated generated id it calls that unqualified
  lookup, then constructs a factory key from the arbitrary registry `controller_id`.
- `with_registry_on_bus` registers the resulting generic
  `BoundedFirstStepCommandJobFactory<A>` at `:11800-11804`.
- The action and intent entry paths repeat the same schema/id-only lookup at `:11761-11766` and
  `:13712-13720`. The public DFF pre-deserialize classifier is weaker still:
  `bounded_first_step_public_wire_limit(id)` at `:11281-11283` accepts only an id.

Consequently, a second `ArtifactApp`/registry can declare a distinct controller, retain (or
accidentally reuse) `DOCUMENT_SCHEMA = "draw.document"`, declare generated
`canvasPointerDown` as `Migrated`, and be registered through the generic factory despite not owning
the Draw proof's source row. Its ActionBus key remains a different controller/tool pair, but the
authorization to create that key came from the first owner's proof. The ActionBus's exact
controller/tool/schema check occurs only after this incorrect registration; it cannot recover the
lost owner authority.

Required repair: make runtime proof lookup and factory construction consume one opaque qualified
identity containing controller/owner, factory, tool id, and document schema; reject any registry
whose exact identity is not in the proof table. Use that same identity for the external public raw
envelope classifier. Add a real regression fixture with two distinct controllers sharing the same
schema/id and prove that only the owning controller receives a factory.

## Confirmed Repairs

- `VcsArtifactApp::new` creates an empty registry, and `dispatch_action` first rejects an unknown
  action, validates classification, then returns `interactive-job.missing-factory` for history,
  clipboard, and revert before the legacy direct branches at plugin `:13189-13193`. The later
  direct branches at `:13201`, `:13249`, and `:13356` are unreachable from that public route while
  reserved factories are absent.
- Exact ActionBus admission does resolve exact controller/tool/schema and enforces the registered
  raw limit before caller decoding; aliases are not accepted by `admit_exact_wire`:
  `🧰️framework/🔨️modules/🎯️action-bus/🦀️component.rs:417-433`.
- Action, manifest-command, and intent app decoders follow admission on their respective paths:
  plugin `:13405-13407`, `:13434-13437`, and `:13716-13721`. A bare command frame stays rejected.
- The worker job checks decoded count, work units, output measurement, and its sub-8-ms elapsed
  contract in its one step: plugin `:11477-11518`; typed dispatch uses one
  `WorkerJobSession::step`, checks watchdog overrun, cancellation, then `validate_commit` before
  presence/transient output exposure: `:13502-13544`.
- `ToolCancellationHandle` retains operation keys containing app instance, parent document,
  operation, base revision, and generation; it supports exact/document/all cancellation and
  supersession; the app exposes a clone and drop cancels all: plugin `:11286-11367,11734-11735,
  13506-13508,13617-13620`.
- Both manifest `bounded_catalog` builders remain `Unclassified`, not self-certifying
  `Migrated`: `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:888-892,1300-1305`.

These improvements do not remove the P0 proof-authority leak above.

## Ledger And Static Gates

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | Exit 0: `self-tests=10 clean` |
| `bun ./📜️script.ts verify interactivity` | Exit 0: deny mode clean |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json` | Expected exit 1: 875 remaining command rows, 34 global candidates, 12 reserved routes |
| Canonical generated ledger vs `📊️p8y-current-command-ledger.json` | **Mismatch**: source reports `aliases: 4`, ledger has `3`; two factory line offsets changed (`NumberFactory` 577→592; bounded factory 11530→11529). All other compared fields match. |
| `git diff --check` over ActionBus, plugin, manifest, and `📜️script.ts` | Exit 0 |

The ledger's broad account is still 775 macro rows (773 unique), 656 literal registrations, 9
bounded admissions, 6 explicit factories, 875 remaining registrations, 34 global candidates, and
12 reserved routes. Its checked-in provenance snapshot must be regenerated after the concurrent
source edits; it is not an exact current ledger.

## Mandatory Unrun Gates

Native type/borrow/Send compilation, Rust tests, Wasm component compilation/activation, actual
cancellation delivery, watchdog timing, checkpoint/progress resumption, stale-result behavior, and
maximum-input/output scale tests remain unrun. They cannot establish the missing runtime proof
authority without the P0 repair first.
