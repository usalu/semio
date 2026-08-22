# P8yc Framework Runtime Owner-Proof Repair

## Disposition

**P8yb's runtime owner-authority P0 is repaired in source and bounded static gates.** The
repository-wide migration remains intentionally red for the unchanged 875 unproved commands, 34
global payload-store candidates, and 12 framework-reserved routes.

No Cargo, native runtime, Rust test execution, Wasm build/activation, cache mutation, git mutation,
or ticket-metadata operation was run.

## Repair

- Each bounded-first-step row now carries one exact controller id in addition to its owner file,
  factory, tool id, document schema, and contract. Lookup returns a private opaque qualified proof;
  the old schema/id and id-only functions no longer exist.
- `AppActionRegistry::tool_job_registration` is bound to `ArtifactApp::APP_ID`. A mismatched registry
  controller produces no registrations, and a proved row is the only input accepted by
  `BoundedFirstStepCommandJobFactory::from_proof`.
- JSON action/manifest-command, UI-intent, and direct typed paths select the exact qualified proof
  before app-specific decoding. Exact ActionBus admission is then checked back against the same
  controller/tool/schema/contract/factory identity.
- The external JSON prescan extracts `appId` only from the bounded address object (including the
  nested app/mode command owner), pairs it with the exact action/command id, and resolves the same
  qualified row. OS/plugin owners, missing app identities, decoys, and lookalike controllers receive
  no command-specific limit.
- The production Rust regression `exact_controller_owner_proof_cannot_be_inherited_by_same_schema_and_id`
  gives two controllers the same `draw.document` schema and `canvasPointerDown` id. It asserts that
  only Draw receives a proof/factory/public limit and exact ActionBus admission; the other controller
  and a copied-controller registry spoof remain rejected.
- The verifier proof key is now controller + owner file + factory + tool + document schema. Its
  synthetic suite has 12 cases, including runtime owner-proof loss and qualified-proof-after-decode.
- The canonical P8y ledger was regenerated. It now records four aliases and current factory source
  locations (`NumberFactory` line 592; bounded factory line 11575).

## Bounded Gates

| Command | Result |
| --- | --- |
| `rustfmt --edition 2021 <plugin>` then `rustfmt --edition 2021 --check <plugin>` | Exit 0 |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | Exit 0; `self-tests=12 clean` |
| `bun ./📜️script.ts verify interactivity` | Exit 0; deny mode clean |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json` | Expected exit 1; exactly 34 global candidates, 12 reserved routes, 875 unproved registrations |
| Fresh generated JSON vs `📊️p8y-current-command-ledger.json` using `cmp -s` | Exit 0; byte-identical |
| `git diff --check -- <five source files + p8y report/ledger>` | Exit 0 |

The retained evidence files are `📊️p8yc-preformat-ledger.json`,
`📊️p8yc-current-command-ledger.json`, and `📊️p8yc-canonical-diff-check.json`.

## Residual Gates

- Native type/borrow/Send compilation and Rust test execution remain unrun under the active disk
  constraint. The production regression is source-complete but is not claimed as executed.
- Native/Wasm cancellation delivery, watchdog timing, maximum-input/output behavior,
  checkpoint/progress behavior, and stale-result rejection remain mandatory integration gates.
- The canonical ledger's three intentional failure categories remain hard failures for their owning
  migration packets; this repair did not broaden into those cohorts.
