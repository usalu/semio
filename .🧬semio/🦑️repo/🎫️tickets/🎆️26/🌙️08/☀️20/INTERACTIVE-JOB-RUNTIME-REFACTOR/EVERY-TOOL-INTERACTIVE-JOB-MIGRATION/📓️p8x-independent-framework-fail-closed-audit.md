# P8x Independent Framework Fail-Closed Audit

## Verdict

**REJECT — P0.** The ActionBus and manifest changes remove the former broad factory
admission, but the claimed fail-closed foundation is not complete. A public
registry-less construction path still reaches direct framework history and clipboard
handlers, and the admitted typed-command path does not enforce its
`ToolExecutionContract` before command decoding. The red migration gate is therefore
not acceptable merely as a fail-closed inventory.

This was a read-only source audit. No Cargo/build/test/Wasm/cache/git/ticket-metadata
operation was run.

## Confirmed Static Improvements

- `ActionDefinition::bounded_catalog` and `CommandDefinition::bounded_catalog` inherit
  `Unclassified`, not `Migrated`: `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:877-892,1290-1305`.
- ActionBus requires a non-zero, sub-8-ms `ToolExecutionContract` when registering a
  `Migrated` factory: `🧰️framework/🔨️modules/🎯️action-bus/🦀️component.rs:310-337`.
  The contract includes raw, decoded-item, work, output, cadence, cancellation and
  freshness fields at `:69-153`.
- An alias cannot be installed until its exact target is a factory key, and unknown
  exact/alias keys return `UnknownController`: action-bus `:377-419`; its unit test is
  at `:538-544`.
- The wire-specific ActionBus route selects the exact (or registered alias) key and
  applies its raw-byte limit before factory decode: action-bus `:426-444`.
- The old `AppCommandJob`/`AppCommandJobFactory` route is absent from production source;
  the typed inner dispatch has no `run_on_worker_async` and bare command frames reject
  without inspecting their bytes: plugin `:13152-13156,13231-13236`.
- `WorkerJobSession::step` submits one worker closure per call and does not self-requeue:
  `🧰️framework/🔨️modules/🧵️job/🦀️component.rs:752-804`. The typed route calls it once,
  then calls `validate_commit` before applying presence/transient output: plugin
  `:13233-13252`.
- Direct media/config binary routes return `interactive-job.missing-factory`: plugin
  `:13290-13307`.

## P0-1 — Reserved History And Clipboard Routes Are Still Reachable

`VcsArtifactApp::new` is public and deliberately builds `AppActionRegistry::default()`:
plugin `:11479-11483`. In that state `dispatch_action` exempts interaction, history,
clipboard and related reserved identifiers from the unknown-key rejection when there is
no manifest definition: `:12896-12907`. It then directly performs the history
dispatch at `:12915-12962` and the app `copy_fragment`, `cut_operations`, and
`paste_operations` path at `:13070-13117`.

This is not dead test text: the test-support documentation calls the registry-less
wrapper a path that skips enforcement (`:6279-6283`), and the production test at
`:21098-21102` explicitly asserts registry-less construction passes a command through
unchecked. Thus a history/clipboard action can avoid the required `Unclassified`
classification and factory gate. The ledger's twelve reserved rows may be intentionally
red, but the source does not make all of them fail closed.

## P0-2 — Contract Bounds Are Not Enforced On The Admitted Typed Route

`ToolExecutionContract::validate` checks only that supplied numbers/policies are
non-zero and below the ceiling; it does not measure decoded cardinality, work, output,
checkpoint, progress, or cancellation at runtime (action-bus `:133-152`). Those
quantities are stored in `contract_by_key`, but ordinary typed dispatch never consults
them: it invokes `.dispatch(...)`, not `.dispatch_wire(...)` (plugin `:13214-13217`;
action-bus `:409-420`). `BoundedFirstStepCommandJobFactory` does not override
`create_job_from_wire` (plugin `:11268-11299`), so the wire route cannot carry these
typed commands either.

