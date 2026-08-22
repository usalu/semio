# P8yd Independent Framework Final Audit

## Verdict

**REJECT — P0.** The P8x registry-less reserved-route bypass, pre-decode admission ordering,
bounded-worker measurement, stale-result check, and externally retained cancellation have improved.
However, runtime does not enforce the claimed controller + owner-file + factory + tool + schema
proof authority, and public `ActionBus` dispatch still accepts aliases.

This was a bounded read-only source/static audit. No production source was edited and no Cargo,
native runtime, Rust test, Wasm, cache, git mutation, or ticket-metadata operation was run.

## P0-1 — Runtime Never Receives Or Compares The Owner File

`BoundedFirstStepProof` stores `owner_file`, but no runtime proof authority function accepts it.
`AppActionRegistry::tool_job_registration` takes only expected controller, schema, and generated ids
and calls the four-argument proof lookup (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:10889-10896`).
`bounded_first_step_proof(controller_id, factory, tool_id, document_schema)` checks those four
values and only asserts `!proof.owner_file.is_empty()` (`:11309-11313`). The typed path repeats that
four-field lookup (`:11784-11806`), while factory construction consumes its result (`:11854-11856`).
The public wire-limit lookup has only controller/tool input (`:11316-11322`).

A copied `ArtifactApp` can therefore use the Draw controller constant, Draw schema, and
`canvasPointerDown` generated id with a `Migrated` registry declaration. It passes registry equality,
obtains the Draw proof, and builds `BoundedFirstStepCommandJobFactory<CopyDrawApp>` from it. The
type-name comparison then succeeds for the copied generic factory; no runtime value distinguishes the
proof owner file. This violates the required copied-controller/same-schema/id non-inheritance rule.

It also conflicts with the framework's own ownership rule: `ArtifactApp` states ownership checks
must use runtime `instance_id()`, not `APP_ID`, for surface wrappers (`:10169-10182`). Activation
instead passes `A::APP_ID` to tool registration (`:11854`), which can bind job registration to a
placeholder instead of the real app identity.

The named regression only varies `expected_controller_id`; it does not make a second app/type with
the same controller and schema, nor provide an owner witness (`:20118-20143`). The verifier is also
blind: it confirms source-table `ownerFile`, but its runtime checker explicitly recognizes the
four-argument lookup and `A::APP_ID` (`📜️script.ts:1152-1159`).

## P0-2 — Public ActionBus Dispatch Still Falls Back Through Aliases

`admit_exact_wire` uses only the exact key (`🧰️framework/🔨️modules/🎯️action-bus/🦀️component.rs:417-433`),
but both other public dispatch APIs resolve `inner.aliases.get(&key)`: typed `dispatch`
(`:435-445`) and raw `dispatch_wire` (`:452-472`). The latter then invokes the factory decoder,
without `admit_exact_wire` ever running. The canonical ledger records four aliases. This is a public
alias-fallback path contrary to the required exact-key boundary, even if the present bounded factory
does not implement wire decoding.

## Confirmed Improvements

- Empty registries reject unknown actions before history/clipboard handling; reserved routes return
  `interactive-job.missing-factory` first (`plugin:13239-13245`).
- JSON action and manifest-command paths admit before `command_from_action` (`plugin:13456-13459,
  13483-13491`); bare command frames reject before byte inspection (`:13497-13500`); intent admits
  before `command_from_intent`.
- The bounded worker measures decoded/work/output/elapsed limits. Dispatch uses one
  `WorkerJobSession::step`, retains a keyed cancellation lease, and validates freshness before
  presence/transient exposure (`plugin:13545-13592`).
- `bounded_catalog` is not a self-certifying `Migrated` disposition and all six discovered factories
  declare explicit contracts.

## Static Gates Run

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | Exit 0; `self-tests=12 clean` |
| Fresh JSON verifier piped directly into `cmp` with `📊️p8y-current-command-ledger.json` | Byte-identical; its expected rejection listed only the migration backlog |
| `bun ./📜️script.ts verify interactivity` | Exit 0; deny mode clean |
| `git diff --check` over ActionBus, manifest, plugin, and verifier | Exit 0 |
| Full canonical parse of the P8y ledger | Exit 0 |

The fully parsed canonical ledger reports 50 macro hosts/invocations, 775 rows (773 unique), 656
literal registrations, 9 bounded admissions, 6 explicit factories, 4 aliases, 875 remaining
commands, 34 global payload-store candidates, and 12 framework-reserved routes. The three broad
backlog classes are its only reported failures, but do not include the two P0 defects above.

## Required Repair And Reaudit

1. Make runtime proof selection, registration, factory construction, and public prescan consume a
   sealed owner witness with controller, factory, tool, and schema; use `app.instance_id()` for
   runtime identity. Add a copied-app/same-controller/same-schema/id rejection before factory
   registration and public-limit admission.
2. Remove alias resolution from every public typed/wire dispatch path, or make aliases incapable of
   selecting a factory. Add typed and wire alias-bypass regressions.
3. Strengthen the verifier to reject proof lookup without the owner witness, and include copied
   controller and direct `dispatch_wire` alias fixtures. Regenerate/byte-compare the ledger, then
   repeat this independent audit.

## Mandatory Unrun Gates

Native type/borrow/Send compilation, Rust tests, Wasm component compile/activation, actual worker
cancellation delivery, 8-ms watchdog behavior, checkpoint/progress/resume, stale-result behavior,
and maximum-input/output scale runs remain unrun. They cannot establish owner proof authority or
eliminate public alias dispatch.
