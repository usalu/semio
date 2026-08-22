# P8ye Framework Owner-Witness And Alias Repair

## Disposition

**The two P8yd P0 findings are repaired in source and bounded static gates.** A copied app type can
no longer inherit an audited job proof, activation and cancellation distinguish static ownership from
the live instance, and public typed/raw-wire dispatch cannot resolve an alias.

No Cargo, native compilation, Rust test execution, runtime activation, Wasm build, cache mutation,
modifying git command, or ticket API was run.

## Repair

- `ToolOwnerWitness` is sealed by private fields and constructed from compiler-derived
  `TypeId::of::<A>()` plus `type_name::<A>()`. Each audited proof row also records the exact static
  owner type name. Generic proof lookup, `AppActionRegistry::tool_job_registration::<A>`, factory
  construction, JSON/UI-intent admission, and public DFF raw-limit selection all require that witness
  together with the exact controller, factory, tool, and document schema.
- `ActionBus` records the concrete factory `TypeId` and type name beside each exact key. Exact wire
  admission returns both, and typed admission compares them with
  `BoundedFirstStepCommandJobFactory<A>` before decoding or dispatch.
- Activation uses `app.instance_id()` instead of the potentially placeholder `A::APP_ID`. Operation
  seeds and `ToolOperationKey` use the live numeric `ActionMeta.instance_id`, while the static
  compiler witness remains solely the proof owner. The two-instance regression cancels instance 41
  without cancelling identical-controller/document work for instance 42.
- `ActionBus::dispatch` and `ActionBus::dispatch_wire` now select only
  `factory_by_key.get(&key)`. Direct alias keys reject with `UnknownController`; alias metadata does
  not select a decoder or factory.
- `CopyDrawApp` deliberately copies Draw's controller, schema, command id, and constants. It obtains
  no registry proof, no generic proof, no public limit, and cannot construct its generic factory even
  when handed an attempted Draw proof. The verifier independently rejects source fixtures that omit
  the compiler witness, scope operations only by controller, or retain typed/raw alias fallback.

## Bounded Gates

| Command | Result |
| --- | --- |
| `rustfmt --edition 2021 <action-bus> <plugin>` | Exit 0 |
| `rustfmt --edition 2021 --check <action-bus> <plugin>` | Exit 0 |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | Exit 0; `self-tests=16 clean` |
| `bun ./📜️script.ts verify interactivity` | Exit 0; deny mode clean |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json` | Expected exit 1; exactly 34 global-store candidates, 12 reserved routes, and 875 unproved commands |
| Fresh generated JSON vs `📊️p8y-current-command-ledger.json` using `cmp -s` | Exit 0; byte-identical |
| `git diff --check -- <three source files + current reports/ledgers>` | Exit 0 |

The retained evidence files are `📊️p8ye-current-command-ledger.json` and
`📊️p8ye-canonical-diff-check.json`. Both verifier invocations returned the intentional exit 1 and
their JSON bodies are byte-identical. The canonical ledger records 50 macro hosts/invocations, 775
rows (773 unique), 656 literal registrations, 9 bounded admissions, 6 explicit factories, 1 bounded
factory registration site, 1 typed production dispatch site, 4 non-dispatching alias declarations,
and 16 self-tests. Its only failures remain the three migration-backlog classes above.

## Residual Gates

- Native type/borrow/Send compilation and the production Rust regressions remain unrun under the
  active disk constraint; source presence is not claimed as execution.
- Native/Wasm activation, cancellation delivery, watchdog timing, maximum-input/output behavior,
  checkpoint/progress/resume, and stale-result rejection remain mandatory integration gates.
- The 875 command rows, 34 candidate global stores, and 12 reserved routes remain intentionally hard
  failures for their owning migration packets. This repair did not broaden into those cohorts.
