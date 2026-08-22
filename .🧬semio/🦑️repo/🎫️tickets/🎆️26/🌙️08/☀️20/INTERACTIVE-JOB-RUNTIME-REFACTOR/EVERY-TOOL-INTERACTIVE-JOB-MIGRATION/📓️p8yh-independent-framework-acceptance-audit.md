# P8yh Independent Framework Acceptance Audit

## Verdict

**PASS — corrected fail-closed foundation accepted on bounded source/static evidence.**

This accepts the framework repair only. The repository-wide migration remains intentionally
fail-closed: the machine ledger still has exactly 875 unproved command registrations, 34
process-global-store candidates, and 12 framework-reserved routes. Those are hard failures for
their owning migration packets, not admissions by this foundation.

This audit was read-only except for this ticket record. No Cargo, native runtime/test, Wasm,
cache, git mutation, or ticket-metadata operation was run.

## Prior P0 Reattack

### Qualified owner and factory authority — PASS

- `ToolOwnerWitness` has private fields and is constructed from both
  `TypeId::of::<A>()` and `type_name::<A>()` at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:11217-11231`.
  Production code cannot construct it from copied controller/schema constants.
- Every admitted proof carries owner-file, compiler type name, controller, factory, tool,
  schema, and contract. `bounded_first_step_proof::<A>` selects its row through the compiler
  witness; factory construction and ActionBus admission subsequently compare the complete witness
  and the concrete generic `BoundedFirstStepCommandJobFactory<A>` `TypeId` and type name
  (`:11233-11259`, `:11346-11351`, `:11608-11611`, `:11823-11846`).
- The concrete `CopyDrawApp` regression copies Draw's static controller, schema, command id, and
  constants yet obtains no registration, proof, public raw limit, or inherited factory
  (`:20113-20249`). Its distinct Rust type is decisive rather than any copyable application
  constant.

### Exact public ActionBus boundary — PASS

- `ActionBus::dispatch` and `ActionBus::dispatch_wire` both select only
  `factory_by_key.get(&key)` (`🧰️framework/🔨️modules/🎯️action-bus/🦀️component.rs:436-471`).
  Neither public execution route reads `aliases`; alias metadata remains non-dispatching.
- `admit_exact_wire` applies exact key, raw-byte, schema, and factory identity checks before a
  caller-specific decoder (`:418-434`). The ActionBus regression rejects aliases for typed,
  raw-wire, and pre-decode admission (`:561-587`).

### Registry-less reserved actions — PASS

`VcsArtifactApp::dispatch_action` rejects an unknown registry key, enforces the manifest
classification, then rejects history, clipboard, and revert with
`interactive-job.missing-factory` before any legacy direct branch (`plugin:13278-13284`). Thus
empty registries cannot reach the later history/clipboard code (`:13292-13494`). Bare command
frames also reject before deserialization (`:13536-13540`).

### Predecode contract, measured job, freshness, and cancellation — PASS

- JSON actions, manifest commands, intents, and direct typed values obtain an exact qualified proof
and raw admission before `command_from_action` or `command_from_intent`
(`plugin:11823-11859`, `:13495-13531`, `:13806-13816`).
- The bounded worker measures decoded cardinality, work units, output bytes, and elapsed time
against the exact contract (`:11556-11598`). It rejects asynchronous task output that cannot be
bounded (`:11494-11530`).
- One `WorkerJobSession::step` is scheduled (`:13612-13617`); the retained cancellation lease is
keyed by live `ActionMeta.instance_id`, parent document, operation, revision, and generation
(`:11364-11455`, `:13597-13599`). Cancellation is checked before output exposure, followed by
`validate_commit` freshness validation (`:13619-13632`). The two-live-instance regression cancels
instance 41 without affecting instance 42 (`:20251-20273`).

## Verifier And Ledger Evidence

The current verifier is owner-file/type/controller/factory/tool/schema-qualified and explicitly
checks compiler witness presence, exact typed/raw dispatch, predecode ordering, reserved-route
closure, public-contract equality, factory identity, live-instance cancellation, and measured
worker dimensions (`📜️script.ts:941-1206`). Its suite contains 16 semantic false-positive
fixtures, including copied-owner, same-id/different-owner, registry-less reserved-route,
predecode, contract mismatch, cancellation, and typed/raw alias cases (`:1048-1105`).

Commands run:

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | Exit 0; `self-tests=16 clean` |
| `bun ./📜️script.ts verify interactivity` | Exit 0; deny-mode clean |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json` | Expected exit 1; stdout was byte-identical to `📊️p8ye-current-command-ledger.json` |
| `git diff --check` over ActionBus, plugin, manifest, and root script | Exit 0 |

The byte-identical fresh ledger reports: 50 macro hosts/invocations, 775 macro rows (773 unique),
656 literal registrations, nine bounded admissions, six explicit factory contracts, one factory
registration and one typed production dispatch site, four non-dispatching aliases, 16 self-tests,
875 remaining commands, 34 global candidates, and 12 reserved routes. Its three failures are
exactly those last three broad backlog classes.

`📊️p8ye-current-command-ledger.json` and `📊️p8ye-canonical-diff-check.json` are byte-identical
SHA-256 snapshots (`b848008ed94a1d39dab7eaab879954c339e78c06b8d917aa49ca500fc5d4a204`). The
earlier P8v ledger is correctly different because it precedes the repairs. I also fully reviewed
P8v, P8x, P8y, P8yb, P8yc, P8yd, and P8ye against the current sources.

## Mandatory Unrun Gates

This acceptance does not establish native type/borrow/Send correctness, Rust test success, Wasm
component compilation or activation, actual external cancellation delivery, 8-ms watchdog timing,
checkpoint/progress/resume, stale-result behavior under concurrency, or declared-maximum scale
behavior. Registry-less legacy test fixtures that intentionally exercised direct command/history/
clipboard behavior must be converted by their owning test packets before a full native test gate can
be claimed; production source is fail-closed at the relevant public boundary.
