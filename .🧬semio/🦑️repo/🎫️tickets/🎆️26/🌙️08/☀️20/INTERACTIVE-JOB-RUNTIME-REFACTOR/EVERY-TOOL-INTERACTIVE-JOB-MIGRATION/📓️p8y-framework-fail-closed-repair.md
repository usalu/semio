# P8y Framework Fail-Closed Repair

## Disposition

**P8x/P8yb/P8yd P0 foundation defects are repaired in source and bounded static gates. Repository migration remains intentionally fail-closed.**

No Cargo, native runtime, Wasm, cache, git mutation, or ticket-metadata operation was run. The
remaining command cohorts and twelve framework-reserved jobs are outside this repair packet and stay
unavailable.

## Repair

- `ActionBus::admit_exact_wire` now resolves only the exact controller/tool key, then validates the
  exact factory type, schema, and raw-wire limit before a caller-specific decoder can run. Registered
  aliases are deliberately not accepted by this entry point. Its unit test covers alias, schema, and
  raw-limit rejection.
- All JSON action, manifest command, UI-intent, and direct typed entry points admit their exact wire
  envelope before app command construction. The decoded command id must equal the admitted id.
- The nine bounded-first-step proofs are exact compiler-derived owner-type + controller + owner-file
  + factory + tool + document-schema rows. The sealed witness carries `TypeId` and
  `type_name::<A>()`; runtime registration uses the app's resolved `instance_id()`, factory
  construction retains the witness, and exact ActionBus admission verifies the concrete generic
  factory `TypeId`. Draw/Flow/Forms public wire prescans require the live witness plus exact addressed
  controller/tool and read the same row. Copies that reuse public constants cannot inherit a limit or
  factory.
- Public typed and raw-wire `ActionBus` dispatch now use exact keys only. Explicit aliases remain
  enumerable registry metadata, but neither public dispatch method can follow one to a factory.
- The bounded worker enforces decoded items, work units, the command-specific sub-8-ms elapsed limit,
  and measured output bytes. Output measurement includes document/config/draft/presence/transient
  operations, effects, events, UI scope, and child emissions. Captured asynchronous tasks are rejected
  because their closure payload is not measurably bounded.
- `ToolCancellationHandle` retains live tokens under the live numeric app-instance + parent-document +
  operation-id + base-revision + generation key. A clone exposes exact-key and document cancellation
  plus active-operation inspection. New document work cancels superseded work, the lease guards result
  exposure, and dropping the app cancels all live work. The operation seed also includes the live
  instance, so two instances of the same static controller remain isolated.
- Empty-registry construction is now store-only: every action/typed command rejects before decoding.
  History, clipboard, and revert return before their old direct branches; their real factories remain
  represented by the red framework-reserved ledger.
- The verifier admits proofs by the full compiler-owner/controller/owner-file/factory/tool/schema identity, checks predecode admission, reserved-route
  fail-closure, shared public limits, all worker contract dimensions, and externally reachable
  cancellation. Its synthetic suite now covers sixteen cases, adding same-id/different-owner,
  registry-less reserved bypass, decode-before-lookup, public/contract mismatch, and local-only
  cancellation, plus runtime owner-proof loss, qualified-proof-after-decode, copied owner strings,
  controller-scoped operation identity, and typed/raw-wire alias fallback.
- Production Rust regressions use a concrete `CopyDrawApp` with Draw's controller, schema, command id,
  and constants, but a distinct Rust type; its registration, proof lookup, public limit, and attempted
  inherited-factory construction all reject. A second regression proves cancellation isolation for two
  live instances sharing one controller and document identity.

## Gates

| Command | Result |
| --- | --- |
| `rustfmt --edition 2021` then `rustfmt --edition 2021 --check` on action-bus and plugin | Exit 0 |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | Exit 0; `self-tests=16 clean` |
| `bun ./📜️script.ts verify interactivity` | Exit 0; deny mode clean |
| `git diff --check -- <three source files + current reports/ledgers>` | Exit 0 |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json` | Expected exit 1; only the repository migration inventory remains |
| Fresh JSON vs `📊️p8y-current-command-ledger.json` using `cmp -s` | Exit 0; byte-identical |

The fresh machine ledger is
[`📊️p8y-current-command-ledger.json`](./📊️p8y-current-command-ledger.json): 50 macro
hosts/invocations, 775 rows (773 unique), 656 literal registrations, 9 exact admissions, 6 explicit
factories, 4 non-dispatching alias declarations, 16 verifier self-tests, 875 unproved command registrations, 34 global-store candidates, and 12 reserved jobs. Its
only failures are those last three intentionally red migration categories.

## Residual Gates

- Native type/borrow/Send compilation and Rust tests were not run under the disk constraint.
- Native and Wasm runtime cancellation delivery, step timing, maximum-input/output behavior,
  checkpoint/progress behavior, and stale-result rejection remain mandatory integration gates.
- Existing registry-less test fixtures that attempt command execution must be converted to real
  registry/factory-backed fixtures by their owning command cohorts; the explicit empty-registry
  rejection test was updated in this packet.
- The 875 command rows, 34 candidate stores, and 12 framework routes remain hard failures until their
  respective migration packets provide exact factories/contracts or audited static exemptions.