More importantly, `dispatch_command` calls `A::command_from_action` before
`dispatch_typed_command_inner` checks the exact ActionBus factory:
plugin `:13143-13148,13160-13172`. A declared `Migrated` key lacking a factory is
therefore decoded before its supposed fail-closed factory admission. The contract is
not a command-specific pre-serde envelope on this path.

The present values demonstrate the mismatch. The static proof table gives Forms and
Flow 4,096 raw bytes, while the public JSON classifier admits 16,384 Forms bytes and
8,192 Flow bytes before deserialization (plugin `:11207-11214,16849-16858`). Draw is
the inverse mismatch (contract 16,384, public cap 8,192). The more restrictive Draw
cap is safe but disproves the claimed exact measured contract; the Forms/Flow caps
permit more raw input than their registered contracts. No runtime check binds decoded
items (64/1,024), work units, output bytes, or the declared progress/checkpoint cadence
to `BoundedFirstStepCommandJob::step` (`:11239-11265`).

The cancellation policy is likewise declarative for this route: a fresh local
`JobScope::root()` token is created at `:13218-13222`, passed to the one awaited step,
and no operation-indexed cancellation handle is retained or exposed. A job checks the
token, but an outside cancellation request has no source handle to signal.

## P1 — Verifier And Ledger Scope

The current generated ledger is reproducible: a fresh
`bun ./📜️script.ts verify interactivity tool-jobs --format json`, canonicalized with
`jq -S`, exactly matched `📊️p8v-remaining-command-ledger.json`. It reports 50 macro
hosts/invocations, 775 rows (773 unique), 656 literal observations, 9 admissions,
875 remaining rows (224 macro, 651 literal), 35 global-store candidates, 12 reserved
routes, and six `ToolJobFactory` implementations. Puzzle is included as 99 remaining
literal/non-macro rows. The three aliases and all six contract implementations were
also located in source.

The machine result is nevertheless a census, not a complete proof:

- Exact proof is not owner/factory-qualified. `toolJobProofIds` returns a global set of
  string IDs (`📜️script.ts:941-956`) and admission is `proofIds.has(row.id)`
  (`:1079-1084`). A second owner that marks, for example, `canvasPointerDown` as
  `Migrated` would be admitted by the verifier without its own factory/proof. The
  current ledger happens to keep other same-named rows red, but this contradicts the
  stated exact-owner admission guarantee.
- Classification extraction only recognizes literal or directly literal-backed `&str`
  constants (`📜️script.ts:924-938`), and row extraction is regex limited to selected
  macro/builder spellings (`:901-920`). Constant aliases, non-literal expressions,
  other registration APIs, or macro expansion forms can become false negatives.
- The five self-tests cover historical macro-default, alias text, nonmacro Puzzle,
  simple data-loop, and default-builder cases (`:958-985`), but none creates a second
  owner for a proved ID, a registry-less reserved action, or an over-contract predecode
  payload. They cannot detect P0-1 or P0-2.
- The global-store detector is intentionally broad (`thread_local!` and
  `OnceLock`/`LazyLock<Mutex>` at `:1048-1053`), so its 35 candidates are neither
  proven payload authority nor granted an audited exemption. The red state is correct,
  but the verifier cannot distinguish either case.

## Commands Run

- `bun ./📜️script.ts verify interactivity tool-jobs --self-test` — exit 0,
  `self-tests=5 clean`.
- `bun ./📜️script.ts verify interactivity tool-jobs --format json` — expected exit 1;
  report emitted 35 global-store candidates, 12 reserved routes, and 875 remaining
  registrations.
- A read-only canonical JSON comparison of that report with the checked-in ledger —
  exact match.

## Unrun Mandatory Gates

No native compile/type/borrow/Send validation, Rust tests, Wasm component compile or
activation, worker cancellation delivery, watchdog timing, progress/checkpoint resume,
stale-result behavior, or maximum-input scale test was run. Those remain mandatory,
but cannot repair the two static P0 reachability/enforcement defects above.
