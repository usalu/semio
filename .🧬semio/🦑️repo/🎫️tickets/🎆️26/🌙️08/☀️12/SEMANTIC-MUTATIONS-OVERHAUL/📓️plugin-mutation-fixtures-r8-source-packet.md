# Plugin Mutation Fixtures R8 Dummy and Transaction Source Packet

## Scope

Implemented only the released Dummy/Transaction fixture authority and close joins, the released registered-testkit helper additions, and the released Viewer seven-verb ordering correction. `new_app` and `new_viewer` remain registry-less; no Store, DSL, viewer ownership-contract, Interaction, planner, or Cargo input was changed.

## Schema-First Evidence

The neutral fixture is [`🧪️plugin-mutation-fixtures-r8/🧫️fixtures/🔣️vectors.json`](🧪️plugin-mutation-fixtures-r8/🧫️fixtures/🔣️vectors.json). It covers all signed-32-bit boundaries, missing/extra/fractional/out-of-range payload rejections, representative increment inverse restoration, and the transaction foreign-notify distinction.

The third-party Ajv 2020 gate was run through scoped Bun/Nx:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-mutation-fixtures-r8/📜️script.ts' schema
```

It completed 81 assertions against four actual leaf payload schemas. The genuine pre-cutover source-red command then completed with fixture authority `0/9`; the corrected source gate completed `15/15`, with 82 total assertions and stable input reads:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-mutation-fixtures-r8/📜️script.ts' source-red
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-mutation-fixtures-r8/📜️script.ts' source-green
```

The final source-gate hashes are:

| File | SHA-256 |
| --- | --- |
| `🎲️dummy/🦀️.rs` | `90fb1595270c12df4e6a095fb6f8040b36b732a87bdfc5a9d1c577466d3b8fa0` |
| `🔀️transaction/🦀️.rs` | `5e464f0e2d23c9252b2d22647d7f3402883016c7afeaf03656e83039f2ba1237` |
| Plugin `🦀️component.rs` | `24133b1d5aed26198d840487a732c8658414ae493b7cd5e48cc0620d3bc9f53d` |
| neutral vectors | `c30530a4558d7cdeaeed0228b1c2f07dc1e6505dfa5e1f2c50b1c4fd36d83512` |

## Mounted Changes

- Dummy owns `DummyFixtureFactory`, `DummyFixtureJob`, one exact `increment` tool id, a real async manifest, bounded document/config/draft close hooks, and the app-local no-state presence/transient hooks. The old registry-less test now asserts `interactive-job.missing-factory`; operation tests use the explicit registered constructor.
- Transaction owns `TxnFixtureFactory`, `TxnFixtureJob`, its three exact tool ids, a real async manifest, complete close hooks, and the app-local no-state hooks. Its former four `100_000`/large-byte manual loops now drive `VcsArtifactApp::close_step` under a 64-turn, one-item/page-grant law.
- Testkit gained only explicit `new_registered_app`, `paired_registered_apps`, and registered ingest/convergence helpers. They require an authored manifest future and concrete `ArtifactApp`; they do not derive factory authority from `Default`, type name, or a fallback table.
- Viewer seven-verb denial now precedes registry lookup, so a registry-less viewer returns `viewer.read-only` for the frozen seven verbs while unrelated keys remain `interactive-job.unknown-key`.

The proposed `FnOnce() -> Future` dummy type-size diagnostic remains ticket-only in [`📓️plugin-mutation-fixtures-r8-diagnosis-and-repair-proposal.md`](📓️plugin-mutation-fixtures-r8-diagnosis-and-repair-proposal.md). It is not mounted or executed.

## Native Status and Next Gate

No Cargo, rustc, executable, native test, stack-size adjustment, budget increase, or `Box::pin` workaround was used. The successful runs above are schema/source-reference evidence only. A root-serialized compiler gate must validate the new factory trait joins, async manifests, all selected Dummy/Transaction native tests, registered helper tests, and the existing R8 stack-overflow diagnostic before any runtime-ready claim.

The unapproved ArtifactViewer ownership-hook extension remains separate and untouched.

## R3 Boxed-Command And Registered-Close Repair

Runtime R3 retained two real `E0308` diagnostics: `ArtifactOwnedToolJobRequest::command` is `Box<A::Command>`, while the new Dummy and Transaction jobs had declared unboxed `Option<…Command>` fields. The narrow repair retains that owned box through each job lifecycle and borrows its command only through `Option::as_deref()`; it does not unbox, clone, or use a stack workaround.

The same review found cleanup in the wrong helpers. The original registry-less convergence/idempotence helpers have been restored to their prior behavior. The two newly registered helpers now close both retained apps after their assertion through the existing `close_registered_fixture_app` state machine. The individually registry-less Dummy rejection test still closes its own app; it is not a helper semantic change.

Before the repair, the expanded ticket source gate retained a true source red: `15/23`, covering both missing boxed-command shapes and absent registered-helper closes. The scoped Bun/Nx rerun after the patch completed `23/23` source-shape assertions plus four actual-schema Ajv validators (`82` assertions total). This remains source/schema evidence only, not a compiler or native-runtime result.

| File | SHA-256 after R3 repair |
| --- | --- |
| `🎲️dummy/🦀️.rs` | `3e09b3b18dc49acfcac1d9994c626500635ff1edebe12409fb30d631afd3fb64` |
| `🔀️transaction/🦀️.rs` | `62123f6784805572069293e253219bc78b696a43abdb3532f331363daeee37e0` |
| Plugin `🦀️component.rs` | `cce0217a13fc7cdb9e5c99770c3e8739a795a154070278fe863124697908af8b` |
| neutral vectors | `c30530a4558d7cdeaeed0228b1c2f07dc1e6505dfa5e1f2c50b1c4fd36d83512` |

The source repair deliberately leaves the separate job close budget/byte-accounting question untouched for the root-owned review.
